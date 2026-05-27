use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct OsConfigOsPolicyAssignmentData {
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
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_await_rollout: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_filter: Option<Vec<OsConfigOsPolicyAssignmentInstanceFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os_policies: Option<Vec<OsConfigOsPolicyAssignmentOsPoliciesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rollout: Option<Vec<OsConfigOsPolicyAssignmentRolloutEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<OsConfigOsPolicyAssignmentTimeoutsEl>,
    dynamic: OsConfigOsPolicyAssignmentDynamic,
}
struct OsConfigOsPolicyAssignment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OsConfigOsPolicyAssignmentData>,
}
#[derive(Clone)]
pub struct OsConfigOsPolicyAssignment(Rc<OsConfigOsPolicyAssignment_>);
impl OsConfigOsPolicyAssignment {
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
    #[doc = "Set the field `description`.\nOS policy assignment description. Length of the description is limited to 1024 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe project for the resource"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `skip_await_rollout`.\nSet to true to skip awaiting rollout during resource creation and update."]
    pub fn set_skip_await_rollout(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_await_rollout = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_filter`.\n"]
    pub fn set_instance_filter(
        self,
        v: impl Into<BlockAssignable<OsConfigOsPolicyAssignmentInstanceFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `os_policies`.\n"]
    pub fn set_os_policies(
        self,
        v: impl Into<BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().os_policies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.os_policies = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rollout`.\n"]
    pub fn set_rollout(
        self,
        v: impl Into<BlockAssignable<OsConfigOsPolicyAssignmentRolloutEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rollout = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rollout = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<OsConfigOsPolicyAssignmentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `baseline` after provisioning.\nOutput only. Indicates that this revision has been successfully rolled out in this zone and new VMs will be assigned OS policies from this revision.\nFor a given OS policy assignment, there is only one revision with a value of 'true' for this field."]
    pub fn baseline(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.baseline", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deleted` after provisioning.\nOutput only. Indicates that this revision deletes the OS policy assignment."]
    pub fn deleted(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deleted", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOS policy assignment description. Length of the description is limited to 1024 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe etag for this OS policy assignment. If this is provided on update, it must match the server's etag."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. Indicates that reconciliation is in progress for the revision. This value is 'true' when the 'rollout_state' is one of:\n* IN_PROGRESS\n* CANCELLING"]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `revision_create_time` after provisioning.\nOutput only. The timestamp that the revision was created."]
    pub fn revision_create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision_create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `revision_id` after provisioning.\nOutput only. The assignment revision ID A new revision is committed whenever a rollout is triggered for a OS policy assignment"]
    pub fn revision_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rollout_state` after provisioning.\nOutput only. OS policy assignment rollout state"]
    pub fn rollout_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rollout_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `skip_await_rollout` after provisioning.\nSet to true to skip awaiting rollout during resource creation and update."]
    pub fn skip_await_rollout(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_await_rollout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. Server generated unique id for the OS policy assignment resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_filter` after provisioning.\n"]
    pub fn instance_filter(&self) -> ListRef<OsConfigOsPolicyAssignmentInstanceFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `os_policies` after provisioning.\n"]
    pub fn os_policies(&self) -> ListRef<OsConfigOsPolicyAssignmentOsPoliciesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.os_policies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rollout` after provisioning.\n"]
    pub fn rollout(&self) -> ListRef<OsConfigOsPolicyAssignmentRolloutElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rollout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OsConfigOsPolicyAssignmentTimeoutsElRef {
        OsConfigOsPolicyAssignmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for OsConfigOsPolicyAssignment {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for OsConfigOsPolicyAssignment {}
impl ToListMappable for OsConfigOsPolicyAssignment {
    type O = ListRef<OsConfigOsPolicyAssignmentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for OsConfigOsPolicyAssignment_ {
    fn extract_resource_type(&self) -> String {
        "google_os_config_os_policy_assignment".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildOsConfigOsPolicyAssignment {
    pub tf_id: String,
    #[doc = "The location for the resource"]
    pub location: PrimField<String>,
    #[doc = "Resource name."]
    pub name: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignment {
    pub fn build(self, stack: &mut Stack) -> OsConfigOsPolicyAssignment {
        let out = OsConfigOsPolicyAssignment(Rc::new(OsConfigOsPolicyAssignment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OsConfigOsPolicyAssignmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                skip_await_rollout: core::default::Default::default(),
                instance_filter: core::default::Default::default(),
                os_policies: core::default::Default::default(),
                rollout: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct OsConfigOsPolicyAssignmentRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl OsConfigOsPolicyAssignmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `baseline` after provisioning.\nOutput only. Indicates that this revision has been successfully rolled out in this zone and new VMs will be assigned OS policies from this revision.\nFor a given OS policy assignment, there is only one revision with a value of 'true' for this field."]
    pub fn baseline(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.baseline", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deleted` after provisioning.\nOutput only. Indicates that this revision deletes the OS policy assignment."]
    pub fn deleted(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deleted", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOS policy assignment description. Length of the description is limited to 1024 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe etag for this OS policy assignment. If this is provided on update, it must match the server's etag."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. Indicates that reconciliation is in progress for the revision. This value is 'true' when the 'rollout_state' is one of:\n* IN_PROGRESS\n* CANCELLING"]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `revision_create_time` after provisioning.\nOutput only. The timestamp that the revision was created."]
    pub fn revision_create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision_create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `revision_id` after provisioning.\nOutput only. The assignment revision ID A new revision is committed whenever a rollout is triggered for a OS policy assignment"]
    pub fn revision_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rollout_state` after provisioning.\nOutput only. OS policy assignment rollout state"]
    pub fn rollout_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rollout_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `skip_await_rollout` after provisioning.\nSet to true to skip awaiting rollout during resource creation and update."]
    pub fn skip_await_rollout(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_await_rollout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. Server generated unique id for the OS policy assignment resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_filter` after provisioning.\n"]
    pub fn instance_filter(&self) -> ListRef<OsConfigOsPolicyAssignmentInstanceFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `os_policies` after provisioning.\n"]
    pub fn os_policies(&self) -> ListRef<OsConfigOsPolicyAssignmentOsPoliciesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.os_policies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rollout` after provisioning.\n"]
    pub fn rollout(&self) -> ListRef<OsConfigOsPolicyAssignmentRolloutElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rollout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OsConfigOsPolicyAssignmentTimeoutsElRef {
        OsConfigOsPolicyAssignmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
}
impl OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsEl {
    #[doc = "Set the field `labels`.\nLabels are identified by key/value pairs in this map. A VM should contain all the key/value pairs specified in this map to be selected."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
}
impl ToListMappable for OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsEl {
    type O = BlockAssignable<OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsEl {}
impl BuildOsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsEl {
        OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsEl {
            labels: core::default::Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsElRef {
        OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels are identified by key/value pairs in this map. A VM should contain all the key/value pairs specified in this map to be selected."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
}
impl OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsEl {
    #[doc = "Set the field `labels`.\nLabels are identified by key/value pairs in this map. A VM should contain all the key/value pairs specified in this map to be selected."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
}
impl ToListMappable for OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsEl {
    type O = BlockAssignable<OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsEl {}
impl BuildOsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsEl {
        OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsEl {
            labels: core::default::Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsElRef {
        OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels are identified by key/value pairs in this map. A VM should contain all the key/value pairs specified in this map to be selected."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentInstanceFilterElInventoriesEl {
    os_short_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os_version: Option<PrimField<String>>,
}
impl OsConfigOsPolicyAssignmentInstanceFilterElInventoriesEl {
    #[doc = "Set the field `os_version`.\nThe OS version Prefix matches are supported if asterisk(*) is provided as the last character. For example, to match all versions with a major version of '7', specify the following value for this field '7.*' An empty string matches all OS versions."]
    pub fn set_os_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.os_version = Some(v.into());
        self
    }
}
impl ToListMappable for OsConfigOsPolicyAssignmentInstanceFilterElInventoriesEl {
    type O = BlockAssignable<OsConfigOsPolicyAssignmentInstanceFilterElInventoriesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentInstanceFilterElInventoriesEl {
    #[doc = "The OS short name"]
    pub os_short_name: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignmentInstanceFilterElInventoriesEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentInstanceFilterElInventoriesEl {
        OsConfigOsPolicyAssignmentInstanceFilterElInventoriesEl {
            os_short_name: self.os_short_name,
            os_version: core::default::Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentInstanceFilterElInventoriesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentInstanceFilterElInventoriesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentInstanceFilterElInventoriesElRef {
        OsConfigOsPolicyAssignmentInstanceFilterElInventoriesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentInstanceFilterElInventoriesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `os_short_name` after provisioning.\nThe OS short name"]
    pub fn os_short_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.os_short_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `os_version` after provisioning.\nThe OS version Prefix matches are supported if asterisk(*) is provided as the last character. For example, to match all versions with a major version of '7', specify the following value for this field '7.*' An empty string matches all OS versions."]
    pub fn os_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.os_version", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentInstanceFilterElDynamic {
    exclusion_labels:
        Option<DynamicBlock<OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsEl>>,
    inclusion_labels:
        Option<DynamicBlock<OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsEl>>,
    inventories: Option<DynamicBlock<OsConfigOsPolicyAssignmentInstanceFilterElInventoriesEl>>,
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentInstanceFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusion_labels: Option<Vec<OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inclusion_labels: Option<Vec<OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inventories: Option<Vec<OsConfigOsPolicyAssignmentInstanceFilterElInventoriesEl>>,
    dynamic: OsConfigOsPolicyAssignmentInstanceFilterElDynamic,
}
impl OsConfigOsPolicyAssignmentInstanceFilterEl {
    #[doc = "Set the field `all`.\nTarget all VMs in the project. If true, no other criteria is permitted."]
    pub fn set_all(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.all = Some(v.into());
        self
    }
    #[doc = "Set the field `exclusion_labels`.\n"]
    pub fn set_exclusion_labels(
        mut self,
        v: impl Into<BlockAssignable<OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exclusion_labels = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exclusion_labels = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `inclusion_labels`.\n"]
    pub fn set_inclusion_labels(
        mut self,
        v: impl Into<BlockAssignable<OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.inclusion_labels = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.inclusion_labels = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `inventories`.\n"]
    pub fn set_inventories(
        mut self,
        v: impl Into<BlockAssignable<OsConfigOsPolicyAssignmentInstanceFilterElInventoriesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.inventories = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.inventories = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OsConfigOsPolicyAssignmentInstanceFilterEl {
    type O = BlockAssignable<OsConfigOsPolicyAssignmentInstanceFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentInstanceFilterEl {}
impl BuildOsConfigOsPolicyAssignmentInstanceFilterEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentInstanceFilterEl {
        OsConfigOsPolicyAssignmentInstanceFilterEl {
            all: core::default::Default::default(),
            exclusion_labels: core::default::Default::default(),
            inclusion_labels: core::default::Default::default(),
            inventories: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentInstanceFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentInstanceFilterElRef {
    fn new(shared: StackShared, base: String) -> OsConfigOsPolicyAssignmentInstanceFilterElRef {
        OsConfigOsPolicyAssignmentInstanceFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentInstanceFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `all` after provisioning.\nTarget all VMs in the project. If true, no other criteria is permitted."]
    pub fn all(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.all", self.base))
    }
    #[doc = "Get a reference to the value of field `exclusion_labels` after provisioning.\n"]
    pub fn exclusion_labels(
        &self,
    ) -> ListRef<OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclusion_labels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `inclusion_labels` after provisioning.\n"]
    pub fn inclusion_labels(
        &self,
    ) -> ListRef<OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inclusion_labels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `inventories` after provisioning.\n"]
    pub fn inventories(
        &self,
    ) -> ListRef<OsConfigOsPolicyAssignmentInstanceFilterElInventoriesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.inventories", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersEl {
    os_short_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os_version: Option<PrimField<String>>,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersEl {
    #[doc = "Set the field `os_version`.\nThe OS version\nPrefix matches are supported if asterisk(*) is provided as the last character. For example, to match all versions with a major version of '7', specify the following value for this field '7.*'\nAn empty string matches all OS versions."]
    pub fn set_os_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.os_version = Some(v.into());
        self
    }
}
impl ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersEl {
    type O =
        BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersEl {
    #[doc = "The OS short name"]
    pub os_short_name: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersEl {
            os_short_name: self.os_short_name,
            os_version: core::default::Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `os_short_name` after provisioning.\nThe OS short name"]
    pub fn os_short_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.os_short_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `os_version` after provisioning.\nThe OS version\nPrefix matches are supported if asterisk(*) is provided as the last character. For example, to match all versions with a major version of '7', specify the following value for this field '7.*'\nAn empty string matches all OS versions."]
    pub fn os_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.os_version", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl
{
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation: Option<PrimField<f64>>,
    object: PrimField<String>,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl {
    #[doc = "Set the field `generation`.\nGeneration number of the Cloud Storage object."]
    pub fn set_generation(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.generation = Some(v.into());
        self
    }
}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl
{
    #[doc = "Bucket of the Cloud Storage object."]
    pub bucket: PrimField<String>,
    #[doc = "Name of the Cloud Storage object."]
    pub object: PrimField<String>,
}
impl
    BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl
{
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl
    {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl { bucket : self . bucket , generation : core :: default :: Default :: default () , object : self . object , }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsElRef { fn new (shared : StackShared , base : String) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsElRef { OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsElRef { shared : shared , base : base . to_string () , } } }
impl
    OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nBucket of the Cloud Storage object."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `generation` after provisioning.\nGeneration number of the Cloud Storage object."]
    pub fn generation(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }
    #[doc = "Get a reference to the value of field `object` after provisioning.\nName of the Cloud Storage object."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256_checksum: Option<PrimField<String>>,
    uri: PrimField<String>,
}
impl
    OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl
{
    #[doc = "Set the field `sha256_checksum`.\nSHA256 checksum of the remote file."]
    pub fn set_sha256_checksum(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sha256_checksum = Some(v.into());
        self
    }
}
impl ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl { type O = BlockAssignable < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl
{
    #[doc = "URI from which to fetch the object. It should contain both the protocol and path following the format '{protocol}://{location}'."]
    pub uri: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl { pub fn build (self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl { OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl { sha256_checksum : core :: default :: Default :: default () , uri : self . uri , } } }
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteElRef { fn new (shared : StackShared , base : String) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteElRef { OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `sha256_checksum` after provisioning.\nSHA256 checksum of the remote file."] pub fn sha256_checksum (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.sha256_checksum" , self . base)) } # [doc = "Get a reference to the value of field `uri` after provisioning.\nURI from which to fetch the object. It should contain both the protocol and path following the format '{protocol}://{location}'."] pub fn uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.uri" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElDynamic { gcs : Option < DynamicBlock < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl >> , remote : Option < DynamicBlock < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl >> , }
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl { # [serde (skip_serializing_if = "Option::is_none")] allow_insecure : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] local_path : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] gcs : Option < Vec < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl > > , # [serde (skip_serializing_if = "Option::is_none")] remote : Option < Vec < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl > > , dynamic : OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElDynamic , }
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl {
    #[doc = "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations based on the file type: Remote: A checksum must be specified. Cloud Storage: An object generation number must be specified."]
    pub fn set_allow_insecure(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_insecure = Some(v.into());
        self
    }
    #[doc = "Set the field `local_path`.\nA local path within the VM to use."]
    pub fn set_local_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_path = Some(v.into());
        self
    }
    #[doc = "Set the field `gcs`.\n"]
    pub fn set_gcs(
        mut self,
        v : impl Into < BlockAssignable < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gcs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `remote`.\n"]
    pub fn set_remote(
        mut self,
        v : impl Into < BlockAssignable < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.remote = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.remote = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl
{}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl
    {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl {
            allow_insecure: core::default::Default::default(),
            local_path: core::default::Default::default(),
            gcs: core::default::Default::default(),
            remote: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRef
    {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations based on the file type: Remote: A checksum must be specified. Cloud Storage: An object generation number must be specified."]
    pub fn allow_insecure(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_insecure", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."]
    pub fn local_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }
    #[doc = "Get a reference to the value of field `gcs` after provisioning.\n"]    pub fn gcs (& self) -> ListRef < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.gcs", self.base))
    }
    #[doc = "Get a reference to the value of field `remote` after provisioning.\n"]    pub fn remote (& self) -> ListRef < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteElRef >{
        ListRef::new(self.shared().clone(), format!("{}.remote", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElDynamic {
    file: Option<
        DynamicBlock<
            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    interpreter: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_file_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    script: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<
        Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl>,
    >,
    dynamic:
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElDynamic,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl {
    #[doc = "Set the field `args`.\nOptional arguments to pass to the source during execution."]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }
    #[doc = "Set the field `output_file_path`.\nOnly recorded for enforce Exec. Path to an output file (that is created by this Exec) whose content will be recorded in OSPolicyResourceCompliance after a successful run. Absence or failure to read this file will result in this ExecResource being non-compliant. Output file size is limited to 100K bytes."]
    pub fn set_output_file_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_file_path = Some(v.into());
        self
    }
    #[doc = "Set the field `script`.\nAn inline script. The size of the script is limited to 1024 characters."]
    pub fn set_script(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.script = Some(v.into());
        self
    }
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v : impl Into < BlockAssignable < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.file = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.file = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl {
    #[doc = "The script interpreter to use. Possible values: [\"INTERPRETER_UNSPECIFIED\", \"NONE\", \"SHELL\", \"POWERSHELL\"]"]
    pub interpreter: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl {
            args: core::default::Default::default(),
            interpreter: self.interpreter,
            output_file_path: core::default::Default::default(),
            script: core::default::Default::default(),
            file: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\nOptional arguments to pass to the source during execution."]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }
    #[doc = "Get a reference to the value of field `interpreter` after provisioning.\nThe script interpreter to use. Possible values: [\"INTERPRETER_UNSPECIFIED\", \"NONE\", \"SHELL\", \"POWERSHELL\"]"]
    pub fn interpreter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interpreter", self.base))
    }
    #[doc = "Get a reference to the value of field `output_file_path` after provisioning.\nOnly recorded for enforce Exec. Path to an output file (that is created by this Exec) whose content will be recorded in OSPolicyResourceCompliance after a successful run. Absence or failure to read this file will result in this ExecResource being non-compliant. Output file size is limited to 100K bytes."]
    pub fn output_file_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_file_path", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `script` after provisioning.\nAn inline script. The size of the script is limited to 1024 characters."]
    pub fn script(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.script", self.base))
    }
    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> ListRef<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl
{
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation: Option<PrimField<f64>>,
    object: PrimField<String>,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl {
    #[doc = "Set the field `generation`.\nGeneration number of the Cloud Storage object."]
    pub fn set_generation(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.generation = Some(v.into());
        self
    }
}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl
{
    type O = BlockAssignable < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl
{
    #[doc = "Bucket of the Cloud Storage object."]
    pub bucket: PrimField<String>,
    #[doc = "Name of the Cloud Storage object."]
    pub object: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl { pub fn build (self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl { OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl { bucket : self . bucket , generation : core :: default :: Default :: default () , object : self . object , } } }
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsElRef { fn new (shared : StackShared , base : String) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsElRef { OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsElRef { shared : shared , base : base . to_string () , } } }
impl
    OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nBucket of the Cloud Storage object."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `generation` after provisioning.\nGeneration number of the Cloud Storage object."]
    pub fn generation(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }
    #[doc = "Get a reference to the value of field `object` after provisioning.\nName of the Cloud Storage object."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256_checksum: Option<PrimField<String>>,
    uri: PrimField<String>,
}
impl
    OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl
{
    #[doc = "Set the field `sha256_checksum`.\nSHA256 checksum of the remote file."]
    pub fn set_sha256_checksum(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sha256_checksum = Some(v.into());
        self
    }
}
impl ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl { type O = BlockAssignable < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl
{
    #[doc = "URI from which to fetch the object. It should contain both the protocol and path following the format '{protocol}://{location}'."]
    pub uri: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl { pub fn build (self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl { OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl { sha256_checksum : core :: default :: Default :: default () , uri : self . uri , } } }
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteElRef { fn new (shared : StackShared , base : String) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteElRef { OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `sha256_checksum` after provisioning.\nSHA256 checksum of the remote file."] pub fn sha256_checksum (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.sha256_checksum" , self . base)) } # [doc = "Get a reference to the value of field `uri` after provisioning.\nURI from which to fetch the object. It should contain both the protocol and path following the format '{protocol}://{location}'."] pub fn uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.uri" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElDynamic { gcs : Option < DynamicBlock < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl >> , remote : Option < DynamicBlock < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl >> , }
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl { # [serde (skip_serializing_if = "Option::is_none")] allow_insecure : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] local_path : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] gcs : Option < Vec < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl > > , # [serde (skip_serializing_if = "Option::is_none")] remote : Option < Vec < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl > > , dynamic : OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElDynamic , }
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl {
    #[doc = "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations based on the file type:\nRemote: A checksum must be specified. Cloud Storage: An object generation number must be specified."]
    pub fn set_allow_insecure(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_insecure = Some(v.into());
        self
    }
    #[doc = "Set the field `local_path`.\nA local path within the VM to use."]
    pub fn set_local_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_path = Some(v.into());
        self
    }
    #[doc = "Set the field `gcs`.\n"]
    pub fn set_gcs(
        mut self,
        v : impl Into < BlockAssignable < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gcs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `remote`.\n"]
    pub fn set_remote(
        mut self,
        v : impl Into < BlockAssignable < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.remote = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.remote = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl
{}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl
    {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl {
            allow_insecure: core::default::Default::default(),
            local_path: core::default::Default::default(),
            gcs: core::default::Default::default(),
            remote: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRef
    {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations based on the file type:\nRemote: A checksum must be specified. Cloud Storage: An object generation number must be specified."]
    pub fn allow_insecure(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_insecure", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."]
    pub fn local_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }
    #[doc = "Get a reference to the value of field `gcs` after provisioning.\n"]    pub fn gcs (& self) -> ListRef < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.gcs", self.base))
    }
    #[doc = "Get a reference to the value of field `remote` after provisioning.\n"]    pub fn remote (& self) -> ListRef < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteElRef >{
        ListRef::new(self.shared().clone(), format!("{}.remote", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElDynamic {
    file: Option<
        DynamicBlock<
            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    interpreter: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_file_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    script: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<
        Vec<
            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl,
        >,
    >,
    dynamic:
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElDynamic,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateEl {
    #[doc = "Set the field `args`.\nOptional arguments to pass to the source during execution."]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }
    #[doc = "Set the field `output_file_path`.\nOnly recorded for enforce Exec. Path to an output file (that is created by this Exec) whose content will be recorded in OSPolicyResourceCompliance after a successful run. Absence or failure to read this file will result in this ExecResource being non-compliant. Output file size is limited to 100K bytes."]
    pub fn set_output_file_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_file_path = Some(v.into());
        self
    }
    #[doc = "Set the field `script`.\nAn inline script. The size of the script is limited to 1024 characters."]
    pub fn set_script(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.script = Some(v.into());
        self
    }
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v : impl Into < BlockAssignable < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.file = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.file = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateEl {
    #[doc = "The script interpreter to use. Possible values: [\"INTERPRETER_UNSPECIFIED\", \"NONE\", \"SHELL\", \"POWERSHELL\"]"]
    pub interpreter: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateEl {
            args: core::default::Default::default(),
            interpreter: self.interpreter,
            output_file_path: core::default::Default::default(),
            script: core::default::Default::default(),
            file: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\nOptional arguments to pass to the source during execution."]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }
    #[doc = "Get a reference to the value of field `interpreter` after provisioning.\nThe script interpreter to use. Possible values: [\"INTERPRETER_UNSPECIFIED\", \"NONE\", \"SHELL\", \"POWERSHELL\"]"]
    pub fn interpreter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interpreter", self.base))
    }
    #[doc = "Get a reference to the value of field `output_file_path` after provisioning.\nOnly recorded for enforce Exec. Path to an output file (that is created by this Exec) whose content will be recorded in OSPolicyResourceCompliance after a successful run. Absence or failure to read this file will result in this ExecResource being non-compliant. Output file size is limited to 100K bytes."]
    pub fn output_file_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_file_path", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `script` after provisioning.\nAn inline script. The size of the script is limited to 1024 characters."]
    pub fn script(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.script", self.base))
    }
    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> ListRef<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElDynamic {
    enforce: Option<
        DynamicBlock<
            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl,
        >,
    >,
    validate: Option<
        DynamicBlock<
            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce: Option<
        Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    validate: Option<
        Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateEl>,
    >,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElDynamic,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecEl {
    #[doc = "Set the field `enforce`.\n"]
    pub fn set_enforce(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.enforce = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.enforce = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `validate`.\n"]
    pub fn set_validate(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.validate = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.validate = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecEl {
    type O =
        BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecEl {}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecEl {
            enforce: core::default::Default::default(),
            validate: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enforce` after provisioning.\n"]
    pub fn enforce(
        &self,
    ) -> ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.enforce", self.base))
    }
    #[doc = "Get a reference to the value of field `validate` after provisioning.\n"]
    pub fn validate(
        &self,
    ) -> ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.validate", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl {
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation: Option<PrimField<f64>>,
    object: PrimField<String>,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl {
    #[doc = "Set the field `generation`.\nGeneration number of the Cloud Storage object."]
    pub fn set_generation(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.generation = Some(v.into());
        self
    }
}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl {
    #[doc = "Bucket of the Cloud Storage object."]
    pub bucket: PrimField<String>,
    #[doc = "Name of the Cloud Storage object."]
    pub object: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl {
            bucket: self.bucket,
            generation: core::default::Default::default(),
            object: self.object,
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nBucket of the Cloud Storage object."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `generation` after provisioning.\nGeneration number of the Cloud Storage object."]
    pub fn generation(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }
    #[doc = "Get a reference to the value of field `object` after provisioning.\nName of the Cloud Storage object."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256_checksum: Option<PrimField<String>>,
    uri: PrimField<String>,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl {
    #[doc = "Set the field `sha256_checksum`.\nSHA256 checksum of the remote file."]
    pub fn set_sha256_checksum(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sha256_checksum = Some(v.into());
        self
    }
}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl
{
    #[doc = "URI from which to fetch the object. It should contain both the protocol and path following the format '{protocol}://{location}'."]
    pub uri: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl {
            sha256_checksum: core::default::Default::default(),
            uri: self.uri,
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteElRef
    {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `sha256_checksum` after provisioning.\nSHA256 checksum of the remote file."]
    pub fn sha256_checksum(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sha256_checksum", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nURI from which to fetch the object. It should contain both the protocol and path following the format '{protocol}://{location}'."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElDynamic {
    gcs: Option<
        DynamicBlock<
            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl,
        >,
    >,
    remote: Option<
        DynamicBlock<
            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_insecure: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs: Option<
        Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    remote: Option<
        Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl>,
    >,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElDynamic,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileEl {
    #[doc = "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations based on the file type: Remote: A checksum must be specified. Cloud Storage: An object generation number must be specified."]
    pub fn set_allow_insecure(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_insecure = Some(v.into());
        self
    }
    #[doc = "Set the field `local_path`.\nA local path within the VM to use."]
    pub fn set_local_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_path = Some(v.into());
        self
    }
    #[doc = "Set the field `gcs`.\n"]
    pub fn set_gcs(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gcs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `remote`.\n"]
    pub fn set_remote(
        mut self,
        v : impl Into < BlockAssignable < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.remote = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.remote = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileEl {}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileEl {
            allow_insecure: core::default::Default::default(),
            local_path: core::default::Default::default(),
            gcs: core::default::Default::default(),
            remote: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations based on the file type: Remote: A checksum must be specified. Cloud Storage: An object generation number must be specified."]
    pub fn allow_insecure(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_insecure", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."]
    pub fn local_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }
    #[doc = "Get a reference to the value of field `gcs` after provisioning.\n"]
    pub fn gcs(
        &self,
    ) -> ListRef<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.gcs", self.base))
    }
    #[doc = "Get a reference to the value of field `remote` after provisioning.\n"]
    pub fn remote(
        &self,
    ) -> ListRef<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.remote", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElDynamic {
    file: Option<
        DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileEl>,
    >,
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<PrimField<String>>,
    path: PrimField<String>,
    state: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file:
        Option<Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileEl>>,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElDynamic,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileEl {
    #[doc = "Set the field `content`.\nA a file with this content. The size of the content is limited to 1024 characters."]
    pub fn set_content(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content = Some(v.into());
        self
    }
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.file = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.file = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileEl {
    type O =
        BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileEl {
    #[doc = "The absolute path of the file within the VM."]
    pub path: PrimField<String>,
    #[doc = "Desired state of the file. Possible values: [\"DESIRED_STATE_UNSPECIFIED\", \"PRESENT\", \"ABSENT\", \"CONTENTS_MATCH\"]"]
    pub state: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileEl {
            content: core::default::Default::default(),
            path: self.path,
            state: self.state,
            file: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `content` after provisioning.\nA a file with this content. The size of the content is limited to 1024 characters."]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nThe absolute path of the file within the VM."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `permissions` after provisioning.\nConsists of three octal digits which represent, in order, the permissions of the owner, group, and other users for the file (similarly to the numeric mode used in the linux chmod utility). Each digit represents a three bit number with the 4 bit corresponding to the read permissions, the 2 bit corresponds to the write bit, and the one bit corresponds to the execute permission. Default behavior is 755.\nBelow are some examples of permissions and their associated values: read, write, and execute: 7 read and execute: 5 read and write: 6 read only: 4"]
    pub fn permissions(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.permissions", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nDesired state of the file. Possible values: [\"DESIRED_STATE_UNSPECIFIED\", \"PRESENT\", \"ABSENT\", \"CONTENTS_MATCH\"]"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptEl {
    name: PrimField<String>,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptEl {}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptEl {
    #[doc = "Package name."]
    pub name: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptEl {
            name: self.name,
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nPackage name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl
{
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation: Option<PrimField<f64>>,
    object: PrimField<String>,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl {
    #[doc = "Set the field `generation`.\nGeneration number of the Cloud Storage object."]
    pub fn set_generation(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.generation = Some(v.into());
        self
    }
}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl
{
    #[doc = "Bucket of the Cloud Storage object."]
    pub bucket: PrimField<String>,
    #[doc = "Name of the Cloud Storage object."]
    pub object: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl
    {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl {
            bucket: self.bucket,
            generation: core::default::Default::default(),
            object: self.object,
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsElRef
    {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsElRef { shared : shared , base : base . to_string () , }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nBucket of the Cloud Storage object."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `generation` after provisioning.\nGeneration number of the Cloud Storage object."]
    pub fn generation(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }
    #[doc = "Get a reference to the value of field `object` after provisioning.\nName of the Cloud Storage object."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256_checksum: Option<PrimField<String>>,
    uri: PrimField<String>,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl {
    #[doc = "Set the field `sha256_checksum`.\nSHA256 checksum of the remote file."]
    pub fn set_sha256_checksum(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sha256_checksum = Some(v.into());
        self
    }
}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl
{
    #[doc = "URI from which to fetch the object. It should contain both the protocol and path following the format '{protocol}://{location}'."]
    pub uri: PrimField<String>,
}
impl
    BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl
{
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl
    {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl { sha256_checksum : core :: default :: Default :: default () , uri : self . uri , }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteElRef { fn new (shared : StackShared , base : String) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteElRef { OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteElRef { shared : shared , base : base . to_string () , } } }
impl
    OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `sha256_checksum` after provisioning.\nSHA256 checksum of the remote file."]
    pub fn sha256_checksum(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sha256_checksum", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nURI from which to fetch the object. It should contain both the protocol and path following the format '{protocol}://{location}'."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElDynamic { gcs : Option < DynamicBlock < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl >> , remote : Option < DynamicBlock < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl >> , }
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl { # [serde (skip_serializing_if = "Option::is_none")] allow_insecure : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] local_path : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] gcs : Option < Vec < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl > > , # [serde (skip_serializing_if = "Option::is_none")] remote : Option < Vec < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl > > , dynamic : OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElDynamic , }
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl {
    #[doc = "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations based on the file type:\nRemote: A checksum must be specified. Cloud Storage: An object generation number must be specified."]
    pub fn set_allow_insecure(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_insecure = Some(v.into());
        self
    }
    #[doc = "Set the field `local_path`.\nA local path within the VM to use."]
    pub fn set_local_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_path = Some(v.into());
        self
    }
    #[doc = "Set the field `gcs`.\n"]
    pub fn set_gcs(
        mut self,
        v : impl Into < BlockAssignable < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gcs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `remote`.\n"]
    pub fn set_remote(
        mut self,
        v : impl Into < BlockAssignable < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.remote = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.remote = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl
{}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl {
            allow_insecure: core::default::Default::default(),
            local_path: core::default::Default::default(),
            gcs: core::default::Default::default(),
            remote: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRef
    {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations based on the file type:\nRemote: A checksum must be specified. Cloud Storage: An object generation number must be specified."]
    pub fn allow_insecure(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_insecure", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."]
    pub fn local_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }
    #[doc = "Get a reference to the value of field `gcs` after provisioning.\n"]
    pub fn gcs(
        &self,
    ) -> ListRef<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.gcs", self.base))
    }
    #[doc = "Get a reference to the value of field `remote` after provisioning.\n"]    pub fn remote (& self) -> ListRef < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteElRef >{
        ListRef::new(self.shared().clone(), format!("{}.remote", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElDynamic {
    source: Option<
        DynamicBlock<
            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pull_deps: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<
        Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl>,
    >,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElDynamic,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebEl {
    #[doc = "Set the field `pull_deps`.\nWhether dependencies should also be installed. - install when false: 'dpkg -i package' - install when true: 'apt-get update && apt-get -y install package.deb'"]
    pub fn set_pull_deps(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.pull_deps = Some(v.into());
        self
    }
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebEl {}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebEl {
            pull_deps: core::default::Default::default(),
            source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pull_deps` after provisioning.\nWhether dependencies should also be installed. - install when false: 'dpkg -i package' - install when true: 'apt-get update && apt-get -y install package.deb'"]
    pub fn pull_deps(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.pull_deps", self.base))
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(
        &self,
    ) -> ListRef<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl {
    name: PrimField<String>,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl {}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl {
    #[doc = "Package name."]
    pub name: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl {
            name: self.name,
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nPackage name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl
{
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation: Option<PrimField<f64>>,
    object: PrimField<String>,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl {
    #[doc = "Set the field `generation`.\nGeneration number of the Cloud Storage object."]
    pub fn set_generation(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.generation = Some(v.into());
        self
    }
}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl
{
    #[doc = "Bucket of the Cloud Storage object."]
    pub bucket: PrimField<String>,
    #[doc = "Name of the Cloud Storage object."]
    pub object: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl
    {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl {
            bucket: self.bucket,
            generation: core::default::Default::default(),
            object: self.object,
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsElRef
    {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsElRef { shared : shared , base : base . to_string () , }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nBucket of the Cloud Storage object."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `generation` after provisioning.\nGeneration number of the Cloud Storage object."]
    pub fn generation(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }
    #[doc = "Get a reference to the value of field `object` after provisioning.\nName of the Cloud Storage object."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256_checksum: Option<PrimField<String>>,
    uri: PrimField<String>,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl {
    #[doc = "Set the field `sha256_checksum`.\nSHA256 checksum of the remote file."]
    pub fn set_sha256_checksum(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sha256_checksum = Some(v.into());
        self
    }
}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl
{
    #[doc = "URI from which to fetch the object. It should contain both the protocol and path following the format '{protocol}://{location}'."]
    pub uri: PrimField<String>,
}
impl
    BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl
{
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl
    {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl { sha256_checksum : core :: default :: Default :: default () , uri : self . uri , }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteElRef { fn new (shared : StackShared , base : String) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteElRef { OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteElRef { shared : shared , base : base . to_string () , } } }
impl
    OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `sha256_checksum` after provisioning.\nSHA256 checksum of the remote file."]
    pub fn sha256_checksum(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sha256_checksum", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nURI from which to fetch the object. It should contain both the protocol and path following the format '{protocol}://{location}'."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElDynamic { gcs : Option < DynamicBlock < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl >> , remote : Option < DynamicBlock < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl >> , }
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl { # [serde (skip_serializing_if = "Option::is_none")] allow_insecure : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] local_path : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] gcs : Option < Vec < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl > > , # [serde (skip_serializing_if = "Option::is_none")] remote : Option < Vec < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl > > , dynamic : OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElDynamic , }
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl {
    #[doc = "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations based on the file type:\nRemote: A checksum must be specified. Cloud Storage: An object generation number must be specified."]
    pub fn set_allow_insecure(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_insecure = Some(v.into());
        self
    }
    #[doc = "Set the field `local_path`.\nA local path within the VM to use."]
    pub fn set_local_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_path = Some(v.into());
        self
    }
    #[doc = "Set the field `gcs`.\n"]
    pub fn set_gcs(
        mut self,
        v : impl Into < BlockAssignable < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gcs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `remote`.\n"]
    pub fn set_remote(
        mut self,
        v : impl Into < BlockAssignable < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.remote = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.remote = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl
{}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl {
            allow_insecure: core::default::Default::default(),
            local_path: core::default::Default::default(),
            gcs: core::default::Default::default(),
            remote: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRef
    {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations based on the file type:\nRemote: A checksum must be specified. Cloud Storage: An object generation number must be specified."]
    pub fn allow_insecure(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_insecure", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."]
    pub fn local_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }
    #[doc = "Get a reference to the value of field `gcs` after provisioning.\n"]
    pub fn gcs(
        &self,
    ) -> ListRef<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.gcs", self.base))
    }
    #[doc = "Get a reference to the value of field `remote` after provisioning.\n"]    pub fn remote (& self) -> ListRef < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteElRef >{
        ListRef::new(self.shared().clone(), format!("{}.remote", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElDynamic {
    source: Option<
        DynamicBlock<
            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<
        Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl>,
    >,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElDynamic,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl {
    #[doc = "Set the field `properties`.\nAdditional properties to use during installation. This should be in the format of Property=Setting. Appended to the defaults of 'ACTION=INSTALL REBOOT=ReallySuppress'."]
    pub fn set_properties(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.properties = Some(v.into());
        self
    }
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl {}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl {
            properties: core::default::Default::default(),
            source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\nAdditional properties to use during installation. This should be in the format of Property=Setting. Appended to the defaults of 'ACTION=INSTALL REBOOT=ReallySuppress'."]
    pub fn properties(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(
        &self,
    ) -> ListRef<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl
{
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation: Option<PrimField<f64>>,
    object: PrimField<String>,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl {
    #[doc = "Set the field `generation`.\nGeneration number of the Cloud Storage object."]
    pub fn set_generation(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.generation = Some(v.into());
        self
    }
}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl
{
    #[doc = "Bucket of the Cloud Storage object."]
    pub bucket: PrimField<String>,
    #[doc = "Name of the Cloud Storage object."]
    pub object: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl
    {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl {
            bucket: self.bucket,
            generation: core::default::Default::default(),
            object: self.object,
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsElRef
    {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsElRef { shared : shared , base : base . to_string () , }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nBucket of the Cloud Storage object."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `generation` after provisioning.\nGeneration number of the Cloud Storage object."]
    pub fn generation(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }
    #[doc = "Get a reference to the value of field `object` after provisioning.\nName of the Cloud Storage object."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256_checksum: Option<PrimField<String>>,
    uri: PrimField<String>,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl {
    #[doc = "Set the field `sha256_checksum`.\nSHA256 checksum of the remote file."]
    pub fn set_sha256_checksum(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sha256_checksum = Some(v.into());
        self
    }
}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl
{
    #[doc = "URI from which to fetch the object. It should contain both the protocol and path following the format '{protocol}://{location}'."]
    pub uri: PrimField<String>,
}
impl
    BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl
{
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl
    {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl { sha256_checksum : core :: default :: Default :: default () , uri : self . uri , }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteElRef { fn new (shared : StackShared , base : String) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteElRef { OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteElRef { shared : shared , base : base . to_string () , } } }
impl
    OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `sha256_checksum` after provisioning.\nSHA256 checksum of the remote file."]
    pub fn sha256_checksum(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sha256_checksum", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nURI from which to fetch the object. It should contain both the protocol and path following the format '{protocol}://{location}'."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElDynamic { gcs : Option < DynamicBlock < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl >> , remote : Option < DynamicBlock < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl >> , }
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl { # [serde (skip_serializing_if = "Option::is_none")] allow_insecure : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] local_path : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] gcs : Option < Vec < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl > > , # [serde (skip_serializing_if = "Option::is_none")] remote : Option < Vec < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl > > , dynamic : OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElDynamic , }
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl {
    #[doc = "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations based on the file type:\nRemote: A checksum must be specified. Cloud Storage: An object generation number must be specified."]
    pub fn set_allow_insecure(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_insecure = Some(v.into());
        self
    }
    #[doc = "Set the field `local_path`.\nA local path within the VM to use."]
    pub fn set_local_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_path = Some(v.into());
        self
    }
    #[doc = "Set the field `gcs`.\n"]
    pub fn set_gcs(
        mut self,
        v : impl Into < BlockAssignable < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gcs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `remote`.\n"]
    pub fn set_remote(
        mut self,
        v : impl Into < BlockAssignable < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.remote = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.remote = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl
{}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl {
            allow_insecure: core::default::Default::default(),
            local_path: core::default::Default::default(),
            gcs: core::default::Default::default(),
            remote: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRef
    {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations based on the file type:\nRemote: A checksum must be specified. Cloud Storage: An object generation number must be specified."]
    pub fn allow_insecure(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_insecure", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."]
    pub fn local_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }
    #[doc = "Get a reference to the value of field `gcs` after provisioning.\n"]
    pub fn gcs(
        &self,
    ) -> ListRef<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.gcs", self.base))
    }
    #[doc = "Get a reference to the value of field `remote` after provisioning.\n"]    pub fn remote (& self) -> ListRef < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteElRef >{
        ListRef::new(self.shared().clone(), format!("{}.remote", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElDynamic {
    source: Option<
        DynamicBlock<
            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pull_deps: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<
        Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl>,
    >,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElDynamic,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl {
    #[doc = "Set the field `pull_deps`.\nWhether dependencies should also be installed. - install when false: 'rpm --upgrade --replacepkgs package.rpm' - install when true: 'yum -y install package.rpm' or 'zypper -y install package.rpm'"]
    pub fn set_pull_deps(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.pull_deps = Some(v.into());
        self
    }
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl {}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl {
            pull_deps: core::default::Default::default(),
            source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pull_deps` after provisioning.\nWhether dependencies should also be installed. - install when false: 'rpm --upgrade --replacepkgs package.rpm' - install when true: 'yum -y install package.rpm' or 'zypper -y install package.rpm'"]
    pub fn pull_deps(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.pull_deps", self.base))
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(
        &self,
    ) -> ListRef<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumEl {
    name: PrimField<String>,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumEl {}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumEl {
    #[doc = "Package name."]
    pub name: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumEl {
            name: self.name,
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nPackage name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl {
    name: PrimField<String>,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl {}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl {
    #[doc = "Package name."]
    pub name: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl {
            name: self.name,
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nPackage name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDynamic {
    apt: Option<
        DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptEl>,
    >,
    deb: Option<
        DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebEl>,
    >,
    googet: Option<
        DynamicBlock<
            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl,
        >,
    >,
    msi: Option<
        DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl>,
    >,
    rpm: Option<
        DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl>,
    >,
    yum: Option<
        DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumEl>,
    >,
    zypper: Option<
        DynamicBlock<
            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgEl {
    desired_state: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apt: Option<Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deb: Option<Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    googet:
        Option<Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    msi: Option<Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rpm: Option<Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    yum: Option<Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zypper:
        Option<Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl>>,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDynamic,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgEl {
    #[doc = "Set the field `apt`.\n"]
    pub fn set_apt(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.apt = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.apt = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `deb`.\n"]
    pub fn set_deb(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.deb = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.deb = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `googet`.\n"]
    pub fn set_googet(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.googet = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.googet = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `msi`.\n"]
    pub fn set_msi(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.msi = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.msi = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rpm`.\n"]
    pub fn set_rpm(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rpm = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rpm = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `yum`.\n"]
    pub fn set_yum(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.yum = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.yum = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `zypper`.\n"]
    pub fn set_zypper(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.zypper = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.zypper = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgEl {
    type O =
        BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgEl {
    #[doc = "The desired state the agent should maintain for this package. Possible values: [\"DESIRED_STATE_UNSPECIFIED\", \"INSTALLED\", \"REMOVED\"]"]
    pub desired_state: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgEl {
            desired_state: self.desired_state,
            apt: core::default::Default::default(),
            deb: core::default::Default::default(),
            googet: core::default::Default::default(),
            msi: core::default::Default::default(),
            rpm: core::default::Default::default(),
            yum: core::default::Default::default(),
            zypper: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `desired_state` after provisioning.\nThe desired state the agent should maintain for this package. Possible values: [\"DESIRED_STATE_UNSPECIFIED\", \"INSTALLED\", \"REMOVED\"]"]
    pub fn desired_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `apt` after provisioning.\n"]
    pub fn apt(
        &self,
    ) -> ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.apt", self.base))
    }
    #[doc = "Get a reference to the value of field `deb` after provisioning.\n"]
    pub fn deb(
        &self,
    ) -> ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.deb", self.base))
    }
    #[doc = "Get a reference to the value of field `googet` after provisioning.\n"]
    pub fn googet(
        &self,
    ) -> ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.googet", self.base))
    }
    #[doc = "Get a reference to the value of field `msi` after provisioning.\n"]
    pub fn msi(
        &self,
    ) -> ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.msi", self.base))
    }
    #[doc = "Get a reference to the value of field `rpm` after provisioning.\n"]
    pub fn rpm(
        &self,
    ) -> ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.rpm", self.base))
    }
    #[doc = "Get a reference to the value of field `yum` after provisioning.\n"]
    pub fn yum(
        &self,
    ) -> ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.yum", self.base))
    }
    #[doc = "Get a reference to the value of field `zypper` after provisioning.\n"]
    pub fn zypper(
        &self,
    ) -> ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.zypper", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl {
    archive_type: PrimField<String>,
    components: ListField<PrimField<String>>,
    distribution: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpg_key: Option<PrimField<String>>,
    uri: PrimField<String>,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl {
    #[doc = "Set the field `gpg_key`.\nURI of the key file for this repository. The agent maintains a keyring at '/etc/apt/trusted.gpg.d/osconfig_agent_managed.gpg'."]
    pub fn set_gpg_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gpg_key = Some(v.into());
        self
    }
}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl {
    #[doc = "Type of archive files in this repository. Possible values: [\"ARCHIVE_TYPE_UNSPECIFIED\", \"DEB\", \"DEB_SRC\"]"]
    pub archive_type: PrimField<String>,
    #[doc = "List of components for this repository. Must contain at least one item."]
    pub components: ListField<PrimField<String>>,
    #[doc = "Distribution of this repository."]
    pub distribution: PrimField<String>,
    #[doc = "URI for this repository."]
    pub uri: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl {
            archive_type: self.archive_type,
            components: self.components,
            distribution: self.distribution,
            gpg_key: core::default::Default::default(),
            uri: self.uri,
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `archive_type` after provisioning.\nType of archive files in this repository. Possible values: [\"ARCHIVE_TYPE_UNSPECIFIED\", \"DEB\", \"DEB_SRC\"]"]
    pub fn archive_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.archive_type", self.base))
    }
    #[doc = "Get a reference to the value of field `components` after provisioning.\nList of components for this repository. Must contain at least one item."]
    pub fn components(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.components", self.base))
    }
    #[doc = "Get a reference to the value of field `distribution` after provisioning.\nDistribution of this repository."]
    pub fn distribution(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.distribution", self.base))
    }
    #[doc = "Get a reference to the value of field `gpg_key` after provisioning.\nURI of the key file for this repository. The agent maintains a keyring at '/etc/apt/trusted.gpg.d/osconfig_agent_managed.gpg'."]
    pub fn gpg_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gpg_key", self.base))
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nURI for this repository."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl {
    name: PrimField<String>,
    url: PrimField<String>,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl {}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl {
    #[doc = "The name of the repository."]
    pub name: PrimField<String>,
    #[doc = "The url of the repository."]
    pub url: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl {
            name: self.name,
            url: self.url,
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the repository."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe url of the repository."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl {
    base_url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpg_keys: Option<ListField<PrimField<String>>>,
    id: PrimField<String>,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl {
    #[doc = "Set the field `display_name`.\nThe display name of the repository."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `gpg_keys`.\nURIs of GPG keys."]
    pub fn set_gpg_keys(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.gpg_keys = Some(v.into());
        self
    }
}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl {
    #[doc = "The location of the repository directory."]
    pub base_url: PrimField<String>,
    #[doc = "A one word, unique name for this repository. This is the 'repo id' in the yum config file and also the 'display_name' if 'display_name' is omitted. This id is also used as the unique identifier when checking for resource conflicts."]
    pub id: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl {
            base_url: self.base_url,
            display_name: core::default::Default::default(),
            gpg_keys: core::default::Default::default(),
            id: self.id,
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `base_url` after provisioning.\nThe location of the repository directory."]
    pub fn base_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base_url", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the repository."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `gpg_keys` after provisioning.\nURIs of GPG keys."]
    pub fn gpg_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.gpg_keys", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nA one word, unique name for this repository. This is the 'repo id' in the yum config file and also the 'display_name' if 'display_name' is omitted. This id is also used as the unique identifier when checking for resource conflicts."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl {
    base_url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpg_keys: Option<ListField<PrimField<String>>>,
    id: PrimField<String>,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl {
    #[doc = "Set the field `display_name`.\nThe display name of the repository."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `gpg_keys`.\nURIs of GPG keys."]
    pub fn set_gpg_keys(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.gpg_keys = Some(v.into());
        self
    }
}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl
{
    #[doc = "The location of the repository directory."]
    pub base_url: PrimField<String>,
    #[doc = "A one word, unique name for this repository. This is the 'repo id' in the zypper config file and also the 'display_name' if 'display_name' is omitted. This id is also used as the unique identifier when checking for GuestPolicy conflicts."]
    pub id: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl {
            base_url: self.base_url,
            display_name: core::default::Default::default(),
            gpg_keys: core::default::Default::default(),
            id: self.id,
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperElRef
    {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `base_url` after provisioning.\nThe location of the repository directory."]
    pub fn base_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base_url", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the repository."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `gpg_keys` after provisioning.\nURIs of GPG keys."]
    pub fn gpg_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.gpg_keys", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nA one word, unique name for this repository. This is the 'repo id' in the zypper config file and also the 'display_name' if 'display_name' is omitted. This id is also used as the unique identifier when checking for GuestPolicy conflicts."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElDynamic {
    apt: Option<
        DynamicBlock<
            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl,
        >,
    >,
    goo: Option<
        DynamicBlock<
            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl,
        >,
    >,
    yum: Option<
        DynamicBlock<
            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl,
        >,
    >,
    zypper: Option<
        DynamicBlock<
            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    apt: Option<
        Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    goo: Option<
        Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    yum: Option<
        Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    zypper: Option<
        Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl>,
    >,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElDynamic,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryEl {
    #[doc = "Set the field `apt`.\n"]
    pub fn set_apt(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.apt = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.apt = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `goo`.\n"]
    pub fn set_goo(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.goo = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.goo = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `yum`.\n"]
    pub fn set_yum(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.yum = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.yum = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `zypper`.\n"]
    pub fn set_zypper(
        mut self,
        v : impl Into < BlockAssignable < OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.zypper = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.zypper = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryEl
{
    type O = BlockAssignable<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryEl {}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryEl {
            apt: core::default::Default::default(),
            goo: core::default::Default::default(),
            yum: core::default::Default::default(),
            zypper: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `apt` after provisioning.\n"]
    pub fn apt(
        &self,
    ) -> ListRef<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.apt", self.base))
    }
    #[doc = "Get a reference to the value of field `goo` after provisioning.\n"]
    pub fn goo(
        &self,
    ) -> ListRef<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.goo", self.base))
    }
    #[doc = "Get a reference to the value of field `yum` after provisioning.\n"]
    pub fn yum(
        &self,
    ) -> ListRef<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.yum", self.base))
    }
    #[doc = "Get a reference to the value of field `zypper` after provisioning.\n"]
    pub fn zypper(
        &self,
    ) -> ListRef<
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.zypper", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElDynamic {
    exec: Option<
        DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecEl>,
    >,
    file: Option<
        DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileEl>,
    >,
    pkg: Option<
        DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgEl>,
    >,
    repository: Option<
        DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryEl>,
    >,
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesEl {
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exec: Option<Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pkg: Option<Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository:
        Option<Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryEl>>,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElDynamic,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesEl {
    #[doc = "Set the field `exec`.\n"]
    pub fn set_exec(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.file = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.file = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `pkg`.\n"]
    pub fn set_pkg(
        mut self,
        v: impl Into<
            BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pkg = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pkg = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `repository`.\n"]
    pub fn set_repository(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.repository = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.repository = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesEl {
    type O = BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesEl {
    #[doc = "The id of the resource with the following restrictions:\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the OS policy."]
    pub id: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesEl {
            id: self.id,
            exec: core::default::Default::default(),
            file: core::default::Default::default(),
            pkg: core::default::Default::default(),
            repository: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe id of the resource with the following restrictions:\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the OS policy."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `exec` after provisioning.\n"]
    pub fn exec(
        &self,
    ) -> ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.exec", self.base))
    }
    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }
    #[doc = "Get a reference to the value of field `pkg` after provisioning.\n"]
    pub fn pkg(
        &self,
    ) -> ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pkg", self.base))
    }
    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(
        &self,
    ) -> ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.repository", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElDynamic {
    inventory_filters: Option<
        DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersEl>,
    >,
    resources:
        Option<DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesEl>>,
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    inventory_filters:
        Option<Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesEl>>,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElDynamic,
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsEl {
    #[doc = "Set the field `inventory_filters`.\n"]
    pub fn set_inventory_filters(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.inventory_filters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.inventory_filters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v: impl Into<BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resources = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsEl {
    type O = BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsEl {}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsEl {
            inventory_filters: core::default::Default::default(),
            resources: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `inventory_filters` after provisioning.\n"]
    pub fn inventory_filters(
        &self,
    ) -> ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inventory_filters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(
        &self,
    ) -> ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElDynamic {
    resource_groups: Option<DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsEl>>,
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_no_resource_group_match: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    id: PrimField<String>,
    mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_groups: Option<Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsEl>>,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElDynamic,
}
impl OsConfigOsPolicyAssignmentOsPoliciesEl {
    #[doc = "Set the field `allow_no_resource_group_match`.\nThis flag determines the OS policy compliance status when none of the resource groups within the policy are applicable for a VM. Set this value to 'true' if the policy needs to be reported as compliant even if the policy has nothing to validate or enforce."]
    pub fn set_allow_no_resource_group_match(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_no_resource_group_match = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nPolicy description. Length of the description is limited to 1024 characters."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_groups`.\n"]
    pub fn set_resource_groups(
        mut self,
        v: impl Into<BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_groups = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_groups = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesEl {
    type O = BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesEl {
    #[doc = "The id of the OS policy with the following restrictions:\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the assignment."]
    pub id: PrimField<String>,
    #[doc = "Policy mode Possible values: [\"MODE_UNSPECIFIED\", \"VALIDATION\", \"ENFORCEMENT\"]"]
    pub mode: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignmentOsPoliciesEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesEl {
        OsConfigOsPolicyAssignmentOsPoliciesEl {
            allow_no_resource_group_match: core::default::Default::default(),
            description: core::default::Default::default(),
            id: self.id,
            mode: self.mode,
            resource_groups: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentOsPoliciesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentOsPoliciesElRef {
    fn new(shared: StackShared, base: String) -> OsConfigOsPolicyAssignmentOsPoliciesElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentOsPoliciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_no_resource_group_match` after provisioning.\nThis flag determines the OS policy compliance status when none of the resource groups within the policy are applicable for a VM. Set this value to 'true' if the policy needs to be reported as compliant even if the policy has nothing to validate or enforce."]
    pub fn allow_no_resource_group_match(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_no_resource_group_match", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nPolicy description. Length of the description is limited to 1024 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe id of the OS policy with the following restrictions:\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the assignment."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nPolicy mode Possible values: [\"MODE_UNSPECIFIED\", \"VALIDATION\", \"ENFORCEMENT\"]"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_groups` after provisioning.\n"]
    pub fn resource_groups(
        &self,
    ) -> ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_groups", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percent: Option<PrimField<f64>>,
}
impl OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetEl {
    #[doc = "Set the field `fixed`.\nSpecifies a fixed value."]
    pub fn set_fixed(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.fixed = Some(v.into());
        self
    }
    #[doc = "Set the field `percent`.\nSpecifies the relative value defined as a percentage, which will be multiplied by a reference value."]
    pub fn set_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percent = Some(v.into());
        self
    }
}
impl ToListMappable for OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetEl {
    type O = BlockAssignable<OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentRolloutElDisruptionBudgetEl {}
impl BuildOsConfigOsPolicyAssignmentRolloutElDisruptionBudgetEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetEl {
        OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetEl {
            fixed: core::default::Default::default(),
            percent: core::default::Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetElRef {
        OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `fixed` after provisioning.\nSpecifies a fixed value."]
    pub fn fixed(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.fixed", self.base))
    }
    #[doc = "Get a reference to the value of field `percent` after provisioning.\nSpecifies the relative value defined as a percentage, which will be multiplied by a reference value."]
    pub fn percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentRolloutElDynamic {
    disruption_budget: Option<DynamicBlock<OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetEl>>,
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentRolloutEl {
    min_wait_duration: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disruption_budget: Option<Vec<OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetEl>>,
    dynamic: OsConfigOsPolicyAssignmentRolloutElDynamic,
}
impl OsConfigOsPolicyAssignmentRolloutEl {
    #[doc = "Set the field `disruption_budget`.\n"]
    pub fn set_disruption_budget(
        mut self,
        v: impl Into<BlockAssignable<OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.disruption_budget = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.disruption_budget = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OsConfigOsPolicyAssignmentRolloutEl {
    type O = BlockAssignable<OsConfigOsPolicyAssignmentRolloutEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentRolloutEl {
    #[doc = "This determines the minimum duration of time to wait after the configuration changes are applied through the current rollout. A VM continues to count towards the 'disruption_budget' at least until this duration of time has passed after configuration changes are applied."]
    pub min_wait_duration: PrimField<String>,
}
impl BuildOsConfigOsPolicyAssignmentRolloutEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentRolloutEl {
        OsConfigOsPolicyAssignmentRolloutEl {
            min_wait_duration: self.min_wait_duration,
            disruption_budget: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentRolloutElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentRolloutElRef {
    fn new(shared: StackShared, base: String) -> OsConfigOsPolicyAssignmentRolloutElRef {
        OsConfigOsPolicyAssignmentRolloutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentRolloutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `min_wait_duration` after provisioning.\nThis determines the minimum duration of time to wait after the configuration changes are applied through the current rollout. A VM continues to count towards the 'disruption_budget' at least until this duration of time has passed after configuration changes are applied."]
    pub fn min_wait_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_wait_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disruption_budget` after provisioning.\n"]
    pub fn disruption_budget(
        &self,
    ) -> ListRef<OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disruption_budget", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl OsConfigOsPolicyAssignmentTimeoutsEl {
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
impl ToListMappable for OsConfigOsPolicyAssignmentTimeoutsEl {
    type O = BlockAssignable<OsConfigOsPolicyAssignmentTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigOsPolicyAssignmentTimeoutsEl {}
impl BuildOsConfigOsPolicyAssignmentTimeoutsEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentTimeoutsEl {
        OsConfigOsPolicyAssignmentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct OsConfigOsPolicyAssignmentTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigOsPolicyAssignmentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> OsConfigOsPolicyAssignmentTimeoutsElRef {
        OsConfigOsPolicyAssignmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigOsPolicyAssignmentTimeoutsElRef {
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
struct OsConfigOsPolicyAssignmentDynamic {
    instance_filter: Option<DynamicBlock<OsConfigOsPolicyAssignmentInstanceFilterEl>>,
    os_policies: Option<DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesEl>>,
    rollout: Option<DynamicBlock<OsConfigOsPolicyAssignmentRolloutEl>>,
}
