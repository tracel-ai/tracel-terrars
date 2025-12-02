use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct QuicksightRefreshScheduleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_account_id: Option<PrimField<String>>,
    data_set_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    schedule_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<Vec<QuicksightRefreshScheduleScheduleEl>>,
    dynamic: QuicksightRefreshScheduleDynamic,
}
struct QuicksightRefreshSchedule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<QuicksightRefreshScheduleData>,
}
#[derive(Clone)]
pub struct QuicksightRefreshSchedule(Rc<QuicksightRefreshSchedule_>);
impl QuicksightRefreshSchedule {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
    #[doc = "Set the field `aws_account_id`.\n"]
    pub fn set_aws_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().aws_account_id = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(
        self,
        v: impl Into<BlockAssignable<QuicksightRefreshScheduleScheduleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schedule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schedule = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_set_id` after provisioning.\n"]
    pub fn data_set_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_set_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schedule_id` after provisioning.\n"]
    pub fn schedule_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schedule_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> ListRef<QuicksightRefreshScheduleScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schedule", self.extract_ref()),
        )
    }
}
impl Referable for QuicksightRefreshSchedule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for QuicksightRefreshSchedule {}
impl ToListMappable for QuicksightRefreshSchedule {
    type O = ListRef<QuicksightRefreshScheduleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for QuicksightRefreshSchedule_ {
    fn extract_resource_type(&self) -> String {
        "aws_quicksight_refresh_schedule".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildQuicksightRefreshSchedule {
    pub tf_id: String,
    #[doc = ""]
    pub data_set_id: PrimField<String>,
    #[doc = ""]
    pub schedule_id: PrimField<String>,
}
impl BuildQuicksightRefreshSchedule {
    pub fn build(self, stack: &mut Stack) -> QuicksightRefreshSchedule {
        let out = QuicksightRefreshSchedule(Rc::new(QuicksightRefreshSchedule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(QuicksightRefreshScheduleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                aws_account_id: core::default::Default::default(),
                data_set_id: self.data_set_id,
                region: core::default::Default::default(),
                schedule_id: self.schedule_id,
                schedule: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct QuicksightRefreshScheduleRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightRefreshScheduleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl QuicksightRefreshScheduleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_set_id` after provisioning.\n"]
    pub fn data_set_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_set_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schedule_id` after provisioning.\n"]
    pub fn schedule_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schedule_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> ListRef<QuicksightRefreshScheduleScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schedule", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightRefreshScheduleScheduleElScheduleFrequencyElRefreshOnDayEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day_of_month: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    day_of_week: Option<PrimField<String>>,
}
impl QuicksightRefreshScheduleScheduleElScheduleFrequencyElRefreshOnDayEl {
    #[doc = "Set the field `day_of_month`.\n"]
    pub fn set_day_of_month(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.day_of_month = Some(v.into());
        self
    }
    #[doc = "Set the field `day_of_week`.\n"]
    pub fn set_day_of_week(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.day_of_week = Some(v.into());
        self
    }
}
impl ToListMappable for QuicksightRefreshScheduleScheduleElScheduleFrequencyElRefreshOnDayEl {
    type O = BlockAssignable<QuicksightRefreshScheduleScheduleElScheduleFrequencyElRefreshOnDayEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightRefreshScheduleScheduleElScheduleFrequencyElRefreshOnDayEl {}
impl BuildQuicksightRefreshScheduleScheduleElScheduleFrequencyElRefreshOnDayEl {
    pub fn build(self) -> QuicksightRefreshScheduleScheduleElScheduleFrequencyElRefreshOnDayEl {
        QuicksightRefreshScheduleScheduleElScheduleFrequencyElRefreshOnDayEl {
            day_of_month: core::default::Default::default(),
            day_of_week: core::default::Default::default(),
        }
    }
}
pub struct QuicksightRefreshScheduleScheduleElScheduleFrequencyElRefreshOnDayElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightRefreshScheduleScheduleElScheduleFrequencyElRefreshOnDayElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightRefreshScheduleScheduleElScheduleFrequencyElRefreshOnDayElRef {
        QuicksightRefreshScheduleScheduleElScheduleFrequencyElRefreshOnDayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightRefreshScheduleScheduleElScheduleFrequencyElRefreshOnDayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day_of_month` after provisioning.\n"]
    pub fn day_of_month(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day_of_month", self.base))
    }
    #[doc = "Get a reference to the value of field `day_of_week` after provisioning.\n"]
    pub fn day_of_week(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }
}
#[derive(Serialize, Default)]
struct QuicksightRefreshScheduleScheduleElScheduleFrequencyElDynamic {
    refresh_on_day:
        Option<DynamicBlock<QuicksightRefreshScheduleScheduleElScheduleFrequencyElRefreshOnDayEl>>,
}
#[derive(Serialize)]
pub struct QuicksightRefreshScheduleScheduleElScheduleFrequencyEl {
    interval: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_of_the_day: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timezone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_on_day:
        Option<Vec<QuicksightRefreshScheduleScheduleElScheduleFrequencyElRefreshOnDayEl>>,
    dynamic: QuicksightRefreshScheduleScheduleElScheduleFrequencyElDynamic,
}
impl QuicksightRefreshScheduleScheduleElScheduleFrequencyEl {
    #[doc = "Set the field `time_of_the_day`.\n"]
    pub fn set_time_of_the_day(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time_of_the_day = Some(v.into());
        self
    }
    #[doc = "Set the field `timezone`.\n"]
    pub fn set_timezone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timezone = Some(v.into());
        self
    }
    #[doc = "Set the field `refresh_on_day`.\n"]
    pub fn set_refresh_on_day(
        mut self,
        v: impl Into<
            BlockAssignable<QuicksightRefreshScheduleScheduleElScheduleFrequencyElRefreshOnDayEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.refresh_on_day = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.refresh_on_day = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for QuicksightRefreshScheduleScheduleElScheduleFrequencyEl {
    type O = BlockAssignable<QuicksightRefreshScheduleScheduleElScheduleFrequencyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightRefreshScheduleScheduleElScheduleFrequencyEl {
    #[doc = ""]
    pub interval: PrimField<String>,
}
impl BuildQuicksightRefreshScheduleScheduleElScheduleFrequencyEl {
    pub fn build(self) -> QuicksightRefreshScheduleScheduleElScheduleFrequencyEl {
        QuicksightRefreshScheduleScheduleElScheduleFrequencyEl {
            interval: self.interval,
            time_of_the_day: core::default::Default::default(),
            timezone: core::default::Default::default(),
            refresh_on_day: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightRefreshScheduleScheduleElScheduleFrequencyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightRefreshScheduleScheduleElScheduleFrequencyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightRefreshScheduleScheduleElScheduleFrequencyElRef {
        QuicksightRefreshScheduleScheduleElScheduleFrequencyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightRefreshScheduleScheduleElScheduleFrequencyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }
    #[doc = "Get a reference to the value of field `time_of_the_day` after provisioning.\n"]
    pub fn time_of_the_day(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_of_the_day", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.base))
    }
    #[doc = "Get a reference to the value of field `refresh_on_day` after provisioning.\n"]
    pub fn refresh_on_day(
        &self,
    ) -> ListRef<QuicksightRefreshScheduleScheduleElScheduleFrequencyElRefreshOnDayElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.refresh_on_day", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct QuicksightRefreshScheduleScheduleElDynamic {
    schedule_frequency:
        Option<DynamicBlock<QuicksightRefreshScheduleScheduleElScheduleFrequencyEl>>,
}
#[derive(Serialize)]
pub struct QuicksightRefreshScheduleScheduleEl {
    refresh_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_after_date_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_frequency: Option<Vec<QuicksightRefreshScheduleScheduleElScheduleFrequencyEl>>,
    dynamic: QuicksightRefreshScheduleScheduleElDynamic,
}
impl QuicksightRefreshScheduleScheduleEl {
    #[doc = "Set the field `start_after_date_time`.\n"]
    pub fn set_start_after_date_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_after_date_time = Some(v.into());
        self
    }
    #[doc = "Set the field `schedule_frequency`.\n"]
    pub fn set_schedule_frequency(
        mut self,
        v: impl Into<BlockAssignable<QuicksightRefreshScheduleScheduleElScheduleFrequencyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schedule_frequency = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schedule_frequency = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for QuicksightRefreshScheduleScheduleEl {
    type O = BlockAssignable<QuicksightRefreshScheduleScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightRefreshScheduleScheduleEl {
    #[doc = ""]
    pub refresh_type: PrimField<String>,
}
impl BuildQuicksightRefreshScheduleScheduleEl {
    pub fn build(self) -> QuicksightRefreshScheduleScheduleEl {
        QuicksightRefreshScheduleScheduleEl {
            refresh_type: self.refresh_type,
            start_after_date_time: core::default::Default::default(),
            schedule_frequency: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightRefreshScheduleScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightRefreshScheduleScheduleElRef {
    fn new(shared: StackShared, base: String) -> QuicksightRefreshScheduleScheduleElRef {
        QuicksightRefreshScheduleScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightRefreshScheduleScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `refresh_type` after provisioning.\n"]
    pub fn refresh_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.refresh_type", self.base))
    }
    #[doc = "Get a reference to the value of field `start_after_date_time` after provisioning.\n"]
    pub fn start_after_date_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_after_date_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `schedule_frequency` after provisioning.\n"]
    pub fn schedule_frequency(
        &self,
    ) -> ListRef<QuicksightRefreshScheduleScheduleElScheduleFrequencyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schedule_frequency", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct QuicksightRefreshScheduleDynamic {
    schedule: Option<DynamicBlock<QuicksightRefreshScheduleScheduleEl>>,
}
