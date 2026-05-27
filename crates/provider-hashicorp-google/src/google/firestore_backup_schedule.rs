use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct FirestoreBackupScheduleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    retention: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    daily_recurrence: Option<Vec<FirestoreBackupScheduleDailyRecurrenceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FirestoreBackupScheduleTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_recurrence: Option<Vec<FirestoreBackupScheduleWeeklyRecurrenceEl>>,
    dynamic: FirestoreBackupScheduleDynamic,
}
struct FirestoreBackupSchedule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FirestoreBackupScheduleData>,
}
#[derive(Clone)]
pub struct FirestoreBackupSchedule(Rc<FirestoreBackupSchedule_>);
impl FirestoreBackupSchedule {
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
    #[doc = "Set the field `database`.\nThe Firestore database id. Defaults to '\"(default)\"'."]
    pub fn set_database(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().database = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
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
    #[doc = "Set the field `daily_recurrence`.\n"]
    pub fn set_daily_recurrence(
        self,
        v: impl Into<BlockAssignable<FirestoreBackupScheduleDailyRecurrenceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().daily_recurrence = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.daily_recurrence = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FirestoreBackupScheduleTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `weekly_recurrence`.\n"]
    pub fn set_weekly_recurrence(
        self,
        v: impl Into<BlockAssignable<FirestoreBackupScheduleWeeklyRecurrenceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().weekly_recurrence = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.weekly_recurrence = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\nThe Firestore database id. Defaults to '\"(default)\"'."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique backup schedule identifier across all locations and databases for the given project. Format:\n'projects/{{project}}/databases/{{database}}/backupSchedules/{{backupSchedule}}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retention` after provisioning.\nAt what relative time in the future, compared to its creation time, the backup should be deleted, e.g. keep backups for 7 days.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\".\n\nYou can set this to a value up to 14 weeks."]
    pub fn retention(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `daily_recurrence` after provisioning.\n"]
    pub fn daily_recurrence(&self) -> ListRef<FirestoreBackupScheduleDailyRecurrenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.daily_recurrence", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirestoreBackupScheduleTimeoutsElRef {
        FirestoreBackupScheduleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `weekly_recurrence` after provisioning.\n"]
    pub fn weekly_recurrence(&self) -> ListRef<FirestoreBackupScheduleWeeklyRecurrenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.weekly_recurrence", self.extract_ref()),
        )
    }
}
impl Referable for FirestoreBackupSchedule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for FirestoreBackupSchedule {}
impl ToListMappable for FirestoreBackupSchedule {
    type O = ListRef<FirestoreBackupScheduleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for FirestoreBackupSchedule_ {
    fn extract_resource_type(&self) -> String {
        "google_firestore_backup_schedule".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildFirestoreBackupSchedule {
    pub tf_id: String,
    #[doc = "At what relative time in the future, compared to its creation time, the backup should be deleted, e.g. keep backups for 7 days.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\".\n\nYou can set this to a value up to 14 weeks."]
    pub retention: PrimField<String>,
}
impl BuildFirestoreBackupSchedule {
    pub fn build(self, stack: &mut Stack) -> FirestoreBackupSchedule {
        let out = FirestoreBackupSchedule(Rc::new(FirestoreBackupSchedule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FirestoreBackupScheduleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                database: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                retention: self.retention,
                daily_recurrence: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                weekly_recurrence: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct FirestoreBackupScheduleRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirestoreBackupScheduleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl FirestoreBackupScheduleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\nThe Firestore database id. Defaults to '\"(default)\"'."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique backup schedule identifier across all locations and databases for the given project. Format:\n'projects/{{project}}/databases/{{database}}/backupSchedules/{{backupSchedule}}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retention` after provisioning.\nAt what relative time in the future, compared to its creation time, the backup should be deleted, e.g. keep backups for 7 days.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\".\n\nYou can set this to a value up to 14 weeks."]
    pub fn retention(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `daily_recurrence` after provisioning.\n"]
    pub fn daily_recurrence(&self) -> ListRef<FirestoreBackupScheduleDailyRecurrenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.daily_recurrence", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirestoreBackupScheduleTimeoutsElRef {
        FirestoreBackupScheduleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `weekly_recurrence` after provisioning.\n"]
    pub fn weekly_recurrence(&self) -> ListRef<FirestoreBackupScheduleWeeklyRecurrenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.weekly_recurrence", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct FirestoreBackupScheduleDailyRecurrenceEl {}
impl FirestoreBackupScheduleDailyRecurrenceEl {}
impl ToListMappable for FirestoreBackupScheduleDailyRecurrenceEl {
    type O = BlockAssignable<FirestoreBackupScheduleDailyRecurrenceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirestoreBackupScheduleDailyRecurrenceEl {}
impl BuildFirestoreBackupScheduleDailyRecurrenceEl {
    pub fn build(self) -> FirestoreBackupScheduleDailyRecurrenceEl {
        FirestoreBackupScheduleDailyRecurrenceEl {}
    }
}
pub struct FirestoreBackupScheduleDailyRecurrenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirestoreBackupScheduleDailyRecurrenceElRef {
    fn new(shared: StackShared, base: String) -> FirestoreBackupScheduleDailyRecurrenceElRef {
        FirestoreBackupScheduleDailyRecurrenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirestoreBackupScheduleDailyRecurrenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct FirestoreBackupScheduleTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl FirestoreBackupScheduleTimeoutsEl {
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
impl ToListMappable for FirestoreBackupScheduleTimeoutsEl {
    type O = BlockAssignable<FirestoreBackupScheduleTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirestoreBackupScheduleTimeoutsEl {}
impl BuildFirestoreBackupScheduleTimeoutsEl {
    pub fn build(self) -> FirestoreBackupScheduleTimeoutsEl {
        FirestoreBackupScheduleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct FirestoreBackupScheduleTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirestoreBackupScheduleTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FirestoreBackupScheduleTimeoutsElRef {
        FirestoreBackupScheduleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirestoreBackupScheduleTimeoutsElRef {
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
#[derive(Serialize)]
pub struct FirestoreBackupScheduleWeeklyRecurrenceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<PrimField<String>>,
}
impl FirestoreBackupScheduleWeeklyRecurrenceEl {
    #[doc = "Set the field `day`.\nThe day of week to run. Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn set_day(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.day = Some(v.into());
        self
    }
}
impl ToListMappable for FirestoreBackupScheduleWeeklyRecurrenceEl {
    type O = BlockAssignable<FirestoreBackupScheduleWeeklyRecurrenceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirestoreBackupScheduleWeeklyRecurrenceEl {}
impl BuildFirestoreBackupScheduleWeeklyRecurrenceEl {
    pub fn build(self) -> FirestoreBackupScheduleWeeklyRecurrenceEl {
        FirestoreBackupScheduleWeeklyRecurrenceEl {
            day: core::default::Default::default(),
        }
    }
}
pub struct FirestoreBackupScheduleWeeklyRecurrenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirestoreBackupScheduleWeeklyRecurrenceElRef {
    fn new(shared: StackShared, base: String) -> FirestoreBackupScheduleWeeklyRecurrenceElRef {
        FirestoreBackupScheduleWeeklyRecurrenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirestoreBackupScheduleWeeklyRecurrenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\nThe day of week to run. Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn day(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
}
#[derive(Serialize, Default)]
struct FirestoreBackupScheduleDynamic {
    daily_recurrence: Option<DynamicBlock<FirestoreBackupScheduleDailyRecurrenceEl>>,
    weekly_recurrence: Option<DynamicBlock<FirestoreBackupScheduleWeeklyRecurrenceEl>>,
}
