use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ContactCenterInsightsQaScorecardRevisionData {
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
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    qa_scorecard: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    qa_scorecard_revision_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ContactCenterInsightsQaScorecardRevisionTimeoutsEl>,
}
struct ContactCenterInsightsQaScorecardRevision_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ContactCenterInsightsQaScorecardRevisionData>,
}
#[derive(Clone)]
pub struct ContactCenterInsightsQaScorecardRevision(Rc<ContactCenterInsightsQaScorecardRevision_>);
impl ContactCenterInsightsQaScorecardRevision {
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
    #[doc = "Set the field `qa_scorecard_revision_id`.\nA unique ID for the new QaScorecardRevision. This ID will become the final\ncomponent of the QaScorecardRevision's resource name.\nIf no ID is specified this resource will get the latest revision on the given scorecard."]
    pub fn set_qa_scorecard_revision_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().qa_scorecard_revision_id = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<ContactCenterInsightsQaScorecardRevisionTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `alternate_ids` after provisioning.\nAlternative IDs for this revision of the scorecard, e.g., 'latest'."]
    pub fn alternate_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.alternate_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp that the revision was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the scorecard revision.\nFormat:\nprojects/{project}/locations/{location}/qaScorecards/{qa_scorecard}/revisions/{revision}"]
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
    #[doc = "Get a reference to the value of field `qa_scorecard` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn qa_scorecard(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.qa_scorecard", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `qa_scorecard_revision_id` after provisioning.\nA unique ID for the new QaScorecardRevision. This ID will become the final\ncomponent of the QaScorecardRevision's resource name.\nIf no ID is specified this resource will get the latest revision on the given scorecard."]
    pub fn qa_scorecard_revision_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.qa_scorecard_revision_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot` after provisioning.\nA QaScorecard represents a collection of questions to be scored during\nanalysis."]
    pub fn snapshot(&self) -> ListRef<ContactCenterInsightsQaScorecardRevisionSnapshotElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snapshot", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the scorecard revision, indicating whether it's ready to\nbe used in analysis.\nPossible values:\nEDITABLE\nTRAINING\nTRAINING_FAILED\nREADY\nDELETING\nTRAINING_CANCELLED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContactCenterInsightsQaScorecardRevisionTimeoutsElRef {
        ContactCenterInsightsQaScorecardRevisionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ContactCenterInsightsQaScorecardRevision {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ContactCenterInsightsQaScorecardRevision {}
impl ToListMappable for ContactCenterInsightsQaScorecardRevision {
    type O = ListRef<ContactCenterInsightsQaScorecardRevisionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ContactCenterInsightsQaScorecardRevision_ {
    fn extract_resource_type(&self) -> String {
        "google_contact_center_insights_qa_scorecard_revision".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildContactCenterInsightsQaScorecardRevision {
    pub tf_id: String,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub qa_scorecard: PrimField<String>,
}
impl BuildContactCenterInsightsQaScorecardRevision {
    pub fn build(self, stack: &mut Stack) -> ContactCenterInsightsQaScorecardRevision {
        let out = ContactCenterInsightsQaScorecardRevision(Rc::new(
            ContactCenterInsightsQaScorecardRevision_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(ContactCenterInsightsQaScorecardRevisionData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    id: core::default::Default::default(),
                    location: self.location,
                    project: core::default::Default::default(),
                    qa_scorecard: self.qa_scorecard,
                    qa_scorecard_revision_id: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ContactCenterInsightsQaScorecardRevisionRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactCenterInsightsQaScorecardRevisionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ContactCenterInsightsQaScorecardRevisionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `alternate_ids` after provisioning.\nAlternative IDs for this revision of the scorecard, e.g., 'latest'."]
    pub fn alternate_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.alternate_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp that the revision was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the scorecard revision.\nFormat:\nprojects/{project}/locations/{location}/qaScorecards/{qa_scorecard}/revisions/{revision}"]
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
    #[doc = "Get a reference to the value of field `qa_scorecard` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn qa_scorecard(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.qa_scorecard", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `qa_scorecard_revision_id` after provisioning.\nA unique ID for the new QaScorecardRevision. This ID will become the final\ncomponent of the QaScorecardRevision's resource name.\nIf no ID is specified this resource will get the latest revision on the given scorecard."]
    pub fn qa_scorecard_revision_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.qa_scorecard_revision_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot` after provisioning.\nA QaScorecard represents a collection of questions to be scored during\nanalysis."]
    pub fn snapshot(&self) -> ListRef<ContactCenterInsightsQaScorecardRevisionSnapshotElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snapshot", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the scorecard revision, indicating whether it's ready to\nbe used in analysis.\nPossible values:\nEDITABLE\nTRAINING\nTRAINING_FAILED\nREADY\nDELETING\nTRAINING_CANCELLED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContactCenterInsightsQaScorecardRevisionTimeoutsElRef {
        ContactCenterInsightsQaScorecardRevisionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ContactCenterInsightsQaScorecardRevisionSnapshotEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_default: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl ContactCenterInsightsQaScorecardRevisionSnapshotEl {
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `is_default`.\n"]
    pub fn set_is_default(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_default = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for ContactCenterInsightsQaScorecardRevisionSnapshotEl {
    type O = BlockAssignable<ContactCenterInsightsQaScorecardRevisionSnapshotEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactCenterInsightsQaScorecardRevisionSnapshotEl {}
impl BuildContactCenterInsightsQaScorecardRevisionSnapshotEl {
    pub fn build(self) -> ContactCenterInsightsQaScorecardRevisionSnapshotEl {
        ContactCenterInsightsQaScorecardRevisionSnapshotEl {
            create_time: core::default::Default::default(),
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            is_default: core::default::Default::default(),
            name: core::default::Default::default(),
            source: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct ContactCenterInsightsQaScorecardRevisionSnapshotElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactCenterInsightsQaScorecardRevisionSnapshotElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContactCenterInsightsQaScorecardRevisionSnapshotElRef {
        ContactCenterInsightsQaScorecardRevisionSnapshotElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactCenterInsightsQaScorecardRevisionSnapshotElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `is_default` after provisioning.\n"]
    pub fn is_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_default", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactCenterInsightsQaScorecardRevisionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl ContactCenterInsightsQaScorecardRevisionTimeoutsEl {
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
impl ToListMappable for ContactCenterInsightsQaScorecardRevisionTimeoutsEl {
    type O = BlockAssignable<ContactCenterInsightsQaScorecardRevisionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactCenterInsightsQaScorecardRevisionTimeoutsEl {}
impl BuildContactCenterInsightsQaScorecardRevisionTimeoutsEl {
    pub fn build(self) -> ContactCenterInsightsQaScorecardRevisionTimeoutsEl {
        ContactCenterInsightsQaScorecardRevisionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct ContactCenterInsightsQaScorecardRevisionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactCenterInsightsQaScorecardRevisionTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContactCenterInsightsQaScorecardRevisionTimeoutsElRef {
        ContactCenterInsightsQaScorecardRevisionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactCenterInsightsQaScorecardRevisionTimeoutsElRef {
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
