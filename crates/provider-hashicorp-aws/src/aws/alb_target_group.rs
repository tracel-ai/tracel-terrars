use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AlbTargetGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_termination: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deregistration_delay: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_multi_value_headers_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancing_algorithm_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancing_anomaly_mitigation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancing_cross_zone_enabled: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preserve_client_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_protocol_v2: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slow_start: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check: Option<Vec<AlbTargetGroupHealthCheckEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stickiness: Option<Vec<AlbTargetGroupStickinessEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_failover: Option<Vec<AlbTargetGroupTargetFailoverEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_group_health: Option<Vec<AlbTargetGroupTargetGroupHealthEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_health_state: Option<Vec<AlbTargetGroupTargetHealthStateEl>>,
    dynamic: AlbTargetGroupDynamic,
}

struct AlbTargetGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AlbTargetGroupData>,
}

#[derive(Clone)]
pub struct AlbTargetGroup(Rc<AlbTargetGroup_>);

impl AlbTargetGroup {
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

    #[doc = "Set the field `connection_termination`.\n"]
    pub fn set_connection_termination(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().connection_termination = Some(v.into());
        self
    }

    #[doc = "Set the field `deregistration_delay`.\n"]
    pub fn set_deregistration_delay(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deregistration_delay = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `ip_address_type`.\n"]
    pub fn set_ip_address_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ip_address_type = Some(v.into());
        self
    }

