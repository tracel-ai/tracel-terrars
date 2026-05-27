use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct OsConfigV2PolicyOrchestratorData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    action: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    policy_orchestrator_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orchestrated_resource: Option<Vec<OsConfigV2PolicyOrchestratorOrchestratedResourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orchestration_scope: Option<Vec<OsConfigV2PolicyOrchestratorOrchestrationScopeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<OsConfigV2PolicyOrchestratorTimeoutsEl>,
    dynamic: OsConfigV2PolicyOrchestratorDynamic,
}
struct OsConfigV2PolicyOrchestrator_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OsConfigV2PolicyOrchestratorData>,
}
#[derive(Clone)]
pub struct OsConfigV2PolicyOrchestrator(Rc<OsConfigV2PolicyOrchestrator_>);
impl OsConfigV2PolicyOrchestrator {
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
    #[doc = "Set the field `description`.\nOptional. Freeform text describing the purpose of the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nOptional. Labels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\nOptional. State of the orchestrator. Can be updated to change orchestrator behaviour.\nAllowed values:\n- 'ACTIVE' - orchestrator is actively looking for actions to be taken.\n- 'STOPPED' - orchestrator won't make any changes.\n\nNote: There might be more states added in the future. We use string here\ninstead of an enum, to avoid the need of propagating new states to all the\nclient code."]
    pub fn set_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().state = Some(v.into());
        self
    }
    #[doc = "Set the field `orchestrated_resource`.\n"]
    pub fn set_orchestrated_resource(
        self,
        v: impl Into<BlockAssignable<OsConfigV2PolicyOrchestratorOrchestratedResourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().orchestrated_resource = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.orchestrated_resource = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `orchestration_scope`.\n"]
    pub fn set_orchestration_scope(
        self,
        v: impl Into<BlockAssignable<OsConfigV2PolicyOrchestratorOrchestrationScopeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().orchestration_scope = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.orchestration_scope = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<OsConfigV2PolicyOrchestratorTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\nRequired. Action to be done by the orchestrator in\n'projects/{project_id}/zones/{zone_id}' locations defined by the\n'orchestration_scope'. Allowed values:\n- 'UPSERT' - Orchestrator will create or update target resources.\n- 'DELETE' - Orchestrator will delete target resources, if they exist"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Timestamp when the policy orchestrator resource was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. Freeform text describing the purpose of the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nImmutable. Identifier. In form of\n* 'organizations/{organization_id}/locations/global/policyOrchestrators/{orchestrator_id}'\n* 'folders/{folder_id}/locations/global/policyOrchestrators/{orchestrator_id}'\n* 'projects/{project_id_or_number}/locations/global/policyOrchestrators/{orchestrator_id}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `orchestration_state` after provisioning.\nDescribes the state of the orchestration process."]
    pub fn orchestration_state(
        &self,
    ) -> ListRef<OsConfigV2PolicyOrchestratorOrchestrationStateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.orchestration_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_orchestrator_id` after provisioning.\nRequired. The logical identifier of the policy orchestrator, with the following\nrestrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the parent."]
    pub fn policy_orchestrator_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_orchestrator_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. Set to true, if the there are ongoing changes being applied by the\norchestrator."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOptional. State of the orchestrator. Can be updated to change orchestrator behaviour.\nAllowed values:\n- 'ACTIVE' - orchestrator is actively looking for actions to be taken.\n- 'STOPPED' - orchestrator won't make any changes.\n\nNote: There might be more states added in the future. We use string here\ninstead of an enum, to avoid the need of propagating new states to all the\nclient code."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Timestamp when the policy orchestrator resource was last modified."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `orchestrated_resource` after provisioning.\n"]
    pub fn orchestrated_resource(
        &self,
    ) -> ListRef<OsConfigV2PolicyOrchestratorOrchestratedResourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.orchestrated_resource", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `orchestration_scope` after provisioning.\n"]
    pub fn orchestration_scope(
        &self,
    ) -> ListRef<OsConfigV2PolicyOrchestratorOrchestrationScopeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.orchestration_scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OsConfigV2PolicyOrchestratorTimeoutsElRef {
        OsConfigV2PolicyOrchestratorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for OsConfigV2PolicyOrchestrator {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for OsConfigV2PolicyOrchestrator {}
impl ToListMappable for OsConfigV2PolicyOrchestrator {
    type O = ListRef<OsConfigV2PolicyOrchestratorRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for OsConfigV2PolicyOrchestrator_ {
    fn extract_resource_type(&self) -> String {
        "google_os_config_v2_policy_orchestrator".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildOsConfigV2PolicyOrchestrator {
    pub tf_id: String,
    #[doc = "Required. Action to be done by the orchestrator in\n'projects/{project_id}/zones/{zone_id}' locations defined by the\n'orchestration_scope'. Allowed values:\n- 'UPSERT' - Orchestrator will create or update target resources.\n- 'DELETE' - Orchestrator will delete target resources, if they exist"]
    pub action: PrimField<String>,
    #[doc = "Required. The logical identifier of the policy orchestrator, with the following\nrestrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the parent."]
    pub policy_orchestrator_id: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestrator {
    pub fn build(self, stack: &mut Stack) -> OsConfigV2PolicyOrchestrator {
        let out = OsConfigV2PolicyOrchestrator(Rc::new(OsConfigV2PolicyOrchestrator_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OsConfigV2PolicyOrchestratorData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                action: self.action,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                policy_orchestrator_id: self.policy_orchestrator_id,
                project: core::default::Default::default(),
                state: core::default::Default::default(),
                orchestrated_resource: core::default::Default::default(),
                orchestration_scope: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct OsConfigV2PolicyOrchestratorRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl OsConfigV2PolicyOrchestratorRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\nRequired. Action to be done by the orchestrator in\n'projects/{project_id}/zones/{zone_id}' locations defined by the\n'orchestration_scope'. Allowed values:\n- 'UPSERT' - Orchestrator will create or update target resources.\n- 'DELETE' - Orchestrator will delete target resources, if they exist"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Timestamp when the policy orchestrator resource was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. Freeform text describing the purpose of the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nImmutable. Identifier. In form of\n* 'organizations/{organization_id}/locations/global/policyOrchestrators/{orchestrator_id}'\n* 'folders/{folder_id}/locations/global/policyOrchestrators/{orchestrator_id}'\n* 'projects/{project_id_or_number}/locations/global/policyOrchestrators/{orchestrator_id}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `orchestration_state` after provisioning.\nDescribes the state of the orchestration process."]
    pub fn orchestration_state(
        &self,
    ) -> ListRef<OsConfigV2PolicyOrchestratorOrchestrationStateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.orchestration_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_orchestrator_id` after provisioning.\nRequired. The logical identifier of the policy orchestrator, with the following\nrestrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the parent."]
    pub fn policy_orchestrator_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_orchestrator_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. Set to true, if the there are ongoing changes being applied by the\norchestrator."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOptional. State of the orchestrator. Can be updated to change orchestrator behaviour.\nAllowed values:\n- 'ACTIVE' - orchestrator is actively looking for actions to be taken.\n- 'STOPPED' - orchestrator won't make any changes.\n\nNote: There might be more states added in the future. We use string here\ninstead of an enum, to avoid the need of propagating new states to all the\nclient code."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Timestamp when the policy orchestrator resource was last modified."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `orchestrated_resource` after provisioning.\n"]
    pub fn orchestrated_resource(
        &self,
    ) -> ListRef<OsConfigV2PolicyOrchestratorOrchestratedResourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.orchestrated_resource", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `orchestration_scope` after provisioning.\n"]
    pub fn orchestration_scope(
        &self,
    ) -> ListRef<OsConfigV2PolicyOrchestratorOrchestrationScopeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.orchestration_scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OsConfigV2PolicyOrchestratorTimeoutsElRef {
        OsConfigV2PolicyOrchestratorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorElDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    type_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorElDetailsEl {
    #[doc = "Set the field `type_url`.\n"]
    pub fn set_type_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_url = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorElDetailsEl
{
    type O = BlockAssignable<
        OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorElDetailsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorElDetailsEl
{}
impl BuildOsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorElDetailsEl {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorElDetailsEl
    {
        OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorElDetailsEl {
            type_url: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorElDetailsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorElDetailsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorElDetailsElRef
    {
        OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorElDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorElDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_url` after provisioning.\n"]
    pub fn type_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type_url", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<
        ListField<
            OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorElDetailsEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        mut self,
        v : impl Into < ListField < OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorElDetailsEl > >,
    ) -> Self {
        self.details = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable
    for OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorEl
{
    type O = BlockAssignable<
        OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorEl {}
impl BuildOsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorEl {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorEl {
        OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorEl {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorElRef {
        OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(
        &self,
    ) -> ListRef<
        OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorElDetailsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.details", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<
        ListField<OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    failed_actions: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finish_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performed_actions: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    progress: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rollout_resource: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateEl {
    #[doc = "Set the field `error`.\n"]
    pub fn set_error(
        mut self,
        v: impl Into<
            ListField<
                OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorEl,
            >,
        >,
    ) -> Self {
        self.error = Some(v.into());
        self
    }
    #[doc = "Set the field `failed_actions`.\n"]
    pub fn set_failed_actions(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.failed_actions = Some(v.into());
        self
    }
    #[doc = "Set the field `finish_time`.\n"]
    pub fn set_finish_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.finish_time = Some(v.into());
        self
    }
    #[doc = "Set the field `performed_actions`.\n"]
    pub fn set_performed_actions(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.performed_actions = Some(v.into());
        self
    }
    #[doc = "Set the field `progress`.\n"]
    pub fn set_progress(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.progress = Some(v.into());
        self
    }
    #[doc = "Set the field `rollout_resource`.\n"]
    pub fn set_rollout_resource(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rollout_resource = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateEl {
    type O =
        BlockAssignable<OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateEl {}
impl BuildOsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateEl {
    pub fn build(self) -> OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateEl {
        OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateEl {
            error: core::default::Default::default(),
            failed_actions: core::default::Default::default(),
            finish_time: core::default::Default::default(),
            performed_actions: core::default::Default::default(),
            progress: core::default::Default::default(),
            rollout_resource: core::default::Default::default(),
            start_time: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElRef {
        OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(
        &self,
    ) -> ListRef<OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElErrorElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.error", self.base))
    }
    #[doc = "Get a reference to the value of field `failed_actions` after provisioning.\n"]
    pub fn failed_actions(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failed_actions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `finish_time` after provisioning.\n"]
    pub fn finish_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.finish_time", self.base))
    }
    #[doc = "Get a reference to the value of field `performed_actions` after provisioning.\n"]
    pub fn performed_actions(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.performed_actions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `progress` after provisioning.\n"]
    pub fn progress(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.progress", self.base))
    }
    #[doc = "Get a reference to the value of field `rollout_resource` after provisioning.\n"]
    pub fn rollout_resource(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rollout_resource", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorElDetailsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    type_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorElDetailsEl {
    #[doc = "Set the field `type_url`.\n"]
    pub fn set_type_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_url = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorElDetailsEl
{
    type O = BlockAssignable<
        OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorElDetailsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorElDetailsEl
{}
impl BuildOsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorElDetailsEl {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorElDetailsEl
    {
        OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorElDetailsEl {
            type_url: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorElDetailsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorElDetailsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorElDetailsElRef
    {
        OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorElDetailsElRef { shared : shared , base : base . to_string () , }
    }
}
impl OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorElDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_url` after provisioning.\n"]
    pub fn type_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type_url", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorEl { # [serde (skip_serializing_if = "Option::is_none")] code : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] details : Option < ListField < OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorElDetailsEl > > , # [serde (skip_serializing_if = "Option::is_none")] message : Option < PrimField < String > > , }
impl OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        mut self,
        v : impl Into < ListField < OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorElDetailsEl > >,
    ) -> Self {
        self.details = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable
    for OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorEl
{
    type O = BlockAssignable<
        OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorEl {}
impl BuildOsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorEl {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorEl {
        OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorEl {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorElRef {
        OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(
        &self,
    ) -> ListRef<
        OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorElDetailsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.details", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<
        ListField<OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    failed_actions: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finish_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performed_actions: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    progress: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rollout_resource: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateEl {
    #[doc = "Set the field `error`.\n"]
    pub fn set_error(
        mut self,
        v: impl Into<
            ListField<
                OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorEl,
            >,
        >,
    ) -> Self {
        self.error = Some(v.into());
        self
    }
    #[doc = "Set the field `failed_actions`.\n"]
    pub fn set_failed_actions(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.failed_actions = Some(v.into());
        self
    }
    #[doc = "Set the field `finish_time`.\n"]
    pub fn set_finish_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.finish_time = Some(v.into());
        self
    }
    #[doc = "Set the field `performed_actions`.\n"]
    pub fn set_performed_actions(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.performed_actions = Some(v.into());
        self
    }
    #[doc = "Set the field `progress`.\n"]
    pub fn set_progress(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.progress = Some(v.into());
        self
    }
    #[doc = "Set the field `rollout_resource`.\n"]
    pub fn set_rollout_resource(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rollout_resource = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateEl {
    type O =
        BlockAssignable<OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateEl {}
impl BuildOsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateEl {
    pub fn build(self) -> OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateEl {
        OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateEl {
            error: core::default::Default::default(),
            failed_actions: core::default::Default::default(),
            finish_time: core::default::Default::default(),
            performed_actions: core::default::Default::default(),
            progress: core::default::Default::default(),
            rollout_resource: core::default::Default::default(),
            start_time: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElRef {
        OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(
        &self,
    ) -> ListRef<OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElErrorElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.error", self.base))
    }
    #[doc = "Get a reference to the value of field `failed_actions` after provisioning.\n"]
    pub fn failed_actions(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failed_actions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `finish_time` after provisioning.\n"]
    pub fn finish_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.finish_time", self.base))
    }
    #[doc = "Get a reference to the value of field `performed_actions` after provisioning.\n"]
    pub fn performed_actions(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.performed_actions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `progress` after provisioning.\n"]
    pub fn progress(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.progress", self.base))
    }
    #[doc = "Get a reference to the value of field `rollout_resource` after provisioning.\n"]
    pub fn rollout_resource(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rollout_resource", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestrationStateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    current_iteration_state:
        Option<ListField<OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    previous_iteration_state:
        Option<ListField<OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateEl>>,
}
impl OsConfigV2PolicyOrchestratorOrchestrationStateEl {
    #[doc = "Set the field `current_iteration_state`.\n"]
    pub fn set_current_iteration_state(
        mut self,
        v: impl Into<ListField<OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateEl>>,
    ) -> Self {
        self.current_iteration_state = Some(v.into());
        self
    }
    #[doc = "Set the field `previous_iteration_state`.\n"]
    pub fn set_previous_iteration_state(
        mut self,
        v: impl Into<
            ListField<OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateEl>,
        >,
    ) -> Self {
        self.previous_iteration_state = Some(v.into());
        self
    }
}
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestrationStateEl {
    type O = BlockAssignable<OsConfigV2PolicyOrchestratorOrchestrationStateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigV2PolicyOrchestratorOrchestrationStateEl {}
impl BuildOsConfigV2PolicyOrchestratorOrchestrationStateEl {
    pub fn build(self) -> OsConfigV2PolicyOrchestratorOrchestrationStateEl {
        OsConfigV2PolicyOrchestratorOrchestrationStateEl {
            current_iteration_state: core::default::Default::default(),
            previous_iteration_state: core::default::Default::default(),
        }
    }
}
pub struct OsConfigV2PolicyOrchestratorOrchestrationStateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestrationStateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigV2PolicyOrchestratorOrchestrationStateElRef {
        OsConfigV2PolicyOrchestratorOrchestrationStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigV2PolicyOrchestratorOrchestrationStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `current_iteration_state` after provisioning.\n"]
    pub fn current_iteration_state(
        &self,
    ) -> ListRef<OsConfigV2PolicyOrchestratorOrchestrationStateElCurrentIterationStateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.current_iteration_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `previous_iteration_state` after provisioning.\n"]
    pub fn previous_iteration_state(
        &self,
    ) -> ListRef<OsConfigV2PolicyOrchestratorOrchestrationStateElPreviousIterationStateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.previous_iteration_state", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsEl { # [doc = "Set the field `labels`.\nLabels are identified by key/value pairs in this map.\nA VM should contain all the key/value pairs specified in this\nmap to be selected."] pub fn set_labels (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . labels = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsEl
{}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsEl { labels : core :: default :: Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `labels` after provisioning.\nLabels are identified by key/value pairs in this map.\nA VM should contain all the key/value pairs specified in this\nmap to be selected."] pub fn labels (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.labels" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsEl { # [doc = "Set the field `labels`.\nLabels are identified by key/value pairs in this map.\nA VM should contain all the key/value pairs specified in this\nmap to be selected."] pub fn set_labels (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . labels = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsEl
{}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsEl { labels : core :: default :: Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `labels` after provisioning.\nLabels are identified by key/value pairs in this map.\nA VM should contain all the key/value pairs specified in this\nmap to be selected."] pub fn labels (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.labels" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesEl
{
    os_short_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os_version: Option<PrimField<String>>,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesEl { # [doc = "Set the field `os_version`.\nThe OS version\n\nPrefix matches are supported if asterisk(*) is provided as the\nlast character. For example, to match all versions with a major\nversion of '7', specify the following value for this field '7.*'\n\nAn empty string matches all OS versions."] pub fn set_os_version (mut self , v : impl Into < PrimField < String > >) -> Self { self . os_version = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesEl
{
    #[doc = "Required. The OS short name"]
    pub os_short_name: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesEl { os_short_name : self . os_short_name , os_version : core :: default :: Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `os_short_name` after provisioning.\nRequired. The OS short name"] pub fn os_short_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.os_short_name" , self . base)) } # [doc = "Get a reference to the value of field `os_version` after provisioning.\nThe OS version\n\nPrefix matches are supported if asterisk(*) is provided as the\nlast character. For example, to match all versions with a major\nversion of '7', specify the following value for this field '7.*'\n\nAn empty string matches all OS versions."] pub fn os_version (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.os_version" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElDynamic { exclusion_labels : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsEl >> , inclusion_labels : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsEl >> , inventories : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterEl { # [serde (skip_serializing_if = "Option::is_none")] all : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] exclusion_labels : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsEl > > , # [serde (skip_serializing_if = "Option::is_none")] inclusion_labels : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsEl > > , # [serde (skip_serializing_if = "Option::is_none")] inventories : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesEl > > , dynamic : OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElDynamic , }
impl
    OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterEl
{
    #[doc = "Set the field `all`.\nTarget all VMs in the project. If true, no other criteria is\npermitted."]
    pub fn set_all(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.all = Some(v.into());
        self
    }
    #[doc = "Set the field `exclusion_labels`.\n"]
    pub fn set_exclusion_labels(
        mut self,
        v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsEl >>,
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
        v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsEl >>,
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
        v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesEl >>,
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
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterEl
{}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterEl { all : core :: default :: Default :: default () , exclusion_labels : core :: default :: Default :: default () , inclusion_labels : core :: default :: Default :: default () , inventories : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `all` after provisioning.\nTarget all VMs in the project. If true, no other criteria is\npermitted."] pub fn all (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.all" , self . base)) } # [doc = "Get a reference to the value of field `exclusion_labels` after provisioning.\n"] pub fn exclusion_labels (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.exclusion_labels" , self . base)) } # [doc = "Get a reference to the value of field `inclusion_labels` after provisioning.\n"] pub fn inclusion_labels (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.inclusion_labels" , self . base)) } # [doc = "Get a reference to the value of field `inventories` after provisioning.\n"] pub fn inventories (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.inventories" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersEl
{
    os_short_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os_version: Option<PrimField<String>>,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersEl { # [doc = "Set the field `os_version`.\nThe OS version\n\nPrefix matches are supported if asterisk(*) is provided as the\nlast character. For example, to match all versions with a major\nversion of '7', specify the following value for this field '7.*'\n\nAn empty string matches all OS versions."] pub fn set_os_version (mut self , v : impl Into < PrimField < String > >) -> Self { self . os_version = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersEl
{
    #[doc = "Required. The OS short name"]
    pub os_short_name: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersEl { os_short_name : self . os_short_name , os_version : core :: default :: Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `os_short_name` after provisioning.\nRequired. The OS short name"] pub fn os_short_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.os_short_name" , self . base)) } # [doc = "Get a reference to the value of field `os_version` after provisioning.\nThe OS version\n\nPrefix matches are supported if asterisk(*) is provided as the\nlast character. For example, to match all versions with a major\nversion of '7', specify the following value for this field '7.*'\n\nAn empty string matches all OS versions."] pub fn os_version (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.os_version" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl
{
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation: Option<PrimField<String>>,
    object: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl { # [doc = "Set the field `generation`.\nGeneration number of the Cloud Storage object."] pub fn set_generation (mut self , v : impl Into < PrimField < String > >) -> Self { self . generation = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl
{
    #[doc = "Required. Bucket of the Cloud Storage object."]
    pub bucket: PrimField<String>,
    #[doc = "Required. Name of the Cloud Storage object."]
    pub object: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl { bucket : self . bucket , generation : core :: default :: Default :: default () , object : self . object , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bucket` after provisioning.\nRequired. Bucket of the Cloud Storage object."] pub fn bucket (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bucket" , self . base)) } # [doc = "Get a reference to the value of field `generation` after provisioning.\nGeneration number of the Cloud Storage object."] pub fn generation (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.generation" , self . base)) } # [doc = "Get a reference to the value of field `object` after provisioning.\nRequired. Name of the Cloud Storage object."] pub fn object (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.object" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256_checksum: Option<PrimField<String>>,
    uri: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl { # [doc = "Set the field `sha256_checksum`.\nSHA256 checksum of the remote file."] pub fn set_sha256_checksum (mut self , v : impl Into < PrimField < String > >) -> Self { self . sha256_checksum = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl
{
    #[doc = "Required. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."]
    pub uri: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl { sha256_checksum : core :: default :: Default :: default () , uri : self . uri , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `sha256_checksum` after provisioning.\nSHA256 checksum of the remote file."] pub fn sha256_checksum (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.sha256_checksum" , self . base)) } # [doc = "Get a reference to the value of field `uri` after provisioning.\nRequired. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."] pub fn uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.uri" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElDynamic { gcs : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl >> , remote : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl { # [serde (skip_serializing_if = "Option::is_none")] allow_insecure : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] local_path : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] gcs : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl > > , # [serde (skip_serializing_if = "Option::is_none")] remote : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl > > , dynamic : OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElDynamic , }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl { # [doc = "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."] pub fn set_allow_insecure (mut self , v : impl Into < PrimField < bool > >) -> Self { self . allow_insecure = Some (v . into ()) ; self } # [doc = "Set the field `local_path`.\nA local path within the VM to use."] pub fn set_local_path (mut self , v : impl Into < PrimField < String > >) -> Self { self . local_path = Some (v . into ()) ; self } # [doc = "Set the field `gcs`.\n"] pub fn set_gcs (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . gcs = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . gcs = Some (d) ; } } self } # [doc = "Set the field `remote`.\n"] pub fn set_remote (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . remote = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . remote = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl
{}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl { allow_insecure : core :: default :: Default :: default () , local_path : core :: default :: Default :: default () , gcs : core :: default :: Default :: default () , remote : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."] pub fn allow_insecure (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_insecure" , self . base)) } # [doc = "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."] pub fn local_path (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.local_path" , self . base)) } # [doc = "Get a reference to the value of field `gcs` after provisioning.\n"] pub fn gcs (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.gcs" , self . base)) } # [doc = "Get a reference to the value of field `remote` after provisioning.\n"] pub fn remote (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.remote" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElDynamic { file : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl { # [serde (skip_serializing_if = "Option::is_none")] args : Option < ListField < PrimField < String > > > , interpreter : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] output_file_path : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] script : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] file : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl > > , dynamic : OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElDynamic , }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl { # [doc = "Set the field `args`.\nOptional arguments to pass to the source during execution."] pub fn set_args (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . args = Some (v . into ()) ; self } # [doc = "Set the field `output_file_path`.\nOnly recorded for enforce Exec.\nPath to an output file (that is created by this Exec) whose\ncontent will be recorded in OSPolicyResourceCompliance after a\nsuccessful run. Absence or failure to read this file will result in\nthis ExecResource being non-compliant. Output file size is limited to\n500K bytes."] pub fn set_output_file_path (mut self , v : impl Into < PrimField < String > >) -> Self { self . output_file_path = Some (v . into ()) ; self } # [doc = "Set the field `script`.\nAn inline script.\nThe size of the script is limited to 32KiB."] pub fn set_script (mut self , v : impl Into < PrimField < String > >) -> Self { self . script = Some (v . into ()) ; self } # [doc = "Set the field `file`.\n"] pub fn set_file (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . file = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . file = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl
{
    #[doc = "Required. The script interpreter to use.\nPossible values:\nINTERPRETER_UNSPECIFIED\nNONE\nSHELL\nPOWERSHELL"]
    pub interpreter: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl { args : core :: default :: Default :: default () , interpreter : self . interpreter , output_file_path : core :: default :: Default :: default () , script : core :: default :: Default :: default () , file : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `args` after provisioning.\nOptional arguments to pass to the source during execution."] pub fn args (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.args" , self . base)) } # [doc = "Get a reference to the value of field `interpreter` after provisioning.\nRequired. The script interpreter to use.\nPossible values:\nINTERPRETER_UNSPECIFIED\nNONE\nSHELL\nPOWERSHELL"] pub fn interpreter (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.interpreter" , self . base)) } # [doc = "Get a reference to the value of field `output_file_path` after provisioning.\nOnly recorded for enforce Exec.\nPath to an output file (that is created by this Exec) whose\ncontent will be recorded in OSPolicyResourceCompliance after a\nsuccessful run. Absence or failure to read this file will result in\nthis ExecResource being non-compliant. Output file size is limited to\n500K bytes."] pub fn output_file_path (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.output_file_path" , self . base)) } # [doc = "Get a reference to the value of field `script` after provisioning.\nAn inline script.\nThe size of the script is limited to 32KiB."] pub fn script (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.script" , self . base)) } # [doc = "Get a reference to the value of field `file` after provisioning.\n"] pub fn file (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.file" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl
{
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation: Option<PrimField<String>>,
    object: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl { # [doc = "Set the field `generation`.\nGeneration number of the Cloud Storage object."] pub fn set_generation (mut self , v : impl Into < PrimField < String > >) -> Self { self . generation = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl
{
    #[doc = "Required. Bucket of the Cloud Storage object."]
    pub bucket: PrimField<String>,
    #[doc = "Required. Name of the Cloud Storage object."]
    pub object: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl { bucket : self . bucket , generation : core :: default :: Default :: default () , object : self . object , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bucket` after provisioning.\nRequired. Bucket of the Cloud Storage object."] pub fn bucket (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bucket" , self . base)) } # [doc = "Get a reference to the value of field `generation` after provisioning.\nGeneration number of the Cloud Storage object."] pub fn generation (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.generation" , self . base)) } # [doc = "Get a reference to the value of field `object` after provisioning.\nRequired. Name of the Cloud Storage object."] pub fn object (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.object" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256_checksum: Option<PrimField<String>>,
    uri: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl { # [doc = "Set the field `sha256_checksum`.\nSHA256 checksum of the remote file."] pub fn set_sha256_checksum (mut self , v : impl Into < PrimField < String > >) -> Self { self . sha256_checksum = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl
{
    #[doc = "Required. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."]
    pub uri: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl { sha256_checksum : core :: default :: Default :: default () , uri : self . uri , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `sha256_checksum` after provisioning.\nSHA256 checksum of the remote file."] pub fn sha256_checksum (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.sha256_checksum" , self . base)) } # [doc = "Get a reference to the value of field `uri` after provisioning.\nRequired. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."] pub fn uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.uri" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElDynamic { gcs : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl >> , remote : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl { # [serde (skip_serializing_if = "Option::is_none")] allow_insecure : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] local_path : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] gcs : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl > > , # [serde (skip_serializing_if = "Option::is_none")] remote : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl > > , dynamic : OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElDynamic , }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl { # [doc = "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."] pub fn set_allow_insecure (mut self , v : impl Into < PrimField < bool > >) -> Self { self . allow_insecure = Some (v . into ()) ; self } # [doc = "Set the field `local_path`.\nA local path within the VM to use."] pub fn set_local_path (mut self , v : impl Into < PrimField < String > >) -> Self { self . local_path = Some (v . into ()) ; self } # [doc = "Set the field `gcs`.\n"] pub fn set_gcs (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . gcs = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . gcs = Some (d) ; } } self } # [doc = "Set the field `remote`.\n"] pub fn set_remote (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . remote = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . remote = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl
{}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl { allow_insecure : core :: default :: Default :: default () , local_path : core :: default :: Default :: default () , gcs : core :: default :: Default :: default () , remote : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."] pub fn allow_insecure (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_insecure" , self . base)) } # [doc = "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."] pub fn local_path (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.local_path" , self . base)) } # [doc = "Get a reference to the value of field `gcs` after provisioning.\n"] pub fn gcs (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.gcs" , self . base)) } # [doc = "Get a reference to the value of field `remote` after provisioning.\n"] pub fn remote (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.remote" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElDynamic { file : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateEl { # [serde (skip_serializing_if = "Option::is_none")] args : Option < ListField < PrimField < String > > > , interpreter : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] output_file_path : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] script : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] file : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl > > , dynamic : OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElDynamic , }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateEl { # [doc = "Set the field `args`.\nOptional arguments to pass to the source during execution."] pub fn set_args (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . args = Some (v . into ()) ; self } # [doc = "Set the field `output_file_path`.\nOnly recorded for enforce Exec.\nPath to an output file (that is created by this Exec) whose\ncontent will be recorded in OSPolicyResourceCompliance after a\nsuccessful run. Absence or failure to read this file will result in\nthis ExecResource being non-compliant. Output file size is limited to\n500K bytes."] pub fn set_output_file_path (mut self , v : impl Into < PrimField < String > >) -> Self { self . output_file_path = Some (v . into ()) ; self } # [doc = "Set the field `script`.\nAn inline script.\nThe size of the script is limited to 32KiB."] pub fn set_script (mut self , v : impl Into < PrimField < String > >) -> Self { self . script = Some (v . into ()) ; self } # [doc = "Set the field `file`.\n"] pub fn set_file (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . file = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . file = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateEl
{
    #[doc = "Required. The script interpreter to use.\nPossible values:\nINTERPRETER_UNSPECIFIED\nNONE\nSHELL\nPOWERSHELL"]
    pub interpreter: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateEl { args : core :: default :: Default :: default () , interpreter : self . interpreter , output_file_path : core :: default :: Default :: default () , script : core :: default :: Default :: default () , file : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `args` after provisioning.\nOptional arguments to pass to the source during execution."] pub fn args (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.args" , self . base)) } # [doc = "Get a reference to the value of field `interpreter` after provisioning.\nRequired. The script interpreter to use.\nPossible values:\nINTERPRETER_UNSPECIFIED\nNONE\nSHELL\nPOWERSHELL"] pub fn interpreter (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.interpreter" , self . base)) } # [doc = "Get a reference to the value of field `output_file_path` after provisioning.\nOnly recorded for enforce Exec.\nPath to an output file (that is created by this Exec) whose\ncontent will be recorded in OSPolicyResourceCompliance after a\nsuccessful run. Absence or failure to read this file will result in\nthis ExecResource being non-compliant. Output file size is limited to\n500K bytes."] pub fn output_file_path (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.output_file_path" , self . base)) } # [doc = "Get a reference to the value of field `script` after provisioning.\nAn inline script.\nThe size of the script is limited to 32KiB."] pub fn script (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.script" , self . base)) } # [doc = "Get a reference to the value of field `file` after provisioning.\n"] pub fn file (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.file" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElDynamic { enforce : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl >> , validate : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecEl { # [serde (skip_serializing_if = "Option::is_none")] enforce : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl > > , # [serde (skip_serializing_if = "Option::is_none")] validate : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateEl > > , dynamic : OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElDynamic , }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecEl { # [doc = "Set the field `enforce`.\n"] pub fn set_enforce (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . enforce = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . enforce = Some (d) ; } } self } # [doc = "Set the field `validate`.\n"] pub fn set_validate (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . validate = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . validate = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecEl
{}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecEl { enforce : core :: default :: Default :: default () , validate : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `enforce` after provisioning.\n"] pub fn enforce (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.enforce" , self . base)) } # [doc = "Get a reference to the value of field `validate` after provisioning.\n"] pub fn validate (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.validate" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl
{
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation: Option<PrimField<String>>,
    object: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl { # [doc = "Set the field `generation`.\nGeneration number of the Cloud Storage object."] pub fn set_generation (mut self , v : impl Into < PrimField < String > >) -> Self { self . generation = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl
{
    #[doc = "Required. Bucket of the Cloud Storage object."]
    pub bucket: PrimField<String>,
    #[doc = "Required. Name of the Cloud Storage object."]
    pub object: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl { bucket : self . bucket , generation : core :: default :: Default :: default () , object : self . object , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bucket` after provisioning.\nRequired. Bucket of the Cloud Storage object."] pub fn bucket (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bucket" , self . base)) } # [doc = "Get a reference to the value of field `generation` after provisioning.\nGeneration number of the Cloud Storage object."] pub fn generation (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.generation" , self . base)) } # [doc = "Get a reference to the value of field `object` after provisioning.\nRequired. Name of the Cloud Storage object."] pub fn object (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.object" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256_checksum: Option<PrimField<String>>,
    uri: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl { # [doc = "Set the field `sha256_checksum`.\nSHA256 checksum of the remote file."] pub fn set_sha256_checksum (mut self , v : impl Into < PrimField < String > >) -> Self { self . sha256_checksum = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl
{
    #[doc = "Required. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."]
    pub uri: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl { sha256_checksum : core :: default :: Default :: default () , uri : self . uri , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `sha256_checksum` after provisioning.\nSHA256 checksum of the remote file."] pub fn sha256_checksum (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.sha256_checksum" , self . base)) } # [doc = "Get a reference to the value of field `uri` after provisioning.\nRequired. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."] pub fn uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.uri" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElDynamic { gcs : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl >> , remote : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileEl { # [serde (skip_serializing_if = "Option::is_none")] allow_insecure : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] local_path : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] gcs : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl > > , # [serde (skip_serializing_if = "Option::is_none")] remote : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl > > , dynamic : OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElDynamic , }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileEl { # [doc = "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."] pub fn set_allow_insecure (mut self , v : impl Into < PrimField < bool > >) -> Self { self . allow_insecure = Some (v . into ()) ; self } # [doc = "Set the field `local_path`.\nA local path within the VM to use."] pub fn set_local_path (mut self , v : impl Into < PrimField < String > >) -> Self { self . local_path = Some (v . into ()) ; self } # [doc = "Set the field `gcs`.\n"] pub fn set_gcs (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . gcs = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . gcs = Some (d) ; } } self } # [doc = "Set the field `remote`.\n"] pub fn set_remote (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . remote = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . remote = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileEl
{}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileEl { allow_insecure : core :: default :: Default :: default () , local_path : core :: default :: Default :: default () , gcs : core :: default :: Default :: default () , remote : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."] pub fn allow_insecure (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_insecure" , self . base)) } # [doc = "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."] pub fn local_path (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.local_path" , self . base)) } # [doc = "Get a reference to the value of field `gcs` after provisioning.\n"] pub fn gcs (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.gcs" , self . base)) } # [doc = "Get a reference to the value of field `remote` after provisioning.\n"] pub fn remote (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.remote" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElDynamic { file : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileEl { # [serde (skip_serializing_if = "Option::is_none")] content : Option < PrimField < String > > , path : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] permissions : Option < PrimField < String > > , state : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] file : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileEl > > , dynamic : OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElDynamic , }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileEl { # [doc = "Set the field `content`.\nA a file with this content.\nThe size of the content is limited to 32KiB."] pub fn set_content (mut self , v : impl Into < PrimField < String > >) -> Self { self . content = Some (v . into ()) ; self } # [doc = "Set the field `permissions`.\nConsists of three octal digits which represent, in\norder, the permissions of the owner, group, and other users for the\nfile (similarly to the numeric mode used in the linux chmod\nutility). Each digit represents a three bit number with the 4 bit\ncorresponding to the read permissions, the 2 bit corresponds to the\nwrite bit, and the one bit corresponds to the execute permission.\nDefault behavior is 755.\n\nBelow are some examples of permissions and their associated values:\nread, write, and execute: 7\nread and execute: 5\nread and write: 6\nread only: 4"] pub fn set_permissions (mut self , v : impl Into < PrimField < String > >) -> Self { self . permissions = Some (v . into ()) ; self } # [doc = "Set the field `file`.\n"] pub fn set_file (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . file = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . file = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileEl
{
    #[doc = "Required. The absolute path of the file within the VM."]
    pub path: PrimField<String>,
    #[doc = "Required. Desired state of the file.\nPossible values:\nDESIRED_STATE_UNSPECIFIED\nPRESENT\nABSENT\nCONTENTS_MATCH"]
    pub state: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileEl { content : core :: default :: Default :: default () , path : self . path , permissions : core :: default :: Default :: default () , state : self . state , file : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `content` after provisioning.\nA a file with this content.\nThe size of the content is limited to 32KiB."] pub fn content (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.content" , self . base)) } # [doc = "Get a reference to the value of field `path` after provisioning.\nRequired. The absolute path of the file within the VM."] pub fn path (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.path" , self . base)) } # [doc = "Get a reference to the value of field `permissions` after provisioning.\nConsists of three octal digits which represent, in\norder, the permissions of the owner, group, and other users for the\nfile (similarly to the numeric mode used in the linux chmod\nutility). Each digit represents a three bit number with the 4 bit\ncorresponding to the read permissions, the 2 bit corresponds to the\nwrite bit, and the one bit corresponds to the execute permission.\nDefault behavior is 755.\n\nBelow are some examples of permissions and their associated values:\nread, write, and execute: 7\nread and execute: 5\nread and write: 6\nread only: 4"] pub fn permissions (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.permissions" , self . base)) } # [doc = "Get a reference to the value of field `state` after provisioning.\nRequired. Desired state of the file.\nPossible values:\nDESIRED_STATE_UNSPECIFIED\nPRESENT\nABSENT\nCONTENTS_MATCH"] pub fn state (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.state" , self . base)) } # [doc = "Get a reference to the value of field `file` after provisioning.\n"] pub fn file (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.file" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptEl
{
    name: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptEl { }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptEl
{
    #[doc = "Required. Package name."]
    pub name: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptEl { name : self . name , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\nRequired. Package name."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl
{
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation: Option<PrimField<String>>,
    object: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl { # [doc = "Set the field `generation`.\nGeneration number of the Cloud Storage object."] pub fn set_generation (mut self , v : impl Into < PrimField < String > >) -> Self { self . generation = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl
{
    #[doc = "Required. Bucket of the Cloud Storage object."]
    pub bucket: PrimField<String>,
    #[doc = "Required. Name of the Cloud Storage object."]
    pub object: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl { bucket : self . bucket , generation : core :: default :: Default :: default () , object : self . object , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bucket` after provisioning.\nRequired. Bucket of the Cloud Storage object."] pub fn bucket (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bucket" , self . base)) } # [doc = "Get a reference to the value of field `generation` after provisioning.\nGeneration number of the Cloud Storage object."] pub fn generation (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.generation" , self . base)) } # [doc = "Get a reference to the value of field `object` after provisioning.\nRequired. Name of the Cloud Storage object."] pub fn object (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.object" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256_checksum: Option<PrimField<String>>,
    uri: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl { # [doc = "Set the field `sha256_checksum`.\nSHA256 checksum of the remote file."] pub fn set_sha256_checksum (mut self , v : impl Into < PrimField < String > >) -> Self { self . sha256_checksum = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl
{
    #[doc = "Required. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."]
    pub uri: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl { sha256_checksum : core :: default :: Default :: default () , uri : self . uri , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `sha256_checksum` after provisioning.\nSHA256 checksum of the remote file."] pub fn sha256_checksum (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.sha256_checksum" , self . base)) } # [doc = "Get a reference to the value of field `uri` after provisioning.\nRequired. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."] pub fn uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.uri" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElDynamic { gcs : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl >> , remote : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl { # [serde (skip_serializing_if = "Option::is_none")] allow_insecure : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] local_path : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] gcs : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl > > , # [serde (skip_serializing_if = "Option::is_none")] remote : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl > > , dynamic : OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElDynamic , }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl { # [doc = "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."] pub fn set_allow_insecure (mut self , v : impl Into < PrimField < bool > >) -> Self { self . allow_insecure = Some (v . into ()) ; self } # [doc = "Set the field `local_path`.\nA local path within the VM to use."] pub fn set_local_path (mut self , v : impl Into < PrimField < String > >) -> Self { self . local_path = Some (v . into ()) ; self } # [doc = "Set the field `gcs`.\n"] pub fn set_gcs (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . gcs = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . gcs = Some (d) ; } } self } # [doc = "Set the field `remote`.\n"] pub fn set_remote (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . remote = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . remote = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl
{}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl { allow_insecure : core :: default :: Default :: default () , local_path : core :: default :: Default :: default () , gcs : core :: default :: Default :: default () , remote : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."] pub fn allow_insecure (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_insecure" , self . base)) } # [doc = "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."] pub fn local_path (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.local_path" , self . base)) } # [doc = "Get a reference to the value of field `gcs` after provisioning.\n"] pub fn gcs (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.gcs" , self . base)) } # [doc = "Get a reference to the value of field `remote` after provisioning.\n"] pub fn remote (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.remote" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElDynamic { source : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebEl { # [serde (skip_serializing_if = "Option::is_none")] pull_deps : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] source : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl > > , dynamic : OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElDynamic , }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebEl { # [doc = "Set the field `pull_deps`.\nWhether dependencies should also be installed.\n- install when false: 'dpkg -i package'\n- install when true: 'apt-get update && apt-get -y install\npackage.deb'"] pub fn set_pull_deps (mut self , v : impl Into < PrimField < bool > >) -> Self { self . pull_deps = Some (v . into ()) ; self } # [doc = "Set the field `source`.\n"] pub fn set_source (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . source = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . source = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebEl
{}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebEl { pull_deps : core :: default :: Default :: default () , source : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `pull_deps` after provisioning.\nWhether dependencies should also be installed.\n- install when false: 'dpkg -i package'\n- install when true: 'apt-get update && apt-get -y install\npackage.deb'"] pub fn pull_deps (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pull_deps" , self . base)) } # [doc = "Get a reference to the value of field `source` after provisioning.\n"] pub fn source (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.source" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl
{
    name: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl { }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl
{
    #[doc = "Required. Package name."]
    pub name: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl { name : self . name , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\nRequired. Package name."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl
{
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation: Option<PrimField<String>>,
    object: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl { # [doc = "Set the field `generation`.\nGeneration number of the Cloud Storage object."] pub fn set_generation (mut self , v : impl Into < PrimField < String > >) -> Self { self . generation = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl
{
    #[doc = "Required. Bucket of the Cloud Storage object."]
    pub bucket: PrimField<String>,
    #[doc = "Required. Name of the Cloud Storage object."]
    pub object: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl { bucket : self . bucket , generation : core :: default :: Default :: default () , object : self . object , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bucket` after provisioning.\nRequired. Bucket of the Cloud Storage object."] pub fn bucket (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bucket" , self . base)) } # [doc = "Get a reference to the value of field `generation` after provisioning.\nGeneration number of the Cloud Storage object."] pub fn generation (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.generation" , self . base)) } # [doc = "Get a reference to the value of field `object` after provisioning.\nRequired. Name of the Cloud Storage object."] pub fn object (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.object" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256_checksum: Option<PrimField<String>>,
    uri: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl { # [doc = "Set the field `sha256_checksum`.\nSHA256 checksum of the remote file."] pub fn set_sha256_checksum (mut self , v : impl Into < PrimField < String > >) -> Self { self . sha256_checksum = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl
{
    #[doc = "Required. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."]
    pub uri: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl { sha256_checksum : core :: default :: Default :: default () , uri : self . uri , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `sha256_checksum` after provisioning.\nSHA256 checksum of the remote file."] pub fn sha256_checksum (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.sha256_checksum" , self . base)) } # [doc = "Get a reference to the value of field `uri` after provisioning.\nRequired. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."] pub fn uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.uri" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElDynamic { gcs : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl >> , remote : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl { # [serde (skip_serializing_if = "Option::is_none")] allow_insecure : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] local_path : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] gcs : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl > > , # [serde (skip_serializing_if = "Option::is_none")] remote : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl > > , dynamic : OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElDynamic , }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl { # [doc = "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."] pub fn set_allow_insecure (mut self , v : impl Into < PrimField < bool > >) -> Self { self . allow_insecure = Some (v . into ()) ; self } # [doc = "Set the field `local_path`.\nA local path within the VM to use."] pub fn set_local_path (mut self , v : impl Into < PrimField < String > >) -> Self { self . local_path = Some (v . into ()) ; self } # [doc = "Set the field `gcs`.\n"] pub fn set_gcs (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . gcs = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . gcs = Some (d) ; } } self } # [doc = "Set the field `remote`.\n"] pub fn set_remote (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . remote = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . remote = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl
{}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl { allow_insecure : core :: default :: Default :: default () , local_path : core :: default :: Default :: default () , gcs : core :: default :: Default :: default () , remote : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."] pub fn allow_insecure (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_insecure" , self . base)) } # [doc = "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."] pub fn local_path (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.local_path" , self . base)) } # [doc = "Get a reference to the value of field `gcs` after provisioning.\n"] pub fn gcs (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.gcs" , self . base)) } # [doc = "Get a reference to the value of field `remote` after provisioning.\n"] pub fn remote (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.remote" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElDynamic { source : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl { # [serde (skip_serializing_if = "Option::is_none")] properties : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] source : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl > > , dynamic : OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElDynamic , }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl { # [doc = "Set the field `properties`.\nAdditional properties to use during installation.\nThis should be in the format of Property=Setting.\nAppended to the defaults of 'ACTION=INSTALL\nREBOOT=ReallySuppress'."] pub fn set_properties (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . properties = Some (v . into ()) ; self } # [doc = "Set the field `source`.\n"] pub fn set_source (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . source = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . source = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl
{}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl { properties : core :: default :: Default :: default () , source : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `properties` after provisioning.\nAdditional properties to use during installation.\nThis should be in the format of Property=Setting.\nAppended to the defaults of 'ACTION=INSTALL\nREBOOT=ReallySuppress'."] pub fn properties (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.properties" , self . base)) } # [doc = "Get a reference to the value of field `source` after provisioning.\n"] pub fn source (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.source" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl
{
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation: Option<PrimField<String>>,
    object: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl { # [doc = "Set the field `generation`.\nGeneration number of the Cloud Storage object."] pub fn set_generation (mut self , v : impl Into < PrimField < String > >) -> Self { self . generation = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl
{
    #[doc = "Required. Bucket of the Cloud Storage object."]
    pub bucket: PrimField<String>,
    #[doc = "Required. Name of the Cloud Storage object."]
    pub object: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl { bucket : self . bucket , generation : core :: default :: Default :: default () , object : self . object , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bucket` after provisioning.\nRequired. Bucket of the Cloud Storage object."] pub fn bucket (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bucket" , self . base)) } # [doc = "Get a reference to the value of field `generation` after provisioning.\nGeneration number of the Cloud Storage object."] pub fn generation (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.generation" , self . base)) } # [doc = "Get a reference to the value of field `object` after provisioning.\nRequired. Name of the Cloud Storage object."] pub fn object (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.object" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256_checksum: Option<PrimField<String>>,
    uri: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl { # [doc = "Set the field `sha256_checksum`.\nSHA256 checksum of the remote file."] pub fn set_sha256_checksum (mut self , v : impl Into < PrimField < String > >) -> Self { self . sha256_checksum = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl
{
    #[doc = "Required. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."]
    pub uri: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl { sha256_checksum : core :: default :: Default :: default () , uri : self . uri , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `sha256_checksum` after provisioning.\nSHA256 checksum of the remote file."] pub fn sha256_checksum (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.sha256_checksum" , self . base)) } # [doc = "Get a reference to the value of field `uri` after provisioning.\nRequired. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."] pub fn uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.uri" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElDynamic { gcs : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl >> , remote : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl { # [serde (skip_serializing_if = "Option::is_none")] allow_insecure : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] local_path : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] gcs : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl > > , # [serde (skip_serializing_if = "Option::is_none")] remote : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl > > , dynamic : OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElDynamic , }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl { # [doc = "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."] pub fn set_allow_insecure (mut self , v : impl Into < PrimField < bool > >) -> Self { self . allow_insecure = Some (v . into ()) ; self } # [doc = "Set the field `local_path`.\nA local path within the VM to use."] pub fn set_local_path (mut self , v : impl Into < PrimField < String > >) -> Self { self . local_path = Some (v . into ()) ; self } # [doc = "Set the field `gcs`.\n"] pub fn set_gcs (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . gcs = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . gcs = Some (d) ; } } self } # [doc = "Set the field `remote`.\n"] pub fn set_remote (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . remote = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . remote = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl
{}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl { allow_insecure : core :: default :: Default :: default () , local_path : core :: default :: Default :: default () , gcs : core :: default :: Default :: default () , remote : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."] pub fn allow_insecure (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_insecure" , self . base)) } # [doc = "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."] pub fn local_path (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.local_path" , self . base)) } # [doc = "Get a reference to the value of field `gcs` after provisioning.\n"] pub fn gcs (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.gcs" , self . base)) } # [doc = "Get a reference to the value of field `remote` after provisioning.\n"] pub fn remote (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.remote" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElDynamic { source : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl { # [serde (skip_serializing_if = "Option::is_none")] pull_deps : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] source : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl > > , dynamic : OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElDynamic , }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl { # [doc = "Set the field `pull_deps`.\nWhether dependencies should also be installed.\n- install when false: 'rpm --upgrade --replacepkgs package.rpm'\n- install when true: 'yum -y install package.rpm' or\n'zypper -y install package.rpm'"] pub fn set_pull_deps (mut self , v : impl Into < PrimField < bool > >) -> Self { self . pull_deps = Some (v . into ()) ; self } # [doc = "Set the field `source`.\n"] pub fn set_source (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . source = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . source = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl
{}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl { pull_deps : core :: default :: Default :: default () , source : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `pull_deps` after provisioning.\nWhether dependencies should also be installed.\n- install when false: 'rpm --upgrade --replacepkgs package.rpm'\n- install when true: 'yum -y install package.rpm' or\n'zypper -y install package.rpm'"] pub fn pull_deps (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pull_deps" , self . base)) } # [doc = "Get a reference to the value of field `source` after provisioning.\n"] pub fn source (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.source" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumEl
{
    name: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumEl { }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumEl
{
    #[doc = "Required. Package name."]
    pub name: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumEl { name : self . name , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\nRequired. Package name."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl
{
    name: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl { }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl
{
    #[doc = "Required. Package name."]
    pub name: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl { name : self . name , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\nRequired. Package name."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDynamic { apt : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptEl >> , deb : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebEl >> , googet : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl >> , msi : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl >> , rpm : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl >> , yum : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumEl >> , zypper : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgEl { desired_state : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] apt : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptEl > > , # [serde (skip_serializing_if = "Option::is_none")] deb : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebEl > > , # [serde (skip_serializing_if = "Option::is_none")] googet : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl > > , # [serde (skip_serializing_if = "Option::is_none")] msi : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl > > , # [serde (skip_serializing_if = "Option::is_none")] rpm : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl > > , # [serde (skip_serializing_if = "Option::is_none")] yum : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumEl > > , # [serde (skip_serializing_if = "Option::is_none")] zypper : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl > > , dynamic : OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDynamic , }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgEl { # [doc = "Set the field `apt`.\n"] pub fn set_apt (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . apt = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . apt = Some (d) ; } } self } # [doc = "Set the field `deb`.\n"] pub fn set_deb (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . deb = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . deb = Some (d) ; } } self } # [doc = "Set the field `googet`.\n"] pub fn set_googet (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . googet = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . googet = Some (d) ; } } self } # [doc = "Set the field `msi`.\n"] pub fn set_msi (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . msi = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . msi = Some (d) ; } } self } # [doc = "Set the field `rpm`.\n"] pub fn set_rpm (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . rpm = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . rpm = Some (d) ; } } self } # [doc = "Set the field `yum`.\n"] pub fn set_yum (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . yum = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . yum = Some (d) ; } } self } # [doc = "Set the field `zypper`.\n"] pub fn set_zypper (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . zypper = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . zypper = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgEl
{
    #[doc = "Required. The desired state the agent should maintain for this package.\nPossible values:\nDESIRED_STATE_UNSPECIFIED\nINSTALLED\nREMOVED"]
    pub desired_state: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgEl { desired_state : self . desired_state , apt : core :: default :: Default :: default () , deb : core :: default :: Default :: default () , googet : core :: default :: Default :: default () , msi : core :: default :: Default :: default () , rpm : core :: default :: Default :: default () , yum : core :: default :: Default :: default () , zypper : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `desired_state` after provisioning.\nRequired. The desired state the agent should maintain for this package.\nPossible values:\nDESIRED_STATE_UNSPECIFIED\nINSTALLED\nREMOVED"] pub fn desired_state (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.desired_state" , self . base)) } # [doc = "Get a reference to the value of field `apt` after provisioning.\n"] pub fn apt (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.apt" , self . base)) } # [doc = "Get a reference to the value of field `deb` after provisioning.\n"] pub fn deb (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.deb" , self . base)) } # [doc = "Get a reference to the value of field `googet` after provisioning.\n"] pub fn googet (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.googet" , self . base)) } # [doc = "Get a reference to the value of field `msi` after provisioning.\n"] pub fn msi (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.msi" , self . base)) } # [doc = "Get a reference to the value of field `rpm` after provisioning.\n"] pub fn rpm (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.rpm" , self . base)) } # [doc = "Get a reference to the value of field `yum` after provisioning.\n"] pub fn yum (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.yum" , self . base)) } # [doc = "Get a reference to the value of field `zypper` after provisioning.\n"] pub fn zypper (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.zypper" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl
{
    archive_type: PrimField<String>,
    components: ListField<PrimField<String>>,
    distribution: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpg_key: Option<PrimField<String>>,
    uri: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl { # [doc = "Set the field `gpg_key`.\nURI of the key file for this repository. The agent maintains a\nkeyring at '/etc/apt/trusted.gpg.d/osconfig_agent_managed.gpg'."] pub fn set_gpg_key (mut self , v : impl Into < PrimField < String > >) -> Self { self . gpg_key = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl
{
    #[doc = "Required. Type of archive files in this repository.\nPossible values:\nARCHIVE_TYPE_UNSPECIFIED\nDEB\nDEB_SRC"]
    pub archive_type: PrimField<String>,
    #[doc = "Required. List of components for this repository. Must contain at least one\nitem."]
    pub components: ListField<PrimField<String>>,
    #[doc = "Required. Distribution of this repository."]
    pub distribution: PrimField<String>,
    #[doc = "Required. URI for this repository."]
    pub uri: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl { archive_type : self . archive_type , components : self . components , distribution : self . distribution , gpg_key : core :: default :: Default :: default () , uri : self . uri , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `archive_type` after provisioning.\nRequired. Type of archive files in this repository.\nPossible values:\nARCHIVE_TYPE_UNSPECIFIED\nDEB\nDEB_SRC"] pub fn archive_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.archive_type" , self . base)) } # [doc = "Get a reference to the value of field `components` after provisioning.\nRequired. List of components for this repository. Must contain at least one\nitem."] pub fn components (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.components" , self . base)) } # [doc = "Get a reference to the value of field `distribution` after provisioning.\nRequired. Distribution of this repository."] pub fn distribution (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.distribution" , self . base)) } # [doc = "Get a reference to the value of field `gpg_key` after provisioning.\nURI of the key file for this repository. The agent maintains a\nkeyring at '/etc/apt/trusted.gpg.d/osconfig_agent_managed.gpg'."] pub fn gpg_key (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.gpg_key" , self . base)) } # [doc = "Get a reference to the value of field `uri` after provisioning.\nRequired. URI for this repository."] pub fn uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.uri" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl
{
    name: PrimField<String>,
    url: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl { }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl
{
    #[doc = "Required. The name of the repository."]
    pub name: PrimField<String>,
    #[doc = "Required. The url of the repository."]
    pub url: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl { name : self . name , url : self . url , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\nRequired. The name of the repository."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `url` after provisioning.\nRequired. The url of the repository."] pub fn url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.url" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl
{
    base_url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpg_keys: Option<ListField<PrimField<String>>>,
    id: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl { # [doc = "Set the field `display_name`.\nThe display name of the repository."] pub fn set_display_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . display_name = Some (v . into ()) ; self } # [doc = "Set the field `gpg_keys`.\nURIs of GPG keys."] pub fn set_gpg_keys (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . gpg_keys = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl
{
    #[doc = "Required. The location of the repository directory."]
    pub base_url: PrimField<String>,
    #[doc = "Required. A one word, unique name for this repository. This is  the 'repo\nid' in the yum config file and also the 'display_name' if\n'display_name' is omitted. This id is also used as the unique\nidentifier when checking for resource conflicts."]
    pub id: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl { base_url : self . base_url , display_name : core :: default :: Default :: default () , gpg_keys : core :: default :: Default :: default () , id : self . id , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `base_url` after provisioning.\nRequired. The location of the repository directory."] pub fn base_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.base_url" , self . base)) } # [doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the repository."] pub fn display_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.display_name" , self . base)) } # [doc = "Get a reference to the value of field `gpg_keys` after provisioning.\nURIs of GPG keys."] pub fn gpg_keys (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.gpg_keys" , self . base)) } # [doc = "Get a reference to the value of field `id` after provisioning.\nRequired. A one word, unique name for this repository. This is  the 'repo\nid' in the yum config file and also the 'display_name' if\n'display_name' is omitted. This id is also used as the unique\nidentifier when checking for resource conflicts."] pub fn id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.id" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl
{
    base_url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpg_keys: Option<ListField<PrimField<String>>>,
    id: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl { # [doc = "Set the field `display_name`.\nThe display name of the repository."] pub fn set_display_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . display_name = Some (v . into ()) ; self } # [doc = "Set the field `gpg_keys`.\nURIs of GPG keys."] pub fn set_gpg_keys (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . gpg_keys = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl
{
    #[doc = "Required. The location of the repository directory."]
    pub base_url: PrimField<String>,
    #[doc = "Required. A one word, unique name for this repository. This is the 'repo\nid' in the zypper config file and also the 'display_name' if\n'display_name' is omitted. This id is also used as the unique\nidentifier when checking for GuestPolicy conflicts."]
    pub id: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl { base_url : self . base_url , display_name : core :: default :: Default :: default () , gpg_keys : core :: default :: Default :: default () , id : self . id , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `base_url` after provisioning.\nRequired. The location of the repository directory."] pub fn base_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.base_url" , self . base)) } # [doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the repository."] pub fn display_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.display_name" , self . base)) } # [doc = "Get a reference to the value of field `gpg_keys` after provisioning.\nURIs of GPG keys."] pub fn gpg_keys (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.gpg_keys" , self . base)) } # [doc = "Get a reference to the value of field `id` after provisioning.\nRequired. A one word, unique name for this repository. This is the 'repo\nid' in the zypper config file and also the 'display_name' if\n'display_name' is omitted. This id is also used as the unique\nidentifier when checking for GuestPolicy conflicts."] pub fn id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.id" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElDynamic { apt : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl >> , goo : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl >> , yum : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl >> , zypper : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryEl { # [serde (skip_serializing_if = "Option::is_none")] apt : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl > > , # [serde (skip_serializing_if = "Option::is_none")] goo : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl > > , # [serde (skip_serializing_if = "Option::is_none")] yum : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl > > , # [serde (skip_serializing_if = "Option::is_none")] zypper : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl > > , dynamic : OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElDynamic , }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryEl { # [doc = "Set the field `apt`.\n"] pub fn set_apt (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . apt = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . apt = Some (d) ; } } self } # [doc = "Set the field `goo`.\n"] pub fn set_goo (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . goo = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . goo = Some (d) ; } } self } # [doc = "Set the field `yum`.\n"] pub fn set_yum (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . yum = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . yum = Some (d) ; } } self } # [doc = "Set the field `zypper`.\n"] pub fn set_zypper (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . zypper = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . zypper = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryEl
{}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryEl { apt : core :: default :: Default :: default () , goo : core :: default :: Default :: default () , yum : core :: default :: Default :: default () , zypper : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `apt` after provisioning.\n"] pub fn apt (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.apt" , self . base)) } # [doc = "Get a reference to the value of field `goo` after provisioning.\n"] pub fn goo (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.goo" , self . base)) } # [doc = "Get a reference to the value of field `yum` after provisioning.\n"] pub fn yum (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.yum" , self . base)) } # [doc = "Get a reference to the value of field `zypper` after provisioning.\n"] pub fn zypper (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.zypper" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElDynamic { exec : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecEl >> , file : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileEl >> , pkg : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgEl >> , repository : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesEl { id : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] exec : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecEl > > , # [serde (skip_serializing_if = "Option::is_none")] file : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileEl > > , # [serde (skip_serializing_if = "Option::is_none")] pkg : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgEl > > , # [serde (skip_serializing_if = "Option::is_none")] repository : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryEl > > , dynamic : OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElDynamic , }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesEl { # [doc = "Set the field `exec`.\n"] pub fn set_exec (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . exec = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . exec = Some (d) ; } } self } # [doc = "Set the field `file`.\n"] pub fn set_file (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . file = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . file = Some (d) ; } } self } # [doc = "Set the field `pkg`.\n"] pub fn set_pkg (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . pkg = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . pkg = Some (d) ; } } self } # [doc = "Set the field `repository`.\n"] pub fn set_repository (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . repository = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . repository = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesEl
{
    #[doc = "Required. The id of the resource with the following restrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the OS policy."]
    pub id: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesEl { id : self . id , exec : core :: default :: Default :: default () , file : core :: default :: Default :: default () , pkg : core :: default :: Default :: default () , repository : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `id` after provisioning.\nRequired. The id of the resource with the following restrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the OS policy."] pub fn id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.id" , self . base)) } # [doc = "Get a reference to the value of field `exec` after provisioning.\n"] pub fn exec (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.exec" , self . base)) } # [doc = "Get a reference to the value of field `file` after provisioning.\n"] pub fn file (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.file" , self . base)) } # [doc = "Get a reference to the value of field `pkg` after provisioning.\n"] pub fn pkg (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.pkg" , self . base)) } # [doc = "Get a reference to the value of field `repository` after provisioning.\n"] pub fn repository (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.repository" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElDynamic { inventory_filters : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersEl >> , resources : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsEl { # [serde (skip_serializing_if = "Option::is_none")] inventory_filters : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersEl > > , # [serde (skip_serializing_if = "Option::is_none")] resources : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesEl > > , dynamic : OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElDynamic , }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsEl { # [doc = "Set the field `inventory_filters`.\n"] pub fn set_inventory_filters (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . inventory_filters = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . inventory_filters = Some (d) ; } } self } # [doc = "Set the field `resources`.\n"] pub fn set_resources (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . resources = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . resources = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsEl
{}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsEl { inventory_filters : core :: default :: Default :: default () , resources : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `inventory_filters` after provisioning.\n"] pub fn inventory_filters (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.inventory_filters" , self . base)) } # [doc = "Get a reference to the value of field `resources` after provisioning.\n"] pub fn resources (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.resources" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElDynamic { resource_groups : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesEl { # [serde (skip_serializing_if = "Option::is_none")] allow_no_resource_group_match : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] description : Option < PrimField < String > > , id : PrimField < String > , mode : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] resource_groups : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsEl > > , dynamic : OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElDynamic , }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesEl {
    #[doc = "Set the field `allow_no_resource_group_match`.\nThis flag determines the OS policy compliance status when none of the\nresource groups within the policy are applicable for a VM. Set this value\nto 'true' if the policy needs to be reported as compliant even if the\npolicy has nothing to validate or enforce."]
    pub fn set_allow_no_resource_group_match(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_no_resource_group_match = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nPolicy description.\nLength of the description is limited to 1024 characters."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_groups`.\n"]
    pub fn set_resource_groups(
        mut self,
        v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsEl >>,
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
impl ToListMappable
    for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesEl
{
    type O = BlockAssignable<
        OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesEl
{
    #[doc = "Required. The id of the OS policy with the following restrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the assignment."]
    pub id: PrimField<String>,
    #[doc = "Required. Policy mode\nPossible values:\nMODE_UNSPECIFIED\nVALIDATION\nENFORCEMENT"]
    pub mode: PrimField<String>,
}
impl
    BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesEl
{
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesEl
    {
        OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesEl { allow_no_resource_group_match : core :: default :: Default :: default () , description : core :: default :: Default :: default () , id : self . id , mode : self . mode , resource_groups : core :: default :: Default :: default () , dynamic : Default :: default () , }
    }
}
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElRef { shared : shared , base : base . to_string () , } } }
impl
    OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_no_resource_group_match` after provisioning.\nThis flag determines the OS policy compliance status when none of the\nresource groups within the policy are applicable for a VM. Set this value\nto 'true' if the policy needs to be reported as compliant even if the\npolicy has nothing to validate or enforce."]
    pub fn allow_no_resource_group_match(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_no_resource_group_match", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nPolicy description.\nLength of the description is limited to 1024 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nRequired. The id of the OS policy with the following restrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the assignment."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nRequired. Policy mode\nPossible values:\nMODE_UNSPECIFIED\nVALIDATION\nENFORCEMENT"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_groups` after provisioning.\n"]    pub fn resource_groups (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_groups", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percent: Option<PrimField<f64>>,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetEl { # [doc = "Set the field `fixed`.\nSpecifies a fixed value."] pub fn set_fixed (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . fixed = Some (v . into ()) ; self } # [doc = "Set the field `percent`.\nSpecifies the relative value defined as a percentage, which will be\nmultiplied by a reference value."] pub fn set_percent (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . percent = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetEl
{}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetEl { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetEl { fixed : core :: default :: Default :: default () , percent : core :: default :: Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetElRef { OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `fixed` after provisioning.\nSpecifies a fixed value."] pub fn fixed (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.fixed" , self . base)) } # [doc = "Get a reference to the value of field `percent` after provisioning.\nSpecifies the relative value defined as a percentage, which will be\nmultiplied by a reference value."] pub fn percent (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.percent" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDynamic { disruption_budget : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutEl { min_wait_duration : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] disruption_budget : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetEl > > , dynamic : OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDynamic , }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutEl {
    #[doc = "Set the field `disruption_budget`.\n"]
    pub fn set_disruption_budget(
        mut self,
        v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetEl >>,
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
impl ToListMappable
    for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutEl
{
    type O = BlockAssignable<
        OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutEl
{
    #[doc = "Required. This determines the minimum duration of time to wait after the\nconfiguration changes are applied through the current rollout. A\nVM continues to count towards the 'disruption_budget' at least\nuntil this duration of time has passed after configuration changes are\napplied."]
    pub min_wait_duration: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutEl {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutEl
    {
        OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutEl {
            min_wait_duration: self.min_wait_duration,
            disruption_budget: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElRef
    {
        OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElRef { shared : shared , base : base . to_string () , }
    }
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `min_wait_duration` after provisioning.\nRequired. This determines the minimum duration of time to wait after the\nconfiguration changes are applied through the current rollout. A\nVM continues to count towards the 'disruption_budget' at least\nuntil this duration of time has passed after configuration changes are\napplied."]
    pub fn min_wait_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_wait_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disruption_budget` after provisioning.\n"]    pub fn disruption_budget (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.disruption_budget", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElDynamic { instance_filter : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterEl >> , os_policies : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesEl >> , rollout : Option < DynamicBlock < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadEl { # [serde (skip_serializing_if = "Option::is_none")] description : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] instance_filter : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterEl > > , # [serde (skip_serializing_if = "Option::is_none")] os_policies : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesEl > > , # [serde (skip_serializing_if = "Option::is_none")] rollout : Option < Vec < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutEl > > , dynamic : OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElDynamic , }
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadEl {
    #[doc = "Set the field `description`.\nOS policy assignment description.\nLength of the description is limited to 1024 characters."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nResource name.\n\nFormat:\n'projects/{project_number}/locations/{location}/osPolicyAssignments/{os_policy_assignment_id}'\n\nThis field is ignored when you create an OS policy assignment."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_filter`.\n"]
    pub fn set_instance_filter(
        mut self,
        v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.instance_filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `os_policies`.\n"]
    pub fn set_os_policies(
        mut self,
        v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.os_policies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.os_policies = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rollout`.\n"]
    pub fn set_rollout(
        mut self,
        v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rollout = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rollout = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadEl
{
    type O = BlockAssignable<
        OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadEl {}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadEl {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadEl {
        OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadEl {
            description: core::default::Default::default(),
            name: core::default::Default::default(),
            instance_filter: core::default::Default::default(),
            os_policies: core::default::Default::default(),
            rollout: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRef {
        OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `baseline` after provisioning.\nOutput only. Indicates that this revision has been successfully rolled out in this zone\nand new VMs will be assigned OS policies from this revision.\n\nFor a given OS policy assignment, there is only one revision with a value\nof 'true' for this field."]
    pub fn baseline(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.baseline", self.base))
    }
    #[doc = "Get a reference to the value of field `deleted` after provisioning.\nOutput only. Indicates that this revision deletes the OS policy assignment."]
    pub fn deleted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deleted", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOS policy assignment description.\nLength of the description is limited to 1024 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name.\n\nFormat:\n'projects/{project_number}/locations/{location}/osPolicyAssignments/{os_policy_assignment_id}'\n\nThis field is ignored when you create an OS policy assignment."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. Indicates that reconciliation is in progress for the revision.\nThis value is 'true' when the 'rollout_state' is one of:\n* IN_PROGRESS\n* CANCELLING"]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.base))
    }
    #[doc = "Get a reference to the value of field `revision_create_time` after provisioning.\nOutput only. The timestamp that the revision was created."]
    pub fn revision_create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision_create_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `revision_id` after provisioning.\nOutput only. The assignment revision ID\nA new revision is committed whenever a rollout is triggered for a OS policy\nassignment"]
    pub fn revision_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision_id", self.base))
    }
    #[doc = "Get a reference to the value of field `rollout_state` after provisioning.\nOutput only. OS policy assignment rollout state\nPossible values:\nROLLOUT_STATE_UNSPECIFIED\nIN_PROGRESS\nCANCELLING\nCANCELLED\nSUCCEEDED"]
    pub fn rollout_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rollout_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. Server generated unique id for the OS policy assignment resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
    #[doc = "Get a reference to the value of field `instance_filter` after provisioning.\n"]    pub fn instance_filter (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_filter", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `os_policies` after provisioning.\n"]    pub fn os_policies (& self) -> ListRef < OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElRef >{
        ListRef::new(self.shared().clone(), format!("{}.os_policies", self.base))
    }
    #[doc = "Get a reference to the value of field `rollout` after provisioning.\n"]
    pub fn rollout(
        &self,
    ) -> ListRef<
        OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.rollout", self.base))
    }
}
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorOrchestratedResourceElDynamic {
    os_policy_assignment_v1_payload: Option<
        DynamicBlock<
            OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os_policy_assignment_v1_payload: Option<
        Vec<OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadEl>,
    >,
    dynamic: OsConfigV2PolicyOrchestratorOrchestratedResourceElDynamic,
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceEl {
    #[doc = "Set the field `id`.\nOptional. ID of the resource to be used while generating set of affected resources.\n\nFor UPSERT action the value is auto-generated during PolicyOrchestrator\ncreation when not set. When the value is set it should following next\nrestrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the project.\n\nFor DELETE action, ID must be specified explicitly during\nPolicyOrchestrator creation."]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `os_policy_assignment_v1_payload`.\n"]
    pub fn set_os_policy_assignment_v1_payload(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.os_policy_assignment_v1_payload = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.os_policy_assignment_v1_payload = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestratedResourceEl {
    type O = BlockAssignable<OsConfigV2PolicyOrchestratorOrchestratedResourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigV2PolicyOrchestratorOrchestratedResourceEl {}
impl BuildOsConfigV2PolicyOrchestratorOrchestratedResourceEl {
    pub fn build(self) -> OsConfigV2PolicyOrchestratorOrchestratedResourceEl {
        OsConfigV2PolicyOrchestratorOrchestratedResourceEl {
            id: core::default::Default::default(),
            os_policy_assignment_v1_payload: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigV2PolicyOrchestratorOrchestratedResourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestratedResourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigV2PolicyOrchestratorOrchestratedResourceElRef {
        OsConfigV2PolicyOrchestratorOrchestratedResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigV2PolicyOrchestratorOrchestratedResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nOptional. ID of the resource to be used while generating set of affected resources.\n\nFor UPSERT action the value is auto-generated during PolicyOrchestrator\ncreation when not set. When the value is set it should following next\nrestrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the project.\n\nFor DELETE action, ID must be specified explicitly during\nPolicyOrchestrator creation."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `os_policy_assignment_v1_payload` after provisioning.\n"]
    pub fn os_policy_assignment_v1_payload(
        &self,
    ) -> ListRef<OsConfigV2PolicyOrchestratorOrchestratedResourceElOsPolicyAssignmentV1PayloadElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.os_policy_assignment_v1_payload", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElLocationSelectorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    included_locations: Option<ListField<PrimField<String>>>,
}
impl OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElLocationSelectorEl {
    #[doc = "Set the field `included_locations`.\nOptional. Names of the locations in scope.\nFormat: 'us-central1-a'"]
    pub fn set_included_locations(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.included_locations = Some(v.into());
        self
    }
}
impl ToListMappable
    for OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElLocationSelectorEl
{
    type O = BlockAssignable<
        OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElLocationSelectorEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElLocationSelectorEl {}
impl BuildOsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElLocationSelectorEl {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElLocationSelectorEl {
        OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElLocationSelectorEl {
            included_locations: core::default::Default::default(),
        }
    }
}
pub struct OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElLocationSelectorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElLocationSelectorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElLocationSelectorElRef {
        OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElLocationSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElLocationSelectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `included_locations` after provisioning.\nOptional. Names of the locations in scope.\nFormat: 'us-central1-a'"]
    pub fn included_locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.included_locations", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElResourceHierarchySelectorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    included_folders: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    included_projects: Option<ListField<PrimField<String>>>,
}
impl OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElResourceHierarchySelectorEl {
    #[doc = "Set the field `included_folders`.\nOptional. Names of the folders in scope.\nFormat: 'folders/{folder_id}'"]
    pub fn set_included_folders(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.included_folders = Some(v.into());
        self
    }
    #[doc = "Set the field `included_projects`.\nOptional. Names of the projects in scope.\nFormat: 'projects/{project_number}'"]
    pub fn set_included_projects(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.included_projects = Some(v.into());
        self
    }
}
impl ToListMappable
    for OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElResourceHierarchySelectorEl
{
    type O = BlockAssignable<
        OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElResourceHierarchySelectorEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElResourceHierarchySelectorEl
{}
impl BuildOsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElResourceHierarchySelectorEl {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElResourceHierarchySelectorEl
    {
        OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElResourceHierarchySelectorEl {
            included_folders: core::default::Default::default(),
            included_projects: core::default::Default::default(),
        }
    }
}
pub struct OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElResourceHierarchySelectorElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElResourceHierarchySelectorElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElResourceHierarchySelectorElRef
    {
        OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElResourceHierarchySelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElResourceHierarchySelectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `included_folders` after provisioning.\nOptional. Names of the folders in scope.\nFormat: 'folders/{folder_id}'"]
    pub fn included_folders(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.included_folders", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `included_projects` after provisioning.\nOptional. Names of the projects in scope.\nFormat: 'projects/{project_number}'"]
    pub fn included_projects(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.included_projects", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElDynamic {
    location_selector: Option<
        DynamicBlock<OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElLocationSelectorEl>,
    >,
    resource_hierarchy_selector: Option<
        DynamicBlock<
            OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElResourceHierarchySelectorEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    location_selector:
        Option<Vec<OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElLocationSelectorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_hierarchy_selector: Option<
        Vec<OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElResourceHierarchySelectorEl>,
    >,
    dynamic: OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElDynamic,
}
impl OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsEl {
    #[doc = "Set the field `location_selector`.\n"]
    pub fn set_location_selector(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElLocationSelectorEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.location_selector = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.location_selector = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_hierarchy_selector`.\n"]
    pub fn set_resource_hierarchy_selector(
        mut self,
        v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElResourceHierarchySelectorEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_hierarchy_selector = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_hierarchy_selector = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsEl {
    type O = BlockAssignable<OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsEl {}
impl BuildOsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsEl {
    pub fn build(self) -> OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsEl {
        OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsEl {
            location_selector: core::default::Default::default(),
            resource_hierarchy_selector: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElRef {
        OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `location_selector` after provisioning.\n"]
    pub fn location_selector(
        &self,
    ) -> ListRef<OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElLocationSelectorElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.location_selector", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_hierarchy_selector` after provisioning.\n"]
    pub fn resource_hierarchy_selector(
        &self,
    ) -> ListRef<
        OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElResourceHierarchySelectorElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_hierarchy_selector", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorOrchestrationScopeElDynamic {
    selectors: Option<DynamicBlock<OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsEl>>,
}
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorOrchestrationScopeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    selectors: Option<Vec<OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsEl>>,
    dynamic: OsConfigV2PolicyOrchestratorOrchestrationScopeElDynamic,
}
impl OsConfigV2PolicyOrchestratorOrchestrationScopeEl {
    #[doc = "Set the field `selectors`.\n"]
    pub fn set_selectors(
        mut self,
        v: impl Into<BlockAssignable<OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.selectors = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.selectors = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OsConfigV2PolicyOrchestratorOrchestrationScopeEl {
    type O = BlockAssignable<OsConfigV2PolicyOrchestratorOrchestrationScopeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigV2PolicyOrchestratorOrchestrationScopeEl {}
impl BuildOsConfigV2PolicyOrchestratorOrchestrationScopeEl {
    pub fn build(self) -> OsConfigV2PolicyOrchestratorOrchestrationScopeEl {
        OsConfigV2PolicyOrchestratorOrchestrationScopeEl {
            selectors: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigV2PolicyOrchestratorOrchestrationScopeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorOrchestrationScopeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigV2PolicyOrchestratorOrchestrationScopeElRef {
        OsConfigV2PolicyOrchestratorOrchestrationScopeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigV2PolicyOrchestratorOrchestrationScopeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `selectors` after provisioning.\n"]
    pub fn selectors(
        &self,
    ) -> ListRef<OsConfigV2PolicyOrchestratorOrchestrationScopeElSelectorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.selectors", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl OsConfigV2PolicyOrchestratorTimeoutsEl {
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
impl ToListMappable for OsConfigV2PolicyOrchestratorTimeoutsEl {
    type O = BlockAssignable<OsConfigV2PolicyOrchestratorTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigV2PolicyOrchestratorTimeoutsEl {}
impl BuildOsConfigV2PolicyOrchestratorTimeoutsEl {
    pub fn build(self) -> OsConfigV2PolicyOrchestratorTimeoutsEl {
        OsConfigV2PolicyOrchestratorTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct OsConfigV2PolicyOrchestratorTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> OsConfigV2PolicyOrchestratorTimeoutsElRef {
        OsConfigV2PolicyOrchestratorTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigV2PolicyOrchestratorTimeoutsElRef {
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
struct OsConfigV2PolicyOrchestratorDynamic {
    orchestrated_resource: Option<DynamicBlock<OsConfigV2PolicyOrchestratorOrchestratedResourceEl>>,
    orchestration_scope: Option<DynamicBlock<OsConfigV2PolicyOrchestratorOrchestrationScopeEl>>,
}
