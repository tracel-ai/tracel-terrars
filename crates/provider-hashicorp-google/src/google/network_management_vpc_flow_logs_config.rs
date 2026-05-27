use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkManagementVpcFlowLogsConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aggregation_interval: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_expr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_sampling: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interconnect_attachment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_fields: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet: Option<PrimField<String>>,
    vpc_flow_logs_config_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpn_tunnel: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkManagementVpcFlowLogsConfigTimeoutsEl>,
}
struct NetworkManagementVpcFlowLogsConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkManagementVpcFlowLogsConfigData>,
}
#[derive(Clone)]
pub struct NetworkManagementVpcFlowLogsConfig(Rc<NetworkManagementVpcFlowLogsConfig_>);
impl NetworkManagementVpcFlowLogsConfig {
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
    #[doc = "Set the field `aggregation_interval`.\nOptional. The aggregation interval for the logs. Default value is\nINTERVAL_5_SEC.   Possible values:  AGGREGATION_INTERVAL_UNSPECIFIED INTERVAL_5_SEC INTERVAL_30_SEC INTERVAL_1_MIN INTERVAL_5_MIN INTERVAL_10_MIN INTERVAL_15_MIN"]
    pub fn set_aggregation_interval(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().aggregation_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nOptional. The user-supplied description of the VPC Flow Logs configuration. Maximum\nof 512 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `filter_expr`.\nOptional. Export filter used to define which VPC Flow Logs should be logged."]
    pub fn set_filter_expr(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter_expr = Some(v.into());
        self
    }
    #[doc = "Set the field `flow_sampling`.\nOptional. The value of the field must be in (0, 1]. The sampling rate\nof VPC Flow Logs where 1.0 means all collected logs are reported. Setting the\nsampling rate to 0.0 is not allowed. If you want to disable VPC Flow Logs, use\nthe state field instead. Default value is 1.0."]
    pub fn set_flow_sampling(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().flow_sampling = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `interconnect_attachment`.\nTraffic will be logged from the Interconnect Attachment. Format: projects/{project_id}/regions/{region}/interconnectAttachments/{name}"]
    pub fn set_interconnect_attachment(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().interconnect_attachment = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nOptional. Resource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata`.\nOptional. Configures whether all, none or a subset of metadata fields\nshould be added to the reported VPC flow logs. Default value is INCLUDE_ALL_METADATA.\n  Possible values:  METADATA_UNSPECIFIED INCLUDE_ALL_METADATA EXCLUDE_ALL_METADATA CUSTOM_METADATA"]
    pub fn set_metadata(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata_fields`.\nOptional. Custom metadata fields to include in the reported VPC flow\nlogs. Can only be specified if \\\"metadata\\\" was set to CUSTOM_METADATA."]
    pub fn set_metadata_fields(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().metadata_fields = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nTraffic will be logged from VMs, VPN tunnels and Interconnect Attachments within the network. Format: projects/{project_id}/global/networks/{name}"]
    pub fn set_network(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\nOptional. The state of the VPC Flow Log configuration. Default value\nis ENABLED. When creating a new configuration, it must be enabled.\nPossible values: STATE_UNSPECIFIED ENABLED DISABLED"]
    pub fn set_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().state = Some(v.into());
        self
    }
    #[doc = "Set the field `subnet`.\nTraffic will be logged from VMs within the subnetwork. Format: projects/{project_id}/regions/{region}/subnetworks/{name}"]
    pub fn set_subnet(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().subnet = Some(v.into());
        self
    }
    #[doc = "Set the field `vpn_tunnel`.\nTraffic will be logged from the VPN Tunnel. Format: projects/{project_id}/regions/{region}/vpnTunnels/{name}"]
    pub fn set_vpn_tunnel(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().vpn_tunnel = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkManagementVpcFlowLogsConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `aggregation_interval` after provisioning.\nOptional. The aggregation interval for the logs. Default value is\nINTERVAL_5_SEC.   Possible values:  AGGREGATION_INTERVAL_UNSPECIFIED INTERVAL_5_SEC INTERVAL_30_SEC INTERVAL_1_MIN INTERVAL_5_MIN INTERVAL_10_MIN INTERVAL_15_MIN"]
    pub fn aggregation_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aggregation_interval", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time the config was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. The user-supplied description of the VPC Flow Logs configuration. Maximum\nof 512 characters."]
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
    #[doc = "Get a reference to the value of field `filter_expr` after provisioning.\nOptional. Export filter used to define which VPC Flow Logs should be logged."]
    pub fn filter_expr(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter_expr", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `flow_sampling` after provisioning.\nOptional. The value of the field must be in (0, 1]. The sampling rate\nof VPC Flow Logs where 1.0 means all collected logs are reported. Setting the\nsampling rate to 0.0 is not allowed. If you want to disable VPC Flow Logs, use\nthe state field instead. Default value is 1.0."]
    pub fn flow_sampling(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.flow_sampling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `interconnect_attachment` after provisioning.\nTraffic will be logged from the Interconnect Attachment. Format: projects/{project_id}/regions/{region}/interconnectAttachments/{name}"]
    pub fn interconnect_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interconnect_attachment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Resource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource\nwithin its parent collection as described in https://google.aip.dev/122. See documentation\nfor resource type 'networkmanagement.googleapis.com/VpcFlowLogsConfig'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nOptional. Configures whether all, none or a subset of metadata fields\nshould be added to the reported VPC flow logs. Default value is INCLUDE_ALL_METADATA.\n  Possible values:  METADATA_UNSPECIFIED INCLUDE_ALL_METADATA EXCLUDE_ALL_METADATA CUSTOM_METADATA"]
    pub fn metadata(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata_fields` after provisioning.\nOptional. Custom metadata fields to include in the reported VPC flow\nlogs. Can only be specified if \\\"metadata\\\" was set to CUSTOM_METADATA."]
    pub fn metadata_fields(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata_fields", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Unique name of the configuration using the form:     'projects/{project_id}/locations/global/vpcFlowLogsConfigs/{vpc_flow_logs_config_id}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nTraffic will be logged from VMs, VPN tunnels and Interconnect Attachments within the network. Format: projects/{project_id}/global/networks/{name}"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOptional. The state of the VPC Flow Log configuration. Default value\nis ENABLED. When creating a new configuration, it must be enabled.\nPossible values: STATE_UNSPECIFIED ENABLED DISABLED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet` after provisioning.\nTraffic will be logged from VMs within the subnetwork. Format: projects/{project_id}/regions/{region}/subnetworks/{name}"]
    pub fn subnet(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_resource_state` after provisioning.\nDescribes the state of the configured target resource for diagnostic\npurposes.\nPossible values:\nTARGET_RESOURCE_STATE_UNSPECIFIED\nTARGET_RESOURCE_EXISTS\nTARGET_RESOURCE_DOES_NOT_EXIST"]
    pub fn target_resource_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_resource_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time the config was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_flow_logs_config_id` after provisioning.\nRequired. ID of the 'VpcFlowLogsConfig'."]
    pub fn vpc_flow_logs_config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_flow_logs_config_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpn_tunnel` after provisioning.\nTraffic will be logged from the VPN Tunnel. Format: projects/{project_id}/regions/{region}/vpnTunnels/{name}"]
    pub fn vpn_tunnel(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpn_tunnel", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkManagementVpcFlowLogsConfigTimeoutsElRef {
        NetworkManagementVpcFlowLogsConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkManagementVpcFlowLogsConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkManagementVpcFlowLogsConfig {}
impl ToListMappable for NetworkManagementVpcFlowLogsConfig {
    type O = ListRef<NetworkManagementVpcFlowLogsConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkManagementVpcFlowLogsConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_network_management_vpc_flow_logs_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkManagementVpcFlowLogsConfig {
    pub tf_id: String,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource\nwithin its parent collection as described in https://google.aip.dev/122. See documentation\nfor resource type 'networkmanagement.googleapis.com/VpcFlowLogsConfig'."]
    pub location: PrimField<String>,
    #[doc = "Required. ID of the 'VpcFlowLogsConfig'."]
    pub vpc_flow_logs_config_id: PrimField<String>,
}
impl BuildNetworkManagementVpcFlowLogsConfig {
    pub fn build(self, stack: &mut Stack) -> NetworkManagementVpcFlowLogsConfig {
        let out =
            NetworkManagementVpcFlowLogsConfig(Rc::new(NetworkManagementVpcFlowLogsConfig_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(NetworkManagementVpcFlowLogsConfigData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    aggregation_interval: core::default::Default::default(),
                    deletion_policy: core::default::Default::default(),
                    description: core::default::Default::default(),
                    filter_expr: core::default::Default::default(),
                    flow_sampling: core::default::Default::default(),
                    id: core::default::Default::default(),
                    interconnect_attachment: core::default::Default::default(),
                    labels: core::default::Default::default(),
                    location: self.location,
                    metadata: core::default::Default::default(),
                    metadata_fields: core::default::Default::default(),
                    network: core::default::Default::default(),
                    project: core::default::Default::default(),
                    state: core::default::Default::default(),
                    subnet: core::default::Default::default(),
                    vpc_flow_logs_config_id: self.vpc_flow_logs_config_id,
                    vpn_tunnel: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkManagementVpcFlowLogsConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkManagementVpcFlowLogsConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkManagementVpcFlowLogsConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aggregation_interval` after provisioning.\nOptional. The aggregation interval for the logs. Default value is\nINTERVAL_5_SEC.   Possible values:  AGGREGATION_INTERVAL_UNSPECIFIED INTERVAL_5_SEC INTERVAL_30_SEC INTERVAL_1_MIN INTERVAL_5_MIN INTERVAL_10_MIN INTERVAL_15_MIN"]
    pub fn aggregation_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aggregation_interval", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time the config was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. The user-supplied description of the VPC Flow Logs configuration. Maximum\nof 512 characters."]
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
    #[doc = "Get a reference to the value of field `filter_expr` after provisioning.\nOptional. Export filter used to define which VPC Flow Logs should be logged."]
    pub fn filter_expr(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter_expr", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `flow_sampling` after provisioning.\nOptional. The value of the field must be in (0, 1]. The sampling rate\nof VPC Flow Logs where 1.0 means all collected logs are reported. Setting the\nsampling rate to 0.0 is not allowed. If you want to disable VPC Flow Logs, use\nthe state field instead. Default value is 1.0."]
    pub fn flow_sampling(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.flow_sampling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `interconnect_attachment` after provisioning.\nTraffic will be logged from the Interconnect Attachment. Format: projects/{project_id}/regions/{region}/interconnectAttachments/{name}"]
    pub fn interconnect_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interconnect_attachment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Resource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource\nwithin its parent collection as described in https://google.aip.dev/122. See documentation\nfor resource type 'networkmanagement.googleapis.com/VpcFlowLogsConfig'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nOptional. Configures whether all, none or a subset of metadata fields\nshould be added to the reported VPC flow logs. Default value is INCLUDE_ALL_METADATA.\n  Possible values:  METADATA_UNSPECIFIED INCLUDE_ALL_METADATA EXCLUDE_ALL_METADATA CUSTOM_METADATA"]
    pub fn metadata(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata_fields` after provisioning.\nOptional. Custom metadata fields to include in the reported VPC flow\nlogs. Can only be specified if \\\"metadata\\\" was set to CUSTOM_METADATA."]
    pub fn metadata_fields(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata_fields", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Unique name of the configuration using the form:     'projects/{project_id}/locations/global/vpcFlowLogsConfigs/{vpc_flow_logs_config_id}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nTraffic will be logged from VMs, VPN tunnels and Interconnect Attachments within the network. Format: projects/{project_id}/global/networks/{name}"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOptional. The state of the VPC Flow Log configuration. Default value\nis ENABLED. When creating a new configuration, it must be enabled.\nPossible values: STATE_UNSPECIFIED ENABLED DISABLED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet` after provisioning.\nTraffic will be logged from VMs within the subnetwork. Format: projects/{project_id}/regions/{region}/subnetworks/{name}"]
    pub fn subnet(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_resource_state` after provisioning.\nDescribes the state of the configured target resource for diagnostic\npurposes.\nPossible values:\nTARGET_RESOURCE_STATE_UNSPECIFIED\nTARGET_RESOURCE_EXISTS\nTARGET_RESOURCE_DOES_NOT_EXIST"]
    pub fn target_resource_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_resource_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time the config was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_flow_logs_config_id` after provisioning.\nRequired. ID of the 'VpcFlowLogsConfig'."]
    pub fn vpc_flow_logs_config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_flow_logs_config_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpn_tunnel` after provisioning.\nTraffic will be logged from the VPN Tunnel. Format: projects/{project_id}/regions/{region}/vpnTunnels/{name}"]
    pub fn vpn_tunnel(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpn_tunnel", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkManagementVpcFlowLogsConfigTimeoutsElRef {
        NetworkManagementVpcFlowLogsConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkManagementVpcFlowLogsConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkManagementVpcFlowLogsConfigTimeoutsEl {
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
impl ToListMappable for NetworkManagementVpcFlowLogsConfigTimeoutsEl {
    type O = BlockAssignable<NetworkManagementVpcFlowLogsConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkManagementVpcFlowLogsConfigTimeoutsEl {}
impl BuildNetworkManagementVpcFlowLogsConfigTimeoutsEl {
    pub fn build(self) -> NetworkManagementVpcFlowLogsConfigTimeoutsEl {
        NetworkManagementVpcFlowLogsConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkManagementVpcFlowLogsConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkManagementVpcFlowLogsConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkManagementVpcFlowLogsConfigTimeoutsElRef {
        NetworkManagementVpcFlowLogsConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkManagementVpcFlowLogsConfigTimeoutsElRef {
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
