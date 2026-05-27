use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct MonitoringUptimeCheckConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    checker_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_check_failures: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selected_regions: Option<ListField<PrimField<String>>>,
    timeout: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_matchers: Option<Vec<MonitoringUptimeCheckConfigContentMatchersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_check: Option<Vec<MonitoringUptimeCheckConfigHttpCheckEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitored_resource: Option<Vec<MonitoringUptimeCheckConfigMonitoredResourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_group: Option<Vec<MonitoringUptimeCheckConfigResourceGroupEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    synthetic_monitor: Option<Vec<MonitoringUptimeCheckConfigSyntheticMonitorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_check: Option<Vec<MonitoringUptimeCheckConfigTcpCheckEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<MonitoringUptimeCheckConfigTimeoutsEl>,
    dynamic: MonitoringUptimeCheckConfigDynamic,
}
struct MonitoringUptimeCheckConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<MonitoringUptimeCheckConfigData>,
}
#[derive(Clone)]
pub struct MonitoringUptimeCheckConfig(Rc<MonitoringUptimeCheckConfig_>);
impl MonitoringUptimeCheckConfig {
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
    #[doc = "Set the field `checker_type`.\nThe checker type to use for the check. If the monitored resource type is 'servicedirectory_service', 'checker_type' must be set to 'VPC_CHECKERS'. Possible values: [\"STATIC_IP_CHECKERS\", \"VPC_CHECKERS\"]"]
    pub fn set_checker_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().checker_type = Some(v.into());
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
    #[doc = "Set the field `log_check_failures`.\nSpecifies whether to log the results of failed probes to Cloud Logging."]
    pub fn set_log_check_failures(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().log_check_failures = Some(v.into());
        self
    }
    #[doc = "Set the field `period`.\nHow often, in seconds, the uptime check is performed. Currently, the only supported values are 60s (1 minute), 300s (5 minutes), 600s (10 minutes), and 900s (15 minutes). Optional, defaults to 300s."]
    pub fn set_period(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().period = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `selected_regions`.\nThe list of regions from which the check will be run. Some regions contain one location, and others contain more than one. If this field is specified, enough regions to include a minimum of 3 locations must be provided, or an error message is returned. Not specifying this field will result in uptime checks running from all regions."]
    pub fn set_selected_regions(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().selected_regions = Some(v.into());
        self
    }
    #[doc = "Set the field `user_labels`.\nUser-supplied key/value data to be used for organizing and identifying the 'UptimeCheckConfig' objects. The field can contain up to 64 entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter."]
    pub fn set_user_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().user_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `content_matchers`.\n"]
    pub fn set_content_matchers(
        self,
        v: impl Into<BlockAssignable<MonitoringUptimeCheckConfigContentMatchersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().content_matchers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.content_matchers = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `http_check`.\n"]
    pub fn set_http_check(
        self,
        v: impl Into<BlockAssignable<MonitoringUptimeCheckConfigHttpCheckEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().http_check = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.http_check = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `monitored_resource`.\n"]
    pub fn set_monitored_resource(
        self,
        v: impl Into<BlockAssignable<MonitoringUptimeCheckConfigMonitoredResourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().monitored_resource = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.monitored_resource = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_group`.\n"]
    pub fn set_resource_group(
        self,
        v: impl Into<BlockAssignable<MonitoringUptimeCheckConfigResourceGroupEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_group = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_group = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `synthetic_monitor`.\n"]
    pub fn set_synthetic_monitor(
        self,
        v: impl Into<BlockAssignable<MonitoringUptimeCheckConfigSyntheticMonitorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().synthetic_monitor = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.synthetic_monitor = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tcp_check`.\n"]
    pub fn set_tcp_check(
        self,
        v: impl Into<BlockAssignable<MonitoringUptimeCheckConfigTcpCheckEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tcp_check = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tcp_check = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<MonitoringUptimeCheckConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `checker_type` after provisioning.\nThe checker type to use for the check. If the monitored resource type is 'servicedirectory_service', 'checker_type' must be set to 'VPC_CHECKERS'. Possible values: [\"STATIC_IP_CHECKERS\", \"VPC_CHECKERS\"]"]
    pub fn checker_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.checker_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA human-friendly name for the uptime check configuration. The display name should be unique within a Stackdriver Workspace in order to make it easier to identify; however, uniqueness is not enforced."]
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
    #[doc = "Get a reference to the value of field `log_check_failures` after provisioning.\nSpecifies whether to log the results of failed probes to Cloud Logging."]
    pub fn log_check_failures(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_check_failures", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA unique resource name for this UptimeCheckConfig. The format is 'projects/[PROJECT_ID]/uptimeCheckConfigs/[UPTIME_CHECK_ID]'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `period` after provisioning.\nHow often, in seconds, the uptime check is performed. Currently, the only supported values are 60s (1 minute), 300s (5 minutes), 600s (10 minutes), and 900s (15 minutes). Optional, defaults to 300s."]
    pub fn period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.period", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `selected_regions` after provisioning.\nThe list of regions from which the check will be run. Some regions contain one location, and others contain more than one. If this field is specified, enough regions to include a minimum of 3 locations must be provided, or an error message is returned. Not specifying this field will result in uptime checks running from all regions."]
    pub fn selected_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.selected_regions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\nThe maximum amount of time to wait for the request to complete (must be between 1 and 60 seconds). [See the accepted formats]( https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Duration)"]
    pub fn timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uptime_check_id` after provisioning.\nThe id of the uptime check"]
    pub fn uptime_check_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.uptime_check_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_labels` after provisioning.\nUser-supplied key/value data to be used for organizing and identifying the 'UptimeCheckConfig' objects. The field can contain up to 64 entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter."]
    pub fn user_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.user_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_matchers` after provisioning.\n"]
    pub fn content_matchers(&self) -> ListRef<MonitoringUptimeCheckConfigContentMatchersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.content_matchers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `http_check` after provisioning.\n"]
    pub fn http_check(&self) -> ListRef<MonitoringUptimeCheckConfigHttpCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monitored_resource` after provisioning.\n"]
    pub fn monitored_resource(&self) -> ListRef<MonitoringUptimeCheckConfigMonitoredResourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.monitored_resource", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_group` after provisioning.\n"]
    pub fn resource_group(&self) -> ListRef<MonitoringUptimeCheckConfigResourceGroupElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `synthetic_monitor` after provisioning.\n"]
    pub fn synthetic_monitor(&self) -> ListRef<MonitoringUptimeCheckConfigSyntheticMonitorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.synthetic_monitor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tcp_check` after provisioning.\n"]
    pub fn tcp_check(&self) -> ListRef<MonitoringUptimeCheckConfigTcpCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tcp_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MonitoringUptimeCheckConfigTimeoutsElRef {
        MonitoringUptimeCheckConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for MonitoringUptimeCheckConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for MonitoringUptimeCheckConfig {}
impl ToListMappable for MonitoringUptimeCheckConfig {
    type O = ListRef<MonitoringUptimeCheckConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for MonitoringUptimeCheckConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_monitoring_uptime_check_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildMonitoringUptimeCheckConfig {
    pub tf_id: String,
    #[doc = "A human-friendly name for the uptime check configuration. The display name should be unique within a Stackdriver Workspace in order to make it easier to identify; however, uniqueness is not enforced."]
    pub display_name: PrimField<String>,
    #[doc = "The maximum amount of time to wait for the request to complete (must be between 1 and 60 seconds). [See the accepted formats]( https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Duration)"]
    pub timeout: PrimField<String>,
}
impl BuildMonitoringUptimeCheckConfig {
    pub fn build(self, stack: &mut Stack) -> MonitoringUptimeCheckConfig {
        let out = MonitoringUptimeCheckConfig(Rc::new(MonitoringUptimeCheckConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(MonitoringUptimeCheckConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                checker_type: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                log_check_failures: core::default::Default::default(),
                period: core::default::Default::default(),
                project: core::default::Default::default(),
                selected_regions: core::default::Default::default(),
                timeout: self.timeout,
                user_labels: core::default::Default::default(),
                content_matchers: core::default::Default::default(),
                http_check: core::default::Default::default(),
                monitored_resource: core::default::Default::default(),
                resource_group: core::default::Default::default(),
                synthetic_monitor: core::default::Default::default(),
                tcp_check: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct MonitoringUptimeCheckConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringUptimeCheckConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl MonitoringUptimeCheckConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `checker_type` after provisioning.\nThe checker type to use for the check. If the monitored resource type is 'servicedirectory_service', 'checker_type' must be set to 'VPC_CHECKERS'. Possible values: [\"STATIC_IP_CHECKERS\", \"VPC_CHECKERS\"]"]
    pub fn checker_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.checker_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA human-friendly name for the uptime check configuration. The display name should be unique within a Stackdriver Workspace in order to make it easier to identify; however, uniqueness is not enforced."]
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
    #[doc = "Get a reference to the value of field `log_check_failures` after provisioning.\nSpecifies whether to log the results of failed probes to Cloud Logging."]
    pub fn log_check_failures(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_check_failures", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA unique resource name for this UptimeCheckConfig. The format is 'projects/[PROJECT_ID]/uptimeCheckConfigs/[UPTIME_CHECK_ID]'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `period` after provisioning.\nHow often, in seconds, the uptime check is performed. Currently, the only supported values are 60s (1 minute), 300s (5 minutes), 600s (10 minutes), and 900s (15 minutes). Optional, defaults to 300s."]
    pub fn period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.period", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `selected_regions` after provisioning.\nThe list of regions from which the check will be run. Some regions contain one location, and others contain more than one. If this field is specified, enough regions to include a minimum of 3 locations must be provided, or an error message is returned. Not specifying this field will result in uptime checks running from all regions."]
    pub fn selected_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.selected_regions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\nThe maximum amount of time to wait for the request to complete (must be between 1 and 60 seconds). [See the accepted formats]( https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Duration)"]
    pub fn timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uptime_check_id` after provisioning.\nThe id of the uptime check"]
    pub fn uptime_check_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.uptime_check_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_labels` after provisioning.\nUser-supplied key/value data to be used for organizing and identifying the 'UptimeCheckConfig' objects. The field can contain up to 64 entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter."]
    pub fn user_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.user_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_matchers` after provisioning.\n"]
    pub fn content_matchers(&self) -> ListRef<MonitoringUptimeCheckConfigContentMatchersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.content_matchers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `http_check` after provisioning.\n"]
    pub fn http_check(&self) -> ListRef<MonitoringUptimeCheckConfigHttpCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monitored_resource` after provisioning.\n"]
    pub fn monitored_resource(&self) -> ListRef<MonitoringUptimeCheckConfigMonitoredResourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.monitored_resource", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_group` after provisioning.\n"]
    pub fn resource_group(&self) -> ListRef<MonitoringUptimeCheckConfigResourceGroupElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `synthetic_monitor` after provisioning.\n"]
    pub fn synthetic_monitor(&self) -> ListRef<MonitoringUptimeCheckConfigSyntheticMonitorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.synthetic_monitor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tcp_check` after provisioning.\n"]
    pub fn tcp_check(&self) -> ListRef<MonitoringUptimeCheckConfigTcpCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tcp_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MonitoringUptimeCheckConfigTimeoutsElRef {
        MonitoringUptimeCheckConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfigContentMatchersElJsonPathMatcherEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    json_matcher: Option<PrimField<String>>,
    json_path: PrimField<String>,
}
impl MonitoringUptimeCheckConfigContentMatchersElJsonPathMatcherEl {
    #[doc = "Set the field `json_matcher`.\nOptions to perform JSONPath content matching. Default value: \"EXACT_MATCH\" Possible values: [\"EXACT_MATCH\", \"REGEX_MATCH\"]"]
    pub fn set_json_matcher(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.json_matcher = Some(v.into());
        self
    }
}
impl ToListMappable for MonitoringUptimeCheckConfigContentMatchersElJsonPathMatcherEl {
    type O = BlockAssignable<MonitoringUptimeCheckConfigContentMatchersElJsonPathMatcherEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringUptimeCheckConfigContentMatchersElJsonPathMatcherEl {
    #[doc = "JSONPath within the response output pointing to the expected 'ContentMatcher::content' to match against."]
    pub json_path: PrimField<String>,
}
impl BuildMonitoringUptimeCheckConfigContentMatchersElJsonPathMatcherEl {
    pub fn build(self) -> MonitoringUptimeCheckConfigContentMatchersElJsonPathMatcherEl {
        MonitoringUptimeCheckConfigContentMatchersElJsonPathMatcherEl {
            json_matcher: core::default::Default::default(),
            json_path: self.json_path,
        }
    }
}
pub struct MonitoringUptimeCheckConfigContentMatchersElJsonPathMatcherElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringUptimeCheckConfigContentMatchersElJsonPathMatcherElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringUptimeCheckConfigContentMatchersElJsonPathMatcherElRef {
        MonitoringUptimeCheckConfigContentMatchersElJsonPathMatcherElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringUptimeCheckConfigContentMatchersElJsonPathMatcherElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `json_matcher` after provisioning.\nOptions to perform JSONPath content matching. Default value: \"EXACT_MATCH\" Possible values: [\"EXACT_MATCH\", \"REGEX_MATCH\"]"]
    pub fn json_matcher(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.json_matcher", self.base))
    }
    #[doc = "Get a reference to the value of field `json_path` after provisioning.\nJSONPath within the response output pointing to the expected 'ContentMatcher::content' to match against."]
    pub fn json_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.json_path", self.base))
    }
}
#[derive(Serialize, Default)]
struct MonitoringUptimeCheckConfigContentMatchersElDynamic {
    json_path_matcher:
        Option<DynamicBlock<MonitoringUptimeCheckConfigContentMatchersElJsonPathMatcherEl>>,
}
#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfigContentMatchersEl {
    content: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matcher: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json_path_matcher: Option<Vec<MonitoringUptimeCheckConfigContentMatchersElJsonPathMatcherEl>>,
    dynamic: MonitoringUptimeCheckConfigContentMatchersElDynamic,
}
impl MonitoringUptimeCheckConfigContentMatchersEl {
    #[doc = "Set the field `matcher`.\nThe type of content matcher that will be applied to the server output, compared to the content string when the check is run. Default value: \"CONTAINS_STRING\" Possible values: [\"CONTAINS_STRING\", \"NOT_CONTAINS_STRING\", \"MATCHES_REGEX\", \"NOT_MATCHES_REGEX\", \"MATCHES_JSON_PATH\", \"NOT_MATCHES_JSON_PATH\"]"]
    pub fn set_matcher(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.matcher = Some(v.into());
        self
    }
    #[doc = "Set the field `json_path_matcher`.\n"]
    pub fn set_json_path_matcher(
        mut self,
        v: impl Into<BlockAssignable<MonitoringUptimeCheckConfigContentMatchersElJsonPathMatcherEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.json_path_matcher = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.json_path_matcher = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MonitoringUptimeCheckConfigContentMatchersEl {
    type O = BlockAssignable<MonitoringUptimeCheckConfigContentMatchersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringUptimeCheckConfigContentMatchersEl {
    #[doc = "String or regex content to match (max 1024 bytes)"]
    pub content: PrimField<String>,
}
impl BuildMonitoringUptimeCheckConfigContentMatchersEl {
    pub fn build(self) -> MonitoringUptimeCheckConfigContentMatchersEl {
        MonitoringUptimeCheckConfigContentMatchersEl {
            content: self.content,
            matcher: core::default::Default::default(),
            json_path_matcher: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MonitoringUptimeCheckConfigContentMatchersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringUptimeCheckConfigContentMatchersElRef {
    fn new(shared: StackShared, base: String) -> MonitoringUptimeCheckConfigContentMatchersElRef {
        MonitoringUptimeCheckConfigContentMatchersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringUptimeCheckConfigContentMatchersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `content` after provisioning.\nString or regex content to match (max 1024 bytes)"]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }
    #[doc = "Get a reference to the value of field `matcher` after provisioning.\nThe type of content matcher that will be applied to the server output, compared to the content string when the check is run. Default value: \"CONTAINS_STRING\" Possible values: [\"CONTAINS_STRING\", \"NOT_CONTAINS_STRING\", \"MATCHES_REGEX\", \"NOT_MATCHES_REGEX\", \"MATCHES_JSON_PATH\", \"NOT_MATCHES_JSON_PATH\"]"]
    pub fn matcher(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.matcher", self.base))
    }
    #[doc = "Get a reference to the value of field `json_path_matcher` after provisioning.\n"]
    pub fn json_path_matcher(
        &self,
    ) -> ListRef<MonitoringUptimeCheckConfigContentMatchersElJsonPathMatcherElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.json_path_matcher", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfigHttpCheckElAcceptedResponseStatusCodesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    status_class: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_value: Option<PrimField<f64>>,
}
impl MonitoringUptimeCheckConfigHttpCheckElAcceptedResponseStatusCodesEl {
    #[doc = "Set the field `status_class`.\nA class of status codes to accept. Possible values: [\"STATUS_CLASS_1XX\", \"STATUS_CLASS_2XX\", \"STATUS_CLASS_3XX\", \"STATUS_CLASS_4XX\", \"STATUS_CLASS_5XX\", \"STATUS_CLASS_ANY\"]"]
    pub fn set_status_class(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status_class = Some(v.into());
        self
    }
    #[doc = "Set the field `status_value`.\nA status code to accept."]
    pub fn set_status_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.status_value = Some(v.into());
        self
    }
}
impl ToListMappable for MonitoringUptimeCheckConfigHttpCheckElAcceptedResponseStatusCodesEl {
    type O = BlockAssignable<MonitoringUptimeCheckConfigHttpCheckElAcceptedResponseStatusCodesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringUptimeCheckConfigHttpCheckElAcceptedResponseStatusCodesEl {}
impl BuildMonitoringUptimeCheckConfigHttpCheckElAcceptedResponseStatusCodesEl {
    pub fn build(self) -> MonitoringUptimeCheckConfigHttpCheckElAcceptedResponseStatusCodesEl {
        MonitoringUptimeCheckConfigHttpCheckElAcceptedResponseStatusCodesEl {
            status_class: core::default::Default::default(),
            status_value: core::default::Default::default(),
        }
    }
}
pub struct MonitoringUptimeCheckConfigHttpCheckElAcceptedResponseStatusCodesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringUptimeCheckConfigHttpCheckElAcceptedResponseStatusCodesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringUptimeCheckConfigHttpCheckElAcceptedResponseStatusCodesElRef {
        MonitoringUptimeCheckConfigHttpCheckElAcceptedResponseStatusCodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringUptimeCheckConfigHttpCheckElAcceptedResponseStatusCodesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `status_class` after provisioning.\nA class of status codes to accept. Possible values: [\"STATUS_CLASS_1XX\", \"STATUS_CLASS_2XX\", \"STATUS_CLASS_3XX\", \"STATUS_CLASS_4XX\", \"STATUS_CLASS_5XX\", \"STATUS_CLASS_ANY\"]"]
    pub fn status_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_class", self.base))
    }
    #[doc = "Get a reference to the value of field `status_value` after provisioning.\nA status code to accept."]
    pub fn status_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_value", self.base))
    }
}
#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfigHttpCheckElAuthInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_wo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_wo_version: Option<PrimField<String>>,
    username: PrimField<String>,
}
impl MonitoringUptimeCheckConfigHttpCheckElAuthInfoEl {
    #[doc = "Set the field `password`.\nThe password to authenticate."]
    pub fn set_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password = Some(v.into());
        self
    }
    #[doc = "Set the field `password_wo`.\nThe password to authenticate."]
    pub fn set_password_wo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password_wo = Some(v.into());
        self
    }
    #[doc = "Set the field `password_wo_version`.\nThe password write-only version."]
    pub fn set_password_wo_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password_wo_version = Some(v.into());
        self
    }
}
impl ToListMappable for MonitoringUptimeCheckConfigHttpCheckElAuthInfoEl {
    type O = BlockAssignable<MonitoringUptimeCheckConfigHttpCheckElAuthInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringUptimeCheckConfigHttpCheckElAuthInfoEl {
    #[doc = "The username to authenticate."]
    pub username: PrimField<String>,
}
impl BuildMonitoringUptimeCheckConfigHttpCheckElAuthInfoEl {
    pub fn build(self) -> MonitoringUptimeCheckConfigHttpCheckElAuthInfoEl {
        MonitoringUptimeCheckConfigHttpCheckElAuthInfoEl {
            password: core::default::Default::default(),
            password_wo: core::default::Default::default(),
            password_wo_version: core::default::Default::default(),
            username: self.username,
        }
    }
}
pub struct MonitoringUptimeCheckConfigHttpCheckElAuthInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringUptimeCheckConfigHttpCheckElAuthInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringUptimeCheckConfigHttpCheckElAuthInfoElRef {
        MonitoringUptimeCheckConfigHttpCheckElAuthInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringUptimeCheckConfigHttpCheckElAuthInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nThe password to authenticate."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `password_wo` after provisioning.\nThe password to authenticate."]
    pub fn password_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_wo", self.base))
    }
    #[doc = "Get a reference to the value of field `password_wo_version` after provisioning.\nThe password write-only version."]
    pub fn password_wo_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password_wo_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nThe username to authenticate."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfigHttpCheckElPingConfigEl {
    pings_count: PrimField<f64>,
}
impl MonitoringUptimeCheckConfigHttpCheckElPingConfigEl {}
impl ToListMappable for MonitoringUptimeCheckConfigHttpCheckElPingConfigEl {
    type O = BlockAssignable<MonitoringUptimeCheckConfigHttpCheckElPingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringUptimeCheckConfigHttpCheckElPingConfigEl {
    #[doc = "Number of ICMP pings. A maximum of 3 ICMP pings is currently supported."]
    pub pings_count: PrimField<f64>,
}
impl BuildMonitoringUptimeCheckConfigHttpCheckElPingConfigEl {
    pub fn build(self) -> MonitoringUptimeCheckConfigHttpCheckElPingConfigEl {
        MonitoringUptimeCheckConfigHttpCheckElPingConfigEl {
            pings_count: self.pings_count,
        }
    }
}
pub struct MonitoringUptimeCheckConfigHttpCheckElPingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringUptimeCheckConfigHttpCheckElPingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringUptimeCheckConfigHttpCheckElPingConfigElRef {
        MonitoringUptimeCheckConfigHttpCheckElPingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringUptimeCheckConfigHttpCheckElPingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pings_count` after provisioning.\nNumber of ICMP pings. A maximum of 3 ICMP pings is currently supported."]
    pub fn pings_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.pings_count", self.base))
    }
}
#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfigHttpCheckElServiceAgentAuthenticationEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl MonitoringUptimeCheckConfigHttpCheckElServiceAgentAuthenticationEl {
    #[doc = "Set the field `type_`.\nThe type of authentication to use. Possible values: [\"SERVICE_AGENT_AUTHENTICATION_TYPE_UNSPECIFIED\", \"OIDC_TOKEN\"]"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for MonitoringUptimeCheckConfigHttpCheckElServiceAgentAuthenticationEl {
    type O = BlockAssignable<MonitoringUptimeCheckConfigHttpCheckElServiceAgentAuthenticationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringUptimeCheckConfigHttpCheckElServiceAgentAuthenticationEl {}
impl BuildMonitoringUptimeCheckConfigHttpCheckElServiceAgentAuthenticationEl {
    pub fn build(self) -> MonitoringUptimeCheckConfigHttpCheckElServiceAgentAuthenticationEl {
        MonitoringUptimeCheckConfigHttpCheckElServiceAgentAuthenticationEl {
            type_: core::default::Default::default(),
        }
    }
}
pub struct MonitoringUptimeCheckConfigHttpCheckElServiceAgentAuthenticationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringUptimeCheckConfigHttpCheckElServiceAgentAuthenticationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringUptimeCheckConfigHttpCheckElServiceAgentAuthenticationElRef {
        MonitoringUptimeCheckConfigHttpCheckElServiceAgentAuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringUptimeCheckConfigHttpCheckElServiceAgentAuthenticationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of authentication to use. Possible values: [\"SERVICE_AGENT_AUTHENTICATION_TYPE_UNSPECIFIED\", \"OIDC_TOKEN\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize, Default)]
struct MonitoringUptimeCheckConfigHttpCheckElDynamic {
    accepted_response_status_codes:
        Option<DynamicBlock<MonitoringUptimeCheckConfigHttpCheckElAcceptedResponseStatusCodesEl>>,
    auth_info: Option<DynamicBlock<MonitoringUptimeCheckConfigHttpCheckElAuthInfoEl>>,
    ping_config: Option<DynamicBlock<MonitoringUptimeCheckConfigHttpCheckElPingConfigEl>>,
    service_agent_authentication:
        Option<DynamicBlock<MonitoringUptimeCheckConfigHttpCheckElServiceAgentAuthenticationEl>>,
}
#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfigHttpCheckEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_content_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mask_headers: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_ssl: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validate_ssl: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accepted_response_status_codes:
        Option<Vec<MonitoringUptimeCheckConfigHttpCheckElAcceptedResponseStatusCodesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_info: Option<Vec<MonitoringUptimeCheckConfigHttpCheckElAuthInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ping_config: Option<Vec<MonitoringUptimeCheckConfigHttpCheckElPingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_agent_authentication:
        Option<Vec<MonitoringUptimeCheckConfigHttpCheckElServiceAgentAuthenticationEl>>,
    dynamic: MonitoringUptimeCheckConfigHttpCheckElDynamic,
}
impl MonitoringUptimeCheckConfigHttpCheckEl {
    #[doc = "Set the field `body`.\nThe request body associated with the HTTP POST request. If 'content_type' is 'URL_ENCODED', the body passed in must be URL-encoded. Users can provide a 'Content-Length' header via the 'headers' field or the API will do so. If the 'request_method' is 'GET' and 'body' is not empty, the API will return an error. The maximum byte size is 1 megabyte. Note - As with all bytes fields JSON representations are base64 encoded. e.g. 'foo=bar' in URL-encoded form is 'foo%3Dbar' and in base64 encoding is 'Zm9vJTI1M0RiYXI='."]
    pub fn set_body(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.body = Some(v.into());
        self
    }
    #[doc = "Set the field `content_type`.\nThe content type to use for the check. Possible values: [\"TYPE_UNSPECIFIED\", \"URL_ENCODED\", \"USER_PROVIDED\"]"]
    pub fn set_content_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_type = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_content_type`.\nA user provided content type header to use for the check. The invalid configurations outlined in the 'content_type' field apply to custom_content_type', as well as the following 1. 'content_type' is 'URL_ENCODED' and 'custom_content_type' is set. 2. 'content_type' is 'USER_PROVIDED' and 'custom_content_type' is not set."]
    pub fn set_custom_content_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.custom_content_type = Some(v.into());
        self
    }
    #[doc = "Set the field `headers`.\nThe list of headers to send as part of the uptime check request. If two headers have the same key and different values, they should be entered as a single header, with the value being a comma-separated list of all the desired values as described in [RFC 2616 (page 31)](https://www.w3.org/Protocols/rfc2616/rfc2616.txt). Entering two separate headers with the same key in a Create call will cause the first to be overwritten by the second. The maximum number of headers allowed is 100."]
    pub fn set_headers(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.headers = Some(v.into());
        self
    }
    #[doc = "Set the field `mask_headers`.\nBoolean specifying whether to encrypt the header information. Encryption should be specified for any headers related to authentication that you do not wish to be seen when retrieving the configuration. The server will be responsible for encrypting the headers. On Get/List calls, if 'mask_headers' is set to 'true' then the headers will be obscured with '******'."]
    pub fn set_mask_headers(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.mask_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\nThe path to the page to run the check against. Will be combined with the host (specified within the MonitoredResource) and port to construct the full URL. If the provided path does not begin with '/', a '/' will be prepended automatically. Optional (defaults to '/')."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nThe port to the page to run the check against. Will be combined with 'host' (specified within the ['monitored_resource'](#nested_monitored_resource)) and path to construct the full URL. Optional (defaults to 80 without SSL, or 443 with SSL)."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `request_method`.\nThe HTTP request method to use for the check. If set to 'METHOD_UNSPECIFIED' then 'request_method' defaults to 'GET'. Default value: \"GET\" Possible values: [\"METHOD_UNSPECIFIED\", \"GET\", \"POST\"]"]
    pub fn set_request_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.request_method = Some(v.into());
        self
    }
    #[doc = "Set the field `use_ssl`.\nIf true, use HTTPS instead of HTTP to run the check."]
    pub fn set_use_ssl(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_ssl = Some(v.into());
        self
    }
    #[doc = "Set the field `validate_ssl`.\nBoolean specifying whether to include SSL certificate validation as a part of the Uptime check. Only applies to checks where 'monitored_resource' is set to 'uptime_url'. If 'use_ssl' is 'false', setting 'validate_ssl' to 'true' has no effect."]
    pub fn set_validate_ssl(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.validate_ssl = Some(v.into());
        self
    }
    #[doc = "Set the field `accepted_response_status_codes`.\n"]
    pub fn set_accepted_response_status_codes(
        mut self,
        v: impl Into<
            BlockAssignable<MonitoringUptimeCheckConfigHttpCheckElAcceptedResponseStatusCodesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.accepted_response_status_codes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.accepted_response_status_codes = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `auth_info`.\n"]
    pub fn set_auth_info(
        mut self,
        v: impl Into<BlockAssignable<MonitoringUptimeCheckConfigHttpCheckElAuthInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.auth_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.auth_info = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ping_config`.\n"]
    pub fn set_ping_config(
        mut self,
        v: impl Into<BlockAssignable<MonitoringUptimeCheckConfigHttpCheckElPingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ping_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ping_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `service_agent_authentication`.\n"]
    pub fn set_service_agent_authentication(
        mut self,
        v: impl Into<
            BlockAssignable<MonitoringUptimeCheckConfigHttpCheckElServiceAgentAuthenticationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_agent_authentication = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_agent_authentication = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MonitoringUptimeCheckConfigHttpCheckEl {
    type O = BlockAssignable<MonitoringUptimeCheckConfigHttpCheckEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringUptimeCheckConfigHttpCheckEl {}
impl BuildMonitoringUptimeCheckConfigHttpCheckEl {
    pub fn build(self) -> MonitoringUptimeCheckConfigHttpCheckEl {
        MonitoringUptimeCheckConfigHttpCheckEl {
            body: core::default::Default::default(),
            content_type: core::default::Default::default(),
            custom_content_type: core::default::Default::default(),
            headers: core::default::Default::default(),
            mask_headers: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            request_method: core::default::Default::default(),
            use_ssl: core::default::Default::default(),
            validate_ssl: core::default::Default::default(),
            accepted_response_status_codes: core::default::Default::default(),
            auth_info: core::default::Default::default(),
            ping_config: core::default::Default::default(),
            service_agent_authentication: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MonitoringUptimeCheckConfigHttpCheckElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringUptimeCheckConfigHttpCheckElRef {
    fn new(shared: StackShared, base: String) -> MonitoringUptimeCheckConfigHttpCheckElRef {
        MonitoringUptimeCheckConfigHttpCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringUptimeCheckConfigHttpCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `body` after provisioning.\nThe request body associated with the HTTP POST request. If 'content_type' is 'URL_ENCODED', the body passed in must be URL-encoded. Users can provide a 'Content-Length' header via the 'headers' field or the API will do so. If the 'request_method' is 'GET' and 'body' is not empty, the API will return an error. The maximum byte size is 1 megabyte. Note - As with all bytes fields JSON representations are base64 encoded. e.g. 'foo=bar' in URL-encoded form is 'foo%3Dbar' and in base64 encoding is 'Zm9vJTI1M0RiYXI='."]
    pub fn body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.body", self.base))
    }
    #[doc = "Get a reference to the value of field `content_type` after provisioning.\nThe content type to use for the check. Possible values: [\"TYPE_UNSPECIFIED\", \"URL_ENCODED\", \"USER_PROVIDED\"]"]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }
    #[doc = "Get a reference to the value of field `custom_content_type` after provisioning.\nA user provided content type header to use for the check. The invalid configurations outlined in the 'content_type' field apply to custom_content_type', as well as the following 1. 'content_type' is 'URL_ENCODED' and 'custom_content_type' is set. 2. 'content_type' is 'USER_PROVIDED' and 'custom_content_type' is not set."]
    pub fn custom_content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_content_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `headers` after provisioning.\nThe list of headers to send as part of the uptime check request. If two headers have the same key and different values, they should be entered as a single header, with the value being a comma-separated list of all the desired values as described in [RFC 2616 (page 31)](https://www.w3.org/Protocols/rfc2616/rfc2616.txt). Entering two separate headers with the same key in a Create call will cause the first to be overwritten by the second. The maximum number of headers allowed is 100."]
    pub fn headers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }
    #[doc = "Get a reference to the value of field `mask_headers` after provisioning.\nBoolean specifying whether to encrypt the header information. Encryption should be specified for any headers related to authentication that you do not wish to be seen when retrieving the configuration. The server will be responsible for encrypting the headers. On Get/List calls, if 'mask_headers' is set to 'true' then the headers will be obscured with '******'."]
    pub fn mask_headers(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mask_headers", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nThe path to the page to run the check against. Will be combined with the host (specified within the MonitoredResource) and port to construct the full URL. If the provided path does not begin with '/', a '/' will be prepended automatically. Optional (defaults to '/')."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe port to the page to run the check against. Will be combined with 'host' (specified within the ['monitored_resource'](#nested_monitored_resource)) and path to construct the full URL. Optional (defaults to 80 without SSL, or 443 with SSL)."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `request_method` after provisioning.\nThe HTTP request method to use for the check. If set to 'METHOD_UNSPECIFIED' then 'request_method' defaults to 'GET'. Default value: \"GET\" Possible values: [\"METHOD_UNSPECIFIED\", \"GET\", \"POST\"]"]
    pub fn request_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_method", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `use_ssl` after provisioning.\nIf true, use HTTPS instead of HTTP to run the check."]
    pub fn use_ssl(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_ssl", self.base))
    }
    #[doc = "Get a reference to the value of field `validate_ssl` after provisioning.\nBoolean specifying whether to include SSL certificate validation as a part of the Uptime check. Only applies to checks where 'monitored_resource' is set to 'uptime_url'. If 'use_ssl' is 'false', setting 'validate_ssl' to 'true' has no effect."]
    pub fn validate_ssl(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.validate_ssl", self.base))
    }
    #[doc = "Get a reference to the value of field `accepted_response_status_codes` after provisioning.\n"]
    pub fn accepted_response_status_codes(
        &self,
    ) -> ListRef<MonitoringUptimeCheckConfigHttpCheckElAcceptedResponseStatusCodesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.accepted_response_status_codes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `auth_info` after provisioning.\n"]
    pub fn auth_info(&self) -> ListRef<MonitoringUptimeCheckConfigHttpCheckElAuthInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auth_info", self.base))
    }
    #[doc = "Get a reference to the value of field `ping_config` after provisioning.\n"]
    pub fn ping_config(&self) -> ListRef<MonitoringUptimeCheckConfigHttpCheckElPingConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ping_config", self.base))
    }
    #[doc = "Get a reference to the value of field `service_agent_authentication` after provisioning.\n"]
    pub fn service_agent_authentication(
        &self,
    ) -> ListRef<MonitoringUptimeCheckConfigHttpCheckElServiceAgentAuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_agent_authentication", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfigMonitoredResourceEl {
    labels: RecField<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl MonitoringUptimeCheckConfigMonitoredResourceEl {}
impl ToListMappable for MonitoringUptimeCheckConfigMonitoredResourceEl {
    type O = BlockAssignable<MonitoringUptimeCheckConfigMonitoredResourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringUptimeCheckConfigMonitoredResourceEl {
    #[doc = "Values for all of the labels listed in the associated monitored resource descriptor. For example, Compute Engine VM instances use the labels 'project_id', 'instance_id', and 'zone'."]
    pub labels: RecField<PrimField<String>>,
    #[doc = "The monitored resource type. This field must match the type field of a ['MonitoredResourceDescriptor'](https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.monitoredResourceDescriptors#MonitoredResourceDescriptor) object. For example, the type of a Compute Engine VM instance is 'gce_instance'. For a list of types, see [Monitoring resource types](https://cloud.google.com/monitoring/api/resources) and [Logging resource types](https://cloud.google.com/logging/docs/api/v2/resource-list)."]
    pub type_: PrimField<String>,
}
impl BuildMonitoringUptimeCheckConfigMonitoredResourceEl {
    pub fn build(self) -> MonitoringUptimeCheckConfigMonitoredResourceEl {
        MonitoringUptimeCheckConfigMonitoredResourceEl {
            labels: self.labels,
            type_: self.type_,
        }
    }
}
pub struct MonitoringUptimeCheckConfigMonitoredResourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringUptimeCheckConfigMonitoredResourceElRef {
    fn new(shared: StackShared, base: String) -> MonitoringUptimeCheckConfigMonitoredResourceElRef {
        MonitoringUptimeCheckConfigMonitoredResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringUptimeCheckConfigMonitoredResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nValues for all of the labels listed in the associated monitored resource descriptor. For example, Compute Engine VM instances use the labels 'project_id', 'instance_id', and 'zone'."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe monitored resource type. This field must match the type field of a ['MonitoredResourceDescriptor'](https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.monitoredResourceDescriptors#MonitoredResourceDescriptor) object. For example, the type of a Compute Engine VM instance is 'gce_instance'. For a list of types, see [Monitoring resource types](https://cloud.google.com/monitoring/api/resources) and [Logging resource types](https://cloud.google.com/logging/docs/api/v2/resource-list)."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfigResourceGroupEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type: Option<PrimField<String>>,
}
impl MonitoringUptimeCheckConfigResourceGroupEl {
    #[doc = "Set the field `group_id`.\nThe group of resources being monitored. Should be the 'name' of a group"]
    pub fn set_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_id = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_type`.\nThe resource type of the group members. Possible values: [\"RESOURCE_TYPE_UNSPECIFIED\", \"INSTANCE\", \"AWS_ELB_LOAD_BALANCER\"]"]
    pub fn set_resource_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_type = Some(v.into());
        self
    }
}
impl ToListMappable for MonitoringUptimeCheckConfigResourceGroupEl {
    type O = BlockAssignable<MonitoringUptimeCheckConfigResourceGroupEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringUptimeCheckConfigResourceGroupEl {}
impl BuildMonitoringUptimeCheckConfigResourceGroupEl {
    pub fn build(self) -> MonitoringUptimeCheckConfigResourceGroupEl {
        MonitoringUptimeCheckConfigResourceGroupEl {
            group_id: core::default::Default::default(),
            resource_type: core::default::Default::default(),
        }
    }
}
pub struct MonitoringUptimeCheckConfigResourceGroupElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringUptimeCheckConfigResourceGroupElRef {
    fn new(shared: StackShared, base: String) -> MonitoringUptimeCheckConfigResourceGroupElRef {
        MonitoringUptimeCheckConfigResourceGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringUptimeCheckConfigResourceGroupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `group_id` after provisioning.\nThe group of resources being monitored. Should be the 'name' of a group"]
    pub fn group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\nThe resource type of the group members. Possible values: [\"RESOURCE_TYPE_UNSPECIFIED\", \"INSTANCE\", \"AWS_ELB_LOAD_BALANCER\"]"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfigSyntheticMonitorElCloudFunctionV2El {
    name: PrimField<String>,
}
impl MonitoringUptimeCheckConfigSyntheticMonitorElCloudFunctionV2El {}
impl ToListMappable for MonitoringUptimeCheckConfigSyntheticMonitorElCloudFunctionV2El {
    type O = BlockAssignable<MonitoringUptimeCheckConfigSyntheticMonitorElCloudFunctionV2El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringUptimeCheckConfigSyntheticMonitorElCloudFunctionV2El {
    #[doc = "The fully qualified name of the cloud function resource."]
    pub name: PrimField<String>,
}
impl BuildMonitoringUptimeCheckConfigSyntheticMonitorElCloudFunctionV2El {
    pub fn build(self) -> MonitoringUptimeCheckConfigSyntheticMonitorElCloudFunctionV2El {
        MonitoringUptimeCheckConfigSyntheticMonitorElCloudFunctionV2El { name: self.name }
    }
}
pub struct MonitoringUptimeCheckConfigSyntheticMonitorElCloudFunctionV2ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringUptimeCheckConfigSyntheticMonitorElCloudFunctionV2ElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringUptimeCheckConfigSyntheticMonitorElCloudFunctionV2ElRef {
        MonitoringUptimeCheckConfigSyntheticMonitorElCloudFunctionV2ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringUptimeCheckConfigSyntheticMonitorElCloudFunctionV2ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe fully qualified name of the cloud function resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize, Default)]
struct MonitoringUptimeCheckConfigSyntheticMonitorElDynamic {
    cloud_function_v2:
        Option<DynamicBlock<MonitoringUptimeCheckConfigSyntheticMonitorElCloudFunctionV2El>>,
}
#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfigSyntheticMonitorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_function_v2: Option<Vec<MonitoringUptimeCheckConfigSyntheticMonitorElCloudFunctionV2El>>,
    dynamic: MonitoringUptimeCheckConfigSyntheticMonitorElDynamic,
}
impl MonitoringUptimeCheckConfigSyntheticMonitorEl {
    #[doc = "Set the field `cloud_function_v2`.\n"]
    pub fn set_cloud_function_v2(
        mut self,
        v: impl Into<BlockAssignable<MonitoringUptimeCheckConfigSyntheticMonitorElCloudFunctionV2El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloud_function_v2 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_function_v2 = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MonitoringUptimeCheckConfigSyntheticMonitorEl {
    type O = BlockAssignable<MonitoringUptimeCheckConfigSyntheticMonitorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringUptimeCheckConfigSyntheticMonitorEl {}
impl BuildMonitoringUptimeCheckConfigSyntheticMonitorEl {
    pub fn build(self) -> MonitoringUptimeCheckConfigSyntheticMonitorEl {
        MonitoringUptimeCheckConfigSyntheticMonitorEl {
            cloud_function_v2: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MonitoringUptimeCheckConfigSyntheticMonitorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringUptimeCheckConfigSyntheticMonitorElRef {
    fn new(shared: StackShared, base: String) -> MonitoringUptimeCheckConfigSyntheticMonitorElRef {
        MonitoringUptimeCheckConfigSyntheticMonitorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringUptimeCheckConfigSyntheticMonitorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_function_v2` after provisioning.\n"]
    pub fn cloud_function_v2(
        &self,
    ) -> ListRef<MonitoringUptimeCheckConfigSyntheticMonitorElCloudFunctionV2ElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_function_v2", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfigTcpCheckElPingConfigEl {
    pings_count: PrimField<f64>,
}
impl MonitoringUptimeCheckConfigTcpCheckElPingConfigEl {}
impl ToListMappable for MonitoringUptimeCheckConfigTcpCheckElPingConfigEl {
    type O = BlockAssignable<MonitoringUptimeCheckConfigTcpCheckElPingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringUptimeCheckConfigTcpCheckElPingConfigEl {
    #[doc = "Number of ICMP pings. A maximum of 3 ICMP pings is currently supported."]
    pub pings_count: PrimField<f64>,
}
impl BuildMonitoringUptimeCheckConfigTcpCheckElPingConfigEl {
    pub fn build(self) -> MonitoringUptimeCheckConfigTcpCheckElPingConfigEl {
        MonitoringUptimeCheckConfigTcpCheckElPingConfigEl {
            pings_count: self.pings_count,
        }
    }
}
pub struct MonitoringUptimeCheckConfigTcpCheckElPingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringUptimeCheckConfigTcpCheckElPingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MonitoringUptimeCheckConfigTcpCheckElPingConfigElRef {
        MonitoringUptimeCheckConfigTcpCheckElPingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringUptimeCheckConfigTcpCheckElPingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pings_count` after provisioning.\nNumber of ICMP pings. A maximum of 3 ICMP pings is currently supported."]
    pub fn pings_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.pings_count", self.base))
    }
}
#[derive(Serialize, Default)]
struct MonitoringUptimeCheckConfigTcpCheckElDynamic {
    ping_config: Option<DynamicBlock<MonitoringUptimeCheckConfigTcpCheckElPingConfigEl>>,
}
#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfigTcpCheckEl {
    port: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ping_config: Option<Vec<MonitoringUptimeCheckConfigTcpCheckElPingConfigEl>>,
    dynamic: MonitoringUptimeCheckConfigTcpCheckElDynamic,
}
impl MonitoringUptimeCheckConfigTcpCheckEl {
    #[doc = "Set the field `ping_config`.\n"]
    pub fn set_ping_config(
        mut self,
        v: impl Into<BlockAssignable<MonitoringUptimeCheckConfigTcpCheckElPingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ping_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ping_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MonitoringUptimeCheckConfigTcpCheckEl {
    type O = BlockAssignable<MonitoringUptimeCheckConfigTcpCheckEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringUptimeCheckConfigTcpCheckEl {
    #[doc = "The port to the page to run the check against. Will be combined with host (specified within the 'monitored_resource') to construct the full URL."]
    pub port: PrimField<f64>,
}
impl BuildMonitoringUptimeCheckConfigTcpCheckEl {
    pub fn build(self) -> MonitoringUptimeCheckConfigTcpCheckEl {
        MonitoringUptimeCheckConfigTcpCheckEl {
            port: self.port,
            ping_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MonitoringUptimeCheckConfigTcpCheckElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringUptimeCheckConfigTcpCheckElRef {
    fn new(shared: StackShared, base: String) -> MonitoringUptimeCheckConfigTcpCheckElRef {
        MonitoringUptimeCheckConfigTcpCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringUptimeCheckConfigTcpCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe port to the page to run the check against. Will be combined with host (specified within the 'monitored_resource') to construct the full URL."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `ping_config` after provisioning.\n"]
    pub fn ping_config(&self) -> ListRef<MonitoringUptimeCheckConfigTcpCheckElPingConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ping_config", self.base))
    }
}
#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl MonitoringUptimeCheckConfigTimeoutsEl {
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
impl ToListMappable for MonitoringUptimeCheckConfigTimeoutsEl {
    type O = BlockAssignable<MonitoringUptimeCheckConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringUptimeCheckConfigTimeoutsEl {}
impl BuildMonitoringUptimeCheckConfigTimeoutsEl {
    pub fn build(self) -> MonitoringUptimeCheckConfigTimeoutsEl {
        MonitoringUptimeCheckConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct MonitoringUptimeCheckConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringUptimeCheckConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> MonitoringUptimeCheckConfigTimeoutsElRef {
        MonitoringUptimeCheckConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringUptimeCheckConfigTimeoutsElRef {
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
struct MonitoringUptimeCheckConfigDynamic {
    content_matchers: Option<DynamicBlock<MonitoringUptimeCheckConfigContentMatchersEl>>,
    http_check: Option<DynamicBlock<MonitoringUptimeCheckConfigHttpCheckEl>>,
    monitored_resource: Option<DynamicBlock<MonitoringUptimeCheckConfigMonitoredResourceEl>>,
    resource_group: Option<DynamicBlock<MonitoringUptimeCheckConfigResourceGroupEl>>,
    synthetic_monitor: Option<DynamicBlock<MonitoringUptimeCheckConfigSyntheticMonitorEl>>,
    tcp_check: Option<DynamicBlock<MonitoringUptimeCheckConfigTcpCheckEl>>,
}
