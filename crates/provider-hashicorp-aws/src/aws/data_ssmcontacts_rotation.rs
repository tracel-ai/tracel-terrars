use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataSsmcontactsRotationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataSsmcontactsRotation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSsmcontactsRotationData>,
}

#[derive(Clone)]
pub struct DataSsmcontactsRotation(Rc<DataSsmcontactsRotation_>);

impl DataSsmcontactsRotation {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
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

    #[doc = "Get a reference to the value of field `recurrence` after provisioning.\n"]
    pub fn recurrence(&self) -> ListRef<DataSsmcontactsRotationRecurrenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.recurrence", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `time_zone_id` after provisioning.\n"]
    pub fn time_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_zone_id", self.extract_ref()),
        )
    }
}

impl Referable for DataSsmcontactsRotation {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataSsmcontactsRotation {}

impl ToListMappable for DataSsmcontactsRotation {
    type O = ListRef<DataSsmcontactsRotationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSsmcontactsRotation_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ssmcontacts_rotation".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSsmcontactsRotation {
    pub tf_id: String,
    #[doc = ""]
    pub arn: PrimField<String>,
}

impl BuildDataSsmcontactsRotation {
    pub fn build(self, stack: &mut Stack) -> DataSsmcontactsRotation {
        let out = DataSsmcontactsRotation(Rc::new(DataSsmcontactsRotation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSsmcontactsRotationData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                arn: self.arn,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSsmcontactsRotationRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmcontactsRotationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataSsmcontactsRotationRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
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

    #[doc = "Get a reference to the value of field `recurrence` after provisioning.\n"]
    pub fn recurrence(&self) -> ListRef<DataSsmcontactsRotationRecurrenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.recurrence", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `time_zone_id` after provisioning.\n"]
    pub fn time_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_zone_id", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataSsmcontactsRotationRecurrenceElDailySettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hour_of_day: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minute_of_hour: Option<PrimField<f64>>,
}

impl DataSsmcontactsRotationRecurrenceElDailySettingsEl {
    #[doc = "Set the field `hour_of_day`.\n"]
    pub fn set_hour_of_day(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hour_of_day = Some(v.into());
        self
    }

    #[doc = "Set the field `minute_of_hour`.\n"]
    pub fn set_minute_of_hour(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minute_of_hour = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsmcontactsRotationRecurrenceElDailySettingsEl {
    type O = BlockAssignable<DataSsmcontactsRotationRecurrenceElDailySettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsRotationRecurrenceElDailySettingsEl {}

impl BuildDataSsmcontactsRotationRecurrenceElDailySettingsEl {
    pub fn build(self) -> DataSsmcontactsRotationRecurrenceElDailySettingsEl {
        DataSsmcontactsRotationRecurrenceElDailySettingsEl {
            hour_of_day: core::default::Default::default(),
            minute_of_hour: core::default::Default::default(),
        }
    }
}

pub struct DataSsmcontactsRotationRecurrenceElDailySettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmcontactsRotationRecurrenceElDailySettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSsmcontactsRotationRecurrenceElDailySettingsElRef {
        DataSsmcontactsRotationRecurrenceElDailySettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsRotationRecurrenceElDailySettingsElRef {
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
pub struct DataSsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hour_of_day: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minute_of_hour: Option<PrimField<f64>>,
}

impl DataSsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeEl {
    #[doc = "Set the field `hour_of_day`.\n"]
    pub fn set_hour_of_day(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hour_of_day = Some(v.into());
        self
    }

    #[doc = "Set the field `minute_of_hour`.\n"]
    pub fn set_minute_of_hour(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minute_of_hour = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeEl {
    type O = BlockAssignable<DataSsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeEl {}

impl BuildDataSsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeEl {
    pub fn build(self) -> DataSsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeEl {
        DataSsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeEl {
            hour_of_day: core::default::Default::default(),
            minute_of_hour: core::default::Default::default(),
        }
    }
}

pub struct DataSsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeElRef {
        DataSsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeElRef {
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
pub struct DataSsmcontactsRotationRecurrenceElMonthlySettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day_of_month: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hand_off_time:
        Option<ListField<DataSsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeEl>>,
}

impl DataSsmcontactsRotationRecurrenceElMonthlySettingsEl {
    #[doc = "Set the field `day_of_month`.\n"]
    pub fn set_day_of_month(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.day_of_month = Some(v.into());
        self
    }

    #[doc = "Set the field `hand_off_time`.\n"]
    pub fn set_hand_off_time(
        mut self,
        v: impl Into<ListField<DataSsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeEl>>,
    ) -> Self {
        self.hand_off_time = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsmcontactsRotationRecurrenceElMonthlySettingsEl {
    type O = BlockAssignable<DataSsmcontactsRotationRecurrenceElMonthlySettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsRotationRecurrenceElMonthlySettingsEl {}

impl BuildDataSsmcontactsRotationRecurrenceElMonthlySettingsEl {
    pub fn build(self) -> DataSsmcontactsRotationRecurrenceElMonthlySettingsEl {
        DataSsmcontactsRotationRecurrenceElMonthlySettingsEl {
            day_of_month: core::default::Default::default(),
            hand_off_time: core::default::Default::default(),
        }
    }
}

pub struct DataSsmcontactsRotationRecurrenceElMonthlySettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmcontactsRotationRecurrenceElMonthlySettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSsmcontactsRotationRecurrenceElMonthlySettingsElRef {
        DataSsmcontactsRotationRecurrenceElMonthlySettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsRotationRecurrenceElMonthlySettingsElRef {
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
    ) -> ListRef<DataSsmcontactsRotationRecurrenceElMonthlySettingsElHandOffTimeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hand_off_time", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hour_of_day: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minute_of_hour: Option<PrimField<f64>>,
}

impl DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndEl {
    #[doc = "Set the field `hour_of_day`.\n"]
    pub fn set_hour_of_day(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hour_of_day = Some(v.into());
        self
    }

    #[doc = "Set the field `minute_of_hour`.\n"]
    pub fn set_minute_of_hour(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minute_of_hour = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndEl {
    type O =
        BlockAssignable<DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndEl {}

impl BuildDataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndEl {
    pub fn build(self) -> DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndEl {
        DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndEl {
            hour_of_day: core::default::Default::default(),
            minute_of_hour: core::default::Default::default(),
        }
    }
}

pub struct DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndElRef {
        DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndElRef {
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
pub struct DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hour_of_day: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minute_of_hour: Option<PrimField<f64>>,
}

impl DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartEl {
    #[doc = "Set the field `hour_of_day`.\n"]
    pub fn set_hour_of_day(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hour_of_day = Some(v.into());
        self
    }

    #[doc = "Set the field `minute_of_hour`.\n"]
    pub fn set_minute_of_hour(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minute_of_hour = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartEl {
    type O =
        BlockAssignable<DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartEl {}

impl BuildDataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartEl {
    pub fn build(
        self,
    ) -> DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartEl {
        DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartEl {
            hour_of_day: core::default::Default::default(),
            minute_of_hour: core::default::Default::default(),
        }
    }
}

pub struct DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartElRef {
        DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartElRef {
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
pub struct DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<ListField<DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<
        ListField<DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartEl>,
    >,
}

impl DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(
        mut self,
        v: impl Into<ListField<DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndEl>>,
    ) -> Self {
        self.end = Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(
        mut self,
        v: impl Into<
            ListField<DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartEl>,
        >,
    ) -> Self {
        self.start = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesEl {
    type O = BlockAssignable<DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesEl {}

impl BuildDataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesEl {
    pub fn build(self) -> DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesEl {
        DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
        }
    }
}

pub struct DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElRef {
        DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(
        &self,
    ) -> ListRef<DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElEndElRef> {
        ListRef::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(
        &self,
    ) -> ListRef<DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElStartElRef> {
        ListRef::new(self.shared().clone(), format!("{}.start", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmcontactsRotationRecurrenceElShiftCoveragesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    coverage_times:
        Option<ListField<DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    map_block_key: Option<PrimField<String>>,
}

impl DataSsmcontactsRotationRecurrenceElShiftCoveragesEl {
    #[doc = "Set the field `coverage_times`.\n"]
    pub fn set_coverage_times(
        mut self,
        v: impl Into<ListField<DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesEl>>,
    ) -> Self {
        self.coverage_times = Some(v.into());
        self
    }

    #[doc = "Set the field `map_block_key`.\n"]
    pub fn set_map_block_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.map_block_key = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsmcontactsRotationRecurrenceElShiftCoveragesEl {
    type O = BlockAssignable<DataSsmcontactsRotationRecurrenceElShiftCoveragesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsRotationRecurrenceElShiftCoveragesEl {}

impl BuildDataSsmcontactsRotationRecurrenceElShiftCoveragesEl {
    pub fn build(self) -> DataSsmcontactsRotationRecurrenceElShiftCoveragesEl {
        DataSsmcontactsRotationRecurrenceElShiftCoveragesEl {
            coverage_times: core::default::Default::default(),
            map_block_key: core::default::Default::default(),
        }
    }
}

pub struct DataSsmcontactsRotationRecurrenceElShiftCoveragesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmcontactsRotationRecurrenceElShiftCoveragesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSsmcontactsRotationRecurrenceElShiftCoveragesElRef {
        DataSsmcontactsRotationRecurrenceElShiftCoveragesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsRotationRecurrenceElShiftCoveragesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `coverage_times` after provisioning.\n"]
    pub fn coverage_times(
        &self,
    ) -> ListRef<DataSsmcontactsRotationRecurrenceElShiftCoveragesElCoverageTimesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.coverage_times", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `map_block_key` after provisioning.\n"]
    pub fn map_block_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.map_block_key", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataSsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hour_of_day: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minute_of_hour: Option<PrimField<f64>>,
}

impl DataSsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeEl {
    #[doc = "Set the field `hour_of_day`.\n"]
    pub fn set_hour_of_day(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hour_of_day = Some(v.into());
        self
    }

    #[doc = "Set the field `minute_of_hour`.\n"]
    pub fn set_minute_of_hour(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minute_of_hour = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeEl {
    type O = BlockAssignable<DataSsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeEl {}

impl BuildDataSsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeEl {
    pub fn build(self) -> DataSsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeEl {
        DataSsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeEl {
            hour_of_day: core::default::Default::default(),
            minute_of_hour: core::default::Default::default(),
        }
    }
}

pub struct DataSsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeElRef {
        DataSsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeElRef {
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
pub struct DataSsmcontactsRotationRecurrenceElWeeklySettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day_of_week: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hand_off_time:
        Option<ListField<DataSsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeEl>>,
}

impl DataSsmcontactsRotationRecurrenceElWeeklySettingsEl {
    #[doc = "Set the field `day_of_week`.\n"]
    pub fn set_day_of_week(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.day_of_week = Some(v.into());
        self
    }

    #[doc = "Set the field `hand_off_time`.\n"]
    pub fn set_hand_off_time(
        mut self,
        v: impl Into<ListField<DataSsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeEl>>,
    ) -> Self {
        self.hand_off_time = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsmcontactsRotationRecurrenceElWeeklySettingsEl {
    type O = BlockAssignable<DataSsmcontactsRotationRecurrenceElWeeklySettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsRotationRecurrenceElWeeklySettingsEl {}

impl BuildDataSsmcontactsRotationRecurrenceElWeeklySettingsEl {
    pub fn build(self) -> DataSsmcontactsRotationRecurrenceElWeeklySettingsEl {
        DataSsmcontactsRotationRecurrenceElWeeklySettingsEl {
            day_of_week: core::default::Default::default(),
            hand_off_time: core::default::Default::default(),
        }
    }
}

pub struct DataSsmcontactsRotationRecurrenceElWeeklySettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmcontactsRotationRecurrenceElWeeklySettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSsmcontactsRotationRecurrenceElWeeklySettingsElRef {
        DataSsmcontactsRotationRecurrenceElWeeklySettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsRotationRecurrenceElWeeklySettingsElRef {
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
    ) -> ListRef<DataSsmcontactsRotationRecurrenceElWeeklySettingsElHandOffTimeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hand_off_time", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataSsmcontactsRotationRecurrenceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    daily_settings: Option<ListField<DataSsmcontactsRotationRecurrenceElDailySettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monthly_settings: Option<ListField<DataSsmcontactsRotationRecurrenceElMonthlySettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number_of_on_calls: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recurrence_multiplier: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shift_coverages: Option<ListField<DataSsmcontactsRotationRecurrenceElShiftCoveragesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_settings: Option<ListField<DataSsmcontactsRotationRecurrenceElWeeklySettingsEl>>,
}

impl DataSsmcontactsRotationRecurrenceEl {
    #[doc = "Set the field `daily_settings`.\n"]
    pub fn set_daily_settings(
        mut self,
        v: impl Into<ListField<DataSsmcontactsRotationRecurrenceElDailySettingsEl>>,
    ) -> Self {
        self.daily_settings = Some(v.into());
        self
    }

    #[doc = "Set the field `monthly_settings`.\n"]
    pub fn set_monthly_settings(
        mut self,
        v: impl Into<ListField<DataSsmcontactsRotationRecurrenceElMonthlySettingsEl>>,
    ) -> Self {
        self.monthly_settings = Some(v.into());
        self
    }

    #[doc = "Set the field `number_of_on_calls`.\n"]
    pub fn set_number_of_on_calls(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.number_of_on_calls = Some(v.into());
        self
    }

    #[doc = "Set the field `recurrence_multiplier`.\n"]
    pub fn set_recurrence_multiplier(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.recurrence_multiplier = Some(v.into());
        self
    }

    #[doc = "Set the field `shift_coverages`.\n"]
    pub fn set_shift_coverages(
        mut self,
        v: impl Into<ListField<DataSsmcontactsRotationRecurrenceElShiftCoveragesEl>>,
    ) -> Self {
        self.shift_coverages = Some(v.into());
        self
    }

    #[doc = "Set the field `weekly_settings`.\n"]
    pub fn set_weekly_settings(
        mut self,
        v: impl Into<ListField<DataSsmcontactsRotationRecurrenceElWeeklySettingsEl>>,
    ) -> Self {
        self.weekly_settings = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsmcontactsRotationRecurrenceEl {
    type O = BlockAssignable<DataSsmcontactsRotationRecurrenceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsRotationRecurrenceEl {}

impl BuildDataSsmcontactsRotationRecurrenceEl {
    pub fn build(self) -> DataSsmcontactsRotationRecurrenceEl {
        DataSsmcontactsRotationRecurrenceEl {
            daily_settings: core::default::Default::default(),
            monthly_settings: core::default::Default::default(),
            number_of_on_calls: core::default::Default::default(),
            recurrence_multiplier: core::default::Default::default(),
            shift_coverages: core::default::Default::default(),
            weekly_settings: core::default::Default::default(),
        }
    }
}

pub struct DataSsmcontactsRotationRecurrenceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmcontactsRotationRecurrenceElRef {
    fn new(shared: StackShared, base: String) -> DataSsmcontactsRotationRecurrenceElRef {
        DataSsmcontactsRotationRecurrenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsRotationRecurrenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `daily_settings` after provisioning.\n"]
    pub fn daily_settings(&self) -> ListRef<DataSsmcontactsRotationRecurrenceElDailySettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.daily_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `monthly_settings` after provisioning.\n"]
    pub fn monthly_settings(
        &self,
    ) -> ListRef<DataSsmcontactsRotationRecurrenceElMonthlySettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.monthly_settings", self.base),
        )
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

    #[doc = "Get a reference to the value of field `shift_coverages` after provisioning.\n"]
    pub fn shift_coverages(
        &self,
    ) -> ListRef<DataSsmcontactsRotationRecurrenceElShiftCoveragesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.shift_coverages", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `weekly_settings` after provisioning.\n"]
    pub fn weekly_settings(
        &self,
    ) -> ListRef<DataSsmcontactsRotationRecurrenceElWeeklySettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.weekly_settings", self.base),
        )
    }
}
