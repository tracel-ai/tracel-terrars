use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct OsConfigPatchDeploymentData {
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
    duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    patch_deployment_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_filter: Option<Vec<OsConfigPatchDeploymentInstanceFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    one_time_schedule: Option<Vec<OsConfigPatchDeploymentOneTimeScheduleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    patch_config: Option<Vec<OsConfigPatchDeploymentPatchConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recurring_schedule: Option<Vec<OsConfigPatchDeploymentRecurringScheduleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rollout: Option<Vec<OsConfigPatchDeploymentRolloutEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<OsConfigPatchDeploymentTimeoutsEl>,
    dynamic: OsConfigPatchDeploymentDynamic,
}
struct OsConfigPatchDeployment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OsConfigPatchDeploymentData>,
}
#[derive(Clone)]
pub struct OsConfigPatchDeployment(Rc<OsConfigPatchDeployment_>);
impl OsConfigPatchDeployment {
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
    #[doc = "Set the field `description`.\nDescription of the patch deployment. Length of the description is limited to 1024 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `duration`.\nDuration of the patch. After the duration ends, the patch times out.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\""]
    pub fn set_duration(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().duration = Some(v.into());
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
    #[doc = "Set the field `instance_filter`.\n"]
    pub fn set_instance_filter(
        self,
        v: impl Into<BlockAssignable<OsConfigPatchDeploymentInstanceFilterEl>>,
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
    #[doc = "Set the field `one_time_schedule`.\n"]
    pub fn set_one_time_schedule(
        self,
        v: impl Into<BlockAssignable<OsConfigPatchDeploymentOneTimeScheduleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().one_time_schedule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.one_time_schedule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `patch_config`.\n"]
    pub fn set_patch_config(
        self,
        v: impl Into<BlockAssignable<OsConfigPatchDeploymentPatchConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().patch_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.patch_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `recurring_schedule`.\n"]
    pub fn set_recurring_schedule(
        self,
        v: impl Into<BlockAssignable<OsConfigPatchDeploymentRecurringScheduleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().recurring_schedule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.recurring_schedule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rollout`.\n"]
    pub fn set_rollout(
        self,
        v: impl Into<BlockAssignable<OsConfigPatchDeploymentRolloutEl>>,
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
    pub fn set_timeouts(self, v: impl Into<OsConfigPatchDeploymentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the patch deployment was created. Timestamp is in RFC3339 text format.\nA timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the patch deployment. Length of the description is limited to 1024 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `duration` after provisioning.\nDuration of the patch. After the duration ends, the patch times out.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\""]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_execute_time` after provisioning.\nThe last time a patch job was started by this deployment. Timestamp is in RFC3339 text format.\nA timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn last_execute_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_execute_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nUnique name for the patch deployment resource in a project.\nThe patch deployment name is in the form: projects/{project_id}/patchDeployments/{patchDeploymentId}."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `patch_deployment_id` after provisioning.\nA name for the patch deployment in the project. When creating a name the following rules apply:\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the project."]
    pub fn patch_deployment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.patch_deployment_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the patch deployment was last updated. Timestamp is in RFC3339 text format.\nA timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_filter` after provisioning.\n"]
    pub fn instance_filter(&self) -> ListRef<OsConfigPatchDeploymentInstanceFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `one_time_schedule` after provisioning.\n"]
    pub fn one_time_schedule(&self) -> ListRef<OsConfigPatchDeploymentOneTimeScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.one_time_schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `patch_config` after provisioning.\n"]
    pub fn patch_config(&self) -> ListRef<OsConfigPatchDeploymentPatchConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.patch_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `recurring_schedule` after provisioning.\n"]
    pub fn recurring_schedule(&self) -> ListRef<OsConfigPatchDeploymentRecurringScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.recurring_schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rollout` after provisioning.\n"]
    pub fn rollout(&self) -> ListRef<OsConfigPatchDeploymentRolloutElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rollout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OsConfigPatchDeploymentTimeoutsElRef {
        OsConfigPatchDeploymentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for OsConfigPatchDeployment {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for OsConfigPatchDeployment {}
impl ToListMappable for OsConfigPatchDeployment {
    type O = ListRef<OsConfigPatchDeploymentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for OsConfigPatchDeployment_ {
    fn extract_resource_type(&self) -> String {
        "google_os_config_patch_deployment".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildOsConfigPatchDeployment {
    pub tf_id: String,
    #[doc = "A name for the patch deployment in the project. When creating a name the following rules apply:\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the project."]
    pub patch_deployment_id: PrimField<String>,
}
impl BuildOsConfigPatchDeployment {
    pub fn build(self, stack: &mut Stack) -> OsConfigPatchDeployment {
        let out = OsConfigPatchDeployment(Rc::new(OsConfigPatchDeployment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OsConfigPatchDeploymentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                duration: core::default::Default::default(),
                id: core::default::Default::default(),
                patch_deployment_id: self.patch_deployment_id,
                project: core::default::Default::default(),
                instance_filter: core::default::Default::default(),
                one_time_schedule: core::default::Default::default(),
                patch_config: core::default::Default::default(),
                recurring_schedule: core::default::Default::default(),
                rollout: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct OsConfigPatchDeploymentRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl OsConfigPatchDeploymentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the patch deployment was created. Timestamp is in RFC3339 text format.\nA timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the patch deployment. Length of the description is limited to 1024 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `duration` after provisioning.\nDuration of the patch. After the duration ends, the patch times out.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\""]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_execute_time` after provisioning.\nThe last time a patch job was started by this deployment. Timestamp is in RFC3339 text format.\nA timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn last_execute_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_execute_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nUnique name for the patch deployment resource in a project.\nThe patch deployment name is in the form: projects/{project_id}/patchDeployments/{patchDeploymentId}."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `patch_deployment_id` after provisioning.\nA name for the patch deployment in the project. When creating a name the following rules apply:\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the project."]
    pub fn patch_deployment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.patch_deployment_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the patch deployment was last updated. Timestamp is in RFC3339 text format.\nA timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_filter` after provisioning.\n"]
    pub fn instance_filter(&self) -> ListRef<OsConfigPatchDeploymentInstanceFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `one_time_schedule` after provisioning.\n"]
    pub fn one_time_schedule(&self) -> ListRef<OsConfigPatchDeploymentOneTimeScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.one_time_schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `patch_config` after provisioning.\n"]
    pub fn patch_config(&self) -> ListRef<OsConfigPatchDeploymentPatchConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.patch_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `recurring_schedule` after provisioning.\n"]
    pub fn recurring_schedule(&self) -> ListRef<OsConfigPatchDeploymentRecurringScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.recurring_schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rollout` after provisioning.\n"]
    pub fn rollout(&self) -> ListRef<OsConfigPatchDeploymentRolloutElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rollout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OsConfigPatchDeploymentTimeoutsElRef {
        OsConfigPatchDeploymentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentInstanceFilterElGroupLabelsEl {
    labels: RecField<PrimField<String>>,
}
impl OsConfigPatchDeploymentInstanceFilterElGroupLabelsEl {}
impl ToListMappable for OsConfigPatchDeploymentInstanceFilterElGroupLabelsEl {
    type O = BlockAssignable<OsConfigPatchDeploymentInstanceFilterElGroupLabelsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentInstanceFilterElGroupLabelsEl {
    #[doc = "Compute Engine instance labels that must be present for a VM instance to be targeted by this filter"]
    pub labels: RecField<PrimField<String>>,
}
impl BuildOsConfigPatchDeploymentInstanceFilterElGroupLabelsEl {
    pub fn build(self) -> OsConfigPatchDeploymentInstanceFilterElGroupLabelsEl {
        OsConfigPatchDeploymentInstanceFilterElGroupLabelsEl {
            labels: self.labels,
        }
    }
}
pub struct OsConfigPatchDeploymentInstanceFilterElGroupLabelsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentInstanceFilterElGroupLabelsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigPatchDeploymentInstanceFilterElGroupLabelsElRef {
        OsConfigPatchDeploymentInstanceFilterElGroupLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentInstanceFilterElGroupLabelsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nCompute Engine instance labels that must be present for a VM instance to be targeted by this filter"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigPatchDeploymentInstanceFilterElDynamic {
    group_labels: Option<DynamicBlock<OsConfigPatchDeploymentInstanceFilterElGroupLabelsEl>>,
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentInstanceFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_name_prefixes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instances: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zones: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_labels: Option<Vec<OsConfigPatchDeploymentInstanceFilterElGroupLabelsEl>>,
    dynamic: OsConfigPatchDeploymentInstanceFilterElDynamic,
}
impl OsConfigPatchDeploymentInstanceFilterEl {
    #[doc = "Set the field `all`.\nTarget all VM instances in the project. If true, no other criteria is permitted."]
    pub fn set_all(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.all = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_name_prefixes`.\nTargets VMs whose name starts with one of these prefixes. Similar to labels, this is another way to group\nVMs when targeting configs, for example prefix=\"prod-\"."]
    pub fn set_instance_name_prefixes(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.instance_name_prefixes = Some(v.into());
        self
    }
    #[doc = "Set the field `instances`.\nTargets any of the VM instances specified. Instances are specified by their URI in the 'form zones/{{zone}}/instances/{{instance_name}}',\n'projects/{{project_id}}/zones/{{zone}}/instances/{{instance_name}}', or\n'https://www.googleapis.com/compute/v1/projects/{{project_id}}/zones/{{zone}}/instances/{{instance_name}}'"]
    pub fn set_instances(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.instances = Some(v.into());
        self
    }
    #[doc = "Set the field `zones`.\nTargets VM instances in ANY of these zones. Leave empty to target VM instances in any zone."]
    pub fn set_zones(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.zones = Some(v.into());
        self
    }
    #[doc = "Set the field `group_labels`.\n"]
    pub fn set_group_labels(
        mut self,
        v: impl Into<BlockAssignable<OsConfigPatchDeploymentInstanceFilterElGroupLabelsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.group_labels = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.group_labels = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OsConfigPatchDeploymentInstanceFilterEl {
    type O = BlockAssignable<OsConfigPatchDeploymentInstanceFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentInstanceFilterEl {}
impl BuildOsConfigPatchDeploymentInstanceFilterEl {
    pub fn build(self) -> OsConfigPatchDeploymentInstanceFilterEl {
        OsConfigPatchDeploymentInstanceFilterEl {
            all: core::default::Default::default(),
            instance_name_prefixes: core::default::Default::default(),
            instances: core::default::Default::default(),
            zones: core::default::Default::default(),
            group_labels: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigPatchDeploymentInstanceFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentInstanceFilterElRef {
    fn new(shared: StackShared, base: String) -> OsConfigPatchDeploymentInstanceFilterElRef {
        OsConfigPatchDeploymentInstanceFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentInstanceFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `all` after provisioning.\nTarget all VM instances in the project. If true, no other criteria is permitted."]
    pub fn all(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.all", self.base))
    }
    #[doc = "Get a reference to the value of field `instance_name_prefixes` after provisioning.\nTargets VMs whose name starts with one of these prefixes. Similar to labels, this is another way to group\nVMs when targeting configs, for example prefix=\"prod-\"."]
    pub fn instance_name_prefixes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_name_prefixes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instances` after provisioning.\nTargets any of the VM instances specified. Instances are specified by their URI in the 'form zones/{{zone}}/instances/{{instance_name}}',\n'projects/{{project_id}}/zones/{{zone}}/instances/{{instance_name}}', or\n'https://www.googleapis.com/compute/v1/projects/{{project_id}}/zones/{{zone}}/instances/{{instance_name}}'"]
    pub fn instances(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.instances", self.base))
    }
    #[doc = "Get a reference to the value of field `zones` after provisioning.\nTargets VM instances in ANY of these zones. Leave empty to target VM instances in any zone."]
    pub fn zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.zones", self.base))
    }
    #[doc = "Get a reference to the value of field `group_labels` after provisioning.\n"]
    pub fn group_labels(&self) -> ListRef<OsConfigPatchDeploymentInstanceFilterElGroupLabelsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.group_labels", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentOneTimeScheduleEl {
    execute_time: PrimField<String>,
}
impl OsConfigPatchDeploymentOneTimeScheduleEl {}
impl ToListMappable for OsConfigPatchDeploymentOneTimeScheduleEl {
    type O = BlockAssignable<OsConfigPatchDeploymentOneTimeScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentOneTimeScheduleEl {
    #[doc = "The desired patch job execution time. A timestamp in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub execute_time: PrimField<String>,
}
impl BuildOsConfigPatchDeploymentOneTimeScheduleEl {
    pub fn build(self) -> OsConfigPatchDeploymentOneTimeScheduleEl {
        OsConfigPatchDeploymentOneTimeScheduleEl {
            execute_time: self.execute_time,
        }
    }
}
pub struct OsConfigPatchDeploymentOneTimeScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentOneTimeScheduleElRef {
    fn new(shared: StackShared, base: String) -> OsConfigPatchDeploymentOneTimeScheduleElRef {
        OsConfigPatchDeploymentOneTimeScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentOneTimeScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `execute_time` after provisioning.\nThe desired patch job execution time. A timestamp in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn execute_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execute_time", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElAptEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    excludes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusive_packages: Option<ListField<PrimField<String>>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl OsConfigPatchDeploymentPatchConfigElAptEl {
    #[doc = "Set the field `excludes`.\nList of packages to exclude from update. These packages will be excluded."]
    pub fn set_excludes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.excludes = Some(v.into());
        self
    }
    #[doc = "Set the field `exclusive_packages`.\nAn exclusive list of packages to be updated. These are the only packages that will be updated.\nIf these packages are not installed, they will be ignored. This field cannot be specified with\nany other patch configuration fields."]
    pub fn set_exclusive_packages(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclusive_packages = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nBy changing the type to DIST, the patching is performed using apt-get dist-upgrade instead. Possible values: [\"DIST\", \"UPGRADE\"]"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for OsConfigPatchDeploymentPatchConfigElAptEl {
    type O = BlockAssignable<OsConfigPatchDeploymentPatchConfigElAptEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentPatchConfigElAptEl {}
impl BuildOsConfigPatchDeploymentPatchConfigElAptEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElAptEl {
        OsConfigPatchDeploymentPatchConfigElAptEl {
            excludes: core::default::Default::default(),
            exclusive_packages: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct OsConfigPatchDeploymentPatchConfigElAptElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentPatchConfigElAptElRef {
    fn new(shared: StackShared, base: String) -> OsConfigPatchDeploymentPatchConfigElAptElRef {
        OsConfigPatchDeploymentPatchConfigElAptElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentPatchConfigElAptElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `excludes` after provisioning.\nList of packages to exclude from update. These packages will be excluded."]
    pub fn excludes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }
    #[doc = "Get a reference to the value of field `exclusive_packages` after provisioning.\nAn exclusive list of packages to be updated. These are the only packages that will be updated.\nIf these packages are not installed, they will be ignored. This field cannot be specified with\nany other patch configuration fields."]
    pub fn exclusive_packages(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclusive_packages", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nBy changing the type to DIST, the patching is performed using apt-get dist-upgrade instead. Possible values: [\"DIST\", \"UPGRADE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElGooEl {
    enabled: PrimField<bool>,
}
impl OsConfigPatchDeploymentPatchConfigElGooEl {}
impl ToListMappable for OsConfigPatchDeploymentPatchConfigElGooEl {
    type O = BlockAssignable<OsConfigPatchDeploymentPatchConfigElGooEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentPatchConfigElGooEl {
    #[doc = "goo update settings. Use this setting to override the default goo patch rules."]
    pub enabled: PrimField<bool>,
}
impl BuildOsConfigPatchDeploymentPatchConfigElGooEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElGooEl {
        OsConfigPatchDeploymentPatchConfigElGooEl {
            enabled: self.enabled,
        }
    }
}
pub struct OsConfigPatchDeploymentPatchConfigElGooElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentPatchConfigElGooElRef {
    fn new(shared: StackShared, base: String) -> OsConfigPatchDeploymentPatchConfigElGooElRef {
        OsConfigPatchDeploymentPatchConfigElGooElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentPatchConfigElGooElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\ngoo update settings. Use this setting to override the default goo patch rules."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectEl {
    bucket: PrimField<String>,
    generation_number: PrimField<String>,
    object: PrimField<String>,
}
impl OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectEl {}
impl ToListMappable
    for OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectEl
{
    type O = BlockAssignable<
        OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectEl {
    #[doc = "Bucket of the Cloud Storage object."]
    pub bucket: PrimField<String>,
    #[doc = "Generation number of the Cloud Storage object. This is used to ensure that the ExecStep specified by this PatchJob does not change."]
    pub generation_number: PrimField<String>,
    #[doc = "Name of the Cloud Storage object."]
    pub object: PrimField<String>,
}
impl BuildOsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectEl {
    pub fn build(
        self,
    ) -> OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectEl {
        OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectEl {
            bucket: self.bucket,
            generation_number: self.generation_number,
            object: self.object,
        }
    }
}
pub struct OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectElRef {
        OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nBucket of the Cloud Storage object."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `generation_number` after provisioning.\nGeneration number of the Cloud Storage object. This is used to ensure that the ExecStep specified by this PatchJob does not change."]
    pub fn generation_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generation_number", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `object` after provisioning.\nName of the Cloud Storage object."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElDynamic {
    gcs_object: Option<
        DynamicBlock<
            OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_success_codes: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interpreter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_object:
        Option<Vec<OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectEl>>,
    dynamic: OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElDynamic,
}
impl OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigEl {
    #[doc = "Set the field `allowed_success_codes`.\nDefaults to [0]. A list of possible return values that the execution can return to indicate a success."]
    pub fn set_allowed_success_codes(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.allowed_success_codes = Some(v.into());
        self
    }
    #[doc = "Set the field `interpreter`.\nThe script interpreter to use to run the script. If no interpreter is specified the script will\nbe executed directly, which will likely only succeed for scripts with shebang lines. Possible values: [\"SHELL\", \"POWERSHELL\"]"]
    pub fn set_interpreter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interpreter = Some(v.into());
        self
    }
    #[doc = "Set the field `local_path`.\nAn absolute path to the executable on the VM."]
    pub fn set_local_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_path = Some(v.into());
        self
    }
    #[doc = "Set the field `gcs_object`.\n"]
    pub fn set_gcs_object(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gcs_object = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs_object = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigEl {
    type O = BlockAssignable<OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigEl {}
impl BuildOsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigEl {
        OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigEl {
            allowed_success_codes: core::default::Default::default(),
            interpreter: core::default::Default::default(),
            local_path: core::default::Default::default(),
            gcs_object: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElRef {
        OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_success_codes` after provisioning.\nDefaults to [0]. A list of possible return values that the execution can return to indicate a success."]
    pub fn allowed_success_codes(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_success_codes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `interpreter` after provisioning.\nThe script interpreter to use to run the script. If no interpreter is specified the script will\nbe executed directly, which will likely only succeed for scripts with shebang lines. Possible values: [\"SHELL\", \"POWERSHELL\"]"]
    pub fn interpreter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interpreter", self.base))
    }
    #[doc = "Get a reference to the value of field `local_path` after provisioning.\nAn absolute path to the executable on the VM."]
    pub fn local_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }
    #[doc = "Get a reference to the value of field `gcs_object` after provisioning.\n"]
    pub fn gcs_object(
        &self,
    ) -> ListRef<OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.gcs_object", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectEl {
    bucket: PrimField<String>,
    generation_number: PrimField<String>,
    object: PrimField<String>,
}
impl OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectEl {}
impl ToListMappable
    for OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectEl
{
    type O = BlockAssignable<
        OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectEl {
    #[doc = "Bucket of the Cloud Storage object."]
    pub bucket: PrimField<String>,
    #[doc = "Generation number of the Cloud Storage object. This is used to ensure that the ExecStep specified by this PatchJob does not change."]
    pub generation_number: PrimField<String>,
    #[doc = "Name of the Cloud Storage object."]
    pub object: PrimField<String>,
}
impl BuildOsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectEl {
    pub fn build(
        self,
    ) -> OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectEl {
        OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectEl {
            bucket: self.bucket,
            generation_number: self.generation_number,
            object: self.object,
        }
    }
}
pub struct OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectElRef {
        OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nBucket of the Cloud Storage object."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `generation_number` after provisioning.\nGeneration number of the Cloud Storage object. This is used to ensure that the ExecStep specified by this PatchJob does not change."]
    pub fn generation_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generation_number", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `object` after provisioning.\nName of the Cloud Storage object."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElDynamic {
    gcs_object: Option<
        DynamicBlock<
            OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_success_codes: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interpreter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_object: Option<
        Vec<OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectEl>,
    >,
    dynamic: OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElDynamic,
}
impl OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigEl {
    #[doc = "Set the field `allowed_success_codes`.\nDefaults to [0]. A list of possible return values that the execution can return to indicate a success."]
    pub fn set_allowed_success_codes(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.allowed_success_codes = Some(v.into());
        self
    }
    #[doc = "Set the field `interpreter`.\nThe script interpreter to use to run the script. If no interpreter is specified the script will\nbe executed directly, which will likely only succeed for scripts with shebang lines. Possible values: [\"SHELL\", \"POWERSHELL\"]"]
    pub fn set_interpreter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interpreter = Some(v.into());
        self
    }
    #[doc = "Set the field `local_path`.\nAn absolute path to the executable on the VM."]
    pub fn set_local_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_path = Some(v.into());
        self
    }
    #[doc = "Set the field `gcs_object`.\n"]
    pub fn set_gcs_object(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gcs_object = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs_object = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigEl {
    type O = BlockAssignable<OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigEl {}
impl BuildOsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigEl {
        OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigEl {
            allowed_success_codes: core::default::Default::default(),
            interpreter: core::default::Default::default(),
            local_path: core::default::Default::default(),
            gcs_object: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElRef {
        OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_success_codes` after provisioning.\nDefaults to [0]. A list of possible return values that the execution can return to indicate a success."]
    pub fn allowed_success_codes(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_success_codes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `interpreter` after provisioning.\nThe script interpreter to use to run the script. If no interpreter is specified the script will\nbe executed directly, which will likely only succeed for scripts with shebang lines. Possible values: [\"SHELL\", \"POWERSHELL\"]"]
    pub fn interpreter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interpreter", self.base))
    }
    #[doc = "Get a reference to the value of field `local_path` after provisioning.\nAn absolute path to the executable on the VM."]
    pub fn local_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }
    #[doc = "Get a reference to the value of field `gcs_object` after provisioning.\n"]
    pub fn gcs_object(
        &self,
    ) -> ListRef<OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.gcs_object", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigPatchDeploymentPatchConfigElPostStepElDynamic {
    linux_exec_step_config:
        Option<DynamicBlock<OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigEl>>,
    windows_exec_step_config:
        Option<DynamicBlock<OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigEl>>,
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElPostStepEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    linux_exec_step_config:
        Option<Vec<OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    windows_exec_step_config:
        Option<Vec<OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigEl>>,
    dynamic: OsConfigPatchDeploymentPatchConfigElPostStepElDynamic,
}
impl OsConfigPatchDeploymentPatchConfigElPostStepEl {
    #[doc = "Set the field `linux_exec_step_config`.\n"]
    pub fn set_linux_exec_step_config(
        mut self,
        v: impl Into<
            BlockAssignable<OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.linux_exec_step_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.linux_exec_step_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `windows_exec_step_config`.\n"]
    pub fn set_windows_exec_step_config(
        mut self,
        v: impl Into<
            BlockAssignable<OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.windows_exec_step_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.windows_exec_step_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OsConfigPatchDeploymentPatchConfigElPostStepEl {
    type O = BlockAssignable<OsConfigPatchDeploymentPatchConfigElPostStepEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentPatchConfigElPostStepEl {}
impl BuildOsConfigPatchDeploymentPatchConfigElPostStepEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElPostStepEl {
        OsConfigPatchDeploymentPatchConfigElPostStepEl {
            linux_exec_step_config: core::default::Default::default(),
            windows_exec_step_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigPatchDeploymentPatchConfigElPostStepElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentPatchConfigElPostStepElRef {
    fn new(shared: StackShared, base: String) -> OsConfigPatchDeploymentPatchConfigElPostStepElRef {
        OsConfigPatchDeploymentPatchConfigElPostStepElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentPatchConfigElPostStepElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `linux_exec_step_config` after provisioning.\n"]
    pub fn linux_exec_step_config(
        &self,
    ) -> ListRef<OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.linux_exec_step_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `windows_exec_step_config` after provisioning.\n"]
    pub fn windows_exec_step_config(
        &self,
    ) -> ListRef<OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.windows_exec_step_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectEl {
    bucket: PrimField<String>,
    generation_number: PrimField<String>,
    object: PrimField<String>,
}
impl OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectEl {}
impl ToListMappable
    for OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectEl
{
    type O = BlockAssignable<
        OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectEl {
    #[doc = "Bucket of the Cloud Storage object."]
    pub bucket: PrimField<String>,
    #[doc = "Generation number of the Cloud Storage object. This is used to ensure that the ExecStep specified by this PatchJob does not change."]
    pub generation_number: PrimField<String>,
    #[doc = "Name of the Cloud Storage object."]
    pub object: PrimField<String>,
}
impl BuildOsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectEl {
    pub fn build(
        self,
    ) -> OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectEl {
        OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectEl {
            bucket: self.bucket,
            generation_number: self.generation_number,
            object: self.object,
        }
    }
}
pub struct OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectElRef {
        OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nBucket of the Cloud Storage object."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `generation_number` after provisioning.\nGeneration number of the Cloud Storage object. This is used to ensure that the ExecStep specified by this PatchJob does not change."]
    pub fn generation_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generation_number", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `object` after provisioning.\nName of the Cloud Storage object."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElDynamic {
    gcs_object: Option<
        DynamicBlock<OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectEl>,
    >,
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_success_codes: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interpreter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_object:
        Option<Vec<OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectEl>>,
    dynamic: OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElDynamic,
}
impl OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigEl {
    #[doc = "Set the field `allowed_success_codes`.\nDefaults to [0]. A list of possible return values that the execution can return to indicate a success."]
    pub fn set_allowed_success_codes(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.allowed_success_codes = Some(v.into());
        self
    }
    #[doc = "Set the field `interpreter`.\nThe script interpreter to use to run the script. If no interpreter is specified the script will\nbe executed directly, which will likely only succeed for scripts with shebang lines. Possible values: [\"SHELL\", \"POWERSHELL\"]"]
    pub fn set_interpreter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interpreter = Some(v.into());
        self
    }
    #[doc = "Set the field `local_path`.\nAn absolute path to the executable on the VM."]
    pub fn set_local_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_path = Some(v.into());
        self
    }
    #[doc = "Set the field `gcs_object`.\n"]
    pub fn set_gcs_object(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gcs_object = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs_object = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigEl {
    type O = BlockAssignable<OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigEl {}
impl BuildOsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigEl {
        OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigEl {
            allowed_success_codes: core::default::Default::default(),
            interpreter: core::default::Default::default(),
            local_path: core::default::Default::default(),
            gcs_object: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElRef {
        OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_success_codes` after provisioning.\nDefaults to [0]. A list of possible return values that the execution can return to indicate a success."]
    pub fn allowed_success_codes(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_success_codes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `interpreter` after provisioning.\nThe script interpreter to use to run the script. If no interpreter is specified the script will\nbe executed directly, which will likely only succeed for scripts with shebang lines. Possible values: [\"SHELL\", \"POWERSHELL\"]"]
    pub fn interpreter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interpreter", self.base))
    }
    #[doc = "Get a reference to the value of field `local_path` after provisioning.\nAn absolute path to the executable on the VM."]
    pub fn local_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }
    #[doc = "Get a reference to the value of field `gcs_object` after provisioning.\n"]
    pub fn gcs_object(
        &self,
    ) -> ListRef<OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.gcs_object", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectEl {
    bucket: PrimField<String>,
    generation_number: PrimField<String>,
    object: PrimField<String>,
}
impl OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectEl {}
impl ToListMappable
    for OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectEl
{
    type O = BlockAssignable<
        OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectEl {
    #[doc = "Bucket of the Cloud Storage object."]
    pub bucket: PrimField<String>,
    #[doc = "Generation number of the Cloud Storage object. This is used to ensure that the ExecStep specified by this PatchJob does not change."]
    pub generation_number: PrimField<String>,
    #[doc = "Name of the Cloud Storage object."]
    pub object: PrimField<String>,
}
impl BuildOsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectEl {
    pub fn build(
        self,
    ) -> OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectEl {
        OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectEl {
            bucket: self.bucket,
            generation_number: self.generation_number,
            object: self.object,
        }
    }
}
pub struct OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectElRef {
        OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nBucket of the Cloud Storage object."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `generation_number` after provisioning.\nGeneration number of the Cloud Storage object. This is used to ensure that the ExecStep specified by this PatchJob does not change."]
    pub fn generation_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generation_number", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `object` after provisioning.\nName of the Cloud Storage object."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElDynamic {
    gcs_object: Option<
        DynamicBlock<
            OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_success_codes: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interpreter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_object: Option<
        Vec<OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectEl>,
    >,
    dynamic: OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElDynamic,
}
impl OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigEl {
    #[doc = "Set the field `allowed_success_codes`.\nDefaults to [0]. A list of possible return values that the execution can return to indicate a success."]
    pub fn set_allowed_success_codes(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.allowed_success_codes = Some(v.into());
        self
    }
    #[doc = "Set the field `interpreter`.\nThe script interpreter to use to run the script. If no interpreter is specified the script will\nbe executed directly, which will likely only succeed for scripts with shebang lines. Possible values: [\"SHELL\", \"POWERSHELL\"]"]
    pub fn set_interpreter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interpreter = Some(v.into());
        self
    }
    #[doc = "Set the field `local_path`.\nAn absolute path to the executable on the VM."]
    pub fn set_local_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_path = Some(v.into());
        self
    }
    #[doc = "Set the field `gcs_object`.\n"]
    pub fn set_gcs_object(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gcs_object = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs_object = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigEl {
    type O = BlockAssignable<OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigEl {}
impl BuildOsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigEl {
        OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigEl {
            allowed_success_codes: core::default::Default::default(),
            interpreter: core::default::Default::default(),
            local_path: core::default::Default::default(),
            gcs_object: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElRef {
        OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_success_codes` after provisioning.\nDefaults to [0]. A list of possible return values that the execution can return to indicate a success."]
    pub fn allowed_success_codes(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_success_codes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `interpreter` after provisioning.\nThe script interpreter to use to run the script. If no interpreter is specified the script will\nbe executed directly, which will likely only succeed for scripts with shebang lines. Possible values: [\"SHELL\", \"POWERSHELL\"]"]
    pub fn interpreter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interpreter", self.base))
    }
    #[doc = "Get a reference to the value of field `local_path` after provisioning.\nAn absolute path to the executable on the VM."]
    pub fn local_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }
    #[doc = "Get a reference to the value of field `gcs_object` after provisioning.\n"]
    pub fn gcs_object(
        &self,
    ) -> ListRef<OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.gcs_object", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigPatchDeploymentPatchConfigElPreStepElDynamic {
    linux_exec_step_config:
        Option<DynamicBlock<OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigEl>>,
    windows_exec_step_config:
        Option<DynamicBlock<OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigEl>>,
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElPreStepEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    linux_exec_step_config:
        Option<Vec<OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    windows_exec_step_config:
        Option<Vec<OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigEl>>,
    dynamic: OsConfigPatchDeploymentPatchConfigElPreStepElDynamic,
}
impl OsConfigPatchDeploymentPatchConfigElPreStepEl {
    #[doc = "Set the field `linux_exec_step_config`.\n"]
    pub fn set_linux_exec_step_config(
        mut self,
        v: impl Into<
            BlockAssignable<OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.linux_exec_step_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.linux_exec_step_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `windows_exec_step_config`.\n"]
    pub fn set_windows_exec_step_config(
        mut self,
        v: impl Into<
            BlockAssignable<OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.windows_exec_step_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.windows_exec_step_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OsConfigPatchDeploymentPatchConfigElPreStepEl {
    type O = BlockAssignable<OsConfigPatchDeploymentPatchConfigElPreStepEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentPatchConfigElPreStepEl {}
impl BuildOsConfigPatchDeploymentPatchConfigElPreStepEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElPreStepEl {
        OsConfigPatchDeploymentPatchConfigElPreStepEl {
            linux_exec_step_config: core::default::Default::default(),
            windows_exec_step_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigPatchDeploymentPatchConfigElPreStepElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentPatchConfigElPreStepElRef {
    fn new(shared: StackShared, base: String) -> OsConfigPatchDeploymentPatchConfigElPreStepElRef {
        OsConfigPatchDeploymentPatchConfigElPreStepElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentPatchConfigElPreStepElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `linux_exec_step_config` after provisioning.\n"]
    pub fn linux_exec_step_config(
        &self,
    ) -> ListRef<OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.linux_exec_step_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `windows_exec_step_config` after provisioning.\n"]
    pub fn windows_exec_step_config(
        &self,
    ) -> ListRef<OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.windows_exec_step_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElWindowsUpdateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    classifications: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excludes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusive_patches: Option<ListField<PrimField<String>>>,
}
impl OsConfigPatchDeploymentPatchConfigElWindowsUpdateEl {
    #[doc = "Set the field `classifications`.\nOnly apply updates of these windows update classifications. If empty, all updates are applied. Possible values: [\"CRITICAL\", \"SECURITY\", \"DEFINITION\", \"DRIVER\", \"FEATURE_PACK\", \"SERVICE_PACK\", \"TOOL\", \"UPDATE_ROLLUP\", \"UPDATE\"]"]
    pub fn set_classifications(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.classifications = Some(v.into());
        self
    }
    #[doc = "Set the field `excludes`.\nList of KBs to exclude from update."]
    pub fn set_excludes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.excludes = Some(v.into());
        self
    }
    #[doc = "Set the field `exclusive_patches`.\nAn exclusive list of kbs to be updated. These are the only patches that will be updated.\nThis field must not be used with other patch configurations."]
    pub fn set_exclusive_patches(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclusive_patches = Some(v.into());
        self
    }
}
impl ToListMappable for OsConfigPatchDeploymentPatchConfigElWindowsUpdateEl {
    type O = BlockAssignable<OsConfigPatchDeploymentPatchConfigElWindowsUpdateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentPatchConfigElWindowsUpdateEl {}
impl BuildOsConfigPatchDeploymentPatchConfigElWindowsUpdateEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElWindowsUpdateEl {
        OsConfigPatchDeploymentPatchConfigElWindowsUpdateEl {
            classifications: core::default::Default::default(),
            excludes: core::default::Default::default(),
            exclusive_patches: core::default::Default::default(),
        }
    }
}
pub struct OsConfigPatchDeploymentPatchConfigElWindowsUpdateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentPatchConfigElWindowsUpdateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigPatchDeploymentPatchConfigElWindowsUpdateElRef {
        OsConfigPatchDeploymentPatchConfigElWindowsUpdateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentPatchConfigElWindowsUpdateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `classifications` after provisioning.\nOnly apply updates of these windows update classifications. If empty, all updates are applied. Possible values: [\"CRITICAL\", \"SECURITY\", \"DEFINITION\", \"DRIVER\", \"FEATURE_PACK\", \"SERVICE_PACK\", \"TOOL\", \"UPDATE_ROLLUP\", \"UPDATE\"]"]
    pub fn classifications(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.classifications", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `excludes` after provisioning.\nList of KBs to exclude from update."]
    pub fn excludes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }
    #[doc = "Get a reference to the value of field `exclusive_patches` after provisioning.\nAn exclusive list of kbs to be updated. These are the only patches that will be updated.\nThis field must not be used with other patch configurations."]
    pub fn exclusive_patches(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclusive_patches", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElYumEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    excludes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusive_packages: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimal: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security: Option<PrimField<bool>>,
}
impl OsConfigPatchDeploymentPatchConfigElYumEl {
    #[doc = "Set the field `excludes`.\nList of packages to exclude from update. These packages will be excluded."]
    pub fn set_excludes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.excludes = Some(v.into());
        self
    }
    #[doc = "Set the field `exclusive_packages`.\nAn exclusive list of packages to be updated. These are the only packages that will be updated.\nIf these packages are not installed, they will be ignored. This field cannot be specified with\nany other patch configuration fields."]
    pub fn set_exclusive_packages(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclusive_packages = Some(v.into());
        self
    }
    #[doc = "Set the field `minimal`.\nWill cause patch to run yum update-minimal instead."]
    pub fn set_minimal(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.minimal = Some(v.into());
        self
    }
    #[doc = "Set the field `security`.\nAdds the --security flag to yum update. Not supported on all platforms."]
    pub fn set_security(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.security = Some(v.into());
        self
    }
}
impl ToListMappable for OsConfigPatchDeploymentPatchConfigElYumEl {
    type O = BlockAssignable<OsConfigPatchDeploymentPatchConfigElYumEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentPatchConfigElYumEl {}
impl BuildOsConfigPatchDeploymentPatchConfigElYumEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElYumEl {
        OsConfigPatchDeploymentPatchConfigElYumEl {
            excludes: core::default::Default::default(),
            exclusive_packages: core::default::Default::default(),
            minimal: core::default::Default::default(),
            security: core::default::Default::default(),
        }
    }
}
pub struct OsConfigPatchDeploymentPatchConfigElYumElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentPatchConfigElYumElRef {
    fn new(shared: StackShared, base: String) -> OsConfigPatchDeploymentPatchConfigElYumElRef {
        OsConfigPatchDeploymentPatchConfigElYumElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentPatchConfigElYumElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `excludes` after provisioning.\nList of packages to exclude from update. These packages will be excluded."]
    pub fn excludes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }
    #[doc = "Get a reference to the value of field `exclusive_packages` after provisioning.\nAn exclusive list of packages to be updated. These are the only packages that will be updated.\nIf these packages are not installed, they will be ignored. This field cannot be specified with\nany other patch configuration fields."]
    pub fn exclusive_packages(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclusive_packages", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `minimal` after provisioning.\nWill cause patch to run yum update-minimal instead."]
    pub fn minimal(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimal", self.base))
    }
    #[doc = "Get a reference to the value of field `security` after provisioning.\nAdds the --security flag to yum update. Not supported on all platforms."]
    pub fn security(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.security", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElZypperEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    categories: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excludes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusive_patches: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    severities: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    with_optional: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    with_update: Option<PrimField<bool>>,
}
impl OsConfigPatchDeploymentPatchConfigElZypperEl {
    #[doc = "Set the field `categories`.\nInstall only patches with these categories. Common categories include security, recommended, and feature."]
    pub fn set_categories(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.categories = Some(v.into());
        self
    }
    #[doc = "Set the field `excludes`.\nList of packages to exclude from update."]
    pub fn set_excludes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.excludes = Some(v.into());
        self
    }
    #[doc = "Set the field `exclusive_patches`.\nAn exclusive list of patches to be updated. These are the only patches that will be installed using 'zypper patch patch:' command.\nThis field must not be used with any other patch configuration fields."]
    pub fn set_exclusive_patches(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclusive_patches = Some(v.into());
        self
    }
    #[doc = "Set the field `severities`.\nInstall only patches with these severities. Common severities include critical, important, moderate, and low."]
    pub fn set_severities(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.severities = Some(v.into());
        self
    }
    #[doc = "Set the field `with_optional`.\nAdds the --with-optional flag to zypper patch."]
    pub fn set_with_optional(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.with_optional = Some(v.into());
        self
    }
    #[doc = "Set the field `with_update`.\nAdds the --with-update flag, to zypper patch."]
    pub fn set_with_update(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.with_update = Some(v.into());
        self
    }
}
impl ToListMappable for OsConfigPatchDeploymentPatchConfigElZypperEl {
    type O = BlockAssignable<OsConfigPatchDeploymentPatchConfigElZypperEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentPatchConfigElZypperEl {}
impl BuildOsConfigPatchDeploymentPatchConfigElZypperEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElZypperEl {
        OsConfigPatchDeploymentPatchConfigElZypperEl {
            categories: core::default::Default::default(),
            excludes: core::default::Default::default(),
            exclusive_patches: core::default::Default::default(),
            severities: core::default::Default::default(),
            with_optional: core::default::Default::default(),
            with_update: core::default::Default::default(),
        }
    }
}
pub struct OsConfigPatchDeploymentPatchConfigElZypperElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentPatchConfigElZypperElRef {
    fn new(shared: StackShared, base: String) -> OsConfigPatchDeploymentPatchConfigElZypperElRef {
        OsConfigPatchDeploymentPatchConfigElZypperElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentPatchConfigElZypperElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `categories` after provisioning.\nInstall only patches with these categories. Common categories include security, recommended, and feature."]
    pub fn categories(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.categories", self.base))
    }
    #[doc = "Get a reference to the value of field `excludes` after provisioning.\nList of packages to exclude from update."]
    pub fn excludes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }
    #[doc = "Get a reference to the value of field `exclusive_patches` after provisioning.\nAn exclusive list of patches to be updated. These are the only patches that will be installed using 'zypper patch patch:' command.\nThis field must not be used with any other patch configuration fields."]
    pub fn exclusive_patches(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclusive_patches", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `severities` after provisioning.\nInstall only patches with these severities. Common severities include critical, important, moderate, and low."]
    pub fn severities(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.severities", self.base))
    }
    #[doc = "Get a reference to the value of field `with_optional` after provisioning.\nAdds the --with-optional flag to zypper patch."]
    pub fn with_optional(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.with_optional", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `with_update` after provisioning.\nAdds the --with-update flag, to zypper patch."]
    pub fn with_update(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.with_update", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigPatchDeploymentPatchConfigElDynamic {
    apt: Option<DynamicBlock<OsConfigPatchDeploymentPatchConfigElAptEl>>,
    goo: Option<DynamicBlock<OsConfigPatchDeploymentPatchConfigElGooEl>>,
    post_step: Option<DynamicBlock<OsConfigPatchDeploymentPatchConfigElPostStepEl>>,
    pre_step: Option<DynamicBlock<OsConfigPatchDeploymentPatchConfigElPreStepEl>>,
    windows_update: Option<DynamicBlock<OsConfigPatchDeploymentPatchConfigElWindowsUpdateEl>>,
    yum: Option<DynamicBlock<OsConfigPatchDeploymentPatchConfigElYumEl>>,
    zypper: Option<DynamicBlock<OsConfigPatchDeploymentPatchConfigElZypperEl>>,
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mig_instances_allowed: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reboot_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_unpatchable_vms: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apt: Option<Vec<OsConfigPatchDeploymentPatchConfigElAptEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    goo: Option<Vec<OsConfigPatchDeploymentPatchConfigElGooEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_step: Option<Vec<OsConfigPatchDeploymentPatchConfigElPostStepEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pre_step: Option<Vec<OsConfigPatchDeploymentPatchConfigElPreStepEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    windows_update: Option<Vec<OsConfigPatchDeploymentPatchConfigElWindowsUpdateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    yum: Option<Vec<OsConfigPatchDeploymentPatchConfigElYumEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zypper: Option<Vec<OsConfigPatchDeploymentPatchConfigElZypperEl>>,
    dynamic: OsConfigPatchDeploymentPatchConfigElDynamic,
}
impl OsConfigPatchDeploymentPatchConfigEl {
    #[doc = "Set the field `mig_instances_allowed`.\nAllows the patch job to run on Managed instance groups (MIGs)."]
    pub fn set_mig_instances_allowed(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.mig_instances_allowed = Some(v.into());
        self
    }
    #[doc = "Set the field `reboot_config`.\nPost-patch reboot settings. Possible values: [\"DEFAULT\", \"ALWAYS\", \"NEVER\"]"]
    pub fn set_reboot_config(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reboot_config = Some(v.into());
        self
    }
    #[doc = "Set the field `skip_unpatchable_vms`.\nEnables enhanced reporting for the patch job:\n\n1. The patch job skips instances that cannot be patched and reports them as 'SKIPPED'. An instance cannot be patched for two reasons:\n    * The instance runs Container-Optimized OS (COS), which cannot be patched.\n    * The instance is part of a managed instance group (MIG), and patching MIG instances is disabled in the patch job's configuration ('mig_instances_allowed' is false).\n2. The patch job is reported as 'SUCCEEDED' if it completes without errors, even if some instances are 'SKIPPED'.\n3. The patch job is reported as 'COMPLETED_WITH_INACTIVE_VMS' if it completes without errors, but does not patch instances that are 'INACTIVE'."]
    pub fn set_skip_unpatchable_vms(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.skip_unpatchable_vms = Some(v.into());
        self
    }
    #[doc = "Set the field `apt`.\n"]
    pub fn set_apt(
        mut self,
        v: impl Into<BlockAssignable<OsConfigPatchDeploymentPatchConfigElAptEl>>,
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
        v: impl Into<BlockAssignable<OsConfigPatchDeploymentPatchConfigElGooEl>>,
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
    #[doc = "Set the field `post_step`.\n"]
    pub fn set_post_step(
        mut self,
        v: impl Into<BlockAssignable<OsConfigPatchDeploymentPatchConfigElPostStepEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.post_step = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.post_step = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `pre_step`.\n"]
    pub fn set_pre_step(
        mut self,
        v: impl Into<BlockAssignable<OsConfigPatchDeploymentPatchConfigElPreStepEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pre_step = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pre_step = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `windows_update`.\n"]
    pub fn set_windows_update(
        mut self,
        v: impl Into<BlockAssignable<OsConfigPatchDeploymentPatchConfigElWindowsUpdateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.windows_update = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.windows_update = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `yum`.\n"]
    pub fn set_yum(
        mut self,
        v: impl Into<BlockAssignable<OsConfigPatchDeploymentPatchConfigElYumEl>>,
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
        v: impl Into<BlockAssignable<OsConfigPatchDeploymentPatchConfigElZypperEl>>,
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
impl ToListMappable for OsConfigPatchDeploymentPatchConfigEl {
    type O = BlockAssignable<OsConfigPatchDeploymentPatchConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentPatchConfigEl {}
impl BuildOsConfigPatchDeploymentPatchConfigEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigEl {
        OsConfigPatchDeploymentPatchConfigEl {
            mig_instances_allowed: core::default::Default::default(),
            reboot_config: core::default::Default::default(),
            skip_unpatchable_vms: core::default::Default::default(),
            apt: core::default::Default::default(),
            goo: core::default::Default::default(),
            post_step: core::default::Default::default(),
            pre_step: core::default::Default::default(),
            windows_update: core::default::Default::default(),
            yum: core::default::Default::default(),
            zypper: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigPatchDeploymentPatchConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentPatchConfigElRef {
    fn new(shared: StackShared, base: String) -> OsConfigPatchDeploymentPatchConfigElRef {
        OsConfigPatchDeploymentPatchConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentPatchConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mig_instances_allowed` after provisioning.\nAllows the patch job to run on Managed instance groups (MIGs)."]
    pub fn mig_instances_allowed(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mig_instances_allowed", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `reboot_config` after provisioning.\nPost-patch reboot settings. Possible values: [\"DEFAULT\", \"ALWAYS\", \"NEVER\"]"]
    pub fn reboot_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reboot_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `skip_unpatchable_vms` after provisioning.\nEnables enhanced reporting for the patch job:\n\n1. The patch job skips instances that cannot be patched and reports them as 'SKIPPED'. An instance cannot be patched for two reasons:\n    * The instance runs Container-Optimized OS (COS), which cannot be patched.\n    * The instance is part of a managed instance group (MIG), and patching MIG instances is disabled in the patch job's configuration ('mig_instances_allowed' is false).\n2. The patch job is reported as 'SUCCEEDED' if it completes without errors, even if some instances are 'SKIPPED'.\n3. The patch job is reported as 'COMPLETED_WITH_INACTIVE_VMS' if it completes without errors, but does not patch instances that are 'INACTIVE'."]
    pub fn skip_unpatchable_vms(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_unpatchable_vms", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `apt` after provisioning.\n"]
    pub fn apt(&self) -> ListRef<OsConfigPatchDeploymentPatchConfigElAptElRef> {
        ListRef::new(self.shared().clone(), format!("{}.apt", self.base))
    }
    #[doc = "Get a reference to the value of field `goo` after provisioning.\n"]
    pub fn goo(&self) -> ListRef<OsConfigPatchDeploymentPatchConfigElGooElRef> {
        ListRef::new(self.shared().clone(), format!("{}.goo", self.base))
    }
    #[doc = "Get a reference to the value of field `post_step` after provisioning.\n"]
    pub fn post_step(&self) -> ListRef<OsConfigPatchDeploymentPatchConfigElPostStepElRef> {
        ListRef::new(self.shared().clone(), format!("{}.post_step", self.base))
    }
    #[doc = "Get a reference to the value of field `pre_step` after provisioning.\n"]
    pub fn pre_step(&self) -> ListRef<OsConfigPatchDeploymentPatchConfigElPreStepElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pre_step", self.base))
    }
    #[doc = "Get a reference to the value of field `windows_update` after provisioning.\n"]
    pub fn windows_update(
        &self,
    ) -> ListRef<OsConfigPatchDeploymentPatchConfigElWindowsUpdateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.windows_update", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `yum` after provisioning.\n"]
    pub fn yum(&self) -> ListRef<OsConfigPatchDeploymentPatchConfigElYumElRef> {
        ListRef::new(self.shared().clone(), format!("{}.yum", self.base))
    }
    #[doc = "Get a reference to the value of field `zypper` after provisioning.\n"]
    pub fn zypper(&self) -> ListRef<OsConfigPatchDeploymentPatchConfigElZypperElRef> {
        ListRef::new(self.shared().clone(), format!("{}.zypper", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthEl {
    day_of_week: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    day_offset: Option<PrimField<f64>>,
    week_ordinal: PrimField<f64>,
}
impl OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthEl {
    #[doc = "Set the field `day_offset`.\nRepresents the number of days before or after the given week day of month that the patch deployment is scheduled for."]
    pub fn set_day_offset(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.day_offset = Some(v.into());
        self
    }
}
impl ToListMappable for OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthEl {
    type O = BlockAssignable<OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthEl {
    #[doc = "A day of the week. Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub day_of_week: PrimField<String>,
    #[doc = "Week number in a month. 1-4 indicates the 1st to 4th week of the month. -1 indicates the last week of the month."]
    pub week_ordinal: PrimField<f64>,
}
impl BuildOsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthEl {
    pub fn build(self) -> OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthEl {
        OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthEl {
            day_of_week: self.day_of_week,
            day_offset: core::default::Default::default(),
            week_ordinal: self.week_ordinal,
        }
    }
}
pub struct OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthElRef {
        OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day_of_week` after provisioning.\nA day of the week. Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn day_of_week(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }
    #[doc = "Get a reference to the value of field `day_offset` after provisioning.\nRepresents the number of days before or after the given week day of month that the patch deployment is scheduled for."]
    pub fn day_offset(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.day_offset", self.base))
    }
    #[doc = "Get a reference to the value of field `week_ordinal` after provisioning.\nWeek number in a month. 1-4 indicates the 1st to 4th week of the month. -1 indicates the last week of the month."]
    pub fn week_ordinal(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.week_ordinal", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigPatchDeploymentRecurringScheduleElMonthlyElDynamic {
    week_day_of_month:
        Option<DynamicBlock<OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthEl>>,
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentRecurringScheduleElMonthlyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    month_day: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    week_day_of_month:
        Option<Vec<OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthEl>>,
    dynamic: OsConfigPatchDeploymentRecurringScheduleElMonthlyElDynamic,
}
impl OsConfigPatchDeploymentRecurringScheduleElMonthlyEl {
    #[doc = "Set the field `month_day`.\nOne day of the month. 1-31 indicates the 1st to the 31st day. -1 indicates the last day of the month.\nMonths without the target day will be skipped. For example, a schedule to run \"every month on the 31st\"\nwill not run in February, April, June, etc."]
    pub fn set_month_day(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.month_day = Some(v.into());
        self
    }
    #[doc = "Set the field `week_day_of_month`.\n"]
    pub fn set_week_day_of_month(
        mut self,
        v: impl Into<
            BlockAssignable<OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.week_day_of_month = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.week_day_of_month = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OsConfigPatchDeploymentRecurringScheduleElMonthlyEl {
    type O = BlockAssignable<OsConfigPatchDeploymentRecurringScheduleElMonthlyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentRecurringScheduleElMonthlyEl {}
impl BuildOsConfigPatchDeploymentRecurringScheduleElMonthlyEl {
    pub fn build(self) -> OsConfigPatchDeploymentRecurringScheduleElMonthlyEl {
        OsConfigPatchDeploymentRecurringScheduleElMonthlyEl {
            month_day: core::default::Default::default(),
            week_day_of_month: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigPatchDeploymentRecurringScheduleElMonthlyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentRecurringScheduleElMonthlyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigPatchDeploymentRecurringScheduleElMonthlyElRef {
        OsConfigPatchDeploymentRecurringScheduleElMonthlyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentRecurringScheduleElMonthlyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `month_day` after provisioning.\nOne day of the month. 1-31 indicates the 1st to the 31st day. -1 indicates the last day of the month.\nMonths without the target day will be skipped. For example, a schedule to run \"every month on the 31st\"\nwill not run in February, April, June, etc."]
    pub fn month_day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.month_day", self.base))
    }
    #[doc = "Get a reference to the value of field `week_day_of_month` after provisioning.\n"]
    pub fn week_day_of_month(
        &self,
    ) -> ListRef<OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.week_day_of_month", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentRecurringScheduleElTimeOfDayEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl OsConfigPatchDeploymentRecurringScheduleElTimeOfDayEl {
    #[doc = "Set the field `hours`.\nHours of day in 24 hour format. Should be from 0 to 23.\nAn API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn set_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hours = Some(v.into());
        self
    }
    #[doc = "Set the field `minutes`.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn set_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minutes = Some(v.into());
        self
    }
    #[doc = "Set the field `nanos`.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\nSeconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds."]
    pub fn set_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable for OsConfigPatchDeploymentRecurringScheduleElTimeOfDayEl {
    type O = BlockAssignable<OsConfigPatchDeploymentRecurringScheduleElTimeOfDayEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentRecurringScheduleElTimeOfDayEl {}
impl BuildOsConfigPatchDeploymentRecurringScheduleElTimeOfDayEl {
    pub fn build(self) -> OsConfigPatchDeploymentRecurringScheduleElTimeOfDayEl {
        OsConfigPatchDeploymentRecurringScheduleElTimeOfDayEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct OsConfigPatchDeploymentRecurringScheduleElTimeOfDayElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentRecurringScheduleElTimeOfDayElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigPatchDeploymentRecurringScheduleElTimeOfDayElRef {
        OsConfigPatchDeploymentRecurringScheduleElTimeOfDayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentRecurringScheduleElTimeOfDayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours` after provisioning.\nHours of day in 24 hour format. Should be from 0 to 23.\nAn API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
    #[doc = "Get a reference to the value of field `minutes` after provisioning.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSeconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds."]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentRecurringScheduleElTimeZoneEl {
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl OsConfigPatchDeploymentRecurringScheduleElTimeZoneEl {
    #[doc = "Set the field `version`.\nIANA Time Zone Database version number, e.g. \"2019a\"."]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for OsConfigPatchDeploymentRecurringScheduleElTimeZoneEl {
    type O = BlockAssignable<OsConfigPatchDeploymentRecurringScheduleElTimeZoneEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentRecurringScheduleElTimeZoneEl {
    #[doc = "IANA Time Zone Database time zone, e.g. \"America/New_York\"."]
    pub id: PrimField<String>,
}
impl BuildOsConfigPatchDeploymentRecurringScheduleElTimeZoneEl {
    pub fn build(self) -> OsConfigPatchDeploymentRecurringScheduleElTimeZoneEl {
        OsConfigPatchDeploymentRecurringScheduleElTimeZoneEl {
            id: self.id,
            version: core::default::Default::default(),
        }
    }
}
pub struct OsConfigPatchDeploymentRecurringScheduleElTimeZoneElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentRecurringScheduleElTimeZoneElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigPatchDeploymentRecurringScheduleElTimeZoneElRef {
        OsConfigPatchDeploymentRecurringScheduleElTimeZoneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentRecurringScheduleElTimeZoneElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nIANA Time Zone Database time zone, e.g. \"America/New_York\"."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nIANA Time Zone Database version number, e.g. \"2019a\"."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentRecurringScheduleElWeeklyEl {
    day_of_week: PrimField<String>,
}
impl OsConfigPatchDeploymentRecurringScheduleElWeeklyEl {}
impl ToListMappable for OsConfigPatchDeploymentRecurringScheduleElWeeklyEl {
    type O = BlockAssignable<OsConfigPatchDeploymentRecurringScheduleElWeeklyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentRecurringScheduleElWeeklyEl {
    #[doc = "IANA Time Zone Database time zone, e.g. \"America/New_York\". Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub day_of_week: PrimField<String>,
}
impl BuildOsConfigPatchDeploymentRecurringScheduleElWeeklyEl {
    pub fn build(self) -> OsConfigPatchDeploymentRecurringScheduleElWeeklyEl {
        OsConfigPatchDeploymentRecurringScheduleElWeeklyEl {
            day_of_week: self.day_of_week,
        }
    }
}
pub struct OsConfigPatchDeploymentRecurringScheduleElWeeklyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentRecurringScheduleElWeeklyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigPatchDeploymentRecurringScheduleElWeeklyElRef {
        OsConfigPatchDeploymentRecurringScheduleElWeeklyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentRecurringScheduleElWeeklyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day_of_week` after provisioning.\nIANA Time Zone Database time zone, e.g. \"America/New_York\". Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn day_of_week(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigPatchDeploymentRecurringScheduleElDynamic {
    monthly: Option<DynamicBlock<OsConfigPatchDeploymentRecurringScheduleElMonthlyEl>>,
    time_of_day: Option<DynamicBlock<OsConfigPatchDeploymentRecurringScheduleElTimeOfDayEl>>,
    time_zone: Option<DynamicBlock<OsConfigPatchDeploymentRecurringScheduleElTimeZoneEl>>,
    weekly: Option<DynamicBlock<OsConfigPatchDeploymentRecurringScheduleElWeeklyEl>>,
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentRecurringScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monthly: Option<Vec<OsConfigPatchDeploymentRecurringScheduleElMonthlyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_of_day: Option<Vec<OsConfigPatchDeploymentRecurringScheduleElTimeOfDayEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<Vec<OsConfigPatchDeploymentRecurringScheduleElTimeZoneEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly: Option<Vec<OsConfigPatchDeploymentRecurringScheduleElWeeklyEl>>,
    dynamic: OsConfigPatchDeploymentRecurringScheduleElDynamic,
}
impl OsConfigPatchDeploymentRecurringScheduleEl {
    #[doc = "Set the field `end_time`.\nThe end time at which a recurring patch deployment schedule is no longer active.\nA timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\nThe time that the recurring schedule becomes effective. Defaults to createTime of the patch deployment.\nA timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
    #[doc = "Set the field `monthly`.\n"]
    pub fn set_monthly(
        mut self,
        v: impl Into<BlockAssignable<OsConfigPatchDeploymentRecurringScheduleElMonthlyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.monthly = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.monthly = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `time_of_day`.\n"]
    pub fn set_time_of_day(
        mut self,
        v: impl Into<BlockAssignable<OsConfigPatchDeploymentRecurringScheduleElTimeOfDayEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.time_of_day = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.time_of_day = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `time_zone`.\n"]
    pub fn set_time_zone(
        mut self,
        v: impl Into<BlockAssignable<OsConfigPatchDeploymentRecurringScheduleElTimeZoneEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.time_zone = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.time_zone = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `weekly`.\n"]
    pub fn set_weekly(
        mut self,
        v: impl Into<BlockAssignable<OsConfigPatchDeploymentRecurringScheduleElWeeklyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.weekly = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.weekly = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OsConfigPatchDeploymentRecurringScheduleEl {
    type O = BlockAssignable<OsConfigPatchDeploymentRecurringScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentRecurringScheduleEl {}
impl BuildOsConfigPatchDeploymentRecurringScheduleEl {
    pub fn build(self) -> OsConfigPatchDeploymentRecurringScheduleEl {
        OsConfigPatchDeploymentRecurringScheduleEl {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
            monthly: core::default::Default::default(),
            time_of_day: core::default::Default::default(),
            time_zone: core::default::Default::default(),
            weekly: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigPatchDeploymentRecurringScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentRecurringScheduleElRef {
    fn new(shared: StackShared, base: String) -> OsConfigPatchDeploymentRecurringScheduleElRef {
        OsConfigPatchDeploymentRecurringScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentRecurringScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\nThe end time at which a recurring patch deployment schedule is no longer active.\nA timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `last_execute_time` after provisioning.\nThe time the last patch job ran successfully.\nA timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn last_execute_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_execute_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `next_execute_time` after provisioning.\nThe time the next patch job is scheduled to run.\nA timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn next_execute_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_execute_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\nThe time that the recurring schedule becomes effective. Defaults to createTime of the patch deployment.\nA timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
    #[doc = "Get a reference to the value of field `monthly` after provisioning.\n"]
    pub fn monthly(&self) -> ListRef<OsConfigPatchDeploymentRecurringScheduleElMonthlyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.monthly", self.base))
    }
    #[doc = "Get a reference to the value of field `time_of_day` after provisioning.\n"]
    pub fn time_of_day(&self) -> ListRef<OsConfigPatchDeploymentRecurringScheduleElTimeOfDayElRef> {
        ListRef::new(self.shared().clone(), format!("{}.time_of_day", self.base))
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(&self) -> ListRef<OsConfigPatchDeploymentRecurringScheduleElTimeZoneElRef> {
        ListRef::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
    #[doc = "Get a reference to the value of field `weekly` after provisioning.\n"]
    pub fn weekly(&self) -> ListRef<OsConfigPatchDeploymentRecurringScheduleElWeeklyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.weekly", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentRolloutElDisruptionBudgetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percentage: Option<PrimField<f64>>,
}
impl OsConfigPatchDeploymentRolloutElDisruptionBudgetEl {
    #[doc = "Set the field `fixed`.\nSpecifies a fixed value."]
    pub fn set_fixed(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.fixed = Some(v.into());
        self
    }
    #[doc = "Set the field `percentage`.\nSpecifies the relative value defined as a percentage, which will be multiplied by a reference value."]
    pub fn set_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percentage = Some(v.into());
        self
    }
}
impl ToListMappable for OsConfigPatchDeploymentRolloutElDisruptionBudgetEl {
    type O = BlockAssignable<OsConfigPatchDeploymentRolloutElDisruptionBudgetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentRolloutElDisruptionBudgetEl {}
impl BuildOsConfigPatchDeploymentRolloutElDisruptionBudgetEl {
    pub fn build(self) -> OsConfigPatchDeploymentRolloutElDisruptionBudgetEl {
        OsConfigPatchDeploymentRolloutElDisruptionBudgetEl {
            fixed: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}
pub struct OsConfigPatchDeploymentRolloutElDisruptionBudgetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentRolloutElDisruptionBudgetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigPatchDeploymentRolloutElDisruptionBudgetElRef {
        OsConfigPatchDeploymentRolloutElDisruptionBudgetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentRolloutElDisruptionBudgetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `fixed` after provisioning.\nSpecifies a fixed value."]
    pub fn fixed(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.fixed", self.base))
    }
    #[doc = "Get a reference to the value of field `percentage` after provisioning.\nSpecifies the relative value defined as a percentage, which will be multiplied by a reference value."]
    pub fn percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigPatchDeploymentRolloutElDynamic {
    disruption_budget: Option<DynamicBlock<OsConfigPatchDeploymentRolloutElDisruptionBudgetEl>>,
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentRolloutEl {
    mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disruption_budget: Option<Vec<OsConfigPatchDeploymentRolloutElDisruptionBudgetEl>>,
    dynamic: OsConfigPatchDeploymentRolloutElDynamic,
}
impl OsConfigPatchDeploymentRolloutEl {
    #[doc = "Set the field `disruption_budget`.\n"]
    pub fn set_disruption_budget(
        mut self,
        v: impl Into<BlockAssignable<OsConfigPatchDeploymentRolloutElDisruptionBudgetEl>>,
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
impl ToListMappable for OsConfigPatchDeploymentRolloutEl {
    type O = BlockAssignable<OsConfigPatchDeploymentRolloutEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentRolloutEl {
    #[doc = "Mode of the patch rollout. Possible values: [\"ZONE_BY_ZONE\", \"CONCURRENT_ZONES\"]"]
    pub mode: PrimField<String>,
}
impl BuildOsConfigPatchDeploymentRolloutEl {
    pub fn build(self) -> OsConfigPatchDeploymentRolloutEl {
        OsConfigPatchDeploymentRolloutEl {
            mode: self.mode,
            disruption_budget: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigPatchDeploymentRolloutElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentRolloutElRef {
    fn new(shared: StackShared, base: String) -> OsConfigPatchDeploymentRolloutElRef {
        OsConfigPatchDeploymentRolloutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentRolloutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nMode of the patch rollout. Possible values: [\"ZONE_BY_ZONE\", \"CONCURRENT_ZONES\"]"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `disruption_budget` after provisioning.\n"]
    pub fn disruption_budget(
        &self,
    ) -> ListRef<OsConfigPatchDeploymentRolloutElDisruptionBudgetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disruption_budget", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OsConfigPatchDeploymentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl OsConfigPatchDeploymentTimeoutsEl {
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
impl ToListMappable for OsConfigPatchDeploymentTimeoutsEl {
    type O = BlockAssignable<OsConfigPatchDeploymentTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigPatchDeploymentTimeoutsEl {}
impl BuildOsConfigPatchDeploymentTimeoutsEl {
    pub fn build(self) -> OsConfigPatchDeploymentTimeoutsEl {
        OsConfigPatchDeploymentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct OsConfigPatchDeploymentTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigPatchDeploymentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> OsConfigPatchDeploymentTimeoutsElRef {
        OsConfigPatchDeploymentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigPatchDeploymentTimeoutsElRef {
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
struct OsConfigPatchDeploymentDynamic {
    instance_filter: Option<DynamicBlock<OsConfigPatchDeploymentInstanceFilterEl>>,
    one_time_schedule: Option<DynamicBlock<OsConfigPatchDeploymentOneTimeScheduleEl>>,
    patch_config: Option<DynamicBlock<OsConfigPatchDeploymentPatchConfigEl>>,
    recurring_schedule: Option<DynamicBlock<OsConfigPatchDeploymentRecurringScheduleEl>>,
    rollout: Option<DynamicBlock<OsConfigPatchDeploymentRolloutEl>>,
}
