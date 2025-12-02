use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VpclatticeTargetGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Vec<VpclatticeTargetGroupConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VpclatticeTargetGroupTimeoutsEl>,
    dynamic: VpclatticeTargetGroupDynamic,
}
struct VpclatticeTargetGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VpclatticeTargetGroupData>,
}
#[derive(Clone)]
pub struct VpclatticeTargetGroup(Rc<VpclatticeTargetGroup_>);
impl VpclatticeTargetGroup {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
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
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(self, v: impl Into<BlockAssignable<VpclatticeTargetGroupConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VpclatticeTargetGroupTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<VpclatticeTargetGroupConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpclatticeTargetGroupTimeoutsElRef {
        VpclatticeTargetGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for VpclatticeTargetGroup {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for VpclatticeTargetGroup {}
impl ToListMappable for VpclatticeTargetGroup {
    type O = ListRef<VpclatticeTargetGroupRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for VpclatticeTargetGroup_ {
    fn extract_resource_type(&self) -> String {
        "aws_vpclattice_target_group".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVpclatticeTargetGroup {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildVpclatticeTargetGroup {
    pub fn build(self, stack: &mut Stack) -> VpclatticeTargetGroup {
        let out = VpclatticeTargetGroup(Rc::new(VpclatticeTargetGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VpclatticeTargetGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: self.type_,
                config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct VpclatticeTargetGroupRef {
    shared: StackShared,
    base: String,
}
impl Ref for VpclatticeTargetGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VpclatticeTargetGroupRef {
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<VpclatticeTargetGroupConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpclatticeTargetGroupTimeoutsElRef {
        VpclatticeTargetGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct VpclatticeTargetGroupConfigElHealthCheckElMatcherEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl VpclatticeTargetGroupConfigElHealthCheckElMatcherEl {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for VpclatticeTargetGroupConfigElHealthCheckElMatcherEl {
    type O = BlockAssignable<VpclatticeTargetGroupConfigElHealthCheckElMatcherEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVpclatticeTargetGroupConfigElHealthCheckElMatcherEl {}
impl BuildVpclatticeTargetGroupConfigElHealthCheckElMatcherEl {
    pub fn build(self) -> VpclatticeTargetGroupConfigElHealthCheckElMatcherEl {
        VpclatticeTargetGroupConfigElHealthCheckElMatcherEl {
            value: core::default::Default::default(),
        }
    }
}
pub struct VpclatticeTargetGroupConfigElHealthCheckElMatcherElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VpclatticeTargetGroupConfigElHealthCheckElMatcherElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VpclatticeTargetGroupConfigElHealthCheckElMatcherElRef {
        VpclatticeTargetGroupConfigElHealthCheckElMatcherElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VpclatticeTargetGroupConfigElHealthCheckElMatcherElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct VpclatticeTargetGroupConfigElHealthCheckElDynamic {
    matcher: Option<DynamicBlock<VpclatticeTargetGroupConfigElHealthCheckElMatcherEl>>,
}
#[derive(Serialize)]
pub struct VpclatticeTargetGroupConfigElHealthCheckEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check_interval_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check_timeout_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    healthy_threshold_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unhealthy_threshold_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matcher: Option<Vec<VpclatticeTargetGroupConfigElHealthCheckElMatcherEl>>,
    dynamic: VpclatticeTargetGroupConfigElHealthCheckElDynamic,
}
impl VpclatticeTargetGroupConfigElHealthCheckEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `health_check_interval_seconds`.\n"]
    pub fn set_health_check_interval_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.health_check_interval_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `health_check_timeout_seconds`.\n"]
    pub fn set_health_check_timeout_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.health_check_timeout_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `healthy_threshold_count`.\n"]
    pub fn set_healthy_threshold_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.healthy_threshold_count = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `protocol_version`.\n"]
    pub fn set_protocol_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol_version = Some(v.into());
        self
    }
    #[doc = "Set the field `unhealthy_threshold_count`.\n"]
    pub fn set_unhealthy_threshold_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.unhealthy_threshold_count = Some(v.into());
        self
    }
    #[doc = "Set the field `matcher`.\n"]
    pub fn set_matcher(
        mut self,
        v: impl Into<BlockAssignable<VpclatticeTargetGroupConfigElHealthCheckElMatcherEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.matcher = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.matcher = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VpclatticeTargetGroupConfigElHealthCheckEl {
    type O = BlockAssignable<VpclatticeTargetGroupConfigElHealthCheckEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVpclatticeTargetGroupConfigElHealthCheckEl {}
impl BuildVpclatticeTargetGroupConfigElHealthCheckEl {
    pub fn build(self) -> VpclatticeTargetGroupConfigElHealthCheckEl {
        VpclatticeTargetGroupConfigElHealthCheckEl {
            enabled: core::default::Default::default(),
            health_check_interval_seconds: core::default::Default::default(),
            health_check_timeout_seconds: core::default::Default::default(),
            healthy_threshold_count: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            protocol_version: core::default::Default::default(),
            unhealthy_threshold_count: core::default::Default::default(),
            matcher: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VpclatticeTargetGroupConfigElHealthCheckElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VpclatticeTargetGroupConfigElHealthCheckElRef {
    fn new(shared: StackShared, base: String) -> VpclatticeTargetGroupConfigElHealthCheckElRef {
        VpclatticeTargetGroupConfigElHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VpclatticeTargetGroupConfigElHealthCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `health_check_interval_seconds` after provisioning.\n"]
    pub fn health_check_interval_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_check_interval_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `health_check_timeout_seconds` after provisioning.\n"]
    pub fn health_check_timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_check_timeout_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `healthy_threshold_count` after provisioning.\n"]
    pub fn healthy_threshold_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.healthy_threshold_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
    #[doc = "Get a reference to the value of field `protocol_version` after provisioning.\n"]
    pub fn protocol_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `unhealthy_threshold_count` after provisioning.\n"]
    pub fn unhealthy_threshold_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unhealthy_threshold_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `matcher` after provisioning.\n"]
    pub fn matcher(&self) -> ListRef<VpclatticeTargetGroupConfigElHealthCheckElMatcherElRef> {
        ListRef::new(self.shared().clone(), format!("{}.matcher", self.base))
    }
}
#[derive(Serialize, Default)]
struct VpclatticeTargetGroupConfigElDynamic {
    health_check: Option<DynamicBlock<VpclatticeTargetGroupConfigElHealthCheckEl>>,
}
#[derive(Serialize)]
pub struct VpclatticeTargetGroupConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_event_structure_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check: Option<Vec<VpclatticeTargetGroupConfigElHealthCheckEl>>,
    dynamic: VpclatticeTargetGroupConfigElDynamic,
}
impl VpclatticeTargetGroupConfigEl {
    #[doc = "Set the field `ip_address_type`.\n"]
    pub fn set_ip_address_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address_type = Some(v.into());
        self
    }
    #[doc = "Set the field `lambda_event_structure_version`.\n"]
    pub fn set_lambda_event_structure_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lambda_event_structure_version = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `protocol_version`.\n"]
    pub fn set_protocol_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol_version = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_identifier`.\n"]
    pub fn set_vpc_identifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_identifier = Some(v.into());
        self
    }
    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(
        mut self,
        v: impl Into<BlockAssignable<VpclatticeTargetGroupConfigElHealthCheckEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.health_check = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.health_check = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VpclatticeTargetGroupConfigEl {
    type O = BlockAssignable<VpclatticeTargetGroupConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVpclatticeTargetGroupConfigEl {}
impl BuildVpclatticeTargetGroupConfigEl {
    pub fn build(self) -> VpclatticeTargetGroupConfigEl {
        VpclatticeTargetGroupConfigEl {
            ip_address_type: core::default::Default::default(),
            lambda_event_structure_version: core::default::Default::default(),
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            protocol_version: core::default::Default::default(),
            vpc_identifier: core::default::Default::default(),
            health_check: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VpclatticeTargetGroupConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VpclatticeTargetGroupConfigElRef {
    fn new(shared: StackShared, base: String) -> VpclatticeTargetGroupConfigElRef {
        VpclatticeTargetGroupConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VpclatticeTargetGroupConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lambda_event_structure_version` after provisioning.\n"]
    pub fn lambda_event_structure_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lambda_event_structure_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
    #[doc = "Get a reference to the value of field `protocol_version` after provisioning.\n"]
    pub fn protocol_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_identifier` after provisioning.\n"]
    pub fn vpc_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_identifier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> ListRef<VpclatticeTargetGroupConfigElHealthCheckElRef> {
        ListRef::new(self.shared().clone(), format!("{}.health_check", self.base))
    }
}
#[derive(Serialize)]
pub struct VpclatticeTargetGroupTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl VpclatticeTargetGroupTimeoutsEl {
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
impl ToListMappable for VpclatticeTargetGroupTimeoutsEl {
    type O = BlockAssignable<VpclatticeTargetGroupTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVpclatticeTargetGroupTimeoutsEl {}
impl BuildVpclatticeTargetGroupTimeoutsEl {
    pub fn build(self) -> VpclatticeTargetGroupTimeoutsEl {
        VpclatticeTargetGroupTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct VpclatticeTargetGroupTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VpclatticeTargetGroupTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VpclatticeTargetGroupTimeoutsElRef {
        VpclatticeTargetGroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VpclatticeTargetGroupTimeoutsElRef {
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
struct VpclatticeTargetGroupDynamic {
    config: Option<DynamicBlock<VpclatticeTargetGroupConfigEl>>,
}