    #[doc = "Set the field `lambda_multi_value_headers_enabled`.\n"]
    pub fn set_lambda_multi_value_headers_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().lambda_multi_value_headers_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `load_balancing_algorithm_type`.\n"]
    pub fn set_load_balancing_algorithm_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().load_balancing_algorithm_type = Some(v.into());
        self
    }

    #[doc = "Set the field `load_balancing_anomaly_mitigation`.\n"]
    pub fn set_load_balancing_anomaly_mitigation(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().load_balancing_anomaly_mitigation = Some(v.into());
        self
    }

    #[doc = "Set the field `load_balancing_cross_zone_enabled`.\n"]
    pub fn set_load_balancing_cross_zone_enabled(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().load_balancing_cross_zone_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = Some(v.into());
        self
    }

    #[doc = "Set the field `preserve_client_ip`.\n"]
    pub fn set_preserve_client_ip(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().preserve_client_ip = Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().protocol = Some(v.into());
        self
    }

    #[doc = "Set the field `protocol_version`.\n"]
    pub fn set_protocol_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().protocol_version = Some(v.into());
        self
    }

    #[doc = "Set the field `proxy_protocol_v2`.\n"]
    pub fn set_proxy_protocol_v2(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().proxy_protocol_v2 = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `slow_start`.\n"]
    pub fn set_slow_start(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().slow_start = Some(v.into());
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

    #[doc = "Set the field `target_type`.\n"]
    pub fn set_target_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().target_type = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().vpc_id = Some(v.into());
        self
    }

    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(self, v: impl Into<BlockAssignable<AlbTargetGroupHealthCheckEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().health_check = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.health_check = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stickiness`.\n"]
    pub fn set_stickiness(self, v: impl Into<BlockAssignable<AlbTargetGroupStickinessEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stickiness = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stickiness = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_failover`.\n"]
    pub fn set_target_failover(self, v: impl Into<BlockAssignable<AlbTargetGroupTargetFailoverEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_failover = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_failover = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_group_health`.\n"]
    pub fn set_target_group_health(self, v: impl Into<BlockAssignable<AlbTargetGroupTargetGroupHealthEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_group_health = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_group_health = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_health_state`.\n"]
    pub fn set_target_health_state(self, v: impl Into<BlockAssignable<AlbTargetGroupTargetHealthStateEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_health_state = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_health_state = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn_suffix` after provisioning.\n"]
    pub fn arn_suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn_suffix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_termination` after provisioning.\n"]
    pub fn connection_termination(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection_termination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deregistration_delay` after provisioning.\n"]
    pub fn deregistration_delay(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deregistration_delay", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_multi_value_headers_enabled` after provisioning.\n"]
    pub fn lambda_multi_value_headers_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.lambda_multi_value_headers_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer_arns` after provisioning.\n"]
    pub fn load_balancer_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.load_balancer_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancing_algorithm_type` after provisioning.\n"]
    pub fn load_balancing_algorithm_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.load_balancing_algorithm_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancing_anomaly_mitigation` after provisioning.\n"]
    pub fn load_balancing_anomaly_mitigation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.load_balancing_anomaly_mitigation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancing_cross_zone_enabled` after provisioning.\n"]
    pub fn load_balancing_cross_zone_enabled(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.load_balancing_cross_zone_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preserve_client_ip` after provisioning.\n"]
    pub fn preserve_client_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preserve_client_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol_version` after provisioning.\n"]
    pub fn protocol_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy_protocol_v2` after provisioning.\n"]
    pub fn proxy_protocol_v2(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxy_protocol_v2", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `slow_start` after provisioning.\n"]
    pub fn slow_start(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.slow_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_type` after provisioning.\n"]
    pub fn target_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> ListRef<AlbTargetGroupHealthCheckElRef> {
        ListRef::new(self.shared().clone(), format!("{}.health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stickiness` after provisioning.\n"]
    pub fn stickiness(&self) -> ListRef<AlbTargetGroupStickinessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.stickiness", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_failover` after provisioning.\n"]
    pub fn target_failover(&self) -> ListRef<AlbTargetGroupTargetFailoverElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target_failover", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_group_health` after provisioning.\n"]
    pub fn target_group_health(&self) -> ListRef<AlbTargetGroupTargetGroupHealthElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target_group_health", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_health_state` after provisioning.\n"]
    pub fn target_health_state(&self) -> ListRef<AlbTargetGroupTargetHealthStateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target_health_state", self.extract_ref()))
    }
}

impl Referable for AlbTargetGroup {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AlbTargetGroup { }

impl ToListMappable for AlbTargetGroup {
    type O = ListRef<AlbTargetGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AlbTargetGroup_ {
    fn extract_resource_type(&self) -> String {
        "aws_alb_target_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAlbTargetGroup {
    pub tf_id: String,
}

impl BuildAlbTargetGroup {
    pub fn build(self, stack: &mut Stack) -> AlbTargetGroup {
        let out = AlbTargetGroup(Rc::new(AlbTargetGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AlbTargetGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                connection_termination: core::default::Default::default(),
                deregistration_delay: core::default::Default::default(),
                id: core::default::Default::default(),
                ip_address_type: core::default::Default::default(),
                lambda_multi_value_headers_enabled: core::default::Default::default(),
                load_balancing_algorithm_type: core::default::Default::default(),
                load_balancing_anomaly_mitigation: core::default::Default::default(),
                load_balancing_cross_zone_enabled: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                port: core::default::Default::default(),
                preserve_client_ip: core::default::Default::default(),
                protocol: core::default::Default::default(),
                protocol_version: core::default::Default::default(),
                proxy_protocol_v2: core::default::Default::default(),
                region: core::default::Default::default(),
                slow_start: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                target_type: core::default::Default::default(),
                vpc_id: core::default::Default::default(),
                health_check: core::default::Default::default(),
                stickiness: core::default::Default::default(),
                target_failover: core::default::Default::default(),
                target_group_health: core::default::Default::default(),
                target_health_state: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AlbTargetGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbTargetGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl AlbTargetGroupRef {
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

    #[doc = "Get a reference to the value of field `arn_suffix` after provisioning.\n"]
    pub fn arn_suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn_suffix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_termination` after provisioning.\n"]
    pub fn connection_termination(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection_termination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deregistration_delay` after provisioning.\n"]
    pub fn deregistration_delay(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deregistration_delay", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_multi_value_headers_enabled` after provisioning.\n"]
    pub fn lambda_multi_value_headers_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.lambda_multi_value_headers_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer_arns` after provisioning.\n"]
    pub fn load_balancer_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.load_balancer_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancing_algorithm_type` after provisioning.\n"]
    pub fn load_balancing_algorithm_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.load_balancing_algorithm_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancing_anomaly_mitigation` after provisioning.\n"]
    pub fn load_balancing_anomaly_mitigation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.load_balancing_anomaly_mitigation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancing_cross_zone_enabled` after provisioning.\n"]
    pub fn load_balancing_cross_zone_enabled(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.load_balancing_cross_zone_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preserve_client_ip` after provisioning.\n"]
    pub fn preserve_client_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preserve_client_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol_version` after provisioning.\n"]
    pub fn protocol_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy_protocol_v2` after provisioning.\n"]
    pub fn proxy_protocol_v2(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxy_protocol_v2", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `slow_start` after provisioning.\n"]
    pub fn slow_start(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.slow_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_type` after provisioning.\n"]
    pub fn target_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> ListRef<AlbTargetGroupHealthCheckElRef> {
        ListRef::new(self.shared().clone(), format!("{}.health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stickiness` after provisioning.\n"]
    pub fn stickiness(&self) -> ListRef<AlbTargetGroupStickinessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.stickiness", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_failover` after provisioning.\n"]
    pub fn target_failover(&self) -> ListRef<AlbTargetGroupTargetFailoverElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target_failover", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_group_health` after provisioning.\n"]
    pub fn target_group_health(&self) -> ListRef<AlbTargetGroupTargetGroupHealthElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target_group_health", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_health_state` after provisioning.\n"]
    pub fn target_health_state(&self) -> ListRef<AlbTargetGroupTargetHealthStateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target_health_state", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AlbTargetGroupHealthCheckEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    healthy_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matcher: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unhealthy_threshold: Option<PrimField<f64>>,
}

impl AlbTargetGroupHealthCheckEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `healthy_threshold`.\n"]
    pub fn set_healthy_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.healthy_threshold = Some(v.into());
        self
    }

    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.interval = Some(v.into());
        self
    }

    #[doc = "Set the field `matcher`.\n"]
    pub fn set_matcher(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.matcher = Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port = Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout = Some(v.into());
        self
    }

    #[doc = "Set the field `unhealthy_threshold`.\n"]
    pub fn set_unhealthy_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.unhealthy_threshold = Some(v.into());
        self
    }
}

impl ToListMappable for AlbTargetGroupHealthCheckEl {
    type O = BlockAssignable<AlbTargetGroupHealthCheckEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbTargetGroupHealthCheckEl {}

impl BuildAlbTargetGroupHealthCheckEl {
    pub fn build(self) -> AlbTargetGroupHealthCheckEl {
        AlbTargetGroupHealthCheckEl {
            enabled: core::default::Default::default(),
            healthy_threshold: core::default::Default::default(),
            interval: core::default::Default::default(),
            matcher: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            timeout: core::default::Default::default(),
            unhealthy_threshold: core::default::Default::default(),
        }
    }
}

pub struct AlbTargetGroupHealthCheckElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbTargetGroupHealthCheckElRef {
    fn new(shared: StackShared, base: String) -> AlbTargetGroupHealthCheckElRef {
        AlbTargetGroupHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbTargetGroupHealthCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `healthy_threshold` after provisioning.\n"]
    pub fn healthy_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.healthy_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `matcher` after provisioning.\n"]
    pub fn matcher(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.matcher", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `unhealthy_threshold` after provisioning.\n"]
    pub fn unhealthy_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.unhealthy_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbTargetGroupStickinessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cookie_duration: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cookie_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl AlbTargetGroupStickinessEl {
    #[doc = "Set the field `cookie_duration`.\n"]
    pub fn set_cookie_duration(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cookie_duration = Some(v.into());
        self
    }

    #[doc = "Set the field `cookie_name`.\n"]
    pub fn set_cookie_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cookie_name = Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for AlbTargetGroupStickinessEl {
    type O = BlockAssignable<AlbTargetGroupStickinessEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbTargetGroupStickinessEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildAlbTargetGroupStickinessEl {
    pub fn build(self) -> AlbTargetGroupStickinessEl {
        AlbTargetGroupStickinessEl {
            cookie_duration: core::default::Default::default(),
            cookie_name: core::default::Default::default(),
            enabled: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct AlbTargetGroupStickinessElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbTargetGroupStickinessElRef {
    fn new(shared: StackShared, base: String) -> AlbTargetGroupStickinessElRef {
        AlbTargetGroupStickinessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbTargetGroupStickinessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cookie_duration` after provisioning.\n"]
    pub fn cookie_duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cookie_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `cookie_name` after provisioning.\n"]
    pub fn cookie_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cookie_name", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbTargetGroupTargetFailoverEl {
    on_deregistration: PrimField<String>,
    on_unhealthy: PrimField<String>,
}

impl AlbTargetGroupTargetFailoverEl { }

impl ToListMappable for AlbTargetGroupTargetFailoverEl {
    type O = BlockAssignable<AlbTargetGroupTargetFailoverEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbTargetGroupTargetFailoverEl {
    #[doc = ""]
    pub on_deregistration: PrimField<String>,
    #[doc = ""]
    pub on_unhealthy: PrimField<String>,
}

impl BuildAlbTargetGroupTargetFailoverEl {
    pub fn build(self) -> AlbTargetGroupTargetFailoverEl {
        AlbTargetGroupTargetFailoverEl {
            on_deregistration: self.on_deregistration,
            on_unhealthy: self.on_unhealthy,
        }
    }
}

pub struct AlbTargetGroupTargetFailoverElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbTargetGroupTargetFailoverElRef {
    fn new(shared: StackShared, base: String) -> AlbTargetGroupTargetFailoverElRef {
        AlbTargetGroupTargetFailoverElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbTargetGroupTargetFailoverElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `on_deregistration` after provisioning.\n"]
    pub fn on_deregistration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.on_deregistration", self.base))
    }

    #[doc = "Get a reference to the value of field `on_unhealthy` after provisioning.\n"]
    pub fn on_unhealthy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.on_unhealthy", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbTargetGroupTargetGroupHealthElDnsFailoverEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_healthy_targets_count: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_healthy_targets_percentage: Option<PrimField<String>>,
}

impl AlbTargetGroupTargetGroupHealthElDnsFailoverEl {
    #[doc = "Set the field `minimum_healthy_targets_count`.\n"]
    pub fn set_minimum_healthy_targets_count(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.minimum_healthy_targets_count = Some(v.into());
        self
    }

    #[doc = "Set the field `minimum_healthy_targets_percentage`.\n"]
    pub fn set_minimum_healthy_targets_percentage(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.minimum_healthy_targets_percentage = Some(v.into());
        self
    }
}

impl ToListMappable for AlbTargetGroupTargetGroupHealthElDnsFailoverEl {
    type O = BlockAssignable<AlbTargetGroupTargetGroupHealthElDnsFailoverEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbTargetGroupTargetGroupHealthElDnsFailoverEl {}

impl BuildAlbTargetGroupTargetGroupHealthElDnsFailoverEl {
    pub fn build(self) -> AlbTargetGroupTargetGroupHealthElDnsFailoverEl {
        AlbTargetGroupTargetGroupHealthElDnsFailoverEl {
            minimum_healthy_targets_count: core::default::Default::default(),
            minimum_healthy_targets_percentage: core::default::Default::default(),
        }
    }
}

pub struct AlbTargetGroupTargetGroupHealthElDnsFailoverElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbTargetGroupTargetGroupHealthElDnsFailoverElRef {
    fn new(shared: StackShared, base: String) -> AlbTargetGroupTargetGroupHealthElDnsFailoverElRef {
        AlbTargetGroupTargetGroupHealthElDnsFailoverElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbTargetGroupTargetGroupHealthElDnsFailoverElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `minimum_healthy_targets_count` after provisioning.\n"]
    pub fn minimum_healthy_targets_count(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum_healthy_targets_count", self.base))
    }

    #[doc = "Get a reference to the value of field `minimum_healthy_targets_percentage` after provisioning.\n"]
    pub fn minimum_healthy_targets_percentage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum_healthy_targets_percentage", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbTargetGroupTargetGroupHealthElUnhealthyStateRoutingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_healthy_targets_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_healthy_targets_percentage: Option<PrimField<String>>,
}

impl AlbTargetGroupTargetGroupHealthElUnhealthyStateRoutingEl {
    #[doc = "Set the field `minimum_healthy_targets_count`.\n"]
    pub fn set_minimum_healthy_targets_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minimum_healthy_targets_count = Some(v.into());
        self
    }

    #[doc = "Set the field `minimum_healthy_targets_percentage`.\n"]
    pub fn set_minimum_healthy_targets_percentage(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.minimum_healthy_targets_percentage = Some(v.into());
        self
    }
}

impl ToListMappable for AlbTargetGroupTargetGroupHealthElUnhealthyStateRoutingEl {
    type O = BlockAssignable<AlbTargetGroupTargetGroupHealthElUnhealthyStateRoutingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbTargetGroupTargetGroupHealthElUnhealthyStateRoutingEl {}

impl BuildAlbTargetGroupTargetGroupHealthElUnhealthyStateRoutingEl {
    pub fn build(self) -> AlbTargetGroupTargetGroupHealthElUnhealthyStateRoutingEl {
        AlbTargetGroupTargetGroupHealthElUnhealthyStateRoutingEl {
            minimum_healthy_targets_count: core::default::Default::default(),
            minimum_healthy_targets_percentage: core::default::Default::default(),
        }
    }
}

pub struct AlbTargetGroupTargetGroupHealthElUnhealthyStateRoutingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbTargetGroupTargetGroupHealthElUnhealthyStateRoutingElRef {
    fn new(shared: StackShared, base: String) -> AlbTargetGroupTargetGroupHealthElUnhealthyStateRoutingElRef {
        AlbTargetGroupTargetGroupHealthElUnhealthyStateRoutingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbTargetGroupTargetGroupHealthElUnhealthyStateRoutingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `minimum_healthy_targets_count` after provisioning.\n"]
    pub fn minimum_healthy_targets_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum_healthy_targets_count", self.base))
    }

    #[doc = "Get a reference to the value of field `minimum_healthy_targets_percentage` after provisioning.\n"]
    pub fn minimum_healthy_targets_percentage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum_healthy_targets_percentage", self.base))
    }
}

#[derive(Serialize, Default)]
struct AlbTargetGroupTargetGroupHealthElDynamic {
    dns_failover: Option<DynamicBlock<AlbTargetGroupTargetGroupHealthElDnsFailoverEl>>,
    unhealthy_state_routing: Option<DynamicBlock<AlbTargetGroupTargetGroupHealthElUnhealthyStateRoutingEl>>,
}

#[derive(Serialize)]
pub struct AlbTargetGroupTargetGroupHealthEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_failover: Option<Vec<AlbTargetGroupTargetGroupHealthElDnsFailoverEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unhealthy_state_routing: Option<Vec<AlbTargetGroupTargetGroupHealthElUnhealthyStateRoutingEl>>,
    dynamic: AlbTargetGroupTargetGroupHealthElDynamic,
}

impl AlbTargetGroupTargetGroupHealthEl {
    #[doc = "Set the field `dns_failover`.\n"]
    pub fn set_dns_failover(
        mut self,
        v: impl Into<BlockAssignable<AlbTargetGroupTargetGroupHealthElDnsFailoverEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dns_failover = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dns_failover = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `unhealthy_state_routing`.\n"]
    pub fn set_unhealthy_state_routing(
        mut self,
        v: impl Into<BlockAssignable<AlbTargetGroupTargetGroupHealthElUnhealthyStateRoutingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.unhealthy_state_routing = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.unhealthy_state_routing = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AlbTargetGroupTargetGroupHealthEl {
    type O = BlockAssignable<AlbTargetGroupTargetGroupHealthEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbTargetGroupTargetGroupHealthEl {}

impl BuildAlbTargetGroupTargetGroupHealthEl {
    pub fn build(self) -> AlbTargetGroupTargetGroupHealthEl {
        AlbTargetGroupTargetGroupHealthEl {
            dns_failover: core::default::Default::default(),
            unhealthy_state_routing: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AlbTargetGroupTargetGroupHealthElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbTargetGroupTargetGroupHealthElRef {
    fn new(shared: StackShared, base: String) -> AlbTargetGroupTargetGroupHealthElRef {
        AlbTargetGroupTargetGroupHealthElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbTargetGroupTargetGroupHealthElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_failover` after provisioning.\n"]
    pub fn dns_failover(&self) -> ListRef<AlbTargetGroupTargetGroupHealthElDnsFailoverElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dns_failover", self.base))
    }

    #[doc = "Get a reference to the value of field `unhealthy_state_routing` after provisioning.\n"]
    pub fn unhealthy_state_routing(&self) -> ListRef<AlbTargetGroupTargetGroupHealthElUnhealthyStateRoutingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.unhealthy_state_routing", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbTargetGroupTargetHealthStateEl {
    enable_unhealthy_connection_termination: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unhealthy_draining_interval: Option<PrimField<f64>>,
}

impl AlbTargetGroupTargetHealthStateEl {
    #[doc = "Set the field `unhealthy_draining_interval`.\n"]
    pub fn set_unhealthy_draining_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.unhealthy_draining_interval = Some(v.into());
        self
    }
}

impl ToListMappable for AlbTargetGroupTargetHealthStateEl {
    type O = BlockAssignable<AlbTargetGroupTargetHealthStateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbTargetGroupTargetHealthStateEl {
    #[doc = ""]
    pub enable_unhealthy_connection_termination: PrimField<bool>,
}

impl BuildAlbTargetGroupTargetHealthStateEl {
    pub fn build(self) -> AlbTargetGroupTargetHealthStateEl {
        AlbTargetGroupTargetHealthStateEl {
            enable_unhealthy_connection_termination: self.enable_unhealthy_connection_termination,
            unhealthy_draining_interval: core::default::Default::default(),
        }
    }
}

pub struct AlbTargetGroupTargetHealthStateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbTargetGroupTargetHealthStateElRef {
    fn new(shared: StackShared, base: String) -> AlbTargetGroupTargetHealthStateElRef {
        AlbTargetGroupTargetHealthStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbTargetGroupTargetHealthStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_unhealthy_connection_termination` after provisioning.\n"]
    pub fn enable_unhealthy_connection_termination(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_unhealthy_connection_termination", self.base))
    }

    #[doc = "Get a reference to the value of field `unhealthy_draining_interval` after provisioning.\n"]
    pub fn unhealthy_draining_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.unhealthy_draining_interval", self.base))
    }
}

#[derive(Serialize, Default)]
struct AlbTargetGroupDynamic {
    health_check: Option<DynamicBlock<AlbTargetGroupHealthCheckEl>>,
    stickiness: Option<DynamicBlock<AlbTargetGroupStickinessEl>>,
    target_failover: Option<DynamicBlock<AlbTargetGroupTargetFailoverEl>>,
    target_group_health: Option<DynamicBlock<AlbTargetGroupTargetGroupHealthEl>>,
    target_health_state: Option<DynamicBlock<AlbTargetGroupTargetHealthStateEl>>,
}
