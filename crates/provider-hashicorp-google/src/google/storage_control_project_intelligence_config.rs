use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct StorageControlProjectIntelligenceConfigData {
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
    filter: Option<Vec<StorageControlProjectIntelligenceConfigFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<StorageControlProjectIntelligenceConfigTimeoutsEl>,
    dynamic: StorageControlProjectIntelligenceConfigDynamic,
}
struct StorageControlProjectIntelligenceConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<StorageControlProjectIntelligenceConfigData>,
}
#[derive(Clone)]
pub struct StorageControlProjectIntelligenceConfig(Rc<StorageControlProjectIntelligenceConfig_>);
impl StorageControlProjectIntelligenceConfig {
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
    #[doc = "Set the field `edition_config`.\nEdition configuration of the Storage Intelligence resource. Valid values are INHERIT, TRIAL, DISABLED and STANDARD."]
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
        v: impl Into<BlockAssignable<StorageControlProjectIntelligenceConfigFilterEl>>,
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
        v: impl Into<StorageControlProjectIntelligenceConfigTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
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
    ) -> ListRef<StorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.effective_intelligence_config", self.extract_ref()),
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
    pub fn trial_config(&self) -> ListRef<StorageControlProjectIntelligenceConfigTrialConfigElRef> {
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
    pub fn filter(&self) -> ListRef<StorageControlProjectIntelligenceConfigFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageControlProjectIntelligenceConfigTimeoutsElRef {
        StorageControlProjectIntelligenceConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for StorageControlProjectIntelligenceConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for StorageControlProjectIntelligenceConfig {}
impl ToListMappable for StorageControlProjectIntelligenceConfig {
    type O = ListRef<StorageControlProjectIntelligenceConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for StorageControlProjectIntelligenceConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_storage_control_project_intelligence_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildStorageControlProjectIntelligenceConfig {
    pub tf_id: String,
    #[doc = "Identifier of the GCP project. For GCP project, this field can be project name or project number."]
    pub name: PrimField<String>,
}
impl BuildStorageControlProjectIntelligenceConfig {
    pub fn build(self, stack: &mut Stack) -> StorageControlProjectIntelligenceConfig {
        let out = StorageControlProjectIntelligenceConfig(Rc::new(
            StorageControlProjectIntelligenceConfig_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(StorageControlProjectIntelligenceConfigData {
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
pub struct StorageControlProjectIntelligenceConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageControlProjectIntelligenceConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl StorageControlProjectIntelligenceConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    ) -> ListRef<StorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.effective_intelligence_config", self.extract_ref()),
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
    pub fn trial_config(&self) -> ListRef<StorageControlProjectIntelligenceConfigTrialConfigElRef> {
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
    pub fn filter(&self) -> ListRef<StorageControlProjectIntelligenceConfigFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageControlProjectIntelligenceConfigTimeoutsElRef {
        StorageControlProjectIntelligenceConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct StorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_edition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    intelligence_config: Option<PrimField<String>>,
}
impl StorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigEl {
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
impl ToListMappable for StorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigEl {
    type O = BlockAssignable<StorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigEl {}
impl BuildStorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigEl {
    pub fn build(self) -> StorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigEl {
        StorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigEl {
            effective_edition: core::default::Default::default(),
            intelligence_config: core::default::Default::default(),
        }
    }
}
pub struct StorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigElRef {
        StorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageControlProjectIntelligenceConfigEffectiveIntelligenceConfigElRef {
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
pub struct StorageControlProjectIntelligenceConfigTrialConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_time: Option<PrimField<String>>,
}
impl StorageControlProjectIntelligenceConfigTrialConfigEl {
    #[doc = "Set the field `expire_time`.\n"]
    pub fn set_expire_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expire_time = Some(v.into());
        self
    }
}
impl ToListMappable for StorageControlProjectIntelligenceConfigTrialConfigEl {
    type O = BlockAssignable<StorageControlProjectIntelligenceConfigTrialConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageControlProjectIntelligenceConfigTrialConfigEl {}
impl BuildStorageControlProjectIntelligenceConfigTrialConfigEl {
    pub fn build(self) -> StorageControlProjectIntelligenceConfigTrialConfigEl {
        StorageControlProjectIntelligenceConfigTrialConfigEl {
            expire_time: core::default::Default::default(),
        }
    }
}
pub struct StorageControlProjectIntelligenceConfigTrialConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageControlProjectIntelligenceConfigTrialConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageControlProjectIntelligenceConfigTrialConfigElRef {
        StorageControlProjectIntelligenceConfigTrialConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageControlProjectIntelligenceConfigTrialConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\n"]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
    bucket_id_regexes: ListField<PrimField<String>>,
}
impl StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {}
impl ToListMappable
    for StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsEl
{
    type O = BlockAssignable<
        StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
    #[doc = "List of bucket id regexes to exclude in the storage intelligence plan."]
    pub bucket_id_regexes: ListField<PrimField<String>>,
}
impl BuildStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
    pub fn build(
        self,
    ) -> StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
        StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
            bucket_id_regexes: self.bucket_id_regexes,
        }
    }
}
pub struct StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
        StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
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
pub struct StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
    locations: ListField<PrimField<String>>,
}
impl StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {}
impl ToListMappable
    for StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsEl
{
    type O = BlockAssignable<
        StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
    #[doc = "List of locations."]
    pub locations: ListField<PrimField<String>>,
}
impl BuildStorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
    pub fn build(
        self,
    ) -> StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
        StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
            locations: self.locations,
        }
    }
}
pub struct StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef {
        StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `locations` after provisioning.\nList of locations."]
    pub fn locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
    bucket_id_regexes: ListField<PrimField<String>>,
}
impl StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {}
impl ToListMappable
    for StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsEl
{
    type O = BlockAssignable<
        StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
    #[doc = "List of bucket id regexes to exclude in the storage intelligence plan."]
    pub bucket_id_regexes: ListField<PrimField<String>>,
}
impl BuildStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
    pub fn build(
        self,
    ) -> StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
        StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
            bucket_id_regexes: self.bucket_id_regexes,
        }
    }
}
pub struct StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
        StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
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
pub struct StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
    locations: ListField<PrimField<String>>,
}
impl StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {}
impl ToListMappable
    for StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsEl
{
    type O = BlockAssignable<
        StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
    #[doc = "List of locations."]
    pub locations: ListField<PrimField<String>>,
}
impl BuildStorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
    pub fn build(
        self,
    ) -> StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
        StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
            locations: self.locations,
        }
    }
}
pub struct StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef {
        StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `locations` after provisioning.\nList of locations."]
    pub fn locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
}
#[derive(Serialize, Default)]
struct StorageControlProjectIntelligenceConfigFilterElDynamic {
    excluded_cloud_storage_buckets: Option<
        DynamicBlock<StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsEl>,
    >,
    excluded_cloud_storage_locations: Option<
        DynamicBlock<
            StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsEl,
        >,
    >,
    included_cloud_storage_buckets: Option<
        DynamicBlock<StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsEl>,
    >,
    included_cloud_storage_locations: Option<
        DynamicBlock<
            StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct StorageControlProjectIntelligenceConfigFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_cloud_storage_buckets:
        Option<Vec<StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_cloud_storage_locations:
        Option<Vec<StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    included_cloud_storage_buckets:
        Option<Vec<StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    included_cloud_storage_locations:
        Option<Vec<StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsEl>>,
    dynamic: StorageControlProjectIntelligenceConfigFilterElDynamic,
}
impl StorageControlProjectIntelligenceConfigFilterEl {
    #[doc = "Set the field `excluded_cloud_storage_buckets`.\n"]
    pub fn set_excluded_cloud_storage_buckets(
        mut self,
        v: impl Into<
            BlockAssignable<
                StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsEl,
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
                StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsEl,
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
                StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsEl,
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
                StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsEl,
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
impl ToListMappable for StorageControlProjectIntelligenceConfigFilterEl {
    type O = BlockAssignable<StorageControlProjectIntelligenceConfigFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageControlProjectIntelligenceConfigFilterEl {}
impl BuildStorageControlProjectIntelligenceConfigFilterEl {
    pub fn build(self) -> StorageControlProjectIntelligenceConfigFilterEl {
        StorageControlProjectIntelligenceConfigFilterEl {
            excluded_cloud_storage_buckets: core::default::Default::default(),
            excluded_cloud_storage_locations: core::default::Default::default(),
            included_cloud_storage_buckets: core::default::Default::default(),
            included_cloud_storage_locations: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct StorageControlProjectIntelligenceConfigFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageControlProjectIntelligenceConfigFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageControlProjectIntelligenceConfigFilterElRef {
        StorageControlProjectIntelligenceConfigFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageControlProjectIntelligenceConfigFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `excluded_cloud_storage_buckets` after provisioning.\n"]
    pub fn excluded_cloud_storage_buckets(
        &self,
    ) -> ListRef<StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.excluded_cloud_storage_buckets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `excluded_cloud_storage_locations` after provisioning.\n"]
    pub fn excluded_cloud_storage_locations(
        &self,
    ) -> ListRef<StorageControlProjectIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.excluded_cloud_storage_locations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `included_cloud_storage_buckets` after provisioning.\n"]
    pub fn included_cloud_storage_buckets(
        &self,
    ) -> ListRef<StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.included_cloud_storage_buckets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `included_cloud_storage_locations` after provisioning.\n"]
    pub fn included_cloud_storage_locations(
        &self,
    ) -> ListRef<StorageControlProjectIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.included_cloud_storage_locations", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageControlProjectIntelligenceConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl StorageControlProjectIntelligenceConfigTimeoutsEl {
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
impl ToListMappable for StorageControlProjectIntelligenceConfigTimeoutsEl {
    type O = BlockAssignable<StorageControlProjectIntelligenceConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageControlProjectIntelligenceConfigTimeoutsEl {}
impl BuildStorageControlProjectIntelligenceConfigTimeoutsEl {
    pub fn build(self) -> StorageControlProjectIntelligenceConfigTimeoutsEl {
        StorageControlProjectIntelligenceConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct StorageControlProjectIntelligenceConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageControlProjectIntelligenceConfigTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageControlProjectIntelligenceConfigTimeoutsElRef {
        StorageControlProjectIntelligenceConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageControlProjectIntelligenceConfigTimeoutsElRef {
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
struct StorageControlProjectIntelligenceConfigDynamic {
    filter: Option<DynamicBlock<StorageControlProjectIntelligenceConfigFilterEl>>,
}
