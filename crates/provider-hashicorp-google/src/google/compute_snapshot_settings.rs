use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeSnapshotSettingsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_location: Option<Vec<ComputeSnapshotSettingsStorageLocationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeSnapshotSettingsTimeoutsEl>,
    dynamic: ComputeSnapshotSettingsDynamic,
}
struct ComputeSnapshotSettings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeSnapshotSettingsData>,
}
#[derive(Clone)]
pub struct ComputeSnapshotSettings(Rc<ComputeSnapshotSettings_>);
impl ComputeSnapshotSettings {
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
    #[doc = "Set the field `storage_location`.\n"]
    pub fn set_storage_location(
        self,
        v: impl Into<BlockAssignable<ComputeSnapshotSettingsStorageLocationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().storage_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.storage_location = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeSnapshotSettingsTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_location` after provisioning.\n"]
    pub fn storage_location(&self) -> ListRef<ComputeSnapshotSettingsStorageLocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeSnapshotSettingsTimeoutsElRef {
        ComputeSnapshotSettingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeSnapshotSettings {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeSnapshotSettings {}
impl ToListMappable for ComputeSnapshotSettings {
    type O = ListRef<ComputeSnapshotSettingsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeSnapshotSettings_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_snapshot_settings".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeSnapshotSettings {
    pub tf_id: String,
}
impl BuildComputeSnapshotSettings {
    pub fn build(self, stack: &mut Stack) -> ComputeSnapshotSettings {
        let out = ComputeSnapshotSettings(Rc::new(ComputeSnapshotSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeSnapshotSettingsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                storage_location: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeSnapshotSettingsRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeSnapshotSettingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeSnapshotSettingsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_location` after provisioning.\n"]
    pub fn storage_location(&self) -> ListRef<ComputeSnapshotSettingsStorageLocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeSnapshotSettingsTimeoutsElRef {
        ComputeSnapshotSettingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeSnapshotSettingsStorageLocationElLocationsEl {
    location: PrimField<String>,
    name: PrimField<String>,
}
impl ComputeSnapshotSettingsStorageLocationElLocationsEl {}
impl ToListMappable for ComputeSnapshotSettingsStorageLocationElLocationsEl {
    type O = BlockAssignable<ComputeSnapshotSettingsStorageLocationElLocationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeSnapshotSettingsStorageLocationElLocationsEl {
    #[doc = ""]
    pub location: PrimField<String>,
    #[doc = "Name of the location. It should be one of the Cloud Storage buckets.\nOnly one location can be specified. (should match location)"]
    pub name: PrimField<String>,
}
impl BuildComputeSnapshotSettingsStorageLocationElLocationsEl {
    pub fn build(self) -> ComputeSnapshotSettingsStorageLocationElLocationsEl {
        ComputeSnapshotSettingsStorageLocationElLocationsEl {
            location: self.location,
            name: self.name,
        }
    }
}
pub struct ComputeSnapshotSettingsStorageLocationElLocationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeSnapshotSettingsStorageLocationElLocationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeSnapshotSettingsStorageLocationElLocationsElRef {
        ComputeSnapshotSettingsStorageLocationElLocationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeSnapshotSettingsStorageLocationElLocationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the location. It should be one of the Cloud Storage buckets.\nOnly one location can be specified. (should match location)"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeSnapshotSettingsStorageLocationElDynamic {
    locations: Option<DynamicBlock<ComputeSnapshotSettingsStorageLocationElLocationsEl>>,
}
#[derive(Serialize)]
pub struct ComputeSnapshotSettingsStorageLocationEl {
    policy: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locations: Option<Vec<ComputeSnapshotSettingsStorageLocationElLocationsEl>>,
    dynamic: ComputeSnapshotSettingsStorageLocationElDynamic,
}
impl ComputeSnapshotSettingsStorageLocationEl {
    #[doc = "Set the field `locations`.\n"]
    pub fn set_locations(
        mut self,
        v: impl Into<BlockAssignable<ComputeSnapshotSettingsStorageLocationElLocationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.locations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.locations = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeSnapshotSettingsStorageLocationEl {
    type O = BlockAssignable<ComputeSnapshotSettingsStorageLocationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeSnapshotSettingsStorageLocationEl {
    #[doc = "The chosen location policy Possible values: [\"NEAREST_MULTI_REGION\", \"LOCAL_REGION\", \"SPECIFIC_LOCATIONS\"]"]
    pub policy: PrimField<String>,
}
impl BuildComputeSnapshotSettingsStorageLocationEl {
    pub fn build(self) -> ComputeSnapshotSettingsStorageLocationEl {
        ComputeSnapshotSettingsStorageLocationEl {
            policy: self.policy,
            locations: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeSnapshotSettingsStorageLocationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeSnapshotSettingsStorageLocationElRef {
    fn new(shared: StackShared, base: String) -> ComputeSnapshotSettingsStorageLocationElRef {
        ComputeSnapshotSettingsStorageLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeSnapshotSettingsStorageLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `policy` after provisioning.\nThe chosen location policy Possible values: [\"NEAREST_MULTI_REGION\", \"LOCAL_REGION\", \"SPECIFIC_LOCATIONS\"]"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeSnapshotSettingsTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeSnapshotSettingsTimeoutsEl {
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
impl ToListMappable for ComputeSnapshotSettingsTimeoutsEl {
    type O = BlockAssignable<ComputeSnapshotSettingsTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeSnapshotSettingsTimeoutsEl {}
impl BuildComputeSnapshotSettingsTimeoutsEl {
    pub fn build(self) -> ComputeSnapshotSettingsTimeoutsEl {
        ComputeSnapshotSettingsTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeSnapshotSettingsTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeSnapshotSettingsTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeSnapshotSettingsTimeoutsElRef {
        ComputeSnapshotSettingsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeSnapshotSettingsTimeoutsElRef {
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
struct ComputeSnapshotSettingsDynamic {
    storage_location: Option<DynamicBlock<ComputeSnapshotSettingsStorageLocationEl>>,
}
