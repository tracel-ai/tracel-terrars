use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DiscoveryEngineDataConnectorData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_run_disabled: Option<PrimField<bool>>,
    collection_display_name: PrimField<String>,
    collection_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connector_modes: Option<ListField<PrimField<String>>>,
    data_source: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_source_version: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    incremental_refresh_interval: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    incremental_sync_disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json_params: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    refresh_interval: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    static_ip_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action_config: Option<Vec<DiscoveryEngineDataConnectorActionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bap_config: Option<Vec<DiscoveryEngineDataConnectorBapConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_configs: Option<Vec<DiscoveryEngineDataConnectorDestinationConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<Vec<DiscoveryEngineDataConnectorEntitiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DiscoveryEngineDataConnectorTimeoutsEl>,
    dynamic: DiscoveryEngineDataConnectorDynamic,
}
struct DiscoveryEngineDataConnector_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DiscoveryEngineDataConnectorData>,
}
#[derive(Clone)]
pub struct DiscoveryEngineDataConnector(Rc<DiscoveryEngineDataConnector_>);
impl DiscoveryEngineDataConnector {
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
    #[doc = "Set the field `auto_run_disabled`.\nIndicates whether full syncs are paused for this connector"]
    pub fn set_auto_run_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_run_disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `connector_modes`.\nThe modes enabled for this connector. The possible value can be:\n'DATA_INGESTION', 'ACTIONS', 'FEDERATED'\n'EUA', 'FEDERATED_AND_EUA'."]
    pub fn set_connector_modes(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().connector_modes = Some(v.into());
        self
    }
    #[doc = "Set the field `data_source_version`.\nThe version of the data source. For example, '3' for Jira v3."]
    pub fn set_data_source_version(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().data_source_version = Some(v.into());
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
    #[doc = "Set the field `incremental_refresh_interval`.\nThe refresh interval specifically for incremental data syncs. If unset,\nincremental syncs will use the default from env, set to 3hrs.\nThe minimum is 30 minutes and maximum is 7 days. Applicable to only 3P\nconnectors. When the refresh interval is\nset to the same value as the incremental refresh interval, incremental\nsync will be disabled."]
    pub fn set_incremental_refresh_interval(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().incremental_refresh_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `incremental_sync_disabled`.\nIndicates whether incremental syncs are paused for this connector."]
    pub fn set_incremental_sync_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().incremental_sync_disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `json_params`.\nParams needed to access the source in the format of json string."]
    pub fn set_json_params(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().json_params = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_name`.\nThe KMS key to be used to protect the DataStores managed by this connector.\nMust be set for requests that need to comply with CMEK Org Policy\nprotections.\nIf this field is set and processed successfully, the DataStores created by\nthis connector will be protected by the KMS key."]
    pub fn set_kms_key_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `params`.\nParams needed to access the source in the format of String-to-String (Key, Value) pairs."]
    pub fn set_params(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().params = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `static_ip_enabled`.\nWhether customer has enabled static IP addresses for this connector."]
    pub fn set_static_ip_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().static_ip_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `sync_mode`.\nThe data synchronization mode supported by the data connector. The possible value can be:\n'PERIODIC', 'STREAMING'."]
    pub fn set_sync_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sync_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `action_config`.\n"]
    pub fn set_action_config(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineDataConnectorActionConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().action_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.action_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `bap_config`.\n"]
    pub fn set_bap_config(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineDataConnectorBapConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bap_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bap_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `destination_configs`.\n"]
    pub fn set_destination_configs(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineDataConnectorDestinationConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_configs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `entities`.\n"]
    pub fn set_entities(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineDataConnectorEntitiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().entities = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.entities = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DiscoveryEngineDataConnectorTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `action_state` after provisioning.\nState of the action connector. This reflects whether the action connector\nis initializing, active or has encountered errors. The possible value can be:\n'STATE_UNSPECIFIED', 'CREATING', 'ACTIVE', 'FAILED', 'RUNNING', 'WARNING',\n'INITIALIZATION_FAILED', 'UPDATING'."]
    pub fn action_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_run_disabled` after provisioning.\nIndicates whether full syncs are paused for this connector"]
    pub fn auto_run_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_run_disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `blocking_reasons` after provisioning.\nUser actions that must be completed before the connector can start syncing data.\nThe possible values can be: 'ALLOWLIST_STATIC_IP', 'ALLOWLIST_IN_SERVICE_ATTACHMENT'."]
    pub fn blocking_reasons(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.blocking_reasons", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `collection_display_name` after provisioning.\nThe display name of the Collection.\nShould be human readable, used to display collections in the Console\nDashboard. UTF-8 encoded string with limit of 1024 characters."]
    pub fn collection_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collection_display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `collection_id` after provisioning.\nThe ID to use for the Collection, which will become the final component\nof the Collection's resource name. A new Collection is created as\npart of the DataConnector setup. DataConnector is a singleton\nresource under Collection, managing all DataStores of the Collection.\nThis field must conform to [RFC-1034](https://tools.ietf.org/html/rfc1034)\nstandard with a length limit of 63 characters. Otherwise, an\nINVALID_ARGUMENT error is returned."]
    pub fn collection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collection_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connector_modes` after provisioning.\nThe modes enabled for this connector. The possible value can be:\n'DATA_INGESTION', 'ACTIONS', 'FEDERATED'\n'EUA', 'FEDERATED_AND_EUA'."]
    pub fn connector_modes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connector_modes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connector_type` after provisioning.\nThe type of connector. Each source can only map to one type.\nFor example, salesforce, confluence and jira have THIRD_PARTY connector\ntype. It is not mutable once set by system. The possible value can be:\n'CONNECTOR_TYPE_UNSPECIFIED', 'THIRD_PARTY', 'GCP_FHIR', 'BIG_QUERY',\n'GCS', 'GOOGLE_MAIL', 'GOOGLE_CALENDAR', 'GOOGLE_DRIVE',\n'NATIVE_CLOUD_IDENTITY', 'THIRD_PARTY_FEDERATED', 'THIRD_PARTY_EUA', 'GCNV'."]
    pub fn connector_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connector_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the DataConnector was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_source` after provisioning.\nThe identifier for the data source.\nThis is a partial list of supported connectors. Please refer to the\n[documentation](https://docs.cloud.google.com/gemini/enterprise/docs/connectors/introduction-to-connectors-and-data-stores)\nfor the full list of connectors.\n\nSupported first-party connectors include:\n\n*   'bigquery'\n*   'gcp_fhir'\n*   'google_mail'\n*   'google_drive'\n*   'google_calendar'\n*   'google_chat'\n\nSupported third-party connectors include:\nGenerally available (GA) connectors:\n\n*   'onedrive'\n*   'outlook'\n*   'confluence'\n*   'jira'\n*   'servicenow'\n*   'sharepoint'\n\nPreview connectors:\n\n*   'asana'\n*   'azure_active_directory'\n*   'box'\n*   'canva'\n*   'confluence_server'\n*   'custom_connector'\n*   'docusign'\n*   'dropbox'\n*   'dynamics365'\n*   'github'\n*   'gitlab'\n*   'hubspot'\n*   'jira_server'\n*   'linear'\n*   'native_cloud_identity'\n*   'notion'\n*   'okta'\n*   'pagerduty'\n*   'peoplesoft'\n*   'salesforce'\n*   'shopify'\n*   'slack'\n*   'snowflake'\n*   'teams'\n*   'trello'\n*   'workday'\n*   'zendesk'"]
    pub fn data_source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_source_version` after provisioning.\nThe version of the data source. For example, '3' for Jira v3."]
    pub fn data_source_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `errors` after provisioning.\nThe errors from initialization or from the latest connector run."]
    pub fn errors(&self) -> ListRef<DiscoveryEngineDataConnectorErrorsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.errors", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `incremental_refresh_interval` after provisioning.\nThe refresh interval specifically for incremental data syncs. If unset,\nincremental syncs will use the default from env, set to 3hrs.\nThe minimum is 30 minutes and maximum is 7 days. Applicable to only 3P\nconnectors. When the refresh interval is\nset to the same value as the incremental refresh interval, incremental\nsync will be disabled."]
    pub fn incremental_refresh_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.incremental_refresh_interval", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `incremental_sync_disabled` after provisioning.\nIndicates whether incremental syncs are paused for this connector."]
    pub fn incremental_sync_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.incremental_sync_disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `json_params` after provisioning.\nParams needed to access the source in the format of json string."]
    pub fn json_params(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.json_params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nThe KMS key to be used to protect the DataStores managed by this connector.\nMust be set for requests that need to comply with CMEK Org Policy\nprotections.\nIf this field is set and processed successfully, the DataStores created by\nthis connector will be protected by the KMS key."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_sync_time` after provisioning.\nFor periodic connectors only, the last time a data sync was completed."]
    pub fn last_sync_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_sync_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `latest_pause_time` after provisioning.\nThe most recent timestamp when this [DataConnector][] was paused,\naffecting all functionalities such as data synchronization.\nPausing a connector has the following effects:\n  - All functionalities, including data synchronization, are halted.\n  - Any ongoing data synchronization job will be canceled.\n  - No future data synchronization runs will be scheduled nor can be\ntriggered."]
    pub fn latest_pause_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_pause_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe full resource name of the Data Connector.\nFormat: 'projects/*/locations/*/collections/*/dataConnector'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\nParams needed to access the source in the format of String-to-String (Key, Value) pairs."]
    pub fn params(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_connectivity_project_id` after provisioning.\nThe tenant project ID associated with private connectivity connectors.\nThis project must be allowlisted by in order for the connector to function."]
    pub fn private_connectivity_project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_connectivity_project_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `realtime_state` after provisioning.\nThe real-time sync state. The possible values can be:\n'STATE_UNSPECIFIED', 'CREATING', 'ACTIVE', 'FAILED', 'RUNNING', 'WARNING',\n'INITIALIZATION_FAILED', 'UPDATING'."]
    pub fn realtime_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.realtime_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `refresh_interval` after provisioning.\nThe refresh interval for data sync. If duration is set to 0, the data will\nbe synced in real time. The streaming feature is not supported yet. The\nminimum is 30 minutes and maximum is 7 days. When the refresh interval is\nset to the same value as the incremental refresh interval, incremental\nsync will be disabled."]
    pub fn refresh_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.refresh_interval", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of connector. The possible value can be:\n'STATE_UNSPECIFIED', 'CREATING', 'ACTIVE', 'FAILED', 'RUNNING', 'WARNING',\n'INITIALIZATION_FAILED', 'UPDATING'."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `static_ip_addresses` after provisioning.\nThe static IP addresses used by this connector."]
    pub fn static_ip_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.static_ip_addresses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `static_ip_enabled` after provisioning.\nWhether customer has enabled static IP addresses for this connector."]
    pub fn static_ip_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.static_ip_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sync_mode` after provisioning.\nThe data synchronization mode supported by the data connector. The possible value can be:\n'PERIODIC', 'STREAMING'."]
    pub fn sync_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sync_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the DataConnector was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `action_config` after provisioning.\n"]
    pub fn action_config(&self) -> ListRef<DiscoveryEngineDataConnectorActionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.action_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bap_config` after provisioning.\n"]
    pub fn bap_config(&self) -> ListRef<DiscoveryEngineDataConnectorBapConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bap_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination_configs` after provisioning.\n"]
    pub fn destination_configs(
        &self,
    ) -> ListRef<DiscoveryEngineDataConnectorDestinationConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entities` after provisioning.\n"]
    pub fn entities(&self) -> ListRef<DiscoveryEngineDataConnectorEntitiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.entities", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineDataConnectorTimeoutsElRef {
        DiscoveryEngineDataConnectorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DiscoveryEngineDataConnector {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DiscoveryEngineDataConnector {}
impl ToListMappable for DiscoveryEngineDataConnector {
    type O = ListRef<DiscoveryEngineDataConnectorRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DiscoveryEngineDataConnector_ {
    fn extract_resource_type(&self) -> String {
        "google_discovery_engine_data_connector".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDiscoveryEngineDataConnector {
    pub tf_id: String,
    #[doc = "The display name of the Collection.\nShould be human readable, used to display collections in the Console\nDashboard. UTF-8 encoded string with limit of 1024 characters."]
    pub collection_display_name: PrimField<String>,
    #[doc = "The ID to use for the Collection, which will become the final component\nof the Collection's resource name. A new Collection is created as\npart of the DataConnector setup. DataConnector is a singleton\nresource under Collection, managing all DataStores of the Collection.\nThis field must conform to [RFC-1034](https://tools.ietf.org/html/rfc1034)\nstandard with a length limit of 63 characters. Otherwise, an\nINVALID_ARGUMENT error is returned."]
    pub collection_id: PrimField<String>,
    #[doc = "The identifier for the data source.\nThis is a partial list of supported connectors. Please refer to the\n[documentation](https://docs.cloud.google.com/gemini/enterprise/docs/connectors/introduction-to-connectors-and-data-stores)\nfor the full list of connectors.\n\nSupported first-party connectors include:\n\n*   'bigquery'\n*   'gcp_fhir'\n*   'google_mail'\n*   'google_drive'\n*   'google_calendar'\n*   'google_chat'\n\nSupported third-party connectors include:\nGenerally available (GA) connectors:\n\n*   'onedrive'\n*   'outlook'\n*   'confluence'\n*   'jira'\n*   'servicenow'\n*   'sharepoint'\n\nPreview connectors:\n\n*   'asana'\n*   'azure_active_directory'\n*   'box'\n*   'canva'\n*   'confluence_server'\n*   'custom_connector'\n*   'docusign'\n*   'dropbox'\n*   'dynamics365'\n*   'github'\n*   'gitlab'\n*   'hubspot'\n*   'jira_server'\n*   'linear'\n*   'native_cloud_identity'\n*   'notion'\n*   'okta'\n*   'pagerduty'\n*   'peoplesoft'\n*   'salesforce'\n*   'shopify'\n*   'slack'\n*   'snowflake'\n*   'teams'\n*   'trello'\n*   'workday'\n*   'zendesk'"]
    pub data_source: PrimField<String>,
    #[doc = "The geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub location: PrimField<String>,
    #[doc = "The refresh interval for data sync. If duration is set to 0, the data will\nbe synced in real time. The streaming feature is not supported yet. The\nminimum is 30 minutes and maximum is 7 days. When the refresh interval is\nset to the same value as the incremental refresh interval, incremental\nsync will be disabled."]
    pub refresh_interval: PrimField<String>,
}
impl BuildDiscoveryEngineDataConnector {
    pub fn build(self, stack: &mut Stack) -> DiscoveryEngineDataConnector {
        let out = DiscoveryEngineDataConnector(Rc::new(DiscoveryEngineDataConnector_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DiscoveryEngineDataConnectorData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                auto_run_disabled: core::default::Default::default(),
                collection_display_name: self.collection_display_name,
                collection_id: self.collection_id,
                connector_modes: core::default::Default::default(),
                data_source: self.data_source,
                data_source_version: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                incremental_refresh_interval: core::default::Default::default(),
                incremental_sync_disabled: core::default::Default::default(),
                json_params: core::default::Default::default(),
                kms_key_name: core::default::Default::default(),
                location: self.location,
                params: core::default::Default::default(),
                project: core::default::Default::default(),
                refresh_interval: self.refresh_interval,
                static_ip_enabled: core::default::Default::default(),
                sync_mode: core::default::Default::default(),
                action_config: core::default::Default::default(),
                bap_config: core::default::Default::default(),
                destination_configs: core::default::Default::default(),
                entities: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DiscoveryEngineDataConnectorRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineDataConnectorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DiscoveryEngineDataConnectorRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action_state` after provisioning.\nState of the action connector. This reflects whether the action connector\nis initializing, active or has encountered errors. The possible value can be:\n'STATE_UNSPECIFIED', 'CREATING', 'ACTIVE', 'FAILED', 'RUNNING', 'WARNING',\n'INITIALIZATION_FAILED', 'UPDATING'."]
    pub fn action_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_run_disabled` after provisioning.\nIndicates whether full syncs are paused for this connector"]
    pub fn auto_run_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_run_disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `blocking_reasons` after provisioning.\nUser actions that must be completed before the connector can start syncing data.\nThe possible values can be: 'ALLOWLIST_STATIC_IP', 'ALLOWLIST_IN_SERVICE_ATTACHMENT'."]
    pub fn blocking_reasons(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.blocking_reasons", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `collection_display_name` after provisioning.\nThe display name of the Collection.\nShould be human readable, used to display collections in the Console\nDashboard. UTF-8 encoded string with limit of 1024 characters."]
    pub fn collection_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collection_display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `collection_id` after provisioning.\nThe ID to use for the Collection, which will become the final component\nof the Collection's resource name. A new Collection is created as\npart of the DataConnector setup. DataConnector is a singleton\nresource under Collection, managing all DataStores of the Collection.\nThis field must conform to [RFC-1034](https://tools.ietf.org/html/rfc1034)\nstandard with a length limit of 63 characters. Otherwise, an\nINVALID_ARGUMENT error is returned."]
    pub fn collection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collection_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connector_modes` after provisioning.\nThe modes enabled for this connector. The possible value can be:\n'DATA_INGESTION', 'ACTIONS', 'FEDERATED'\n'EUA', 'FEDERATED_AND_EUA'."]
    pub fn connector_modes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connector_modes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connector_type` after provisioning.\nThe type of connector. Each source can only map to one type.\nFor example, salesforce, confluence and jira have THIRD_PARTY connector\ntype. It is not mutable once set by system. The possible value can be:\n'CONNECTOR_TYPE_UNSPECIFIED', 'THIRD_PARTY', 'GCP_FHIR', 'BIG_QUERY',\n'GCS', 'GOOGLE_MAIL', 'GOOGLE_CALENDAR', 'GOOGLE_DRIVE',\n'NATIVE_CLOUD_IDENTITY', 'THIRD_PARTY_FEDERATED', 'THIRD_PARTY_EUA', 'GCNV'."]
    pub fn connector_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connector_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the DataConnector was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_source` after provisioning.\nThe identifier for the data source.\nThis is a partial list of supported connectors. Please refer to the\n[documentation](https://docs.cloud.google.com/gemini/enterprise/docs/connectors/introduction-to-connectors-and-data-stores)\nfor the full list of connectors.\n\nSupported first-party connectors include:\n\n*   'bigquery'\n*   'gcp_fhir'\n*   'google_mail'\n*   'google_drive'\n*   'google_calendar'\n*   'google_chat'\n\nSupported third-party connectors include:\nGenerally available (GA) connectors:\n\n*   'onedrive'\n*   'outlook'\n*   'confluence'\n*   'jira'\n*   'servicenow'\n*   'sharepoint'\n\nPreview connectors:\n\n*   'asana'\n*   'azure_active_directory'\n*   'box'\n*   'canva'\n*   'confluence_server'\n*   'custom_connector'\n*   'docusign'\n*   'dropbox'\n*   'dynamics365'\n*   'github'\n*   'gitlab'\n*   'hubspot'\n*   'jira_server'\n*   'linear'\n*   'native_cloud_identity'\n*   'notion'\n*   'okta'\n*   'pagerduty'\n*   'peoplesoft'\n*   'salesforce'\n*   'shopify'\n*   'slack'\n*   'snowflake'\n*   'teams'\n*   'trello'\n*   'workday'\n*   'zendesk'"]
    pub fn data_source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_source_version` after provisioning.\nThe version of the data source. For example, '3' for Jira v3."]
    pub fn data_source_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `errors` after provisioning.\nThe errors from initialization or from the latest connector run."]
    pub fn errors(&self) -> ListRef<DiscoveryEngineDataConnectorErrorsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.errors", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `incremental_refresh_interval` after provisioning.\nThe refresh interval specifically for incremental data syncs. If unset,\nincremental syncs will use the default from env, set to 3hrs.\nThe minimum is 30 minutes and maximum is 7 days. Applicable to only 3P\nconnectors. When the refresh interval is\nset to the same value as the incremental refresh interval, incremental\nsync will be disabled."]
    pub fn incremental_refresh_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.incremental_refresh_interval", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `incremental_sync_disabled` after provisioning.\nIndicates whether incremental syncs are paused for this connector."]
    pub fn incremental_sync_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.incremental_sync_disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `json_params` after provisioning.\nParams needed to access the source in the format of json string."]
    pub fn json_params(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.json_params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nThe KMS key to be used to protect the DataStores managed by this connector.\nMust be set for requests that need to comply with CMEK Org Policy\nprotections.\nIf this field is set and processed successfully, the DataStores created by\nthis connector will be protected by the KMS key."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_sync_time` after provisioning.\nFor periodic connectors only, the last time a data sync was completed."]
    pub fn last_sync_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_sync_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `latest_pause_time` after provisioning.\nThe most recent timestamp when this [DataConnector][] was paused,\naffecting all functionalities such as data synchronization.\nPausing a connector has the following effects:\n  - All functionalities, including data synchronization, are halted.\n  - Any ongoing data synchronization job will be canceled.\n  - No future data synchronization runs will be scheduled nor can be\ntriggered."]
    pub fn latest_pause_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_pause_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe full resource name of the Data Connector.\nFormat: 'projects/*/locations/*/collections/*/dataConnector'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\nParams needed to access the source in the format of String-to-String (Key, Value) pairs."]
    pub fn params(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_connectivity_project_id` after provisioning.\nThe tenant project ID associated with private connectivity connectors.\nThis project must be allowlisted by in order for the connector to function."]
    pub fn private_connectivity_project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_connectivity_project_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `realtime_state` after provisioning.\nThe real-time sync state. The possible values can be:\n'STATE_UNSPECIFIED', 'CREATING', 'ACTIVE', 'FAILED', 'RUNNING', 'WARNING',\n'INITIALIZATION_FAILED', 'UPDATING'."]
    pub fn realtime_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.realtime_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `refresh_interval` after provisioning.\nThe refresh interval for data sync. If duration is set to 0, the data will\nbe synced in real time. The streaming feature is not supported yet. The\nminimum is 30 minutes and maximum is 7 days. When the refresh interval is\nset to the same value as the incremental refresh interval, incremental\nsync will be disabled."]
    pub fn refresh_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.refresh_interval", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of connector. The possible value can be:\n'STATE_UNSPECIFIED', 'CREATING', 'ACTIVE', 'FAILED', 'RUNNING', 'WARNING',\n'INITIALIZATION_FAILED', 'UPDATING'."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `static_ip_addresses` after provisioning.\nThe static IP addresses used by this connector."]
    pub fn static_ip_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.static_ip_addresses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `static_ip_enabled` after provisioning.\nWhether customer has enabled static IP addresses for this connector."]
    pub fn static_ip_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.static_ip_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sync_mode` after provisioning.\nThe data synchronization mode supported by the data connector. The possible value can be:\n'PERIODIC', 'STREAMING'."]
    pub fn sync_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sync_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the DataConnector was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `action_config` after provisioning.\n"]
    pub fn action_config(&self) -> ListRef<DiscoveryEngineDataConnectorActionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.action_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bap_config` after provisioning.\n"]
    pub fn bap_config(&self) -> ListRef<DiscoveryEngineDataConnectorBapConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bap_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination_configs` after provisioning.\n"]
    pub fn destination_configs(
        &self,
    ) -> ListRef<DiscoveryEngineDataConnectorDestinationConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entities` after provisioning.\n"]
    pub fn entities(&self) -> ListRef<DiscoveryEngineDataConnectorEntitiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.entities", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineDataConnectorTimeoutsElRef {
        DiscoveryEngineDataConnectorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineDataConnectorErrorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl DiscoveryEngineDataConnectorErrorsEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineDataConnectorErrorsEl {
    type O = BlockAssignable<DiscoveryEngineDataConnectorErrorsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineDataConnectorErrorsEl {}
impl BuildDiscoveryEngineDataConnectorErrorsEl {
    pub fn build(self) -> DiscoveryEngineDataConnectorErrorsEl {
        DiscoveryEngineDataConnectorErrorsEl {
            code: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineDataConnectorErrorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineDataConnectorErrorsElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineDataConnectorErrorsElRef {
        DiscoveryEngineDataConnectorErrorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineDataConnectorErrorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineDataConnectorActionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action_params: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_bap_connection: Option<PrimField<bool>>,
}
impl DiscoveryEngineDataConnectorActionConfigEl {
    #[doc = "Set the field `action_params`.\nParams needed to configure the actions in the format of\nString-to-String (Key, Value) pairs. Contains connection\ncredentials and configuration for the action connector."]
    pub fn set_action_params(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.action_params = Some(v.into());
        self
    }
    #[doc = "Set the field `create_bap_connection`.\nWhether to create a BAP (Business Application Platform) connection\nfor this action connector."]
    pub fn set_create_bap_connection(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.create_bap_connection = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineDataConnectorActionConfigEl {
    type O = BlockAssignable<DiscoveryEngineDataConnectorActionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineDataConnectorActionConfigEl {}
impl BuildDiscoveryEngineDataConnectorActionConfigEl {
    pub fn build(self) -> DiscoveryEngineDataConnectorActionConfigEl {
        DiscoveryEngineDataConnectorActionConfigEl {
            action_params: core::default::Default::default(),
            create_bap_connection: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineDataConnectorActionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineDataConnectorActionConfigElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineDataConnectorActionConfigElRef {
        DiscoveryEngineDataConnectorActionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineDataConnectorActionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action_params` after provisioning.\nParams needed to configure the actions in the format of\nString-to-String (Key, Value) pairs. Contains connection\ncredentials and configuration for the action connector."]
    pub fn action_params(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.action_params", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_bap_connection` after provisioning.\nWhether to create a BAP (Business Application Platform) connection\nfor this action connector."]
    pub fn create_bap_connection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_bap_connection", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `is_action_configured` after provisioning.\nWhether the action connector is fully configured. Set by the system\nafter the action configuration is validated."]
    pub fn is_action_configured(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_action_configured", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineDataConnectorBapConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled_actions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supported_connector_modes: Option<ListField<PrimField<String>>>,
}
impl DiscoveryEngineDataConnectorBapConfigEl {
    #[doc = "Set the field `enabled_actions`.\nThe list of enabled actions for this connector. Supported\nvalues include: 'create_issue', 'update_issue',\n'change_issue_status', 'create_comment', 'update_comment',\n'upload_attachment'."]
    pub fn set_enabled_actions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.enabled_actions = Some(v.into());
        self
    }
    #[doc = "Set the field `supported_connector_modes`.\nThe connector modes supported by the BAP configuration.\nThe possible values include: 'ACTIONS'."]
    pub fn set_supported_connector_modes(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.supported_connector_modes = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineDataConnectorBapConfigEl {
    type O = BlockAssignable<DiscoveryEngineDataConnectorBapConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineDataConnectorBapConfigEl {}
impl BuildDiscoveryEngineDataConnectorBapConfigEl {
    pub fn build(self) -> DiscoveryEngineDataConnectorBapConfigEl {
        DiscoveryEngineDataConnectorBapConfigEl {
            enabled_actions: core::default::Default::default(),
            supported_connector_modes: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineDataConnectorBapConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineDataConnectorBapConfigElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineDataConnectorBapConfigElRef {
        DiscoveryEngineDataConnectorBapConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineDataConnectorBapConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled_actions` after provisioning.\nThe list of enabled actions for this connector. Supported\nvalues include: 'create_issue', 'update_issue',\n'change_issue_status', 'create_comment', 'update_comment',\n'upload_attachment'."]
    pub fn enabled_actions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enabled_actions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `supported_connector_modes` after provisioning.\nThe connector modes supported by the BAP configuration.\nThe possible values include: 'ACTIONS'."]
    pub fn supported_connector_modes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_connector_modes", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineDataConnectorDestinationConfigsElDestinationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl DiscoveryEngineDataConnectorDestinationConfigsElDestinationsEl {
    #[doc = "Set the field `host`.\nThe host of the destination, for example\n'https://example.atlassian.net'."]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nTarget port number accepted by the destination."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineDataConnectorDestinationConfigsElDestinationsEl {
    type O = BlockAssignable<DiscoveryEngineDataConnectorDestinationConfigsElDestinationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineDataConnectorDestinationConfigsElDestinationsEl {}
impl BuildDiscoveryEngineDataConnectorDestinationConfigsElDestinationsEl {
    pub fn build(self) -> DiscoveryEngineDataConnectorDestinationConfigsElDestinationsEl {
        DiscoveryEngineDataConnectorDestinationConfigsElDestinationsEl {
            host: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineDataConnectorDestinationConfigsElDestinationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineDataConnectorDestinationConfigsElDestinationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineDataConnectorDestinationConfigsElDestinationsElRef {
        DiscoveryEngineDataConnectorDestinationConfigsElDestinationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineDataConnectorDestinationConfigsElDestinationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\nThe host of the destination, for example\n'https://example.atlassian.net'."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nTarget port number accepted by the destination."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize, Default)]
struct DiscoveryEngineDataConnectorDestinationConfigsElDynamic {
    destinations:
        Option<DynamicBlock<DiscoveryEngineDataConnectorDestinationConfigsElDestinationsEl>>,
}
#[derive(Serialize)]
pub struct DiscoveryEngineDataConnectorDestinationConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destinations: Option<Vec<DiscoveryEngineDataConnectorDestinationConfigsElDestinationsEl>>,
    dynamic: DiscoveryEngineDataConnectorDestinationConfigsElDynamic,
}
impl DiscoveryEngineDataConnectorDestinationConfigsEl {
    #[doc = "Set the field `key`.\nThe key of the destination configuration, for example 'url'."]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `params`.\nAdditional parameters for this destination config in structured json format."]
    pub fn set_params(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.params = Some(v.into());
        self
    }
    #[doc = "Set the field `destinations`.\n"]
    pub fn set_destinations(
        mut self,
        v: impl Into<BlockAssignable<DiscoveryEngineDataConnectorDestinationConfigsElDestinationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destinations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destinations = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DiscoveryEngineDataConnectorDestinationConfigsEl {
    type O = BlockAssignable<DiscoveryEngineDataConnectorDestinationConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineDataConnectorDestinationConfigsEl {}
impl BuildDiscoveryEngineDataConnectorDestinationConfigsEl {
    pub fn build(self) -> DiscoveryEngineDataConnectorDestinationConfigsEl {
        DiscoveryEngineDataConnectorDestinationConfigsEl {
            key: core::default::Default::default(),
            params: core::default::Default::default(),
            destinations: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DiscoveryEngineDataConnectorDestinationConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineDataConnectorDestinationConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineDataConnectorDestinationConfigsElRef {
        DiscoveryEngineDataConnectorDestinationConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineDataConnectorDestinationConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nThe key of the destination configuration, for example 'url'."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\nAdditional parameters for this destination config in structured json format."]
    pub fn params(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.params", self.base))
    }
    #[doc = "Get a reference to the value of field `destinations` after provisioning.\n"]
    pub fn destinations(
        &self,
    ) -> ListRef<DiscoveryEngineDataConnectorDestinationConfigsElDestinationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destinations", self.base))
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineDataConnectorEntitiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_property_mappings: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<PrimField<String>>,
}
impl DiscoveryEngineDataConnectorEntitiesEl {
    #[doc = "Set the field `entity_name`.\nThe name of the entity. Supported values by data source:\n* Salesforce: 'Lead', 'Opportunity', 'Contact', 'Account', 'Case', 'Contract', 'Campaign'\n* Jira: project, issue, attachment, comment, worklog\n* Confluence: 'Content', 'Space'"]
    pub fn set_entity_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.entity_name = Some(v.into());
        self
    }
    #[doc = "Set the field `key_property_mappings`.\nAttributes for indexing.\nKey: Field name.\nValue: The key property to map a field to, such as 'title', and\n'description'. Supported key properties:\n* 'title': The title for data record. This would be displayed on search\n  results.\n* 'description': The description for data record. This would be displayed\n  on search results."]
    pub fn set_key_property_mappings(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.key_property_mappings = Some(v.into());
        self
    }
    #[doc = "Set the field `params`.\nThe parameters for the entity to facilitate data ingestion."]
    pub fn set_params(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.params = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineDataConnectorEntitiesEl {
    type O = BlockAssignable<DiscoveryEngineDataConnectorEntitiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineDataConnectorEntitiesEl {}
impl BuildDiscoveryEngineDataConnectorEntitiesEl {
    pub fn build(self) -> DiscoveryEngineDataConnectorEntitiesEl {
        DiscoveryEngineDataConnectorEntitiesEl {
            entity_name: core::default::Default::default(),
            key_property_mappings: core::default::Default::default(),
            params: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineDataConnectorEntitiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineDataConnectorEntitiesElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineDataConnectorEntitiesElRef {
        DiscoveryEngineDataConnectorEntitiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineDataConnectorEntitiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_store` after provisioning.\nThe full resource name of the associated data store for the source\nentity.\nFormat: 'projects/*/locations/*/collections/*/dataStores/*'.\nWhen the connector is initialized by the DataConnectorService.SetUpDataConnector\nmethod, a DataStore is automatically created for each source entity."]
    pub fn data_store(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_store", self.base))
    }
    #[doc = "Get a reference to the value of field `entity_name` after provisioning.\nThe name of the entity. Supported values by data source:\n* Salesforce: 'Lead', 'Opportunity', 'Contact', 'Account', 'Case', 'Contract', 'Campaign'\n* Jira: project, issue, attachment, comment, worklog\n* Confluence: 'Content', 'Space'"]
    pub fn entity_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.entity_name", self.base))
    }
    #[doc = "Get a reference to the value of field `key_property_mappings` after provisioning.\nAttributes for indexing.\nKey: Field name.\nValue: The key property to map a field to, such as 'title', and\n'description'. Supported key properties:\n* 'title': The title for data record. This would be displayed on search\n  results.\n* 'description': The description for data record. This would be displayed\n  on search results."]
    pub fn key_property_mappings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.key_property_mappings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\nThe parameters for the entity to facilitate data ingestion."]
    pub fn params(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.params", self.base))
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineDataConnectorTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DiscoveryEngineDataConnectorTimeoutsEl {
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
impl ToListMappable for DiscoveryEngineDataConnectorTimeoutsEl {
    type O = BlockAssignable<DiscoveryEngineDataConnectorTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineDataConnectorTimeoutsEl {}
impl BuildDiscoveryEngineDataConnectorTimeoutsEl {
    pub fn build(self) -> DiscoveryEngineDataConnectorTimeoutsEl {
        DiscoveryEngineDataConnectorTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineDataConnectorTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineDataConnectorTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineDataConnectorTimeoutsElRef {
        DiscoveryEngineDataConnectorTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineDataConnectorTimeoutsElRef {
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
struct DiscoveryEngineDataConnectorDynamic {
    action_config: Option<DynamicBlock<DiscoveryEngineDataConnectorActionConfigEl>>,
    bap_config: Option<DynamicBlock<DiscoveryEngineDataConnectorBapConfigEl>>,
    destination_configs: Option<DynamicBlock<DiscoveryEngineDataConnectorDestinationConfigsEl>>,
    entities: Option<DynamicBlock<DiscoveryEngineDataConnectorEntitiesEl>>,
}
