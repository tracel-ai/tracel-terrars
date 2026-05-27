use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeInterconnectAttachmentGroupData {
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
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interconnect_group: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attachments: Option<Vec<ComputeInterconnectAttachmentGroupAttachmentsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    intent: Option<Vec<ComputeInterconnectAttachmentGroupIntentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeInterconnectAttachmentGroupTimeoutsEl>,
    dynamic: ComputeInterconnectAttachmentGroupDynamic,
}
struct ComputeInterconnectAttachmentGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeInterconnectAttachmentGroupData>,
}
#[derive(Clone)]
pub struct ComputeInterconnectAttachmentGroup(Rc<ComputeInterconnectAttachmentGroup_>);
impl ComputeInterconnectAttachmentGroup {
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
    #[doc = "Set the field `description`.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `interconnect_group`.\nThe URL of an InterconnectGroup that groups these Attachments'\nInterconnects. Customers do not need to set this unless directed by\nGoogle Support."]
    pub fn set_interconnect_group(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().interconnect_group = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `attachments`.\n"]
    pub fn set_attachments(
        self,
        v: impl Into<BlockAssignable<ComputeInterconnectAttachmentGroupAttachmentsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attachments = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attachments = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `intent`.\n"]
    pub fn set_intent(
        self,
        v: impl Into<BlockAssignable<ComputeInterconnectAttachmentGroupIntentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().intent = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.intent = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeInterconnectAttachmentGroupTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `configured` after provisioning.\nThe redundancy this group is configured to support. The way a\nuser queries what SLA their Attachment gets is by looking at this field of\nthe Attachment's AttachmentGroup."]
    pub fn configured(&self) -> ListRef<ComputeInterconnectAttachmentGroupConfiguredElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configured", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `interconnect_group` after provisioning.\nThe URL of an InterconnectGroup that groups these Attachments'\nInterconnects. Customers do not need to set this unless directed by\nGoogle Support."]
    pub fn interconnect_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interconnect_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logical_structure` after provisioning.\nAn analysis of the logical layout of Attachments in this\ngroup. Every Attachment in the group is shown once in this structure."]
    pub fn logical_structure(
        &self,
    ) -> ListRef<ComputeInterconnectAttachmentGroupLogicalStructureElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logical_structure", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created. The name must be\n1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters must be a dash,\nlowercase letter, or digit, except the last character, which cannot be a dash."]
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
    #[doc = "Get a reference to the value of field `intent` after provisioning.\n"]
    pub fn intent(&self) -> ListRef<ComputeInterconnectAttachmentGroupIntentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.intent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInterconnectAttachmentGroupTimeoutsElRef {
        ComputeInterconnectAttachmentGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeInterconnectAttachmentGroup {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeInterconnectAttachmentGroup {}
impl ToListMappable for ComputeInterconnectAttachmentGroup {
    type O = ListRef<ComputeInterconnectAttachmentGroupRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeInterconnectAttachmentGroup_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_interconnect_attachment_group".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeInterconnectAttachmentGroup {
    pub tf_id: String,
    #[doc = "Name of the resource. Provided by the client when the resource is created. The name must be\n1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters must be a dash,\nlowercase letter, or digit, except the last character, which cannot be a dash."]
    pub name: PrimField<String>,
}
impl BuildComputeInterconnectAttachmentGroup {
    pub fn build(self, stack: &mut Stack) -> ComputeInterconnectAttachmentGroup {
        let out =
            ComputeInterconnectAttachmentGroup(Rc::new(ComputeInterconnectAttachmentGroup_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(ComputeInterconnectAttachmentGroupData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    deletion_policy: core::default::Default::default(),
                    description: core::default::Default::default(),
                    id: core::default::Default::default(),
                    interconnect_group: core::default::Default::default(),
                    name: self.name,
                    project: core::default::Default::default(),
                    attachments: core::default::Default::default(),
                    intent: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeInterconnectAttachmentGroupRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectAttachmentGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeInterconnectAttachmentGroupRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `configured` after provisioning.\nThe redundancy this group is configured to support. The way a\nuser queries what SLA their Attachment gets is by looking at this field of\nthe Attachment's AttachmentGroup."]
    pub fn configured(&self) -> ListRef<ComputeInterconnectAttachmentGroupConfiguredElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configured", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `interconnect_group` after provisioning.\nThe URL of an InterconnectGroup that groups these Attachments'\nInterconnects. Customers do not need to set this unless directed by\nGoogle Support."]
    pub fn interconnect_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interconnect_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logical_structure` after provisioning.\nAn analysis of the logical layout of Attachments in this\ngroup. Every Attachment in the group is shown once in this structure."]
    pub fn logical_structure(
        &self,
    ) -> ListRef<ComputeInterconnectAttachmentGroupLogicalStructureElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logical_structure", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created. The name must be\n1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters must be a dash,\nlowercase letter, or digit, except the last character, which cannot be a dash."]
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
    #[doc = "Get a reference to the value of field `intent` after provisioning.\n"]
    pub fn intent(&self) -> ListRef<ComputeInterconnectAttachmentGroupIntentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.intent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInterconnectAttachmentGroupTimeoutsElRef {
        ComputeInterconnectAttachmentGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaElIntendedSlaBlockersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attachments: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blocker_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    documentation_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metros: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zones: Option<ListField<PrimField<String>>>,
}
impl ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaElIntendedSlaBlockersEl {
    #[doc = "Set the field `attachments`.\n"]
    pub fn set_attachments(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.attachments = Some(v.into());
        self
    }
    #[doc = "Set the field `blocker_type`.\n"]
    pub fn set_blocker_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.blocker_type = Some(v.into());
        self
    }
    #[doc = "Set the field `documentation_link`.\n"]
    pub fn set_documentation_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.documentation_link = Some(v.into());
        self
    }
    #[doc = "Set the field `explanation`.\n"]
    pub fn set_explanation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.explanation = Some(v.into());
        self
    }
    #[doc = "Set the field `metros`.\n"]
    pub fn set_metros(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.metros = Some(v.into());
        self
    }
    #[doc = "Set the field `regions`.\n"]
    pub fn set_regions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.regions = Some(v.into());
        self
    }
    #[doc = "Set the field `zones`.\n"]
    pub fn set_zones(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.zones = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaElIntendedSlaBlockersEl
{
    type O = BlockAssignable<
        ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaElIntendedSlaBlockersEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaElIntendedSlaBlockersEl
{}
impl BuildComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaElIntendedSlaBlockersEl {
    pub fn build(
        self,
    ) -> ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaElIntendedSlaBlockersEl {
        ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaElIntendedSlaBlockersEl {
            attachments: core::default::Default::default(),
            blocker_type: core::default::Default::default(),
            documentation_link: core::default::Default::default(),
            explanation: core::default::Default::default(),
            metros: core::default::Default::default(),
            regions: core::default::Default::default(),
            zones: core::default::Default::default(),
        }
    }
}
pub struct ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaElIntendedSlaBlockersElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaElIntendedSlaBlockersElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaElIntendedSlaBlockersElRef
    {
        ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaElIntendedSlaBlockersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaElIntendedSlaBlockersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `attachments` after provisioning.\n"]
    pub fn attachments(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.attachments", self.base))
    }
    #[doc = "Get a reference to the value of field `blocker_type` after provisioning.\n"]
    pub fn blocker_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.blocker_type", self.base))
    }
    #[doc = "Get a reference to the value of field `documentation_link` after provisioning.\n"]
    pub fn documentation_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.documentation_link", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `explanation` after provisioning.\n"]
    pub fn explanation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.explanation", self.base))
    }
    #[doc = "Get a reference to the value of field `metros` after provisioning.\n"]
    pub fn metros(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.metros", self.base))
    }
    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }
    #[doc = "Get a reference to the value of field `zones` after provisioning.\n"]
    pub fn zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.zones", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_sla: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    intended_sla_blockers: Option<
        ListField<
            ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaElIntendedSlaBlockersEl,
        >,
    >,
}
impl ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaEl {
    #[doc = "Set the field `effective_sla`.\n"]
    pub fn set_effective_sla(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.effective_sla = Some(v.into());
        self
    }
    #[doc = "Set the field `intended_sla_blockers`.\n"]
    pub fn set_intended_sla_blockers(
        mut self,
        v : impl Into < ListField < ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaElIntendedSlaBlockersEl > >,
    ) -> Self {
        self.intended_sla_blockers = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaEl {
    type O = BlockAssignable<ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaEl {}
impl BuildComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaEl {
    pub fn build(self) -> ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaEl {
        ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaEl {
            effective_sla: core::default::Default::default(),
            intended_sla_blockers: core::default::Default::default(),
        }
    }
}
pub struct ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaElRef {
        ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `effective_sla` after provisioning.\n"]
    pub fn effective_sla(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_sla", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `intended_sla_blockers` after provisioning.\n"]
    pub fn intended_sla_blockers(
        &self,
    ) -> ListRef<
        ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaElIntendedSlaBlockersElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.intended_sla_blockers", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectAttachmentGroupConfiguredEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_sla:
        Option<ListField<ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaEl>>,
}
impl ComputeInterconnectAttachmentGroupConfiguredEl {
    #[doc = "Set the field `availability_sla`.\n"]
    pub fn set_availability_sla(
        mut self,
        v: impl Into<ListField<ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaEl>>,
    ) -> Self {
        self.availability_sla = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInterconnectAttachmentGroupConfiguredEl {
    type O = BlockAssignable<ComputeInterconnectAttachmentGroupConfiguredEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectAttachmentGroupConfiguredEl {}
impl BuildComputeInterconnectAttachmentGroupConfiguredEl {
    pub fn build(self) -> ComputeInterconnectAttachmentGroupConfiguredEl {
        ComputeInterconnectAttachmentGroupConfiguredEl {
            availability_sla: core::default::Default::default(),
        }
    }
}
pub struct ComputeInterconnectAttachmentGroupConfiguredElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectAttachmentGroupConfiguredElRef {
    fn new(shared: StackShared, base: String) -> ComputeInterconnectAttachmentGroupConfiguredElRef {
        ComputeInterconnectAttachmentGroupConfiguredElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectAttachmentGroupConfiguredElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `availability_sla` after provisioning.\n"]
    pub fn availability_sla(
        &self,
    ) -> ListRef<ComputeInterconnectAttachmentGroupConfiguredElAvailabilitySlaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.availability_sla", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesElZonesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    attachment: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attachments: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
impl ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesElZonesEl {
    #[doc = "Set the field `attachment`.\n"]
    pub fn set_attachment(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.attachment = Some(v.into());
        self
    }
    #[doc = "Set the field `attachments`.\n"]
    pub fn set_attachments(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.attachments = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesElZonesEl
{
    type O = BlockAssignable<
        ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesElZonesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesElZonesEl
{}
impl BuildComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesElZonesEl {
    pub fn build(
        self,
    ) -> ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesElZonesEl
    {
        ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesElZonesEl {
            attachment: core::default::Default::default(),
            attachments: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}
pub struct ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesElZonesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesElZonesElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesElZonesElRef
    {
        ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesElZonesElRef { shared : shared , base : base . to_string () , }
    }
}
impl ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesElZonesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `attachment` after provisioning.\n"]
    pub fn attachment(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.attachment", self.base))
    }
    #[doc = "Get a reference to the value of field `attachments` after provisioning.\n"]
    pub fn attachments(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.attachments", self.base))
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesEl { # [serde (skip_serializing_if = "Option::is_none")] facility : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] zones : Option < ListField < ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesElZonesEl > > , }
impl ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesEl {
    #[doc = "Set the field `facility`.\n"]
    pub fn set_facility(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.facility = Some(v.into());
        self
    }
    #[doc = "Set the field `zones`.\n"]
    pub fn set_zones(
        mut self,
        v : impl Into < ListField < ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesElZonesEl > >,
    ) -> Self {
        self.zones = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesEl
{
    type O = BlockAssignable<
        ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesEl {
}
impl BuildComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesEl {
    pub fn build(
        self,
    ) -> ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesEl {
        ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesEl {
            facility: core::default::Default::default(),
            zones: core::default::Default::default(),
        }
    }
}
pub struct ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesElRef {
        ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `facility` after provisioning.\n"]
    pub fn facility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.facility", self.base))
    }
    #[doc = "Get a reference to the value of field `zones` after provisioning.\n"]
    pub fn zones(
        &self,
    ) -> ListRef<
        ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesElZonesElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.zones", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    facilities: Option<
        ListField<
            ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    metro: Option<PrimField<String>>,
}
impl ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosEl {
    #[doc = "Set the field `facilities`.\n"]
    pub fn set_facilities(
        mut self,
        v: impl Into<
            ListField<
                ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesEl,
            >,
        >,
    ) -> Self {
        self.facilities = Some(v.into());
        self
    }
    #[doc = "Set the field `metro`.\n"]
    pub fn set_metro(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metro = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosEl {
    type O = BlockAssignable<ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosEl {}
impl BuildComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosEl {
    pub fn build(self) -> ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosEl {
        ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosEl {
            facilities: core::default::Default::default(),
            metro: core::default::Default::default(),
        }
    }
}
pub struct ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElRef {
        ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `facilities` after provisioning.\n"]
    pub fn facilities(
        &self,
    ) -> ListRef<ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElFacilitiesElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.facilities", self.base))
    }
    #[doc = "Get a reference to the value of field `metro` after provisioning.\n"]
    pub fn metro(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metro", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectAttachmentGroupLogicalStructureElRegionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metros:
        Option<ListField<ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
impl ComputeInterconnectAttachmentGroupLogicalStructureElRegionsEl {
    #[doc = "Set the field `metros`.\n"]
    pub fn set_metros(
        mut self,
        v: impl Into<ListField<ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosEl>>,
    ) -> Self {
        self.metros = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInterconnectAttachmentGroupLogicalStructureElRegionsEl {
    type O = BlockAssignable<ComputeInterconnectAttachmentGroupLogicalStructureElRegionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectAttachmentGroupLogicalStructureElRegionsEl {}
impl BuildComputeInterconnectAttachmentGroupLogicalStructureElRegionsEl {
    pub fn build(self) -> ComputeInterconnectAttachmentGroupLogicalStructureElRegionsEl {
        ComputeInterconnectAttachmentGroupLogicalStructureElRegionsEl {
            metros: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}
pub struct ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElRef {
        ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metros` after provisioning.\n"]
    pub fn metros(
        &self,
    ) -> ListRef<ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElMetrosElRef> {
        ListRef::new(self.shared().clone(), format!("{}.metros", self.base))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectAttachmentGroupLogicalStructureEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    regions: Option<ListField<ComputeInterconnectAttachmentGroupLogicalStructureElRegionsEl>>,
}
impl ComputeInterconnectAttachmentGroupLogicalStructureEl {
    #[doc = "Set the field `regions`.\n"]
    pub fn set_regions(
        mut self,
        v: impl Into<ListField<ComputeInterconnectAttachmentGroupLogicalStructureElRegionsEl>>,
    ) -> Self {
        self.regions = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInterconnectAttachmentGroupLogicalStructureEl {
    type O = BlockAssignable<ComputeInterconnectAttachmentGroupLogicalStructureEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectAttachmentGroupLogicalStructureEl {}
impl BuildComputeInterconnectAttachmentGroupLogicalStructureEl {
    pub fn build(self) -> ComputeInterconnectAttachmentGroupLogicalStructureEl {
        ComputeInterconnectAttachmentGroupLogicalStructureEl {
            regions: core::default::Default::default(),
        }
    }
}
pub struct ComputeInterconnectAttachmentGroupLogicalStructureElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectAttachmentGroupLogicalStructureElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInterconnectAttachmentGroupLogicalStructureElRef {
        ComputeInterconnectAttachmentGroupLogicalStructureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectAttachmentGroupLogicalStructureElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(
        &self,
    ) -> ListRef<ComputeInterconnectAttachmentGroupLogicalStructureElRegionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectAttachmentGroupAttachmentsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attachment: Option<PrimField<String>>,
    name: PrimField<String>,
}
impl ComputeInterconnectAttachmentGroupAttachmentsEl {
    #[doc = "Set the field `attachment`.\n"]
    pub fn set_attachment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attachment = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInterconnectAttachmentGroupAttachmentsEl {
    type O = BlockAssignable<ComputeInterconnectAttachmentGroupAttachmentsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectAttachmentGroupAttachmentsEl {
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildComputeInterconnectAttachmentGroupAttachmentsEl {
    pub fn build(self) -> ComputeInterconnectAttachmentGroupAttachmentsEl {
        ComputeInterconnectAttachmentGroupAttachmentsEl {
            attachment: core::default::Default::default(),
            name: self.name,
        }
    }
}
pub struct ComputeInterconnectAttachmentGroupAttachmentsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectAttachmentGroupAttachmentsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInterconnectAttachmentGroupAttachmentsElRef {
        ComputeInterconnectAttachmentGroupAttachmentsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectAttachmentGroupAttachmentsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `attachment` after provisioning.\n"]
    pub fn attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attachment", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectAttachmentGroupIntentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_sla: Option<PrimField<String>>,
}
impl ComputeInterconnectAttachmentGroupIntentEl {
    #[doc = "Set the field `availability_sla`.\nWhich SLA the user intends this group to support. Possible values: [\"PRODUCTION_NON_CRITICAL\", \"PRODUCTION_CRITICAL\", \"NO_SLA\", \"AVAILABILITY_SLA_UNSPECIFIED\"]"]
    pub fn set_availability_sla(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.availability_sla = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInterconnectAttachmentGroupIntentEl {
    type O = BlockAssignable<ComputeInterconnectAttachmentGroupIntentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectAttachmentGroupIntentEl {}
impl BuildComputeInterconnectAttachmentGroupIntentEl {
    pub fn build(self) -> ComputeInterconnectAttachmentGroupIntentEl {
        ComputeInterconnectAttachmentGroupIntentEl {
            availability_sla: core::default::Default::default(),
        }
    }
}
pub struct ComputeInterconnectAttachmentGroupIntentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectAttachmentGroupIntentElRef {
    fn new(shared: StackShared, base: String) -> ComputeInterconnectAttachmentGroupIntentElRef {
        ComputeInterconnectAttachmentGroupIntentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectAttachmentGroupIntentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `availability_sla` after provisioning.\nWhich SLA the user intends this group to support. Possible values: [\"PRODUCTION_NON_CRITICAL\", \"PRODUCTION_CRITICAL\", \"NO_SLA\", \"AVAILABILITY_SLA_UNSPECIFIED\"]"]
    pub fn availability_sla(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_sla", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectAttachmentGroupTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeInterconnectAttachmentGroupTimeoutsEl {
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
impl ToListMappable for ComputeInterconnectAttachmentGroupTimeoutsEl {
    type O = BlockAssignable<ComputeInterconnectAttachmentGroupTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectAttachmentGroupTimeoutsEl {}
impl BuildComputeInterconnectAttachmentGroupTimeoutsEl {
    pub fn build(self) -> ComputeInterconnectAttachmentGroupTimeoutsEl {
        ComputeInterconnectAttachmentGroupTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeInterconnectAttachmentGroupTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectAttachmentGroupTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeInterconnectAttachmentGroupTimeoutsElRef {
        ComputeInterconnectAttachmentGroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectAttachmentGroupTimeoutsElRef {
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
struct ComputeInterconnectAttachmentGroupDynamic {
    attachments: Option<DynamicBlock<ComputeInterconnectAttachmentGroupAttachmentsEl>>,
    intent: Option<DynamicBlock<ComputeInterconnectAttachmentGroupIntentEl>>,
}
