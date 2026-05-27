use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataVmwareengineUpgradesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    parent: PrimField<String>,
}
struct DataVmwareengineUpgrades_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataVmwareengineUpgradesData>,
}
#[derive(Clone)]
pub struct DataVmwareengineUpgrades(Rc<DataVmwareengineUpgrades_>);
impl DataVmwareengineUpgrades {
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
    #[doc = "Set the field `name`.\nThe resource name of the specific Upgrade to retrieve. If provided, the 'upgrades' list will contain only this upgrade."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the specific Upgrade to retrieve. If provided, the 'upgrades' list will contain only this upgrade."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the private cloud for which upgrades will be listed.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `upgrades` after provisioning.\nA list of VMware Engine upgrades. Contains one element if 'name' is specified in the arguments, otherwise all upgrades for the private cloud."]
    pub fn upgrades(&self) -> ListRef<DataVmwareengineUpgradesUpgradesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upgrades", self.extract_ref()),
        )
    }
}
impl Referable for DataVmwareengineUpgrades {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataVmwareengineUpgrades {}
impl ToListMappable for DataVmwareengineUpgrades {
    type O = ListRef<DataVmwareengineUpgradesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataVmwareengineUpgrades_ {
    fn extract_datasource_type(&self) -> String {
        "google_vmwareengine_upgrades".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataVmwareengineUpgrades {
    pub tf_id: String,
    #[doc = "The resource name of the private cloud for which upgrades will be listed.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub parent: PrimField<String>,
}
impl BuildDataVmwareengineUpgrades {
    pub fn build(self, stack: &mut Stack) -> DataVmwareengineUpgrades {
        let out = DataVmwareengineUpgrades(Rc::new(DataVmwareengineUpgrades_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataVmwareengineUpgradesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                parent: self.parent,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataVmwareengineUpgradesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineUpgradesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataVmwareengineUpgradesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the specific Upgrade to retrieve. If provided, the 'upgrades' list will contain only this upgrade."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the private cloud for which upgrades will be listed.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `upgrades` after provisioning.\nA list of VMware Engine upgrades. Contains one element if 'name' is specified in the arguments, otherwise all upgrades for the private cloud."]
    pub fn upgrades(&self) -> ListRef<DataVmwareengineUpgradesUpgradesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upgrades", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataVmwareengineUpgradesUpgradesElComponentUpgradesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    component_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl DataVmwareengineUpgradesUpgradesElComponentUpgradesEl {
    #[doc = "Set the field `component_type`.\n"]
    pub fn set_component_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.component_type = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for DataVmwareengineUpgradesUpgradesElComponentUpgradesEl {
    type O = BlockAssignable<DataVmwareengineUpgradesUpgradesElComponentUpgradesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVmwareengineUpgradesUpgradesElComponentUpgradesEl {}
impl BuildDataVmwareengineUpgradesUpgradesElComponentUpgradesEl {
    pub fn build(self) -> DataVmwareengineUpgradesUpgradesElComponentUpgradesEl {
        DataVmwareengineUpgradesUpgradesElComponentUpgradesEl {
            component_type: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct DataVmwareengineUpgradesUpgradesElComponentUpgradesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineUpgradesUpgradesElComponentUpgradesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataVmwareengineUpgradesUpgradesElComponentUpgradesElRef {
        DataVmwareengineUpgradesUpgradesElComponentUpgradesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVmwareengineUpgradesUpgradesElComponentUpgradesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `component_type` after provisioning.\n"]
    pub fn component_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.component_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElEndTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
}
impl DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElEndTimeEl {
    #[doc = "Set the field `hours`.\n"]
    pub fn set_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hours = Some(v.into());
        self
    }
    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minutes = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElEndTimeEl
{
    type O = BlockAssignable<
        DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElEndTimeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElEndTimeEl
{}
impl BuildDataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElEndTimeEl {
    pub fn build(
        self,
    ) -> DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElEndTimeEl
    {
        DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElEndTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
        }
    }
}
pub struct DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElEndTimeElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElEndTimeElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElEndTimeElRef
    {
        DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElEndTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElEndTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours` after provisioning.\n"]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
}
#[derive(Serialize)]
pub struct DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElStartTimeEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
}
impl DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElStartTimeEl {
    #[doc = "Set the field `hours`.\n"]
    pub fn set_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hours = Some(v.into());
        self
    }
    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minutes = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElStartTimeEl
{
    type O = BlockAssignable<
        DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElStartTimeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElStartTimeEl
{}
impl
    BuildDataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElStartTimeEl
{
    pub fn build(
        self,
    ) -> DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElStartTimeEl
    {
        DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElStartTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
        }
    }
}
pub struct DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElStartTimeElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElStartTimeElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElStartTimeElRef
    {
        DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElStartTimeElRef { shared : shared , base : base . to_string () , }
    }
}
impl DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElStartTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours` after provisioning.\n"]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
}
#[derive(Serialize)]
pub struct DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsEl { # [serde (skip_serializing_if = "Option::is_none")] end_day : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] end_time : Option < ListField < DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElEndTimeEl > > , # [serde (skip_serializing_if = "Option::is_none")] start_day : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] start_time : Option < ListField < DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElStartTimeEl > > , }
impl DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsEl {
    #[doc = "Set the field `end_day`.\n"]
    pub fn set_end_day(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_day = Some(v.into());
        self
    }
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(
        mut self,
        v : impl Into < ListField < DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElEndTimeEl > >,
    ) -> Self {
        self.end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `start_day`.\n"]
    pub fn set_start_day(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_day = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v : impl Into < ListField < DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElStartTimeEl > >,
    ) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsEl
{
    type O = BlockAssignable<
        DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsEl {}
impl BuildDataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsEl {
    pub fn build(
        self,
    ) -> DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsEl {
        DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsEl {
            end_day: core::default::Default::default(),
            end_time: core::default::Default::default(),
            start_day: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElRef {
        DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_day` after provisioning.\n"]
    pub fn end_day(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_day", self.base))
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(
        &self,
    ) -> ListRef<
        DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElEndTimeElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `start_day` after provisioning.\n"]
    pub fn start_day(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_day", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]    pub fn start_time (& self) -> ListRef < DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElStartTimeElRef >{
        ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElRescheduleDateRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElRescheduleDateRangeEl {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElRescheduleDateRangeEl
{
    type O = BlockAssignable<
        DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElRescheduleDateRangeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVmwareengineUpgradesUpgradesElScheduleElConstraintsElRescheduleDateRangeEl {}
impl BuildDataVmwareengineUpgradesUpgradesElScheduleElConstraintsElRescheduleDateRangeEl {
    pub fn build(
        self,
    ) -> DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElRescheduleDateRangeEl {
        DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElRescheduleDateRangeEl {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElRescheduleDateRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElRescheduleDateRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElRescheduleDateRangeElRef {
        DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElRescheduleDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElRescheduleDateRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataVmwareengineUpgradesUpgradesElScheduleElConstraintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disallowed_intervals: Option<
        ListField<DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_hours_day: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_hours_week: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reschedule_date_range: Option<
        ListField<DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElRescheduleDateRangeEl>,
    >,
}
impl DataVmwareengineUpgradesUpgradesElScheduleElConstraintsEl {
    #[doc = "Set the field `disallowed_intervals`.\n"]
    pub fn set_disallowed_intervals(
        mut self,
        v: impl Into<
            ListField<
                DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsEl,
            >,
        >,
    ) -> Self {
        self.disallowed_intervals = Some(v.into());
        self
    }
    #[doc = "Set the field `min_hours_day`.\n"]
    pub fn set_min_hours_day(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_hours_day = Some(v.into());
        self
    }
    #[doc = "Set the field `min_hours_week`.\n"]
    pub fn set_min_hours_week(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_hours_week = Some(v.into());
        self
    }
    #[doc = "Set the field `reschedule_date_range`.\n"]
    pub fn set_reschedule_date_range(
        mut self,
        v: impl Into<
            ListField<
                DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElRescheduleDateRangeEl,
            >,
        >,
    ) -> Self {
        self.reschedule_date_range = Some(v.into());
        self
    }
}
impl ToListMappable for DataVmwareengineUpgradesUpgradesElScheduleElConstraintsEl {
    type O = BlockAssignable<DataVmwareengineUpgradesUpgradesElScheduleElConstraintsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVmwareengineUpgradesUpgradesElScheduleElConstraintsEl {}
impl BuildDataVmwareengineUpgradesUpgradesElScheduleElConstraintsEl {
    pub fn build(self) -> DataVmwareengineUpgradesUpgradesElScheduleElConstraintsEl {
        DataVmwareengineUpgradesUpgradesElScheduleElConstraintsEl {
            disallowed_intervals: core::default::Default::default(),
            min_hours_day: core::default::Default::default(),
            min_hours_week: core::default::Default::default(),
            reschedule_date_range: core::default::Default::default(),
        }
    }
}
pub struct DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElRef {
        DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disallowed_intervals` after provisioning.\n"]
    pub fn disallowed_intervals(
        &self,
    ) -> ListRef<DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElDisallowedIntervalsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disallowed_intervals", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_hours_day` after provisioning.\n"]
    pub fn min_hours_day(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_hours_day", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_hours_week` after provisioning.\n"]
    pub fn min_hours_week(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_hours_week", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `reschedule_date_range` after provisioning.\n"]
    pub fn reschedule_date_range(
        &self,
    ) -> ListRef<DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElRescheduleDateRangeElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reschedule_date_range", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataVmwareengineUpgradesUpgradesElScheduleElEditWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl DataVmwareengineUpgradesUpgradesElScheduleElEditWindowEl {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataVmwareengineUpgradesUpgradesElScheduleElEditWindowEl {
    type O = BlockAssignable<DataVmwareengineUpgradesUpgradesElScheduleElEditWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVmwareengineUpgradesUpgradesElScheduleElEditWindowEl {}
impl BuildDataVmwareengineUpgradesUpgradesElScheduleElEditWindowEl {
    pub fn build(self) -> DataVmwareengineUpgradesUpgradesElScheduleElEditWindowEl {
        DataVmwareengineUpgradesUpgradesElScheduleElEditWindowEl {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DataVmwareengineUpgradesUpgradesElScheduleElEditWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineUpgradesUpgradesElScheduleElEditWindowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataVmwareengineUpgradesUpgradesElScheduleElEditWindowElRef {
        DataVmwareengineUpgradesUpgradesElScheduleElEditWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVmwareengineUpgradesUpgradesElScheduleElEditWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsElStartTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
}
impl DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsElStartTimeEl {
    #[doc = "Set the field `hours`.\n"]
    pub fn set_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hours = Some(v.into());
        self
    }
    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minutes = Some(v.into());
        self
    }
}
impl ToListMappable for DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsElStartTimeEl {
    type O =
        BlockAssignable<DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsElStartTimeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsElStartTimeEl {}
impl BuildDataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsElStartTimeEl {
    pub fn build(self) -> DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsElStartTimeEl {
        DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsElStartTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
        }
    }
}
pub struct DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsElStartTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsElStartTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsElStartTimeElRef {
        DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsElStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsElStartTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours` after provisioning.\n"]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
}
#[derive(Serialize)]
pub struct DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day_of_week: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time:
        Option<ListField<DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsElStartTimeEl>>,
}
impl DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsEl {
    #[doc = "Set the field `day_of_week`.\n"]
    pub fn set_day_of_week(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.day_of_week = Some(v.into());
        self
    }
    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.duration = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v: impl Into<ListField<DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsElStartTimeEl>>,
    ) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsEl {
    type O = BlockAssignable<DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsEl {}
impl BuildDataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsEl {
    pub fn build(self) -> DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsEl {
        DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsEl {
            day_of_week: core::default::Default::default(),
            duration: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsElRef {
        DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day_of_week` after provisioning.\n"]
    pub fn day_of_week(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }
    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> ListRef<DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsElStartTimeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataVmwareengineUpgradesUpgradesElScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    constraints: Option<ListField<DataVmwareengineUpgradesUpgradesElScheduleElConstraintsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edit_window: Option<ListField<DataVmwareengineUpgradesUpgradesElScheduleElEditWindowEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_editor: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_windows: Option<ListField<DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsEl>>,
}
impl DataVmwareengineUpgradesUpgradesElScheduleEl {
    #[doc = "Set the field `constraints`.\n"]
    pub fn set_constraints(
        mut self,
        v: impl Into<ListField<DataVmwareengineUpgradesUpgradesElScheduleElConstraintsEl>>,
    ) -> Self {
        self.constraints = Some(v.into());
        self
    }
    #[doc = "Set the field `edit_window`.\n"]
    pub fn set_edit_window(
        mut self,
        v: impl Into<ListField<DataVmwareengineUpgradesUpgradesElScheduleElEditWindowEl>>,
    ) -> Self {
        self.edit_window = Some(v.into());
        self
    }
    #[doc = "Set the field `last_editor`.\n"]
    pub fn set_last_editor(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_editor = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
    #[doc = "Set the field `weekly_windows`.\n"]
    pub fn set_weekly_windows(
        mut self,
        v: impl Into<ListField<DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsEl>>,
    ) -> Self {
        self.weekly_windows = Some(v.into());
        self
    }
}
impl ToListMappable for DataVmwareengineUpgradesUpgradesElScheduleEl {
    type O = BlockAssignable<DataVmwareengineUpgradesUpgradesElScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVmwareengineUpgradesUpgradesElScheduleEl {}
impl BuildDataVmwareengineUpgradesUpgradesElScheduleEl {
    pub fn build(self) -> DataVmwareengineUpgradesUpgradesElScheduleEl {
        DataVmwareengineUpgradesUpgradesElScheduleEl {
            constraints: core::default::Default::default(),
            edit_window: core::default::Default::default(),
            last_editor: core::default::Default::default(),
            start_time: core::default::Default::default(),
            weekly_windows: core::default::Default::default(),
        }
    }
}
pub struct DataVmwareengineUpgradesUpgradesElScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineUpgradesUpgradesElScheduleElRef {
    fn new(shared: StackShared, base: String) -> DataVmwareengineUpgradesUpgradesElScheduleElRef {
        DataVmwareengineUpgradesUpgradesElScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVmwareengineUpgradesUpgradesElScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `constraints` after provisioning.\n"]
    pub fn constraints(
        &self,
    ) -> ListRef<DataVmwareengineUpgradesUpgradesElScheduleElConstraintsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.constraints", self.base))
    }
    #[doc = "Get a reference to the value of field `edit_window` after provisioning.\n"]
    pub fn edit_window(
        &self,
    ) -> ListRef<DataVmwareengineUpgradesUpgradesElScheduleElEditWindowElRef> {
        ListRef::new(self.shared().clone(), format!("{}.edit_window", self.base))
    }
    #[doc = "Get a reference to the value of field `last_editor` after provisioning.\n"]
    pub fn last_editor(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_editor", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
    #[doc = "Get a reference to the value of field `weekly_windows` after provisioning.\n"]
    pub fn weekly_windows(
        &self,
    ) -> ListRef<DataVmwareengineUpgradesUpgradesElScheduleElWeeklyWindowsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.weekly_windows", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataVmwareengineUpgradesUpgradesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    component_upgrades: Option<ListField<DataVmwareengineUpgradesUpgradesElComponentUpgradesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    estimated_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<ListField<DataVmwareengineUpgradesUpgradesElScheduleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_version: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataVmwareengineUpgradesUpgradesEl {
    #[doc = "Set the field `component_upgrades`.\n"]
    pub fn set_component_upgrades(
        mut self,
        v: impl Into<ListField<DataVmwareengineUpgradesUpgradesElComponentUpgradesEl>>,
    ) -> Self {
        self.component_upgrades = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `estimated_duration`.\n"]
    pub fn set_estimated_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.estimated_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(
        mut self,
        v: impl Into<ListField<DataVmwareengineUpgradesUpgradesElScheduleEl>>,
    ) -> Self {
        self.schedule = Some(v.into());
        self
    }
    #[doc = "Set the field `start_version`.\n"]
    pub fn set_start_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_version = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `target_version`.\n"]
    pub fn set_target_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_version = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataVmwareengineUpgradesUpgradesEl {
    type O = BlockAssignable<DataVmwareengineUpgradesUpgradesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVmwareengineUpgradesUpgradesEl {}
impl BuildDataVmwareengineUpgradesUpgradesEl {
    pub fn build(self) -> DataVmwareengineUpgradesUpgradesEl {
        DataVmwareengineUpgradesUpgradesEl {
            component_upgrades: core::default::Default::default(),
            description: core::default::Default::default(),
            end_time: core::default::Default::default(),
            estimated_duration: core::default::Default::default(),
            name: core::default::Default::default(),
            schedule: core::default::Default::default(),
            start_version: core::default::Default::default(),
            state: core::default::Default::default(),
            target_version: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataVmwareengineUpgradesUpgradesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineUpgradesUpgradesElRef {
    fn new(shared: StackShared, base: String) -> DataVmwareengineUpgradesUpgradesElRef {
        DataVmwareengineUpgradesUpgradesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVmwareengineUpgradesUpgradesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `component_upgrades` after provisioning.\n"]
    pub fn component_upgrades(
        &self,
    ) -> ListRef<DataVmwareengineUpgradesUpgradesElComponentUpgradesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.component_upgrades", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `estimated_duration` after provisioning.\n"]
    pub fn estimated_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.estimated_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> ListRef<DataVmwareengineUpgradesUpgradesElScheduleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
    #[doc = "Get a reference to the value of field `start_version` after provisioning.\n"]
    pub fn start_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `target_version` after provisioning.\n"]
    pub fn target_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
