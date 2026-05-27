use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataplexEntryLinkData {
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
    entry_group_id: PrimField<String>,
    entry_link_id: PrimField<String>,
    entry_link_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aspects: Option<Vec<DataplexEntryLinkAspectsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entry_references: Option<Vec<DataplexEntryLinkEntryReferencesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataplexEntryLinkTimeoutsEl>,
    dynamic: DataplexEntryLinkDynamic,
}
struct DataplexEntryLink_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataplexEntryLinkData>,
}
#[derive(Clone)]
pub struct DataplexEntryLink(Rc<DataplexEntryLink_>);
impl DataplexEntryLink {
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
    #[doc = "Set the field `aspects`.\n"]
    pub fn set_aspects(self, v: impl Into<BlockAssignable<DataplexEntryLinkAspectsEl>>) -> Self {
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
    #[doc = "Set the field `entry_references`.\n"]
    pub fn set_entry_references(
        self,
        v: impl Into<BlockAssignable<DataplexEntryLinkEntryReferencesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().entry_references = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.entry_references = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataplexEntryLinkTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the Entry Link was created."]
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
    #[doc = "Get a reference to the value of field `entry_group_id` after provisioning.\nThe id of the entry group this entry link is in."]
    pub fn entry_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entry_group_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entry_link_id` after provisioning.\nThe id of the entry link to create."]
    pub fn entry_link_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entry_link_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entry_link_type` after provisioning.\nRelative resource name of the Entry Link Type used to create this Entry Link. For example:\nprojects/dataplex-types/locations/global/entryLinkTypes/definition"]
    pub fn entry_link_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entry_link_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the entry."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the Entry Link, of the form:\nprojects/{project_id_or_number}/locations/{location_id}/entryGroups/{entry_group_id}/entryLinks/{entry_link_id}"]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the Entry Link was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `aspects` after provisioning.\n"]
    pub fn aspects(&self) -> ListRef<DataplexEntryLinkAspectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.aspects", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entry_references` after provisioning.\n"]
    pub fn entry_references(&self) -> ListRef<DataplexEntryLinkEntryReferencesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.entry_references", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataplexEntryLinkTimeoutsElRef {
        DataplexEntryLinkTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataplexEntryLink {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataplexEntryLink {}
impl ToListMappable for DataplexEntryLink {
    type O = ListRef<DataplexEntryLinkRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataplexEntryLink_ {
    fn extract_resource_type(&self) -> String {
        "google_dataplex_entry_link".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataplexEntryLink {
    pub tf_id: String,
    #[doc = "The id of the entry group this entry link is in."]
    pub entry_group_id: PrimField<String>,
    #[doc = "The id of the entry link to create."]
    pub entry_link_id: PrimField<String>,
    #[doc = "Relative resource name of the Entry Link Type used to create this Entry Link. For example:\nprojects/dataplex-types/locations/global/entryLinkTypes/definition"]
    pub entry_link_type: PrimField<String>,
    #[doc = "The location for the entry."]
    pub location: PrimField<String>,
}
impl BuildDataplexEntryLink {
    pub fn build(self, stack: &mut Stack) -> DataplexEntryLink {
        let out = DataplexEntryLink(Rc::new(DataplexEntryLink_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataplexEntryLinkData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                entry_group_id: self.entry_group_id,
                entry_link_id: self.entry_link_id,
                entry_link_type: self.entry_link_type,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                aspects: core::default::Default::default(),
                entry_references: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DataplexEntryLinkRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexEntryLinkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataplexEntryLinkRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the Entry Link was created."]
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
    #[doc = "Get a reference to the value of field `entry_group_id` after provisioning.\nThe id of the entry group this entry link is in."]
    pub fn entry_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entry_group_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entry_link_id` after provisioning.\nThe id of the entry link to create."]
    pub fn entry_link_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entry_link_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entry_link_type` after provisioning.\nRelative resource name of the Entry Link Type used to create this Entry Link. For example:\nprojects/dataplex-types/locations/global/entryLinkTypes/definition"]
    pub fn entry_link_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entry_link_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the entry."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the Entry Link, of the form:\nprojects/{project_id_or_number}/locations/{location_id}/entryGroups/{entry_group_id}/entryLinks/{entry_link_id}"]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the Entry Link was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `aspects` after provisioning.\n"]
    pub fn aspects(&self) -> ListRef<DataplexEntryLinkAspectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.aspects", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entry_references` after provisioning.\n"]
    pub fn entry_references(&self) -> ListRef<DataplexEntryLinkEntryReferencesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.entry_references", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataplexEntryLinkTimeoutsElRef {
        DataplexEntryLinkTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexEntryLinkAspectsElAspectEl {
    data: PrimField<String>,
}
impl DataplexEntryLinkAspectsElAspectEl {}
impl ToListMappable for DataplexEntryLinkAspectsElAspectEl {
    type O = BlockAssignable<DataplexEntryLinkAspectsElAspectEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexEntryLinkAspectsElAspectEl {
    #[doc = "The content of the aspect in JSON form, according to its aspect type schema. The maximum size of the field is 120KB (encoded as UTF-8)."]
    pub data: PrimField<String>,
}
impl BuildDataplexEntryLinkAspectsElAspectEl {
    pub fn build(self) -> DataplexEntryLinkAspectsElAspectEl {
        DataplexEntryLinkAspectsElAspectEl { data: self.data }
    }
}
pub struct DataplexEntryLinkAspectsElAspectElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexEntryLinkAspectsElAspectElRef {
    fn new(shared: StackShared, base: String) -> DataplexEntryLinkAspectsElAspectElRef {
        DataplexEntryLinkAspectsElAspectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexEntryLinkAspectsElAspectElRef {
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
    #[doc = "Get a reference to the value of field `path` after provisioning.\nThe path in the entry link under which the aspect is attached."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the Aspect was last modified."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataplexEntryLinkAspectsElDynamic {
    aspect: Option<DynamicBlock<DataplexEntryLinkAspectsElAspectEl>>,
}
#[derive(Serialize)]
pub struct DataplexEntryLinkAspectsEl {
    aspect_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aspect: Option<Vec<DataplexEntryLinkAspectsElAspectEl>>,
    dynamic: DataplexEntryLinkAspectsElDynamic,
}
impl DataplexEntryLinkAspectsEl {
    #[doc = "Set the field `aspect`.\n"]
    pub fn set_aspect(
        mut self,
        v: impl Into<BlockAssignable<DataplexEntryLinkAspectsElAspectEl>>,
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
impl ToListMappable for DataplexEntryLinkAspectsEl {
    type O = BlockAssignable<DataplexEntryLinkAspectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexEntryLinkAspectsEl {
    #[doc = "The map keys of the Aspects which the service should modify.\nIt should be the aspect type reference in the format '{project_number}.{location_id}.{aspect_type_id}'."]
    pub aspect_key: PrimField<String>,
}
impl BuildDataplexEntryLinkAspectsEl {
    pub fn build(self) -> DataplexEntryLinkAspectsEl {
        DataplexEntryLinkAspectsEl {
            aspect_key: self.aspect_key,
            aspect: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataplexEntryLinkAspectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexEntryLinkAspectsElRef {
    fn new(shared: StackShared, base: String) -> DataplexEntryLinkAspectsElRef {
        DataplexEntryLinkAspectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexEntryLinkAspectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aspect_key` after provisioning.\nThe map keys of the Aspects which the service should modify.\nIt should be the aspect type reference in the format '{project_number}.{location_id}.{aspect_type_id}'."]
    pub fn aspect_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aspect_key", self.base))
    }
    #[doc = "Get a reference to the value of field `aspect` after provisioning.\n"]
    pub fn aspect(&self) -> ListRef<DataplexEntryLinkAspectsElAspectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aspect", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexEntryLinkEntryReferencesEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataplexEntryLinkEntryReferencesEl {
    #[doc = "Set the field `path`.\nThe path in the Entry that is referenced in the Entry Link.\nEmpty path denotes that the Entry itself is referenced in the Entry Link."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nThe reference type of the Entry. Possible values: [\"SOURCE\", \"TARGET\"]"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexEntryLinkEntryReferencesEl {
    type O = BlockAssignable<DataplexEntryLinkEntryReferencesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexEntryLinkEntryReferencesEl {
    #[doc = "The relative resource name of the referenced Entry, of the form:\nprojects/{project_id_or_number}/locations/{location_id}/entryGroups/{entry_group_id}/entries/{entry_id}"]
    pub name: PrimField<String>,
}
impl BuildDataplexEntryLinkEntryReferencesEl {
    pub fn build(self) -> DataplexEntryLinkEntryReferencesEl {
        DataplexEntryLinkEntryReferencesEl {
            name: self.name,
            path: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataplexEntryLinkEntryReferencesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexEntryLinkEntryReferencesElRef {
    fn new(shared: StackShared, base: String) -> DataplexEntryLinkEntryReferencesElRef {
        DataplexEntryLinkEntryReferencesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexEntryLinkEntryReferencesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the referenced Entry, of the form:\nprojects/{project_id_or_number}/locations/{location_id}/entryGroups/{entry_group_id}/entries/{entry_id}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nThe path in the Entry that is referenced in the Entry Link.\nEmpty path denotes that the Entry itself is referenced in the Entry Link."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe reference type of the Entry. Possible values: [\"SOURCE\", \"TARGET\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexEntryLinkTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DataplexEntryLinkTimeoutsEl {
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
impl ToListMappable for DataplexEntryLinkTimeoutsEl {
    type O = BlockAssignable<DataplexEntryLinkTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexEntryLinkTimeoutsEl {}
impl BuildDataplexEntryLinkTimeoutsEl {
    pub fn build(self) -> DataplexEntryLinkTimeoutsEl {
        DataplexEntryLinkTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DataplexEntryLinkTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexEntryLinkTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataplexEntryLinkTimeoutsElRef {
        DataplexEntryLinkTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexEntryLinkTimeoutsElRef {
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
struct DataplexEntryLinkDynamic {
    aspects: Option<DynamicBlock<DataplexEntryLinkAspectsEl>>,
    entry_references: Option<DynamicBlock<DataplexEntryLinkEntryReferencesEl>>,
}
