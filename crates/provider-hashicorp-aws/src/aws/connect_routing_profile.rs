use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ConnectRoutingProfileData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    default_outbound_queue_id: PrimField<String>,
    description: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_id: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    media_concurrencies: Option<Vec<ConnectRoutingProfileMediaConcurrenciesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queue_configs: Option<Vec<ConnectRoutingProfileQueueConfigsEl>>,
    dynamic: ConnectRoutingProfileDynamic,
}
struct ConnectRoutingProfile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ConnectRoutingProfileData>,
}
#[derive(Clone)]
pub struct ConnectRoutingProfile(Rc<ConnectRoutingProfile_>);
impl ConnectRoutingProfile {
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
    #[doc = "Set the field `media_concurrencies`.\n"]
    pub fn set_media_concurrencies(
        self,
        v: impl Into<BlockAssignable<ConnectRoutingProfileMediaConcurrenciesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().media_concurrencies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.media_concurrencies = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `queue_configs`.\n"]
    pub fn set_queue_configs(
        self,
        v: impl Into<BlockAssignable<ConnectRoutingProfileQueueConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().queue_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.queue_configs = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `default_outbound_queue_id` after provisioning.\n"]
    pub fn default_outbound_queue_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_outbound_queue_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `routing_profile_id` after provisioning.\n"]
    pub fn routing_profile_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_profile_id", self.extract_ref()),
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
}
impl Referable for ConnectRoutingProfile {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ConnectRoutingProfile {}
impl ToListMappable for ConnectRoutingProfile {
    type O = ListRef<ConnectRoutingProfileRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ConnectRoutingProfile_ {
    fn extract_resource_type(&self) -> String {
        "aws_connect_routing_profile".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildConnectRoutingProfile {
    pub tf_id: String,
    #[doc = ""]
    pub default_outbound_queue_id: PrimField<String>,
    #[doc = ""]
    pub description: PrimField<String>,
    #[doc = ""]
    pub instance_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildConnectRoutingProfile {
    pub fn build(self, stack: &mut Stack) -> ConnectRoutingProfile {
        let out = ConnectRoutingProfile(Rc::new(ConnectRoutingProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ConnectRoutingProfileData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                default_outbound_queue_id: self.default_outbound_queue_id,
                description: self.description,
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                media_concurrencies: core::default::Default::default(),
                queue_configs: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ConnectRoutingProfileRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConnectRoutingProfileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ConnectRoutingProfileRef {
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
    #[doc = "Get a reference to the value of field `default_outbound_queue_id` after provisioning.\n"]
    pub fn default_outbound_queue_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_outbound_queue_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `routing_profile_id` after provisioning.\n"]
    pub fn routing_profile_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_profile_id", self.extract_ref()),
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
}
#[derive(Serialize)]
pub struct ConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorEl {
    behavior_type: PrimField<String>,
}
impl ConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorEl {}
impl ToListMappable for ConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorEl {
    type O = BlockAssignable<ConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorEl {
    #[doc = ""]
    pub behavior_type: PrimField<String>,
}
impl BuildConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorEl {
    pub fn build(self) -> ConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorEl {
        ConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorEl {
            behavior_type: self.behavior_type,
        }
    }
}
pub struct ConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorElRef {
        ConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `behavior_type` after provisioning.\n"]
    pub fn behavior_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.behavior_type", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ConnectRoutingProfileMediaConcurrenciesElDynamic {
    cross_channel_behavior:
        Option<DynamicBlock<ConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorEl>>,
}
#[derive(Serialize)]
pub struct ConnectRoutingProfileMediaConcurrenciesEl {
    channel: PrimField<String>,
    concurrency: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cross_channel_behavior:
        Option<Vec<ConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorEl>>,
    dynamic: ConnectRoutingProfileMediaConcurrenciesElDynamic,
}
impl ConnectRoutingProfileMediaConcurrenciesEl {
    #[doc = "Set the field `cross_channel_behavior`.\n"]
    pub fn set_cross_channel_behavior(
        mut self,
        v: impl Into<BlockAssignable<ConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cross_channel_behavior = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cross_channel_behavior = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ConnectRoutingProfileMediaConcurrenciesEl {
    type O = BlockAssignable<ConnectRoutingProfileMediaConcurrenciesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConnectRoutingProfileMediaConcurrenciesEl {
    #[doc = ""]
    pub channel: PrimField<String>,
    #[doc = ""]
    pub concurrency: PrimField<f64>,
}
impl BuildConnectRoutingProfileMediaConcurrenciesEl {
    pub fn build(self) -> ConnectRoutingProfileMediaConcurrenciesEl {
        ConnectRoutingProfileMediaConcurrenciesEl {
            channel: self.channel,
            concurrency: self.concurrency,
            cross_channel_behavior: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ConnectRoutingProfileMediaConcurrenciesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConnectRoutingProfileMediaConcurrenciesElRef {
    fn new(shared: StackShared, base: String) -> ConnectRoutingProfileMediaConcurrenciesElRef {
        ConnectRoutingProfileMediaConcurrenciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConnectRoutingProfileMediaConcurrenciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `channel` after provisioning.\n"]
    pub fn channel(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }
    #[doc = "Get a reference to the value of field `concurrency` after provisioning.\n"]
    pub fn concurrency(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.concurrency", self.base))
    }
    #[doc = "Get a reference to the value of field `cross_channel_behavior` after provisioning.\n"]
    pub fn cross_channel_behavior(
        &self,
    ) -> ListRef<ConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cross_channel_behavior", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ConnectRoutingProfileQueueConfigsEl {
    channel: PrimField<String>,
    delay: PrimField<f64>,
    priority: PrimField<f64>,
    queue_id: PrimField<String>,
}
impl ConnectRoutingProfileQueueConfigsEl {}
impl ToListMappable for ConnectRoutingProfileQueueConfigsEl {
    type O = BlockAssignable<ConnectRoutingProfileQueueConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConnectRoutingProfileQueueConfigsEl {
    #[doc = ""]
    pub channel: PrimField<String>,
    #[doc = ""]
    pub delay: PrimField<f64>,
    #[doc = ""]
    pub priority: PrimField<f64>,
    #[doc = ""]
    pub queue_id: PrimField<String>,
}
impl BuildConnectRoutingProfileQueueConfigsEl {
    pub fn build(self) -> ConnectRoutingProfileQueueConfigsEl {
        ConnectRoutingProfileQueueConfigsEl {
            channel: self.channel,
            delay: self.delay,
            priority: self.priority,
            queue_id: self.queue_id,
        }
    }
}
pub struct ConnectRoutingProfileQueueConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConnectRoutingProfileQueueConfigsElRef {
    fn new(shared: StackShared, base: String) -> ConnectRoutingProfileQueueConfigsElRef {
        ConnectRoutingProfileQueueConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConnectRoutingProfileQueueConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `channel` after provisioning.\n"]
    pub fn channel(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }
    #[doc = "Get a reference to the value of field `delay` after provisioning.\n"]
    pub fn delay(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.delay", self.base))
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
    #[doc = "Get a reference to the value of field `queue_arn` after provisioning.\n"]
    pub fn queue_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.queue_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `queue_id` after provisioning.\n"]
    pub fn queue_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.queue_id", self.base))
    }
    #[doc = "Get a reference to the value of field `queue_name` after provisioning.\n"]
    pub fn queue_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.queue_name", self.base))
    }
}
#[derive(Serialize, Default)]
struct ConnectRoutingProfileDynamic {
    media_concurrencies: Option<DynamicBlock<ConnectRoutingProfileMediaConcurrenciesEl>>,
    queue_configs: Option<DynamicBlock<ConnectRoutingProfileQueueConfigsEl>>,
}
