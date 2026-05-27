use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct FirebaseAppCheckResourcePolicyData {
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
    enforcement_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    service_id: PrimField<String>,
    target_resource: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FirebaseAppCheckResourcePolicyTimeoutsEl>,
}
struct FirebaseAppCheckResourcePolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FirebaseAppCheckResourcePolicyData>,
}
#[derive(Clone)]
pub struct FirebaseAppCheckResourcePolicy(Rc<FirebaseAppCheckResourcePolicy_>);
impl FirebaseAppCheckResourcePolicy {
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
    #[doc = "Set the field `enforcement_mode`.\nThe App Check enforcement mode for a service supported by App Check. This will override the EnforcementMode setting on the service.\nValid values are:\n\n(Unset)\nFirebase App Check is not enforced for the service, nor are App Check metrics collected.\nThough the service is not protected by App Check in this mode, other applicable protections,\nsuch as user authorization, are still enforced. An unconfigured service is in this mode by default.\nThis is equivalent to OFF in the REST API. Deleting the Terraform resource will also switch the\nenforcement to OFF for this service.\n\nUNENFORCED\nFirebase App Check is not enforced for the service. App Check metrics are collected to help you\ndecide when to turn on enforcement for the service. Though the service is not protected by App Check\nin this mode, other applicable protections, such as user authorization, are still enforced.\n\nENFORCED\nFirebase App Check is enforced for the service. The service will reject any request that attempts to\naccess your project's resources if it does not have valid App Check token attached, with some exceptions\ndepending on the service; for example, some services will still allow requests bearing the developer's\nprivileged service account credentials without an App Check token. App Check metrics continue to be\ncollected to help you detect issues with your App Check integration and monitor the composition of your\ncallers. While the service is protected by App Check, other applicable protections, such as user\nauthorization, continue to be enforced at the same time.\n\nUse caution when choosing to enforce App Check on a Firebase service. If your users have not updated\nto an App Check capable version of your app, their apps will no longer be able to use your Firebase\nservices that are enforcing App Check. App Check metrics can help you decide whether to enforce App\nCheck on your Firebase services.\n\nIf your app has not launched yet, you should enable enforcement immediately, since there are no outdated\nclients in use. Possible values: [\"UNENFORCED\", \"ENFORCED\"]"]
    pub fn set_enforcement_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().enforcement_mode = Some(v.into());
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
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FirebaseAppCheckResourcePolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enforcement_mode` after provisioning.\nThe App Check enforcement mode for a service supported by App Check. This will override the EnforcementMode setting on the service.\nValid values are:\n\n(Unset)\nFirebase App Check is not enforced for the service, nor are App Check metrics collected.\nThough the service is not protected by App Check in this mode, other applicable protections,\nsuch as user authorization, are still enforced. An unconfigured service is in this mode by default.\nThis is equivalent to OFF in the REST API. Deleting the Terraform resource will also switch the\nenforcement to OFF for this service.\n\nUNENFORCED\nFirebase App Check is not enforced for the service. App Check metrics are collected to help you\ndecide when to turn on enforcement for the service. Though the service is not protected by App Check\nin this mode, other applicable protections, such as user authorization, are still enforced.\n\nENFORCED\nFirebase App Check is enforced for the service. The service will reject any request that attempts to\naccess your project's resources if it does not have valid App Check token attached, with some exceptions\ndepending on the service; for example, some services will still allow requests bearing the developer's\nprivileged service account credentials without an App Check token. App Check metrics continue to be\ncollected to help you detect issues with your App Check integration and monitor the composition of your\ncallers. While the service is protected by App Check, other applicable protections, such as user\nauthorization, continue to be enforced at the same time.\n\nUse caution when choosing to enforce App Check on a Firebase service. If your users have not updated\nto an App Check capable version of your app, their apps will no longer be able to use your Firebase\nservices that are enforcing App Check. App Check metrics can help you decide whether to enforce App\nCheck on your Firebase services.\n\nIf your app has not launched yet, you should enable enforcement immediately, since there are no outdated\nclients in use. Possible values: [\"UNENFORCED\", \"ENFORCED\"]"]
    pub fn enforcement_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforcement_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
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
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_policy_id` after provisioning.\nThe server-generated UID for the Resource Policy."]
    pub fn resource_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_id` after provisioning.\nThe identifier of the service to configure a Resource Policy for.\n\nCurrently, the following service IDs are supported:\n* 'oauth2.googleapis.com' (Google Identity for iOS)"]
    pub fn service_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_resource` after provisioning.\nService specific name of the resource object to which this policy applies, in the format:\n* iOS OAuth clients (Google Identity for iOS):\n  '//oauth2.googleapis.com/projects/{project_number}/oauthClients/{oauthClientId}'"]
    pub fn target_resource(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_resource", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Timestamp when this resource policy configuration object was most recently updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaseAppCheckResourcePolicyTimeoutsElRef {
        FirebaseAppCheckResourcePolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for FirebaseAppCheckResourcePolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for FirebaseAppCheckResourcePolicy {}
impl ToListMappable for FirebaseAppCheckResourcePolicy {
    type O = ListRef<FirebaseAppCheckResourcePolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for FirebaseAppCheckResourcePolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_firebase_app_check_resource_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildFirebaseAppCheckResourcePolicy {
    pub tf_id: String,
    #[doc = "The identifier of the service to configure a Resource Policy for.\n\nCurrently, the following service IDs are supported:\n* 'oauth2.googleapis.com' (Google Identity for iOS)"]
    pub service_id: PrimField<String>,
    #[doc = "Service specific name of the resource object to which this policy applies, in the format:\n* iOS OAuth clients (Google Identity for iOS):\n  '//oauth2.googleapis.com/projects/{project_number}/oauthClients/{oauthClientId}'"]
    pub target_resource: PrimField<String>,
}
impl BuildFirebaseAppCheckResourcePolicy {
    pub fn build(self, stack: &mut Stack) -> FirebaseAppCheckResourcePolicy {
        let out = FirebaseAppCheckResourcePolicy(Rc::new(FirebaseAppCheckResourcePolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FirebaseAppCheckResourcePolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                enforcement_mode: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                service_id: self.service_id,
                target_resource: self.target_resource,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct FirebaseAppCheckResourcePolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppCheckResourcePolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl FirebaseAppCheckResourcePolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enforcement_mode` after provisioning.\nThe App Check enforcement mode for a service supported by App Check. This will override the EnforcementMode setting on the service.\nValid values are:\n\n(Unset)\nFirebase App Check is not enforced for the service, nor are App Check metrics collected.\nThough the service is not protected by App Check in this mode, other applicable protections,\nsuch as user authorization, are still enforced. An unconfigured service is in this mode by default.\nThis is equivalent to OFF in the REST API. Deleting the Terraform resource will also switch the\nenforcement to OFF for this service.\n\nUNENFORCED\nFirebase App Check is not enforced for the service. App Check metrics are collected to help you\ndecide when to turn on enforcement for the service. Though the service is not protected by App Check\nin this mode, other applicable protections, such as user authorization, are still enforced.\n\nENFORCED\nFirebase App Check is enforced for the service. The service will reject any request that attempts to\naccess your project's resources if it does not have valid App Check token attached, with some exceptions\ndepending on the service; for example, some services will still allow requests bearing the developer's\nprivileged service account credentials without an App Check token. App Check metrics continue to be\ncollected to help you detect issues with your App Check integration and monitor the composition of your\ncallers. While the service is protected by App Check, other applicable protections, such as user\nauthorization, continue to be enforced at the same time.\n\nUse caution when choosing to enforce App Check on a Firebase service. If your users have not updated\nto an App Check capable version of your app, their apps will no longer be able to use your Firebase\nservices that are enforcing App Check. App Check metrics can help you decide whether to enforce App\nCheck on your Firebase services.\n\nIf your app has not launched yet, you should enable enforcement immediately, since there are no outdated\nclients in use. Possible values: [\"UNENFORCED\", \"ENFORCED\"]"]
    pub fn enforcement_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforcement_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
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
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_policy_id` after provisioning.\nThe server-generated UID for the Resource Policy."]
    pub fn resource_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_id` after provisioning.\nThe identifier of the service to configure a Resource Policy for.\n\nCurrently, the following service IDs are supported:\n* 'oauth2.googleapis.com' (Google Identity for iOS)"]
    pub fn service_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_resource` after provisioning.\nService specific name of the resource object to which this policy applies, in the format:\n* iOS OAuth clients (Google Identity for iOS):\n  '//oauth2.googleapis.com/projects/{project_number}/oauthClients/{oauthClientId}'"]
    pub fn target_resource(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_resource", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Timestamp when this resource policy configuration object was most recently updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaseAppCheckResourcePolicyTimeoutsElRef {
        FirebaseAppCheckResourcePolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct FirebaseAppCheckResourcePolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl FirebaseAppCheckResourcePolicyTimeoutsEl {
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
impl ToListMappable for FirebaseAppCheckResourcePolicyTimeoutsEl {
    type O = BlockAssignable<FirebaseAppCheckResourcePolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppCheckResourcePolicyTimeoutsEl {}
impl BuildFirebaseAppCheckResourcePolicyTimeoutsEl {
    pub fn build(self) -> FirebaseAppCheckResourcePolicyTimeoutsEl {
        FirebaseAppCheckResourcePolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct FirebaseAppCheckResourcePolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppCheckResourcePolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FirebaseAppCheckResourcePolicyTimeoutsElRef {
        FirebaseAppCheckResourcePolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppCheckResourcePolicyTimeoutsElRef {
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
