use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct StorageControlOrganizationIntelligenceConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edition_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<StorageControlOrganizationIntelligenceConfigFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<StorageControlOrganizationIntelligenceConfigTimeoutsEl>,
    dynamic: StorageControlOrganizationIntelligenceConfigDynamic,
}
struct StorageControlOrganizationIntelligenceConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<StorageControlOrganizationIntelligenceConfigData>,
}
#[derive(Clone)]
pub struct StorageControlOrganizationIntelligenceConfig(
    Rc<StorageControlOrganizationIntelligenceConfig_>,
);
impl StorageControlOrganizationIntelligenceConfig {
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
    #[doc = "Set the field `edition_config`.\nEdition configuration of the Storage Intelligence resource. Valid values are INHERIT, DISABLED, TRIAL and STANDARD."]
    pub fn set_edition_config(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().edition_config = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl Into<BlockAssignable<StorageControlOrganizationIntelligenceConfigFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<StorageControlOrganizationIntelligenceConfigTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
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
    ) -> ListRef<StorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.effective_intelligence_config", self.extract_ref()),
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
    ) -> ListRef<StorageControlOrganizationIntelligenceConfigTrialConfigElRef> {
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
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<StorageControlOrganizationIntelligenceConfigFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageControlOrganizationIntelligenceConfigTimeoutsElRef {
        StorageControlOrganizationIntelligenceConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for StorageControlOrganizationIntelligenceConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for StorageControlOrganizationIntelligenceConfig {}
impl ToListMappable for StorageControlOrganizationIntelligenceConfig {
    type O = ListRef<StorageControlOrganizationIntelligenceConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for StorageControlOrganizationIntelligenceConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_storage_control_organization_intelligence_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildStorageControlOrganizationIntelligenceConfig {
    pub tf_id: String,
    #[doc = "Identifier of the GCP Organization. For GCP org, this field should be organization number."]
    pub name: PrimField<String>,
}
impl BuildStorageControlOrganizationIntelligenceConfig {
    pub fn build(self, stack: &mut Stack) -> StorageControlOrganizationIntelligenceConfig {
        let out = StorageControlOrganizationIntelligenceConfig(Rc::new(
            StorageControlOrganizationIntelligenceConfig_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(StorageControlOrganizationIntelligenceConfigData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    edition_config: core::default::Default::default(),
                    id: core::default::Default::default(),
                    name: self.name,
                    filter: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct StorageControlOrganizationIntelligenceConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageControlOrganizationIntelligenceConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl StorageControlOrganizationIntelligenceConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    ) -> ListRef<StorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.effective_intelligence_config", self.extract_ref()),
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
    ) -> ListRef<StorageControlOrganizationIntelligenceConfigTrialConfigElRef> {
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
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<StorageControlOrganizationIntelligenceConfigFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageControlOrganizationIntelligenceConfigTimeoutsElRef {
        StorageControlOrganizationIntelligenceConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct StorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_edition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    intelligence_config: Option<PrimField<String>>,
}
impl StorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigEl {
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
impl ToListMappable for StorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigEl {
    type O =
        BlockAssignable<StorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigEl {}
impl BuildStorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigEl {
    pub fn build(
        self,
    ) -> StorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigEl {
        StorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigEl {
            effective_edition: core::default::Default::default(),
            intelligence_config: core::default::Default::default(),
        }
    }
}
pub struct StorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigElRef {
        StorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageControlOrganizationIntelligenceConfigEffectiveIntelligenceConfigElRef {
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
pub struct StorageControlOrganizationIntelligenceConfigTrialConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_time: Option<PrimField<String>>,
}
impl StorageControlOrganizationIntelligenceConfigTrialConfigEl {
    #[doc = "Set the field `expire_time`.\n"]
    pub fn set_expire_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expire_time = Some(v.into());
        self
    }
}
impl ToListMappable for StorageControlOrganizationIntelligenceConfigTrialConfigEl {
    type O = BlockAssignable<StorageControlOrganizationIntelligenceConfigTrialConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageControlOrganizationIntelligenceConfigTrialConfigEl {}
impl BuildStorageControlOrganizationIntelligenceConfigTrialConfigEl {
    pub fn build(self) -> StorageControlOrganizationIntelligenceConfigTrialConfigEl {
        StorageControlOrganizationIntelligenceConfigTrialConfigEl {
            expire_time: core::default::Default::default(),
        }
    }
}
pub struct StorageControlOrganizationIntelligenceConfigTrialConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageControlOrganizationIntelligenceConfigTrialConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageControlOrganizationIntelligenceConfigTrialConfigElRef {
        StorageControlOrganizationIntelligenceConfigTrialConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageControlOrganizationIntelligenceConfigTrialConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\n"]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
    bucket_id_regexes: ListField<PrimField<String>>,
}
impl StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {}
impl ToListMappable
    for StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsEl
{
    type O = BlockAssignable<
        StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
    #[doc = "List of bucket id regexes to exclude in the storage intelligence plan."]
    pub bucket_id_regexes: ListField<PrimField<String>>,
}
impl BuildStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
    pub fn build(
        self,
    ) -> StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
        StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
            bucket_id_regexes: self.bucket_id_regexes,
        }
    }
}
pub struct StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
        StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_id_regexes` after provisioning.\nList of bucket id regexes to exclude in the storage intelligence plan."]
    pub fn bucket_id_regexes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bucket_id_regexes", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
    locations: ListField<PrimField<String>>,
}
impl StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {}
impl ToListMappable
    for StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsEl
{
    type O = BlockAssignable<
        StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsEl
{
    #[doc = "List of locations."]
    pub locations: ListField<PrimField<String>>,
}
impl BuildStorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
    pub fn build(
        self,
    ) -> StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
        StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
            locations: self.locations,
        }
    }
}
pub struct StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef
    {
        StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `locations` after provisioning.\nList of locations."]
    pub fn locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
    bucket_id_regexes: ListField<PrimField<String>>,
}
impl StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {}
impl ToListMappable
    for StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsEl
{
    type O = BlockAssignable<
        StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
    #[doc = "List of bucket id regexes to exclude in the storage intelligence plan."]
    pub bucket_id_regexes: ListField<PrimField<String>>,
}
impl BuildStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
    pub fn build(
        self,
    ) -> StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
        StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
            bucket_id_regexes: self.bucket_id_regexes,
        }
    }
}
pub struct StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
        StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_id_regexes` after provisioning.\nList of bucket id regexes to exclude in the storage intelligence plan."]
    pub fn bucket_id_regexes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bucket_id_regexes", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
    locations: ListField<PrimField<String>>,
}
impl StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {}
impl ToListMappable
    for StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsEl
{
    type O = BlockAssignable<
        StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsEl
{
    #[doc = "List of locations."]
    pub locations: ListField<PrimField<String>>,
}
impl BuildStorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
    pub fn build(
        self,
    ) -> StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
        StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
            locations: self.locations,
        }
    }
}
pub struct StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef
    {
        StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `locations` after provisioning.\nList of locations."]
    pub fn locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
}
#[derive(Serialize, Default)]
struct StorageControlOrganizationIntelligenceConfigFilterElDynamic {
    excluded_cloud_storage_buckets: Option<
        DynamicBlock<
            StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsEl,
        >,
    >,
    excluded_cloud_storage_locations: Option<
        DynamicBlock<
            StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsEl,
        >,
    >,
    included_cloud_storage_buckets: Option<
        DynamicBlock<
            StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsEl,
        >,
    >,
    included_cloud_storage_locations: Option<
        DynamicBlock<
            StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct StorageControlOrganizationIntelligenceConfigFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_cloud_storage_buckets: Option<
        Vec<StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_cloud_storage_locations: Option<
        Vec<StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    included_cloud_storage_buckets: Option<
        Vec<StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    included_cloud_storage_locations: Option<
        Vec<StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsEl>,
    >,
    dynamic: StorageControlOrganizationIntelligenceConfigFilterElDynamic,
}
impl StorageControlOrganizationIntelligenceConfigFilterEl {
    #[doc = "Set the field `excluded_cloud_storage_buckets`.\n"]
    pub fn set_excluded_cloud_storage_buckets(
        mut self,
        v: impl Into<
            BlockAssignable<
                StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.excluded_cloud_storage_buckets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.excluded_cloud_storage_buckets = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `excluded_cloud_storage_locations`.\n"]
    pub fn set_excluded_cloud_storage_locations(
        mut self,
        v: impl Into<
            BlockAssignable<
                StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.excluded_cloud_storage_locations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.excluded_cloud_storage_locations = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `included_cloud_storage_buckets`.\n"]
    pub fn set_included_cloud_storage_buckets(
        mut self,
        v: impl Into<
            BlockAssignable<
                StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.included_cloud_storage_buckets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.included_cloud_storage_buckets = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `included_cloud_storage_locations`.\n"]
    pub fn set_included_cloud_storage_locations(
        mut self,
        v: impl Into<
            BlockAssignable<
                StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.included_cloud_storage_locations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.included_cloud_storage_locations = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for StorageControlOrganizationIntelligenceConfigFilterEl {
    type O = BlockAssignable<StorageControlOrganizationIntelligenceConfigFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageControlOrganizationIntelligenceConfigFilterEl {}
impl BuildStorageControlOrganizationIntelligenceConfigFilterEl {
    pub fn build(self) -> StorageControlOrganizationIntelligenceConfigFilterEl {
        StorageControlOrganizationIntelligenceConfigFilterEl {
            excluded_cloud_storage_buckets: core::default::Default::default(),
            excluded_cloud_storage_locations: core::default::Default::default(),
            included_cloud_storage_buckets: core::default::Default::default(),
            included_cloud_storage_locations: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct StorageControlOrganizationIntelligenceConfigFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageControlOrganizationIntelligenceConfigFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageControlOrganizationIntelligenceConfigFilterElRef {
        StorageControlOrganizationIntelligenceConfigFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageControlOrganizationIntelligenceConfigFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `excluded_cloud_storage_buckets` after provisioning.\n"]
    pub fn excluded_cloud_storage_buckets(
        &self,
    ) -> ListRef<StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef>
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
        StorageControlOrganizationIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.excluded_cloud_storage_locations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `included_cloud_storage_buckets` after provisioning.\n"]
    pub fn included_cloud_storage_buckets(
        &self,
    ) -> ListRef<StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef>
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
        StorageControlOrganizationIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.included_cloud_storage_locations", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageControlOrganizationIntelligenceConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl StorageControlOrganizationIntelligenceConfigTimeoutsEl {
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
impl ToListMappable for StorageControlOrganizationIntelligenceConfigTimeoutsEl {
    type O = BlockAssignable<StorageControlOrganizationIntelligenceConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageControlOrganizationIntelligenceConfigTimeoutsEl {}
impl BuildStorageControlOrganizationIntelligenceConfigTimeoutsEl {
    pub fn build(self) -> StorageControlOrganizationIntelligenceConfigTimeoutsEl {
        StorageControlOrganizationIntelligenceConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct StorageControlOrganizationIntelligenceConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageControlOrganizationIntelligenceConfigTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageControlOrganizationIntelligenceConfigTimeoutsElRef {
        StorageControlOrganizationIntelligenceConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageControlOrganizationIntelligenceConfigTimeoutsElRef {
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
struct StorageControlOrganizationIntelligenceConfigDynamic {
    filter: Option<DynamicBlock<StorageControlOrganizationIntelligenceConfigFilterEl>>,
}
