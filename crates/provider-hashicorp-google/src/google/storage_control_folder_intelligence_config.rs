use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct StorageControlFolderIntelligenceConfigData {
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
    filter: Option<Vec<StorageControlFolderIntelligenceConfigFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<StorageControlFolderIntelligenceConfigTimeoutsEl>,
    dynamic: StorageControlFolderIntelligenceConfigDynamic,
}
struct StorageControlFolderIntelligenceConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<StorageControlFolderIntelligenceConfigData>,
}
#[derive(Clone)]
pub struct StorageControlFolderIntelligenceConfig(Rc<StorageControlFolderIntelligenceConfig_>);
impl StorageControlFolderIntelligenceConfig {
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
        v: impl Into<BlockAssignable<StorageControlFolderIntelligenceConfigFilterEl>>,
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
        v: impl Into<StorageControlFolderIntelligenceConfigTimeoutsEl>,
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
    ) -> ListRef<StorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.effective_intelligence_config", self.extract_ref()),
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
    pub fn trial_config(&self) -> ListRef<StorageControlFolderIntelligenceConfigTrialConfigElRef> {
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
    pub fn filter(&self) -> ListRef<StorageControlFolderIntelligenceConfigFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageControlFolderIntelligenceConfigTimeoutsElRef {
        StorageControlFolderIntelligenceConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for StorageControlFolderIntelligenceConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for StorageControlFolderIntelligenceConfig {}
impl ToListMappable for StorageControlFolderIntelligenceConfig {
    type O = ListRef<StorageControlFolderIntelligenceConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for StorageControlFolderIntelligenceConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_storage_control_folder_intelligence_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildStorageControlFolderIntelligenceConfig {
    pub tf_id: String,
    #[doc = "Identifier of the GCP Folder. For GCP Folder, this field can be folder number."]
    pub name: PrimField<String>,
}
impl BuildStorageControlFolderIntelligenceConfig {
    pub fn build(self, stack: &mut Stack) -> StorageControlFolderIntelligenceConfig {
        let out = StorageControlFolderIntelligenceConfig(Rc::new(
            StorageControlFolderIntelligenceConfig_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(StorageControlFolderIntelligenceConfigData {
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
pub struct StorageControlFolderIntelligenceConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageControlFolderIntelligenceConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl StorageControlFolderIntelligenceConfigRef {
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
    ) -> ListRef<StorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.effective_intelligence_config", self.extract_ref()),
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
    pub fn trial_config(&self) -> ListRef<StorageControlFolderIntelligenceConfigTrialConfigElRef> {
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
    pub fn filter(&self) -> ListRef<StorageControlFolderIntelligenceConfigFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageControlFolderIntelligenceConfigTimeoutsElRef {
        StorageControlFolderIntelligenceConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct StorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_edition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    intelligence_config: Option<PrimField<String>>,
}
impl StorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigEl {
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
impl ToListMappable for StorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigEl {
    type O = BlockAssignable<StorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigEl {}
impl BuildStorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigEl {
    pub fn build(self) -> StorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigEl {
        StorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigEl {
            effective_edition: core::default::Default::default(),
            intelligence_config: core::default::Default::default(),
        }
    }
}
pub struct StorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigElRef {
        StorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageControlFolderIntelligenceConfigEffectiveIntelligenceConfigElRef {
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
pub struct StorageControlFolderIntelligenceConfigTrialConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_time: Option<PrimField<String>>,
}
impl StorageControlFolderIntelligenceConfigTrialConfigEl {
    #[doc = "Set the field `expire_time`.\n"]
    pub fn set_expire_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expire_time = Some(v.into());
        self
    }
}
impl ToListMappable for StorageControlFolderIntelligenceConfigTrialConfigEl {
    type O = BlockAssignable<StorageControlFolderIntelligenceConfigTrialConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageControlFolderIntelligenceConfigTrialConfigEl {}
impl BuildStorageControlFolderIntelligenceConfigTrialConfigEl {
    pub fn build(self) -> StorageControlFolderIntelligenceConfigTrialConfigEl {
        StorageControlFolderIntelligenceConfigTrialConfigEl {
            expire_time: core::default::Default::default(),
        }
    }
}
pub struct StorageControlFolderIntelligenceConfigTrialConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageControlFolderIntelligenceConfigTrialConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageControlFolderIntelligenceConfigTrialConfigElRef {
        StorageControlFolderIntelligenceConfigTrialConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageControlFolderIntelligenceConfigTrialConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\n"]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
    bucket_id_regexes: ListField<PrimField<String>>,
}
impl StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {}
impl ToListMappable
    for StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsEl
{
    type O = BlockAssignable<
        StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
    #[doc = "List of bucket id regexes to exclude in the storage intelligence plan."]
    pub bucket_id_regexes: ListField<PrimField<String>>,
}
impl BuildStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
    pub fn build(
        self,
    ) -> StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
        StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsEl {
            bucket_id_regexes: self.bucket_id_regexes,
        }
    }
}
pub struct StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
        StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef {
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
pub struct StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
    locations: ListField<PrimField<String>>,
}
impl StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {}
impl ToListMappable
    for StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsEl
{
    type O = BlockAssignable<
        StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
    #[doc = "List of locations."]
    pub locations: ListField<PrimField<String>>,
}
impl BuildStorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
    pub fn build(
        self,
    ) -> StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
        StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsEl {
            locations: self.locations,
        }
    }
}
pub struct StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef {
        StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `locations` after provisioning.\nList of locations."]
    pub fn locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
    bucket_id_regexes: ListField<PrimField<String>>,
}
impl StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {}
impl ToListMappable
    for StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsEl
{
    type O = BlockAssignable<
        StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
    #[doc = "List of bucket id regexes to exclude in the storage intelligence plan."]
    pub bucket_id_regexes: ListField<PrimField<String>>,
}
impl BuildStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
    pub fn build(
        self,
    ) -> StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
        StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsEl {
            bucket_id_regexes: self.bucket_id_regexes,
        }
    }
}
pub struct StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
        StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef {
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
pub struct StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
    locations: ListField<PrimField<String>>,
}
impl StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {}
impl ToListMappable
    for StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsEl
{
    type O = BlockAssignable<
        StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
    #[doc = "List of locations."]
    pub locations: ListField<PrimField<String>>,
}
impl BuildStorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
    pub fn build(
        self,
    ) -> StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
        StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsEl {
            locations: self.locations,
        }
    }
}
pub struct StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef {
        StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `locations` after provisioning.\nList of locations."]
    pub fn locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
}
#[derive(Serialize, Default)]
struct StorageControlFolderIntelligenceConfigFilterElDynamic {
    excluded_cloud_storage_buckets: Option<
        DynamicBlock<StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsEl>,
    >,
    excluded_cloud_storage_locations: Option<
        DynamicBlock<StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsEl>,
    >,
    included_cloud_storage_buckets: Option<
        DynamicBlock<StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsEl>,
    >,
    included_cloud_storage_locations: Option<
        DynamicBlock<StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsEl>,
    >,
}
#[derive(Serialize)]
pub struct StorageControlFolderIntelligenceConfigFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_cloud_storage_buckets:
        Option<Vec<StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_cloud_storage_locations:
        Option<Vec<StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    included_cloud_storage_buckets:
        Option<Vec<StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    included_cloud_storage_locations:
        Option<Vec<StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsEl>>,
    dynamic: StorageControlFolderIntelligenceConfigFilterElDynamic,
}
impl StorageControlFolderIntelligenceConfigFilterEl {
    #[doc = "Set the field `excluded_cloud_storage_buckets`.\n"]
    pub fn set_excluded_cloud_storage_buckets(
        mut self,
        v: impl Into<
            BlockAssignable<
                StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsEl,
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
                StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsEl,
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
                StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsEl,
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
                StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsEl,
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
impl ToListMappable for StorageControlFolderIntelligenceConfigFilterEl {
    type O = BlockAssignable<StorageControlFolderIntelligenceConfigFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageControlFolderIntelligenceConfigFilterEl {}
impl BuildStorageControlFolderIntelligenceConfigFilterEl {
    pub fn build(self) -> StorageControlFolderIntelligenceConfigFilterEl {
        StorageControlFolderIntelligenceConfigFilterEl {
            excluded_cloud_storage_buckets: core::default::Default::default(),
            excluded_cloud_storage_locations: core::default::Default::default(),
            included_cloud_storage_buckets: core::default::Default::default(),
            included_cloud_storage_locations: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct StorageControlFolderIntelligenceConfigFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageControlFolderIntelligenceConfigFilterElRef {
    fn new(shared: StackShared, base: String) -> StorageControlFolderIntelligenceConfigFilterElRef {
        StorageControlFolderIntelligenceConfigFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageControlFolderIntelligenceConfigFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `excluded_cloud_storage_buckets` after provisioning.\n"]
    pub fn excluded_cloud_storage_buckets(
        &self,
    ) -> ListRef<StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageBucketsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.excluded_cloud_storage_buckets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `excluded_cloud_storage_locations` after provisioning.\n"]
    pub fn excluded_cloud_storage_locations(
        &self,
    ) -> ListRef<StorageControlFolderIntelligenceConfigFilterElExcludedCloudStorageLocationsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.excluded_cloud_storage_locations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `included_cloud_storage_buckets` after provisioning.\n"]
    pub fn included_cloud_storage_buckets(
        &self,
    ) -> ListRef<StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageBucketsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.included_cloud_storage_buckets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `included_cloud_storage_locations` after provisioning.\n"]
    pub fn included_cloud_storage_locations(
        &self,
    ) -> ListRef<StorageControlFolderIntelligenceConfigFilterElIncludedCloudStorageLocationsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.included_cloud_storage_locations", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageControlFolderIntelligenceConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl StorageControlFolderIntelligenceConfigTimeoutsEl {
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
impl ToListMappable for StorageControlFolderIntelligenceConfigTimeoutsEl {
    type O = BlockAssignable<StorageControlFolderIntelligenceConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageControlFolderIntelligenceConfigTimeoutsEl {}
impl BuildStorageControlFolderIntelligenceConfigTimeoutsEl {
    pub fn build(self) -> StorageControlFolderIntelligenceConfigTimeoutsEl {
        StorageControlFolderIntelligenceConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct StorageControlFolderIntelligenceConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageControlFolderIntelligenceConfigTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageControlFolderIntelligenceConfigTimeoutsElRef {
        StorageControlFolderIntelligenceConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageControlFolderIntelligenceConfigTimeoutsElRef {
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
struct StorageControlFolderIntelligenceConfigDynamic {
    filter: Option<DynamicBlock<StorageControlFolderIntelligenceConfigFilterEl>>,
}
