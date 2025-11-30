use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct SsmcontactsRotationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    contact_ids: ListField<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    time_zone_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recurrence: Option<Vec<SsmcontactsRotationRecurrenceEl>>,
    dynamic: SsmcontactsRotationDynamic,
}

struct SsmcontactsRotation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SsmcontactsRotationData>,
}

#[derive(Clone)]
pub struct SsmcontactsRotation(Rc<SsmcontactsRotation_>);

impl SsmcontactsRotation {
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

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().start_time = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `recurrence`.\n"]
    pub fn set_recurrence(
        self,
        v: impl Into<BlockAssignable<SsmcontactsRotationRecurrenceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().recurrence = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.recurrence = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `contact_ids` after provisioning.\n"]
    pub fn contact_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.contact_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `time_zone_id` after provisioning.\n"]
    pub fn time_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_zone_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `recurrence` after provisioning.\n"]
    pub fn recurrence(&self) -> ListRef<SsmcontactsRotationRecurrenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.recurrence", self.extract_ref()),
        )
    }
}

impl Referable for SsmcontactsRotation {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for SsmcontactsRotation {}

impl ToListMappable for SsmcontactsRotation {
    type O = ListRef<SsmcontactsRotationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SsmcontactsRotation_ {
    fn extract_resource_type(&self) -> String {
        "aws_ssmcontacts_rotation".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSsmcontactsRotation {
    pub tf_id: String,
    #[doc = ""]
    pub contact_ids: ListField<PrimField<String>>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub time_zone_id: PrimField<String>,
}

impl BuildSsmcontactsRotation {
    pub fn build(self, stack: &mut Stack) -> SsmcontactsRotation {
        let out = SsmcontactsRotation(Rc::new(SsmcontactsRotation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SsmcontactsRotationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                contact_ids: self.contact_ids,
                name: self.name,
                region: core::default::Default::default(),
                start_time: core::default::Default::default(),
                tags: core::default::Default::default(),
                time_zone_id: self.time_zone_id,
                recurrence: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SsmcontactsRotationRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmcontactsRotationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl SsmcontactsRotationRef {
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

    #[doc = "Get a reference to the value of field `contact_ids` after provisioning.\n"]
    pub fn contact_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.contact_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `time_zone_id` after provisioning.\n"]
    pub fn time_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_zone_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `recurrence` after provisioning.\n"]
    pub fn recurrence(&self) -> ListRef<SsmcontactsRotationRecurrenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.recurrence", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct SsmcontactsRotationRecurrenceElDailySettingsEl {
    hour_of_day: PrimField<f64>,
    minute_of_hour: PrimField<f64>,
}

impl SsmcontactsRotationRecurrenceElDailySettingsEl {}

impl ToListMappable for SsmcontactsRotationRecurrenceElDailySettingsEl {
    type O = BlockAssignable<SsmcontactsRotationRecurrenceElDailySettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmcontactsRotationRecurrenceElDailySettingsEl {
    #[doc = ""]
    pub hour_of_day: PrimField<f64>,
    #[doc = ""]
    pub minute_of_hour: PrimField<f64>,
}

impl BuildSsmcontactsRotationRecurrenceElDailySettingsEl {
    pub fn build(self) -> SsmcontactsRotationRecurrenceElDailySettingsEl {
        SsmcontactsRotationRecurrenceElDailySettingsEl {
            hour_of_day: self.hour_of_day,
            minute_of_hour: self.minute_of_hour,
        }
    }
}

pub struct SsmcontactsRotationRecurrenceElDailySettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmcontactsRotationRecurrenceElDailySettingsElRef {
    fn new(shared: StackShared, base: String) -> SsmcontactsRotationRecurrenceElDailySettingsElRef {
        SsmcontactsRotationRecurrenceElDailySettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmcontactsRotationRecurrenceElDailySettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hour_of_day` after provisioning.\n"]
    pub fn hour_of_day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hour_of_day", self.base))
    }

    #[doc = "Get a reference to the value of field `minute_of_hour` after provisioning.\n"]
    pub fn minute_of_hour(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minute_of_hour", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeEl {
    hour_of_day: PrimField<f64>,
    minute_of_hour: PrimField<f64>,
}

impl SsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeEl {}

impl ToListMappable for SsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeEl {
    type O = BlockAssignable<SsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeEl {
    #[doc = ""]
    pub hour_of_day: PrimField<f64>,
    #[doc = ""]
    pub minute_of_hour: PrimField<f64>,
}

impl BuildSsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeEl {
    pub fn build(self) -> SsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeEl {
        SsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeEl {
            hour_of_day: self.hour_of_day,
            minute_of_hour: self.minute_of_hour,
        }
    }
}

pub struct SsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeElRef {
        SsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hour_of_day` after provisioning.\n"]
    pub fn hour_of_day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hour_of_day", self.base))
    }

    #[doc = "Get a reference to the value of field `minute_of_hour` after provisioning.\n"]
    pub fn minute_of_hour(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minute_of_hour", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SsmcontactsRotationRecurrenceElMonthlySettingsElDynamic {
    hand_off_time:
        Option<DynamicBlock<SsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeEl>>,
}

#[derive(Serialize)]
pub struct SsmcontactsRotationRecurrenceElMonthlySettingsEl {
    day_of_month: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hand_off_time: Option<Vec<SsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeEl>>,
    dynamic: SsmcontactsRotationRecurrenceElMonthlySettingsElDynamic,
}

impl SsmcontactsRotationRecurrenceElMonthlySettingsEl {
    #[doc = "Set the field `hand_off_time`.\n"]
    pub fn set_hand_off_time(
        mut self,
        v: impl Into<BlockAssignable<SsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hand_off_time = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hand_off_time = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SsmcontactsRotationRecurrenceElMonthlySettingsEl {
    type O = BlockAssignable<SsmcontactsRotationRecurrenceElMonthlySettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmcontactsRotationRecurrenceElMonthlySettingsEl {
    #[doc = ""]
    pub day_of_month: PrimField<f64>,
}

impl BuildSsmcontactsRotationRecurrenceElMonthlySettingsEl {
    pub fn build(self) -> SsmcontactsRotationRecurrenceElMonthlySettingsEl {
        SsmcontactsRotationRecurrenceElMonthlySettingsEl {
            day_of_month: self.day_of_month,
            hand_off_time: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SsmcontactsRotationRecurrenceElMonthlySettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmcontactsRotationRecurrenceElMonthlySettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsmcontactsRotationRecurrenceElMonthlySettingsElRef {
        SsmcontactsRotationRecurrenceElMonthlySettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmcontactsRotationRecurrenceElMonthlySettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day_of_month` after provisioning.\n"]
    pub fn day_of_month(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.day_of_month", self.base))
    }

    #[doc = "Get a reference to the value of field `hand_off_time` after provisioning.\n"]
    pub fn hand_off_time(
        &self,
    ) -> ListRef<SsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hand_off_time", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndEl {
    hour_of_day: PrimField<f64>,
    minute_of_hour: PrimField<f64>,
}

impl SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndEl {}

impl ToListMappable for SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndEl {
    type O = BlockAssignable<SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndEl {
    #[doc = ""]
    pub hour_of_day: PrimField<f64>,
    #[doc = ""]
    pub minute_of_hour: PrimField<f64>,
}

impl BuildSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndEl {
    pub fn build(self) -> SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndEl {
        SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndEl {
            hour_of_day: self.hour_of_day,
            minute_of_hour: self.minute_of_hour,
        }
    }
}

pub struct SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndElRef {
        SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hour_of_day` after provisioning.\n"]
    pub fn hour_of_day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hour_of_day", self.base))
    }

    #[doc = "Get a reference to the value of field `minute_of_hour` after provisioning.\n"]
    pub fn minute_of_hour(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minute_of_hour", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartEl {
    hour_of_day: PrimField<f64>,
    minute_of_hour: PrimField<f64>,
}

impl SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartEl {}

impl ToListMappable for SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartEl {
    type O = BlockAssignable<SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartEl {
    #[doc = ""]
    pub hour_of_day: PrimField<f64>,
    #[doc = ""]
    pub minute_of_hour: PrimField<f64>,
}

impl BuildSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartEl {
    pub fn build(self) -> SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartEl {
        SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartEl {
            hour_of_day: self.hour_of_day,
            minute_of_hour: self.minute_of_hour,
        }
    }
}

pub struct SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartElRef {
        SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hour_of_day` after provisioning.\n"]
    pub fn hour_of_day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hour_of_day", self.base))
    }

    #[doc = "Get a reference to the value of field `minute_of_hour` after provisioning.\n"]
    pub fn minute_of_hour(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minute_of_hour", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElDynamic {
    end: Option<DynamicBlock<SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndEl>>,
    start:
        Option<DynamicBlock<SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartEl>>,
}

#[derive(Serialize)]
pub struct SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<Vec<SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<Vec<SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartEl>>,
    dynamic: SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElDynamic,
}

impl SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(
        mut self,
        v: impl Into<
            BlockAssignable<SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.end = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.end = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(
        mut self,
        v: impl Into<
            BlockAssignable<SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.start = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.start = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesEl {
    type O = BlockAssignable<SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesEl {}

impl BuildSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesEl {
    pub fn build(self) -> SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesEl {
        SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElRef {
        SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(
        &self,
    ) -> ListRef<SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndElRef> {
        ListRef::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(
        &self,
    ) -> ListRef<SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartElRef> {
        ListRef::new(self.shared().clone(), format!("{}.start", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsmcontactsRotationRecurrenceElShiftCoveragesElDynamic {
    coverage_times:
        Option<DynamicBlock<SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesEl>>,
}

#[derive(Serialize)]
pub struct SsmcontactsRotationRecurrenceElShiftCoveragesEl {
    map_block_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coverage_times: Option<Vec<SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesEl>>,
    dynamic: SsmcontactsRotationRecurrenceElShiftCoveragesElDynamic,
}

impl SsmcontactsRotationRecurrenceElShiftCoveragesEl {
    #[doc = "Set the field `coverage_times`.\n"]
    pub fn set_coverage_times(
        mut self,
        v: impl Into<BlockAssignable<SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.coverage_times = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.coverage_times = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SsmcontactsRotationRecurrenceElShiftCoveragesEl {
    type O = BlockAssignable<SsmcontactsRotationRecurrenceElShiftCoveragesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmcontactsRotationRecurrenceElShiftCoveragesEl {
    #[doc = ""]
    pub map_block_key: PrimField<String>,
}

impl BuildSsmcontactsRotationRecurrenceElShiftCoveragesEl {
    pub fn build(self) -> SsmcontactsRotationRecurrenceElShiftCoveragesEl {
        SsmcontactsRotationRecurrenceElShiftCoveragesEl {
            map_block_key: self.map_block_key,
            coverage_times: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SsmcontactsRotationRecurrenceElShiftCoveragesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmcontactsRotationRecurrenceElShiftCoveragesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsmcontactsRotationRecurrenceElShiftCoveragesElRef {
        SsmcontactsRotationRecurrenceElShiftCoveragesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmcontactsRotationRecurrenceElShiftCoveragesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `map_block_key` after provisioning.\n"]
    pub fn map_block_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.map_block_key", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `coverage_times` after provisioning.\n"]
    pub fn coverage_times(
        &self,
    ) -> ListRef<SsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.coverage_times", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeEl {
    hour_of_day: PrimField<f64>,
    minute_of_hour: PrimField<f64>,
}

impl SsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeEl {}

impl ToListMappable for SsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeEl {
    type O = BlockAssignable<SsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeEl {
    #[doc = ""]
    pub hour_of_day: PrimField<f64>,
    #[doc = ""]
    pub minute_of_hour: PrimField<f64>,
}

impl BuildSsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeEl {
    pub fn build(self) -> SsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeEl {
        SsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeEl {
            hour_of_day: self.hour_of_day,
            minute_of_hour: self.minute_of_hour,
        }
    }
}

pub struct SsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeElRef {
        SsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hour_of_day` after provisioning.\n"]
    pub fn hour_of_day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hour_of_day", self.base))
    }

    #[doc = "Get a reference to the value of field `minute_of_hour` after provisioning.\n"]
    pub fn minute_of_hour(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minute_of_hour", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SsmcontactsRotationRecurrenceElWeeklySettingsElDynamic {
    hand_off_time:
        Option<DynamicBlock<SsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeEl>>,
}

#[derive(Serialize)]
pub struct SsmcontactsRotationRecurrenceElWeeklySettingsEl {
    day_of_week: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hand_off_time: Option<Vec<SsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeEl>>,
    dynamic: SsmcontactsRotationRecurrenceElWeeklySettingsElDynamic,
}

impl SsmcontactsRotationRecurrenceElWeeklySettingsEl {
    #[doc = "Set the field `hand_off_time`.\n"]
    pub fn set_hand_off_time(
        mut self,
        v: impl Into<BlockAssignable<SsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hand_off_time = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hand_off_time = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SsmcontactsRotationRecurrenceElWeeklySettingsEl {
    type O = BlockAssignable<SsmcontactsRotationRecurrenceElWeeklySettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmcontactsRotationRecurrenceElWeeklySettingsEl {
    #[doc = ""]
    pub day_of_week: PrimField<String>,
}

impl BuildSsmcontactsRotationRecurrenceElWeeklySettingsEl {
    pub fn build(self) -> SsmcontactsRotationRecurrenceElWeeklySettingsEl {
        SsmcontactsRotationRecurrenceElWeeklySettingsEl {
            day_of_week: self.day_of_week,
            hand_off_time: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SsmcontactsRotationRecurrenceElWeeklySettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmcontactsRotationRecurrenceElWeeklySettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsmcontactsRotationRecurrenceElWeeklySettingsElRef {
        SsmcontactsRotationRecurrenceElWeeklySettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmcontactsRotationRecurrenceElWeeklySettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day_of_week` after provisioning.\n"]
    pub fn day_of_week(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }

    #[doc = "Get a reference to the value of field `hand_off_time` after provisioning.\n"]
    pub fn hand_off_time(
        &self,
    ) -> ListRef<SsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hand_off_time", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SsmcontactsRotationRecurrenceElDynamic {
    daily_settings: Option<DynamicBlock<SsmcontactsRotationRecurrenceElDailySettingsEl>>,
    monthly_settings: Option<DynamicBlock<SsmcontactsRotationRecurrenceElMonthlySettingsEl>>,
    shift_coverages: Option<DynamicBlock<SsmcontactsRotationRecurrenceElShiftCoveragesEl>>,
    weekly_settings: Option<DynamicBlock<SsmcontactsRotationRecurrenceElWeeklySettingsEl>>,
}

#[derive(Serialize)]
pub struct SsmcontactsRotationRecurrenceEl {
    number_of_on_calls: PrimField<f64>,
    recurrence_multiplier: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    daily_settings: Option<Vec<SsmcontactsRotationRecurrenceElDailySettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monthly_settings: Option<Vec<SsmcontactsRotationRecurrenceElMonthlySettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shift_coverages: Option<Vec<SsmcontactsRotationRecurrenceElShiftCoveragesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_settings: Option<Vec<SsmcontactsRotationRecurrenceElWeeklySettingsEl>>,
    dynamic: SsmcontactsRotationRecurrenceElDynamic,
}

impl SsmcontactsRotationRecurrenceEl {
    #[doc = "Set the field `daily_settings`.\n"]
    pub fn set_daily_settings(
        mut self,
        v: impl Into<BlockAssignable<SsmcontactsRotationRecurrenceElDailySettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.daily_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.daily_settings = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `monthly_settings`.\n"]
    pub fn set_monthly_settings(
        mut self,
        v: impl Into<BlockAssignable<SsmcontactsRotationRecurrenceElMonthlySettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.monthly_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.monthly_settings = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `shift_coverages`.\n"]
    pub fn set_shift_coverages(
        mut self,
        v: impl Into<BlockAssignable<SsmcontactsRotationRecurrenceElShiftCoveragesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.shift_coverages = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.shift_coverages = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `weekly_settings`.\n"]
    pub fn set_weekly_settings(
        mut self,
        v: impl Into<BlockAssignable<SsmcontactsRotationRecurrenceElWeeklySettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.weekly_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.weekly_settings = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SsmcontactsRotationRecurrenceEl {
    type O = BlockAssignable<SsmcontactsRotationRecurrenceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmcontactsRotationRecurrenceEl {
    #[doc = ""]
    pub number_of_on_calls: PrimField<f64>,
    #[doc = ""]
    pub recurrence_multiplier: PrimField<f64>,
}

impl BuildSsmcontactsRotationRecurrenceEl {
    pub fn build(self) -> SsmcontactsRotationRecurrenceEl {
        SsmcontactsRotationRecurrenceEl {
            number_of_on_calls: self.number_of_on_calls,
            recurrence_multiplier: self.recurrence_multiplier,
            daily_settings: core::default::Default::default(),
            monthly_settings: core::default::Default::default(),
            shift_coverages: core::default::Default::default(),
            weekly_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SsmcontactsRotationRecurrenceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmcontactsRotationRecurrenceElRef {
    fn new(shared: StackShared, base: String) -> SsmcontactsRotationRecurrenceElRef {
        SsmcontactsRotationRecurrenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmcontactsRotationRecurrenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `number_of_on_calls` after provisioning.\n"]
    pub fn number_of_on_calls(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.number_of_on_calls", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `recurrence_multiplier` after provisioning.\n"]
    pub fn recurrence_multiplier(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.recurrence_multiplier", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `daily_settings` after provisioning.\n"]
    pub fn daily_settings(&self) -> ListRef<SsmcontactsRotationRecurrenceElDailySettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.daily_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `monthly_settings` after provisioning.\n"]
    pub fn monthly_settings(&self) -> ListRef<SsmcontactsRotationRecurrenceElMonthlySettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.monthly_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `shift_coverages` after provisioning.\n"]
    pub fn shift_coverages(&self) -> ListRef<SsmcontactsRotationRecurrenceElShiftCoveragesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.shift_coverages", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `weekly_settings` after provisioning.\n"]
    pub fn weekly_settings(&self) -> ListRef<SsmcontactsRotationRecurrenceElWeeklySettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.weekly_settings", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SsmcontactsRotationDynamic {
    recurrence: Option<DynamicBlock<SsmcontactsRotationRecurrenceEl>>,
}
