use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DialogflowCxSecuritySettingsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deidentify_template: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inspect_template: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    purge_data_types: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redaction_scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redaction_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_window_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_export_settings: Option<Vec<DialogflowCxSecuritySettingsAudioExportSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insights_export_settings: Option<Vec<DialogflowCxSecuritySettingsInsightsExportSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DialogflowCxSecuritySettingsTimeoutsEl>,
    dynamic: DialogflowCxSecuritySettingsDynamic,
}
struct DialogflowCxSecuritySettings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DialogflowCxSecuritySettingsData>,
}
#[derive(Clone)]
pub struct DialogflowCxSecuritySettings(Rc<DialogflowCxSecuritySettings_>);
impl DialogflowCxSecuritySettings {
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
    #[doc = "Set the field `deidentify_template`.\n[DLP](https://cloud.google.com/dlp/docs) deidentify template name. Use this template to define de-identification configuration for the content. If empty, Dialogflow replaces sensitive info with [redacted] text.\nNote: deidentifyTemplate must be located in the same region as the SecuritySettings.\nFormat: projects/<Project ID>/locations/<Location ID>/deidentifyTemplates/<Template ID> OR organizations/<Organization ID>/locations/<Location ID>/deidentifyTemplates/<Template ID>"]
    pub fn set_deidentify_template(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deidentify_template = Some(v.into());
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
    #[doc = "Set the field `inspect_template`.\n[DLP](https://cloud.google.com/dlp/docs) inspect template name. Use this template to define inspect base settings. If empty, we use the default DLP inspect config.\nNote: inspectTemplate must be located in the same region as the SecuritySettings.\nFormat: projects/<Project ID>/locations/<Location ID>/inspectTemplates/<Template ID> OR organizations/<Organization ID>/locations/<Location ID>/inspectTemplates/<Template ID>"]
    pub fn set_inspect_template(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().inspect_template = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `purge_data_types`.\nList of types of data to remove when retention settings triggers purge. Possible values: [\"DIALOGFLOW_HISTORY\"]"]
    pub fn set_purge_data_types(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().purge_data_types = Some(v.into());
        self
    }
    #[doc = "Set the field `redaction_scope`.\nDefines what types of data to redact. If not set, defaults to not redacting any kind of data.\n* REDACT_DISK_STORAGE: On data to be written to disk or similar devices that are capable of holding data even if power is disconnected. This includes data that are temporarily saved on disk. Possible values: [\"REDACT_DISK_STORAGE\"]"]
    pub fn set_redaction_scope(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().redaction_scope = Some(v.into());
        self
    }
    #[doc = "Set the field `redaction_strategy`.\nDefines how we redact data. If not set, defaults to not redacting.\n* REDACT_WITH_SERVICE: Call redaction service to clean up the data to be persisted. Possible values: [\"REDACT_WITH_SERVICE\"]"]
    pub fn set_redaction_strategy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().redaction_strategy = Some(v.into());
        self
    }
    #[doc = "Set the field `retention_strategy`.\nDefines how long we retain persisted data that contains sensitive info. Only one of 'retention_window_days' and 'retention_strategy' may be set.\n* REMOVE_AFTER_CONVERSATION: Removes data when the conversation ends. If there is no conversation explicitly established, a default conversation ends when the corresponding Dialogflow session ends. Possible values: [\"REMOVE_AFTER_CONVERSATION\"]"]
    pub fn set_retention_strategy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().retention_strategy = Some(v.into());
        self
    }
    #[doc = "Set the field `retention_window_days`.\nRetains the data for the specified number of days. User must set a value lower than Dialogflow's default 365d TTL (30 days for Agent Assist traffic), higher value will be ignored and use default. Setting a value higher than that has no effect. A missing value or setting to 0 also means we use default TTL.\nOnly one of 'retention_window_days' and 'retention_strategy' may be set."]
    pub fn set_retention_window_days(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().retention_window_days = Some(v.into());
        self
    }
    #[doc = "Set the field `audio_export_settings`.\n"]
    pub fn set_audio_export_settings(
        self,
        v: impl Into<BlockAssignable<DialogflowCxSecuritySettingsAudioExportSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().audio_export_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.audio_export_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `insights_export_settings`.\n"]
    pub fn set_insights_export_settings(
        self,
        v: impl Into<BlockAssignable<DialogflowCxSecuritySettingsInsightsExportSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().insights_export_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.insights_export_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DialogflowCxSecuritySettingsTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deidentify_template` after provisioning.\n[DLP](https://cloud.google.com/dlp/docs) deidentify template name. Use this template to define de-identification configuration for the content. If empty, Dialogflow replaces sensitive info with [redacted] text.\nNote: deidentifyTemplate must be located in the same region as the SecuritySettings.\nFormat: projects/<Project ID>/locations/<Location ID>/deidentifyTemplates/<Template ID> OR organizations/<Organization ID>/locations/<Location ID>/deidentifyTemplates/<Template ID>"]
    pub fn deidentify_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deidentify_template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the security settings, unique within the location."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `inspect_template` after provisioning.\n[DLP](https://cloud.google.com/dlp/docs) inspect template name. Use this template to define inspect base settings. If empty, we use the default DLP inspect config.\nNote: inspectTemplate must be located in the same region as the SecuritySettings.\nFormat: projects/<Project ID>/locations/<Location ID>/inspectTemplates/<Template ID> OR organizations/<Organization ID>/locations/<Location ID>/inspectTemplates/<Template ID>"]
    pub fn inspect_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inspect_template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location these settings are located in. Settings can only be applied to an agent in the same location.\nSee [Available Regions](https://cloud.google.com/dialogflow/cx/docs/concept/region#avail) for a list of supported locations."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the settings.\nFormat: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>."]
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
    #[doc = "Get a reference to the value of field `purge_data_types` after provisioning.\nList of types of data to remove when retention settings triggers purge. Possible values: [\"DIALOGFLOW_HISTORY\"]"]
    pub fn purge_data_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.purge_data_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redaction_scope` after provisioning.\nDefines what types of data to redact. If not set, defaults to not redacting any kind of data.\n* REDACT_DISK_STORAGE: On data to be written to disk or similar devices that are capable of holding data even if power is disconnected. This includes data that are temporarily saved on disk. Possible values: [\"REDACT_DISK_STORAGE\"]"]
    pub fn redaction_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redaction_scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redaction_strategy` after provisioning.\nDefines how we redact data. If not set, defaults to not redacting.\n* REDACT_WITH_SERVICE: Call redaction service to clean up the data to be persisted. Possible values: [\"REDACT_WITH_SERVICE\"]"]
    pub fn redaction_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redaction_strategy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retention_strategy` after provisioning.\nDefines how long we retain persisted data that contains sensitive info. Only one of 'retention_window_days' and 'retention_strategy' may be set.\n* REMOVE_AFTER_CONVERSATION: Removes data when the conversation ends. If there is no conversation explicitly established, a default conversation ends when the corresponding Dialogflow session ends. Possible values: [\"REMOVE_AFTER_CONVERSATION\"]"]
    pub fn retention_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention_strategy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retention_window_days` after provisioning.\nRetains the data for the specified number of days. User must set a value lower than Dialogflow's default 365d TTL (30 days for Agent Assist traffic), higher value will be ignored and use default. Setting a value higher than that has no effect. A missing value or setting to 0 also means we use default TTL.\nOnly one of 'retention_window_days' and 'retention_strategy' may be set."]
    pub fn retention_window_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention_window_days", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `audio_export_settings` after provisioning.\n"]
    pub fn audio_export_settings(
        &self,
    ) -> ListRef<DialogflowCxSecuritySettingsAudioExportSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.audio_export_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `insights_export_settings` after provisioning.\n"]
    pub fn insights_export_settings(
        &self,
    ) -> ListRef<DialogflowCxSecuritySettingsInsightsExportSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.insights_export_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxSecuritySettingsTimeoutsElRef {
        DialogflowCxSecuritySettingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DialogflowCxSecuritySettings {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DialogflowCxSecuritySettings {}
impl ToListMappable for DialogflowCxSecuritySettings {
    type O = ListRef<DialogflowCxSecuritySettingsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DialogflowCxSecuritySettings_ {
    fn extract_resource_type(&self) -> String {
        "google_dialogflow_cx_security_settings".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDialogflowCxSecuritySettings {
    pub tf_id: String,
    #[doc = "The human-readable name of the security settings, unique within the location."]
    pub display_name: PrimField<String>,
    #[doc = "The location these settings are located in. Settings can only be applied to an agent in the same location.\nSee [Available Regions](https://cloud.google.com/dialogflow/cx/docs/concept/region#avail) for a list of supported locations."]
    pub location: PrimField<String>,
}
impl BuildDialogflowCxSecuritySettings {
    pub fn build(self, stack: &mut Stack) -> DialogflowCxSecuritySettings {
        let out = DialogflowCxSecuritySettings(Rc::new(DialogflowCxSecuritySettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DialogflowCxSecuritySettingsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deidentify_template: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                inspect_template: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                purge_data_types: core::default::Default::default(),
                redaction_scope: core::default::Default::default(),
                redaction_strategy: core::default::Default::default(),
                retention_strategy: core::default::Default::default(),
                retention_window_days: core::default::Default::default(),
                audio_export_settings: core::default::Default::default(),
                insights_export_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DialogflowCxSecuritySettingsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxSecuritySettingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DialogflowCxSecuritySettingsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deidentify_template` after provisioning.\n[DLP](https://cloud.google.com/dlp/docs) deidentify template name. Use this template to define de-identification configuration for the content. If empty, Dialogflow replaces sensitive info with [redacted] text.\nNote: deidentifyTemplate must be located in the same region as the SecuritySettings.\nFormat: projects/<Project ID>/locations/<Location ID>/deidentifyTemplates/<Template ID> OR organizations/<Organization ID>/locations/<Location ID>/deidentifyTemplates/<Template ID>"]
    pub fn deidentify_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deidentify_template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the security settings, unique within the location."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `inspect_template` after provisioning.\n[DLP](https://cloud.google.com/dlp/docs) inspect template name. Use this template to define inspect base settings. If empty, we use the default DLP inspect config.\nNote: inspectTemplate must be located in the same region as the SecuritySettings.\nFormat: projects/<Project ID>/locations/<Location ID>/inspectTemplates/<Template ID> OR organizations/<Organization ID>/locations/<Location ID>/inspectTemplates/<Template ID>"]
    pub fn inspect_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inspect_template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location these settings are located in. Settings can only be applied to an agent in the same location.\nSee [Available Regions](https://cloud.google.com/dialogflow/cx/docs/concept/region#avail) for a list of supported locations."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the settings.\nFormat: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>."]
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
    #[doc = "Get a reference to the value of field `purge_data_types` after provisioning.\nList of types of data to remove when retention settings triggers purge. Possible values: [\"DIALOGFLOW_HISTORY\"]"]
    pub fn purge_data_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.purge_data_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redaction_scope` after provisioning.\nDefines what types of data to redact. If not set, defaults to not redacting any kind of data.\n* REDACT_DISK_STORAGE: On data to be written to disk or similar devices that are capable of holding data even if power is disconnected. This includes data that are temporarily saved on disk. Possible values: [\"REDACT_DISK_STORAGE\"]"]
    pub fn redaction_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redaction_scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redaction_strategy` after provisioning.\nDefines how we redact data. If not set, defaults to not redacting.\n* REDACT_WITH_SERVICE: Call redaction service to clean up the data to be persisted. Possible values: [\"REDACT_WITH_SERVICE\"]"]
    pub fn redaction_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redaction_strategy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retention_strategy` after provisioning.\nDefines how long we retain persisted data that contains sensitive info. Only one of 'retention_window_days' and 'retention_strategy' may be set.\n* REMOVE_AFTER_CONVERSATION: Removes data when the conversation ends. If there is no conversation explicitly established, a default conversation ends when the corresponding Dialogflow session ends. Possible values: [\"REMOVE_AFTER_CONVERSATION\"]"]
    pub fn retention_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention_strategy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retention_window_days` after provisioning.\nRetains the data for the specified number of days. User must set a value lower than Dialogflow's default 365d TTL (30 days for Agent Assist traffic), higher value will be ignored and use default. Setting a value higher than that has no effect. A missing value or setting to 0 also means we use default TTL.\nOnly one of 'retention_window_days' and 'retention_strategy' may be set."]
    pub fn retention_window_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention_window_days", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `audio_export_settings` after provisioning.\n"]
    pub fn audio_export_settings(
        &self,
    ) -> ListRef<DialogflowCxSecuritySettingsAudioExportSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.audio_export_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `insights_export_settings` after provisioning.\n"]
    pub fn insights_export_settings(
        &self,
    ) -> ListRef<DialogflowCxSecuritySettingsInsightsExportSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.insights_export_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxSecuritySettingsTimeoutsElRef {
        DialogflowCxSecuritySettingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxSecuritySettingsAudioExportSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_export_pattern: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_audio_redaction: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_bucket: Option<PrimField<String>>,
}
impl DialogflowCxSecuritySettingsAudioExportSettingsEl {
    #[doc = "Set the field `audio_export_pattern`.\nFilename pattern for exported audio."]
    pub fn set_audio_export_pattern(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.audio_export_pattern = Some(v.into());
        self
    }
    #[doc = "Set the field `audio_format`.\nFile format for exported audio file. Currently only in telephony recordings.\n* MULAW: G.711 mu-law PCM with 8kHz sample rate.\n* MP3: MP3 file format.\n* OGG: OGG Vorbis. Possible values: [\"MULAW\", \"MP3\", \"OGG\"]"]
    pub fn set_audio_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.audio_format = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_audio_redaction`.\nEnable audio redaction if it is true."]
    pub fn set_enable_audio_redaction(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_audio_redaction = Some(v.into());
        self
    }
    #[doc = "Set the field `gcs_bucket`.\nCloud Storage bucket to export audio record to. Setting this field would grant the Storage Object Creator role to the Dialogflow Service Agent. API caller that tries to modify this field should have the permission of storage.buckets.setIamPolicy."]
    pub fn set_gcs_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcs_bucket = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxSecuritySettingsAudioExportSettingsEl {
    type O = BlockAssignable<DialogflowCxSecuritySettingsAudioExportSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxSecuritySettingsAudioExportSettingsEl {}
impl BuildDialogflowCxSecuritySettingsAudioExportSettingsEl {
    pub fn build(self) -> DialogflowCxSecuritySettingsAudioExportSettingsEl {
        DialogflowCxSecuritySettingsAudioExportSettingsEl {
            audio_export_pattern: core::default::Default::default(),
            audio_format: core::default::Default::default(),
            enable_audio_redaction: core::default::Default::default(),
            gcs_bucket: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxSecuritySettingsAudioExportSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxSecuritySettingsAudioExportSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxSecuritySettingsAudioExportSettingsElRef {
        DialogflowCxSecuritySettingsAudioExportSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxSecuritySettingsAudioExportSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `audio_export_pattern` after provisioning.\nFilename pattern for exported audio."]
    pub fn audio_export_pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.audio_export_pattern", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `audio_format` after provisioning.\nFile format for exported audio file. Currently only in telephony recordings.\n* MULAW: G.711 mu-law PCM with 8kHz sample rate.\n* MP3: MP3 file format.\n* OGG: OGG Vorbis. Possible values: [\"MULAW\", \"MP3\", \"OGG\"]"]
    pub fn audio_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.audio_format", self.base))
    }
    #[doc = "Get a reference to the value of field `enable_audio_redaction` after provisioning.\nEnable audio redaction if it is true."]
    pub fn enable_audio_redaction(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_audio_redaction", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_bucket` after provisioning.\nCloud Storage bucket to export audio record to. Setting this field would grant the Storage Object Creator role to the Dialogflow Service Agent. API caller that tries to modify this field should have the permission of storage.buckets.setIamPolicy."]
    pub fn gcs_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gcs_bucket", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxSecuritySettingsInsightsExportSettingsEl {
    enable_insights_export: PrimField<bool>,
}
impl DialogflowCxSecuritySettingsInsightsExportSettingsEl {}
impl ToListMappable for DialogflowCxSecuritySettingsInsightsExportSettingsEl {
    type O = BlockAssignable<DialogflowCxSecuritySettingsInsightsExportSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxSecuritySettingsInsightsExportSettingsEl {
    #[doc = "If enabled, we will automatically exports conversations to Insights and Insights runs its analyzers."]
    pub enable_insights_export: PrimField<bool>,
}
impl BuildDialogflowCxSecuritySettingsInsightsExportSettingsEl {
    pub fn build(self) -> DialogflowCxSecuritySettingsInsightsExportSettingsEl {
        DialogflowCxSecuritySettingsInsightsExportSettingsEl {
            enable_insights_export: self.enable_insights_export,
        }
    }
}
pub struct DialogflowCxSecuritySettingsInsightsExportSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxSecuritySettingsInsightsExportSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxSecuritySettingsInsightsExportSettingsElRef {
        DialogflowCxSecuritySettingsInsightsExportSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxSecuritySettingsInsightsExportSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_insights_export` after provisioning.\nIf enabled, we will automatically exports conversations to Insights and Insights runs its analyzers."]
    pub fn enable_insights_export(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_insights_export", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxSecuritySettingsTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DialogflowCxSecuritySettingsTimeoutsEl {
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
impl ToListMappable for DialogflowCxSecuritySettingsTimeoutsEl {
    type O = BlockAssignable<DialogflowCxSecuritySettingsTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxSecuritySettingsTimeoutsEl {}
impl BuildDialogflowCxSecuritySettingsTimeoutsEl {
    pub fn build(self) -> DialogflowCxSecuritySettingsTimeoutsEl {
        DialogflowCxSecuritySettingsTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxSecuritySettingsTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxSecuritySettingsTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxSecuritySettingsTimeoutsElRef {
        DialogflowCxSecuritySettingsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxSecuritySettingsTimeoutsElRef {
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
struct DialogflowCxSecuritySettingsDynamic {
    audio_export_settings: Option<DynamicBlock<DialogflowCxSecuritySettingsAudioExportSettingsEl>>,
    insights_export_settings:
        Option<DynamicBlock<DialogflowCxSecuritySettingsInsightsExportSettingsEl>>,
}
