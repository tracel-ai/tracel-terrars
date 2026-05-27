use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct MonitoringNotificationChannelData {
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
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_delete: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sensitive_labels: Option<Vec<MonitoringNotificationChannelSensitiveLabelsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<MonitoringNotificationChannelTimeoutsEl>,
    dynamic: MonitoringNotificationChannelDynamic,
}
struct MonitoringNotificationChannel_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<MonitoringNotificationChannelData>,
}
#[derive(Clone)]
pub struct MonitoringNotificationChannel(Rc<MonitoringNotificationChannel_>);
impl MonitoringNotificationChannel {
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
    #[doc = "Set the field `description`.\nAn optional human-readable description of this notification channel. This description may provide additional details, beyond the display name, for the channel. This may not exceed 1024 Unicode characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nAn optional human-readable name for this notification channel. It is recommended that you specify a non-empty and unique name in order to make it easier to identify the channels in your project, though this is not enforced. The display name is limited to 512 Unicode characters."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\nWhether notifications are forwarded to the described channel. This makes it possible to disable delivery of notifications to a particular channel without removing the channel from all alerting policies that reference the channel. This is a more convenient approach when the change is temporary and you want to receive notifications from the same set of alerting policies on the channel at some point in the future."]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `force_delete`.\nIf true, the notification channel will be deleted regardless\nof its use in alert policies (the policies will be updated\nto remove the channel). If false, channels that are still\nreferenced by an existing alerting policy will fail to be\ndeleted in a delete operation."]
    pub fn set_force_delete(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_delete = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nConfiguration fields that define the channel and its behavior. The\npermissible and required labels are specified in the\nNotificationChannelDescriptor corresponding to the type field.\n\nLabels with sensitive data are obfuscated by the API and therefore Terraform cannot\ndetermine if there are upstream changes to these fields. They can also be configured via\nthe sensitive_labels block, but cannot be configured in both places."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `user_labels`.\nUser-supplied key/value data that does not need to conform to the corresponding NotificationChannelDescriptor's schema, unlike the labels field. This field is intended to be used for organizing and identifying the NotificationChannel objects.The field can contain up to 64 entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter."]
    pub fn set_user_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().user_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `sensitive_labels`.\n"]
    pub fn set_sensitive_labels(
        self,
        v: impl Into<BlockAssignable<MonitoringNotificationChannelSensitiveLabelsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sensitive_labels = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sensitive_labels = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<MonitoringNotificationChannelTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional human-readable description of this notification channel. This description may provide additional details, beyond the display name, for the channel. This may not exceed 1024 Unicode characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nAn optional human-readable name for this notification channel. It is recommended that you specify a non-empty and unique name in order to make it easier to identify the channels in your project, though this is not enforced. The display name is limited to 512 Unicode characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether notifications are forwarded to the described channel. This makes it possible to disable delivery of notifications to a particular channel without removing the channel from all alerting policies that reference the channel. This is a more convenient approach when the change is temporary and you want to receive notifications from the same set of alerting policies on the channel at some point in the future."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_delete` after provisioning.\nIf true, the notification channel will be deleted regardless\nof its use in alert policies (the policies will be updated\nto remove the channel). If false, channels that are still\nreferenced by an existing alerting policy will fail to be\ndeleted in a delete operation."]
    pub fn force_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_delete", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nConfiguration fields that define the channel and its behavior. The\npermissible and required labels are specified in the\nNotificationChannelDescriptor corresponding to the type field.\n\nLabels with sensitive data are obfuscated by the API and therefore Terraform cannot\ndetermine if there are upstream changes to these fields. They can also be configured via\nthe sensitive_labels block, but cannot be configured in both places."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe full REST resource name for this channel. The syntax is:\nprojects/[PROJECT_ID]/notificationChannels/[CHANNEL_ID]\nThe [CHANNEL_ID] is automatically assigned by the server on creation."]
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the notification channel. This field matches the value of the NotificationChannelDescriptor.type field. See https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.notificationChannelDescriptors/list to get the list of valid values such as \"email\", \"slack\", etc..."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_labels` after provisioning.\nUser-supplied key/value data that does not need to conform to the corresponding NotificationChannelDescriptor's schema, unlike the labels field. This field is intended to be used for organizing and identifying the NotificationChannel objects.The field can contain up to 64 entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter."]
    pub fn user_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.user_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `verification_status` after provisioning.\nIndicates whether this channel has been verified or not. On a ListNotificationChannels or GetNotificationChannel operation, this field is expected to be populated.If the value is UNVERIFIED, then it indicates that the channel is non-functioning (it both requires verification and lacks verification); otherwise, it is assumed that the channel works.If the channel is neither VERIFIED nor UNVERIFIED, it implies that the channel is of a type that does not require verification or that this specific channel has been exempted from verification because it was created prior to verification being required for channels of this type.This field cannot be modified using a standard UpdateNotificationChannel operation. To change the value of this field, you must call VerifyNotificationChannel."]
    pub fn verification_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.verification_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sensitive_labels` after provisioning.\n"]
    pub fn sensitive_labels(&self) -> ListRef<MonitoringNotificationChannelSensitiveLabelsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sensitive_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MonitoringNotificationChannelTimeoutsElRef {
        MonitoringNotificationChannelTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for MonitoringNotificationChannel {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for MonitoringNotificationChannel {}
impl ToListMappable for MonitoringNotificationChannel {
    type O = ListRef<MonitoringNotificationChannelRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for MonitoringNotificationChannel_ {
    fn extract_resource_type(&self) -> String {
        "google_monitoring_notification_channel".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildMonitoringNotificationChannel {
    pub tf_id: String,
    #[doc = "The type of the notification channel. This field matches the value of the NotificationChannelDescriptor.type field. See https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.notificationChannelDescriptors/list to get the list of valid values such as \"email\", \"slack\", etc..."]
    pub type_: PrimField<String>,
}
impl BuildMonitoringNotificationChannel {
    pub fn build(self, stack: &mut Stack) -> MonitoringNotificationChannel {
        let out = MonitoringNotificationChannel(Rc::new(MonitoringNotificationChannel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(MonitoringNotificationChannelData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                enabled: core::default::Default::default(),
                force_delete: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                project: core::default::Default::default(),
                type_: self.type_,
                user_labels: core::default::Default::default(),
                sensitive_labels: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct MonitoringNotificationChannelRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringNotificationChannelRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl MonitoringNotificationChannelRef {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional human-readable description of this notification channel. This description may provide additional details, beyond the display name, for the channel. This may not exceed 1024 Unicode characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nAn optional human-readable name for this notification channel. It is recommended that you specify a non-empty and unique name in order to make it easier to identify the channels in your project, though this is not enforced. The display name is limited to 512 Unicode characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether notifications are forwarded to the described channel. This makes it possible to disable delivery of notifications to a particular channel without removing the channel from all alerting policies that reference the channel. This is a more convenient approach when the change is temporary and you want to receive notifications from the same set of alerting policies on the channel at some point in the future."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_delete` after provisioning.\nIf true, the notification channel will be deleted regardless\nof its use in alert policies (the policies will be updated\nto remove the channel). If false, channels that are still\nreferenced by an existing alerting policy will fail to be\ndeleted in a delete operation."]
    pub fn force_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_delete", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nConfiguration fields that define the channel and its behavior. The\npermissible and required labels are specified in the\nNotificationChannelDescriptor corresponding to the type field.\n\nLabels with sensitive data are obfuscated by the API and therefore Terraform cannot\ndetermine if there are upstream changes to these fields. They can also be configured via\nthe sensitive_labels block, but cannot be configured in both places."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe full REST resource name for this channel. The syntax is:\nprojects/[PROJECT_ID]/notificationChannels/[CHANNEL_ID]\nThe [CHANNEL_ID] is automatically assigned by the server on creation."]
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the notification channel. This field matches the value of the NotificationChannelDescriptor.type field. See https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.notificationChannelDescriptors/list to get the list of valid values such as \"email\", \"slack\", etc..."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_labels` after provisioning.\nUser-supplied key/value data that does not need to conform to the corresponding NotificationChannelDescriptor's schema, unlike the labels field. This field is intended to be used for organizing and identifying the NotificationChannel objects.The field can contain up to 64 entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter."]
    pub fn user_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.user_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `verification_status` after provisioning.\nIndicates whether this channel has been verified or not. On a ListNotificationChannels or GetNotificationChannel operation, this field is expected to be populated.If the value is UNVERIFIED, then it indicates that the channel is non-functioning (it both requires verification and lacks verification); otherwise, it is assumed that the channel works.If the channel is neither VERIFIED nor UNVERIFIED, it implies that the channel is of a type that does not require verification or that this specific channel has been exempted from verification because it was created prior to verification being required for channels of this type.This field cannot be modified using a standard UpdateNotificationChannel operation. To change the value of this field, you must call VerifyNotificationChannel."]
    pub fn verification_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.verification_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sensitive_labels` after provisioning.\n"]
    pub fn sensitive_labels(&self) -> ListRef<MonitoringNotificationChannelSensitiveLabelsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sensitive_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MonitoringNotificationChannelTimeoutsElRef {
        MonitoringNotificationChannelTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct MonitoringNotificationChannelSensitiveLabelsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_token_wo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_token_wo_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_wo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_wo_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_key_wo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_key_wo_version: Option<PrimField<String>>,
}
impl MonitoringNotificationChannelSensitiveLabelsEl {
    #[doc = "Set the field `auth_token`.\nAn authorization token for a notification channel. Channel types that support this field include: slack"]
    pub fn set_auth_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_token = Some(v.into());
        self
    }
    #[doc = "Set the field `auth_token_wo`.\nAn authorization token for a notification channel. Channel types that support this field include: slack"]
    pub fn set_auth_token_wo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_token_wo = Some(v.into());
        self
    }
    #[doc = "Set the field `auth_token_wo_version`.\nTriggers update of 'auth_token_wo' write-only. Increment this value when an update to 'auth_token_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn set_auth_token_wo_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_token_wo_version = Some(v.into());
        self
    }
    #[doc = "Set the field `password`.\nAn password for a notification channel. Channel types that support this field include: webhook_basicauth"]
    pub fn set_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password = Some(v.into());
        self
    }
    #[doc = "Set the field `password_wo`.\nAn password for a notification channel. Channel types that support this field include: webhook_basicauth"]
    pub fn set_password_wo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password_wo = Some(v.into());
        self
    }
    #[doc = "Set the field `password_wo_version`.\nTriggers update of 'password_wo' write-only. Increment this value when an update to 'password_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn set_password_wo_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password_wo_version = Some(v.into());
        self
    }
    #[doc = "Set the field `service_key`.\nAn servicekey token for a notification channel. Channel types that support this field include: pagerduty"]
    pub fn set_service_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_key = Some(v.into());
        self
    }
    #[doc = "Set the field `service_key_wo`.\nAn servicekey token for a notification channel. Channel types that support this field include: pagerduty"]
    pub fn set_service_key_wo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_key_wo = Some(v.into());
        self
    }
    #[doc = "Set the field `service_key_wo_version`.\nTriggers update of 'service_key_wo' write-only. Increment this value when an update to 'service_key_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn set_service_key_wo_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_key_wo_version = Some(v.into());
        self
    }
}
impl ToListMappable for MonitoringNotificationChannelSensitiveLabelsEl {
    type O = BlockAssignable<MonitoringNotificationChannelSensitiveLabelsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringNotificationChannelSensitiveLabelsEl {}
impl BuildMonitoringNotificationChannelSensitiveLabelsEl {
    pub fn build(self) -> MonitoringNotificationChannelSensitiveLabelsEl {
        MonitoringNotificationChannelSensitiveLabelsEl {
            auth_token: core::default::Default::default(),
            auth_token_wo: core::default::Default::default(),
            auth_token_wo_version: core::default::Default::default(),
            password: core::default::Default::default(),
            password_wo: core::default::Default::default(),
            password_wo_version: core::default::Default::default(),
            service_key: core::default::Default::default(),
            service_key_wo: core::default::Default::default(),
            service_key_wo_version: core::default::Default::default(),
        }
    }
}
pub struct MonitoringNotificationChannelSensitiveLabelsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringNotificationChannelSensitiveLabelsElRef {
    fn new(shared: StackShared, base: String) -> MonitoringNotificationChannelSensitiveLabelsElRef {
        MonitoringNotificationChannelSensitiveLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringNotificationChannelSensitiveLabelsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auth_token` after provisioning.\nAn authorization token for a notification channel. Channel types that support this field include: slack"]
    pub fn auth_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_token", self.base))
    }
    #[doc = "Get a reference to the value of field `auth_token_wo` after provisioning.\nAn authorization token for a notification channel. Channel types that support this field include: slack"]
    pub fn auth_token_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auth_token_wo", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `auth_token_wo_version` after provisioning.\nTriggers update of 'auth_token_wo' write-only. Increment this value when an update to 'auth_token_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn auth_token_wo_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auth_token_wo_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nAn password for a notification channel. Channel types that support this field include: webhook_basicauth"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `password_wo` after provisioning.\nAn password for a notification channel. Channel types that support this field include: webhook_basicauth"]
    pub fn password_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_wo", self.base))
    }
    #[doc = "Get a reference to the value of field `password_wo_version` after provisioning.\nTriggers update of 'password_wo' write-only. Increment this value when an update to 'password_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn password_wo_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password_wo_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_key` after provisioning.\nAn servicekey token for a notification channel. Channel types that support this field include: pagerduty"]
    pub fn service_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_key", self.base))
    }
    #[doc = "Get a reference to the value of field `service_key_wo` after provisioning.\nAn servicekey token for a notification channel. Channel types that support this field include: pagerduty"]
    pub fn service_key_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_key_wo", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_key_wo_version` after provisioning.\nTriggers update of 'service_key_wo' write-only. Increment this value when an update to 'service_key_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn service_key_wo_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_key_wo_version", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MonitoringNotificationChannelTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl MonitoringNotificationChannelTimeoutsEl {
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
impl ToListMappable for MonitoringNotificationChannelTimeoutsEl {
    type O = BlockAssignable<MonitoringNotificationChannelTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringNotificationChannelTimeoutsEl {}
impl BuildMonitoringNotificationChannelTimeoutsEl {
    pub fn build(self) -> MonitoringNotificationChannelTimeoutsEl {
        MonitoringNotificationChannelTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct MonitoringNotificationChannelTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringNotificationChannelTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> MonitoringNotificationChannelTimeoutsElRef {
        MonitoringNotificationChannelTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringNotificationChannelTimeoutsElRef {
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
struct MonitoringNotificationChannelDynamic {
    sensitive_labels: Option<DynamicBlock<MonitoringNotificationChannelSensitiveLabelsEl>>,
}
