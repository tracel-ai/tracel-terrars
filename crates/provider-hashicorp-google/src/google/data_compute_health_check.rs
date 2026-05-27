use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeHealthCheckData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataComputeHealthCheck_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeHealthCheckData>,
}
#[derive(Clone)]
pub struct DataComputeHealthCheck(Rc<DataComputeHealthCheck_>);
impl DataComputeHealthCheck {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
    #[doc = "Get a reference to the value of field `grpc_health_check` after provisioning.\nA nested object resource."]
    pub fn grpc_health_check(&self) -> ListRef<DataComputeHealthCheckGrpcHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.grpc_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `grpc_tls_health_check` after provisioning.\nA nested object resource."]
    pub fn grpc_tls_health_check(&self) -> ListRef<DataComputeHealthCheckGrpcTlsHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.grpc_tls_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `healthy_threshold` after provisioning.\nA so-far unhealthy instance will be marked healthy after this many\nconsecutive successes. The default value is 2."]
    pub fn healthy_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.healthy_threshold", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `http2_health_check` after provisioning.\nA nested object resource."]
    pub fn http2_health_check(&self) -> ListRef<DataComputeHealthCheckHttp2HealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http2_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `http_health_check` after provisioning.\nA nested object resource."]
    pub fn http_health_check(&self) -> ListRef<DataComputeHealthCheckHttpHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `https_health_check` after provisioning.\nA nested object resource."]
    pub fn https_health_check(&self) -> ListRef<DataComputeHealthCheckHttpsHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.https_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `log_config` after provisioning.\nConfigure logging on this health check."]
    pub fn log_config(&self) -> ListRef<DataComputeHealthCheckLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `ssl_health_check` after provisioning.\nA nested object resource."]
    pub fn ssl_health_check(&self) -> ListRef<DataComputeHealthCheckSslHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ssl_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tcp_health_check` after provisioning.\nA nested object resource."]
    pub fn tcp_health_check(&self) -> ListRef<DataComputeHealthCheckTcpHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tcp_health_check", self.extract_ref()),
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
}
impl Referable for DataComputeHealthCheck {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeHealthCheck {}
impl ToListMappable for DataComputeHealthCheck {
    type O = ListRef<DataComputeHealthCheckRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeHealthCheck_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_health_check".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeHealthCheck {
    pub tf_id: String,
    #[doc = "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
    pub name: PrimField<String>,
}
impl BuildDataComputeHealthCheck {
    pub fn build(self, stack: &mut Stack) -> DataComputeHealthCheck {
        let out = DataComputeHealthCheck(Rc::new(DataComputeHealthCheck_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeHealthCheckData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeHealthCheckRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeHealthCheckRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeHealthCheckRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
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
    #[doc = "Get a reference to the value of field `grpc_health_check` after provisioning.\nA nested object resource."]
    pub fn grpc_health_check(&self) -> ListRef<DataComputeHealthCheckGrpcHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.grpc_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `grpc_tls_health_check` after provisioning.\nA nested object resource."]
    pub fn grpc_tls_health_check(&self) -> ListRef<DataComputeHealthCheckGrpcTlsHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.grpc_tls_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `healthy_threshold` after provisioning.\nA so-far unhealthy instance will be marked healthy after this many\nconsecutive successes. The default value is 2."]
    pub fn healthy_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.healthy_threshold", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `http2_health_check` after provisioning.\nA nested object resource."]
    pub fn http2_health_check(&self) -> ListRef<DataComputeHealthCheckHttp2HealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http2_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `http_health_check` after provisioning.\nA nested object resource."]
    pub fn http_health_check(&self) -> ListRef<DataComputeHealthCheckHttpHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `https_health_check` after provisioning.\nA nested object resource."]
    pub fn https_health_check(&self) -> ListRef<DataComputeHealthCheckHttpsHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.https_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `log_config` after provisioning.\nConfigure logging on this health check."]
    pub fn log_config(&self) -> ListRef<DataComputeHealthCheckLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `ssl_health_check` after provisioning.\nA nested object resource."]
    pub fn ssl_health_check(&self) -> ListRef<DataComputeHealthCheckSslHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ssl_health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tcp_health_check` after provisioning.\nA nested object resource."]
    pub fn tcp_health_check(&self) -> ListRef<DataComputeHealthCheckTcpHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tcp_health_check", self.extract_ref()),
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
}
#[derive(Serialize)]
pub struct DataComputeHealthCheckGrpcHealthCheckEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc_service_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_specification: Option<PrimField<String>>,
}
impl DataComputeHealthCheckGrpcHealthCheckEl {
    #[doc = "Set the field `grpc_service_name`.\n"]
    pub fn set_grpc_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.grpc_service_name = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `port_name`.\n"]
    pub fn set_port_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_name = Some(v.into());
        self
    }
    #[doc = "Set the field `port_specification`.\n"]
    pub fn set_port_specification(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_specification = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeHealthCheckGrpcHealthCheckEl {
    type O = BlockAssignable<DataComputeHealthCheckGrpcHealthCheckEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeHealthCheckGrpcHealthCheckEl {}
impl BuildDataComputeHealthCheckGrpcHealthCheckEl {
    pub fn build(self) -> DataComputeHealthCheckGrpcHealthCheckEl {
        DataComputeHealthCheckGrpcHealthCheckEl {
            grpc_service_name: core::default::Default::default(),
            port: core::default::Default::default(),
            port_name: core::default::Default::default(),
            port_specification: core::default::Default::default(),
        }
    }
}
pub struct DataComputeHealthCheckGrpcHealthCheckElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeHealthCheckGrpcHealthCheckElRef {
    fn new(shared: StackShared, base: String) -> DataComputeHealthCheckGrpcHealthCheckElRef {
        DataComputeHealthCheckGrpcHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeHealthCheckGrpcHealthCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `grpc_service_name` after provisioning.\n"]
    pub fn grpc_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.grpc_service_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `port_name` after provisioning.\n"]
    pub fn port_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }
    #[doc = "Get a reference to the value of field `port_specification` after provisioning.\n"]
    pub fn port_specification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port_specification", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeHealthCheckGrpcTlsHealthCheckEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc_service_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_specification: Option<PrimField<String>>,
}
impl DataComputeHealthCheckGrpcTlsHealthCheckEl {
    #[doc = "Set the field `grpc_service_name`.\n"]
    pub fn set_grpc_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.grpc_service_name = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `port_specification`.\n"]
    pub fn set_port_specification(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_specification = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeHealthCheckGrpcTlsHealthCheckEl {
    type O = BlockAssignable<DataComputeHealthCheckGrpcTlsHealthCheckEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeHealthCheckGrpcTlsHealthCheckEl {}
impl BuildDataComputeHealthCheckGrpcTlsHealthCheckEl {
    pub fn build(self) -> DataComputeHealthCheckGrpcTlsHealthCheckEl {
        DataComputeHealthCheckGrpcTlsHealthCheckEl {
            grpc_service_name: core::default::Default::default(),
            port: core::default::Default::default(),
            port_specification: core::default::Default::default(),
        }
    }
}
pub struct DataComputeHealthCheckGrpcTlsHealthCheckElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeHealthCheckGrpcTlsHealthCheckElRef {
    fn new(shared: StackShared, base: String) -> DataComputeHealthCheckGrpcTlsHealthCheckElRef {
        DataComputeHealthCheckGrpcTlsHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeHealthCheckGrpcTlsHealthCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `grpc_service_name` after provisioning.\n"]
    pub fn grpc_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.grpc_service_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `port_specification` after provisioning.\n"]
    pub fn port_specification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port_specification", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeHealthCheckHttp2HealthCheckEl {
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
impl DataComputeHealthCheckHttp2HealthCheckEl {
    #[doc = "Set the field `host`.\n"]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `port_name`.\n"]
    pub fn set_port_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_name = Some(v.into());
        self
    }
    #[doc = "Set the field `port_specification`.\n"]
    pub fn set_port_specification(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_specification = Some(v.into());
        self
    }
    #[doc = "Set the field `proxy_header`.\n"]
    pub fn set_proxy_header(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.proxy_header = Some(v.into());
        self
    }
    #[doc = "Set the field `request_path`.\n"]
    pub fn set_request_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.request_path = Some(v.into());
        self
    }
    #[doc = "Set the field `response`.\n"]
    pub fn set_response(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeHealthCheckHttp2HealthCheckEl {
    type O = BlockAssignable<DataComputeHealthCheckHttp2HealthCheckEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeHealthCheckHttp2HealthCheckEl {}
impl BuildDataComputeHealthCheckHttp2HealthCheckEl {
    pub fn build(self) -> DataComputeHealthCheckHttp2HealthCheckEl {
        DataComputeHealthCheckHttp2HealthCheckEl {
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
pub struct DataComputeHealthCheckHttp2HealthCheckElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeHealthCheckHttp2HealthCheckElRef {
    fn new(shared: StackShared, base: String) -> DataComputeHealthCheckHttp2HealthCheckElRef {
        DataComputeHealthCheckHttp2HealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeHealthCheckHttp2HealthCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `port_name` after provisioning.\n"]
    pub fn port_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }
    #[doc = "Get a reference to the value of field `port_specification` after provisioning.\n"]
    pub fn port_specification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port_specification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `proxy_header` after provisioning.\n"]
    pub fn proxy_header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxy_header", self.base))
    }
    #[doc = "Get a reference to the value of field `request_path` after provisioning.\n"]
    pub fn request_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_path", self.base))
    }
    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.response", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeHealthCheckHttpHealthCheckEl {
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
impl DataComputeHealthCheckHttpHealthCheckEl {
    #[doc = "Set the field `host`.\n"]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `port_name`.\n"]
    pub fn set_port_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_name = Some(v.into());
        self
    }
    #[doc = "Set the field `port_specification`.\n"]
    pub fn set_port_specification(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_specification = Some(v.into());
        self
    }
    #[doc = "Set the field `proxy_header`.\n"]
    pub fn set_proxy_header(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.proxy_header = Some(v.into());
        self
    }
    #[doc = "Set the field `request_path`.\n"]
    pub fn set_request_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.request_path = Some(v.into());
        self
    }
    #[doc = "Set the field `response`.\n"]
    pub fn set_response(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeHealthCheckHttpHealthCheckEl {
    type O = BlockAssignable<DataComputeHealthCheckHttpHealthCheckEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeHealthCheckHttpHealthCheckEl {}
impl BuildDataComputeHealthCheckHttpHealthCheckEl {
    pub fn build(self) -> DataComputeHealthCheckHttpHealthCheckEl {
        DataComputeHealthCheckHttpHealthCheckEl {
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
pub struct DataComputeHealthCheckHttpHealthCheckElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeHealthCheckHttpHealthCheckElRef {
    fn new(shared: StackShared, base: String) -> DataComputeHealthCheckHttpHealthCheckElRef {
        DataComputeHealthCheckHttpHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeHealthCheckHttpHealthCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `port_name` after provisioning.\n"]
    pub fn port_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }
    #[doc = "Get a reference to the value of field `port_specification` after provisioning.\n"]
    pub fn port_specification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port_specification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `proxy_header` after provisioning.\n"]
    pub fn proxy_header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxy_header", self.base))
    }
    #[doc = "Get a reference to the value of field `request_path` after provisioning.\n"]
    pub fn request_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_path", self.base))
    }
    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.response", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeHealthCheckHttpsHealthCheckEl {
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
impl DataComputeHealthCheckHttpsHealthCheckEl {
    #[doc = "Set the field `host`.\n"]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `port_name`.\n"]
    pub fn set_port_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_name = Some(v.into());
        self
    }
    #[doc = "Set the field `port_specification`.\n"]
    pub fn set_port_specification(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_specification = Some(v.into());
        self
    }
    #[doc = "Set the field `proxy_header`.\n"]
    pub fn set_proxy_header(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.proxy_header = Some(v.into());
        self
    }
    #[doc = "Set the field `request_path`.\n"]
    pub fn set_request_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.request_path = Some(v.into());
        self
    }
    #[doc = "Set the field `response`.\n"]
    pub fn set_response(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeHealthCheckHttpsHealthCheckEl {
    type O = BlockAssignable<DataComputeHealthCheckHttpsHealthCheckEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeHealthCheckHttpsHealthCheckEl {}
impl BuildDataComputeHealthCheckHttpsHealthCheckEl {
    pub fn build(self) -> DataComputeHealthCheckHttpsHealthCheckEl {
        DataComputeHealthCheckHttpsHealthCheckEl {
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
pub struct DataComputeHealthCheckHttpsHealthCheckElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeHealthCheckHttpsHealthCheckElRef {
    fn new(shared: StackShared, base: String) -> DataComputeHealthCheckHttpsHealthCheckElRef {
        DataComputeHealthCheckHttpsHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeHealthCheckHttpsHealthCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `port_name` after provisioning.\n"]
    pub fn port_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }
    #[doc = "Get a reference to the value of field `port_specification` after provisioning.\n"]
    pub fn port_specification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port_specification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `proxy_header` after provisioning.\n"]
    pub fn proxy_header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxy_header", self.base))
    }
    #[doc = "Get a reference to the value of field `request_path` after provisioning.\n"]
    pub fn request_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_path", self.base))
    }
    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.response", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeHealthCheckLogConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable: Option<PrimField<bool>>,
}
impl DataComputeHealthCheckLogConfigEl {
    #[doc = "Set the field `enable`.\n"]
    pub fn set_enable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeHealthCheckLogConfigEl {
    type O = BlockAssignable<DataComputeHealthCheckLogConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeHealthCheckLogConfigEl {}
impl BuildDataComputeHealthCheckLogConfigEl {
    pub fn build(self) -> DataComputeHealthCheckLogConfigEl {
        DataComputeHealthCheckLogConfigEl {
            enable: core::default::Default::default(),
        }
    }
}
pub struct DataComputeHealthCheckLogConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeHealthCheckLogConfigElRef {
    fn new(shared: StackShared, base: String) -> DataComputeHealthCheckLogConfigElRef {
        DataComputeHealthCheckLogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeHealthCheckLogConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeHealthCheckSslHealthCheckEl {
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
impl DataComputeHealthCheckSslHealthCheckEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `port_name`.\n"]
    pub fn set_port_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_name = Some(v.into());
        self
    }
    #[doc = "Set the field `port_specification`.\n"]
    pub fn set_port_specification(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_specification = Some(v.into());
        self
    }
    #[doc = "Set the field `proxy_header`.\n"]
    pub fn set_proxy_header(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.proxy_header = Some(v.into());
        self
    }
    #[doc = "Set the field `request`.\n"]
    pub fn set_request(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.request = Some(v.into());
        self
    }
    #[doc = "Set the field `response`.\n"]
    pub fn set_response(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeHealthCheckSslHealthCheckEl {
    type O = BlockAssignable<DataComputeHealthCheckSslHealthCheckEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeHealthCheckSslHealthCheckEl {}
impl BuildDataComputeHealthCheckSslHealthCheckEl {
    pub fn build(self) -> DataComputeHealthCheckSslHealthCheckEl {
        DataComputeHealthCheckSslHealthCheckEl {
            port: core::default::Default::default(),
            port_name: core::default::Default::default(),
            port_specification: core::default::Default::default(),
            proxy_header: core::default::Default::default(),
            request: core::default::Default::default(),
            response: core::default::Default::default(),
        }
    }
}
pub struct DataComputeHealthCheckSslHealthCheckElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeHealthCheckSslHealthCheckElRef {
    fn new(shared: StackShared, base: String) -> DataComputeHealthCheckSslHealthCheckElRef {
        DataComputeHealthCheckSslHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeHealthCheckSslHealthCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `port_name` after provisioning.\n"]
    pub fn port_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }
    #[doc = "Get a reference to the value of field `port_specification` after provisioning.\n"]
    pub fn port_specification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port_specification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `proxy_header` after provisioning.\n"]
    pub fn proxy_header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxy_header", self.base))
    }
    #[doc = "Get a reference to the value of field `request` after provisioning.\n"]
    pub fn request(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request", self.base))
    }
    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.response", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeHealthCheckTcpHealthCheckEl {
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
impl DataComputeHealthCheckTcpHealthCheckEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `port_name`.\n"]
    pub fn set_port_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_name = Some(v.into());
        self
    }
    #[doc = "Set the field `port_specification`.\n"]
    pub fn set_port_specification(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_specification = Some(v.into());
        self
    }
    #[doc = "Set the field `proxy_header`.\n"]
    pub fn set_proxy_header(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.proxy_header = Some(v.into());
        self
    }
    #[doc = "Set the field `request`.\n"]
    pub fn set_request(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.request = Some(v.into());
        self
    }
    #[doc = "Set the field `response`.\n"]
    pub fn set_response(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeHealthCheckTcpHealthCheckEl {
    type O = BlockAssignable<DataComputeHealthCheckTcpHealthCheckEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeHealthCheckTcpHealthCheckEl {}
impl BuildDataComputeHealthCheckTcpHealthCheckEl {
    pub fn build(self) -> DataComputeHealthCheckTcpHealthCheckEl {
        DataComputeHealthCheckTcpHealthCheckEl {
            port: core::default::Default::default(),
            port_name: core::default::Default::default(),
            port_specification: core::default::Default::default(),
            proxy_header: core::default::Default::default(),
            request: core::default::Default::default(),
            response: core::default::Default::default(),
        }
    }
}
pub struct DataComputeHealthCheckTcpHealthCheckElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeHealthCheckTcpHealthCheckElRef {
    fn new(shared: StackShared, base: String) -> DataComputeHealthCheckTcpHealthCheckElRef {
        DataComputeHealthCheckTcpHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeHealthCheckTcpHealthCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `port_name` after provisioning.\n"]
    pub fn port_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }
    #[doc = "Get a reference to the value of field `port_specification` after provisioning.\n"]
    pub fn port_specification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port_specification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `proxy_header` after provisioning.\n"]
    pub fn proxy_header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxy_header", self.base))
    }
    #[doc = "Get a reference to the value of field `request` after provisioning.\n"]
    pub fn request(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request", self.base))
    }
    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.response", self.base))
    }
}
