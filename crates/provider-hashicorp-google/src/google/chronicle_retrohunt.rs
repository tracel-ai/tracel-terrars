use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ChronicleRetrohuntData {
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
    instance: PrimField<String>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retrohunt: Option<PrimField<String>>,
    rule: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    process_interval: Option<Vec<ChronicleRetrohuntProcessIntervalEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ChronicleRetrohuntTimeoutsEl>,
    dynamic: ChronicleRetrohuntDynamic,
}
struct ChronicleRetrohunt_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ChronicleRetrohuntData>,
}
#[derive(Clone)]
pub struct ChronicleRetrohunt(Rc<ChronicleRetrohunt_>);
impl ChronicleRetrohunt {
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
    #[doc = "Set the field `retrohunt`.\nThe retrohunt ID of the Retrohunt. A retrohunt is an execution of a Rule over a time range in the past."]
    pub fn set_retrohunt(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().retrohunt = Some(v.into());
        self
    }
    #[doc = "Set the field `process_interval`.\n"]
    pub fn set_process_interval(
        self,
        v: impl Into<BlockAssignable<ChronicleRetrohuntProcessIntervalEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().process_interval = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.process_interval = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ChronicleRetrohuntTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `execution_interval` after provisioning.\nRepresents a time interval, encoded as a Timestamp start (inclusive) and a\nTimestamp end (exclusive).\n\nThe start must be less than or equal to the end.\nWhen the start equals the end, the interval is empty (matches no time).\nWhen both start and end are unspecified, the interval matches any time."]
    pub fn execution_interval(&self) -> ListRef<ChronicleRetrohuntExecutionIntervalElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.execution_interval", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the retrohunt.\nRetrohunt is the child of a rule revision. {rule} in the format below is\nstructured as {rule_id@revision_id}.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/rules/{rule}/retrohunts/{retrohunt}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `progress_percentage` after provisioning.\nOutput only. Percent progress of the retrohunt towards completion, from 0.00 to 100.00."]
    pub fn progress_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.progress_percentage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retrohunt` after provisioning.\nThe retrohunt ID of the Retrohunt. A retrohunt is an execution of a Rule over a time range in the past."]
    pub fn retrohunt(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retrohunt", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule` after provisioning.\nThe Rule ID of the rule."]
    pub fn rule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The state of the retrohunt.\nPossible values:\nRUNNING\nDONE\nCANCELLED\nFAILED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `process_interval` after provisioning.\n"]
    pub fn process_interval(&self) -> ListRef<ChronicleRetrohuntProcessIntervalElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.process_interval", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleRetrohuntTimeoutsElRef {
        ChronicleRetrohuntTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ChronicleRetrohunt {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ChronicleRetrohunt {}
impl ToListMappable for ChronicleRetrohunt {
    type O = ListRef<ChronicleRetrohuntRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ChronicleRetrohunt_ {
    fn extract_resource_type(&self) -> String {
        "google_chronicle_retrohunt".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildChronicleRetrohunt {
    pub tf_id: String,
    #[doc = "The unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub instance: PrimField<String>,
    #[doc = "The location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub location: PrimField<String>,
    #[doc = "The Rule ID of the rule."]
    pub rule: PrimField<String>,
}
impl BuildChronicleRetrohunt {
    pub fn build(self, stack: &mut Stack) -> ChronicleRetrohunt {
        let out = ChronicleRetrohunt(Rc::new(ChronicleRetrohunt_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ChronicleRetrohuntData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                instance: self.instance,
                location: self.location,
                project: core::default::Default::default(),
                retrohunt: core::default::Default::default(),
                rule: self.rule,
                process_interval: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ChronicleRetrohuntRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleRetrohuntRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ChronicleRetrohuntRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `execution_interval` after provisioning.\nRepresents a time interval, encoded as a Timestamp start (inclusive) and a\nTimestamp end (exclusive).\n\nThe start must be less than or equal to the end.\nWhen the start equals the end, the interval is empty (matches no time).\nWhen both start and end are unspecified, the interval matches any time."]
    pub fn execution_interval(&self) -> ListRef<ChronicleRetrohuntExecutionIntervalElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.execution_interval", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the retrohunt.\nRetrohunt is the child of a rule revision. {rule} in the format below is\nstructured as {rule_id@revision_id}.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/rules/{rule}/retrohunts/{retrohunt}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `progress_percentage` after provisioning.\nOutput only. Percent progress of the retrohunt towards completion, from 0.00 to 100.00."]
    pub fn progress_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.progress_percentage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retrohunt` after provisioning.\nThe retrohunt ID of the Retrohunt. A retrohunt is an execution of a Rule over a time range in the past."]
    pub fn retrohunt(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retrohunt", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule` after provisioning.\nThe Rule ID of the rule."]
    pub fn rule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The state of the retrohunt.\nPossible values:\nRUNNING\nDONE\nCANCELLED\nFAILED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `process_interval` after provisioning.\n"]
    pub fn process_interval(&self) -> ListRef<ChronicleRetrohuntProcessIntervalElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.process_interval", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleRetrohuntTimeoutsElRef {
        ChronicleRetrohuntTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleRetrohuntExecutionIntervalEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl ChronicleRetrohuntExecutionIntervalEl {
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
impl ToListMappable for ChronicleRetrohuntExecutionIntervalEl {
    type O = BlockAssignable<ChronicleRetrohuntExecutionIntervalEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleRetrohuntExecutionIntervalEl {}
impl BuildChronicleRetrohuntExecutionIntervalEl {
    pub fn build(self) -> ChronicleRetrohuntExecutionIntervalEl {
        ChronicleRetrohuntExecutionIntervalEl {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct ChronicleRetrohuntExecutionIntervalElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleRetrohuntExecutionIntervalElRef {
    fn new(shared: StackShared, base: String) -> ChronicleRetrohuntExecutionIntervalElRef {
        ChronicleRetrohuntExecutionIntervalElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleRetrohuntExecutionIntervalElRef {
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
pub struct ChronicleRetrohuntProcessIntervalEl {
    end_time: PrimField<String>,
    start_time: PrimField<String>,
}
impl ChronicleRetrohuntProcessIntervalEl {}
impl ToListMappable for ChronicleRetrohuntProcessIntervalEl {
    type O = BlockAssignable<ChronicleRetrohuntProcessIntervalEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleRetrohuntProcessIntervalEl {
    #[doc = "Exclusive end of the interval."]
    pub end_time: PrimField<String>,
    #[doc = "Inclusive start of the interval."]
    pub start_time: PrimField<String>,
}
impl BuildChronicleRetrohuntProcessIntervalEl {
    pub fn build(self) -> ChronicleRetrohuntProcessIntervalEl {
        ChronicleRetrohuntProcessIntervalEl {
            end_time: self.end_time,
            start_time: self.start_time,
        }
    }
}
pub struct ChronicleRetrohuntProcessIntervalElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleRetrohuntProcessIntervalElRef {
    fn new(shared: StackShared, base: String) -> ChronicleRetrohuntProcessIntervalElRef {
        ChronicleRetrohuntProcessIntervalElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleRetrohuntProcessIntervalElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\nExclusive end of the interval."]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\nInclusive start of the interval."]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleRetrohuntTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl ChronicleRetrohuntTimeoutsEl {
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
}
impl ToListMappable for ChronicleRetrohuntTimeoutsEl {
    type O = BlockAssignable<ChronicleRetrohuntTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleRetrohuntTimeoutsEl {}
impl BuildChronicleRetrohuntTimeoutsEl {
    pub fn build(self) -> ChronicleRetrohuntTimeoutsEl {
        ChronicleRetrohuntTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct ChronicleRetrohuntTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleRetrohuntTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ChronicleRetrohuntTimeoutsElRef {
        ChronicleRetrohuntTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleRetrohuntTimeoutsElRef {
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
}
#[derive(Serialize, Default)]
struct ChronicleRetrohuntDynamic {
    process_interval: Option<DynamicBlock<ChronicleRetrohuntProcessIntervalEl>>,
}
