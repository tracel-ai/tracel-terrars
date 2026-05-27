use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeMachineTypesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
struct DataComputeMachineTypes_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeMachineTypesData>,
}
#[derive(Clone)]
pub struct DataComputeMachineTypes(Rc<DataComputeMachineTypes_>);
impl DataComputeMachineTypes {
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
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nProject ID for this request."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\nThe name of the zone for this request."]
    pub fn set_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `machine_types` after provisioning.\nThe list of machine types"]
    pub fn machine_types(&self) -> ListRef<DataComputeMachineTypesMachineTypesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.machine_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject ID for this request."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe name of the zone for this request."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
}
impl Referable for DataComputeMachineTypes {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeMachineTypes {}
impl ToListMappable for DataComputeMachineTypes {
    type O = ListRef<DataComputeMachineTypesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeMachineTypes_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_machine_types".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeMachineTypes {
    pub tf_id: String,
}
impl BuildDataComputeMachineTypes {
    pub fn build(self, stack: &mut Stack) -> DataComputeMachineTypes {
        let out = DataComputeMachineTypes(Rc::new(DataComputeMachineTypes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeMachineTypesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                zone: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeMachineTypesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeMachineTypesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeMachineTypesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `machine_types` after provisioning.\nThe list of machine types"]
    pub fn machine_types(&self) -> ListRef<DataComputeMachineTypesMachineTypesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.machine_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject ID for this request."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe name of the zone for this request."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeMachineTypesMachineTypesElAcceleratorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_accelerator_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_accelerator_type: Option<PrimField<String>>,
}
impl DataComputeMachineTypesMachineTypesElAcceleratorsEl {
    #[doc = "Set the field `guest_accelerator_count`.\n"]
    pub fn set_guest_accelerator_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.guest_accelerator_count = Some(v.into());
        self
    }
    #[doc = "Set the field `guest_accelerator_type`.\n"]
    pub fn set_guest_accelerator_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.guest_accelerator_type = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeMachineTypesMachineTypesElAcceleratorsEl {
    type O = BlockAssignable<DataComputeMachineTypesMachineTypesElAcceleratorsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeMachineTypesMachineTypesElAcceleratorsEl {}
impl BuildDataComputeMachineTypesMachineTypesElAcceleratorsEl {
    pub fn build(self) -> DataComputeMachineTypesMachineTypesElAcceleratorsEl {
        DataComputeMachineTypesMachineTypesElAcceleratorsEl {
            guest_accelerator_count: core::default::Default::default(),
            guest_accelerator_type: core::default::Default::default(),
        }
    }
}
pub struct DataComputeMachineTypesMachineTypesElAcceleratorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeMachineTypesMachineTypesElAcceleratorsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeMachineTypesMachineTypesElAcceleratorsElRef {
        DataComputeMachineTypesMachineTypesElAcceleratorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeMachineTypesMachineTypesElAcceleratorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `guest_accelerator_count` after provisioning.\n"]
    pub fn guest_accelerator_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.guest_accelerator_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `guest_accelerator_type` after provisioning.\n"]
    pub fn guest_accelerator_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.guest_accelerator_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeMachineTypesMachineTypesElDeprecatedEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    replacement: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl DataComputeMachineTypesMachineTypesElDeprecatedEl {
    #[doc = "Set the field `replacement`.\n"]
    pub fn set_replacement(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.replacement = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeMachineTypesMachineTypesElDeprecatedEl {
    type O = BlockAssignable<DataComputeMachineTypesMachineTypesElDeprecatedEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeMachineTypesMachineTypesElDeprecatedEl {}
impl BuildDataComputeMachineTypesMachineTypesElDeprecatedEl {
    pub fn build(self) -> DataComputeMachineTypesMachineTypesElDeprecatedEl {
        DataComputeMachineTypesMachineTypesElDeprecatedEl {
            replacement: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct DataComputeMachineTypesMachineTypesElDeprecatedElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeMachineTypesMachineTypesElDeprecatedElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeMachineTypesMachineTypesElDeprecatedElRef {
        DataComputeMachineTypesMachineTypesElDeprecatedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeMachineTypesMachineTypesElDeprecatedElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `replacement` after provisioning.\n"]
    pub fn replacement(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.replacement", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeMachineTypesMachineTypesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerators: Option<ListField<DataComputeMachineTypesMachineTypesElAcceleratorsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deprecated: Option<SetField<DataComputeMachineTypesMachineTypesElDeprecatedEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_cpus: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_shared_cpus: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_persistent_disks: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_persistent_disks_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_mb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_link: Option<PrimField<String>>,
}
impl DataComputeMachineTypesMachineTypesEl {
    #[doc = "Set the field `accelerators`.\n"]
    pub fn set_accelerators(
        mut self,
        v: impl Into<ListField<DataComputeMachineTypesMachineTypesElAcceleratorsEl>>,
    ) -> Self {
        self.accelerators = Some(v.into());
        self
    }
    #[doc = "Set the field `deprecated`.\n"]
    pub fn set_deprecated(
        mut self,
        v: impl Into<SetField<DataComputeMachineTypesMachineTypesElDeprecatedEl>>,
    ) -> Self {
        self.deprecated = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `guest_cpus`.\n"]
    pub fn set_guest_cpus(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.guest_cpus = Some(v.into());
        self
    }
    #[doc = "Set the field `is_shared_cpus`.\n"]
    pub fn set_is_shared_cpus(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_shared_cpus = Some(v.into());
        self
    }
    #[doc = "Set the field `maximum_persistent_disks`.\n"]
    pub fn set_maximum_persistent_disks(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_persistent_disks = Some(v.into());
        self
    }
    #[doc = "Set the field `maximum_persistent_disks_size_gb`.\n"]
    pub fn set_maximum_persistent_disks_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_persistent_disks_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_mb`.\n"]
    pub fn set_memory_mb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_mb = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `self_link`.\n"]
    pub fn set_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.self_link = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeMachineTypesMachineTypesEl {
    type O = BlockAssignable<DataComputeMachineTypesMachineTypesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeMachineTypesMachineTypesEl {}
impl BuildDataComputeMachineTypesMachineTypesEl {
    pub fn build(self) -> DataComputeMachineTypesMachineTypesEl {
        DataComputeMachineTypesMachineTypesEl {
            accelerators: core::default::Default::default(),
            deprecated: core::default::Default::default(),
            description: core::default::Default::default(),
            guest_cpus: core::default::Default::default(),
            is_shared_cpus: core::default::Default::default(),
            maximum_persistent_disks: core::default::Default::default(),
            maximum_persistent_disks_size_gb: core::default::Default::default(),
            memory_mb: core::default::Default::default(),
            name: core::default::Default::default(),
            self_link: core::default::Default::default(),
        }
    }
}
pub struct DataComputeMachineTypesMachineTypesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeMachineTypesMachineTypesElRef {
    fn new(shared: StackShared, base: String) -> DataComputeMachineTypesMachineTypesElRef {
        DataComputeMachineTypesMachineTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeMachineTypesMachineTypesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerators` after provisioning.\n"]
    pub fn accelerators(&self) -> ListRef<DataComputeMachineTypesMachineTypesElAcceleratorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.accelerators", self.base))
    }
    #[doc = "Get a reference to the value of field `deprecated` after provisioning.\n"]
    pub fn deprecated(&self) -> SetRef<DataComputeMachineTypesMachineTypesElDeprecatedElRef> {
        SetRef::new(self.shared().clone(), format!("{}.deprecated", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `guest_cpus` after provisioning.\n"]
    pub fn guest_cpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.guest_cpus", self.base))
    }
    #[doc = "Get a reference to the value of field `is_shared_cpus` after provisioning.\n"]
    pub fn is_shared_cpus(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_shared_cpus", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maximum_persistent_disks` after provisioning.\n"]
    pub fn maximum_persistent_disks(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_persistent_disks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maximum_persistent_disks_size_gb` after provisioning.\n"]
    pub fn maximum_persistent_disks_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_persistent_disks_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_mb` after provisioning.\n"]
    pub fn memory_mb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_mb", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }
}
