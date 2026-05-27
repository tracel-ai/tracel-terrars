use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataplexEntryData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entry_group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entry_id: Option<PrimField<String>>,
    entry_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fully_qualified_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_entry: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aspects: Option<Vec<DataplexEntryAspectsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entry_source: Option<Vec<DataplexEntryEntrySourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataplexEntryTimeoutsEl>,
    dynamic: DataplexEntryDynamic,
}
struct DataplexEntry_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataplexEntryData>,
}
#[derive(Clone)]
pub struct DataplexEntry(Rc<DataplexEntry_>);
impl DataplexEntry {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `entry_group_id`.\nThe entry group id of the entry group the entry will be created in."]
    pub fn set_entry_group_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().entry_group_id = Some(v.into());
        self
    }
    #[doc = "Set the field `entry_id`.\nThe entry id of the entry."]
    pub fn set_entry_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().entry_id = Some(v.into());
        self
    }
    #[doc = "Set the field `fully_qualified_name`.\nA name for the entry that can be referenced by an external system. For more information, see https://cloud.google.com/dataplex/docs/fully-qualified-names.\nThe maximum size of the field is 4000 characters."]
    pub fn set_fully_qualified_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().fully_qualified_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe location where entry will be created."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `parent_entry`.\nThe resource name of the parent entry, in the format projects/{project_number}/locations/{locationId}/entryGroups/{entryGroupId}/entries/{entryId}."]
    pub fn set_parent_entry(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent_entry = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `aspects`.\n"]
    pub fn set_aspects(self, v: impl Into<BlockAssignable<DataplexEntryAspectsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().aspects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.aspects = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `entry_source`.\n"]
    pub fn set_entry_source(
        self,
        v: impl Into<BlockAssignable<DataplexEntryEntrySourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().entry_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.entry_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataplexEntryTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the Entry was created in Dataplex."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entry_group_id` after provisioning.\nThe entry group id of the entry group the entry will be created in."]
    pub fn entry_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entry_group_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entry_id` after provisioning.\nThe entry id of the entry."]
    pub fn entry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entry_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entry_type` after provisioning.\nThe relative resource name of the entry type that was used to create this entry, in the format projects/{project_number}/locations/{locationId}/entryTypes/{entryTypeId}."]
    pub fn entry_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entry_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fully_qualified_name` after provisioning.\nA name for the entry that can be referenced by an external system. For more information, see https://cloud.google.com/dataplex/docs/fully-qualified-names.\nThe maximum size of the field is 4000 characters."]
    pub fn fully_qualified_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fully_qualified_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location where entry will be created."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the entry, in the format projects/{project_number}/locations/{locationId}/entryGroups/{entryGroupId}/entries/{entryId}."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent_entry` after provisioning.\nThe resource name of the parent entry, in the format projects/{project_number}/locations/{locationId}/entryGroups/{entryGroupId}/entries/{entryId}."]
    pub fn parent_entry(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_entry", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the entry was last updated in Dataplex."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `aspects` after provisioning.\n"]
    pub fn aspects(&self) -> ListRef<DataplexEntryAspectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.aspects", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entry_source` after provisioning.\n"]
    pub fn entry_source(&self) -> ListRef<DataplexEntryEntrySourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.entry_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataplexEntryTimeoutsElRef {
        DataplexEntryTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataplexEntry {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataplexEntry {}
impl ToListMappable for DataplexEntry {
    type O = ListRef<DataplexEntryRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataplexEntry_ {
    fn extract_resource_type(&self) -> String {
        "google_dataplex_entry".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataplexEntry {
    pub tf_id: String,
    #[doc = "The relative resource name of the entry type that was used to create this entry, in the format projects/{project_number}/locations/{locationId}/entryTypes/{entryTypeId}."]
    pub entry_type: PrimField<String>,
}
impl BuildDataplexEntry {
    pub fn build(self, stack: &mut Stack) -> DataplexEntry {
        let out = DataplexEntry(Rc::new(DataplexEntry_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataplexEntryData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                entry_group_id: core::default::Default::default(),
                entry_id: core::default::Default::default(),
                entry_type: self.entry_type,
                fully_qualified_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                parent_entry: core::default::Default::default(),
                project: core::default::Default::default(),
                aspects: core::default::Default::default(),
                entry_source: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DataplexEntryRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexEntryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataplexEntryRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the Entry was created in Dataplex."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entry_group_id` after provisioning.\nThe entry group id of the entry group the entry will be created in."]
    pub fn entry_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entry_group_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entry_id` after provisioning.\nThe entry id of the entry."]
    pub fn entry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entry_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entry_type` after provisioning.\nThe relative resource name of the entry type that was used to create this entry, in the format projects/{project_number}/locations/{locationId}/entryTypes/{entryTypeId}."]
    pub fn entry_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entry_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fully_qualified_name` after provisioning.\nA name for the entry that can be referenced by an external system. For more information, see https://cloud.google.com/dataplex/docs/fully-qualified-names.\nThe maximum size of the field is 4000 characters."]
    pub fn fully_qualified_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fully_qualified_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location where entry will be created."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the entry, in the format projects/{project_number}/locations/{locationId}/entryGroups/{entryGroupId}/entries/{entryId}."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent_entry` after provisioning.\nThe resource name of the parent entry, in the format projects/{project_number}/locations/{locationId}/entryGroups/{entryGroupId}/entries/{entryId}."]
    pub fn parent_entry(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_entry", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the entry was last updated in Dataplex."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `aspects` after provisioning.\n"]
    pub fn aspects(&self) -> ListRef<DataplexEntryAspectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.aspects", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entry_source` after provisioning.\n"]
    pub fn entry_source(&self) -> ListRef<DataplexEntryEntrySourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.entry_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataplexEntryTimeoutsElRef {
        DataplexEntryTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexEntryAspectsElAspectEl {
    data: PrimField<String>,
}
impl DataplexEntryAspectsElAspectEl {}
impl ToListMappable for DataplexEntryAspectsElAspectEl {
    type O = BlockAssignable<DataplexEntryAspectsElAspectEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexEntryAspectsElAspectEl {
    #[doc = "The content of the aspect in JSON form, according to its aspect type schema. The maximum size of the field is 120KB (encoded as UTF-8)."]
    pub data: PrimField<String>,
}
impl BuildDataplexEntryAspectsElAspectEl {
    pub fn build(self) -> DataplexEntryAspectsElAspectEl {
        DataplexEntryAspectsElAspectEl { data: self.data }
    }
}
pub struct DataplexEntryAspectsElAspectElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexEntryAspectsElAspectElRef {
    fn new(shared: StackShared, base: String) -> DataplexEntryAspectsElAspectElRef {
        DataplexEntryAspectsElAspectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexEntryAspectsElAspectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aspect_type` after provisioning.\nThe resource name of the type used to create this Aspect."]
    pub fn aspect_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aspect_type", self.base))
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the Aspect was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `data` after provisioning.\nThe content of the aspect in JSON form, according to its aspect type schema. The maximum size of the field is 120KB (encoded as UTF-8)."]
    pub fn data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nThe path in the entry under which the aspect is attached."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the Aspect was last modified."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataplexEntryAspectsElDynamic {
    aspect: Option<DynamicBlock<DataplexEntryAspectsElAspectEl>>,
}
#[derive(Serialize)]
pub struct DataplexEntryAspectsEl {
    aspect_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aspect: Option<Vec<DataplexEntryAspectsElAspectEl>>,
    dynamic: DataplexEntryAspectsElDynamic,
}
impl DataplexEntryAspectsEl {
    #[doc = "Set the field `aspect`.\n"]
    pub fn set_aspect(
        mut self,
        v: impl Into<BlockAssignable<DataplexEntryAspectsElAspectEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aspect = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aspect = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataplexEntryAspectsEl {
    type O = BlockAssignable<DataplexEntryAspectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexEntryAspectsEl {
    #[doc = "Depending on how the aspect is attached to the entry, the format of the aspect key can be one of the following:\n\nIf the aspect is attached directly to the entry: {project_number}.{locationId}.{aspectTypeId}\nIf the aspect is attached to an entry's path: {project_number}.{locationId}.{aspectTypeId}@{path}"]
    pub aspect_key: PrimField<String>,
}
impl BuildDataplexEntryAspectsEl {
    pub fn build(self) -> DataplexEntryAspectsEl {
        DataplexEntryAspectsEl {
            aspect_key: self.aspect_key,
            aspect: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataplexEntryAspectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexEntryAspectsElRef {
    fn new(shared: StackShared, base: String) -> DataplexEntryAspectsElRef {
        DataplexEntryAspectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexEntryAspectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aspect_key` after provisioning.\nDepending on how the aspect is attached to the entry, the format of the aspect key can be one of the following:\n\nIf the aspect is attached directly to the entry: {project_number}.{locationId}.{aspectTypeId}\nIf the aspect is attached to an entry's path: {project_number}.{locationId}.{aspectTypeId}@{path}"]
    pub fn aspect_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aspect_key", self.base))
    }
    #[doc = "Get a reference to the value of field `aspect` after provisioning.\n"]
    pub fn aspect(&self) -> ListRef<DataplexEntryAspectsElAspectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aspect", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexEntryEntrySourceElAncestorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataplexEntryEntrySourceElAncestorsEl {
    #[doc = "Set the field `name`.\nThe name of the ancestor resource."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nThe type of the ancestor resource."]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexEntryEntrySourceElAncestorsEl {
    type O = BlockAssignable<DataplexEntryEntrySourceElAncestorsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexEntryEntrySourceElAncestorsEl {}
impl BuildDataplexEntryEntrySourceElAncestorsEl {
    pub fn build(self) -> DataplexEntryEntrySourceElAncestorsEl {
        DataplexEntryEntrySourceElAncestorsEl {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataplexEntryEntrySourceElAncestorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexEntryEntrySourceElAncestorsElRef {
    fn new(shared: StackShared, base: String) -> DataplexEntryEntrySourceElAncestorsElRef {
        DataplexEntryEntrySourceElAncestorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexEntryEntrySourceElAncestorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the ancestor resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the ancestor resource."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataplexEntryEntrySourceElDynamic {
    ancestors: Option<DynamicBlock<DataplexEntryEntrySourceElAncestorsEl>>,
}
#[derive(Serialize)]
pub struct DataplexEntryEntrySourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ancestors: Option<Vec<DataplexEntryEntrySourceElAncestorsEl>>,
    dynamic: DataplexEntryEntrySourceElDynamic,
}
impl DataplexEntryEntrySourceEl {
    #[doc = "Set the field `create_time`.\nThe time when the resource was created in the source system."]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA description of the data resource. Maximum length is 2,000 characters."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nA user-friendly display name. Maximum length is 500 characters."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nUser-defined labels. The maximum size of keys and values is 128 characters each.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `platform`.\nThe platform containing the source system. Maximum length is 64 characters."]
    pub fn set_platform(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.platform = Some(v.into());
        self
    }
    #[doc = "Set the field `resource`.\nThe name of the resource in the source system. Maximum length is 4,000 characters."]
    pub fn set_resource(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource = Some(v.into());
        self
    }
    #[doc = "Set the field `system`.\nThe name of the source system. Maximum length is 64 characters."]
    pub fn set_system(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.system = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\nThe time when the resource was last updated in the source system.\nIf the entry exists in the system and its EntrySource has updateTime populated,\nfurther updates to the EntrySource of the entry must provide incremental updates to its updateTime."]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
    #[doc = "Set the field `ancestors`.\n"]
    pub fn set_ancestors(
        mut self,
        v: impl Into<BlockAssignable<DataplexEntryEntrySourceElAncestorsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ancestors = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ancestors = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataplexEntryEntrySourceEl {
    type O = BlockAssignable<DataplexEntryEntrySourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexEntryEntrySourceEl {}
impl BuildDataplexEntryEntrySourceEl {
    pub fn build(self) -> DataplexEntryEntrySourceEl {
        DataplexEntryEntrySourceEl {
            create_time: core::default::Default::default(),
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            labels: core::default::Default::default(),
            platform: core::default::Default::default(),
            resource: core::default::Default::default(),
            system: core::default::Default::default(),
            update_time: core::default::Default::default(),
            ancestors: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataplexEntryEntrySourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexEntryEntrySourceElRef {
    fn new(shared: StackShared, base: String) -> DataplexEntryEntrySourceElRef {
        DataplexEntryEntrySourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexEntryEntrySourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the resource was created in the source system."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the data resource. Maximum length is 2,000 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA user-friendly display name. Maximum length is 500 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels. The maximum size of keys and values is 128 characters each.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation of the resource in the source system. You can search the entry by this location.\nBy default, this should match the location of the entry group containing this entry.\nA different value allows capturing the source location for data external to Google Cloud."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `platform` after provisioning.\nThe platform containing the source system. Maximum length is 64 characters."]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform", self.base))
    }
    #[doc = "Get a reference to the value of field `resource` after provisioning.\nThe name of the resource in the source system. Maximum length is 4,000 characters."]
    pub fn resource(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }
    #[doc = "Get a reference to the value of field `system` after provisioning.\nThe name of the source system. Maximum length is 64 characters."]
    pub fn system(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.system", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the resource was last updated in the source system.\nIf the entry exists in the system and its EntrySource has updateTime populated,\nfurther updates to the EntrySource of the entry must provide incremental updates to its updateTime."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
    #[doc = "Get a reference to the value of field `ancestors` after provisioning.\n"]
    pub fn ancestors(&self) -> ListRef<DataplexEntryEntrySourceElAncestorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ancestors", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexEntryTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DataplexEntryTimeoutsEl {
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
impl ToListMappable for DataplexEntryTimeoutsEl {
    type O = BlockAssignable<DataplexEntryTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexEntryTimeoutsEl {}
impl BuildDataplexEntryTimeoutsEl {
    pub fn build(self) -> DataplexEntryTimeoutsEl {
        DataplexEntryTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DataplexEntryTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexEntryTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataplexEntryTimeoutsElRef {
        DataplexEntryTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexEntryTimeoutsElRef {
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
struct DataplexEntryDynamic {
    aspects: Option<DynamicBlock<DataplexEntryAspectsEl>>,
    entry_source: Option<DynamicBlock<DataplexEntryEntrySourceEl>>,
}
