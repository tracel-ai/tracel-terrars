use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ChronicleReferenceListData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    description: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance: PrimField<String>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    reference_list_id: PrimField<String>,
    syntax_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entries: Option<Vec<ChronicleReferenceListEntriesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope_info: Option<Vec<ChronicleReferenceListScopeInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ChronicleReferenceListTimeoutsEl>,
    dynamic: ChronicleReferenceListDynamic,
}
struct ChronicleReferenceList_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ChronicleReferenceListData>,
}
#[derive(Clone)]
pub struct ChronicleReferenceList(Rc<ChronicleReferenceList_>);
impl ChronicleReferenceList {
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
    #[doc = "Set the field `entries`.\n"]
    pub fn set_entries(
        self,
        v: impl Into<BlockAssignable<ChronicleReferenceListEntriesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().entries = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.entries = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scope_info`.\n"]
    pub fn set_scope_info(
        self,
        v: impl Into<BlockAssignable<ChronicleReferenceListScopeInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scope_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scope_info = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ChronicleReferenceListTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nRequired. A user-provided description of the reference list."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nOutput only. The unique display name of the reference list."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nOutput only. The resource name of the reference list.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/referenceLists/{reference_list}"]
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
    #[doc = "Get a reference to the value of field `reference_list_id` after provisioning.\nRequired. The ID to use for the reference list. This is also the display name for\nthe reference list. It must satisfy the following requirements:\n- Starts with letter.\n- Contains only letters, numbers and underscore.\n- Has length < 256.\n- Must be unique."]
    pub fn reference_list_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reference_list_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `revision_create_time` after provisioning.\nOutput only. The timestamp when the reference list was last updated."]
    pub fn revision_create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision_create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_associations_count` after provisioning.\nOutput only. The count of self-authored rules using the reference list."]
    pub fn rule_associations_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_associations_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\nOutput only. The resource names for the associated self-authored Rules that use this\nreference list.\nThis is returned only when the view is REFERENCE_LIST_VIEW_FULL."]
    pub fn rules(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `syntax_type` after provisioning.\nPossible values:\nREFERENCE_LIST_SYNTAX_TYPE_PLAIN_TEXT_STRING\nREFERENCE_LIST_SYNTAX_TYPE_REGEX\nREFERENCE_LIST_SYNTAX_TYPE_CIDR"]
    pub fn syntax_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.syntax_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entries` after provisioning.\n"]
    pub fn entries(&self) -> ListRef<ChronicleReferenceListEntriesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.entries", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scope_info` after provisioning.\n"]
    pub fn scope_info(&self) -> ListRef<ChronicleReferenceListScopeInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scope_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleReferenceListTimeoutsElRef {
        ChronicleReferenceListTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ChronicleReferenceList {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ChronicleReferenceList {}
impl ToListMappable for ChronicleReferenceList {
    type O = ListRef<ChronicleReferenceListRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ChronicleReferenceList_ {
    fn extract_resource_type(&self) -> String {
        "google_chronicle_reference_list".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildChronicleReferenceList {
    pub tf_id: String,
    #[doc = "Required. A user-provided description of the reference list."]
    pub description: PrimField<String>,
    #[doc = "The unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub instance: PrimField<String>,
    #[doc = "The location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub location: PrimField<String>,
    #[doc = "Required. The ID to use for the reference list. This is also the display name for\nthe reference list. It must satisfy the following requirements:\n- Starts with letter.\n- Contains only letters, numbers and underscore.\n- Has length < 256.\n- Must be unique."]
    pub reference_list_id: PrimField<String>,
    #[doc = "Possible values:\nREFERENCE_LIST_SYNTAX_TYPE_PLAIN_TEXT_STRING\nREFERENCE_LIST_SYNTAX_TYPE_REGEX\nREFERENCE_LIST_SYNTAX_TYPE_CIDR"]
    pub syntax_type: PrimField<String>,
}
impl BuildChronicleReferenceList {
    pub fn build(self, stack: &mut Stack) -> ChronicleReferenceList {
        let out = ChronicleReferenceList(Rc::new(ChronicleReferenceList_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ChronicleReferenceListData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: self.description,
                id: core::default::Default::default(),
                instance: self.instance,
                location: self.location,
                project: core::default::Default::default(),
                reference_list_id: self.reference_list_id,
                syntax_type: self.syntax_type,
                entries: core::default::Default::default(),
                scope_info: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ChronicleReferenceListRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleReferenceListRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ChronicleReferenceListRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nRequired. A user-provided description of the reference list."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nOutput only. The unique display name of the reference list."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nOutput only. The resource name of the reference list.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/referenceLists/{reference_list}"]
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
    #[doc = "Get a reference to the value of field `reference_list_id` after provisioning.\nRequired. The ID to use for the reference list. This is also the display name for\nthe reference list. It must satisfy the following requirements:\n- Starts with letter.\n- Contains only letters, numbers and underscore.\n- Has length < 256.\n- Must be unique."]
    pub fn reference_list_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reference_list_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `revision_create_time` after provisioning.\nOutput only. The timestamp when the reference list was last updated."]
    pub fn revision_create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision_create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_associations_count` after provisioning.\nOutput only. The count of self-authored rules using the reference list."]
    pub fn rule_associations_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_associations_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\nOutput only. The resource names for the associated self-authored Rules that use this\nreference list.\nThis is returned only when the view is REFERENCE_LIST_VIEW_FULL."]
    pub fn rules(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `syntax_type` after provisioning.\nPossible values:\nREFERENCE_LIST_SYNTAX_TYPE_PLAIN_TEXT_STRING\nREFERENCE_LIST_SYNTAX_TYPE_REGEX\nREFERENCE_LIST_SYNTAX_TYPE_CIDR"]
    pub fn syntax_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.syntax_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entries` after provisioning.\n"]
    pub fn entries(&self) -> ListRef<ChronicleReferenceListEntriesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.entries", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scope_info` after provisioning.\n"]
    pub fn scope_info(&self) -> ListRef<ChronicleReferenceListScopeInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scope_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleReferenceListTimeoutsElRef {
        ChronicleReferenceListTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleReferenceListEntriesEl {
    value: PrimField<String>,
}
impl ChronicleReferenceListEntriesEl {}
impl ToListMappable for ChronicleReferenceListEntriesEl {
    type O = BlockAssignable<ChronicleReferenceListEntriesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleReferenceListEntriesEl {
    #[doc = "Required. The value of the entry. Maximum length is 512 characters."]
    pub value: PrimField<String>,
}
impl BuildChronicleReferenceListEntriesEl {
    pub fn build(self) -> ChronicleReferenceListEntriesEl {
        ChronicleReferenceListEntriesEl { value: self.value }
    }
}
pub struct ChronicleReferenceListEntriesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleReferenceListEntriesElRef {
    fn new(shared: StackShared, base: String) -> ChronicleReferenceListEntriesElRef {
        ChronicleReferenceListEntriesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleReferenceListEntriesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nRequired. The value of the entry. Maximum length is 512 characters."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleReferenceListScopeInfoElReferenceListScopeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    scope_names: Option<ListField<PrimField<String>>>,
}
impl ChronicleReferenceListScopeInfoElReferenceListScopeEl {
    #[doc = "Set the field `scope_names`.\nOptional. The list of scope names of the reference list. The scope names should be\nfull resource names and should be of the format:\n\"projects/{project}/locations/{location}/instances/{instance}/dataAccessScopes/{scope_name}\"."]
    pub fn set_scope_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.scope_names = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleReferenceListScopeInfoElReferenceListScopeEl {
    type O = BlockAssignable<ChronicleReferenceListScopeInfoElReferenceListScopeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleReferenceListScopeInfoElReferenceListScopeEl {}
impl BuildChronicleReferenceListScopeInfoElReferenceListScopeEl {
    pub fn build(self) -> ChronicleReferenceListScopeInfoElReferenceListScopeEl {
        ChronicleReferenceListScopeInfoElReferenceListScopeEl {
            scope_names: core::default::Default::default(),
        }
    }
}
pub struct ChronicleReferenceListScopeInfoElReferenceListScopeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleReferenceListScopeInfoElReferenceListScopeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleReferenceListScopeInfoElReferenceListScopeElRef {
        ChronicleReferenceListScopeInfoElReferenceListScopeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleReferenceListScopeInfoElReferenceListScopeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scope_names` after provisioning.\nOptional. The list of scope names of the reference list. The scope names should be\nfull resource names and should be of the format:\n\"projects/{project}/locations/{location}/instances/{instance}/dataAccessScopes/{scope_name}\"."]
    pub fn scope_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.scope_names", self.base))
    }
}
#[derive(Serialize, Default)]
struct ChronicleReferenceListScopeInfoElDynamic {
    reference_list_scope:
        Option<DynamicBlock<ChronicleReferenceListScopeInfoElReferenceListScopeEl>>,
}
#[derive(Serialize)]
pub struct ChronicleReferenceListScopeInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    reference_list_scope: Option<Vec<ChronicleReferenceListScopeInfoElReferenceListScopeEl>>,
    dynamic: ChronicleReferenceListScopeInfoElDynamic,
}
impl ChronicleReferenceListScopeInfoEl {
    #[doc = "Set the field `reference_list_scope`.\n"]
    pub fn set_reference_list_scope(
        mut self,
        v: impl Into<BlockAssignable<ChronicleReferenceListScopeInfoElReferenceListScopeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.reference_list_scope = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.reference_list_scope = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ChronicleReferenceListScopeInfoEl {
    type O = BlockAssignable<ChronicleReferenceListScopeInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleReferenceListScopeInfoEl {}
impl BuildChronicleReferenceListScopeInfoEl {
    pub fn build(self) -> ChronicleReferenceListScopeInfoEl {
        ChronicleReferenceListScopeInfoEl {
            reference_list_scope: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ChronicleReferenceListScopeInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleReferenceListScopeInfoElRef {
    fn new(shared: StackShared, base: String) -> ChronicleReferenceListScopeInfoElRef {
        ChronicleReferenceListScopeInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleReferenceListScopeInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `reference_list_scope` after provisioning.\n"]
    pub fn reference_list_scope(
        &self,
    ) -> ListRef<ChronicleReferenceListScopeInfoElReferenceListScopeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reference_list_scope", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleReferenceListTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ChronicleReferenceListTimeoutsEl {
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
impl ToListMappable for ChronicleReferenceListTimeoutsEl {
    type O = BlockAssignable<ChronicleReferenceListTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleReferenceListTimeoutsEl {}
impl BuildChronicleReferenceListTimeoutsEl {
    pub fn build(self) -> ChronicleReferenceListTimeoutsEl {
        ChronicleReferenceListTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ChronicleReferenceListTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleReferenceListTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ChronicleReferenceListTimeoutsElRef {
        ChronicleReferenceListTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleReferenceListTimeoutsElRef {
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
struct ChronicleReferenceListDynamic {
    entries: Option<DynamicBlock<ChronicleReferenceListEntriesEl>>,
    scope_info: Option<DynamicBlock<ChronicleReferenceListScopeInfoEl>>,
}
