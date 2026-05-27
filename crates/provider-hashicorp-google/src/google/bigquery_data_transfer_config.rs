use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BigqueryDataTransferConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_refresh_window_days: Option<PrimField<f64>>,
    data_source_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_dataset_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_pubsub_topic: Option<PrimField<String>>,
    params: RecField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_preferences: Option<Vec<BigqueryDataTransferConfigEmailPreferencesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_configuration: Option<Vec<BigqueryDataTransferConfigEncryptionConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_options: Option<Vec<BigqueryDataTransferConfigScheduleOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sensitive_params: Option<Vec<BigqueryDataTransferConfigSensitiveParamsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BigqueryDataTransferConfigTimeoutsEl>,
    dynamic: BigqueryDataTransferConfigDynamic,
}
struct BigqueryDataTransferConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BigqueryDataTransferConfigData>,
}
#[derive(Clone)]
pub struct BigqueryDataTransferConfig(Rc<BigqueryDataTransferConfig_>);
impl BigqueryDataTransferConfig {
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
    #[doc = "Set the field `data_refresh_window_days`.\nThe number of days to look back to automatically refresh the data.\nFor example, if dataRefreshWindowDays = 10, then every day BigQuery\nreingests data for [today-10, today-1], rather than ingesting data for\njust [today-1]. Only valid if the data source supports the feature.\nSet the value to 0 to use the default value."]
    pub fn set_data_refresh_window_days(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().data_refresh_window_days = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `destination_dataset_id`.\nThe BigQuery target dataset id."]
    pub fn set_destination_dataset_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().destination_dataset_id = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nWhen set to true, no runs are scheduled for a given transfer."]
    pub fn set_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe geographic location where the transfer config should reside.\nExamples: US, EU, asia-northeast1. The default value is US."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `notification_pubsub_topic`.\nPub/Sub topic where notifications will be sent after transfer runs\nassociated with this transfer config finish."]
    pub fn set_notification_pubsub_topic(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().notification_pubsub_topic = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `schedule`.\nData transfer schedule. If the data source does not support a custom\nschedule, this should be empty. If it is empty, the default value for\nthe data source will be used. The specified times are in UTC. Examples\nof valid format: 1st,3rd monday of month 15:30, every wed,fri of jan,\njun 13:15, and first sunday of quarter 00:00. See more explanation\nabout the format here:\nhttps://cloud.google.com/appengine/docs/flexible/python/scheduling-jobs-with-cron-yaml#the_schedule_format\nNOTE: The minimum interval time between recurring transfers depends\non the data source; refer to the documentation for your data source."]
    pub fn set_schedule(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().schedule = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account_name`.\nService account email. If this field is set, transfer config will\nbe created with this service account credentials. It requires that\nrequesting user calling this API has permissions to act as this service account."]
    pub fn set_service_account_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_account_name = Some(v.into());
        self
    }
    #[doc = "Set the field `email_preferences`.\n"]
    pub fn set_email_preferences(
        self,
        v: impl Into<BlockAssignable<BigqueryDataTransferConfigEmailPreferencesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().email_preferences = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.email_preferences = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        self,
        v: impl Into<BlockAssignable<BigqueryDataTransferConfigEncryptionConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `schedule_options`.\n"]
    pub fn set_schedule_options(
        self,
        v: impl Into<BlockAssignable<BigqueryDataTransferConfigScheduleOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schedule_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schedule_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sensitive_params`.\n"]
    pub fn set_sensitive_params(
        self,
        v: impl Into<BlockAssignable<BigqueryDataTransferConfigSensitiveParamsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sensitive_params = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sensitive_params = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BigqueryDataTransferConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `data_refresh_window_days` after provisioning.\nThe number of days to look back to automatically refresh the data.\nFor example, if dataRefreshWindowDays = 10, then every day BigQuery\nreingests data for [today-10, today-1], rather than ingesting data for\njust [today-1]. Only valid if the data source supports the feature.\nSet the value to 0 to use the default value."]
    pub fn data_refresh_window_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_refresh_window_days", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\nThe data source id. Cannot be changed once the transfer config is created."]
    pub fn data_source_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination_dataset_id` after provisioning.\nThe BigQuery target dataset id."]
    pub fn destination_dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_dataset_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhen set to true, no runs are scheduled for a given transfer."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe user specified display name for the transfer config."]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the transfer config should reside.\nExamples: US, EU, asia-northeast1. The default value is US."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the transfer config. Transfer config names have the\nform projects/{projectId}/locations/{location}/transferConfigs/{configId}\nor projects/{projectId}/transferConfigs/{configId},\nwhere configId is usually a uuid, but this is not required.\nThe name is ignored when creating a transfer config."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notification_pubsub_topic` after provisioning.\nPub/Sub topic where notifications will be sent after transfer runs\nassociated with this transfer config finish."]
    pub fn notification_pubsub_topic(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notification_pubsub_topic", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\nParameters specific to each data source. For more information see the bq tab in the 'Setting up a data transfer'\nsection for each data source. For example the parameters for Cloud Storage transfers are listed here:\nhttps://cloud.google.com/bigquery-transfer/docs/cloud-storage-transfer#bq\n\n**NOTE** : If you are attempting to update a parameter that cannot be updated (due to api limitations) [please force recreation of the resource](https://www.terraform.io/cli/state/taint#forcing-re-creation-of-resources)."]
    pub fn params(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\nData transfer schedule. If the data source does not support a custom\nschedule, this should be empty. If it is empty, the default value for\nthe data source will be used. The specified times are in UTC. Examples\nof valid format: 1st,3rd monday of month 15:30, every wed,fri of jan,\njun 13:15, and first sunday of quarter 00:00. See more explanation\nabout the format here:\nhttps://cloud.google.com/appengine/docs/flexible/python/scheduling-jobs-with-cron-yaml#the_schedule_format\nNOTE: The minimum interval time between recurring transfers depends\non the data source; refer to the documentation for your data source."]
    pub fn schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account_name` after provisioning.\nService account email. If this field is set, transfer config will\nbe created with this service account credentials. It requires that\nrequesting user calling this API has permissions to act as this service account."]
    pub fn service_account_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `email_preferences` after provisioning.\n"]
    pub fn email_preferences(&self) -> ListRef<BigqueryDataTransferConfigEmailPreferencesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.email_preferences", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> ListRef<BigqueryDataTransferConfigEncryptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schedule_options` after provisioning.\n"]
    pub fn schedule_options(&self) -> ListRef<BigqueryDataTransferConfigScheduleOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schedule_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sensitive_params` after provisioning.\n"]
    pub fn sensitive_params(&self) -> ListRef<BigqueryDataTransferConfigSensitiveParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sensitive_params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryDataTransferConfigTimeoutsElRef {
        BigqueryDataTransferConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BigqueryDataTransferConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BigqueryDataTransferConfig {}
impl ToListMappable for BigqueryDataTransferConfig {
    type O = ListRef<BigqueryDataTransferConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BigqueryDataTransferConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_bigquery_data_transfer_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBigqueryDataTransferConfig {
    pub tf_id: String,
    #[doc = "The data source id. Cannot be changed once the transfer config is created."]
    pub data_source_id: PrimField<String>,
    #[doc = "The user specified display name for the transfer config."]
    pub display_name: PrimField<String>,
    #[doc = "Parameters specific to each data source. For more information see the bq tab in the 'Setting up a data transfer'\nsection for each data source. For example the parameters for Cloud Storage transfers are listed here:\nhttps://cloud.google.com/bigquery-transfer/docs/cloud-storage-transfer#bq\n\n**NOTE** : If you are attempting to update a parameter that cannot be updated (due to api limitations) [please force recreation of the resource](https://www.terraform.io/cli/state/taint#forcing-re-creation-of-resources)."]
    pub params: RecField<PrimField<String>>,
}
impl BuildBigqueryDataTransferConfig {
    pub fn build(self, stack: &mut Stack) -> BigqueryDataTransferConfig {
        let out = BigqueryDataTransferConfig(Rc::new(BigqueryDataTransferConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BigqueryDataTransferConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                data_refresh_window_days: core::default::Default::default(),
                data_source_id: self.data_source_id,
                deletion_policy: core::default::Default::default(),
                destination_dataset_id: core::default::Default::default(),
                disabled: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                notification_pubsub_topic: core::default::Default::default(),
                params: self.params,
                project: core::default::Default::default(),
                schedule: core::default::Default::default(),
                service_account_name: core::default::Default::default(),
                email_preferences: core::default::Default::default(),
                encryption_configuration: core::default::Default::default(),
                schedule_options: core::default::Default::default(),
                sensitive_params: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BigqueryDataTransferConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDataTransferConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BigqueryDataTransferConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_refresh_window_days` after provisioning.\nThe number of days to look back to automatically refresh the data.\nFor example, if dataRefreshWindowDays = 10, then every day BigQuery\nreingests data for [today-10, today-1], rather than ingesting data for\njust [today-1]. Only valid if the data source supports the feature.\nSet the value to 0 to use the default value."]
    pub fn data_refresh_window_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_refresh_window_days", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\nThe data source id. Cannot be changed once the transfer config is created."]
    pub fn data_source_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination_dataset_id` after provisioning.\nThe BigQuery target dataset id."]
    pub fn destination_dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_dataset_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhen set to true, no runs are scheduled for a given transfer."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe user specified display name for the transfer config."]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the transfer config should reside.\nExamples: US, EU, asia-northeast1. The default value is US."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the transfer config. Transfer config names have the\nform projects/{projectId}/locations/{location}/transferConfigs/{configId}\nor projects/{projectId}/transferConfigs/{configId},\nwhere configId is usually a uuid, but this is not required.\nThe name is ignored when creating a transfer config."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notification_pubsub_topic` after provisioning.\nPub/Sub topic where notifications will be sent after transfer runs\nassociated with this transfer config finish."]
    pub fn notification_pubsub_topic(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notification_pubsub_topic", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\nParameters specific to each data source. For more information see the bq tab in the 'Setting up a data transfer'\nsection for each data source. For example the parameters for Cloud Storage transfers are listed here:\nhttps://cloud.google.com/bigquery-transfer/docs/cloud-storage-transfer#bq\n\n**NOTE** : If you are attempting to update a parameter that cannot be updated (due to api limitations) [please force recreation of the resource](https://www.terraform.io/cli/state/taint#forcing-re-creation-of-resources)."]
    pub fn params(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\nData transfer schedule. If the data source does not support a custom\nschedule, this should be empty. If it is empty, the default value for\nthe data source will be used. The specified times are in UTC. Examples\nof valid format: 1st,3rd monday of month 15:30, every wed,fri of jan,\njun 13:15, and first sunday of quarter 00:00. See more explanation\nabout the format here:\nhttps://cloud.google.com/appengine/docs/flexible/python/scheduling-jobs-with-cron-yaml#the_schedule_format\nNOTE: The minimum interval time between recurring transfers depends\non the data source; refer to the documentation for your data source."]
    pub fn schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account_name` after provisioning.\nService account email. If this field is set, transfer config will\nbe created with this service account credentials. It requires that\nrequesting user calling this API has permissions to act as this service account."]
    pub fn service_account_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `email_preferences` after provisioning.\n"]
    pub fn email_preferences(&self) -> ListRef<BigqueryDataTransferConfigEmailPreferencesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.email_preferences", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> ListRef<BigqueryDataTransferConfigEncryptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schedule_options` after provisioning.\n"]
    pub fn schedule_options(&self) -> ListRef<BigqueryDataTransferConfigScheduleOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schedule_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sensitive_params` after provisioning.\n"]
    pub fn sensitive_params(&self) -> ListRef<BigqueryDataTransferConfigSensitiveParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sensitive_params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryDataTransferConfigTimeoutsElRef {
        BigqueryDataTransferConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryDataTransferConfigEmailPreferencesEl {
    enable_failure_email: PrimField<bool>,
}
impl BigqueryDataTransferConfigEmailPreferencesEl {}
impl ToListMappable for BigqueryDataTransferConfigEmailPreferencesEl {
    type O = BlockAssignable<BigqueryDataTransferConfigEmailPreferencesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryDataTransferConfigEmailPreferencesEl {
    #[doc = "If true, email notifications will be sent on transfer run failures."]
    pub enable_failure_email: PrimField<bool>,
}
impl BuildBigqueryDataTransferConfigEmailPreferencesEl {
    pub fn build(self) -> BigqueryDataTransferConfigEmailPreferencesEl {
        BigqueryDataTransferConfigEmailPreferencesEl {
            enable_failure_email: self.enable_failure_email,
        }
    }
}
pub struct BigqueryDataTransferConfigEmailPreferencesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDataTransferConfigEmailPreferencesElRef {
    fn new(shared: StackShared, base: String) -> BigqueryDataTransferConfigEmailPreferencesElRef {
        BigqueryDataTransferConfigEmailPreferencesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryDataTransferConfigEmailPreferencesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_failure_email` after provisioning.\nIf true, email notifications will be sent on transfer run failures."]
    pub fn enable_failure_email(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_failure_email", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryDataTransferConfigEncryptionConfigurationEl {
    kms_key_name: PrimField<String>,
}
impl BigqueryDataTransferConfigEncryptionConfigurationEl {}
impl ToListMappable for BigqueryDataTransferConfigEncryptionConfigurationEl {
    type O = BlockAssignable<BigqueryDataTransferConfigEncryptionConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryDataTransferConfigEncryptionConfigurationEl {
    #[doc = "The name of the KMS key used for encrypting BigQuery data."]
    pub kms_key_name: PrimField<String>,
}
impl BuildBigqueryDataTransferConfigEncryptionConfigurationEl {
    pub fn build(self) -> BigqueryDataTransferConfigEncryptionConfigurationEl {
        BigqueryDataTransferConfigEncryptionConfigurationEl {
            kms_key_name: self.kms_key_name,
        }
    }
}
pub struct BigqueryDataTransferConfigEncryptionConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDataTransferConfigEncryptionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BigqueryDataTransferConfigEncryptionConfigurationElRef {
        BigqueryDataTransferConfigEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryDataTransferConfigEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nThe name of the KMS key used for encrypting BigQuery data."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryDataTransferConfigScheduleOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_auto_scheduling: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl BigqueryDataTransferConfigScheduleOptionsEl {
    #[doc = "Set the field `disable_auto_scheduling`.\nIf true, automatic scheduling of data transfer runs for this\nconfiguration will be disabled. The runs can be started on ad-hoc\nbasis using transferConfigs.startManualRuns API. When automatic\nscheduling is disabled, the TransferConfig.schedule field will\nbe ignored."]
    pub fn set_disable_auto_scheduling(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_auto_scheduling = Some(v.into());
        self
    }
    #[doc = "Set the field `end_time`.\nDefines time to stop scheduling transfer runs. A transfer run cannot be\nscheduled at or after the end time. The end time can be changed at any\nmoment. The time when a data transfer can be triggered manually is not\nlimited by this option."]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\nSpecifies time to start scheduling transfer runs. The first run will be\nscheduled at or after the start time according to a recurrence pattern\ndefined in the schedule string. The start time can be changed at any\nmoment. The time when a data transfer can be triggered manually is not\nlimited by this option."]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryDataTransferConfigScheduleOptionsEl {
    type O = BlockAssignable<BigqueryDataTransferConfigScheduleOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryDataTransferConfigScheduleOptionsEl {}
impl BuildBigqueryDataTransferConfigScheduleOptionsEl {
    pub fn build(self) -> BigqueryDataTransferConfigScheduleOptionsEl {
        BigqueryDataTransferConfigScheduleOptionsEl {
            disable_auto_scheduling: core::default::Default::default(),
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct BigqueryDataTransferConfigScheduleOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDataTransferConfigScheduleOptionsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryDataTransferConfigScheduleOptionsElRef {
        BigqueryDataTransferConfigScheduleOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryDataTransferConfigScheduleOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_auto_scheduling` after provisioning.\nIf true, automatic scheduling of data transfer runs for this\nconfiguration will be disabled. The runs can be started on ad-hoc\nbasis using transferConfigs.startManualRuns API. When automatic\nscheduling is disabled, the TransferConfig.schedule field will\nbe ignored."]
    pub fn disable_auto_scheduling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_auto_scheduling", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\nDefines time to stop scheduling transfer runs. A transfer run cannot be\nscheduled at or after the end time. The end time can be changed at any\nmoment. The time when a data transfer can be triggered manually is not\nlimited by this option."]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\nSpecifies time to start scheduling transfer runs. The first run will be\nscheduled at or after the start time according to a recurrence pattern\ndefined in the schedule string. The start time can be changed at any\nmoment. The time when a data transfer can be triggered manually is not\nlimited by this option."]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryDataTransferConfigSensitiveParamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_access_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_access_key_wo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_access_key_wo_version: Option<PrimField<f64>>,
}
impl BigqueryDataTransferConfigSensitiveParamsEl {
    #[doc = "Set the field `secret_access_key`.\nThe Secret Access Key of the AWS account transferring data from."]
    pub fn set_secret_access_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_access_key = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_access_key_wo`.\nThe Secret Access Key of the AWS account transferring data from."]
    pub fn set_secret_access_key_wo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_access_key_wo = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_access_key_wo_version`.\nThe version of the sensitive params - used to trigger updates of the write-only params. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn set_secret_access_key_wo_version(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.secret_access_key_wo_version = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryDataTransferConfigSensitiveParamsEl {
    type O = BlockAssignable<BigqueryDataTransferConfigSensitiveParamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryDataTransferConfigSensitiveParamsEl {}
impl BuildBigqueryDataTransferConfigSensitiveParamsEl {
    pub fn build(self) -> BigqueryDataTransferConfigSensitiveParamsEl {
        BigqueryDataTransferConfigSensitiveParamsEl {
            secret_access_key: core::default::Default::default(),
            secret_access_key_wo: core::default::Default::default(),
            secret_access_key_wo_version: core::default::Default::default(),
        }
    }
}
pub struct BigqueryDataTransferConfigSensitiveParamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDataTransferConfigSensitiveParamsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryDataTransferConfigSensitiveParamsElRef {
        BigqueryDataTransferConfigSensitiveParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryDataTransferConfigSensitiveParamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret_access_key` after provisioning.\nThe Secret Access Key of the AWS account transferring data from."]
    pub fn secret_access_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_access_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secret_access_key_wo` after provisioning.\nThe Secret Access Key of the AWS account transferring data from."]
    pub fn secret_access_key_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_access_key_wo", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secret_access_key_wo_version` after provisioning.\nThe version of the sensitive params - used to trigger updates of the write-only params. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn secret_access_key_wo_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_access_key_wo_version", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryDataTransferConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BigqueryDataTransferConfigTimeoutsEl {
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
impl ToListMappable for BigqueryDataTransferConfigTimeoutsEl {
    type O = BlockAssignable<BigqueryDataTransferConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryDataTransferConfigTimeoutsEl {}
impl BuildBigqueryDataTransferConfigTimeoutsEl {
    pub fn build(self) -> BigqueryDataTransferConfigTimeoutsEl {
        BigqueryDataTransferConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BigqueryDataTransferConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDataTransferConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryDataTransferConfigTimeoutsElRef {
        BigqueryDataTransferConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryDataTransferConfigTimeoutsElRef {
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
struct BigqueryDataTransferConfigDynamic {
    email_preferences: Option<DynamicBlock<BigqueryDataTransferConfigEmailPreferencesEl>>,
    encryption_configuration:
        Option<DynamicBlock<BigqueryDataTransferConfigEncryptionConfigurationEl>>,
    schedule_options: Option<DynamicBlock<BigqueryDataTransferConfigScheduleOptionsEl>>,
    sensitive_params: Option<DynamicBlock<BigqueryDataTransferConfigSensitiveParamsEl>>,
}
