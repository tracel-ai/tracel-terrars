use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct PrivilegedAccessManagerEntitlementData {
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
    entitlement_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    max_request_duration: PrimField<String>,
    parent: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_notification_targets:
        Option<Vec<PrivilegedAccessManagerEntitlementAdditionalNotificationTargetsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    approval_workflow: Option<Vec<PrivilegedAccessManagerEntitlementApprovalWorkflowEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eligible_users: Option<Vec<PrivilegedAccessManagerEntitlementEligibleUsersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    privileged_access: Option<Vec<PrivilegedAccessManagerEntitlementPrivilegedAccessEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requester_justification_config:
        Option<Vec<PrivilegedAccessManagerEntitlementRequesterJustificationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<PrivilegedAccessManagerEntitlementTimeoutsEl>,
    dynamic: PrivilegedAccessManagerEntitlementDynamic,
}
struct PrivilegedAccessManagerEntitlement_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PrivilegedAccessManagerEntitlementData>,
}
#[derive(Clone)]
pub struct PrivilegedAccessManagerEntitlement(Rc<PrivilegedAccessManagerEntitlement_>);
impl PrivilegedAccessManagerEntitlement {
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
    #[doc = "Set the field `additional_notification_targets`.\n"]
    pub fn set_additional_notification_targets(
        self,
        v: impl Into<BlockAssignable<PrivilegedAccessManagerEntitlementAdditionalNotificationTargetsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().additional_notification_targets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .additional_notification_targets = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `approval_workflow`.\n"]
    pub fn set_approval_workflow(
        self,
        v: impl Into<BlockAssignable<PrivilegedAccessManagerEntitlementApprovalWorkflowEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().approval_workflow = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.approval_workflow = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `eligible_users`.\n"]
    pub fn set_eligible_users(
        self,
        v: impl Into<BlockAssignable<PrivilegedAccessManagerEntitlementEligibleUsersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().eligible_users = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.eligible_users = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `privileged_access`.\n"]
    pub fn set_privileged_access(
        self,
        v: impl Into<BlockAssignable<PrivilegedAccessManagerEntitlementPrivilegedAccessEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().privileged_access = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.privileged_access = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `requester_justification_config`.\n"]
    pub fn set_requester_justification_config(
        self,
        v: impl Into<BlockAssignable<PrivilegedAccessManagerEntitlementRequesterJustificationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().requester_justification_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .requester_justification_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<PrivilegedAccessManagerEntitlementTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
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
    #[doc = "Get a reference to the value of field `additional_notification_targets` after provisioning.\n"]
    pub fn additional_notification_targets(
        &self,
    ) -> ListRef<PrivilegedAccessManagerEntitlementAdditionalNotificationTargetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_notification_targets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `approval_workflow` after provisioning.\n"]
    pub fn approval_workflow(
        &self,
    ) -> ListRef<PrivilegedAccessManagerEntitlementApprovalWorkflowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.approval_workflow", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `eligible_users` after provisioning.\n"]
    pub fn eligible_users(&self) -> ListRef<PrivilegedAccessManagerEntitlementEligibleUsersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.eligible_users", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `privileged_access` after provisioning.\n"]
    pub fn privileged_access(
        &self,
    ) -> ListRef<PrivilegedAccessManagerEntitlementPrivilegedAccessElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.privileged_access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `requester_justification_config` after provisioning.\n"]
    pub fn requester_justification_config(
        &self,
    ) -> ListRef<PrivilegedAccessManagerEntitlementRequesterJustificationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.requester_justification_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrivilegedAccessManagerEntitlementTimeoutsElRef {
        PrivilegedAccessManagerEntitlementTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for PrivilegedAccessManagerEntitlement {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for PrivilegedAccessManagerEntitlement {}
impl ToListMappable for PrivilegedAccessManagerEntitlement {
    type O = ListRef<PrivilegedAccessManagerEntitlementRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for PrivilegedAccessManagerEntitlement_ {
    fn extract_resource_type(&self) -> String {
        "google_privileged_access_manager_entitlement".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildPrivilegedAccessManagerEntitlement {
    pub tf_id: String,
    #[doc = "The ID to use for this Entitlement. This will become the last part of the resource name.\nThis value should be 4-63 characters, and valid characters are \"[a-z]\", \"[0-9]\", and \"-\". The first character should be from [a-z].\nThis value should be unique among all other Entitlements under the specified 'parent'."]
    pub entitlement_id: PrimField<String>,
    #[doc = "The region of the Entitlement resource."]
    pub location: PrimField<String>,
    #[doc = "The maximum amount of time for which access would be granted for a request.\nA requester can choose to ask for access for less than this duration but never more.\nFormat: calculate the time in seconds and concatenate it with 's' i.e. 2 hours = \"7200s\", 45 minutes = \"2700s\""]
    pub max_request_duration: PrimField<String>,
    #[doc = "Format: projects/{project-id|project-number} or organizations/{organization-number} or folders/{folder-number}"]
    pub parent: PrimField<String>,
}
impl BuildPrivilegedAccessManagerEntitlement {
    pub fn build(self, stack: &mut Stack) -> PrivilegedAccessManagerEntitlement {
        let out =
            PrivilegedAccessManagerEntitlement(Rc::new(PrivilegedAccessManagerEntitlement_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(PrivilegedAccessManagerEntitlementData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    deletion_policy: core::default::Default::default(),
                    entitlement_id: self.entitlement_id,
                    id: core::default::Default::default(),
                    location: self.location,
                    max_request_duration: self.max_request_duration,
                    parent: self.parent,
                    additional_notification_targets: core::default::Default::default(),
                    approval_workflow: core::default::Default::default(),
                    eligible_users: core::default::Default::default(),
                    privileged_access: core::default::Default::default(),
                    requester_justification_config: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct PrivilegedAccessManagerEntitlementRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivilegedAccessManagerEntitlementRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl PrivilegedAccessManagerEntitlementRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `additional_notification_targets` after provisioning.\n"]
    pub fn additional_notification_targets(
        &self,
    ) -> ListRef<PrivilegedAccessManagerEntitlementAdditionalNotificationTargetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_notification_targets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `approval_workflow` after provisioning.\n"]
    pub fn approval_workflow(
        &self,
    ) -> ListRef<PrivilegedAccessManagerEntitlementApprovalWorkflowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.approval_workflow", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `eligible_users` after provisioning.\n"]
    pub fn eligible_users(&self) -> ListRef<PrivilegedAccessManagerEntitlementEligibleUsersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.eligible_users", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `privileged_access` after provisioning.\n"]
    pub fn privileged_access(
        &self,
    ) -> ListRef<PrivilegedAccessManagerEntitlementPrivilegedAccessElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.privileged_access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `requester_justification_config` after provisioning.\n"]
    pub fn requester_justification_config(
        &self,
    ) -> ListRef<PrivilegedAccessManagerEntitlementRequesterJustificationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.requester_justification_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrivilegedAccessManagerEntitlementTimeoutsElRef {
        PrivilegedAccessManagerEntitlementTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlementAdditionalNotificationTargetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_email_recipients: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requester_email_recipients: Option<SetField<PrimField<String>>>,
}
impl PrivilegedAccessManagerEntitlementAdditionalNotificationTargetsEl {
    #[doc = "Set the field `admin_email_recipients`.\nOptional. Additional email addresses to be notified when a principal(requester) is granted access."]
    pub fn set_admin_email_recipients(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.admin_email_recipients = Some(v.into());
        self
    }
    #[doc = "Set the field `requester_email_recipients`.\nOptional. Additional email address to be notified about an eligible entitlement."]
    pub fn set_requester_email_recipients(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.requester_email_recipients = Some(v.into());
        self
    }
}
impl ToListMappable for PrivilegedAccessManagerEntitlementAdditionalNotificationTargetsEl {
    type O = BlockAssignable<PrivilegedAccessManagerEntitlementAdditionalNotificationTargetsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivilegedAccessManagerEntitlementAdditionalNotificationTargetsEl {}
impl BuildPrivilegedAccessManagerEntitlementAdditionalNotificationTargetsEl {
    pub fn build(self) -> PrivilegedAccessManagerEntitlementAdditionalNotificationTargetsEl {
        PrivilegedAccessManagerEntitlementAdditionalNotificationTargetsEl {
            admin_email_recipients: core::default::Default::default(),
            requester_email_recipients: core::default::Default::default(),
        }
    }
}
pub struct PrivilegedAccessManagerEntitlementAdditionalNotificationTargetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivilegedAccessManagerEntitlementAdditionalNotificationTargetsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivilegedAccessManagerEntitlementAdditionalNotificationTargetsElRef {
        PrivilegedAccessManagerEntitlementAdditionalNotificationTargetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivilegedAccessManagerEntitlementAdditionalNotificationTargetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `admin_email_recipients` after provisioning.\nOptional. Additional email addresses to be notified when a principal(requester) is granted access."]
    pub fn admin_email_recipients(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.admin_email_recipients", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `requester_email_recipients` after provisioning.\nOptional. Additional email address to be notified about an eligible entitlement."]
    pub fn requester_email_recipients(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.requester_email_recipients", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversEl {
    principals: SetField<PrimField<String>>,
}
impl PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversEl {}
impl ToListMappable
    for PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversEl
{
    type O = BlockAssignable<
        PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversEl
{
    #[doc = "Users who are being allowed for the operation. Each entry should be a valid v1 IAM Principal Identifier. Format for these is documented at: https://cloud.google.com/iam/docs/principal-identifiers#v1"]
    pub principals: SetField<PrimField<String>>,
}
impl BuildPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversEl {
    pub fn build(
        self,
    ) -> PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversEl
    {
        PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversEl {
            principals: self.principals,
        }
    }
}
pub struct PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversElRef
    {
        PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `principals` after provisioning.\nUsers who are being allowed for the operation. Each entry should be a valid v1 IAM Principal Identifier. Format for these is documented at: https://cloud.google.com/iam/docs/principal-identifiers#v1"]
    pub fn principals(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.principals", self.base))
    }
}
#[derive(Serialize, Default)]
struct PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElDynamic {
    approvers: Option<
        DynamicBlock<
            PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    approvals_needed: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    approver_email_recipients: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    approvers: Option<
        Vec<
            PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversEl,
        >,
    >,
    dynamic: PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElDynamic,
}
impl PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsEl {
    #[doc = "Set the field `approvals_needed`.\nHow many users from the above list need to approve.\nIf there are not enough distinct users in the list above then the workflow\nwill indefinitely block. Should always be greater than 0. Currently 1 is the only\nsupported value."]
    pub fn set_approvals_needed(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.approvals_needed = Some(v.into());
        self
    }
    #[doc = "Set the field `approver_email_recipients`.\nOptional. Additional email addresses to be notified when a grant is pending approval."]
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
        v : impl Into < BlockAssignable < PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.approvers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.approvers = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsEl
{
    type O = BlockAssignable<
        PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsEl {}
impl BuildPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsEl {
    pub fn build(
        self,
    ) -> PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsEl {
        PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsEl {
            approvals_needed: core::default::Default::default(),
            approver_email_recipients: core::default::Default::default(),
            approvers: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElRef {
        PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `approvals_needed` after provisioning.\nHow many users from the above list need to approve.\nIf there are not enough distinct users in the list above then the workflow\nwill indefinitely block. Should always be greater than 0. Currently 1 is the only\nsupported value."]
    pub fn approvals_needed(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.approvals_needed", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `approver_email_recipients` after provisioning.\nOptional. Additional email addresses to be notified when a grant is pending approval."]
    pub fn approver_email_recipients(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.approver_email_recipients", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `approvers` after provisioning.\n"]
    pub fn approvers(
        &self,
    ) -> ListRef<
        PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElApproversElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.approvers", self.base))
    }
}
#[derive(Serialize, Default)]
struct PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElDynamic {
    steps: Option<
        DynamicBlock<PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsEl>,
    >,
}
#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    require_approver_justification: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    steps:
        Option<Vec<PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsEl>>,
    dynamic: PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElDynamic,
}
impl PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsEl {
    #[doc = "Set the field `require_approver_justification`.\nOptional. Do the approvers need to provide a justification for their actions?"]
    pub fn set_require_approver_justification(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_approver_justification = Some(v.into());
        self
    }
    #[doc = "Set the field `steps`.\n"]
    pub fn set_steps(
        mut self,
        v: impl Into<
            BlockAssignable<
                PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.steps = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.steps = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsEl {
    type O = BlockAssignable<PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsEl {}
impl BuildPrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsEl {
    pub fn build(self) -> PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsEl {
        PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsEl {
            require_approver_justification: core::default::Default::default(),
            steps: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElRef {
        PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `require_approver_justification` after provisioning.\nOptional. Do the approvers need to provide a justification for their actions?"]
    pub fn require_approver_justification(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_approver_justification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `steps` after provisioning.\n"]
    pub fn steps(
        &self,
    ) -> ListRef<PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElStepsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.steps", self.base))
    }
}
#[derive(Serialize, Default)]
struct PrivilegedAccessManagerEntitlementApprovalWorkflowElDynamic {
    manual_approvals:
        Option<DynamicBlock<PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsEl>>,
}
#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlementApprovalWorkflowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    manual_approvals:
        Option<Vec<PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsEl>>,
    dynamic: PrivilegedAccessManagerEntitlementApprovalWorkflowElDynamic,
}
impl PrivilegedAccessManagerEntitlementApprovalWorkflowEl {
    #[doc = "Set the field `manual_approvals`.\n"]
    pub fn set_manual_approvals(
        mut self,
        v: impl Into<
            BlockAssignable<PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.manual_approvals = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.manual_approvals = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PrivilegedAccessManagerEntitlementApprovalWorkflowEl {
    type O = BlockAssignable<PrivilegedAccessManagerEntitlementApprovalWorkflowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivilegedAccessManagerEntitlementApprovalWorkflowEl {}
impl BuildPrivilegedAccessManagerEntitlementApprovalWorkflowEl {
    pub fn build(self) -> PrivilegedAccessManagerEntitlementApprovalWorkflowEl {
        PrivilegedAccessManagerEntitlementApprovalWorkflowEl {
            manual_approvals: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PrivilegedAccessManagerEntitlementApprovalWorkflowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivilegedAccessManagerEntitlementApprovalWorkflowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivilegedAccessManagerEntitlementApprovalWorkflowElRef {
        PrivilegedAccessManagerEntitlementApprovalWorkflowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivilegedAccessManagerEntitlementApprovalWorkflowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `manual_approvals` after provisioning.\n"]
    pub fn manual_approvals(
        &self,
    ) -> ListRef<PrivilegedAccessManagerEntitlementApprovalWorkflowElManualApprovalsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.manual_approvals", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlementEligibleUsersEl {
    principals: SetField<PrimField<String>>,
}
impl PrivilegedAccessManagerEntitlementEligibleUsersEl {}
impl ToListMappable for PrivilegedAccessManagerEntitlementEligibleUsersEl {
    type O = BlockAssignable<PrivilegedAccessManagerEntitlementEligibleUsersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivilegedAccessManagerEntitlementEligibleUsersEl {
    #[doc = "Users who are being allowed for the operation. Each entry should be a valid v1 IAM Principal Identifier. Format for these is documented at \"https://cloud.google.com/iam/docs/principal-identifiers#v1\""]
    pub principals: SetField<PrimField<String>>,
}
impl BuildPrivilegedAccessManagerEntitlementEligibleUsersEl {
    pub fn build(self) -> PrivilegedAccessManagerEntitlementEligibleUsersEl {
        PrivilegedAccessManagerEntitlementEligibleUsersEl {
            principals: self.principals,
        }
    }
}
pub struct PrivilegedAccessManagerEntitlementEligibleUsersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivilegedAccessManagerEntitlementEligibleUsersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivilegedAccessManagerEntitlementEligibleUsersElRef {
        PrivilegedAccessManagerEntitlementEligibleUsersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivilegedAccessManagerEntitlementEligibleUsersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `principals` after provisioning.\nUsers who are being allowed for the operation. Each entry should be a valid v1 IAM Principal Identifier. Format for these is documented at \"https://cloud.google.com/iam/docs/principal-identifiers#v1\""]
    pub fn principals(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.principals", self.base))
    }
}
#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    condition_expression: Option<PrimField<String>>,
    role: PrimField<String>,
}
impl PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsEl {
    #[doc = "Set the field `condition_expression`.\nThe expression field of the IAM condition to be associated with the role. If specified, a user with an active grant for this entitlement would be able to access the resource only if this condition evaluates to true for their request.\nhttps://cloud.google.com/iam/docs/conditions-overview#attributes."]
    pub fn set_condition_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.condition_expression = Some(v.into());
        self
    }
}
impl ToListMappable
    for PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsEl
{
    type O = BlockAssignable<
        PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsEl {
    #[doc = "IAM role to be granted. https://cloud.google.com/iam/docs/roles-overview."]
    pub role: PrimField<String>,
}
impl BuildPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsEl {
    pub fn build(
        self,
    ) -> PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsEl {
        PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsEl {
            condition_expression: core::default::Default::default(),
            role: self.role,
        }
    }
}
pub struct PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsElRef {
        PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `condition_expression` after provisioning.\nThe expression field of the IAM condition to be associated with the role. If specified, a user with an active grant for this entitlement would be able to access the resource only if this condition evaluates to true for their request.\nhttps://cloud.google.com/iam/docs/conditions-overview#attributes."]
    pub fn condition_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.condition_expression", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\nIAM role to be granted. https://cloud.google.com/iam/docs/roles-overview."]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }
}
#[derive(Serialize, Default)]
struct PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElDynamic {
    role_bindings: Option<
        DynamicBlock<
            PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessEl {
    resource: PrimField<String>,
    resource_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_bindings: Option<
        Vec<PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsEl>,
    >,
    dynamic: PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElDynamic,
}
impl PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessEl {
    #[doc = "Set the field `role_bindings`.\n"]
    pub fn set_role_bindings(
        mut self,
        v: impl Into<
            BlockAssignable<
                PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.role_bindings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.role_bindings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessEl {
    type O = BlockAssignable<PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessEl {
    #[doc = "Name of the resource."]
    pub resource: PrimField<String>,
    #[doc = "The type of this resource."]
    pub resource_type: PrimField<String>,
}
impl BuildPrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessEl {
    pub fn build(self) -> PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessEl {
        PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessEl {
            resource: self.resource,
            resource_type: self.resource_type,
            role_bindings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRef {
        PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource` after provisioning.\nName of the resource."]
    pub fn resource(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\nThe type of this resource."]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `role_bindings` after provisioning.\n"]
    pub fn role_bindings(
        &self,
    ) -> ListRef<PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRoleBindingsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.role_bindings", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct PrivilegedAccessManagerEntitlementPrivilegedAccessElDynamic {
    gcp_iam_access:
        Option<DynamicBlock<PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessEl>>,
}
#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlementPrivilegedAccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_iam_access: Option<Vec<PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessEl>>,
    dynamic: PrivilegedAccessManagerEntitlementPrivilegedAccessElDynamic,
}
impl PrivilegedAccessManagerEntitlementPrivilegedAccessEl {
    #[doc = "Set the field `gcp_iam_access`.\n"]
    pub fn set_gcp_iam_access(
        mut self,
        v: impl Into<
            BlockAssignable<PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gcp_iam_access = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gcp_iam_access = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PrivilegedAccessManagerEntitlementPrivilegedAccessEl {
    type O = BlockAssignable<PrivilegedAccessManagerEntitlementPrivilegedAccessEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivilegedAccessManagerEntitlementPrivilegedAccessEl {}
impl BuildPrivilegedAccessManagerEntitlementPrivilegedAccessEl {
    pub fn build(self) -> PrivilegedAccessManagerEntitlementPrivilegedAccessEl {
        PrivilegedAccessManagerEntitlementPrivilegedAccessEl {
            gcp_iam_access: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PrivilegedAccessManagerEntitlementPrivilegedAccessElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivilegedAccessManagerEntitlementPrivilegedAccessElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivilegedAccessManagerEntitlementPrivilegedAccessElRef {
        PrivilegedAccessManagerEntitlementPrivilegedAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivilegedAccessManagerEntitlementPrivilegedAccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gcp_iam_access` after provisioning.\n"]
    pub fn gcp_iam_access(
        &self,
    ) -> ListRef<PrivilegedAccessManagerEntitlementPrivilegedAccessElGcpIamAccessElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcp_iam_access", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryEl {}
impl PrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryEl {}
impl ToListMappable
    for PrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryEl
{
    type O = BlockAssignable<
        PrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryEl {}
impl BuildPrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryEl {
    pub fn build(
        self,
    ) -> PrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryEl {
        PrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryEl {}
    }
}
pub struct PrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryElRef {
        PrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredEl {}
impl PrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredEl {}
impl ToListMappable
    for PrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredEl
{
    type O = BlockAssignable<
        PrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredEl {}
impl BuildPrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredEl {
    pub fn build(
        self,
    ) -> PrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredEl {
        PrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredEl {}
    }
}
pub struct PrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredElRef {
        PrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct PrivilegedAccessManagerEntitlementRequesterJustificationConfigElDynamic {
    not_mandatory: Option<
        DynamicBlock<
            PrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryEl,
        >,
    >,
    unstructured: Option<
        DynamicBlock<
            PrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlementRequesterJustificationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    not_mandatory:
        Option<Vec<PrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unstructured:
        Option<Vec<PrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredEl>>,
    dynamic: PrivilegedAccessManagerEntitlementRequesterJustificationConfigElDynamic,
}
impl PrivilegedAccessManagerEntitlementRequesterJustificationConfigEl {
    #[doc = "Set the field `not_mandatory`.\n"]
    pub fn set_not_mandatory(
        mut self,
        v: impl Into<
            BlockAssignable<
                PrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.not_mandatory = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.not_mandatory = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `unstructured`.\n"]
    pub fn set_unstructured(
        mut self,
        v: impl Into<
            BlockAssignable<
                PrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.unstructured = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.unstructured = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PrivilegedAccessManagerEntitlementRequesterJustificationConfigEl {
    type O = BlockAssignable<PrivilegedAccessManagerEntitlementRequesterJustificationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivilegedAccessManagerEntitlementRequesterJustificationConfigEl {}
impl BuildPrivilegedAccessManagerEntitlementRequesterJustificationConfigEl {
    pub fn build(self) -> PrivilegedAccessManagerEntitlementRequesterJustificationConfigEl {
        PrivilegedAccessManagerEntitlementRequesterJustificationConfigEl {
            not_mandatory: core::default::Default::default(),
            unstructured: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PrivilegedAccessManagerEntitlementRequesterJustificationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivilegedAccessManagerEntitlementRequesterJustificationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivilegedAccessManagerEntitlementRequesterJustificationConfigElRef {
        PrivilegedAccessManagerEntitlementRequesterJustificationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivilegedAccessManagerEntitlementRequesterJustificationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `not_mandatory` after provisioning.\n"]
    pub fn not_mandatory(
        &self,
    ) -> ListRef<PrivilegedAccessManagerEntitlementRequesterJustificationConfigElNotMandatoryElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.not_mandatory", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `unstructured` after provisioning.\n"]
    pub fn unstructured(
        &self,
    ) -> ListRef<PrivilegedAccessManagerEntitlementRequesterJustificationConfigElUnstructuredElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.unstructured", self.base))
    }
}
#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlementTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl PrivilegedAccessManagerEntitlementTimeoutsEl {
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
impl ToListMappable for PrivilegedAccessManagerEntitlementTimeoutsEl {
    type O = BlockAssignable<PrivilegedAccessManagerEntitlementTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivilegedAccessManagerEntitlementTimeoutsEl {}
impl BuildPrivilegedAccessManagerEntitlementTimeoutsEl {
    pub fn build(self) -> PrivilegedAccessManagerEntitlementTimeoutsEl {
        PrivilegedAccessManagerEntitlementTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct PrivilegedAccessManagerEntitlementTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivilegedAccessManagerEntitlementTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> PrivilegedAccessManagerEntitlementTimeoutsElRef {
        PrivilegedAccessManagerEntitlementTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivilegedAccessManagerEntitlementTimeoutsElRef {
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
struct PrivilegedAccessManagerEntitlementDynamic {
    additional_notification_targets:
        Option<DynamicBlock<PrivilegedAccessManagerEntitlementAdditionalNotificationTargetsEl>>,
    approval_workflow: Option<DynamicBlock<PrivilegedAccessManagerEntitlementApprovalWorkflowEl>>,
    eligible_users: Option<DynamicBlock<PrivilegedAccessManagerEntitlementEligibleUsersEl>>,
    privileged_access: Option<DynamicBlock<PrivilegedAccessManagerEntitlementPrivilegedAccessEl>>,
    requester_justification_config:
        Option<DynamicBlock<PrivilegedAccessManagerEntitlementRequesterJustificationConfigEl>>,
}
