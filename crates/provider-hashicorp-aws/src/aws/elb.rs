use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ElbData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zones: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_draining: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_draining_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cross_zone_load_balancing: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desync_mitigation_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instances: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_security_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnets: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_logs: Option<Vec<ElbAccessLogsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check: Option<Vec<ElbHealthCheckEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    listener: Option<Vec<ElbListenerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ElbTimeoutsEl>,
    dynamic: ElbDynamic,
}
struct Elb_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ElbData>,
}
#[derive(Clone)]
pub struct Elb(Rc<Elb_>);
impl Elb {
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
    #[doc = "Set the field `availability_zones`.\n"]
    pub fn set_availability_zones(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().availability_zones = Some(v.into());
        self
    }
    #[doc = "Set the field `connection_draining`.\n"]
    pub fn set_connection_draining(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().connection_draining = Some(v.into());
        self
    }
    #[doc = "Set the field `connection_draining_timeout`.\n"]
    pub fn set_connection_draining_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().connection_draining_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `cross_zone_load_balancing`.\n"]
    pub fn set_cross_zone_load_balancing(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().cross_zone_load_balancing = Some(v.into());
        self
    }
    #[doc = "Set the field `desync_mitigation_mode`.\n"]
    pub fn set_desync_mitigation_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().desync_mitigation_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `idle_timeout`.\n"]
    pub fn set_idle_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().idle_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `instances`.\n"]
    pub fn set_instances(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().instances = Some(v.into());
        self
    }
    #[doc = "Set the field `internal`.\n"]
    pub fn set_internal(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().internal = Some(v.into());
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().security_groups = Some(v.into());
        self
    }
    #[doc = "Set the field `source_security_group`.\n"]
    pub fn set_source_security_group(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_security_group = Some(v.into());
        self
    }
    #[doc = "Set the field `subnets`.\n"]
    pub fn set_subnets(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().subnets = Some(v.into());
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
    #[doc = "Set the field `access_logs`.\n"]
    pub fn set_access_logs(self, v: impl Into<BlockAssignable<ElbAccessLogsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_logs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_logs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(self, v: impl Into<BlockAssignable<ElbHealthCheckEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().health_check = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.health_check = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `listener`.\n"]
    pub fn set_listener(self, v: impl Into<BlockAssignable<ElbListenerEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().listener = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.listener = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ElbTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.availability_zones", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_draining` after provisioning.\n"]
    pub fn connection_draining(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_draining", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_draining_timeout` after provisioning.\n"]
    pub fn connection_draining_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_draining_timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cross_zone_load_balancing` after provisioning.\n"]
    pub fn cross_zone_load_balancing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cross_zone_load_balancing", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desync_mitigation_mode` after provisioning.\n"]
    pub fn desync_mitigation_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desync_mitigation_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `idle_timeout` after provisioning.\n"]
    pub fn idle_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idle_timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.instances", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `internal` after provisioning.\n"]
    pub fn internal(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.internal", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_security_group` after provisioning.\n"]
    pub fn source_security_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_security_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_security_group_id` after provisioning.\n"]
    pub fn source_security_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_security_group_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subnets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `access_logs` after provisioning.\n"]
    pub fn access_logs(&self) -> ListRef<ElbAccessLogsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_logs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> ListRef<ElbHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ElbTimeoutsElRef {
        ElbTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for Elb {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for Elb {}
impl ToListMappable for Elb {
    type O = ListRef<ElbRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for Elb_ {
    fn extract_resource_type(&self) -> String {
        "aws_elb".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildElb {
    pub tf_id: String,
}
impl BuildElb {
    pub fn build(self, stack: &mut Stack) -> Elb {
        let out = Elb(Rc::new(Elb_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ElbData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                availability_zones: core::default::Default::default(),
                connection_draining: core::default::Default::default(),
                connection_draining_timeout: core::default::Default::default(),
                cross_zone_load_balancing: core::default::Default::default(),
                desync_mitigation_mode: core::default::Default::default(),
                id: core::default::Default::default(),
                idle_timeout: core::default::Default::default(),
                instances: core::default::Default::default(),
                internal: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                region: core::default::Default::default(),
                security_groups: core::default::Default::default(),
                source_security_group: core::default::Default::default(),
                subnets: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                access_logs: core::default::Default::default(),
                health_check: core::default::Default::default(),
                listener: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ElbRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElbRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ElbRef {
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
    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.availability_zones", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_draining` after provisioning.\n"]
    pub fn connection_draining(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_draining", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_draining_timeout` after provisioning.\n"]
    pub fn connection_draining_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_draining_timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cross_zone_load_balancing` after provisioning.\n"]
    pub fn cross_zone_load_balancing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cross_zone_load_balancing", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desync_mitigation_mode` after provisioning.\n"]
    pub fn desync_mitigation_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desync_mitigation_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `idle_timeout` after provisioning.\n"]
    pub fn idle_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idle_timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.instances", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `internal` after provisioning.\n"]
    pub fn internal(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.internal", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_security_group` after provisioning.\n"]
    pub fn source_security_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_security_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_security_group_id` after provisioning.\n"]
    pub fn source_security_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_security_group_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subnets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `access_logs` after provisioning.\n"]
    pub fn access_logs(&self) -> ListRef<ElbAccessLogsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_logs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> ListRef<ElbHealthCheckElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.health_check", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ElbTimeoutsElRef {
        ElbTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ElbAccessLogsEl {
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<PrimField<f64>>,
}
impl ElbAccessLogsEl {
    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.interval = Some(v.into());
        self
    }
}
impl ToListMappable for ElbAccessLogsEl {
    type O = BlockAssignable<ElbAccessLogsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElbAccessLogsEl {
    #[doc = ""]
    pub bucket: PrimField<String>,
}
impl BuildElbAccessLogsEl {
    pub fn build(self) -> ElbAccessLogsEl {
        ElbAccessLogsEl {
            bucket: self.bucket,
            bucket_prefix: core::default::Default::default(),
            enabled: core::default::Default::default(),
            interval: core::default::Default::default(),
        }
    }
}
pub struct ElbAccessLogsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElbAccessLogsElRef {
    fn new(shared: StackShared, base: String) -> ElbAccessLogsElRef {
        ElbAccessLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElbAccessLogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }
}
#[derive(Serialize)]
pub struct ElbHealthCheckEl {
    healthy_threshold: PrimField<f64>,
    interval: PrimField<f64>,
    target: PrimField<String>,
    timeout: PrimField<f64>,
    unhealthy_threshold: PrimField<f64>,
}
impl ElbHealthCheckEl {}
impl ToListMappable for ElbHealthCheckEl {
    type O = BlockAssignable<ElbHealthCheckEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElbHealthCheckEl {
    #[doc = ""]
    pub healthy_threshold: PrimField<f64>,
    #[doc = ""]
    pub interval: PrimField<f64>,
    #[doc = ""]
    pub target: PrimField<String>,
    #[doc = ""]
    pub timeout: PrimField<f64>,
    #[doc = ""]
    pub unhealthy_threshold: PrimField<f64>,
}
impl BuildElbHealthCheckEl {
    pub fn build(self) -> ElbHealthCheckEl {
        ElbHealthCheckEl {
            healthy_threshold: self.healthy_threshold,
            interval: self.interval,
            target: self.target,
            timeout: self.timeout,
            unhealthy_threshold: self.unhealthy_threshold,
        }
    }
}
pub struct ElbHealthCheckElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElbHealthCheckElRef {
    fn new(shared: StackShared, base: String) -> ElbHealthCheckElRef {
        ElbHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElbHealthCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `healthy_threshold` after provisioning.\n"]
    pub fn healthy_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.healthy_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
    #[doc = "Get a reference to the value of field `unhealthy_threshold` after provisioning.\n"]
    pub fn unhealthy_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unhealthy_threshold", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ElbListenerEl {
    instance_port: PrimField<f64>,
    instance_protocol: PrimField<String>,
    lb_port: PrimField<f64>,
    lb_protocol: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_certificate_id: Option<PrimField<String>>,
}
impl ElbListenerEl {
    #[doc = "Set the field `ssl_certificate_id`.\n"]
    pub fn set_ssl_certificate_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_certificate_id = Some(v.into());
        self
    }
}
impl ToListMappable for ElbListenerEl {
    type O = BlockAssignable<ElbListenerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElbListenerEl {
    #[doc = ""]
    pub instance_port: PrimField<f64>,
    #[doc = ""]
    pub instance_protocol: PrimField<String>,
    #[doc = ""]
    pub lb_port: PrimField<f64>,
    #[doc = ""]
    pub lb_protocol: PrimField<String>,
}
impl BuildElbListenerEl {
    pub fn build(self) -> ElbListenerEl {
        ElbListenerEl {
            instance_port: self.instance_port,
            instance_protocol: self.instance_protocol,
            lb_port: self.lb_port,
            lb_protocol: self.lb_protocol,
            ssl_certificate_id: core::default::Default::default(),
        }
    }
}
pub struct ElbListenerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElbListenerElRef {
    fn new(shared: StackShared, base: String) -> ElbListenerElRef {
        ElbListenerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElbListenerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_port` after provisioning.\n"]
    pub fn instance_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_port", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance_protocol` after provisioning.\n"]
    pub fn instance_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_protocol", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lb_port` after provisioning.\n"]
    pub fn lb_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.lb_port", self.base))
    }
    #[doc = "Get a reference to the value of field `lb_protocol` after provisioning.\n"]
    pub fn lb_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lb_protocol", self.base))
    }
    #[doc = "Get a reference to the value of field `ssl_certificate_id` after provisioning.\n"]
    pub fn ssl_certificate_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_certificate_id", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ElbTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ElbTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for ElbTimeoutsEl {
    type O = BlockAssignable<ElbTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElbTimeoutsEl {}
impl BuildElbTimeoutsEl {
    pub fn build(self) -> ElbTimeoutsEl {
        ElbTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ElbTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElbTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ElbTimeoutsElRef {
        ElbTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElbTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct ElbDynamic {
    access_logs: Option<DynamicBlock<ElbAccessLogsEl>>,
    health_check: Option<DynamicBlock<ElbHealthCheckEl>>,
    listener: Option<DynamicBlock<ElbListenerEl>>,
}
