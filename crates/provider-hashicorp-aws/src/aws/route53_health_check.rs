use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Route53HealthCheckData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    child_health_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    child_healthchecks: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_alarm_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_alarm_region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_sni: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fqdn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insufficient_data_health_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_healthcheck: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    measure_latency: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_control_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search_string: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    triggers: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

struct Route53HealthCheck_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Route53HealthCheckData>,
}

#[derive(Clone)]
pub struct Route53HealthCheck(Rc<Route53HealthCheck_>);

impl Route53HealthCheck {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `child_health_threshold`.\n"]
    pub fn set_child_health_threshold(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().child_health_threshold = Some(v.into());
        self
    }

    #[doc = "Set the field `child_healthchecks`.\n"]
    pub fn set_child_healthchecks(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().child_healthchecks = Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_alarm_name`.\n"]
    pub fn set_cloudwatch_alarm_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloudwatch_alarm_name = Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_alarm_region`.\n"]
    pub fn set_cloudwatch_alarm_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloudwatch_alarm_region = Some(v.into());
        self
    }

    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_sni`.\n"]
    pub fn set_enable_sni(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_sni = Some(v.into());
        self
    }

    #[doc = "Set the field `failure_threshold`.\n"]
    pub fn set_failure_threshold(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().failure_threshold = Some(v.into());
        self
    }

    #[doc = "Set the field `fqdn`.\n"]
    pub fn set_fqdn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().fqdn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `insufficient_data_health_status`.\n"]
    pub fn set_insufficient_data_health_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().insufficient_data_health_status = Some(v.into());
        self
    }

    #[doc = "Set the field `invert_healthcheck`.\n"]
    pub fn set_invert_healthcheck(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().invert_healthcheck = Some(v.into());
        self
    }

    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ip_address = Some(v.into());
        self
    }

    #[doc = "Set the field `measure_latency`.\n"]
    pub fn set_measure_latency(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().measure_latency = Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = Some(v.into());
        self
    }

    #[doc = "Set the field `reference_name`.\n"]
    pub fn set_reference_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().reference_name = Some(v.into());
        self
    }

    #[doc = "Set the field `regions`.\n"]
    pub fn set_regions(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().regions = Some(v.into());
        self
    }

    #[doc = "Set the field `request_interval`.\n"]
    pub fn set_request_interval(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().request_interval = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_path`.\n"]
    pub fn set_resource_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().resource_path = Some(v.into());
        self
    }

    #[doc = "Set the field `routing_control_arn`.\n"]
    pub fn set_routing_control_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().routing_control_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `search_string`.\n"]
    pub fn set_search_string(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().search_string = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `triggers`.\n"]
    pub fn set_triggers(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().triggers = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `child_health_threshold` after provisioning.\n"]
    pub fn child_health_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.child_health_threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `child_healthchecks` after provisioning.\n"]
    pub fn child_healthchecks(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.child_healthchecks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_alarm_name` after provisioning.\n"]
    pub fn cloudwatch_alarm_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_alarm_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_alarm_region` after provisioning.\n"]
    pub fn cloudwatch_alarm_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_alarm_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_sni` after provisioning.\n"]
    pub fn enable_sni(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_sni", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\n"]
    pub fn failure_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fqdn` after provisioning.\n"]
    pub fn fqdn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fqdn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `insufficient_data_health_status` after provisioning.\n"]
    pub fn insufficient_data_health_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.insufficient_data_health_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `invert_healthcheck` after provisioning.\n"]
    pub fn invert_healthcheck(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_healthcheck", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `measure_latency` after provisioning.\n"]
    pub fn measure_latency(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.measure_latency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reference_name` after provisioning.\n"]
    pub fn reference_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reference_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_interval` after provisioning.\n"]
    pub fn request_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_path` after provisioning.\n"]
    pub fn resource_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing_control_arn` after provisioning.\n"]
    pub fn routing_control_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.routing_control_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `search_string` after provisioning.\n"]
    pub fn search_string(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.search_string", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `triggers` after provisioning.\n"]
    pub fn triggers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.triggers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl Referable for Route53HealthCheck {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Route53HealthCheck { }

impl ToListMappable for Route53HealthCheck {
    type O = ListRef<Route53HealthCheckRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Route53HealthCheck_ {
    fn extract_resource_type(&self) -> String {
        "aws_route53_health_check".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRoute53HealthCheck {
    pub tf_id: String,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildRoute53HealthCheck {
    pub fn build(self, stack: &mut Stack) -> Route53HealthCheck {
        let out = Route53HealthCheck(Rc::new(Route53HealthCheck_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Route53HealthCheckData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                child_health_threshold: core::default::Default::default(),
                child_healthchecks: core::default::Default::default(),
                cloudwatch_alarm_name: core::default::Default::default(),
                cloudwatch_alarm_region: core::default::Default::default(),
                disabled: core::default::Default::default(),
                enable_sni: core::default::Default::default(),
                failure_threshold: core::default::Default::default(),
                fqdn: core::default::Default::default(),
                id: core::default::Default::default(),
                insufficient_data_health_status: core::default::Default::default(),
                invert_healthcheck: core::default::Default::default(),
                ip_address: core::default::Default::default(),
                measure_latency: core::default::Default::default(),
                port: core::default::Default::default(),
                reference_name: core::default::Default::default(),
                regions: core::default::Default::default(),
                request_interval: core::default::Default::default(),
                resource_path: core::default::Default::default(),
                routing_control_arn: core::default::Default::default(),
                search_string: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                triggers: core::default::Default::default(),
                type_: self.type_,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Route53HealthCheckRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53HealthCheckRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl Route53HealthCheckRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `child_health_threshold` after provisioning.\n"]
    pub fn child_health_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.child_health_threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `child_healthchecks` after provisioning.\n"]
    pub fn child_healthchecks(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.child_healthchecks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_alarm_name` after provisioning.\n"]
    pub fn cloudwatch_alarm_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_alarm_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_alarm_region` after provisioning.\n"]
    pub fn cloudwatch_alarm_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_alarm_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_sni` after provisioning.\n"]
    pub fn enable_sni(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_sni", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\n"]
    pub fn failure_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fqdn` after provisioning.\n"]
    pub fn fqdn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fqdn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `insufficient_data_health_status` after provisioning.\n"]
    pub fn insufficient_data_health_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.insufficient_data_health_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `invert_healthcheck` after provisioning.\n"]
    pub fn invert_healthcheck(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_healthcheck", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `measure_latency` after provisioning.\n"]
    pub fn measure_latency(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.measure_latency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reference_name` after provisioning.\n"]
    pub fn reference_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reference_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_interval` after provisioning.\n"]
    pub fn request_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_path` after provisioning.\n"]
    pub fn resource_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing_control_arn` after provisioning.\n"]
    pub fn routing_control_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.routing_control_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `search_string` after provisioning.\n"]
    pub fn search_string(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.search_string", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `triggers` after provisioning.\n"]
    pub fn triggers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.triggers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}
