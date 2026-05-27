use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ContainerAnalysisNoteData {
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
    expiration_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    long_description: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_note_names: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    short_description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attestation_authority: Option<Vec<ContainerAnalysisNoteAttestationAuthorityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_url: Option<Vec<ContainerAnalysisNoteRelatedUrlEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ContainerAnalysisNoteTimeoutsEl>,
    dynamic: ContainerAnalysisNoteDynamic,
}
struct ContainerAnalysisNote_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ContainerAnalysisNoteData>,
}
#[derive(Clone)]
pub struct ContainerAnalysisNote(Rc<ContainerAnalysisNote_>);
impl ContainerAnalysisNote {
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
    #[doc = "Set the field `expiration_time`.\nTime of expiration for this note. Leave empty if note does not expire."]
    pub fn set_expiration_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().expiration_time = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `long_description`.\nA detailed description of the note"]
    pub fn set_long_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().long_description = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `related_note_names`.\nNames of other notes related to this note."]
    pub fn set_related_note_names(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().related_note_names = Some(v.into());
        self
    }
    #[doc = "Set the field `short_description`.\nA one sentence description of the note."]
    pub fn set_short_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().short_description = Some(v.into());
        self
    }
    #[doc = "Set the field `attestation_authority`.\n"]
    pub fn set_attestation_authority(
        self,
        v: impl Into<BlockAssignable<ContainerAnalysisNoteAttestationAuthorityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attestation_authority = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attestation_authority = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `related_url`.\n"]
    pub fn set_related_url(
        self,
        v: impl Into<BlockAssignable<ContainerAnalysisNoteRelatedUrlEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().related_url = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.related_url = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ContainerAnalysisNoteTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time this note was created."]
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
    #[doc = "Get a reference to the value of field `expiration_time` after provisioning.\nTime of expiration for this note. Leave empty if note does not expire."]
    pub fn expiration_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expiration_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kind` after provisioning.\nThe type of analysis this note describes"]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kind", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `long_description` after provisioning.\nA detailed description of the note"]
    pub fn long_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.long_description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the note."]
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
    #[doc = "Get a reference to the value of field `related_note_names` after provisioning.\nNames of other notes related to this note."]
    pub fn related_note_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.related_note_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `short_description` after provisioning.\nA one sentence description of the note."]
    pub fn short_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.short_description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time this note was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `attestation_authority` after provisioning.\n"]
    pub fn attestation_authority(&self) -> ListRef<ContainerAnalysisNoteAttestationAuthorityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attestation_authority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAnalysisNoteTimeoutsElRef {
        ContainerAnalysisNoteTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ContainerAnalysisNote {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ContainerAnalysisNote {}
impl ToListMappable for ContainerAnalysisNote {
    type O = ListRef<ContainerAnalysisNoteRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ContainerAnalysisNote_ {
    fn extract_resource_type(&self) -> String {
        "google_container_analysis_note".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildContainerAnalysisNote {
    pub tf_id: String,
    #[doc = "The name of the note."]
    pub name: PrimField<String>,
}
impl BuildContainerAnalysisNote {
    pub fn build(self, stack: &mut Stack) -> ContainerAnalysisNote {
        let out = ContainerAnalysisNote(Rc::new(ContainerAnalysisNote_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ContainerAnalysisNoteData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                expiration_time: core::default::Default::default(),
                id: core::default::Default::default(),
                long_description: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                related_note_names: core::default::Default::default(),
                short_description: core::default::Default::default(),
                attestation_authority: core::default::Default::default(),
                related_url: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ContainerAnalysisNoteRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAnalysisNoteRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ContainerAnalysisNoteRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time this note was created."]
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
    #[doc = "Get a reference to the value of field `expiration_time` after provisioning.\nTime of expiration for this note. Leave empty if note does not expire."]
    pub fn expiration_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expiration_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kind` after provisioning.\nThe type of analysis this note describes"]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kind", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `long_description` after provisioning.\nA detailed description of the note"]
    pub fn long_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.long_description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the note."]
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
    #[doc = "Get a reference to the value of field `related_note_names` after provisioning.\nNames of other notes related to this note."]
    pub fn related_note_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.related_note_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `short_description` after provisioning.\nA one sentence description of the note."]
    pub fn short_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.short_description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time this note was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `attestation_authority` after provisioning.\n"]
    pub fn attestation_authority(&self) -> ListRef<ContainerAnalysisNoteAttestationAuthorityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attestation_authority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAnalysisNoteTimeoutsElRef {
        ContainerAnalysisNoteTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerAnalysisNoteAttestationAuthorityElHintEl {
    human_readable_name: PrimField<String>,
}
impl ContainerAnalysisNoteAttestationAuthorityElHintEl {}
impl ToListMappable for ContainerAnalysisNoteAttestationAuthorityElHintEl {
    type O = BlockAssignable<ContainerAnalysisNoteAttestationAuthorityElHintEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAnalysisNoteAttestationAuthorityElHintEl {
    #[doc = "The human readable name of this Attestation Authority, for\nexample \"qa\"."]
    pub human_readable_name: PrimField<String>,
}
impl BuildContainerAnalysisNoteAttestationAuthorityElHintEl {
    pub fn build(self) -> ContainerAnalysisNoteAttestationAuthorityElHintEl {
        ContainerAnalysisNoteAttestationAuthorityElHintEl {
            human_readable_name: self.human_readable_name,
        }
    }
}
pub struct ContainerAnalysisNoteAttestationAuthorityElHintElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAnalysisNoteAttestationAuthorityElHintElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerAnalysisNoteAttestationAuthorityElHintElRef {
        ContainerAnalysisNoteAttestationAuthorityElHintElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAnalysisNoteAttestationAuthorityElHintElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `human_readable_name` after provisioning.\nThe human readable name of this Attestation Authority, for\nexample \"qa\"."]
    pub fn human_readable_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.human_readable_name", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ContainerAnalysisNoteAttestationAuthorityElDynamic {
    hint: Option<DynamicBlock<ContainerAnalysisNoteAttestationAuthorityElHintEl>>,
}
#[derive(Serialize)]
pub struct ContainerAnalysisNoteAttestationAuthorityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hint: Option<Vec<ContainerAnalysisNoteAttestationAuthorityElHintEl>>,
    dynamic: ContainerAnalysisNoteAttestationAuthorityElDynamic,
}
impl ContainerAnalysisNoteAttestationAuthorityEl {
    #[doc = "Set the field `hint`.\n"]
    pub fn set_hint(
        mut self,
        v: impl Into<BlockAssignable<ContainerAnalysisNoteAttestationAuthorityElHintEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hint = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hint = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContainerAnalysisNoteAttestationAuthorityEl {
    type O = BlockAssignable<ContainerAnalysisNoteAttestationAuthorityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAnalysisNoteAttestationAuthorityEl {}
impl BuildContainerAnalysisNoteAttestationAuthorityEl {
    pub fn build(self) -> ContainerAnalysisNoteAttestationAuthorityEl {
        ContainerAnalysisNoteAttestationAuthorityEl {
            hint: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContainerAnalysisNoteAttestationAuthorityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAnalysisNoteAttestationAuthorityElRef {
    fn new(shared: StackShared, base: String) -> ContainerAnalysisNoteAttestationAuthorityElRef {
        ContainerAnalysisNoteAttestationAuthorityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAnalysisNoteAttestationAuthorityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hint` after provisioning.\n"]
    pub fn hint(&self) -> ListRef<ContainerAnalysisNoteAttestationAuthorityElHintElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hint", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAnalysisNoteRelatedUrlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<PrimField<String>>,
    url: PrimField<String>,
}
impl ContainerAnalysisNoteRelatedUrlEl {
    #[doc = "Set the field `label`.\nLabel to describe usage of the URL"]
    pub fn set_label(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.label = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerAnalysisNoteRelatedUrlEl {
    type O = BlockAssignable<ContainerAnalysisNoteRelatedUrlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAnalysisNoteRelatedUrlEl {
    #[doc = "Specific URL associated with the resource."]
    pub url: PrimField<String>,
}
impl BuildContainerAnalysisNoteRelatedUrlEl {
    pub fn build(self) -> ContainerAnalysisNoteRelatedUrlEl {
        ContainerAnalysisNoteRelatedUrlEl {
            label: core::default::Default::default(),
            url: self.url,
        }
    }
}
pub struct ContainerAnalysisNoteRelatedUrlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAnalysisNoteRelatedUrlElRef {
    fn new(shared: StackShared, base: String) -> ContainerAnalysisNoteRelatedUrlElRef {
        ContainerAnalysisNoteRelatedUrlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAnalysisNoteRelatedUrlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `label` after provisioning.\nLabel to describe usage of the URL"]
    pub fn label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nSpecific URL associated with the resource."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAnalysisNoteTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ContainerAnalysisNoteTimeoutsEl {
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
impl ToListMappable for ContainerAnalysisNoteTimeoutsEl {
    type O = BlockAssignable<ContainerAnalysisNoteTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAnalysisNoteTimeoutsEl {}
impl BuildContainerAnalysisNoteTimeoutsEl {
    pub fn build(self) -> ContainerAnalysisNoteTimeoutsEl {
        ContainerAnalysisNoteTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ContainerAnalysisNoteTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAnalysisNoteTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ContainerAnalysisNoteTimeoutsElRef {
        ContainerAnalysisNoteTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAnalysisNoteTimeoutsElRef {
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
struct ContainerAnalysisNoteDynamic {
    attestation_authority: Option<DynamicBlock<ContainerAnalysisNoteAttestationAuthorityEl>>,
    related_url: Option<DynamicBlock<ContainerAnalysisNoteRelatedUrlEl>>,
}
