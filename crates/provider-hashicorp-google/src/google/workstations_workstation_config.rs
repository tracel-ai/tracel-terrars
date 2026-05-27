use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct WorkstationsWorkstationConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_tcp_connections: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_audit_agent: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_usable_workstations: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_zones: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    running_timeout: Option<PrimField<String>>,
    workstation_cluster_id: PrimField<String>,
    workstation_config_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_ports: Option<Vec<WorkstationsWorkstationConfigAllowedPortsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container: Option<Vec<WorkstationsWorkstationConfigContainerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_key: Option<Vec<WorkstationsWorkstationConfigEncryptionKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ephemeral_directories: Option<Vec<WorkstationsWorkstationConfigEphemeralDirectoriesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<Vec<WorkstationsWorkstationConfigHostEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    persistent_directories: Option<Vec<WorkstationsWorkstationConfigPersistentDirectoriesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    readiness_checks: Option<Vec<WorkstationsWorkstationConfigReadinessChecksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<WorkstationsWorkstationConfigTimeoutsEl>,
    dynamic: WorkstationsWorkstationConfigDynamic,
}
struct WorkstationsWorkstationConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<WorkstationsWorkstationConfigData>,
}
#[derive(Clone)]
pub struct WorkstationsWorkstationConfig(Rc<WorkstationsWorkstationConfig_>);
impl WorkstationsWorkstationConfig {
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
    #[doc = "Set the field `annotations`.\nClient-specified annotations. This is distinct from labels.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_tcp_connections`.\nDisables support for plain TCP connections in the workstation. By default the service supports TCP connections via a websocket relay. Setting this option to true disables that relay, which prevents the usage of services that require plain tcp connections, such as ssh. When enabled, all communication must occur over https or wss."]
    pub fn set_disable_tcp_connections(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_tcp_connections = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nHuman-readable name for this resource."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_audit_agent`.\nWhether to enable Linux 'auditd' logging on the workstation. When enabled, a service account must also be specified that has 'logging.buckets.write' permission on the project. Operating system audit logging is distinct from Cloud Audit Logs."]
    pub fn set_enable_audit_agent(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_audit_agent = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `idle_timeout`.\nHow long to wait before automatically stopping an instance that hasn't recently received any user traffic. A value of 0 indicates that this instance should never time out from idleness. Defaults to 20 minutes.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn set_idle_timeout(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().idle_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nClient-specified labels that are applied to the resource and that are also propagated to the underlying Compute Engine resources.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `max_usable_workstations`.\nMaximum number of workstations under this configuration a user can have workstations.workstation.use permission on. Only enforced on CreateWorkstation API calls on the user issuing the API request."]
    pub fn set_max_usable_workstations(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_usable_workstations = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `replica_zones`.\nSpecifies the zones used to replicate the VM and disk resources within the region. If set, exactly two zones within the workstation cluster's region must be specified—for example, '['us-central1-a', 'us-central1-f']'.\nIf this field is empty, two default zones within the region are used. Immutable after the workstation configuration is created."]
    pub fn set_replica_zones(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().replica_zones = Some(v.into());
        self
    }
    #[doc = "Set the field `running_timeout`.\nHow long to wait before automatically stopping a workstation after it was started. A value of 0 indicates that workstations using this configuration should never time out from running duration. Must be greater than 0 and less than 24 hours if 'encryption_key' is set. Defaults to 12 hours.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn set_running_timeout(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().running_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `allowed_ports`.\n"]
    pub fn set_allowed_ports(
        self,
        v: impl Into<BlockAssignable<WorkstationsWorkstationConfigAllowedPortsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().allowed_ports = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.allowed_ports = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `container`.\n"]
    pub fn set_container(
        self,
        v: impl Into<BlockAssignable<WorkstationsWorkstationConfigContainerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().container = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.container = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `encryption_key`.\n"]
    pub fn set_encryption_key(
        self,
        v: impl Into<BlockAssignable<WorkstationsWorkstationConfigEncryptionKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_key = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ephemeral_directories`.\n"]
    pub fn set_ephemeral_directories(
        self,
        v: impl Into<BlockAssignable<WorkstationsWorkstationConfigEphemeralDirectoriesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ephemeral_directories = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ephemeral_directories = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `host`.\n"]
    pub fn set_host(
        self,
        v: impl Into<BlockAssignable<WorkstationsWorkstationConfigHostEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().host = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.host = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `persistent_directories`.\n"]
    pub fn set_persistent_directories(
        self,
        v: impl Into<BlockAssignable<WorkstationsWorkstationConfigPersistentDirectoriesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().persistent_directories = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.persistent_directories = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `readiness_checks`.\n"]
    pub fn set_readiness_checks(
        self,
        v: impl Into<BlockAssignable<WorkstationsWorkstationConfigReadinessChecksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().readiness_checks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.readiness_checks = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<WorkstationsWorkstationConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nClient-specified annotations. This is distinct from labels.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\nStatus conditions describing the current resource state."]
    pub fn conditions(&self) -> ListRef<WorkstationsWorkstationConfigConditionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conditions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime when this resource was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `degraded` after provisioning.\nWhether this resource is in degraded mode, in which case it may require user action to restore full functionality. Details can be found in the conditions field."]
    pub fn degraded(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.degraded", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disable_tcp_connections` after provisioning.\nDisables support for plain TCP connections in the workstation. By default the service supports TCP connections via a websocket relay. Setting this option to true disables that relay, which prevents the usage of services that require plain tcp connections, such as ssh. When enabled, all communication must occur over https or wss."]
    pub fn disable_tcp_connections(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_tcp_connections", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable name for this resource."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_audit_agent` after provisioning.\nWhether to enable Linux 'auditd' logging on the workstation. When enabled, a service account must also be specified that has 'logging.buckets.write' permission on the project. Operating system audit logging is distinct from Cloud Audit Logs."]
    pub fn enable_audit_agent(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_audit_agent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nChecksum computed by the server.\nMay be sent on update and delete requests to ensure that the client has an up-to-date value before proceeding."]
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
    #[doc = "Get a reference to the value of field `idle_timeout` after provisioning.\nHow long to wait before automatically stopping an instance that hasn't recently received any user traffic. A value of 0 indicates that this instance should never time out from idleness. Defaults to 20 minutes.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn idle_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idle_timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nClient-specified labels that are applied to the resource and that are also propagated to the underlying Compute Engine resources.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location where the workstation cluster config should reside."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_usable_workstations` after provisioning.\nMaximum number of workstations under this configuration a user can have workstations.workstation.use permission on. Only enforced on CreateWorkstation API calls on the user issuing the API request."]
    pub fn max_usable_workstations(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_usable_workstations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nFull name of this resource."]
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
    #[doc = "Get a reference to the value of field `replica_zones` after provisioning.\nSpecifies the zones used to replicate the VM and disk resources within the region. If set, exactly two zones within the workstation cluster's region must be specified—for example, '['us-central1-a', 'us-central1-f']'.\nIf this field is empty, two default zones within the region are used. Immutable after the workstation configuration is created."]
    pub fn replica_zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replica_zones", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `running_timeout` after provisioning.\nHow long to wait before automatically stopping a workstation after it was started. A value of 0 indicates that workstations using this configuration should never time out from running duration. Must be greater than 0 and less than 24 hours if 'encryption_key' is set. Defaults to 12 hours.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn running_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.running_timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe system-generated UID of the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `workstation_cluster_id` after provisioning.\nThe ID of the parent workstation cluster."]
    pub fn workstation_cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workstation_cluster_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workstation_config_id` after provisioning.\nThe ID to be assigned to the workstation cluster config."]
    pub fn workstation_config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workstation_config_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allowed_ports` after provisioning.\n"]
    pub fn allowed_ports(&self) -> ListRef<WorkstationsWorkstationConfigAllowedPortsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_ports", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(&self) -> ListRef<WorkstationsWorkstationConfigContainerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.container", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_key` after provisioning.\n"]
    pub fn encryption_key(&self) -> ListRef<WorkstationsWorkstationConfigEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ephemeral_directories` after provisioning.\n"]
    pub fn ephemeral_directories(
        &self,
    ) -> ListRef<WorkstationsWorkstationConfigEphemeralDirectoriesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ephemeral_directories", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> ListRef<WorkstationsWorkstationConfigHostElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.host", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `persistent_directories` after provisioning.\n"]
    pub fn persistent_directories(
        &self,
    ) -> ListRef<WorkstationsWorkstationConfigPersistentDirectoriesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.persistent_directories", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `readiness_checks` after provisioning.\n"]
    pub fn readiness_checks(&self) -> ListRef<WorkstationsWorkstationConfigReadinessChecksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.readiness_checks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> WorkstationsWorkstationConfigTimeoutsElRef {
        WorkstationsWorkstationConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for WorkstationsWorkstationConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for WorkstationsWorkstationConfig {}
impl ToListMappable for WorkstationsWorkstationConfig {
    type O = ListRef<WorkstationsWorkstationConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for WorkstationsWorkstationConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_workstations_workstation_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildWorkstationsWorkstationConfig {
    pub tf_id: String,
    #[doc = "The location where the workstation cluster config should reside."]
    pub location: PrimField<String>,
    #[doc = "The ID of the parent workstation cluster."]
    pub workstation_cluster_id: PrimField<String>,
    #[doc = "The ID to be assigned to the workstation cluster config."]
    pub workstation_config_id: PrimField<String>,
}
impl BuildWorkstationsWorkstationConfig {
    pub fn build(self, stack: &mut Stack) -> WorkstationsWorkstationConfig {
        let out = WorkstationsWorkstationConfig(Rc::new(WorkstationsWorkstationConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(WorkstationsWorkstationConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                disable_tcp_connections: core::default::Default::default(),
                display_name: core::default::Default::default(),
                enable_audit_agent: core::default::Default::default(),
                id: core::default::Default::default(),
                idle_timeout: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                max_usable_workstations: core::default::Default::default(),
                project: core::default::Default::default(),
                replica_zones: core::default::Default::default(),
                running_timeout: core::default::Default::default(),
                workstation_cluster_id: self.workstation_cluster_id,
                workstation_config_id: self.workstation_config_id,
                allowed_ports: core::default::Default::default(),
                container: core::default::Default::default(),
                encryption_key: core::default::Default::default(),
                ephemeral_directories: core::default::Default::default(),
                host: core::default::Default::default(),
                persistent_directories: core::default::Default::default(),
                readiness_checks: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct WorkstationsWorkstationConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkstationsWorkstationConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl WorkstationsWorkstationConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nClient-specified annotations. This is distinct from labels.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\nStatus conditions describing the current resource state."]
    pub fn conditions(&self) -> ListRef<WorkstationsWorkstationConfigConditionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conditions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime when this resource was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `degraded` after provisioning.\nWhether this resource is in degraded mode, in which case it may require user action to restore full functionality. Details can be found in the conditions field."]
    pub fn degraded(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.degraded", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disable_tcp_connections` after provisioning.\nDisables support for plain TCP connections in the workstation. By default the service supports TCP connections via a websocket relay. Setting this option to true disables that relay, which prevents the usage of services that require plain tcp connections, such as ssh. When enabled, all communication must occur over https or wss."]
    pub fn disable_tcp_connections(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_tcp_connections", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable name for this resource."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_audit_agent` after provisioning.\nWhether to enable Linux 'auditd' logging on the workstation. When enabled, a service account must also be specified that has 'logging.buckets.write' permission on the project. Operating system audit logging is distinct from Cloud Audit Logs."]
    pub fn enable_audit_agent(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_audit_agent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nChecksum computed by the server.\nMay be sent on update and delete requests to ensure that the client has an up-to-date value before proceeding."]
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
    #[doc = "Get a reference to the value of field `idle_timeout` after provisioning.\nHow long to wait before automatically stopping an instance that hasn't recently received any user traffic. A value of 0 indicates that this instance should never time out from idleness. Defaults to 20 minutes.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn idle_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idle_timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nClient-specified labels that are applied to the resource and that are also propagated to the underlying Compute Engine resources.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location where the workstation cluster config should reside."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_usable_workstations` after provisioning.\nMaximum number of workstations under this configuration a user can have workstations.workstation.use permission on. Only enforced on CreateWorkstation API calls on the user issuing the API request."]
    pub fn max_usable_workstations(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_usable_workstations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nFull name of this resource."]
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
    #[doc = "Get a reference to the value of field `replica_zones` after provisioning.\nSpecifies the zones used to replicate the VM and disk resources within the region. If set, exactly two zones within the workstation cluster's region must be specified—for example, '['us-central1-a', 'us-central1-f']'.\nIf this field is empty, two default zones within the region are used. Immutable after the workstation configuration is created."]
    pub fn replica_zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replica_zones", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `running_timeout` after provisioning.\nHow long to wait before automatically stopping a workstation after it was started. A value of 0 indicates that workstations using this configuration should never time out from running duration. Must be greater than 0 and less than 24 hours if 'encryption_key' is set. Defaults to 12 hours.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn running_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.running_timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe system-generated UID of the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `workstation_cluster_id` after provisioning.\nThe ID of the parent workstation cluster."]
    pub fn workstation_cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workstation_cluster_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workstation_config_id` after provisioning.\nThe ID to be assigned to the workstation cluster config."]
    pub fn workstation_config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workstation_config_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allowed_ports` after provisioning.\n"]
    pub fn allowed_ports(&self) -> ListRef<WorkstationsWorkstationConfigAllowedPortsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_ports", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(&self) -> ListRef<WorkstationsWorkstationConfigContainerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.container", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_key` after provisioning.\n"]
    pub fn encryption_key(&self) -> ListRef<WorkstationsWorkstationConfigEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ephemeral_directories` after provisioning.\n"]
    pub fn ephemeral_directories(
        &self,
    ) -> ListRef<WorkstationsWorkstationConfigEphemeralDirectoriesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ephemeral_directories", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> ListRef<WorkstationsWorkstationConfigHostElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.host", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `persistent_directories` after provisioning.\n"]
    pub fn persistent_directories(
        &self,
    ) -> ListRef<WorkstationsWorkstationConfigPersistentDirectoriesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.persistent_directories", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `readiness_checks` after provisioning.\n"]
    pub fn readiness_checks(&self) -> ListRef<WorkstationsWorkstationConfigReadinessChecksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.readiness_checks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> WorkstationsWorkstationConfigTimeoutsElRef {
        WorkstationsWorkstationConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct WorkstationsWorkstationConfigConditionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<ListField<RecField<PrimField<String>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl WorkstationsWorkstationConfigConditionsEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(mut self, v: impl Into<ListField<RecField<PrimField<String>>>>) -> Self {
        self.details = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable for WorkstationsWorkstationConfigConditionsEl {
    type O = BlockAssignable<WorkstationsWorkstationConfigConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkstationsWorkstationConfigConditionsEl {}
impl BuildWorkstationsWorkstationConfigConditionsEl {
    pub fn build(self) -> WorkstationsWorkstationConfigConditionsEl {
        WorkstationsWorkstationConfigConditionsEl {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct WorkstationsWorkstationConfigConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkstationsWorkstationConfigConditionsElRef {
    fn new(shared: StackShared, base: String) -> WorkstationsWorkstationConfigConditionsElRef {
        WorkstationsWorkstationConfigConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkstationsWorkstationConfigConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> ListRef<RecRef<PrimExpr<String>>> {
        ListRef::new(self.shared().clone(), format!("{}.details", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct WorkstationsWorkstationConfigAllowedPortsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    first: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last: Option<PrimField<f64>>,
}
impl WorkstationsWorkstationConfigAllowedPortsEl {
    #[doc = "Set the field `first`.\nStarting port number for the current range of ports. Valid ports are 22, 80, and ports within the range 1024-65535."]
    pub fn set_first(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.first = Some(v.into());
        self
    }
    #[doc = "Set the field `last`.\nEnding port number for the current range of ports. Valid ports are 22, 80, and ports within the range 1024-65535."]
    pub fn set_last(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.last = Some(v.into());
        self
    }
}
impl ToListMappable for WorkstationsWorkstationConfigAllowedPortsEl {
    type O = BlockAssignable<WorkstationsWorkstationConfigAllowedPortsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkstationsWorkstationConfigAllowedPortsEl {}
impl BuildWorkstationsWorkstationConfigAllowedPortsEl {
    pub fn build(self) -> WorkstationsWorkstationConfigAllowedPortsEl {
        WorkstationsWorkstationConfigAllowedPortsEl {
            first: core::default::Default::default(),
            last: core::default::Default::default(),
        }
    }
}
pub struct WorkstationsWorkstationConfigAllowedPortsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkstationsWorkstationConfigAllowedPortsElRef {
    fn new(shared: StackShared, base: String) -> WorkstationsWorkstationConfigAllowedPortsElRef {
        WorkstationsWorkstationConfigAllowedPortsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkstationsWorkstationConfigAllowedPortsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `first` after provisioning.\nStarting port number for the current range of ports. Valid ports are 22, 80, and ports within the range 1024-65535."]
    pub fn first(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.first", self.base))
    }
    #[doc = "Get a reference to the value of field `last` after provisioning.\nEnding port number for the current range of ports. Valid ports are 22, 80, and ports within the range 1024-65535."]
    pub fn last(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.last", self.base))
    }
}
#[derive(Serialize)]
pub struct WorkstationsWorkstationConfigContainerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_as_user: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    working_dir: Option<PrimField<String>>,
}
impl WorkstationsWorkstationConfigContainerEl {
    #[doc = "Set the field `args`.\nArguments passed to the entrypoint."]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }
    #[doc = "Set the field `command`.\nIf set, overrides the default ENTRYPOINT specified by the image."]
    pub fn set_command(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.command = Some(v.into());
        self
    }
    #[doc = "Set the field `env`.\nEnvironment variables passed to the container.\nThe elements are of the form \"KEY=VALUE\" for the environment variable \"KEY\" being given the value \"VALUE\"."]
    pub fn set_env(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.env = Some(v.into());
        self
    }
    #[doc = "Set the field `image`.\nDocker image defining the container. This image must be accessible by the config's service account."]
    pub fn set_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image = Some(v.into());
        self
    }
    #[doc = "Set the field `run_as_user`.\nIf set, overrides the USER specified in the image with the given uid."]
    pub fn set_run_as_user(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.run_as_user = Some(v.into());
        self
    }
    #[doc = "Set the field `working_dir`.\nIf set, overrides the default DIR specified by the image."]
    pub fn set_working_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.working_dir = Some(v.into());
        self
    }
}
impl ToListMappable for WorkstationsWorkstationConfigContainerEl {
    type O = BlockAssignable<WorkstationsWorkstationConfigContainerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkstationsWorkstationConfigContainerEl {}
impl BuildWorkstationsWorkstationConfigContainerEl {
    pub fn build(self) -> WorkstationsWorkstationConfigContainerEl {
        WorkstationsWorkstationConfigContainerEl {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            env: core::default::Default::default(),
            image: core::default::Default::default(),
            run_as_user: core::default::Default::default(),
            working_dir: core::default::Default::default(),
        }
    }
}
pub struct WorkstationsWorkstationConfigContainerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkstationsWorkstationConfigContainerElRef {
    fn new(shared: StackShared, base: String) -> WorkstationsWorkstationConfigContainerElRef {
        WorkstationsWorkstationConfigContainerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkstationsWorkstationConfigContainerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\nArguments passed to the entrypoint."]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }
    #[doc = "Get a reference to the value of field `command` after provisioning.\nIf set, overrides the default ENTRYPOINT specified by the image."]
    pub fn command(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }
    #[doc = "Get a reference to the value of field `env` after provisioning.\nEnvironment variables passed to the container.\nThe elements are of the form \"KEY=VALUE\" for the environment variable \"KEY\" being given the value \"VALUE\"."]
    pub fn env(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.env", self.base))
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\nDocker image defining the container. This image must be accessible by the config's service account."]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
    #[doc = "Get a reference to the value of field `run_as_user` after provisioning.\nIf set, overrides the USER specified in the image with the given uid."]
    pub fn run_as_user(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.run_as_user", self.base))
    }
    #[doc = "Get a reference to the value of field `working_dir` after provisioning.\nIf set, overrides the default DIR specified by the image."]
    pub fn working_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.working_dir", self.base))
    }
}
#[derive(Serialize)]
pub struct WorkstationsWorkstationConfigEncryptionKeyEl {
    kms_key: PrimField<String>,
    kms_key_service_account: PrimField<String>,
}
impl WorkstationsWorkstationConfigEncryptionKeyEl {}
impl ToListMappable for WorkstationsWorkstationConfigEncryptionKeyEl {
    type O = BlockAssignable<WorkstationsWorkstationConfigEncryptionKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkstationsWorkstationConfigEncryptionKeyEl {
    #[doc = "The name of the Google Cloud KMS encryption key."]
    pub kms_key: PrimField<String>,
    #[doc = "The service account to use with the specified KMS key."]
    pub kms_key_service_account: PrimField<String>,
}
impl BuildWorkstationsWorkstationConfigEncryptionKeyEl {
    pub fn build(self) -> WorkstationsWorkstationConfigEncryptionKeyEl {
        WorkstationsWorkstationConfigEncryptionKeyEl {
            kms_key: self.kms_key,
            kms_key_service_account: self.kms_key_service_account,
        }
    }
}
pub struct WorkstationsWorkstationConfigEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkstationsWorkstationConfigEncryptionKeyElRef {
    fn new(shared: StackShared, base: String) -> WorkstationsWorkstationConfigEncryptionKeyElRef {
        WorkstationsWorkstationConfigEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkstationsWorkstationConfigEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nThe name of the Google Cloud KMS encryption key."]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_key_service_account` after provisioning.\nThe service account to use with the specified KMS key."]
    pub fn kms_key_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_service_account", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct WorkstationsWorkstationConfigEphemeralDirectoriesElGcePdEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_snapshot: Option<PrimField<String>>,
}
impl WorkstationsWorkstationConfigEphemeralDirectoriesElGcePdEl {
    #[doc = "Set the field `disk_type`.\nType of the disk to use. Defaults to '\"pd-standard\"'."]
    pub fn set_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_type = Some(v.into());
        self
    }
    #[doc = "Set the field `read_only`.\nWhether the disk is read only. If true, the disk may be shared by multiple VMs and 'sourceSnapshot' must be set."]
    pub fn set_read_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.read_only = Some(v.into());
        self
    }
    #[doc = "Set the field `source_image`.\nName of the disk image to use as the source for the disk.\n\nMust be empty 'sourceSnapshot' is set.\nUpdating 'sourceImage' will update content in the ephemeral directory after the workstation is restarted."]
    pub fn set_source_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_image = Some(v.into());
        self
    }
    #[doc = "Set the field `source_snapshot`.\nName of the snapshot to use as the source for the disk.\n\nMust be empty if 'sourceImage' is set.\nMust be empty if 'read_only' is false.\nUpdating 'source_snapshot' will update content in the ephemeral directory after the workstation is restarted."]
    pub fn set_source_snapshot(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_snapshot = Some(v.into());
        self
    }
}
impl ToListMappable for WorkstationsWorkstationConfigEphemeralDirectoriesElGcePdEl {
    type O = BlockAssignable<WorkstationsWorkstationConfigEphemeralDirectoriesElGcePdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkstationsWorkstationConfigEphemeralDirectoriesElGcePdEl {}
impl BuildWorkstationsWorkstationConfigEphemeralDirectoriesElGcePdEl {
    pub fn build(self) -> WorkstationsWorkstationConfigEphemeralDirectoriesElGcePdEl {
        WorkstationsWorkstationConfigEphemeralDirectoriesElGcePdEl {
            disk_type: core::default::Default::default(),
            read_only: core::default::Default::default(),
            source_image: core::default::Default::default(),
            source_snapshot: core::default::Default::default(),
        }
    }
}
pub struct WorkstationsWorkstationConfigEphemeralDirectoriesElGcePdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkstationsWorkstationConfigEphemeralDirectoriesElGcePdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WorkstationsWorkstationConfigEphemeralDirectoriesElGcePdElRef {
        WorkstationsWorkstationConfigEphemeralDirectoriesElGcePdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkstationsWorkstationConfigEphemeralDirectoriesElGcePdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\nType of the disk to use. Defaults to '\"pd-standard\"'."]
    pub fn disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }
    #[doc = "Get a reference to the value of field `read_only` after provisioning.\nWhether the disk is read only. If true, the disk may be shared by multiple VMs and 'sourceSnapshot' must be set."]
    pub fn read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }
    #[doc = "Get a reference to the value of field `source_image` after provisioning.\nName of the disk image to use as the source for the disk.\n\nMust be empty 'sourceSnapshot' is set.\nUpdating 'sourceImage' will update content in the ephemeral directory after the workstation is restarted."]
    pub fn source_image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_image", self.base))
    }
    #[doc = "Get a reference to the value of field `source_snapshot` after provisioning.\nName of the snapshot to use as the source for the disk.\n\nMust be empty if 'sourceImage' is set.\nMust be empty if 'read_only' is false.\nUpdating 'source_snapshot' will update content in the ephemeral directory after the workstation is restarted."]
    pub fn source_snapshot(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_snapshot", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct WorkstationsWorkstationConfigEphemeralDirectoriesElDynamic {
    gce_pd: Option<DynamicBlock<WorkstationsWorkstationConfigEphemeralDirectoriesElGcePdEl>>,
}
#[derive(Serialize)]
pub struct WorkstationsWorkstationConfigEphemeralDirectoriesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mount_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gce_pd: Option<Vec<WorkstationsWorkstationConfigEphemeralDirectoriesElGcePdEl>>,
    dynamic: WorkstationsWorkstationConfigEphemeralDirectoriesElDynamic,
}
impl WorkstationsWorkstationConfigEphemeralDirectoriesEl {
    #[doc = "Set the field `mount_path`.\nLocation of this directory in the running workstation."]
    pub fn set_mount_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mount_path = Some(v.into());
        self
    }
    #[doc = "Set the field `gce_pd`.\n"]
    pub fn set_gce_pd(
        mut self,
        v: impl Into<BlockAssignable<WorkstationsWorkstationConfigEphemeralDirectoriesElGcePdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gce_pd = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gce_pd = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for WorkstationsWorkstationConfigEphemeralDirectoriesEl {
    type O = BlockAssignable<WorkstationsWorkstationConfigEphemeralDirectoriesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkstationsWorkstationConfigEphemeralDirectoriesEl {}
impl BuildWorkstationsWorkstationConfigEphemeralDirectoriesEl {
    pub fn build(self) -> WorkstationsWorkstationConfigEphemeralDirectoriesEl {
        WorkstationsWorkstationConfigEphemeralDirectoriesEl {
            mount_path: core::default::Default::default(),
            gce_pd: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct WorkstationsWorkstationConfigEphemeralDirectoriesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkstationsWorkstationConfigEphemeralDirectoriesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WorkstationsWorkstationConfigEphemeralDirectoriesElRef {
        WorkstationsWorkstationConfigEphemeralDirectoriesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkstationsWorkstationConfigEphemeralDirectoriesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\nLocation of this directory in the running workstation."]
    pub fn mount_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }
    #[doc = "Get a reference to the value of field `gce_pd` after provisioning.\n"]
    pub fn gce_pd(&self) -> ListRef<WorkstationsWorkstationConfigEphemeralDirectoriesElGcePdElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gce_pd", self.base))
    }
}
#[derive(Serialize)]
pub struct WorkstationsWorkstationConfigHostElGceInstanceElAcceleratorsEl {
    count: PrimField<f64>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl WorkstationsWorkstationConfigHostElGceInstanceElAcceleratorsEl {}
impl ToListMappable for WorkstationsWorkstationConfigHostElGceInstanceElAcceleratorsEl {
    type O = BlockAssignable<WorkstationsWorkstationConfigHostElGceInstanceElAcceleratorsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkstationsWorkstationConfigHostElGceInstanceElAcceleratorsEl {
    #[doc = "Number of accelerator cards exposed to the instance."]
    pub count: PrimField<f64>,
    #[doc = "Type of accelerator resource to attach to the instance, for example, \"nvidia-tesla-p100\"."]
    pub type_: PrimField<String>,
}
impl BuildWorkstationsWorkstationConfigHostElGceInstanceElAcceleratorsEl {
    pub fn build(self) -> WorkstationsWorkstationConfigHostElGceInstanceElAcceleratorsEl {
        WorkstationsWorkstationConfigHostElGceInstanceElAcceleratorsEl {
            count: self.count,
            type_: self.type_,
        }
    }
}
pub struct WorkstationsWorkstationConfigHostElGceInstanceElAcceleratorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkstationsWorkstationConfigHostElGceInstanceElAcceleratorsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WorkstationsWorkstationConfigHostElGceInstanceElAcceleratorsElRef {
        WorkstationsWorkstationConfigHostElGceInstanceElAcceleratorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkstationsWorkstationConfigHostElGceInstanceElAcceleratorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\nNumber of accelerator cards exposed to the instance."]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of accelerator resource to attach to the instance, for example, \"nvidia-tesla-p100\"."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsElAcceleratorsEl {
    count: PrimField<f64>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsElAcceleratorsEl {}
impl ToListMappable
    for WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsElAcceleratorsEl
{
    type O = BlockAssignable<
        WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsElAcceleratorsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsElAcceleratorsEl {
    #[doc = "Number of accelerator cards exposed to the instance."]
    pub count: PrimField<f64>,
    #[doc = "Type of accelerator resource to attach to the instance, for example, \"nvidia-tesla-p100\"."]
    pub type_: PrimField<String>,
}
impl BuildWorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsElAcceleratorsEl {
    pub fn build(
        self,
    ) -> WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsElAcceleratorsEl {
        WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsElAcceleratorsEl {
            count: self.count,
            type_: self.type_,
        }
    }
}
pub struct WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsElAcceleratorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsElAcceleratorsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsElAcceleratorsElRef {
        WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsElAcceleratorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsElAcceleratorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\nNumber of accelerator cards exposed to the instance."]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of accelerator resource to attach to the instance, for example, \"nvidia-tesla-p100\"."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize, Default)]
struct WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsElDynamic {
    accelerators: Option<
        DynamicBlock<WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsElAcceleratorsEl>,
    >,
}
#[derive(Serialize)]
pub struct WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_nested_virtualization: Option<PrimField<bool>>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pool_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerators:
        Option<Vec<WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsElAcceleratorsEl>>,
    dynamic: WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsElDynamic,
}
impl WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsEl {
    #[doc = "Set the field `boot_disk_size_gb`.\nSize of the boot disk in GB. The minimum boot disk size is '30' GB. Defaults to '50' GB."]
    pub fn set_boot_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.boot_disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_nested_virtualization`.\nWhether to enable nested virtualization on the Compute Engine VMs backing boosted Workstations.\n\nSee https://cloud.google.com/workstations/docs/reference/rest/v1/projects.locations.workstationClusters.workstationConfigs#GceInstance.FIELDS.enable_nested_virtualization"]
    pub fn set_enable_nested_virtualization(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_nested_virtualization = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\nThe type of machine that boosted VM instances will use—for example, e2-standard-4. For more information about machine types that Cloud Workstations supports, see the list of available machine types https://cloud.google.com/workstations/docs/available-machine-types. Defaults to e2-standard-4."]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
    #[doc = "Set the field `pool_size`.\nNumber of instances to pool for faster workstation boosting."]
    pub fn set_pool_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.pool_size = Some(v.into());
        self
    }
    #[doc = "Set the field `accelerators`.\n"]
    pub fn set_accelerators(
        mut self,
        v: impl Into<
            BlockAssignable<
                WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsElAcceleratorsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.accelerators = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerators = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsEl {
    type O = BlockAssignable<WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsEl {
    #[doc = "The id to be used for the boost config."]
    pub id: PrimField<String>,
}
impl BuildWorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsEl {
    pub fn build(self) -> WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsEl {
        WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsEl {
            boot_disk_size_gb: core::default::Default::default(),
            enable_nested_virtualization: core::default::Default::default(),
            id: self.id,
            machine_type: core::default::Default::default(),
            pool_size: core::default::Default::default(),
            accelerators: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsElRef {
        WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `boot_disk_size_gb` after provisioning.\nSize of the boot disk in GB. The minimum boot disk size is '30' GB. Defaults to '50' GB."]
    pub fn boot_disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_nested_virtualization` after provisioning.\nWhether to enable nested virtualization on the Compute Engine VMs backing boosted Workstations.\n\nSee https://cloud.google.com/workstations/docs/reference/rest/v1/projects.locations.workstationClusters.workstationConfigs#GceInstance.FIELDS.enable_nested_virtualization"]
    pub fn enable_nested_virtualization(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_nested_virtualization", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe id to be used for the boost config."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nThe type of machine that boosted VM instances will use—for example, e2-standard-4. For more information about machine types that Cloud Workstations supports, see the list of available machine types https://cloud.google.com/workstations/docs/available-machine-types. Defaults to e2-standard-4."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `pool_size` after provisioning.\nNumber of instances to pool for faster workstation boosting."]
    pub fn pool_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.pool_size", self.base))
    }
    #[doc = "Get a reference to the value of field `accelerators` after provisioning.\n"]
    pub fn accelerators(
        &self,
    ) -> ListRef<WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsElAcceleratorsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.accelerators", self.base))
    }
}
#[derive(Serialize)]
pub struct WorkstationsWorkstationConfigHostElGceInstanceElConfidentialInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_confidential_compute: Option<PrimField<bool>>,
}
impl WorkstationsWorkstationConfigHostElGceInstanceElConfidentialInstanceConfigEl {
    #[doc = "Set the field `enable_confidential_compute`.\nWhether the instance has confidential compute enabled."]
    pub fn set_enable_confidential_compute(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_confidential_compute = Some(v.into());
        self
    }
}
impl ToListMappable
    for WorkstationsWorkstationConfigHostElGceInstanceElConfidentialInstanceConfigEl
{
    type O = BlockAssignable<
        WorkstationsWorkstationConfigHostElGceInstanceElConfidentialInstanceConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkstationsWorkstationConfigHostElGceInstanceElConfidentialInstanceConfigEl {}
impl BuildWorkstationsWorkstationConfigHostElGceInstanceElConfidentialInstanceConfigEl {
    pub fn build(
        self,
    ) -> WorkstationsWorkstationConfigHostElGceInstanceElConfidentialInstanceConfigEl {
        WorkstationsWorkstationConfigHostElGceInstanceElConfidentialInstanceConfigEl {
            enable_confidential_compute: core::default::Default::default(),
        }
    }
}
pub struct WorkstationsWorkstationConfigHostElGceInstanceElConfidentialInstanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkstationsWorkstationConfigHostElGceInstanceElConfidentialInstanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WorkstationsWorkstationConfigHostElGceInstanceElConfidentialInstanceConfigElRef {
        WorkstationsWorkstationConfigHostElGceInstanceElConfidentialInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkstationsWorkstationConfigHostElGceInstanceElConfidentialInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_confidential_compute` after provisioning.\nWhether the instance has confidential compute enabled."]
    pub fn enable_confidential_compute(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_confidential_compute", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct WorkstationsWorkstationConfigHostElGceInstanceElShieldedInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_integrity_monitoring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_secure_boot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_vtpm: Option<PrimField<bool>>,
}
impl WorkstationsWorkstationConfigHostElGceInstanceElShieldedInstanceConfigEl {
    #[doc = "Set the field `enable_integrity_monitoring`.\nWhether the instance has integrity monitoring enabled."]
    pub fn set_enable_integrity_monitoring(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_secure_boot`.\nWhether the instance has Secure Boot enabled."]
    pub fn set_enable_secure_boot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_secure_boot = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_vtpm`.\nWhether the instance has the vTPM enabled."]
    pub fn set_enable_vtpm(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_vtpm = Some(v.into());
        self
    }
}
impl ToListMappable for WorkstationsWorkstationConfigHostElGceInstanceElShieldedInstanceConfigEl {
    type O =
        BlockAssignable<WorkstationsWorkstationConfigHostElGceInstanceElShieldedInstanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkstationsWorkstationConfigHostElGceInstanceElShieldedInstanceConfigEl {}
impl BuildWorkstationsWorkstationConfigHostElGceInstanceElShieldedInstanceConfigEl {
    pub fn build(self) -> WorkstationsWorkstationConfigHostElGceInstanceElShieldedInstanceConfigEl {
        WorkstationsWorkstationConfigHostElGceInstanceElShieldedInstanceConfigEl {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
            enable_vtpm: core::default::Default::default(),
        }
    }
}
pub struct WorkstationsWorkstationConfigHostElGceInstanceElShieldedInstanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkstationsWorkstationConfigHostElGceInstanceElShieldedInstanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WorkstationsWorkstationConfigHostElGceInstanceElShieldedInstanceConfigElRef {
        WorkstationsWorkstationConfigHostElGceInstanceElShieldedInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkstationsWorkstationConfigHostElGceInstanceElShieldedInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\nWhether the instance has integrity monitoring enabled."]
    pub fn enable_integrity_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_integrity_monitoring", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_secure_boot` after provisioning.\nWhether the instance has Secure Boot enabled."]
    pub fn enable_secure_boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_secure_boot", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_vtpm` after provisioning.\nWhether the instance has the vTPM enabled."]
    pub fn enable_vtpm(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_vtpm", self.base))
    }
}
#[derive(Serialize, Default)]
struct WorkstationsWorkstationConfigHostElGceInstanceElDynamic {
    accelerators:
        Option<DynamicBlock<WorkstationsWorkstationConfigHostElGceInstanceElAcceleratorsEl>>,
    boost_configs:
        Option<DynamicBlock<WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsEl>>,
    confidential_instance_config: Option<
        DynamicBlock<WorkstationsWorkstationConfigHostElGceInstanceElConfidentialInstanceConfigEl>,
    >,
    shielded_instance_config: Option<
        DynamicBlock<WorkstationsWorkstationConfigHostElGceInstanceElShieldedInstanceConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct WorkstationsWorkstationConfigHostElGceInstanceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_public_ip_addresses: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_ssh: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_nested_virtualization: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pool_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_scopes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vm_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerators: Option<Vec<WorkstationsWorkstationConfigHostElGceInstanceElAcceleratorsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boost_configs: Option<Vec<WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential_instance_config:
        Option<Vec<WorkstationsWorkstationConfigHostElGceInstanceElConfidentialInstanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shielded_instance_config:
        Option<Vec<WorkstationsWorkstationConfigHostElGceInstanceElShieldedInstanceConfigEl>>,
    dynamic: WorkstationsWorkstationConfigHostElGceInstanceElDynamic,
}
impl WorkstationsWorkstationConfigHostElGceInstanceEl {
    #[doc = "Set the field `boot_disk_size_gb`.\nSize of the boot disk in GB."]
    pub fn set_boot_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.boot_disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_public_ip_addresses`.\nWhether instances have no public IP address."]
    pub fn set_disable_public_ip_addresses(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_public_ip_addresses = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_ssh`.\nWhether to disable SSH access to the VM."]
    pub fn set_disable_ssh(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_ssh = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_nested_virtualization`.\nWhether to enable nested virtualization on the Compute Engine VMs backing the Workstations.\n\nSee https://cloud.google.com/workstations/docs/reference/rest/v1/projects.locations.workstationClusters.workstationConfigs#GceInstance.FIELDS.enable_nested_virtualization"]
    pub fn set_enable_nested_virtualization(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_nested_virtualization = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\nThe name of a Compute Engine machine type."]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
    #[doc = "Set the field `pool_size`.\nNumber of instances to pool for faster workstation startup."]
    pub fn set_pool_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.pool_size = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\nEmail address of the service account that will be used on VM instances used to support this config. This service account must have permission to pull the specified container image. If not set, VMs will run without a service account, in which case the image must be publicly accessible."]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account_scopes`.\nScopes to grant to the service_account. Various scopes are automatically added based on feature usage. When specified, users of workstations under this configuration must have 'iam.serviceAccounts.actAs' on the service account."]
    pub fn set_service_account_scopes(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.service_account_scopes = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\nNetwork tags to add to the Compute Engine machines backing the Workstations."]
    pub fn set_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
    #[doc = "Set the field `vm_tags`.\nResource manager tags to be bound to the VM instances backing the Workstations.\nTag keys and values have the same definition as\nhttps://docs.cloud.google.com/resource-manager/docs/tags/tags-overview\nKeys must be in the format 'tagKeys/{tag_key_id}', and\nvalues are in the format 'tagValues/456'."]
    pub fn set_vm_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.vm_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `accelerators`.\n"]
    pub fn set_accelerators(
        mut self,
        v: impl Into<BlockAssignable<WorkstationsWorkstationConfigHostElGceInstanceElAcceleratorsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.accelerators = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerators = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `boost_configs`.\n"]
    pub fn set_boost_configs(
        mut self,
        v: impl Into<BlockAssignable<WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.boost_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.boost_configs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `confidential_instance_config`.\n"]
    pub fn set_confidential_instance_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                WorkstationsWorkstationConfigHostElGceInstanceElConfidentialInstanceConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.confidential_instance_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.confidential_instance_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                WorkstationsWorkstationConfigHostElGceInstanceElShieldedInstanceConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.shielded_instance_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.shielded_instance_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for WorkstationsWorkstationConfigHostElGceInstanceEl {
    type O = BlockAssignable<WorkstationsWorkstationConfigHostElGceInstanceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkstationsWorkstationConfigHostElGceInstanceEl {}
impl BuildWorkstationsWorkstationConfigHostElGceInstanceEl {
    pub fn build(self) -> WorkstationsWorkstationConfigHostElGceInstanceEl {
        WorkstationsWorkstationConfigHostElGceInstanceEl {
            boot_disk_size_gb: core::default::Default::default(),
            disable_public_ip_addresses: core::default::Default::default(),
            disable_ssh: core::default::Default::default(),
            enable_nested_virtualization: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            pool_size: core::default::Default::default(),
            service_account: core::default::Default::default(),
            service_account_scopes: core::default::Default::default(),
            tags: core::default::Default::default(),
            vm_tags: core::default::Default::default(),
            accelerators: core::default::Default::default(),
            boost_configs: core::default::Default::default(),
            confidential_instance_config: core::default::Default::default(),
            shielded_instance_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct WorkstationsWorkstationConfigHostElGceInstanceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkstationsWorkstationConfigHostElGceInstanceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WorkstationsWorkstationConfigHostElGceInstanceElRef {
        WorkstationsWorkstationConfigHostElGceInstanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkstationsWorkstationConfigHostElGceInstanceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `boot_disk_size_gb` after provisioning.\nSize of the boot disk in GB."]
    pub fn boot_disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disable_public_ip_addresses` after provisioning.\nWhether instances have no public IP address."]
    pub fn disable_public_ip_addresses(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_public_ip_addresses", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disable_ssh` after provisioning.\nWhether to disable SSH access to the VM."]
    pub fn disable_ssh(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_ssh", self.base))
    }
    #[doc = "Get a reference to the value of field `enable_nested_virtualization` after provisioning.\nWhether to enable nested virtualization on the Compute Engine VMs backing the Workstations.\n\nSee https://cloud.google.com/workstations/docs/reference/rest/v1/projects.locations.workstationClusters.workstationConfigs#GceInstance.FIELDS.enable_nested_virtualization"]
    pub fn enable_nested_virtualization(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_nested_virtualization", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nThe name of a Compute Engine machine type."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `pool_size` after provisioning.\nNumber of instances to pool for faster workstation startup."]
    pub fn pool_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.pool_size", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nEmail address of the service account that will be used on VM instances used to support this config. This service account must have permission to pull the specified container image. If not set, VMs will run without a service account, in which case the image must be publicly accessible."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_account_scopes` after provisioning.\nScopes to grant to the service_account. Various scopes are automatically added based on feature usage. When specified, users of workstations under this configuration must have 'iam.serviceAccounts.actAs' on the service account."]
    pub fn service_account_scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_account_scopes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nNetwork tags to add to the Compute Engine machines backing the Workstations."]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
    #[doc = "Get a reference to the value of field `vm_tags` after provisioning.\nResource manager tags to be bound to the VM instances backing the Workstations.\nTag keys and values have the same definition as\nhttps://docs.cloud.google.com/resource-manager/docs/tags/tags-overview\nKeys must be in the format 'tagKeys/{tag_key_id}', and\nvalues are in the format 'tagValues/456'."]
    pub fn vm_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.vm_tags", self.base))
    }
    #[doc = "Get a reference to the value of field `accelerators` after provisioning.\n"]
    pub fn accelerators(
        &self,
    ) -> ListRef<WorkstationsWorkstationConfigHostElGceInstanceElAcceleratorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.accelerators", self.base))
    }
    #[doc = "Get a reference to the value of field `boost_configs` after provisioning.\n"]
    pub fn boost_configs(
        &self,
    ) -> ListRef<WorkstationsWorkstationConfigHostElGceInstanceElBoostConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.boost_configs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `confidential_instance_config` after provisioning.\n"]
    pub fn confidential_instance_config(
        &self,
    ) -> ListRef<WorkstationsWorkstationConfigHostElGceInstanceElConfidentialInstanceConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.confidential_instance_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(
        &self,
    ) -> ListRef<WorkstationsWorkstationConfigHostElGceInstanceElShieldedInstanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.shielded_instance_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct WorkstationsWorkstationConfigHostElDynamic {
    gce_instance: Option<DynamicBlock<WorkstationsWorkstationConfigHostElGceInstanceEl>>,
}
#[derive(Serialize)]
pub struct WorkstationsWorkstationConfigHostEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gce_instance: Option<Vec<WorkstationsWorkstationConfigHostElGceInstanceEl>>,
    dynamic: WorkstationsWorkstationConfigHostElDynamic,
}
impl WorkstationsWorkstationConfigHostEl {
    #[doc = "Set the field `gce_instance`.\n"]
    pub fn set_gce_instance(
        mut self,
        v: impl Into<BlockAssignable<WorkstationsWorkstationConfigHostElGceInstanceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gce_instance = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gce_instance = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for WorkstationsWorkstationConfigHostEl {
    type O = BlockAssignable<WorkstationsWorkstationConfigHostEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkstationsWorkstationConfigHostEl {}
impl BuildWorkstationsWorkstationConfigHostEl {
    pub fn build(self) -> WorkstationsWorkstationConfigHostEl {
        WorkstationsWorkstationConfigHostEl {
            gce_instance: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct WorkstationsWorkstationConfigHostElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkstationsWorkstationConfigHostElRef {
    fn new(shared: StackShared, base: String) -> WorkstationsWorkstationConfigHostElRef {
        WorkstationsWorkstationConfigHostElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkstationsWorkstationConfigHostElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gce_instance` after provisioning.\n"]
    pub fn gce_instance(&self) -> ListRef<WorkstationsWorkstationConfigHostElGceInstanceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gce_instance", self.base))
    }
}
#[derive(Serialize)]
pub struct WorkstationsWorkstationConfigPersistentDirectoriesElGceHdEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    archive_timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reclaim_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_snapshot: Option<PrimField<String>>,
}
impl WorkstationsWorkstationConfigPersistentDirectoriesElGceHdEl {
    #[doc = "Set the field `archive_timeout`.\nHow long to wait before converting the disk into a snapshot."]
    pub fn set_archive_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.archive_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `reclaim_policy`.\nWhether the persistent disk should be deleted when the workstation is deleted. Possible values: [\"DELETE\", \"RETAIN\"]"]
    pub fn set_reclaim_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reclaim_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `size_gb`.\nThe GB capacity of a persistent home directory. Defaults to '200'."]
    pub fn set_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `source_snapshot`.\nName of the snapshot to use as the source for the disk."]
    pub fn set_source_snapshot(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_snapshot = Some(v.into());
        self
    }
}
impl ToListMappable for WorkstationsWorkstationConfigPersistentDirectoriesElGceHdEl {
    type O = BlockAssignable<WorkstationsWorkstationConfigPersistentDirectoriesElGceHdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkstationsWorkstationConfigPersistentDirectoriesElGceHdEl {}
impl BuildWorkstationsWorkstationConfigPersistentDirectoriesElGceHdEl {
    pub fn build(self) -> WorkstationsWorkstationConfigPersistentDirectoriesElGceHdEl {
        WorkstationsWorkstationConfigPersistentDirectoriesElGceHdEl {
            archive_timeout: core::default::Default::default(),
            reclaim_policy: core::default::Default::default(),
            size_gb: core::default::Default::default(),
            source_snapshot: core::default::Default::default(),
        }
    }
}
pub struct WorkstationsWorkstationConfigPersistentDirectoriesElGceHdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkstationsWorkstationConfigPersistentDirectoriesElGceHdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WorkstationsWorkstationConfigPersistentDirectoriesElGceHdElRef {
        WorkstationsWorkstationConfigPersistentDirectoriesElGceHdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkstationsWorkstationConfigPersistentDirectoriesElGceHdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `archive_timeout` after provisioning.\nHow long to wait before converting the disk into a snapshot."]
    pub fn archive_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.archive_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `reclaim_policy` after provisioning.\nWhether the persistent disk should be deleted when the workstation is deleted. Possible values: [\"DELETE\", \"RETAIN\"]"]
    pub fn reclaim_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reclaim_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `size_gb` after provisioning.\nThe GB capacity of a persistent home directory. Defaults to '200'."]
    pub fn size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `source_snapshot` after provisioning.\nName of the snapshot to use as the source for the disk."]
    pub fn source_snapshot(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_snapshot", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct WorkstationsWorkstationConfigPersistentDirectoriesElGcePdEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fs_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reclaim_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_snapshot: Option<PrimField<String>>,
}
impl WorkstationsWorkstationConfigPersistentDirectoriesElGcePdEl {
    #[doc = "Set the field `disk_type`.\nThe type of the persistent disk for the home directory. Defaults to 'pd-standard'."]
    pub fn set_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_type = Some(v.into());
        self
    }
    #[doc = "Set the field `fs_type`.\nType of file system that the disk should be formatted with. The workstation image must support this file system type. Must be empty if 'sourceSnapshot' is set. Defaults to 'ext4'."]
    pub fn set_fs_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fs_type = Some(v.into());
        self
    }
    #[doc = "Set the field `reclaim_policy`.\nWhether the persistent disk should be deleted when the workstation is deleted. Valid values are 'DELETE' and 'RETAIN'. Defaults to 'DELETE'. Possible values: [\"DELETE\", \"RETAIN\"]"]
    pub fn set_reclaim_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reclaim_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `size_gb`.\nThe GB capacity of a persistent home directory for each workstation created with this configuration. Must be empty if 'sourceSnapshot' is set.\nValid values are '10', '50', '100', '200', '500', or '1000'. Defaults to '200'. If less than '200' GB, the 'diskType' must be 'pd-balanced' or 'pd-ssd'."]
    pub fn set_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `source_snapshot`.\nName of the snapshot to use as the source for the disk. This can be the snapshot's 'self_link', 'id', or a string in the format of 'projects/{project}/global/snapshots/{snapshot}'. If set, 'sizeGb' and 'fsType' must be empty. Can only be updated if it has an existing value."]
    pub fn set_source_snapshot(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_snapshot = Some(v.into());
        self
    }
}
impl ToListMappable for WorkstationsWorkstationConfigPersistentDirectoriesElGcePdEl {
    type O = BlockAssignable<WorkstationsWorkstationConfigPersistentDirectoriesElGcePdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkstationsWorkstationConfigPersistentDirectoriesElGcePdEl {}
impl BuildWorkstationsWorkstationConfigPersistentDirectoriesElGcePdEl {
    pub fn build(self) -> WorkstationsWorkstationConfigPersistentDirectoriesElGcePdEl {
        WorkstationsWorkstationConfigPersistentDirectoriesElGcePdEl {
            disk_type: core::default::Default::default(),
            fs_type: core::default::Default::default(),
            reclaim_policy: core::default::Default::default(),
            size_gb: core::default::Default::default(),
            source_snapshot: core::default::Default::default(),
        }
    }
}
pub struct WorkstationsWorkstationConfigPersistentDirectoriesElGcePdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkstationsWorkstationConfigPersistentDirectoriesElGcePdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WorkstationsWorkstationConfigPersistentDirectoriesElGcePdElRef {
        WorkstationsWorkstationConfigPersistentDirectoriesElGcePdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkstationsWorkstationConfigPersistentDirectoriesElGcePdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\nThe type of the persistent disk for the home directory. Defaults to 'pd-standard'."]
    pub fn disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }
    #[doc = "Get a reference to the value of field `fs_type` after provisioning.\nType of file system that the disk should be formatted with. The workstation image must support this file system type. Must be empty if 'sourceSnapshot' is set. Defaults to 'ext4'."]
    pub fn fs_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fs_type", self.base))
    }
    #[doc = "Get a reference to the value of field `reclaim_policy` after provisioning.\nWhether the persistent disk should be deleted when the workstation is deleted. Valid values are 'DELETE' and 'RETAIN'. Defaults to 'DELETE'. Possible values: [\"DELETE\", \"RETAIN\"]"]
    pub fn reclaim_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reclaim_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `size_gb` after provisioning.\nThe GB capacity of a persistent home directory for each workstation created with this configuration. Must be empty if 'sourceSnapshot' is set.\nValid values are '10', '50', '100', '200', '500', or '1000'. Defaults to '200'. If less than '200' GB, the 'diskType' must be 'pd-balanced' or 'pd-ssd'."]
    pub fn size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `source_snapshot` after provisioning.\nName of the snapshot to use as the source for the disk. This can be the snapshot's 'self_link', 'id', or a string in the format of 'projects/{project}/global/snapshots/{snapshot}'. If set, 'sizeGb' and 'fsType' must be empty. Can only be updated if it has an existing value."]
    pub fn source_snapshot(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_snapshot", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct WorkstationsWorkstationConfigPersistentDirectoriesElDynamic {
    gce_hd: Option<DynamicBlock<WorkstationsWorkstationConfigPersistentDirectoriesElGceHdEl>>,
    gce_pd: Option<DynamicBlock<WorkstationsWorkstationConfigPersistentDirectoriesElGcePdEl>>,
}
#[derive(Serialize)]
pub struct WorkstationsWorkstationConfigPersistentDirectoriesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mount_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gce_hd: Option<Vec<WorkstationsWorkstationConfigPersistentDirectoriesElGceHdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gce_pd: Option<Vec<WorkstationsWorkstationConfigPersistentDirectoriesElGcePdEl>>,
    dynamic: WorkstationsWorkstationConfigPersistentDirectoriesElDynamic,
}
impl WorkstationsWorkstationConfigPersistentDirectoriesEl {
    #[doc = "Set the field `mount_path`.\nLocation of this directory in the running workstation."]
    pub fn set_mount_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mount_path = Some(v.into());
        self
    }
    #[doc = "Set the field `gce_hd`.\n"]
    pub fn set_gce_hd(
        mut self,
        v: impl Into<BlockAssignable<WorkstationsWorkstationConfigPersistentDirectoriesElGceHdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gce_hd = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gce_hd = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gce_pd`.\n"]
    pub fn set_gce_pd(
        mut self,
        v: impl Into<BlockAssignable<WorkstationsWorkstationConfigPersistentDirectoriesElGcePdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gce_pd = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gce_pd = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for WorkstationsWorkstationConfigPersistentDirectoriesEl {
    type O = BlockAssignable<WorkstationsWorkstationConfigPersistentDirectoriesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkstationsWorkstationConfigPersistentDirectoriesEl {}
impl BuildWorkstationsWorkstationConfigPersistentDirectoriesEl {
    pub fn build(self) -> WorkstationsWorkstationConfigPersistentDirectoriesEl {
        WorkstationsWorkstationConfigPersistentDirectoriesEl {
            mount_path: core::default::Default::default(),
            gce_hd: core::default::Default::default(),
            gce_pd: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct WorkstationsWorkstationConfigPersistentDirectoriesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkstationsWorkstationConfigPersistentDirectoriesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WorkstationsWorkstationConfigPersistentDirectoriesElRef {
        WorkstationsWorkstationConfigPersistentDirectoriesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkstationsWorkstationConfigPersistentDirectoriesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\nLocation of this directory in the running workstation."]
    pub fn mount_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }
    #[doc = "Get a reference to the value of field `gce_hd` after provisioning.\n"]
    pub fn gce_hd(
        &self,
    ) -> ListRef<WorkstationsWorkstationConfigPersistentDirectoriesElGceHdElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gce_hd", self.base))
    }
    #[doc = "Get a reference to the value of field `gce_pd` after provisioning.\n"]
    pub fn gce_pd(
        &self,
    ) -> ListRef<WorkstationsWorkstationConfigPersistentDirectoriesElGcePdElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gce_pd", self.base))
    }
}
#[derive(Serialize)]
pub struct WorkstationsWorkstationConfigReadinessChecksEl {
    path: PrimField<String>,
    port: PrimField<f64>,
}
impl WorkstationsWorkstationConfigReadinessChecksEl {}
impl ToListMappable for WorkstationsWorkstationConfigReadinessChecksEl {
    type O = BlockAssignable<WorkstationsWorkstationConfigReadinessChecksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkstationsWorkstationConfigReadinessChecksEl {
    #[doc = "Path to which the request should be sent."]
    pub path: PrimField<String>,
    #[doc = "Port to which the request should be sent."]
    pub port: PrimField<f64>,
}
impl BuildWorkstationsWorkstationConfigReadinessChecksEl {
    pub fn build(self) -> WorkstationsWorkstationConfigReadinessChecksEl {
        WorkstationsWorkstationConfigReadinessChecksEl {
            path: self.path,
            port: self.port,
        }
    }
}
pub struct WorkstationsWorkstationConfigReadinessChecksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkstationsWorkstationConfigReadinessChecksElRef {
    fn new(shared: StackShared, base: String) -> WorkstationsWorkstationConfigReadinessChecksElRef {
        WorkstationsWorkstationConfigReadinessChecksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkstationsWorkstationConfigReadinessChecksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nPath to which the request should be sent."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort to which the request should be sent."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct WorkstationsWorkstationConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl WorkstationsWorkstationConfigTimeoutsEl {
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
impl ToListMappable for WorkstationsWorkstationConfigTimeoutsEl {
    type O = BlockAssignable<WorkstationsWorkstationConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkstationsWorkstationConfigTimeoutsEl {}
impl BuildWorkstationsWorkstationConfigTimeoutsEl {
    pub fn build(self) -> WorkstationsWorkstationConfigTimeoutsEl {
        WorkstationsWorkstationConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct WorkstationsWorkstationConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkstationsWorkstationConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> WorkstationsWorkstationConfigTimeoutsElRef {
        WorkstationsWorkstationConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkstationsWorkstationConfigTimeoutsElRef {
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
struct WorkstationsWorkstationConfigDynamic {
    allowed_ports: Option<DynamicBlock<WorkstationsWorkstationConfigAllowedPortsEl>>,
    container: Option<DynamicBlock<WorkstationsWorkstationConfigContainerEl>>,
    encryption_key: Option<DynamicBlock<WorkstationsWorkstationConfigEncryptionKeyEl>>,
    ephemeral_directories:
        Option<DynamicBlock<WorkstationsWorkstationConfigEphemeralDirectoriesEl>>,
    host: Option<DynamicBlock<WorkstationsWorkstationConfigHostEl>>,
    persistent_directories:
        Option<DynamicBlock<WorkstationsWorkstationConfigPersistentDirectoriesEl>>,
    readiness_checks: Option<DynamicBlock<WorkstationsWorkstationConfigReadinessChecksEl>>,
}
