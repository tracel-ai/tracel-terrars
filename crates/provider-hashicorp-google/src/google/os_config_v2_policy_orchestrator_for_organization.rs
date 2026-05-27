use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct OsConfigV2PolicyOrchestratorForOrganizationData {
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
    organization_id: PrimField<String>,
    policy_orchestrator_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orchestrated_resource:
        Option<Vec<OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orchestration_scope:
        Option<Vec<OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<OsConfigV2PolicyOrchestratorForOrganizationTimeoutsEl>,
    dynamic: OsConfigV2PolicyOrchestratorForOrganizationDynamic,
}
struct OsConfigV2PolicyOrchestratorForOrganization_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OsConfigV2PolicyOrchestratorForOrganizationData>,
}
#[derive(Clone)]
pub struct OsConfigV2PolicyOrchestratorForOrganization(
    Rc<OsConfigV2PolicyOrchestratorForOrganization_>,
);
impl OsConfigV2PolicyOrchestratorForOrganization {
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
    #[doc = "Set the field `state`.\nOptional. State of the orchestrator. Can be updated to change orchestrator behaviour.\nAllowed values:\n- 'ACTIVE' - orchestrator is actively looking for actions to be taken.\n- 'STOPPED' - orchestrator won't make any changes.\n\nNote: There might be more states added in the future. We use string here\ninstead of an enum, to avoid the need of propagating new states to all the\nclient code."]
    pub fn set_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().state = Some(v.into());
        self
    }
    #[doc = "Set the field `orchestrated_resource`.\n"]
    pub fn set_orchestrated_resource(
        self,
        v: impl Into<BlockAssignable<OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceEl>>,
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
        v: impl Into<BlockAssignable<OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeEl>>,
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
    pub fn set_timeouts(
        self,
        v: impl Into<OsConfigV2PolicyOrchestratorForOrganizationTimeoutsEl>,
    ) -> Self {
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nOutput only. This checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding."]
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
    ) -> ListRef<OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.orchestration_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization_id` after provisioning.\nPart of 'parent'. Required. The parent resource name in the form of:\n* 'organizations/{organization_id}/locations/global'\n* 'folders/{folder_id}/locations/global'\n* 'projects/{project_id_or_number}/locations/global'"]
    pub fn organization_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_orchestrator_id` after provisioning.\nRequired. The logical identifier of the policy orchestrator, with the following\nrestrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the parent."]
    pub fn policy_orchestrator_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_orchestrator_id", self.extract_ref()),
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
    ) -> ListRef<OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.orchestrated_resource", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `orchestration_scope` after provisioning.\n"]
    pub fn orchestration_scope(
        &self,
    ) -> ListRef<OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.orchestration_scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OsConfigV2PolicyOrchestratorForOrganizationTimeoutsElRef {
        OsConfigV2PolicyOrchestratorForOrganizationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for OsConfigV2PolicyOrchestratorForOrganization {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for OsConfigV2PolicyOrchestratorForOrganization {}
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganization {
    type O = ListRef<OsConfigV2PolicyOrchestratorForOrganizationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for OsConfigV2PolicyOrchestratorForOrganization_ {
    fn extract_resource_type(&self) -> String {
        "google_os_config_v2_policy_orchestrator_for_organization".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildOsConfigV2PolicyOrchestratorForOrganization {
    pub tf_id: String,
    #[doc = "Required. Action to be done by the orchestrator in\n'projects/{project_id}/zones/{zone_id}' locations defined by the\n'orchestration_scope'. Allowed values:\n- 'UPSERT' - Orchestrator will create or update target resources.\n- 'DELETE' - Orchestrator will delete target resources, if they exist"]
    pub action: PrimField<String>,
    #[doc = "Part of 'parent'. Required. The parent resource name in the form of:\n* 'organizations/{organization_id}/locations/global'\n* 'folders/{folder_id}/locations/global'\n* 'projects/{project_id_or_number}/locations/global'"]
    pub organization_id: PrimField<String>,
    #[doc = "Required. The logical identifier of the policy orchestrator, with the following\nrestrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the parent."]
    pub policy_orchestrator_id: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganization {
    pub fn build(self, stack: &mut Stack) -> OsConfigV2PolicyOrchestratorForOrganization {
        let out = OsConfigV2PolicyOrchestratorForOrganization(Rc::new(
            OsConfigV2PolicyOrchestratorForOrganization_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(OsConfigV2PolicyOrchestratorForOrganizationData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    action: self.action,
                    deletion_policy: core::default::Default::default(),
                    description: core::default::Default::default(),
                    id: core::default::Default::default(),
                    labels: core::default::Default::default(),
                    organization_id: self.organization_id,
                    policy_orchestrator_id: self.policy_orchestrator_id,
                    state: core::default::Default::default(),
                    orchestrated_resource: core::default::Default::default(),
                    orchestration_scope: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct OsConfigV2PolicyOrchestratorForOrganizationRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl OsConfigV2PolicyOrchestratorForOrganizationRef {
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nOutput only. This checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding."]
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
    ) -> ListRef<OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.orchestration_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization_id` after provisioning.\nPart of 'parent'. Required. The parent resource name in the form of:\n* 'organizations/{organization_id}/locations/global'\n* 'folders/{folder_id}/locations/global'\n* 'projects/{project_id_or_number}/locations/global'"]
    pub fn organization_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_orchestrator_id` after provisioning.\nRequired. The logical identifier of the policy orchestrator, with the following\nrestrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the parent."]
    pub fn policy_orchestrator_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_orchestrator_id", self.extract_ref()),
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
    ) -> ListRef<OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.orchestrated_resource", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `orchestration_scope` after provisioning.\n"]
    pub fn orchestration_scope(
        &self,
    ) -> ListRef<OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.orchestration_scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OsConfigV2PolicyOrchestratorForOrganizationTimeoutsElRef {
        OsConfigV2PolicyOrchestratorForOrganizationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorElDetailsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    type_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorElDetailsEl { # [doc = "Set the field `type_url`.\n"] pub fn set_type_url (mut self , v : impl Into < PrimField < String > >) -> Self { self . type_url = Some (v . into ()) ; self } # [doc = "Set the field `value`.\n"] pub fn set_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . value = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorElDetailsEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorElDetailsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorElDetailsEl
{}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorElDetailsEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorElDetailsEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorElDetailsEl { type_url : core :: default :: Default :: default () , value : core :: default :: Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorElDetailsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorElDetailsElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorElDetailsElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorElDetailsElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorElDetailsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `type_url` after provisioning.\n"] pub fn type_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type_url" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorEl { # [serde (skip_serializing_if = "Option::is_none")] code : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] details : Option < ListField < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorElDetailsEl > > , # [serde (skip_serializing_if = "Option::is_none")] message : Option < PrimField < String > > , }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        mut self,
        v : impl Into < ListField < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorElDetailsEl > >,
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
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorEl
{}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorEl { code : core :: default :: Default :: default () , details : core :: default :: Default :: default () , message : core :: default :: Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorElRef { shared : shared , base : base . to_string () , } } }
impl
    OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]    pub fn details (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorElDetailsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.details", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateEl { # [serde (skip_serializing_if = "Option::is_none")] error : Option < ListField < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorEl > > , # [serde (skip_serializing_if = "Option::is_none")] failed_actions : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] finish_time : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] performed_actions : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] progress : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] rollout_resource : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] start_time : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] state : Option < PrimField < String > > , }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateEl {
    #[doc = "Set the field `error`.\n"]
    pub fn set_error(
        mut self,
        v : impl Into < ListField < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorEl > >,
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
impl ToListMappable
    for OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateEl
{
    type O = BlockAssignable<
        OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateEl
{}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateEl {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateEl
    {
        OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateEl {
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
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElRef
    {
        OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]    pub fn error (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElErrorElRef >{
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
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorElDetailsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    type_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorElDetailsEl { # [doc = "Set the field `type_url`.\n"] pub fn set_type_url (mut self , v : impl Into < PrimField < String > >) -> Self { self . type_url = Some (v . into ()) ; self } # [doc = "Set the field `value`.\n"] pub fn set_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . value = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorElDetailsEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorElDetailsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorElDetailsEl
{}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorElDetailsEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorElDetailsEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorElDetailsEl { type_url : core :: default :: Default :: default () , value : core :: default :: Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorElDetailsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorElDetailsElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorElDetailsElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorElDetailsElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorElDetailsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `type_url` after provisioning.\n"] pub fn type_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type_url" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorEl { # [serde (skip_serializing_if = "Option::is_none")] code : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] details : Option < ListField < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorElDetailsEl > > , # [serde (skip_serializing_if = "Option::is_none")] message : Option < PrimField < String > > , }
impl
    OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorEl
{
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        mut self,
        v : impl Into < ListField < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorElDetailsEl > >,
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
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorEl
{}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorEl { code : core :: default :: Default :: default () , details : core :: default :: Default :: default () , message : core :: default :: Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `code` after provisioning.\n"] pub fn code (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.code" , self . base)) } # [doc = "Get a reference to the value of field `details` after provisioning.\n"] pub fn details (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorElDetailsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.details" , self . base)) } # [doc = "Get a reference to the value of field `message` after provisioning.\n"] pub fn message (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.message" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateEl { # [serde (skip_serializing_if = "Option::is_none")] error : Option < ListField < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorEl > > , # [serde (skip_serializing_if = "Option::is_none")] failed_actions : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] finish_time : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] performed_actions : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] progress : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] rollout_resource : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] start_time : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] state : Option < PrimField < String > > , }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateEl {
    #[doc = "Set the field `error`.\n"]
    pub fn set_error(
        mut self,
        v : impl Into < ListField < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorEl > >,
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
impl ToListMappable
    for OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateEl
{
    type O = BlockAssignable<
        OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateEl
{}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateEl {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateEl
    {
        OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateEl {
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
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElRef
    {
        OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]    pub fn error (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElErrorElRef >{
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
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    current_iteration_state: Option<
        ListField<
            OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    previous_iteration_state: Option<
        ListField<
            OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateEl,
        >,
    >,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateEl {
    #[doc = "Set the field `current_iteration_state`.\n"]
    pub fn set_current_iteration_state(
        mut self,
        v : impl Into < ListField < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateEl > >,
    ) -> Self {
        self.current_iteration_state = Some(v.into());
        self
    }
    #[doc = "Set the field `previous_iteration_state`.\n"]
    pub fn set_previous_iteration_state(
        mut self,
        v : impl Into < ListField < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateEl > >,
    ) -> Self {
        self.previous_iteration_state = Some(v.into());
        self
    }
}
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateEl {
    type O = BlockAssignable<OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateEl {}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateEl {
    pub fn build(self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateEl {
        OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateEl {
            current_iteration_state: core::default::Default::default(),
            previous_iteration_state: core::default::Default::default(),
        }
    }
}
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElRef {
        OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `current_iteration_state` after provisioning.\n"]
    pub fn current_iteration_state(
        &self,
    ) -> ListRef<
        OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElCurrentIterationStateElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.current_iteration_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `previous_iteration_state` after provisioning.\n"]
    pub fn previous_iteration_state(
        &self,
    ) -> ListRef<
        OsConfigV2PolicyOrchestratorForOrganizationOrchestrationStateElPreviousIterationStateElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.previous_iteration_state", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsEl { # [doc = "Set the field `labels`.\nLabels are identified by key/value pairs in this map.\nA VM should contain all the key/value pairs specified in this\nmap to be selected."] pub fn set_labels (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . labels = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsEl
{}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsEl { labels : core :: default :: Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `labels` after provisioning.\nLabels are identified by key/value pairs in this map.\nA VM should contain all the key/value pairs specified in this\nmap to be selected."] pub fn labels (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.labels" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsEl { # [doc = "Set the field `labels`.\nLabels are identified by key/value pairs in this map.\nA VM should contain all the key/value pairs specified in this\nmap to be selected."] pub fn set_labels (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . labels = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsEl
{}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsEl { labels : core :: default :: Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `labels` after provisioning.\nLabels are identified by key/value pairs in this map.\nA VM should contain all the key/value pairs specified in this\nmap to be selected."] pub fn labels (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.labels" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesEl
{
    os_short_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os_version: Option<PrimField<String>>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesEl { # [doc = "Set the field `os_version`.\nThe OS version\n\nPrefix matches are supported if asterisk(*) is provided as the\nlast character. For example, to match all versions with a major\nversion of '7', specify the following value for this field '7.*'\n\nAn empty string matches all OS versions."] pub fn set_os_version (mut self , v : impl Into < PrimField < String > >) -> Self { self . os_version = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesEl
{
    #[doc = "Required. The OS short name"]
    pub os_short_name: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesEl { os_short_name : self . os_short_name , os_version : core :: default :: Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `os_short_name` after provisioning.\nRequired. The OS short name"] pub fn os_short_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.os_short_name" , self . base)) } # [doc = "Get a reference to the value of field `os_version` after provisioning.\nThe OS version\n\nPrefix matches are supported if asterisk(*) is provided as the\nlast character. For example, to match all versions with a major\nversion of '7', specify the following value for this field '7.*'\n\nAn empty string matches all OS versions."] pub fn os_version (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.os_version" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElDynamic { exclusion_labels : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsEl >> , inclusion_labels : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsEl >> , inventories : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterEl { # [serde (skip_serializing_if = "Option::is_none")] all : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] exclusion_labels : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsEl > > , # [serde (skip_serializing_if = "Option::is_none")] inclusion_labels : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsEl > > , # [serde (skip_serializing_if = "Option::is_none")] inventories : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesEl > > , dynamic : OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElDynamic , }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterEl { # [doc = "Set the field `all`.\nTarget all VMs in the project. If true, no other criteria is\npermitted."] pub fn set_all (mut self , v : impl Into < PrimField < bool > >) -> Self { self . all = Some (v . into ()) ; self } # [doc = "Set the field `exclusion_labels`.\n"] pub fn set_exclusion_labels (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . exclusion_labels = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . exclusion_labels = Some (d) ; } } self } # [doc = "Set the field `inclusion_labels`.\n"] pub fn set_inclusion_labels (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . inclusion_labels = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . inclusion_labels = Some (d) ; } } self } # [doc = "Set the field `inventories`.\n"] pub fn set_inventories (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . inventories = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . inventories = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterEl
{}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterEl { all : core :: default :: Default :: default () , exclusion_labels : core :: default :: Default :: default () , inclusion_labels : core :: default :: Default :: default () , inventories : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `all` after provisioning.\nTarget all VMs in the project. If true, no other criteria is\npermitted."] pub fn all (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.all" , self . base)) } # [doc = "Get a reference to the value of field `exclusion_labels` after provisioning.\n"] pub fn exclusion_labels (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElExclusionLabelsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.exclusion_labels" , self . base)) } # [doc = "Get a reference to the value of field `inclusion_labels` after provisioning.\n"] pub fn inclusion_labels (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInclusionLabelsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.inclusion_labels" , self . base)) } # [doc = "Get a reference to the value of field `inventories` after provisioning.\n"] pub fn inventories (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElInventoriesElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.inventories" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersEl
{
    os_short_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os_version: Option<PrimField<String>>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersEl { # [doc = "Set the field `os_version`.\nThe OS version\n\nPrefix matches are supported if asterisk(*) is provided as the\nlast character. For example, to match all versions with a major\nversion of '7', specify the following value for this field '7.*'\n\nAn empty string matches all OS versions."] pub fn set_os_version (mut self , v : impl Into < PrimField < String > >) -> Self { self . os_version = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersEl
{
    #[doc = "Required. The OS short name"]
    pub os_short_name: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersEl { os_short_name : self . os_short_name , os_version : core :: default :: Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `os_short_name` after provisioning.\nRequired. The OS short name"] pub fn os_short_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.os_short_name" , self . base)) } # [doc = "Get a reference to the value of field `os_version` after provisioning.\nThe OS version\n\nPrefix matches are supported if asterisk(*) is provided as the\nlast character. For example, to match all versions with a major\nversion of '7', specify the following value for this field '7.*'\n\nAn empty string matches all OS versions."] pub fn os_version (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.os_version" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl
{
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation: Option<PrimField<String>>,
    object: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl { # [doc = "Set the field `generation`.\nGeneration number of the Cloud Storage object."] pub fn set_generation (mut self , v : impl Into < PrimField < String > >) -> Self { self . generation = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl
{
    #[doc = "Required. Bucket of the Cloud Storage object."]
    pub bucket: PrimField<String>,
    #[doc = "Required. Name of the Cloud Storage object."]
    pub object: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl { bucket : self . bucket , generation : core :: default :: Default :: default () , object : self . object , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bucket` after provisioning.\nRequired. Bucket of the Cloud Storage object."] pub fn bucket (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bucket" , self . base)) } # [doc = "Get a reference to the value of field `generation` after provisioning.\nGeneration number of the Cloud Storage object."] pub fn generation (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.generation" , self . base)) } # [doc = "Get a reference to the value of field `object` after provisioning.\nRequired. Name of the Cloud Storage object."] pub fn object (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.object" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256_checksum: Option<PrimField<String>>,
    uri: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl { # [doc = "Set the field `sha256_checksum`.\nSHA256 checksum of the remote file."] pub fn set_sha256_checksum (mut self , v : impl Into < PrimField < String > >) -> Self { self . sha256_checksum = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl
{
    #[doc = "Required. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."]
    pub uri: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl { sha256_checksum : core :: default :: Default :: default () , uri : self . uri , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `sha256_checksum` after provisioning.\nSHA256 checksum of the remote file."] pub fn sha256_checksum (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.sha256_checksum" , self . base)) } # [doc = "Get a reference to the value of field `uri` after provisioning.\nRequired. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."] pub fn uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.uri" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElDynamic { gcs : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl >> , remote : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl { # [serde (skip_serializing_if = "Option::is_none")] allow_insecure : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] local_path : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] gcs : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl > > , # [serde (skip_serializing_if = "Option::is_none")] remote : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl > > , dynamic : OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElDynamic , }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl { # [doc = "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."] pub fn set_allow_insecure (mut self , v : impl Into < PrimField < bool > >) -> Self { self . allow_insecure = Some (v . into ()) ; self } # [doc = "Set the field `local_path`.\nA local path within the VM to use."] pub fn set_local_path (mut self , v : impl Into < PrimField < String > >) -> Self { self . local_path = Some (v . into ()) ; self } # [doc = "Set the field `gcs`.\n"] pub fn set_gcs (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . gcs = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . gcs = Some (d) ; } } self } # [doc = "Set the field `remote`.\n"] pub fn set_remote (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . remote = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . remote = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl
{}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl { allow_insecure : core :: default :: Default :: default () , local_path : core :: default :: Default :: default () , gcs : core :: default :: Default :: default () , remote : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."] pub fn allow_insecure (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_insecure" , self . base)) } # [doc = "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."] pub fn local_path (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.local_path" , self . base)) } # [doc = "Get a reference to the value of field `gcs` after provisioning.\n"] pub fn gcs (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.gcs" , self . base)) } # [doc = "Get a reference to the value of field `remote` after provisioning.\n"] pub fn remote (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.remote" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElDynamic { file : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl { # [serde (skip_serializing_if = "Option::is_none")] args : Option < ListField < PrimField < String > > > , interpreter : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] output_file_path : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] script : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] file : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl > > , dynamic : OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElDynamic , }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl { # [doc = "Set the field `args`.\nOptional arguments to pass to the source during execution."] pub fn set_args (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . args = Some (v . into ()) ; self } # [doc = "Set the field `output_file_path`.\nOnly recorded for enforce Exec.\nPath to an output file (that is created by this Exec) whose\ncontent will be recorded in OSPolicyResourceCompliance after a\nsuccessful run. Absence or failure to read this file will result in\nthis ExecResource being non-compliant. Output file size is limited to\n500K bytes."] pub fn set_output_file_path (mut self , v : impl Into < PrimField < String > >) -> Self { self . output_file_path = Some (v . into ()) ; self } # [doc = "Set the field `script`.\nAn inline script.\nThe size of the script is limited to 32KiB."] pub fn set_script (mut self , v : impl Into < PrimField < String > >) -> Self { self . script = Some (v . into ()) ; self } # [doc = "Set the field `file`.\n"] pub fn set_file (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . file = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . file = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl
{
    #[doc = "Required. The script interpreter to use.\nPossible values:\nINTERPRETER_UNSPECIFIED\nNONE\nSHELL\nPOWERSHELL"]
    pub interpreter: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl { args : core :: default :: Default :: default () , interpreter : self . interpreter , output_file_path : core :: default :: Default :: default () , script : core :: default :: Default :: default () , file : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `args` after provisioning.\nOptional arguments to pass to the source during execution."] pub fn args (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.args" , self . base)) } # [doc = "Get a reference to the value of field `interpreter` after provisioning.\nRequired. The script interpreter to use.\nPossible values:\nINTERPRETER_UNSPECIFIED\nNONE\nSHELL\nPOWERSHELL"] pub fn interpreter (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.interpreter" , self . base)) } # [doc = "Get a reference to the value of field `output_file_path` after provisioning.\nOnly recorded for enforce Exec.\nPath to an output file (that is created by this Exec) whose\ncontent will be recorded in OSPolicyResourceCompliance after a\nsuccessful run. Absence or failure to read this file will result in\nthis ExecResource being non-compliant. Output file size is limited to\n500K bytes."] pub fn output_file_path (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.output_file_path" , self . base)) } # [doc = "Get a reference to the value of field `script` after provisioning.\nAn inline script.\nThe size of the script is limited to 32KiB."] pub fn script (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.script" , self . base)) } # [doc = "Get a reference to the value of field `file` after provisioning.\n"] pub fn file (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.file" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl
{
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation: Option<PrimField<String>>,
    object: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl { # [doc = "Set the field `generation`.\nGeneration number of the Cloud Storage object."] pub fn set_generation (mut self , v : impl Into < PrimField < String > >) -> Self { self . generation = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl
{
    #[doc = "Required. Bucket of the Cloud Storage object."]
    pub bucket: PrimField<String>,
    #[doc = "Required. Name of the Cloud Storage object."]
    pub object: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl { bucket : self . bucket , generation : core :: default :: Default :: default () , object : self . object , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bucket` after provisioning.\nRequired. Bucket of the Cloud Storage object."] pub fn bucket (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bucket" , self . base)) } # [doc = "Get a reference to the value of field `generation` after provisioning.\nGeneration number of the Cloud Storage object."] pub fn generation (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.generation" , self . base)) } # [doc = "Get a reference to the value of field `object` after provisioning.\nRequired. Name of the Cloud Storage object."] pub fn object (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.object" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256_checksum: Option<PrimField<String>>,
    uri: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl { # [doc = "Set the field `sha256_checksum`.\nSHA256 checksum of the remote file."] pub fn set_sha256_checksum (mut self , v : impl Into < PrimField < String > >) -> Self { self . sha256_checksum = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl
{
    #[doc = "Required. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."]
    pub uri: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl { sha256_checksum : core :: default :: Default :: default () , uri : self . uri , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `sha256_checksum` after provisioning.\nSHA256 checksum of the remote file."] pub fn sha256_checksum (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.sha256_checksum" , self . base)) } # [doc = "Get a reference to the value of field `uri` after provisioning.\nRequired. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."] pub fn uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.uri" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElDynamic { gcs : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl >> , remote : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl { # [serde (skip_serializing_if = "Option::is_none")] allow_insecure : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] local_path : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] gcs : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl > > , # [serde (skip_serializing_if = "Option::is_none")] remote : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl > > , dynamic : OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElDynamic , }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl { # [doc = "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."] pub fn set_allow_insecure (mut self , v : impl Into < PrimField < bool > >) -> Self { self . allow_insecure = Some (v . into ()) ; self } # [doc = "Set the field `local_path`.\nA local path within the VM to use."] pub fn set_local_path (mut self , v : impl Into < PrimField < String > >) -> Self { self . local_path = Some (v . into ()) ; self } # [doc = "Set the field `gcs`.\n"] pub fn set_gcs (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . gcs = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . gcs = Some (d) ; } } self } # [doc = "Set the field `remote`.\n"] pub fn set_remote (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . remote = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . remote = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl
{}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl { allow_insecure : core :: default :: Default :: default () , local_path : core :: default :: Default :: default () , gcs : core :: default :: Default :: default () , remote : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."] pub fn allow_insecure (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_insecure" , self . base)) } # [doc = "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."] pub fn local_path (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.local_path" , self . base)) } # [doc = "Get a reference to the value of field `gcs` after provisioning.\n"] pub fn gcs (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.gcs" , self . base)) } # [doc = "Get a reference to the value of field `remote` after provisioning.\n"] pub fn remote (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.remote" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElDynamic { file : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateEl { # [serde (skip_serializing_if = "Option::is_none")] args : Option < ListField < PrimField < String > > > , interpreter : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] output_file_path : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] script : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] file : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl > > , dynamic : OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElDynamic , }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateEl { # [doc = "Set the field `args`.\nOptional arguments to pass to the source during execution."] pub fn set_args (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . args = Some (v . into ()) ; self } # [doc = "Set the field `output_file_path`.\nOnly recorded for enforce Exec.\nPath to an output file (that is created by this Exec) whose\ncontent will be recorded in OSPolicyResourceCompliance after a\nsuccessful run. Absence or failure to read this file will result in\nthis ExecResource being non-compliant. Output file size is limited to\n500K bytes."] pub fn set_output_file_path (mut self , v : impl Into < PrimField < String > >) -> Self { self . output_file_path = Some (v . into ()) ; self } # [doc = "Set the field `script`.\nAn inline script.\nThe size of the script is limited to 32KiB."] pub fn set_script (mut self , v : impl Into < PrimField < String > >) -> Self { self . script = Some (v . into ()) ; self } # [doc = "Set the field `file`.\n"] pub fn set_file (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . file = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . file = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateEl
{
    #[doc = "Required. The script interpreter to use.\nPossible values:\nINTERPRETER_UNSPECIFIED\nNONE\nSHELL\nPOWERSHELL"]
    pub interpreter: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateEl { args : core :: default :: Default :: default () , interpreter : self . interpreter , output_file_path : core :: default :: Default :: default () , script : core :: default :: Default :: default () , file : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `args` after provisioning.\nOptional arguments to pass to the source during execution."] pub fn args (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.args" , self . base)) } # [doc = "Get a reference to the value of field `interpreter` after provisioning.\nRequired. The script interpreter to use.\nPossible values:\nINTERPRETER_UNSPECIFIED\nNONE\nSHELL\nPOWERSHELL"] pub fn interpreter (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.interpreter" , self . base)) } # [doc = "Get a reference to the value of field `output_file_path` after provisioning.\nOnly recorded for enforce Exec.\nPath to an output file (that is created by this Exec) whose\ncontent will be recorded in OSPolicyResourceCompliance after a\nsuccessful run. Absence or failure to read this file will result in\nthis ExecResource being non-compliant. Output file size is limited to\n500K bytes."] pub fn output_file_path (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.output_file_path" , self . base)) } # [doc = "Get a reference to the value of field `script` after provisioning.\nAn inline script.\nThe size of the script is limited to 32KiB."] pub fn script (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.script" , self . base)) } # [doc = "Get a reference to the value of field `file` after provisioning.\n"] pub fn file (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.file" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElDynamic { enforce : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl >> , validate : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecEl { # [serde (skip_serializing_if = "Option::is_none")] enforce : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl > > , # [serde (skip_serializing_if = "Option::is_none")] validate : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateEl > > , dynamic : OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElDynamic , }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecEl { # [doc = "Set the field `enforce`.\n"] pub fn set_enforce (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . enforce = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . enforce = Some (d) ; } } self } # [doc = "Set the field `validate`.\n"] pub fn set_validate (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . validate = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . validate = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecEl
{}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecEl { enforce : core :: default :: Default :: default () , validate : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `enforce` after provisioning.\n"] pub fn enforce (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElEnforceElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.enforce" , self . base)) } # [doc = "Get a reference to the value of field `validate` after provisioning.\n"] pub fn validate (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElValidateElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.validate" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl
{
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation: Option<PrimField<String>>,
    object: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl { # [doc = "Set the field `generation`.\nGeneration number of the Cloud Storage object."] pub fn set_generation (mut self , v : impl Into < PrimField < String > >) -> Self { self . generation = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl
{
    #[doc = "Required. Bucket of the Cloud Storage object."]
    pub bucket: PrimField<String>,
    #[doc = "Required. Name of the Cloud Storage object."]
    pub object: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl { bucket : self . bucket , generation : core :: default :: Default :: default () , object : self . object , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bucket` after provisioning.\nRequired. Bucket of the Cloud Storage object."] pub fn bucket (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bucket" , self . base)) } # [doc = "Get a reference to the value of field `generation` after provisioning.\nGeneration number of the Cloud Storage object."] pub fn generation (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.generation" , self . base)) } # [doc = "Get a reference to the value of field `object` after provisioning.\nRequired. Name of the Cloud Storage object."] pub fn object (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.object" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256_checksum: Option<PrimField<String>>,
    uri: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl { # [doc = "Set the field `sha256_checksum`.\nSHA256 checksum of the remote file."] pub fn set_sha256_checksum (mut self , v : impl Into < PrimField < String > >) -> Self { self . sha256_checksum = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl
{
    #[doc = "Required. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."]
    pub uri: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl { sha256_checksum : core :: default :: Default :: default () , uri : self . uri , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `sha256_checksum` after provisioning.\nSHA256 checksum of the remote file."] pub fn sha256_checksum (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.sha256_checksum" , self . base)) } # [doc = "Get a reference to the value of field `uri` after provisioning.\nRequired. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."] pub fn uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.uri" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElDynamic { gcs : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl >> , remote : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileEl { # [serde (skip_serializing_if = "Option::is_none")] allow_insecure : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] local_path : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] gcs : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl > > , # [serde (skip_serializing_if = "Option::is_none")] remote : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl > > , dynamic : OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElDynamic , }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileEl { # [doc = "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."] pub fn set_allow_insecure (mut self , v : impl Into < PrimField < bool > >) -> Self { self . allow_insecure = Some (v . into ()) ; self } # [doc = "Set the field `local_path`.\nA local path within the VM to use."] pub fn set_local_path (mut self , v : impl Into < PrimField < String > >) -> Self { self . local_path = Some (v . into ()) ; self } # [doc = "Set the field `gcs`.\n"] pub fn set_gcs (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . gcs = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . gcs = Some (d) ; } } self } # [doc = "Set the field `remote`.\n"] pub fn set_remote (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . remote = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . remote = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileEl
{}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileEl { allow_insecure : core :: default :: Default :: default () , local_path : core :: default :: Default :: default () , gcs : core :: default :: Default :: default () , remote : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."] pub fn allow_insecure (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_insecure" , self . base)) } # [doc = "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."] pub fn local_path (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.local_path" , self . base)) } # [doc = "Get a reference to the value of field `gcs` after provisioning.\n"] pub fn gcs (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.gcs" , self . base)) } # [doc = "Get a reference to the value of field `remote` after provisioning.\n"] pub fn remote (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.remote" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElDynamic { file : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileEl { # [serde (skip_serializing_if = "Option::is_none")] content : Option < PrimField < String > > , path : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] permissions : Option < PrimField < String > > , state : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] file : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileEl > > , dynamic : OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElDynamic , }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileEl { # [doc = "Set the field `content`.\nA a file with this content.\nThe size of the content is limited to 32KiB."] pub fn set_content (mut self , v : impl Into < PrimField < String > >) -> Self { self . content = Some (v . into ()) ; self } # [doc = "Set the field `permissions`.\nConsists of three octal digits which represent, in\norder, the permissions of the owner, group, and other users for the\nfile (similarly to the numeric mode used in the linux chmod\nutility). Each digit represents a three bit number with the 4 bit\ncorresponding to the read permissions, the 2 bit corresponds to the\nwrite bit, and the one bit corresponds to the execute permission.\nDefault behavior is 755.\n\nBelow are some examples of permissions and their associated values:\nread, write, and execute: 7\nread and execute: 5\nread and write: 6\nread only: 4"] pub fn set_permissions (mut self , v : impl Into < PrimField < String > >) -> Self { self . permissions = Some (v . into ()) ; self } # [doc = "Set the field `file`.\n"] pub fn set_file (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . file = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . file = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileEl
{
    #[doc = "Required. The absolute path of the file within the VM."]
    pub path: PrimField<String>,
    #[doc = "Required. Desired state of the file.\nPossible values:\nDESIRED_STATE_UNSPECIFIED\nPRESENT\nABSENT\nCONTENTS_MATCH"]
    pub state: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileEl { content : core :: default :: Default :: default () , path : self . path , permissions : core :: default :: Default :: default () , state : self . state , file : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `content` after provisioning.\nA a file with this content.\nThe size of the content is limited to 32KiB."] pub fn content (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.content" , self . base)) } # [doc = "Get a reference to the value of field `path` after provisioning.\nRequired. The absolute path of the file within the VM."] pub fn path (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.path" , self . base)) } # [doc = "Get a reference to the value of field `permissions` after provisioning.\nConsists of three octal digits which represent, in\norder, the permissions of the owner, group, and other users for the\nfile (similarly to the numeric mode used in the linux chmod\nutility). Each digit represents a three bit number with the 4 bit\ncorresponding to the read permissions, the 2 bit corresponds to the\nwrite bit, and the one bit corresponds to the execute permission.\nDefault behavior is 755.\n\nBelow are some examples of permissions and their associated values:\nread, write, and execute: 7\nread and execute: 5\nread and write: 6\nread only: 4"] pub fn permissions (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.permissions" , self . base)) } # [doc = "Get a reference to the value of field `state` after provisioning.\nRequired. Desired state of the file.\nPossible values:\nDESIRED_STATE_UNSPECIFIED\nPRESENT\nABSENT\nCONTENTS_MATCH"] pub fn state (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.state" , self . base)) } # [doc = "Get a reference to the value of field `file` after provisioning.\n"] pub fn file (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElFileElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.file" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptEl
{
    name: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptEl { }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptEl
{
    #[doc = "Required. Package name."]
    pub name: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptEl { name : self . name , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\nRequired. Package name."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl
{
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation: Option<PrimField<String>>,
    object: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl { # [doc = "Set the field `generation`.\nGeneration number of the Cloud Storage object."] pub fn set_generation (mut self , v : impl Into < PrimField < String > >) -> Self { self . generation = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl
{
    #[doc = "Required. Bucket of the Cloud Storage object."]
    pub bucket: PrimField<String>,
    #[doc = "Required. Name of the Cloud Storage object."]
    pub object: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl { bucket : self . bucket , generation : core :: default :: Default :: default () , object : self . object , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bucket` after provisioning.\nRequired. Bucket of the Cloud Storage object."] pub fn bucket (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bucket" , self . base)) } # [doc = "Get a reference to the value of field `generation` after provisioning.\nGeneration number of the Cloud Storage object."] pub fn generation (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.generation" , self . base)) } # [doc = "Get a reference to the value of field `object` after provisioning.\nRequired. Name of the Cloud Storage object."] pub fn object (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.object" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256_checksum: Option<PrimField<String>>,
    uri: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl { # [doc = "Set the field `sha256_checksum`.\nSHA256 checksum of the remote file."] pub fn set_sha256_checksum (mut self , v : impl Into < PrimField < String > >) -> Self { self . sha256_checksum = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl
{
    #[doc = "Required. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."]
    pub uri: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl { sha256_checksum : core :: default :: Default :: default () , uri : self . uri , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `sha256_checksum` after provisioning.\nSHA256 checksum of the remote file."] pub fn sha256_checksum (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.sha256_checksum" , self . base)) } # [doc = "Get a reference to the value of field `uri` after provisioning.\nRequired. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."] pub fn uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.uri" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElDynamic { gcs : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl >> , remote : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl { # [serde (skip_serializing_if = "Option::is_none")] allow_insecure : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] local_path : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] gcs : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl > > , # [serde (skip_serializing_if = "Option::is_none")] remote : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl > > , dynamic : OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElDynamic , }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl { # [doc = "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."] pub fn set_allow_insecure (mut self , v : impl Into < PrimField < bool > >) -> Self { self . allow_insecure = Some (v . into ()) ; self } # [doc = "Set the field `local_path`.\nA local path within the VM to use."] pub fn set_local_path (mut self , v : impl Into < PrimField < String > >) -> Self { self . local_path = Some (v . into ()) ; self } # [doc = "Set the field `gcs`.\n"] pub fn set_gcs (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . gcs = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . gcs = Some (d) ; } } self } # [doc = "Set the field `remote`.\n"] pub fn set_remote (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . remote = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . remote = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl
{}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl { allow_insecure : core :: default :: Default :: default () , local_path : core :: default :: Default :: default () , gcs : core :: default :: Default :: default () , remote : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."] pub fn allow_insecure (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_insecure" , self . base)) } # [doc = "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."] pub fn local_path (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.local_path" , self . base)) } # [doc = "Get a reference to the value of field `gcs` after provisioning.\n"] pub fn gcs (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.gcs" , self . base)) } # [doc = "Get a reference to the value of field `remote` after provisioning.\n"] pub fn remote (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.remote" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElDynamic { source : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebEl { # [serde (skip_serializing_if = "Option::is_none")] pull_deps : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] source : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl > > , dynamic : OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElDynamic , }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebEl { # [doc = "Set the field `pull_deps`.\nWhether dependencies should also be installed.\n- install when false: 'dpkg -i package'\n- install when true: 'apt-get update && apt-get -y install\npackage.deb'"] pub fn set_pull_deps (mut self , v : impl Into < PrimField < bool > >) -> Self { self . pull_deps = Some (v . into ()) ; self } # [doc = "Set the field `source`.\n"] pub fn set_source (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . source = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . source = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebEl
{}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebEl { pull_deps : core :: default :: Default :: default () , source : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `pull_deps` after provisioning.\nWhether dependencies should also be installed.\n- install when false: 'dpkg -i package'\n- install when true: 'apt-get update && apt-get -y install\npackage.deb'"] pub fn pull_deps (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pull_deps" , self . base)) } # [doc = "Get a reference to the value of field `source` after provisioning.\n"] pub fn source (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.source" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl
{
    name: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl { }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl
{
    #[doc = "Required. Package name."]
    pub name: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl { name : self . name , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\nRequired. Package name."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl
{
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation: Option<PrimField<String>>,
    object: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl { # [doc = "Set the field `generation`.\nGeneration number of the Cloud Storage object."] pub fn set_generation (mut self , v : impl Into < PrimField < String > >) -> Self { self . generation = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl
{
    #[doc = "Required. Bucket of the Cloud Storage object."]
    pub bucket: PrimField<String>,
    #[doc = "Required. Name of the Cloud Storage object."]
    pub object: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl { bucket : self . bucket , generation : core :: default :: Default :: default () , object : self . object , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bucket` after provisioning.\nRequired. Bucket of the Cloud Storage object."] pub fn bucket (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bucket" , self . base)) } # [doc = "Get a reference to the value of field `generation` after provisioning.\nGeneration number of the Cloud Storage object."] pub fn generation (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.generation" , self . base)) } # [doc = "Get a reference to the value of field `object` after provisioning.\nRequired. Name of the Cloud Storage object."] pub fn object (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.object" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256_checksum: Option<PrimField<String>>,
    uri: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl { # [doc = "Set the field `sha256_checksum`.\nSHA256 checksum of the remote file."] pub fn set_sha256_checksum (mut self , v : impl Into < PrimField < String > >) -> Self { self . sha256_checksum = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl
{
    #[doc = "Required. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."]
    pub uri: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl { sha256_checksum : core :: default :: Default :: default () , uri : self . uri , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `sha256_checksum` after provisioning.\nSHA256 checksum of the remote file."] pub fn sha256_checksum (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.sha256_checksum" , self . base)) } # [doc = "Get a reference to the value of field `uri` after provisioning.\nRequired. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."] pub fn uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.uri" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElDynamic { gcs : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl >> , remote : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl { # [serde (skip_serializing_if = "Option::is_none")] allow_insecure : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] local_path : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] gcs : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl > > , # [serde (skip_serializing_if = "Option::is_none")] remote : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl > > , dynamic : OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElDynamic , }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl { # [doc = "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."] pub fn set_allow_insecure (mut self , v : impl Into < PrimField < bool > >) -> Self { self . allow_insecure = Some (v . into ()) ; self } # [doc = "Set the field `local_path`.\nA local path within the VM to use."] pub fn set_local_path (mut self , v : impl Into < PrimField < String > >) -> Self { self . local_path = Some (v . into ()) ; self } # [doc = "Set the field `gcs`.\n"] pub fn set_gcs (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . gcs = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . gcs = Some (d) ; } } self } # [doc = "Set the field `remote`.\n"] pub fn set_remote (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . remote = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . remote = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl
{}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl { allow_insecure : core :: default :: Default :: default () , local_path : core :: default :: Default :: default () , gcs : core :: default :: Default :: default () , remote : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."] pub fn allow_insecure (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_insecure" , self . base)) } # [doc = "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."] pub fn local_path (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.local_path" , self . base)) } # [doc = "Get a reference to the value of field `gcs` after provisioning.\n"] pub fn gcs (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.gcs" , self . base)) } # [doc = "Get a reference to the value of field `remote` after provisioning.\n"] pub fn remote (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.remote" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElDynamic { source : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl { # [serde (skip_serializing_if = "Option::is_none")] properties : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] source : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl > > , dynamic : OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElDynamic , }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl { # [doc = "Set the field `properties`.\nAdditional properties to use during installation.\nThis should be in the format of Property=Setting.\nAppended to the defaults of 'ACTION=INSTALL\nREBOOT=ReallySuppress'."] pub fn set_properties (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . properties = Some (v . into ()) ; self } # [doc = "Set the field `source`.\n"] pub fn set_source (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . source = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . source = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl
{}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl { properties : core :: default :: Default :: default () , source : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `properties` after provisioning.\nAdditional properties to use during installation.\nThis should be in the format of Property=Setting.\nAppended to the defaults of 'ACTION=INSTALL\nREBOOT=ReallySuppress'."] pub fn properties (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.properties" , self . base)) } # [doc = "Get a reference to the value of field `source` after provisioning.\n"] pub fn source (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.source" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl
{
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation: Option<PrimField<String>>,
    object: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl { # [doc = "Set the field `generation`.\nGeneration number of the Cloud Storage object."] pub fn set_generation (mut self , v : impl Into < PrimField < String > >) -> Self { self . generation = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl
{
    #[doc = "Required. Bucket of the Cloud Storage object."]
    pub bucket: PrimField<String>,
    #[doc = "Required. Name of the Cloud Storage object."]
    pub object: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl { bucket : self . bucket , generation : core :: default :: Default :: default () , object : self . object , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bucket` after provisioning.\nRequired. Bucket of the Cloud Storage object."] pub fn bucket (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bucket" , self . base)) } # [doc = "Get a reference to the value of field `generation` after provisioning.\nGeneration number of the Cloud Storage object."] pub fn generation (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.generation" , self . base)) } # [doc = "Get a reference to the value of field `object` after provisioning.\nRequired. Name of the Cloud Storage object."] pub fn object (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.object" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256_checksum: Option<PrimField<String>>,
    uri: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl { # [doc = "Set the field `sha256_checksum`.\nSHA256 checksum of the remote file."] pub fn set_sha256_checksum (mut self , v : impl Into < PrimField < String > >) -> Self { self . sha256_checksum = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl
{
    #[doc = "Required. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."]
    pub uri: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl { sha256_checksum : core :: default :: Default :: default () , uri : self . uri , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `sha256_checksum` after provisioning.\nSHA256 checksum of the remote file."] pub fn sha256_checksum (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.sha256_checksum" , self . base)) } # [doc = "Get a reference to the value of field `uri` after provisioning.\nRequired. URI from which to fetch the object. It should contain both the\nprotocol and path following the format '{protocol}://{location}'."] pub fn uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.uri" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElDynamic { gcs : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl >> , remote : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl { # [serde (skip_serializing_if = "Option::is_none")] allow_insecure : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] local_path : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] gcs : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl > > , # [serde (skip_serializing_if = "Option::is_none")] remote : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl > > , dynamic : OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElDynamic , }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl { # [doc = "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."] pub fn set_allow_insecure (mut self , v : impl Into < PrimField < bool > >) -> Self { self . allow_insecure = Some (v . into ()) ; self } # [doc = "Set the field `local_path`.\nA local path within the VM to use."] pub fn set_local_path (mut self , v : impl Into < PrimField < String > >) -> Self { self . local_path = Some (v . into ()) ; self } # [doc = "Set the field `gcs`.\n"] pub fn set_gcs (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . gcs = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . gcs = Some (d) ; } } self } # [doc = "Set the field `remote`.\n"] pub fn set_remote (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . remote = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . remote = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl
{}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl { allow_insecure : core :: default :: Default :: default () , local_path : core :: default :: Default :: default () , gcs : core :: default :: Default :: default () , remote : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations\nbased on the file type:\n\nRemote: A checksum must be specified.\nCloud Storage: An object generation number must be specified."] pub fn allow_insecure (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_insecure" , self . base)) } # [doc = "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."] pub fn local_path (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.local_path" , self . base)) } # [doc = "Get a reference to the value of field `gcs` after provisioning.\n"] pub fn gcs (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.gcs" , self . base)) } # [doc = "Get a reference to the value of field `remote` after provisioning.\n"] pub fn remote (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.remote" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElDynamic { source : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl { # [serde (skip_serializing_if = "Option::is_none")] pull_deps : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] source : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl > > , dynamic : OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElDynamic , }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl { # [doc = "Set the field `pull_deps`.\nWhether dependencies should also be installed.\n- install when false: 'rpm --upgrade --replacepkgs package.rpm'\n- install when true: 'yum -y install package.rpm' or\n'zypper -y install package.rpm'"] pub fn set_pull_deps (mut self , v : impl Into < PrimField < bool > >) -> Self { self . pull_deps = Some (v . into ()) ; self } # [doc = "Set the field `source`.\n"] pub fn set_source (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . source = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . source = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl
{}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl { pull_deps : core :: default :: Default :: default () , source : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `pull_deps` after provisioning.\nWhether dependencies should also be installed.\n- install when false: 'rpm --upgrade --replacepkgs package.rpm'\n- install when true: 'yum -y install package.rpm' or\n'zypper -y install package.rpm'"] pub fn pull_deps (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pull_deps" , self . base)) } # [doc = "Get a reference to the value of field `source` after provisioning.\n"] pub fn source (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.source" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumEl
{
    name: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumEl { }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumEl
{
    #[doc = "Required. Package name."]
    pub name: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumEl { name : self . name , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\nRequired. Package name."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl
{
    name: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl { }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl
{
    #[doc = "Required. Package name."]
    pub name: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl { name : self . name , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\nRequired. Package name."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDynamic { apt : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptEl >> , deb : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebEl >> , googet : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl >> , msi : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl >> , rpm : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl >> , yum : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumEl >> , zypper : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgEl { desired_state : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] apt : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptEl > > , # [serde (skip_serializing_if = "Option::is_none")] deb : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebEl > > , # [serde (skip_serializing_if = "Option::is_none")] googet : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl > > , # [serde (skip_serializing_if = "Option::is_none")] msi : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl > > , # [serde (skip_serializing_if = "Option::is_none")] rpm : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl > > , # [serde (skip_serializing_if = "Option::is_none")] yum : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumEl > > , # [serde (skip_serializing_if = "Option::is_none")] zypper : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl > > , dynamic : OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDynamic , }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgEl { # [doc = "Set the field `apt`.\n"] pub fn set_apt (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . apt = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . apt = Some (d) ; } } self } # [doc = "Set the field `deb`.\n"] pub fn set_deb (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . deb = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . deb = Some (d) ; } } self } # [doc = "Set the field `googet`.\n"] pub fn set_googet (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . googet = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . googet = Some (d) ; } } self } # [doc = "Set the field `msi`.\n"] pub fn set_msi (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . msi = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . msi = Some (d) ; } } self } # [doc = "Set the field `rpm`.\n"] pub fn set_rpm (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . rpm = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . rpm = Some (d) ; } } self } # [doc = "Set the field `yum`.\n"] pub fn set_yum (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . yum = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . yum = Some (d) ; } } self } # [doc = "Set the field `zypper`.\n"] pub fn set_zypper (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . zypper = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . zypper = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgEl
{
    #[doc = "Required. The desired state the agent should maintain for this package.\nPossible values:\nDESIRED_STATE_UNSPECIFIED\nINSTALLED\nREMOVED"]
    pub desired_state: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgEl { desired_state : self . desired_state , apt : core :: default :: Default :: default () , deb : core :: default :: Default :: default () , googet : core :: default :: Default :: default () , msi : core :: default :: Default :: default () , rpm : core :: default :: Default :: default () , yum : core :: default :: Default :: default () , zypper : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `desired_state` after provisioning.\nRequired. The desired state the agent should maintain for this package.\nPossible values:\nDESIRED_STATE_UNSPECIFIED\nINSTALLED\nREMOVED"] pub fn desired_state (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.desired_state" , self . base)) } # [doc = "Get a reference to the value of field `apt` after provisioning.\n"] pub fn apt (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElAptElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.apt" , self . base)) } # [doc = "Get a reference to the value of field `deb` after provisioning.\n"] pub fn deb (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElDebElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.deb" , self . base)) } # [doc = "Get a reference to the value of field `googet` after provisioning.\n"] pub fn googet (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElGoogetElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.googet" , self . base)) } # [doc = "Get a reference to the value of field `msi` after provisioning.\n"] pub fn msi (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElMsiElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.msi" , self . base)) } # [doc = "Get a reference to the value of field `rpm` after provisioning.\n"] pub fn rpm (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRpmElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.rpm" , self . base)) } # [doc = "Get a reference to the value of field `yum` after provisioning.\n"] pub fn yum (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElYumElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.yum" , self . base)) } # [doc = "Get a reference to the value of field `zypper` after provisioning.\n"] pub fn zypper (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElZypperElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.zypper" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl
{
    archive_type: PrimField<String>,
    components: ListField<PrimField<String>>,
    distribution: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpg_key: Option<PrimField<String>>,
    uri: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl { # [doc = "Set the field `gpg_key`.\nURI of the key file for this repository. The agent maintains a\nkeyring at '/etc/apt/trusted.gpg.d/osconfig_agent_managed.gpg'."] pub fn set_gpg_key (mut self , v : impl Into < PrimField < String > >) -> Self { self . gpg_key = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl
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
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl { archive_type : self . archive_type , components : self . components , distribution : self . distribution , gpg_key : core :: default :: Default :: default () , uri : self . uri , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `archive_type` after provisioning.\nRequired. Type of archive files in this repository.\nPossible values:\nARCHIVE_TYPE_UNSPECIFIED\nDEB\nDEB_SRC"] pub fn archive_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.archive_type" , self . base)) } # [doc = "Get a reference to the value of field `components` after provisioning.\nRequired. List of components for this repository. Must contain at least one\nitem."] pub fn components (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.components" , self . base)) } # [doc = "Get a reference to the value of field `distribution` after provisioning.\nRequired. Distribution of this repository."] pub fn distribution (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.distribution" , self . base)) } # [doc = "Get a reference to the value of field `gpg_key` after provisioning.\nURI of the key file for this repository. The agent maintains a\nkeyring at '/etc/apt/trusted.gpg.d/osconfig_agent_managed.gpg'."] pub fn gpg_key (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.gpg_key" , self . base)) } # [doc = "Get a reference to the value of field `uri` after provisioning.\nRequired. URI for this repository."] pub fn uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.uri" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl
{
    name: PrimField<String>,
    url: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl { }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl
{
    #[doc = "Required. The name of the repository."]
    pub name: PrimField<String>,
    #[doc = "Required. The url of the repository."]
    pub url: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl { name : self . name , url : self . url , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\nRequired. The name of the repository."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `url` after provisioning.\nRequired. The url of the repository."] pub fn url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.url" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl
{
    base_url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpg_keys: Option<ListField<PrimField<String>>>,
    id: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl { # [doc = "Set the field `display_name`.\nThe display name of the repository."] pub fn set_display_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . display_name = Some (v . into ()) ; self } # [doc = "Set the field `gpg_keys`.\nURIs of GPG keys."] pub fn set_gpg_keys (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . gpg_keys = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl
{
    #[doc = "Required. The location of the repository directory."]
    pub base_url: PrimField<String>,
    #[doc = "Required. A one word, unique name for this repository. This is  the 'repo\nid' in the yum config file and also the 'display_name' if\n'display_name' is omitted. This id is also used as the unique\nidentifier when checking for resource conflicts."]
    pub id: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl { base_url : self . base_url , display_name : core :: default :: Default :: default () , gpg_keys : core :: default :: Default :: default () , id : self . id , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `base_url` after provisioning.\nRequired. The location of the repository directory."] pub fn base_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.base_url" , self . base)) } # [doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the repository."] pub fn display_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.display_name" , self . base)) } # [doc = "Get a reference to the value of field `gpg_keys` after provisioning.\nURIs of GPG keys."] pub fn gpg_keys (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.gpg_keys" , self . base)) } # [doc = "Get a reference to the value of field `id` after provisioning.\nRequired. A one word, unique name for this repository. This is  the 'repo\nid' in the yum config file and also the 'display_name' if\n'display_name' is omitted. This id is also used as the unique\nidentifier when checking for resource conflicts."] pub fn id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.id" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl
{
    base_url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpg_keys: Option<ListField<PrimField<String>>>,
    id: PrimField<String>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl { # [doc = "Set the field `display_name`.\nThe display name of the repository."] pub fn set_display_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . display_name = Some (v . into ()) ; self } # [doc = "Set the field `gpg_keys`.\nURIs of GPG keys."] pub fn set_gpg_keys (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . gpg_keys = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl
{
    #[doc = "Required. The location of the repository directory."]
    pub base_url: PrimField<String>,
    #[doc = "Required. A one word, unique name for this repository. This is the 'repo\nid' in the zypper config file and also the 'display_name' if\n'display_name' is omitted. This id is also used as the unique\nidentifier when checking for GuestPolicy conflicts."]
    pub id: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl { base_url : self . base_url , display_name : core :: default :: Default :: default () , gpg_keys : core :: default :: Default :: default () , id : self . id , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `base_url` after provisioning.\nRequired. The location of the repository directory."] pub fn base_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.base_url" , self . base)) } # [doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the repository."] pub fn display_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.display_name" , self . base)) } # [doc = "Get a reference to the value of field `gpg_keys` after provisioning.\nURIs of GPG keys."] pub fn gpg_keys (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.gpg_keys" , self . base)) } # [doc = "Get a reference to the value of field `id` after provisioning.\nRequired. A one word, unique name for this repository. This is the 'repo\nid' in the zypper config file and also the 'display_name' if\n'display_name' is omitted. This id is also used as the unique\nidentifier when checking for GuestPolicy conflicts."] pub fn id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.id" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElDynamic { apt : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl >> , goo : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl >> , yum : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl >> , zypper : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryEl { # [serde (skip_serializing_if = "Option::is_none")] apt : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl > > , # [serde (skip_serializing_if = "Option::is_none")] goo : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl > > , # [serde (skip_serializing_if = "Option::is_none")] yum : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl > > , # [serde (skip_serializing_if = "Option::is_none")] zypper : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl > > , dynamic : OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElDynamic , }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryEl { # [doc = "Set the field `apt`.\n"] pub fn set_apt (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . apt = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . apt = Some (d) ; } } self } # [doc = "Set the field `goo`.\n"] pub fn set_goo (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . goo = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . goo = Some (d) ; } } self } # [doc = "Set the field `yum`.\n"] pub fn set_yum (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . yum = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . yum = Some (d) ; } } self } # [doc = "Set the field `zypper`.\n"] pub fn set_zypper (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . zypper = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . zypper = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryEl
{}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryEl { apt : core :: default :: Default :: default () , goo : core :: default :: Default :: default () , yum : core :: default :: Default :: default () , zypper : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `apt` after provisioning.\n"] pub fn apt (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElAptElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.apt" , self . base)) } # [doc = "Get a reference to the value of field `goo` after provisioning.\n"] pub fn goo (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElGooElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.goo" , self . base)) } # [doc = "Get a reference to the value of field `yum` after provisioning.\n"] pub fn yum (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElYumElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.yum" , self . base)) } # [doc = "Get a reference to the value of field `zypper` after provisioning.\n"] pub fn zypper (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.zypper" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElDynamic { exec : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecEl >> , file : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileEl >> , pkg : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgEl >> , repository : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesEl { id : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] exec : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecEl > > , # [serde (skip_serializing_if = "Option::is_none")] file : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileEl > > , # [serde (skip_serializing_if = "Option::is_none")] pkg : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgEl > > , # [serde (skip_serializing_if = "Option::is_none")] repository : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryEl > > , dynamic : OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElDynamic , }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesEl { # [doc = "Set the field `exec`.\n"] pub fn set_exec (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . exec = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . exec = Some (d) ; } } self } # [doc = "Set the field `file`.\n"] pub fn set_file (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . file = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . file = Some (d) ; } } self } # [doc = "Set the field `pkg`.\n"] pub fn set_pkg (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . pkg = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . pkg = Some (d) ; } } self } # [doc = "Set the field `repository`.\n"] pub fn set_repository (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . repository = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . repository = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesEl
{
    #[doc = "Required. The id of the resource with the following restrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the OS policy."]
    pub id: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesEl { id : self . id , exec : core :: default :: Default :: default () , file : core :: default :: Default :: default () , pkg : core :: default :: Default :: default () , repository : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `id` after provisioning.\nRequired. The id of the resource with the following restrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the OS policy."] pub fn id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.id" , self . base)) } # [doc = "Get a reference to the value of field `exec` after provisioning.\n"] pub fn exec (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElExecElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.exec" , self . base)) } # [doc = "Get a reference to the value of field `file` after provisioning.\n"] pub fn file (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElFileElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.file" , self . base)) } # [doc = "Get a reference to the value of field `pkg` after provisioning.\n"] pub fn pkg (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElPkgElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.pkg" , self . base)) } # [doc = "Get a reference to the value of field `repository` after provisioning.\n"] pub fn repository (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRepositoryElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.repository" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElDynamic { inventory_filters : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersEl >> , resources : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsEl { # [serde (skip_serializing_if = "Option::is_none")] inventory_filters : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersEl > > , # [serde (skip_serializing_if = "Option::is_none")] resources : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesEl > > , dynamic : OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElDynamic , }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsEl { # [doc = "Set the field `inventory_filters`.\n"] pub fn set_inventory_filters (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . inventory_filters = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . inventory_filters = Some (d) ; } } self } # [doc = "Set the field `resources`.\n"] pub fn set_resources (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . resources = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . resources = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsEl
{}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsEl { inventory_filters : core :: default :: Default :: default () , resources : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `inventory_filters` after provisioning.\n"] pub fn inventory_filters (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElInventoryFiltersElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.inventory_filters" , self . base)) } # [doc = "Get a reference to the value of field `resources` after provisioning.\n"] pub fn resources (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElResourcesElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.resources" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElDynamic { resource_groups : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesEl { # [serde (skip_serializing_if = "Option::is_none")] allow_no_resource_group_match : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] description : Option < PrimField < String > > , id : PrimField < String > , mode : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] resource_groups : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsEl > > , dynamic : OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElDynamic , }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesEl { # [doc = "Set the field `allow_no_resource_group_match`.\nThis flag determines the OS policy compliance status when none of the\nresource groups within the policy are applicable for a VM. Set this value\nto 'true' if the policy needs to be reported as compliant even if the\npolicy has nothing to validate or enforce."] pub fn set_allow_no_resource_group_match (mut self , v : impl Into < PrimField < bool > >) -> Self { self . allow_no_resource_group_match = Some (v . into ()) ; self } # [doc = "Set the field `description`.\nPolicy description.\nLength of the description is limited to 1024 characters."] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `resource_groups`.\n"] pub fn set_resource_groups (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . resource_groups = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . resource_groups = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesEl
{
    #[doc = "Required. The id of the OS policy with the following restrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the assignment."]
    pub id: PrimField<String>,
    #[doc = "Required. Policy mode\nPossible values:\nMODE_UNSPECIFIED\nVALIDATION\nENFORCEMENT"]
    pub mode: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesEl { allow_no_resource_group_match : core :: default :: Default :: default () , description : core :: default :: Default :: default () , id : self . id , mode : self . mode , resource_groups : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_no_resource_group_match` after provisioning.\nThis flag determines the OS policy compliance status when none of the\nresource groups within the policy are applicable for a VM. Set this value\nto 'true' if the policy needs to be reported as compliant even if the\npolicy has nothing to validate or enforce."] pub fn allow_no_resource_group_match (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_no_resource_group_match" , self . base)) } # [doc = "Get a reference to the value of field `description` after provisioning.\nPolicy description.\nLength of the description is limited to 1024 characters."] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `id` after provisioning.\nRequired. The id of the OS policy with the following restrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the assignment."] pub fn id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.id" , self . base)) } # [doc = "Get a reference to the value of field `mode` after provisioning.\nRequired. Policy mode\nPossible values:\nMODE_UNSPECIFIED\nVALIDATION\nENFORCEMENT"] pub fn mode (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.mode" , self . base)) } # [doc = "Get a reference to the value of field `resource_groups` after provisioning.\n"] pub fn resource_groups (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElResourceGroupsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.resource_groups" , self . base)) } }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percent: Option<PrimField<f64>>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetEl { # [doc = "Set the field `fixed`.\nSpecifies a fixed value."] pub fn set_fixed (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . fixed = Some (v . into ()) ; self } # [doc = "Set the field `percent`.\nSpecifies the relative value defined as a percentage, which will be\nmultiplied by a reference value."] pub fn set_percent (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . percent = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetEl
{}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetEl { fixed : core :: default :: Default :: default () , percent : core :: default :: Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `fixed` after provisioning.\nSpecifies a fixed value."] pub fn fixed (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.fixed" , self . base)) } # [doc = "Get a reference to the value of field `percent` after provisioning.\nSpecifies the relative value defined as a percentage, which will be\nmultiplied by a reference value."] pub fn percent (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.percent" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDynamic { disruption_budget : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutEl { min_wait_duration : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] disruption_budget : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetEl > > , dynamic : OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDynamic , }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutEl { # [doc = "Set the field `disruption_budget`.\n"] pub fn set_disruption_budget (mut self , v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . disruption_budget = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . disruption_budget = Some (d) ; } } self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutEl
{
    #[doc = "Required. This determines the minimum duration of time to wait after the\nconfiguration changes are applied through the current rollout. A\nVM continues to count towards the 'disruption_budget' at least\nuntil this duration of time has passed after configuration changes are\napplied."]
    pub min_wait_duration: PrimField<String>,
}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutEl { min_wait_duration : self . min_wait_duration , disruption_budget : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `min_wait_duration` after provisioning.\nRequired. This determines the minimum duration of time to wait after the\nconfiguration changes are applied through the current rollout. A\nVM continues to count towards the 'disruption_budget' at least\nuntil this duration of time has passed after configuration changes are\napplied."] pub fn min_wait_duration (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.min_wait_duration" , self . base)) } # [doc = "Get a reference to the value of field `disruption_budget` after provisioning.\n"] pub fn disruption_budget (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElDisruptionBudgetElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.disruption_budget" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElDynamic { instance_filter : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterEl >> , os_policies : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesEl >> , rollout : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadEl { # [serde (skip_serializing_if = "Option::is_none")] description : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] etag : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] instance_filter : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterEl > > , # [serde (skip_serializing_if = "Option::is_none")] os_policies : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesEl > > , # [serde (skip_serializing_if = "Option::is_none")] rollout : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutEl > > , dynamic : OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElDynamic , }
impl
    OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadEl
{
    #[doc = "Set the field `description`.\nOS policy assignment description.\nLength of the description is limited to 1024 characters."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `etag`.\nThe etag for this OS policy assignment.\nIf this is provided on update, it must match the server's etag."]
    pub fn set_etag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.etag = Some(v.into());
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
        v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterEl >>,
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
        v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesEl >>,
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
        v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutEl >>,
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
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadEl
{}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadEl { description : core :: default :: Default :: default () , etag : core :: default :: Default :: default () , name : core :: default :: Default :: default () , instance_filter : core :: default :: Default :: default () , os_policies : core :: default :: Default :: default () , rollout : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `baseline` after provisioning.\nOutput only. Indicates that this revision has been successfully rolled out in this zone\nand new VMs will be assigned OS policies from this revision.\n\nFor a given OS policy assignment, there is only one revision with a value\nof 'true' for this field."] pub fn baseline (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.baseline" , self . base)) } # [doc = "Get a reference to the value of field `deleted` after provisioning.\nOutput only. Indicates that this revision deletes the OS policy assignment."] pub fn deleted (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.deleted" , self . base)) } # [doc = "Get a reference to the value of field `description` after provisioning.\nOS policy assignment description.\nLength of the description is limited to 1024 characters."] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `etag` after provisioning.\nThe etag for this OS policy assignment.\nIf this is provided on update, it must match the server's etag."] pub fn etag (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.etag" , self . base)) } # [doc = "Get a reference to the value of field `name` after provisioning.\nResource name.\n\nFormat:\n'projects/{project_number}/locations/{location}/osPolicyAssignments/{os_policy_assignment_id}'\n\nThis field is ignored when you create an OS policy assignment."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. Indicates that reconciliation is in progress for the revision.\nThis value is 'true' when the 'rollout_state' is one of:\n* IN_PROGRESS\n* CANCELLING"] pub fn reconciling (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.reconciling" , self . base)) } # [doc = "Get a reference to the value of field `revision_create_time` after provisioning.\nOutput only. The timestamp that the revision was created."] pub fn revision_create_time (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.revision_create_time" , self . base)) } # [doc = "Get a reference to the value of field `revision_id` after provisioning.\nOutput only. The assignment revision ID\nA new revision is committed whenever a rollout is triggered for a OS policy\nassignment"] pub fn revision_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.revision_id" , self . base)) } # [doc = "Get a reference to the value of field `rollout_state` after provisioning.\nOutput only. OS policy assignment rollout state\nPossible values:\nROLLOUT_STATE_UNSPECIFIED\nIN_PROGRESS\nCANCELLING\nCANCELLED\nSUCCEEDED"] pub fn rollout_state (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.rollout_state" , self . base)) } # [doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. Server generated unique id for the OS policy assignment resource."] pub fn uid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.uid" , self . base)) } # [doc = "Get a reference to the value of field `instance_filter` after provisioning.\n"] pub fn instance_filter (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElInstanceFilterElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.instance_filter" , self . base)) } # [doc = "Get a reference to the value of field `os_policies` after provisioning.\n"] pub fn os_policies (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElOsPoliciesElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.os_policies" , self . base)) } # [doc = "Get a reference to the value of field `rollout` after provisioning.\n"] pub fn rollout (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRolloutElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.rollout" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElDynamic { os_policy_assignment_v1_payload : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceEl { # [serde (skip_serializing_if = "Option::is_none")] id : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] os_policy_assignment_v1_payload : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadEl > > , dynamic : OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElDynamic , }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceEl {
    #[doc = "Set the field `id`.\nOptional. ID of the resource to be used while generating set of affected resources.\n\nFor UPSERT action the value is auto-generated during PolicyOrchestrator\ncreation when not set. When the value is set it should following next\nrestrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the project.\n\nFor DELETE action, ID must be specified explicitly during\nPolicyOrchestrator creation."]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `os_policy_assignment_v1_payload`.\n"]
    pub fn set_os_policy_assignment_v1_payload(
        mut self,
        v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadEl >>,
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
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceEl {
    type O = BlockAssignable<OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceEl {}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceEl {
    pub fn build(self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceEl {
        OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceEl {
            id: core::default::Default::default(),
            os_policy_assignment_v1_payload: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElRef {
        OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nOptional. ID of the resource to be used while generating set of affected resources.\n\nFor UPSERT action the value is auto-generated during PolicyOrchestrator\ncreation when not set. When the value is set it should following next\nrestrictions:\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the project.\n\nFor DELETE action, ID must be specified explicitly during\nPolicyOrchestrator creation."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `os_policy_assignment_v1_payload` after provisioning.\n"]    pub fn os_policy_assignment_v1_payload (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceElOsPolicyAssignmentV1PayloadElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.os_policy_assignment_v1_payload", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElLocationSelectorEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    included_locations: Option<ListField<PrimField<String>>>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElLocationSelectorEl {
    #[doc = "Set the field `included_locations`.\nOptional. Names of the locations in scope.\nFormat: 'us-central1-a'"]
    pub fn set_included_locations(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.included_locations = Some(v.into());
        self
    }
}
impl ToListMappable
    for OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElLocationSelectorEl
{
    type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElLocationSelectorEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElLocationSelectorEl
{}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElLocationSelectorEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElLocationSelectorEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElLocationSelectorEl { included_locations : core :: default :: Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElLocationSelectorElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElLocationSelectorElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElLocationSelectorElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElLocationSelectorElRef { shared : shared , base : base . to_string () , } } }
impl
    OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElLocationSelectorElRef
{
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
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElResourceHierarchySelectorEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    included_folders: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    included_projects: Option<ListField<PrimField<String>>>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElResourceHierarchySelectorEl { # [doc = "Set the field `included_folders`.\nOptional. Names of the folders in scope.\nFormat: 'folders/{folder_id}'"] pub fn set_included_folders (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . included_folders = Some (v . into ()) ; self } # [doc = "Set the field `included_projects`.\nOptional. Names of the projects in scope.\nFormat: 'projects/{project_number}'"] pub fn set_included_projects (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . included_projects = Some (v . into ()) ; self } }
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElResourceHierarchySelectorEl { type O = BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElResourceHierarchySelectorEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElResourceHierarchySelectorEl
{}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElResourceHierarchySelectorEl { pub fn build (self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElResourceHierarchySelectorEl { OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElResourceHierarchySelectorEl { included_folders : core :: default :: Default :: default () , included_projects : core :: default :: Default :: default () , } } }
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElResourceHierarchySelectorElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElResourceHierarchySelectorElRef { fn new (shared : StackShared , base : String) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElResourceHierarchySelectorElRef { OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElResourceHierarchySelectorElRef { shared : shared , base : base . to_string () , } } }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElResourceHierarchySelectorElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `included_folders` after provisioning.\nOptional. Names of the folders in scope.\nFormat: 'folders/{folder_id}'"] pub fn included_folders (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.included_folders" , self . base)) } # [doc = "Get a reference to the value of field `included_projects` after provisioning.\nOptional. Names of the projects in scope.\nFormat: 'projects/{project_number}'"] pub fn included_projects (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.included_projects" , self . base)) } }
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElDynamic { location_selector : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElLocationSelectorEl >> , resource_hierarchy_selector : Option < DynamicBlock < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElResourceHierarchySelectorEl >> , }
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsEl { # [serde (skip_serializing_if = "Option::is_none")] location_selector : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElLocationSelectorEl > > , # [serde (skip_serializing_if = "Option::is_none")] resource_hierarchy_selector : Option < Vec < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElResourceHierarchySelectorEl > > , dynamic : OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElDynamic , }
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsEl {
    #[doc = "Set the field `location_selector`.\n"]
    pub fn set_location_selector(
        mut self,
        v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElLocationSelectorEl >>,
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
        v : impl Into < BlockAssignable < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElResourceHierarchySelectorEl >>,
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
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsEl {
    type O =
        BlockAssignable<OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsEl {}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsEl {
    pub fn build(
        self,
    ) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsEl {
        OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsEl {
            location_selector: core::default::Default::default(),
            resource_hierarchy_selector: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElRef {
        OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `location_selector` after provisioning.\n"]    pub fn location_selector (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElLocationSelectorElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.location_selector", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_hierarchy_selector` after provisioning.\n"]    pub fn resource_hierarchy_selector (& self) -> ListRef < OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElResourceHierarchySelectorElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_hierarchy_selector", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElDynamic {
    selectors: Option<
        DynamicBlock<OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsEl>,
    >,
}
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    selectors:
        Option<Vec<OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsEl>>,
    dynamic: OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElDynamic,
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeEl {
    #[doc = "Set the field `selectors`.\n"]
    pub fn set_selectors(
        mut self,
        v: impl Into<
            BlockAssignable<
                OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsEl,
            >,
        >,
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
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeEl {
    type O = BlockAssignable<OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeEl {}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeEl {
    pub fn build(self) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeEl {
        OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeEl {
            selectors: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElRef {
        OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `selectors` after provisioning.\n"]
    pub fn selectors(
        &self,
    ) -> ListRef<OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeElSelectorsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.selectors", self.base))
    }
}
#[derive(Serialize)]
pub struct OsConfigV2PolicyOrchestratorForOrganizationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl OsConfigV2PolicyOrchestratorForOrganizationTimeoutsEl {
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
impl ToListMappable for OsConfigV2PolicyOrchestratorForOrganizationTimeoutsEl {
    type O = BlockAssignable<OsConfigV2PolicyOrchestratorForOrganizationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOsConfigV2PolicyOrchestratorForOrganizationTimeoutsEl {}
impl BuildOsConfigV2PolicyOrchestratorForOrganizationTimeoutsEl {
    pub fn build(self) -> OsConfigV2PolicyOrchestratorForOrganizationTimeoutsEl {
        OsConfigV2PolicyOrchestratorForOrganizationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct OsConfigV2PolicyOrchestratorForOrganizationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OsConfigV2PolicyOrchestratorForOrganizationTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OsConfigV2PolicyOrchestratorForOrganizationTimeoutsElRef {
        OsConfigV2PolicyOrchestratorForOrganizationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OsConfigV2PolicyOrchestratorForOrganizationTimeoutsElRef {
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
struct OsConfigV2PolicyOrchestratorForOrganizationDynamic {
    orchestrated_resource:
        Option<DynamicBlock<OsConfigV2PolicyOrchestratorForOrganizationOrchestratedResourceEl>>,
    orchestration_scope:
        Option<DynamicBlock<OsConfigV2PolicyOrchestratorForOrganizationOrchestrationScopeEl>>,
}
