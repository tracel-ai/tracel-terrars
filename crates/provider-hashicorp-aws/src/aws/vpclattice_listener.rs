use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VpclatticeListenerData {
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
    port: Option<PrimField<f64>>,
    protocol: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_action: Option<Vec<VpclatticeListenerDefaultActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VpclatticeListenerTimeoutsEl>,
    dynamic: VpclatticeListenerDynamic,
}
struct VpclatticeListener_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VpclatticeListenerData>,
}
#[derive(Clone)]
pub struct VpclatticeListener(Rc<VpclatticeListener_>);
impl VpclatticeListener {
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
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `service_arn`.\n"]
    pub fn set_service_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `service_identifier`.\n"]
    pub fn set_service_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_identifier = Some(v.into());
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
    #[doc = "Set the field `default_action`.\n"]
    pub fn set_default_action(
        self,
        v: impl Into<BlockAssignable<VpclatticeListenerDefaultActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VpclatticeListenerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_updated_at` after provisioning.\n"]
    pub fn last_updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_updated_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `listener_id` after provisioning.\n"]
    pub fn listener_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.listener_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_arn` after provisioning.\n"]
    pub fn service_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_identifier` after provisioning.\n"]
    pub fn service_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_identifier", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> ListRef<VpclatticeListenerDefaultActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpclatticeListenerTimeoutsElRef {
        VpclatticeListenerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for VpclatticeListener {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for VpclatticeListener {}
impl ToListMappable for VpclatticeListener {
    type O = ListRef<VpclatticeListenerRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for VpclatticeListener_ {
    fn extract_resource_type(&self) -> String {
        "aws_vpclattice_listener".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVpclatticeListener {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub protocol: PrimField<String>,
}
impl BuildVpclatticeListener {
    pub fn build(self, stack: &mut Stack) -> VpclatticeListener {
        let out = VpclatticeListener(Rc::new(VpclatticeListener_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VpclatticeListenerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                port: core::default::Default::default(),
                protocol: self.protocol,
                region: core::default::Default::default(),
                service_arn: core::default::Default::default(),
                service_identifier: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                default_action: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct VpclatticeListenerRef {
    shared: StackShared,
    base: String,
}
impl Ref for VpclatticeListenerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VpclatticeListenerRef {
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
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_updated_at` after provisioning.\n"]
    pub fn last_updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_updated_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `listener_id` after provisioning.\n"]
    pub fn listener_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.listener_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_arn` after provisioning.\n"]
    pub fn service_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_identifier` after provisioning.\n"]
    pub fn service_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_identifier", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> ListRef<VpclatticeListenerDefaultActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpclatticeListenerTimeoutsElRef {
        VpclatticeListenerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct VpclatticeListenerDefaultActionElFixedResponseEl {
    status_code: PrimField<f64>,
}
impl VpclatticeListenerDefaultActionElFixedResponseEl {}
impl ToListMappable for VpclatticeListenerDefaultActionElFixedResponseEl {
    type O = BlockAssignable<VpclatticeListenerDefaultActionElFixedResponseEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVpclatticeListenerDefaultActionElFixedResponseEl {
    #[doc = ""]
    pub status_code: PrimField<f64>,
}
impl BuildVpclatticeListenerDefaultActionElFixedResponseEl {
    pub fn build(self) -> VpclatticeListenerDefaultActionElFixedResponseEl {
        VpclatticeListenerDefaultActionElFixedResponseEl {
            status_code: self.status_code,
        }
    }
}
pub struct VpclatticeListenerDefaultActionElFixedResponseElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VpclatticeListenerDefaultActionElFixedResponseElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VpclatticeListenerDefaultActionElFixedResponseElRef {
        VpclatticeListenerDefaultActionElFixedResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VpclatticeListenerDefaultActionElFixedResponseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}
#[derive(Serialize)]
pub struct VpclatticeListenerDefaultActionElForwardElTargetGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target_group_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}
impl VpclatticeListenerDefaultActionElForwardElTargetGroupsEl {
    #[doc = "Set the field `target_group_identifier`.\n"]
    pub fn set_target_group_identifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_group_identifier = Some(v.into());
        self
    }
    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}
impl ToListMappable for VpclatticeListenerDefaultActionElForwardElTargetGroupsEl {
    type O = BlockAssignable<VpclatticeListenerDefaultActionElForwardElTargetGroupsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVpclatticeListenerDefaultActionElForwardElTargetGroupsEl {}
impl BuildVpclatticeListenerDefaultActionElForwardElTargetGroupsEl {
    pub fn build(self) -> VpclatticeListenerDefaultActionElForwardElTargetGroupsEl {
        VpclatticeListenerDefaultActionElForwardElTargetGroupsEl {
            target_group_identifier: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}
pub struct VpclatticeListenerDefaultActionElForwardElTargetGroupsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VpclatticeListenerDefaultActionElForwardElTargetGroupsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VpclatticeListenerDefaultActionElForwardElTargetGroupsElRef {
        VpclatticeListenerDefaultActionElForwardElTargetGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VpclatticeListenerDefaultActionElForwardElTargetGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target_group_identifier` after provisioning.\n"]
    pub fn target_group_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_group_identifier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}
#[derive(Serialize, Default)]
struct VpclatticeListenerDefaultActionElForwardElDynamic {
    target_groups: Option<DynamicBlock<VpclatticeListenerDefaultActionElForwardElTargetGroupsEl>>,
}
#[derive(Serialize)]
pub struct VpclatticeListenerDefaultActionElForwardEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target_groups: Option<Vec<VpclatticeListenerDefaultActionElForwardElTargetGroupsEl>>,
    dynamic: VpclatticeListenerDefaultActionElForwardElDynamic,
}
impl VpclatticeListenerDefaultActionElForwardEl {
    #[doc = "Set the field `target_groups`.\n"]
    pub fn set_target_groups(
        mut self,
        v: impl Into<BlockAssignable<VpclatticeListenerDefaultActionElForwardElTargetGroupsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target_groups = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target_groups = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VpclatticeListenerDefaultActionElForwardEl {
    type O = BlockAssignable<VpclatticeListenerDefaultActionElForwardEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVpclatticeListenerDefaultActionElForwardEl {}
impl BuildVpclatticeListenerDefaultActionElForwardEl {
    pub fn build(self) -> VpclatticeListenerDefaultActionElForwardEl {
        VpclatticeListenerDefaultActionElForwardEl {
            target_groups: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VpclatticeListenerDefaultActionElForwardElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VpclatticeListenerDefaultActionElForwardElRef {
    fn new(shared: StackShared, base: String) -> VpclatticeListenerDefaultActionElForwardElRef {
        VpclatticeListenerDefaultActionElForwardElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VpclatticeListenerDefaultActionElForwardElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target_groups` after provisioning.\n"]
    pub fn target_groups(
        &self,
    ) -> ListRef<VpclatticeListenerDefaultActionElForwardElTargetGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_groups", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct VpclatticeListenerDefaultActionElDynamic {
    fixed_response: Option<DynamicBlock<VpclatticeListenerDefaultActionElFixedResponseEl>>,
    forward: Option<DynamicBlock<VpclatticeListenerDefaultActionElForwardEl>>,
}
#[derive(Serialize)]
pub struct VpclatticeListenerDefaultActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_response: Option<Vec<VpclatticeListenerDefaultActionElFixedResponseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forward: Option<Vec<VpclatticeListenerDefaultActionElForwardEl>>,
    dynamic: VpclatticeListenerDefaultActionElDynamic,
}
impl VpclatticeListenerDefaultActionEl {
    #[doc = "Set the field `fixed_response`.\n"]
    pub fn set_fixed_response(
        mut self,
        v: impl Into<BlockAssignable<VpclatticeListenerDefaultActionElFixedResponseEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fixed_response = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_response = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `forward`.\n"]
    pub fn set_forward(
        mut self,
        v: impl Into<BlockAssignable<VpclatticeListenerDefaultActionElForwardEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.forward = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.forward = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VpclatticeListenerDefaultActionEl {
    type O = BlockAssignable<VpclatticeListenerDefaultActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVpclatticeListenerDefaultActionEl {}
impl BuildVpclatticeListenerDefaultActionEl {
    pub fn build(self) -> VpclatticeListenerDefaultActionEl {
        VpclatticeListenerDefaultActionEl {
            fixed_response: core::default::Default::default(),
            forward: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VpclatticeListenerDefaultActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VpclatticeListenerDefaultActionElRef {
    fn new(shared: StackShared, base: String) -> VpclatticeListenerDefaultActionElRef {
        VpclatticeListenerDefaultActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VpclatticeListenerDefaultActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `fixed_response` after provisioning.\n"]
    pub fn fixed_response(&self) -> ListRef<VpclatticeListenerDefaultActionElFixedResponseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fixed_response", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `forward` after provisioning.\n"]
    pub fn forward(&self) -> ListRef<VpclatticeListenerDefaultActionElForwardElRef> {
        ListRef::new(self.shared().clone(), format!("{}.forward", self.base))
    }
}
#[derive(Serialize)]
pub struct VpclatticeListenerTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl VpclatticeListenerTimeoutsEl {
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
impl ToListMappable for VpclatticeListenerTimeoutsEl {
    type O = BlockAssignable<VpclatticeListenerTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVpclatticeListenerTimeoutsEl {}
impl BuildVpclatticeListenerTimeoutsEl {
    pub fn build(self) -> VpclatticeListenerTimeoutsEl {
        VpclatticeListenerTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct VpclatticeListenerTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VpclatticeListenerTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VpclatticeListenerTimeoutsElRef {
        VpclatticeListenerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VpclatticeListenerTimeoutsElRef {
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
struct VpclatticeListenerDynamic {
    default_action: Option<DynamicBlock<VpclatticeListenerDefaultActionEl>>,
}
