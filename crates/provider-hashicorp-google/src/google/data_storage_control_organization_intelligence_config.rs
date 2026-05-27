use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataStorageControlOrganizationIntelligenceConfigData {
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
struct DataStorageControlOrganizationIntelligenceConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataStorageControlOrganizationIntelligenceConfigData>,
}
#[derive(Clone)]
pub struct DataStorageControlOrganizationIntelligenceConfig(
    Rc<DataStorageControlOrganizationIntelligenceConfig_>,
);
impl DataStorageControlOrganizationIntelligenceConfig {
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
    #[doc = "Get a reference to the value of field `edition_config` after provisioning.\nEdition configuration of the Storage Intelligence resource. Valid values are INHERIT, DISABLED, TRIAL and STANDARD."]
    pub fn edition_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.edition_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_intelligence_config` after provisioning.\nThe Intelligence config that is effective for the resource."]
    pub fn effective_intelligence_config(
        &self,
    ) -> ListRef<DataStorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.effective_intelligence_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nFilter over location and bucket using include or exclude semantics. Resources that match the include or exclude filter are exclusively included or excluded from the Storage Intelligence plan."]
    pub fn filter(&self) -> ListRef<DataStorageControlOrganizationIntelligenceConfigFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier of the GCP Organization. For GCP org, this field should be organization number."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trial_config` after provisioning.\nThe trial configuration of the Storage Intelligence resource."]
    pub fn trial_config(
        &self,
    ) -> ListRef<DataStorageControlOrganizationIntelligenceConfigTrialConfigElRef> {
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
impl Referable for DataStorageControlOrganizationIntelligenceConfig {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataStorageControlOrganizationIntelligenceConfig {}
impl ToListMappable for DataStorageControlOrganizationIntelligenceConfig {
    type O = ListRef<DataStorageControlOrganizationIntelligenceConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataStorageControlOrganizationIntelligenceConfig_ {
    fn extract_datasource_type(&self) -> String {
        "google_storage_control_organization_intelligence_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataStorageControlOrganizationIntelligenceConfig {
    pub tf_id: String,
    #[doc = "Identifier of the GCP Organization. For GCP org, this field should be organization number."]
    pub name: PrimField<String>,
}
impl BuildDataStorageControlOrganizationIntelligenceConfig {
    pub fn build(self, stack: &mut Stack) -> DataStorageControlOrganizationIntelligenceConfig {
        let out = DataStorageControlOrganizationIntelligenceConfig(Rc::new(
            DataStorageControlOrganizationIntelligenceConfig_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataStorageControlOrganizationIntelligenceConfigData {
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
pub struct DataStorageControlOrganizationIntelligenceConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageControlOrganizationIntelligenceConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataStorageControlOrganizationIntelligenceConfigRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `edition_config` after provisioning.\nEdition configuration of the Storage Intelligence resource. Valid values are INHERIT, DISABLED, TRIAL and STANDARD."]
    pub fn edition_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.edition_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_intelligence_config` after provisioning.\nThe Intelligence config that is effective for the resource."]
    pub fn effective_intelligence_config(
        &self,
    ) -> ListRef<DataStorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.effective_intelligence_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nFilter over location and bucket using include or exclude semantics. Resources that match the include or exclude filter are exclusively included or excluded from the Storage Intelligence plan."]
    pub fn filter(&self) -> ListRef<DataStorageControlOrganizationIntelligenceConfigFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier of the GCP Organization. For GCP org, this field should be organization number."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trial_config` after provisioning.\nThe trial configuration of the Storage Intelligence resource."]
    pub fn trial_config(
        &self,
    ) -> ListRef<DataStorageControlOrganizationIntelligenceConfigTrialConfigElRef> {
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
pub struct DataStorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_edition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    intelligence_config: Option<PrimField<String>>,
}
impl DataStorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigEl {
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
impl ToListMappable
    for DataStorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigEl
{
    type O = BlockAssignable<
        DataStorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigEl {}
impl BuildDataStorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigEl {
    pub fn build(
        self,
    ) -> DataStorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigEl {
        DataStorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigEl {
            effective_edition: core::default::Default::default(),
            intelligence_config: core::default::Default::default(),
        }
    }
}
pub struct DataStorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigElRef {
        DataStorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigElRef {
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
pub struct DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_id_regexes: Option<ListField<PrimField<String>>>,
}
impl DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
    #[doc = "Set the field `bucket_id_regexes`.\n"]
    pub fn set_bucket_id_regexes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.bucket_id_regexes = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsEl
{
    type O = BlockAssignable<
        DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsEl
{}
impl BuildDataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
    pub fn build(
        self,
    ) -> DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
        DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
            bucket_id_regexes: core::default::Default::default(),
        }
    }
}
pub struct DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef
    {
        DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
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
pub struct DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    locations: Option<ListField<PrimField<String>>>,
}
impl DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
    #[doc = "Set the field `locations`.\n"]
    pub fn set_locations(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.locations = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsEl
{
    type O = BlockAssignable<
        DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsEl
{}
impl BuildDataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
    pub fn build(
        self,
    ) -> DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsEl
    {
        DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
            locations: core::default::Default::default(),
        }
    }
}
pub struct DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef
    {
        DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `locations` after provisioning.\n"]
    pub fn locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
}
#[derive(Serialize)]
pub struct DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_id_regexes: Option<ListField<PrimField<String>>>,
}
impl DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
    #[doc = "Set the field `bucket_id_regexes`.\n"]
    pub fn set_bucket_id_regexes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.bucket_id_regexes = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsEl
{
    type O = BlockAssignable<
        DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsEl
{}
impl BuildDataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
    pub fn build(
        self,
    ) -> DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
        DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
            bucket_id_regexes: core::default::Default::default(),
        }
    }
}
pub struct DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef
    {
        DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
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
pub struct DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    locations: Option<ListField<PrimField<String>>>,
}
impl DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
    #[doc = "Set the field `locations`.\n"]
    pub fn set_locations(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.locations = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsEl
{
    type O = BlockAssignable<
        DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsEl
{}
impl BuildDataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
    pub fn build(
        self,
    ) -> DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsEl
    {
        DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
            locations: core::default::Default::default(),
        }
    }
}
pub struct DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef
    {
        DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `locations` after provisioning.\n"]
    pub fn locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
}
#[derive(Serialize)]
pub struct DataStorageControlOrganizationIntelligenceConfigFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_cloud_storage_buckets: Option<
        ListField<
            DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_cloud_storage_locations: Option<
        ListField<
            DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    included_cloud_storage_buckets: Option<
        ListField<
            DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    included_cloud_storage_locations: Option<
        ListField<
            DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsEl,
        >,
    >,
}
impl DataStorageControlOrganizationIntelligenceConfigFilterEl {
    #[doc = "Set the field `excluded_cloud_storage_buckets`.\n"]
    pub fn set_excluded_cloud_storage_buckets(
        mut self,
        v : impl Into < ListField < DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsEl > >,
    ) -> Self {
        self.excluded_cloud_storage_buckets = Some(v.into());
        self
    }
    #[doc = "Set the field `excluded_cloud_storage_locations`.\n"]
    pub fn set_excluded_cloud_storage_locations(
        mut self,
        v : impl Into < ListField < DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsEl > >,
    ) -> Self {
        self.excluded_cloud_storage_locations = Some(v.into());
        self
    }
    #[doc = "Set the field `included_cloud_storage_buckets`.\n"]
    pub fn set_included_cloud_storage_buckets(
        mut self,
        v : impl Into < ListField < DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsEl > >,
    ) -> Self {
        self.included_cloud_storage_buckets = Some(v.into());
        self
    }
    #[doc = "Set the field `included_cloud_storage_locations`.\n"]
    pub fn set_included_cloud_storage_locations(
        mut self,
        v : impl Into < ListField < DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsEl > >,
    ) -> Self {
        self.included_cloud_storage_locations = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageControlOrganizationIntelligenceConfigFilterEl {
    type O = BlockAssignable<DataStorageControlOrganizationIntelligenceConfigFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageControlOrganizationIntelligenceConfigFilterEl {}
impl BuildDataStorageControlOrganizationIntelligenceConfigFilterEl {
    pub fn build(self) -> DataStorageControlOrganizationIntelligenceConfigFilterEl {
        DataStorageControlOrganizationIntelligenceConfigFilterEl {
            excluded_cloud_storage_buckets: core::default::Default::default(),
            excluded_cloud_storage_locations: core::default::Default::default(),
            included_cloud_storage_buckets: core::default::Default::default(),
            included_cloud_storage_locations: core::default::Default::default(),
        }
    }
}
pub struct DataStorageControlOrganizationIntelligenceConfigFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageControlOrganizationIntelligenceConfigFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageControlOrganizationIntelligenceConfigFilterElRef {
        DataStorageControlOrganizationIntelligenceConfigFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageControlOrganizationIntelligenceConfigFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `excluded_cloud_storage_buckets` after provisioning.\n"]
    pub fn excluded_cloud_storage_buckets(
        &self,
    ) -> ListRef<
        DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.excluded_cloud_storage_buckets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `excluded_cloud_storage_locations` after provisioning.\n"]
    pub fn excluded_cloud_storage_locations(
        &self,
    ) -> ListRef<
        DataStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.excluded_cloud_storage_locations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `included_cloud_storage_buckets` after provisioning.\n"]
    pub fn included_cloud_storage_buckets(
        &self,
    ) -> ListRef<
        DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.included_cloud_storage_buckets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `included_cloud_storage_locations` after provisioning.\n"]
    pub fn included_cloud_storage_locations(
        &self,
    ) -> ListRef<
        DataStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.included_cloud_storage_locations", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageControlOrganizationIntelligenceConfigTrialConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_time: Option<PrimField<String>>,
}
impl DataStorageControlOrganizationIntelligenceConfigTrialConfigEl {
    #[doc = "Set the field `expire_time`.\n"]
    pub fn set_expire_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expire_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageControlOrganizationIntelligenceConfigTrialConfigEl {
    type O = BlockAssignable<DataStorageControlOrganizationIntelligenceConfigTrialConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageControlOrganizationIntelligenceConfigTrialConfigEl {}
impl BuildDataStorageControlOrganizationIntelligenceConfigTrialConfigEl {
    pub fn build(self) -> DataStorageControlOrganizationIntelligenceConfigTrialConfigEl {
        DataStorageControlOrganizationIntelligenceConfigTrialConfigEl {
            expire_time: core::default::Default::default(),
        }
    }
}
pub struct DataStorageControlOrganizationIntelligenceConfigTrialConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageControlOrganizationIntelligenceConfigTrialConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageControlOrganizationIntelligenceConfigTrialConfigElRef {
        DataStorageControlOrganizationIntelligenceConfigTrialConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageControlOrganizationIntelligenceConfigTrialConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\n"]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.base))
    }
}
