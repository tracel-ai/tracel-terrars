use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeHealthCheckData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    check_interval_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    healthy_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_regions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unhealthy_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc_health_check: Option<Vec<ComputeHealthCheckGrpcHealthCheckEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc_tls_health_check: Option<Vec<ComputeHealthCheckGrpcTlsHealthCheckEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http2_health_check: Option<Vec<ComputeHealthCheckHttp2HealthCheckEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_health_check: Option<Vec<ComputeHealthCheckHttpHealthCheckEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    https_health_check: Option<Vec<ComputeHealthCheckHttpsHealthCheckEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_config: Option<Vec<ComputeHealthCheckLogConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_health_check: Option<Vec<ComputeHealthCheckSslHealthCheckEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_health_check: Option<Vec<ComputeHealthCheckTcpHealthCheckEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeHealthCheckTimeoutsEl>,
    dynamic: ComputeHealthCheckDynamic,
}
struct ComputeHealthCheck_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeHealthCheckData>,
}
#[derive(Clone)]
pub struct ComputeHealthCheck(Rc<ComputeHealthCheck_>);
impl ComputeHealthCheck {
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
    #[doc = "Set the field `check_interval_sec`.\nHow often (in seconds) to send a health check. The default value is 5\nseconds."]
    pub fn set_check_interval_sec(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().check_interval_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `healthy_threshold`.\nA so-far unhealthy instance will be marked healthy after this many\nconsecutive successes. The default value is 2."]
    pub fn set_healthy_threshold(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().healthy_threshold = Some(v.into());
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
    #[doc = "Set the field `source_regions`.\nThe list of cloud regions from which health checks are performed. If\nany regions are specified, then exactly 3 regions should be specified.\nThe region names must be valid names of Google Cloud regions. This can\nonly be set for global health check. If this list is non-empty, then\nthere are restrictions on what other health check fields are supported\nand what other resources can use this health check:\n\n* SSL, HTTP2, and GRPC protocols are not supported.\n\n* The TCP request field is not supported.\n\n* The proxyHeader field for HTTP, HTTPS, and TCP is not supported.\n\n* The checkIntervalSec field must be at least 30.\n\n* The health check cannot be used with BackendService nor with managed\ninstance group auto-healing."]
    pub fn set_source_regions(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().source_regions = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout_sec`.\nHow long (in seconds) to wait before claiming failure.\nThe default value is 5 seconds.  It is invalid for timeoutSec to have\ngreater value than checkIntervalSec."]
    pub fn set_timeout_sec(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().timeout_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `unhealthy_threshold`.\nA so-far healthy instance will be marked unhealthy after this many\nconsecutive failures. The default value is 2."]
    pub fn set_unhealthy_threshold(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().unhealthy_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `grpc_health_check`.\n"]
    pub fn set_grpc_health_check(
        self,
        v: impl Into<BlockAssignable<ComputeHealthCheckGrpcHealthCheckEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().grpc_health_check = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.grpc_health_check = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `grpc_tls_health_check`.\n"]
    pub fn set_grpc_tls_health_check(
        self,
        v: impl Into<BlockAssignable<ComputeHealthCheckGrpcTlsHealthCheckEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().grpc_tls_health_check = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.grpc_tls_health_check = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `http2_health_check`.\n"]
    pub fn set_http2_health_check(
        self,
        v: impl Into<BlockAssignable<ComputeHealthCheckHttp2HealthCheckEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().http2_health_check = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.http2_health_check = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `http_health_check`.\n"]
    pub fn set_http_health_check(
        self,
        v: impl Into<BlockAssignable<ComputeHealthCheckHttpHealthCheckEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().http_health_check = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.http_health_check = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `https_health_check`.\n"]
    pub fn set_https_health_check(
        self,
        v: impl Into<BlockAssignable<ComputeHealthCheckHttpsHealthCheckEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().https_health_check = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.https_health_check = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `log_config`.\n"]
    pub fn set_log_config(
        self,
        v: impl Into<BlockAssignable<ComputeHealthCheckLogConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ssl_health_check`.\n"]
    pub fn set_ssl_health_check(
        self,
        v: impl Into<BlockAssignable<ComputeHealthCheckSslHealthCheckEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ssl_health_check = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ssl_health_check = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tcp_health_check`.\n"]
    pub fn set_tcp_health_check(
        self,
        v: impl Into<BlockAssignable<ComputeHealthCheckTcpHealthCheckEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tcp_health_check = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tcp_health_check = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeHealthCheckTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `check_interval_sec` after provisioning.\nHow often (in seconds) to send a health check. The default value is 5\nseconds."]
    pub fn check_interval_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.check_interval_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `healthy_threshold` after provisioning.\nA so-far unhealthy instance will be marked healthy after this many\nconsecutive successes. The default value is 2."]
    pub fn healthy_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.healthy_threshold", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
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
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_regions` after provisioning.\nThe list of cloud regions from which health checks are performed. If\nany regions are specified, then exactly 3 regions should be specified.\nThe region names must be valid names of Google Cloud regions. This can\nonly be set for global health check. If this list is non-empty, then\nthere are restrictions on what other health check fields are supported\nand what other resources can use this health check:\n\n* SSL, HTTP2, and GRPC protocols are not supported.\n\n* The TCP request field is not supported.\n\n* The proxyHeader field for HTTP, HTTPS, and TCP is not supported.\n\n* The checkIntervalSec field must be at least 30.\n\n* The health check cannot be used with BackendService nor with managed\ninstance group auto-healing."]
    pub fn source_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_regions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeout_sec` after provisioning.\nHow long (in seconds) to wait before claiming failure.\nThe default value is 5 seconds.  It is invalid for timeoutSec to have\ngreater value than checkIntervalSec."]
    pub fn timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the health check. One of HTTP, HTTPS, TCP, or SSL."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `unhealthy_threshold` after provisioning.\nA so-far healthy instance will be marked unhealthy after this many\nconsecutive failures. The default value is 2."]
    pub fn unhealthy_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unhealthy_threshold", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `grpc_health_check` after provisioning.\n"]
    pub fn grpc_health_check(&self) -> ListRef<ComputeHealthCheckGrpcHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.grpc_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `grpc_tls_health_check` after provisioning.\n"]
    pub fn grpc_tls_health_check(&self) -> ListRef<ComputeHealthCheckGrpcTlsHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.grpc_tls_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `http2_health_check` after provisioning.\n"]
    pub fn http2_health_check(&self) -> ListRef<ComputeHealthCheckHttp2HealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http2_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `http_health_check` after provisioning.\n"]
    pub fn http_health_check(&self) -> ListRef<ComputeHealthCheckHttpHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `https_health_check` after provisioning.\n"]
    pub fn https_health_check(&self) -> ListRef<ComputeHealthCheckHttpsHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.https_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> ListRef<ComputeHealthCheckLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_health_check` after provisioning.\n"]
    pub fn ssl_health_check(&self) -> ListRef<ComputeHealthCheckSslHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ssl_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tcp_health_check` after provisioning.\n"]
    pub fn tcp_health_check(&self) -> ListRef<ComputeHealthCheckTcpHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tcp_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeHealthCheckTimeoutsElRef {
        ComputeHealthCheckTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeHealthCheck {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeHealthCheck {}
impl ToListMappable for ComputeHealthCheck {
    type O = ListRef<ComputeHealthCheckRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeHealthCheck_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_health_check".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeHealthCheck {
    pub tf_id: String,
    #[doc = "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
    pub name: PrimField<String>,
}
impl BuildComputeHealthCheck {
    pub fn build(self, stack: &mut Stack) -> ComputeHealthCheck {
        let out = ComputeHealthCheck(Rc::new(ComputeHealthCheck_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeHealthCheckData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                check_interval_sec: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                healthy_threshold: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                source_regions: core::default::Default::default(),
                timeout_sec: core::default::Default::default(),
                unhealthy_threshold: core::default::Default::default(),
                grpc_health_check: core::default::Default::default(),
                grpc_tls_health_check: core::default::Default::default(),
                http2_health_check: core::default::Default::default(),
                http_health_check: core::default::Default::default(),
                https_health_check: core::default::Default::default(),
                log_config: core::default::Default::default(),
                ssl_health_check: core::default::Default::default(),
                tcp_health_check: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeHealthCheckRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeHealthCheckRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeHealthCheckRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `check_interval_sec` after provisioning.\nHow often (in seconds) to send a health check. The default value is 5\nseconds."]
    pub fn check_interval_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.check_interval_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `healthy_threshold` after provisioning.\nA so-far unhealthy instance will be marked healthy after this many\nconsecutive successes. The default value is 2."]
    pub fn healthy_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.healthy_threshold", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
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
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_regions` after provisioning.\nThe list of cloud regions from which health checks are performed. If\nany regions are specified, then exactly 3 regions should be specified.\nThe region names must be valid names of Google Cloud regions. This can\nonly be set for global health check. If this list is non-empty, then\nthere are restrictions on what other health check fields are supported\nand what other resources can use this health check:\n\n* SSL, HTTP2, and GRPC protocols are not supported.\n\n* The TCP request field is not supported.\n\n* The proxyHeader field for HTTP, HTTPS, and TCP is not supported.\n\n* The checkIntervalSec field must be at least 30.\n\n* The health check cannot be used with BackendService nor with managed\ninstance group auto-healing."]
    pub fn source_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_regions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeout_sec` after provisioning.\nHow long (in seconds) to wait before claiming failure.\nThe default value is 5 seconds.  It is invalid for timeoutSec to have\ngreater value than checkIntervalSec."]
    pub fn timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_sec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the health check. One of HTTP, HTTPS, TCP, or SSL."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `unhealthy_threshold` after provisioning.\nA so-far healthy instance will be marked unhealthy after this many\nconsecutive failures. The default value is 2."]
    pub fn unhealthy_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unhealthy_threshold", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `grpc_health_check` after provisioning.\n"]
    pub fn grpc_health_check(&self) -> ListRef<ComputeHealthCheckGrpcHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.grpc_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `grpc_tls_health_check` after provisioning.\n"]
    pub fn grpc_tls_health_check(&self) -> ListRef<ComputeHealthCheckGrpcTlsHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.grpc_tls_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `http2_health_check` after provisioning.\n"]
    pub fn http2_health_check(&self) -> ListRef<ComputeHealthCheckHttp2HealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http2_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `http_health_check` after provisioning.\n"]
    pub fn http_health_check(&self) -> ListRef<ComputeHealthCheckHttpHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `https_health_check` after provisioning.\n"]
    pub fn https_health_check(&self) -> ListRef<ComputeHealthCheckHttpsHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.https_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> ListRef<ComputeHealthCheckLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_health_check` after provisioning.\n"]
    pub fn ssl_health_check(&self) -> ListRef<ComputeHealthCheckSslHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ssl_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tcp_health_check` after provisioning.\n"]
    pub fn tcp_health_check(&self) -> ListRef<ComputeHealthCheckTcpHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tcp_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeHealthCheckTimeoutsElRef {
        ComputeHealthCheckTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeHealthCheckGrpcHealthCheckEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc_service_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_specification: Option<PrimField<String>>,
}
impl ComputeHealthCheckGrpcHealthCheckEl {
    #[doc = "Set the field `grpc_service_name`.\nThe gRPC service name for the health check.\nThe value of grpcServiceName has the following meanings by convention:\n  - Empty serviceName means the overall status of all services at the backend.\n  - Non-empty serviceName means the health of that gRPC service, as defined by the owner of the service.\nThe grpcServiceName can only be ASCII."]
    pub fn set_grpc_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.grpc_service_name = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nThe port number for the health check request.\nMust be specified if portName and portSpecification are not set\nor if port_specification is USE_FIXED_PORT. Valid values are 1 through 65535."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `port_name`.\nPort name as defined in InstanceGroup#NamedPort#name. If both port and\nport_name are defined, port takes precedence."]
    pub fn set_port_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_name = Some(v.into());
        self
    }
    #[doc = "Set the field `port_specification`.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': The 'portName' is used for health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, gRPC health check follows behavior specified in 'port' and\n'portName' fields. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn set_port_specification(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_specification = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeHealthCheckGrpcHealthCheckEl {
    type O = BlockAssignable<ComputeHealthCheckGrpcHealthCheckEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeHealthCheckGrpcHealthCheckEl {}
impl BuildComputeHealthCheckGrpcHealthCheckEl {
    pub fn build(self) -> ComputeHealthCheckGrpcHealthCheckEl {
        ComputeHealthCheckGrpcHealthCheckEl {
            grpc_service_name: core::default::Default::default(),
            port: core::default::Default::default(),
            port_name: core::default::Default::default(),
            port_specification: core::default::Default::default(),
        }
    }
}
pub struct ComputeHealthCheckGrpcHealthCheckElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeHealthCheckGrpcHealthCheckElRef {
    fn new(shared: StackShared, base: String) -> ComputeHealthCheckGrpcHealthCheckElRef {
        ComputeHealthCheckGrpcHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeHealthCheckGrpcHealthCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `grpc_service_name` after provisioning.\nThe gRPC service name for the health check.\nThe value of grpcServiceName has the following meanings by convention:\n  - Empty serviceName means the overall status of all services at the backend.\n  - Non-empty serviceName means the health of that gRPC service, as defined by the owner of the service.\nThe grpcServiceName can only be ASCII."]
    pub fn grpc_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.grpc_service_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe port number for the health check request.\nMust be specified if portName and portSpecification are not set\nor if port_specification is USE_FIXED_PORT. Valid values are 1 through 65535."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `port_name` after provisioning.\nPort name as defined in InstanceGroup#NamedPort#name. If both port and\nport_name are defined, port takes precedence."]
    pub fn port_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }
    #[doc = "Get a reference to the value of field `port_specification` after provisioning.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': The 'portName' is used for health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, gRPC health check follows behavior specified in 'port' and\n'portName' fields. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn port_specification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port_specification", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeHealthCheckGrpcTlsHealthCheckEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc_service_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_specification: Option<PrimField<String>>,
}
impl ComputeHealthCheckGrpcTlsHealthCheckEl {
    #[doc = "Set the field `grpc_service_name`.\nThe gRPC service name for the health check.\nThe value of grpcServiceName has the following meanings by convention:\n  - Empty serviceName means the overall status of all services at the backend.\n  - Non-empty serviceName means the health of that gRPC service, as defined by the owner of the service.\nThe grpcServiceName can only be ASCII."]
    pub fn set_grpc_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.grpc_service_name = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nThe port number for the health check request.\nMust be specified if port_specification is USE_FIXED_PORT. Valid values are 1 through 65535."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `port_specification`.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': Not supported for GRPC with TLS health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, gRPC with TLS health check follows behavior specified in the 'port' field. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn set_port_specification(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_specification = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeHealthCheckGrpcTlsHealthCheckEl {
    type O = BlockAssignable<ComputeHealthCheckGrpcTlsHealthCheckEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeHealthCheckGrpcTlsHealthCheckEl {}
impl BuildComputeHealthCheckGrpcTlsHealthCheckEl {
    pub fn build(self) -> ComputeHealthCheckGrpcTlsHealthCheckEl {
        ComputeHealthCheckGrpcTlsHealthCheckEl {
            grpc_service_name: core::default::Default::default(),
            port: core::default::Default::default(),
            port_specification: core::default::Default::default(),
        }
    }
}
pub struct ComputeHealthCheckGrpcTlsHealthCheckElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeHealthCheckGrpcTlsHealthCheckElRef {
    fn new(shared: StackShared, base: String) -> ComputeHealthCheckGrpcTlsHealthCheckElRef {
        ComputeHealthCheckGrpcTlsHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeHealthCheckGrpcTlsHealthCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `grpc_service_name` after provisioning.\nThe gRPC service name for the health check.\nThe value of grpcServiceName has the following meanings by convention:\n  - Empty serviceName means the overall status of all services at the backend.\n  - Non-empty serviceName means the health of that gRPC service, as defined by the owner of the service.\nThe grpcServiceName can only be ASCII."]
    pub fn grpc_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.grpc_service_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe port number for the health check request.\nMust be specified if port_specification is USE_FIXED_PORT. Valid values are 1 through 65535."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `port_specification` after provisioning.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': Not supported for GRPC with TLS health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, gRPC with TLS health check follows behavior specified in the 'port' field. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn port_specification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port_specification", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeHealthCheckHttp2HealthCheckEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_specification: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_header: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<PrimField<String>>,
}
impl ComputeHealthCheckHttp2HealthCheckEl {
    #[doc = "Set the field `host`.\nThe value of the host header in the HTTP2 health check request.\nIf left empty (default value), the public IP on behalf of which this health\ncheck is performed will be used."]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nThe TCP port number for the HTTP2 health check request.\nThe default value is 443."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `port_name`.\nPort name as defined in InstanceGroup#NamedPort#name. If both port and\nport_name are defined, port takes precedence."]
    pub fn set_port_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_name = Some(v.into());
        self
    }
    #[doc = "Set the field `port_specification`.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': The 'portName' is used for health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, HTTP2 health check follows behavior specified in 'port' and\n'portName' fields. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn set_port_specification(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_specification = Some(v.into());
        self
    }
    #[doc = "Set the field `proxy_header`.\nSpecifies the type of proxy header to append before sending data to the\nbackend. Default value: \"NONE\" Possible values: [\"NONE\", \"PROXY_V1\"]"]
    pub fn set_proxy_header(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.proxy_header = Some(v.into());
        self
    }
    #[doc = "Set the field `request_path`.\nThe request path of the HTTP2 health check request.\nThe default value is /."]
    pub fn set_request_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.request_path = Some(v.into());
        self
    }
    #[doc = "Set the field `response`.\nThe bytes to match against the beginning of the response data. If left empty\n(the default value), any response will indicate health. The response data\ncan only be ASCII."]
    pub fn set_response(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeHealthCheckHttp2HealthCheckEl {
    type O = BlockAssignable<ComputeHealthCheckHttp2HealthCheckEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeHealthCheckHttp2HealthCheckEl {}
impl BuildComputeHealthCheckHttp2HealthCheckEl {
    pub fn build(self) -> ComputeHealthCheckHttp2HealthCheckEl {
        ComputeHealthCheckHttp2HealthCheckEl {
            host: core::default::Default::default(),
            port: core::default::Default::default(),
            port_name: core::default::Default::default(),
            port_specification: core::default::Default::default(),
            proxy_header: core::default::Default::default(),
            request_path: core::default::Default::default(),
            response: core::default::Default::default(),
        }
    }
}
pub struct ComputeHealthCheckHttp2HealthCheckElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeHealthCheckHttp2HealthCheckElRef {
    fn new(shared: StackShared, base: String) -> ComputeHealthCheckHttp2HealthCheckElRef {
        ComputeHealthCheckHttp2HealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeHealthCheckHttp2HealthCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\nThe value of the host header in the HTTP2 health check request.\nIf left empty (default value), the public IP on behalf of which this health\ncheck is performed will be used."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe TCP port number for the HTTP2 health check request.\nThe default value is 443."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `port_name` after provisioning.\nPort name as defined in InstanceGroup#NamedPort#name. If both port and\nport_name are defined, port takes precedence."]
    pub fn port_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }
    #[doc = "Get a reference to the value of field `port_specification` after provisioning.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': The 'portName' is used for health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, HTTP2 health check follows behavior specified in 'port' and\n'portName' fields. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn port_specification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port_specification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `proxy_header` after provisioning.\nSpecifies the type of proxy header to append before sending data to the\nbackend. Default value: \"NONE\" Possible values: [\"NONE\", \"PROXY_V1\"]"]
    pub fn proxy_header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxy_header", self.base))
    }
    #[doc = "Get a reference to the value of field `request_path` after provisioning.\nThe request path of the HTTP2 health check request.\nThe default value is /."]
    pub fn request_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_path", self.base))
    }
    #[doc = "Get a reference to the value of field `response` after provisioning.\nThe bytes to match against the beginning of the response data. If left empty\n(the default value), any response will indicate health. The response data\ncan only be ASCII."]
    pub fn response(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.response", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeHealthCheckHttpHealthCheckEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_specification: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_header: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<PrimField<String>>,
}
impl ComputeHealthCheckHttpHealthCheckEl {
    #[doc = "Set the field `host`.\nThe value of the host header in the HTTP health check request.\nIf left empty (default value), the public IP on behalf of which this health\ncheck is performed will be used."]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nThe TCP port number for the HTTP health check request.\nThe default value is 80."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `port_name`.\nPort name as defined in InstanceGroup#NamedPort#name. If both port and\nport_name are defined, port takes precedence."]
    pub fn set_port_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_name = Some(v.into());
        self
    }
    #[doc = "Set the field `port_specification`.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': The 'portName' is used for health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, HTTP health check follows behavior specified in 'port' and\n'portName' fields. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn set_port_specification(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_specification = Some(v.into());
        self
    }
    #[doc = "Set the field `proxy_header`.\nSpecifies the type of proxy header to append before sending data to the\nbackend. Default value: \"NONE\" Possible values: [\"NONE\", \"PROXY_V1\"]"]
    pub fn set_proxy_header(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.proxy_header = Some(v.into());
        self
    }
    #[doc = "Set the field `request_path`.\nThe request path of the HTTP health check request.\nThe default value is /."]
    pub fn set_request_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.request_path = Some(v.into());
        self
    }
    #[doc = "Set the field `response`.\nThe bytes to match against the beginning of the response data. If left empty\n(the default value), any response will indicate health. The response data\ncan only be ASCII."]
    pub fn set_response(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeHealthCheckHttpHealthCheckEl {
    type O = BlockAssignable<ComputeHealthCheckHttpHealthCheckEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeHealthCheckHttpHealthCheckEl {}
impl BuildComputeHealthCheckHttpHealthCheckEl {
    pub fn build(self) -> ComputeHealthCheckHttpHealthCheckEl {
        ComputeHealthCheckHttpHealthCheckEl {
            host: core::default::Default::default(),
            port: core::default::Default::default(),
            port_name: core::default::Default::default(),
            port_specification: core::default::Default::default(),
            proxy_header: core::default::Default::default(),
            request_path: core::default::Default::default(),
            response: core::default::Default::default(),
        }
    }
}
pub struct ComputeHealthCheckHttpHealthCheckElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeHealthCheckHttpHealthCheckElRef {
    fn new(shared: StackShared, base: String) -> ComputeHealthCheckHttpHealthCheckElRef {
        ComputeHealthCheckHttpHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeHealthCheckHttpHealthCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\nThe value of the host header in the HTTP health check request.\nIf left empty (default value), the public IP on behalf of which this health\ncheck is performed will be used."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe TCP port number for the HTTP health check request.\nThe default value is 80."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `port_name` after provisioning.\nPort name as defined in InstanceGroup#NamedPort#name. If both port and\nport_name are defined, port takes precedence."]
    pub fn port_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }
    #[doc = "Get a reference to the value of field `port_specification` after provisioning.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': The 'portName' is used for health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, HTTP health check follows behavior specified in 'port' and\n'portName' fields. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn port_specification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port_specification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `proxy_header` after provisioning.\nSpecifies the type of proxy header to append before sending data to the\nbackend. Default value: \"NONE\" Possible values: [\"NONE\", \"PROXY_V1\"]"]
    pub fn proxy_header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxy_header", self.base))
    }
    #[doc = "Get a reference to the value of field `request_path` after provisioning.\nThe request path of the HTTP health check request.\nThe default value is /."]
    pub fn request_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_path", self.base))
    }
    #[doc = "Get a reference to the value of field `response` after provisioning.\nThe bytes to match against the beginning of the response data. If left empty\n(the default value), any response will indicate health. The response data\ncan only be ASCII."]
    pub fn response(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.response", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeHealthCheckHttpsHealthCheckEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_specification: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_header: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<PrimField<String>>,
}
impl ComputeHealthCheckHttpsHealthCheckEl {
    #[doc = "Set the field `host`.\nThe value of the host header in the HTTPS health check request.\nIf left empty (default value), the public IP on behalf of which this health\ncheck is performed will be used."]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nThe TCP port number for the HTTPS health check request.\nThe default value is 443."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `port_name`.\nPort name as defined in InstanceGroup#NamedPort#name. If both port and\nport_name are defined, port takes precedence."]
    pub fn set_port_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_name = Some(v.into());
        self
    }
    #[doc = "Set the field `port_specification`.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': The 'portName' is used for health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, HTTPS health check follows behavior specified in 'port' and\n'portName' fields. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn set_port_specification(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_specification = Some(v.into());
        self
    }
    #[doc = "Set the field `proxy_header`.\nSpecifies the type of proxy header to append before sending data to the\nbackend. Default value: \"NONE\" Possible values: [\"NONE\", \"PROXY_V1\"]"]
    pub fn set_proxy_header(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.proxy_header = Some(v.into());
        self
    }
    #[doc = "Set the field `request_path`.\nThe request path of the HTTPS health check request.\nThe default value is /."]
    pub fn set_request_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.request_path = Some(v.into());
        self
    }
    #[doc = "Set the field `response`.\nThe bytes to match against the beginning of the response data. If left empty\n(the default value), any response will indicate health. The response data\ncan only be ASCII."]
    pub fn set_response(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeHealthCheckHttpsHealthCheckEl {
    type O = BlockAssignable<ComputeHealthCheckHttpsHealthCheckEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeHealthCheckHttpsHealthCheckEl {}
impl BuildComputeHealthCheckHttpsHealthCheckEl {
    pub fn build(self) -> ComputeHealthCheckHttpsHealthCheckEl {
        ComputeHealthCheckHttpsHealthCheckEl {
            host: core::default::Default::default(),
            port: core::default::Default::default(),
            port_name: core::default::Default::default(),
            port_specification: core::default::Default::default(),
            proxy_header: core::default::Default::default(),
            request_path: core::default::Default::default(),
            response: core::default::Default::default(),
        }
    }
}
pub struct ComputeHealthCheckHttpsHealthCheckElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeHealthCheckHttpsHealthCheckElRef {
    fn new(shared: StackShared, base: String) -> ComputeHealthCheckHttpsHealthCheckElRef {
        ComputeHealthCheckHttpsHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeHealthCheckHttpsHealthCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\nThe value of the host header in the HTTPS health check request.\nIf left empty (default value), the public IP on behalf of which this health\ncheck is performed will be used."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe TCP port number for the HTTPS health check request.\nThe default value is 443."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `port_name` after provisioning.\nPort name as defined in InstanceGroup#NamedPort#name. If both port and\nport_name are defined, port takes precedence."]
    pub fn port_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }
    #[doc = "Get a reference to the value of field `port_specification` after provisioning.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': The 'portName' is used for health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, HTTPS health check follows behavior specified in 'port' and\n'portName' fields. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn port_specification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port_specification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `proxy_header` after provisioning.\nSpecifies the type of proxy header to append before sending data to the\nbackend. Default value: \"NONE\" Possible values: [\"NONE\", \"PROXY_V1\"]"]
    pub fn proxy_header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxy_header", self.base))
    }
    #[doc = "Get a reference to the value of field `request_path` after provisioning.\nThe request path of the HTTPS health check request.\nThe default value is /."]
    pub fn request_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_path", self.base))
    }
    #[doc = "Get a reference to the value of field `response` after provisioning.\nThe bytes to match against the beginning of the response data. If left empty\n(the default value), any response will indicate health. The response data\ncan only be ASCII."]
    pub fn response(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.response", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeHealthCheckLogConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable: Option<PrimField<bool>>,
}
impl ComputeHealthCheckLogConfigEl {
    #[doc = "Set the field `enable`.\nIndicates whether or not to export logs. This is false by default,\nwhich means no health check logging will be done."]
    pub fn set_enable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeHealthCheckLogConfigEl {
    type O = BlockAssignable<ComputeHealthCheckLogConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeHealthCheckLogConfigEl {}
impl BuildComputeHealthCheckLogConfigEl {
    pub fn build(self) -> ComputeHealthCheckLogConfigEl {
        ComputeHealthCheckLogConfigEl {
            enable: core::default::Default::default(),
        }
    }
}
pub struct ComputeHealthCheckLogConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeHealthCheckLogConfigElRef {
    fn new(shared: StackShared, base: String) -> ComputeHealthCheckLogConfigElRef {
        ComputeHealthCheckLogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeHealthCheckLogConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable` after provisioning.\nIndicates whether or not to export logs. This is false by default,\nwhich means no health check logging will be done."]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeHealthCheckSslHealthCheckEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_specification: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_header: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<PrimField<String>>,
}
impl ComputeHealthCheckSslHealthCheckEl {
    #[doc = "Set the field `port`.\nThe TCP port number for the SSL health check request.\nThe default value is 443."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `port_name`.\nPort name as defined in InstanceGroup#NamedPort#name. If both port and\nport_name are defined, port takes precedence."]
    pub fn set_port_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_name = Some(v.into());
        self
    }
    #[doc = "Set the field `port_specification`.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': The 'portName' is used for health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, SSL health check follows behavior specified in 'port' and\n'portName' fields. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn set_port_specification(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_specification = Some(v.into());
        self
    }
    #[doc = "Set the field `proxy_header`.\nSpecifies the type of proxy header to append before sending data to the\nbackend. Default value: \"NONE\" Possible values: [\"NONE\", \"PROXY_V1\"]"]
    pub fn set_proxy_header(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.proxy_header = Some(v.into());
        self
    }
    #[doc = "Set the field `request`.\nThe application data to send once the SSL connection has been\nestablished (default value is empty). If both request and response are\nempty, the connection establishment alone will indicate health. The request\ndata can only be ASCII."]
    pub fn set_request(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.request = Some(v.into());
        self
    }
    #[doc = "Set the field `response`.\nThe bytes to match against the beginning of the response data. If left empty\n(the default value), any response will indicate health. The response data\ncan only be ASCII."]
    pub fn set_response(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeHealthCheckSslHealthCheckEl {
    type O = BlockAssignable<ComputeHealthCheckSslHealthCheckEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeHealthCheckSslHealthCheckEl {}
impl BuildComputeHealthCheckSslHealthCheckEl {
    pub fn build(self) -> ComputeHealthCheckSslHealthCheckEl {
        ComputeHealthCheckSslHealthCheckEl {
            port: core::default::Default::default(),
            port_name: core::default::Default::default(),
            port_specification: core::default::Default::default(),
            proxy_header: core::default::Default::default(),
            request: core::default::Default::default(),
            response: core::default::Default::default(),
        }
    }
}
pub struct ComputeHealthCheckSslHealthCheckElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeHealthCheckSslHealthCheckElRef {
    fn new(shared: StackShared, base: String) -> ComputeHealthCheckSslHealthCheckElRef {
        ComputeHealthCheckSslHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeHealthCheckSslHealthCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe TCP port number for the SSL health check request.\nThe default value is 443."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `port_name` after provisioning.\nPort name as defined in InstanceGroup#NamedPort#name. If both port and\nport_name are defined, port takes precedence."]
    pub fn port_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }
    #[doc = "Get a reference to the value of field `port_specification` after provisioning.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': The 'portName' is used for health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, SSL health check follows behavior specified in 'port' and\n'portName' fields. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn port_specification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port_specification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `proxy_header` after provisioning.\nSpecifies the type of proxy header to append before sending data to the\nbackend. Default value: \"NONE\" Possible values: [\"NONE\", \"PROXY_V1\"]"]
    pub fn proxy_header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxy_header", self.base))
    }
    #[doc = "Get a reference to the value of field `request` after provisioning.\nThe application data to send once the SSL connection has been\nestablished (default value is empty). If both request and response are\nempty, the connection establishment alone will indicate health. The request\ndata can only be ASCII."]
    pub fn request(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request", self.base))
    }
    #[doc = "Get a reference to the value of field `response` after provisioning.\nThe bytes to match against the beginning of the response data. If left empty\n(the default value), any response will indicate health. The response data\ncan only be ASCII."]
    pub fn response(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.response", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeHealthCheckTcpHealthCheckEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_specification: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_header: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<PrimField<String>>,
}
impl ComputeHealthCheckTcpHealthCheckEl {
    #[doc = "Set the field `port`.\nThe TCP port number for the TCP health check request.\nThe default value is 443."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `port_name`.\nPort name as defined in InstanceGroup#NamedPort#name. If both port and\nport_name are defined, port takes precedence."]
    pub fn set_port_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_name = Some(v.into());
        self
    }
    #[doc = "Set the field `port_specification`.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': The 'portName' is used for health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, TCP health check follows behavior specified in 'port' and\n'portName' fields. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn set_port_specification(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_specification = Some(v.into());
        self
    }
    #[doc = "Set the field `proxy_header`.\nSpecifies the type of proxy header to append before sending data to the\nbackend. Default value: \"NONE\" Possible values: [\"NONE\", \"PROXY_V1\"]"]
    pub fn set_proxy_header(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.proxy_header = Some(v.into());
        self
    }
    #[doc = "Set the field `request`.\nThe application data to send once the TCP connection has been\nestablished (default value is empty). If both request and response are\nempty, the connection establishment alone will indicate health. The request\ndata can only be ASCII."]
    pub fn set_request(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.request = Some(v.into());
        self
    }
    #[doc = "Set the field `response`.\nThe bytes to match against the beginning of the response data. If left empty\n(the default value), any response will indicate health. The response data\ncan only be ASCII."]
    pub fn set_response(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeHealthCheckTcpHealthCheckEl {
    type O = BlockAssignable<ComputeHealthCheckTcpHealthCheckEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeHealthCheckTcpHealthCheckEl {}
impl BuildComputeHealthCheckTcpHealthCheckEl {
    pub fn build(self) -> ComputeHealthCheckTcpHealthCheckEl {
        ComputeHealthCheckTcpHealthCheckEl {
            port: core::default::Default::default(),
            port_name: core::default::Default::default(),
            port_specification: core::default::Default::default(),
            proxy_header: core::default::Default::default(),
            request: core::default::Default::default(),
            response: core::default::Default::default(),
        }
    }
}
pub struct ComputeHealthCheckTcpHealthCheckElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeHealthCheckTcpHealthCheckElRef {
    fn new(shared: StackShared, base: String) -> ComputeHealthCheckTcpHealthCheckElRef {
        ComputeHealthCheckTcpHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeHealthCheckTcpHealthCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe TCP port number for the TCP health check request.\nThe default value is 443."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `port_name` after provisioning.\nPort name as defined in InstanceGroup#NamedPort#name. If both port and\nport_name are defined, port takes precedence."]
    pub fn port_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }
    #[doc = "Get a reference to the value of field `port_specification` after provisioning.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': The 'portName' is used for health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, TCP health check follows behavior specified in 'port' and\n'portName' fields. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn port_specification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port_specification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `proxy_header` after provisioning.\nSpecifies the type of proxy header to append before sending data to the\nbackend. Default value: \"NONE\" Possible values: [\"NONE\", \"PROXY_V1\"]"]
    pub fn proxy_header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxy_header", self.base))
    }
    #[doc = "Get a reference to the value of field `request` after provisioning.\nThe application data to send once the TCP connection has been\nestablished (default value is empty). If both request and response are\nempty, the connection establishment alone will indicate health. The request\ndata can only be ASCII."]
    pub fn request(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request", self.base))
    }
    #[doc = "Get a reference to the value of field `response` after provisioning.\nThe bytes to match against the beginning of the response data. If left empty\n(the default value), any response will indicate health. The response data\ncan only be ASCII."]
    pub fn response(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.response", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeHealthCheckTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeHealthCheckTimeoutsEl {
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
impl ToListMappable for ComputeHealthCheckTimeoutsEl {
    type O = BlockAssignable<ComputeHealthCheckTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeHealthCheckTimeoutsEl {}
impl BuildComputeHealthCheckTimeoutsEl {
    pub fn build(self) -> ComputeHealthCheckTimeoutsEl {
        ComputeHealthCheckTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeHealthCheckTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeHealthCheckTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeHealthCheckTimeoutsElRef {
        ComputeHealthCheckTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeHealthCheckTimeoutsElRef {
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
struct ComputeHealthCheckDynamic {
    grpc_health_check: Option<DynamicBlock<ComputeHealthCheckGrpcHealthCheckEl>>,
    grpc_tls_health_check: Option<DynamicBlock<ComputeHealthCheckGrpcTlsHealthCheckEl>>,
    http2_health_check: Option<DynamicBlock<ComputeHealthCheckHttp2HealthCheckEl>>,
    http_health_check: Option<DynamicBlock<ComputeHealthCheckHttpHealthCheckEl>>,
    https_health_check: Option<DynamicBlock<ComputeHealthCheckHttpsHealthCheckEl>>,
    log_config: Option<DynamicBlock<ComputeHealthCheckLogConfigEl>>,
    ssl_health_check: Option<DynamicBlock<ComputeHealthCheckSslHealthCheckEl>>,
    tcp_health_check: Option<DynamicBlock<ComputeHealthCheckTcpHealthCheckEl>>,
}
