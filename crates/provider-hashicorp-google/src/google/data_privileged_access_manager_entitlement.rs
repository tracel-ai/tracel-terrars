use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataPrivilegedAccessManagerEntitlementData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entitlement_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<PrimField<String>>,
}
struct DataPrivilegedAccessManagerEntitlement_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataPrivilegedAccessManagerEntitlementData>,
}
#[derive(Clone)]
pub struct DataPrivilegedAccessManagerEntitlement(Rc<DataPrivilegedAccessManagerEntitlement_>);
impl DataPrivilegedAccessManagerEntitlement {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `entitlement_id`.\nThe ID to use for this Entitlement. This will become the last part of the resource name.\nThis value should be 4-63 characters, and valid characters are \"[a-z]\", \"[0-9]\", and \"-\". The first character should be from [a-z].\nThis value should be unique among all other Entitlements under the specified 'parent'."]
    pub fn set_entitlement_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().entitlement_id = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe region of the Entitlement resource."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `parent`.\nFormat: projects/{project-id|project-number} or organizations/{organization-number} or folders/{folder-number}"]
    pub fn set_parent(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `additional_notification_targets` after provisioning.\nAdditionalNotificationTargets includes email addresses to be notified."]
    pub fn additional_notification_targets(
        &self,
    ) -> ListRef<DataPrivilegedAccessManagerEntitlementAdditionalNotificationTargetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_notification_targets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `approval_workflow` after provisioning.\nThe approvals needed before access will be granted to a requester.\nNo approvals will be needed if this field is null. Different types of approval workflows that can be used to gate privileged access granting."]
    pub fn approval_workflow(
        &self,
    ) -> ListRef<DataPrivilegedAccessManagerEntitlementApprovalWorkflowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.approval_workflow", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Create time stamp. A timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\""]
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
    #[doc = "Get a reference to the value of field `eligible_users` after provisioning.\nWho can create Grants using Entitlement. This list should contain at most one entry"]
    pub fn eligible_users(
        &self,
    ) -> ListRef<DataPrivilegedAccessManagerEntitlementEligibleUsersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.eligible_users", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entitlement_id` after provisioning.\nThe ID to use for this Entitlement. This will become the last part of the resource name.\nThis value should be 4-63 characters, and valid characters are \"[a-z]\", \"[0-9]\", and \"-\". The first character should be from [a-z].\nThis value should be unique among all other Entitlements under the specified 'parent'."]
    pub fn entitlement_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entitlement_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nFor Resource freshness validation (https://google.aip.dev/154)"]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe region of the Entitlement resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_request_duration` after provisioning.\nThe maximum amount of time for which access would be granted for a request.\nA requester can choose to ask for access for less than this duration but never more.\nFormat: calculate the time in seconds and concatenate it with 's' i.e. 2 hours = \"7200s\", 45 minutes = \"2700s\""]
    pub fn max_request_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_request_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nOutput Only. The entitlement's name follows a hierarchical structure, comprising the organization, folder, or project, alongside the region and a unique entitlement ID.\nFormats: organizations/{organization-number}/locations/{region}/entitlements/{entitlement-id}, folders/{folder-number}/locations/{region}/entitlements/{entitlement-id}, and projects/{project-id|project-number}/locations/{region}/entitlements/{entitlement-id}."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nFormat: projects/{project-id|project-number} or organizations/{organization-number} or folders/{folder-number}"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `privileged_access` after provisioning.\nPrivileged access that this service can be used to gate."]
    pub fn privileged_access(
        &self,
    ) -> ListRef<DataPrivilegedAccessManagerEntitlementPrivilegedAccessElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.privileged_access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `requester_justification_config` after provisioning.\nDefines the ways in which a requester should provide the justification while requesting for access."]
    pub fn requester_justification_config(
        &self,
    ) -> ListRef<DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.requester_justification_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The current state of the Entitlement."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Update time stamp. A timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
}
impl Referable for DataPrivilegedAccessManagerEntitlement {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataPrivilegedAccessManagerEntitlement {}
impl ToListMappable for DataPrivilegedAccessManagerEntitlement {
    type O = ListRef<DataPrivilegedAccessManagerEntitlementRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataPrivilegedAccessManagerEntitlement_ {
    fn extract_datasource_type(&self) -> String {
        "google_privileged_access_manager_entitlement".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataPrivilegedAccessManagerEntitlement {
    pub tf_id: String,
}
impl BuildDataPrivilegedAccessManagerEntitlement {
    pub fn build(self, stack: &mut Stack) -> DataPrivilegedAccessManagerEntitlement {
        let out = DataPrivilegedAccessManagerEntitlement(Rc::new(
            DataPrivilegedAccessManagerEntitlement_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataPrivilegedAccessManagerEntitlementData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    entitlement_id: core::default::Default::default(),
                    id: core::default::Default::default(),
                    location: core::default::Default::default(),
                    parent: core::default::Default::default(),
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataPrivilegedAccessManagerEntitlementRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPrivilegedAccessManagerEntitlementRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataPrivilegedAccessManagerEntitlementRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `additional_notification_targets` after provisioning.\nAdditionalNotificationTargets includes email addresses to be notified."]
    pub fn additional_notification_targets(
        &self,
    ) -> ListRef<DataPrivilegedAccessManagerEntitlementAdditionalNotificationTargetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_notification_targets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `approval_workflow` after provisioning.\nThe approvals needed before access will be granted to a requester.\nNo approvals will be needed if this field is null. Different types of approval workflows that can be used to gate privileged access granting."]
    pub fn approval_workflow(
        &self,
    ) -> ListRef<DataPrivilegedAccessManagerEntitlementApprovalWorkflowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.approval_workflow", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Create time stamp. A timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\""]
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
    #[doc = "Get a reference to the value of field `eligible_users` after provisioning.\nWho can create Grants using Entitlement. This list should contain at most one entry"]
    pub fn eligible_users(
        &self,
    ) -> ListRef<DataPrivilegedAccessManagerEntitlementEligibleUsersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.eligible_users", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entitlement_id` after provisioning.\nThe ID to use for this Entitlement. This will become the last part of the resource name.\nThis value should be 4-63 characters, and valid characters are \"[a-z]\", \"[0-9]\", and \"-\". The first character should be from [a-z].\nThis value should be unique among all other Entitlements under the specified 'parent'."]
    pub fn entitlement_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entitlement_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nFor Resource freshness validation (https://google.aip.dev/154)"]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe region of the Entitlement resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_request_duration` after provisioning.\nThe maximum amount of time for which access would be granted for a request.\nA requester can choose to ask for access for less than this duration but never more.\nFormat: calculate the time in seconds and concatenate it with 's' i.e. 2 hours = \"7200s\", 45 minutes = \"2700s\""]
    pub fn max_request_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_request_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nOutput Only. The entitlement's name follows a hierarchical structure, comprising the organization, folder, or project, alongside the region and a unique entitlement ID.\nFormats: organizations/{organization-number}/locations/{region}/entitlements/{entitlement-id}, folders/{folder-number}/locations/{region}/entitlements/{entitlement-id}, and projects/{project-id|project-number}/locations/{region}/entitlements/{entitlement-id}."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nFormat: projects/{project-id|project-number} or organizations/{organization-number} or folders/{folder-number}"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `privileged_access` after provisioning.\nPrivileged access that this service can be used to gate."]
    pub fn privileged_access(
        &self,
    ) -> ListRef<DataPrivilegedAccessManagerEntitlementPrivilegedAccessElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.privileged_access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `requester_justification_config` after provisioning.\nDefines the ways in which a requester should provide the justification while requesting for access."]
    pub fn requester_justification_config(
        &self,
    ) -> ListRef<DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.requester_justification_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The current state of the Entitlement."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Update time stamp. A timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataPrivilegedAccessManagerEntitlementAdditionalNotificationTargetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_email_recipients: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requester_email_recipients: Option<SetField<PrimField<String>>>,
}
impl DataPrivilegedAccessManagerEntitlementAdditionalNotificationTargetsEl {
    #[doc = "Set the field `admin_email_recipients`.\n"]
    pub fn set_admin_email_recipients(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.admin_email_recipients = Some(v.into());
        self
    }
    #[doc = "Set the field `requester_email_recipients`.\n"]
    pub fn set_requester_email_recipients(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.requester_email_recipients = Some(v.into());
        self
    }
}
impl ToListMappable for DataPrivilegedAccessManagerEntitlementAdditionalNotificationTargetsEl {
    type O = BlockAssignable<DataPrivilegedAccessManagerEntitlementAdditionalNotificationTargetsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPrivilegedAccessManagerEntitlementAdditionalNotificationTargetsEl {}
impl BuildDataPrivilegedAccessManagerEntitlementAdditionalNotificationTargetsEl {
    pub fn build(self) -> DataPrivilegedAccessManagerEntitlementAdditionalNotificationTargetsEl {
        DataPrivilegedAccessManagerEntitlementAdditionalNotificationTargetsEl {
            admin_email_recipients: core::default::Default::default(),
            requester_email_recipients: core::default::Default::default(),
        }
    }
}
pub struct DataPrivilegedAccessManagerEntitlementAdditionalNotificationTargetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPrivilegedAccessManagerEntitlementAdditionalNotificationTargetsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPrivilegedAccessManagerEntitlementAdditionalNotificationTargetsElRef {
        DataPrivilegedAccessManagerEntitlementAdditionalNotificationTargetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPrivilegedAccessManagerEntitlementAdditionalNotificationTargetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `admin_email_recipients` after provisioning.\n"]
    pub fn admin_email_recipients(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.admin_email_recipients", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `requester_email_recipients` after provisioning.\n"]
    pub fn requester_email_recipients(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.requester_email_recipients", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    principals: Option<SetField<PrimField<String>>>,
}
impl DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversEl {
    #[doc = "Set the field `principals`.\n"]
    pub fn set_principals(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.principals = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversEl
{
    type O = BlockAssignable<
        DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversEl
{}
impl
    BuildDataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversEl
{
    pub fn build(
        self,
    ) -> DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversEl
    {
        DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversEl { principals : core :: default :: Default :: default () , }
    }
}
pub struct DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversElRef { fn new (shared : StackShared , base : String) -> DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversElRef { DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversElRef { shared : shared , base : base . to_string () , } } }
impl
    DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `principals` after provisioning.\n"]
    pub fn principals(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.principals", self.base))
    }
}
#[derive(Serialize)]
pub struct DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsEl { # [serde (skip_serializing_if = "Option::is_none")] approvals_needed : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] approver_email_recipients : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] approvers : Option < ListField < DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversEl > > , }
impl DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsEl {
    #[doc = "Set the field `approvals_needed`.\n"]
    pub fn set_approvals_needed(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.approvals_needed = Some(v.into());
        self
    }
    #[doc = "Set the field `approver_email_recipients`.\n"]
    pub fn set_approver_email_recipients(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.approver_email_recipients = Some(v.into());
        self
    }
    #[doc = "Set the field `approvers`.\n"]
    pub fn set_approvers(
        mut self,
        v : impl Into < ListField < DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversEl > >,
    ) -> Self {
        self.approvers = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsEl
{
    type O = BlockAssignable<
        DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsEl {
}
impl BuildDataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsEl {
    pub fn build(
        self,
    ) -> DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsEl {
        DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsEl {
            approvals_needed: core::default::Default::default(),
            approver_email_recipients: core::default::Default::default(),
            approvers: core::default::Default::default(),
        }
    }
}
pub struct DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElRef {
        DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `approvals_needed` after provisioning.\n"]
    pub fn approvals_needed(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.approvals_needed", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `approver_email_recipients` after provisioning.\n"]
    pub fn approver_email_recipients(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.approver_email_recipients", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `approvers` after provisioning.\n"]    pub fn approvers (& self) -> ListRef < DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversElRef >{
        ListRef::new(self.shared().clone(), format!("{}.approvers", self.base))
    }
}
#[derive(Serialize)]
pub struct DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    require_approver_justification: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    steps: Option<
        ListField<DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsEl>,
    >,
}
impl DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsEl {
    #[doc = "Set the field `require_approver_justification`.\n"]
    pub fn set_require_approver_justification(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_approver_justification = Some(v.into());
        self
    }
    #[doc = "Set the field `steps`.\n"]
    pub fn set_steps(
        mut self,
        v: impl Into<
            ListField<
                DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsEl,
            >,
        >,
    ) -> Self {
        self.steps = Some(v.into());
        self
    }
}
impl ToListMappable for DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsEl {
    type O =
        BlockAssignable<DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsEl {}
impl BuildDataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsEl {
    pub fn build(
        self,
    ) -> DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsEl {
        DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsEl {
            require_approver_justification: core::default::Default::default(),
            steps: core::default::Default::default(),
        }
    }
}
pub struct DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElRef {
        DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `require_approver_justification` after provisioning.\n"]
    pub fn require_approver_justification(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_approver_justification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `steps` after provisioning.\n"]
    pub fn steps(
        &self,
    ) -> ListRef<DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.steps", self.base))
    }
}
#[derive(Serialize)]
pub struct DataPrivilegedAccessManagerEntitlementApprovalWorkflowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    manual_approvals: Option<
        ListField<DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsEl>,
    >,
}
impl DataPrivilegedAccessManagerEntitlementApprovalWorkflowEl {
    #[doc = "Set the field `manual_approvals`.\n"]
    pub fn set_manual_approvals(
        mut self,
        v: impl Into<
            ListField<DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsEl>,
        >,
    ) -> Self {
        self.manual_approvals = Some(v.into());
        self
    }
}
impl ToListMappable for DataPrivilegedAccessManagerEntitlementApprovalWorkflowEl {
    type O = BlockAssignable<DataPrivilegedAccessManagerEntitlementApprovalWorkflowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPrivilegedAccessManagerEntitlementApprovalWorkflowEl {}
impl BuildDataPrivilegedAccessManagerEntitlementApprovalWorkflowEl {
    pub fn build(self) -> DataPrivilegedAccessManagerEntitlementApprovalWorkflowEl {
        DataPrivilegedAccessManagerEntitlementApprovalWorkflowEl {
            manual_approvals: core::default::Default::default(),
        }
    }
}
pub struct DataPrivilegedAccessManagerEntitlementApprovalWorkflowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPrivilegedAccessManagerEntitlementApprovalWorkflowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPrivilegedAccessManagerEntitlementApprovalWorkflowElRef {
        DataPrivilegedAccessManagerEntitlementApprovalWorkflowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPrivilegedAccessManagerEntitlementApprovalWorkflowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `manual_approvals` after provisioning.\n"]
    pub fn manual_approvals(
        &self,
    ) -> ListRef<DataPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.manual_approvals", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataPrivilegedAccessManagerEntitlementEligibleUsersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    principals: Option<SetField<PrimField<String>>>,
}
impl DataPrivilegedAccessManagerEntitlementEligibleUsersEl {
    #[doc = "Set the field `principals`.\n"]
    pub fn set_principals(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.principals = Some(v.into());
        self
    }
}
impl ToListMappable for DataPrivilegedAccessManagerEntitlementEligibleUsersEl {
    type O = BlockAssignable<DataPrivilegedAccessManagerEntitlementEligibleUsersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPrivilegedAccessManagerEntitlementEligibleUsersEl {}
impl BuildDataPrivilegedAccessManagerEntitlementEligibleUsersEl {
    pub fn build(self) -> DataPrivilegedAccessManagerEntitlementEligibleUsersEl {
        DataPrivilegedAccessManagerEntitlementEligibleUsersEl {
            principals: core::default::Default::default(),
        }
    }
}
pub struct DataPrivilegedAccessManagerEntitlementEligibleUsersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPrivilegedAccessManagerEntitlementEligibleUsersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPrivilegedAccessManagerEntitlementEligibleUsersElRef {
        DataPrivilegedAccessManagerEntitlementEligibleUsersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPrivilegedAccessManagerEntitlementEligibleUsersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `principals` after provisioning.\n"]
    pub fn principals(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.principals", self.base))
    }
}
#[derive(Serialize)]
pub struct DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    condition_expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<PrimField<String>>,
}
impl DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsEl {
    #[doc = "Set the field `condition_expression`.\n"]
    pub fn set_condition_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.condition_expression = Some(v.into());
        self
    }
    #[doc = "Set the field `role`.\n"]
    pub fn set_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsEl
{
    type O = BlockAssignable<
        DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsEl
{}
impl BuildDataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsEl {
    pub fn build(
        self,
    ) -> DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsEl {
        DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsEl {
            condition_expression: core::default::Default::default(),
            role: core::default::Default::default(),
        }
    }
}
pub struct DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsElRef
    {
        DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `condition_expression` after provisioning.\n"]
    pub fn condition_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.condition_expression", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }
}
#[derive(Serialize)]
pub struct DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_bindings: Option<
        ListField<
            DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsEl,
        >,
    >,
}
impl DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessEl {
    #[doc = "Set the field `resource`.\n"]
    pub fn set_resource(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_type = Some(v.into());
        self
    }
    #[doc = "Set the field `role_bindings`.\n"]
    pub fn set_role_bindings(
        mut self,
        v : impl Into < ListField < DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsEl > >,
    ) -> Self {
        self.role_bindings = Some(v.into());
        self
    }
}
impl ToListMappable for DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessEl {
    type O =
        BlockAssignable<DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessEl {}
impl BuildDataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessEl {
    pub fn build(self) -> DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessEl {
        DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessEl {
            resource: core::default::Default::default(),
            resource_type: core::default::Default::default(),
            role_bindings: core::default::Default::default(),
        }
    }
}
pub struct DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRef {
        DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `role_bindings` after provisioning.\n"]
    pub fn role_bindings(
        &self,
    ) -> ListRef<
        DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.role_bindings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataPrivilegedAccessManagerEntitlementPrivilegedAccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_iam_access:
        Option<ListField<DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessEl>>,
}
impl DataPrivilegedAccessManagerEntitlementPrivilegedAccessEl {
    #[doc = "Set the field `gcp_iam_access`.\n"]
    pub fn set_gcp_iam_access(
        mut self,
        v: impl Into<ListField<DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessEl>>,
    ) -> Self {
        self.gcp_iam_access = Some(v.into());
        self
    }
}
impl ToListMappable for DataPrivilegedAccessManagerEntitlementPrivilegedAccessEl {
    type O = BlockAssignable<DataPrivilegedAccessManagerEntitlementPrivilegedAccessEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPrivilegedAccessManagerEntitlementPrivilegedAccessEl {}
impl BuildDataPrivilegedAccessManagerEntitlementPrivilegedAccessEl {
    pub fn build(self) -> DataPrivilegedAccessManagerEntitlementPrivilegedAccessEl {
        DataPrivilegedAccessManagerEntitlementPrivilegedAccessEl {
            gcp_iam_access: core::default::Default::default(),
        }
    }
}
pub struct DataPrivilegedAccessManagerEntitlementPrivilegedAccessElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPrivilegedAccessManagerEntitlementPrivilegedAccessElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPrivilegedAccessManagerEntitlementPrivilegedAccessElRef {
        DataPrivilegedAccessManagerEntitlementPrivilegedAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPrivilegedAccessManagerEntitlementPrivilegedAccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gcp_iam_access` after provisioning.\n"]
    pub fn gcp_iam_access(
        &self,
    ) -> ListRef<DataPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcp_iam_access", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryEl {}
impl DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryEl {}
impl ToListMappable
    for DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryEl
{
    type O = BlockAssignable<
        DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryEl
{}
impl BuildDataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryEl {
    pub fn build(
        self,
    ) -> DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryEl {
        DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryEl {}
    }
}
pub struct DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryElRef {
        DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredEl {}
impl DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredEl {}
impl ToListMappable
    for DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredEl
{
    type O = BlockAssignable<
        DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredEl
{}
impl BuildDataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredEl {
    pub fn build(
        self,
    ) -> DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredEl {
        DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredEl {}
    }
}
pub struct DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredElRef {
        DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    not_mandatory: Option<
        ListField<
            DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    unstructured: Option<
        ListField<
            DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredEl,
        >,
    >,
}
impl DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigEl {
    #[doc = "Set the field `not_mandatory`.\n"]
    pub fn set_not_mandatory(
        mut self,
        v: impl Into<
            ListField<
                DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryEl,
            >,
        >,
    ) -> Self {
        self.not_mandatory = Some(v.into());
        self
    }
    #[doc = "Set the field `unstructured`.\n"]
    pub fn set_unstructured(
        mut self,
        v: impl Into<
            ListField<
                DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredEl,
            >,
        >,
    ) -> Self {
        self.unstructured = Some(v.into());
        self
    }
}
impl ToListMappable for DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigEl {
    type O = BlockAssignable<DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPrivilegedAccessManagerEntitlementRequesterJustificationConfigEl {}
impl BuildDataPrivilegedAccessManagerEntitlementRequesterJustificationConfigEl {
    pub fn build(self) -> DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigEl {
        DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigEl {
            not_mandatory: core::default::Default::default(),
            unstructured: core::default::Default::default(),
        }
    }
}
pub struct DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElRef {
        DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `not_mandatory` after provisioning.\n"]
    pub fn not_mandatory(
        &self,
    ) -> ListRef<
        DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.not_mandatory", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `unstructured` after provisioning.\n"]
    pub fn unstructured(
        &self,
    ) -> ListRef<
        DataPrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.unstructured", self.base))
    }
}
