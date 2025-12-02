use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct GameliftGameSessionQueueData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_event_data: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destinations: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_target: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    player_latency_policy: Option<Vec<GameliftGameSessionQueuePlayerLatencyPolicyEl>>,
    dynamic: GameliftGameSessionQueueDynamic,
}
struct GameliftGameSessionQueue_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GameliftGameSessionQueueData>,
}
#[derive(Clone)]
pub struct GameliftGameSessionQueue(Rc<GameliftGameSessionQueue_>);
impl GameliftGameSessionQueue {
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
    #[doc = "Set the field `custom_event_data`.\n"]
    pub fn set_custom_event_data(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().custom_event_data = Some(v.into());
        self
    }
    #[doc = "Set the field `destinations`.\n"]
    pub fn set_destinations(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().destinations = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `notification_target`.\n"]
    pub fn set_notification_target(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().notification_target = Some(v.into());
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
    #[doc = "Set the field `timeout_in_seconds`.\n"]
    pub fn set_timeout_in_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().timeout_in_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `player_latency_policy`.\n"]
    pub fn set_player_latency_policy(
        self,
        v: impl Into<BlockAssignable<GameliftGameSessionQueuePlayerLatencyPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().player_latency_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.player_latency_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `custom_event_data` after provisioning.\n"]
    pub fn custom_event_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_event_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destinations` after provisioning.\n"]
    pub fn destinations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destinations", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `notification_target` after provisioning.\n"]
    pub fn notification_target(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notification_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `timeout_in_seconds` after provisioning.\n"]
    pub fn timeout_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_in_seconds", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `player_latency_policy` after provisioning.\n"]
    pub fn player_latency_policy(
        &self,
    ) -> ListRef<GameliftGameSessionQueuePlayerLatencyPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.player_latency_policy", self.extract_ref()),
        )
    }
}
impl Referable for GameliftGameSessionQueue {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for GameliftGameSessionQueue {}
impl ToListMappable for GameliftGameSessionQueue {
    type O = ListRef<GameliftGameSessionQueueRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for GameliftGameSessionQueue_ {
    fn extract_resource_type(&self) -> String {
        "aws_gamelift_game_session_queue".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildGameliftGameSessionQueue {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildGameliftGameSessionQueue {
    pub fn build(self, stack: &mut Stack) -> GameliftGameSessionQueue {
        let out = GameliftGameSessionQueue(Rc::new(GameliftGameSessionQueue_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GameliftGameSessionQueueData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                custom_event_data: core::default::Default::default(),
                destinations: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                notification_target: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                timeout_in_seconds: core::default::Default::default(),
                player_latency_policy: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct GameliftGameSessionQueueRef {
    shared: StackShared,
    base: String,
}
impl Ref for GameliftGameSessionQueueRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl GameliftGameSessionQueueRef {
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
    #[doc = "Get a reference to the value of field `custom_event_data` after provisioning.\n"]
    pub fn custom_event_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_event_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destinations` after provisioning.\n"]
    pub fn destinations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destinations", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `notification_target` after provisioning.\n"]
    pub fn notification_target(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notification_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `timeout_in_seconds` after provisioning.\n"]
    pub fn timeout_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_in_seconds", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `player_latency_policy` after provisioning.\n"]
    pub fn player_latency_policy(
        &self,
    ) -> ListRef<GameliftGameSessionQueuePlayerLatencyPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.player_latency_policy", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct GameliftGameSessionQueuePlayerLatencyPolicyEl {
    maximum_individual_player_latency_milliseconds: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_duration_seconds: Option<PrimField<f64>>,
}
impl GameliftGameSessionQueuePlayerLatencyPolicyEl {
    #[doc = "Set the field `policy_duration_seconds`.\n"]
    pub fn set_policy_duration_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.policy_duration_seconds = Some(v.into());
        self
    }
}
impl ToListMappable for GameliftGameSessionQueuePlayerLatencyPolicyEl {
    type O = BlockAssignable<GameliftGameSessionQueuePlayerLatencyPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGameliftGameSessionQueuePlayerLatencyPolicyEl {
    #[doc = ""]
    pub maximum_individual_player_latency_milliseconds: PrimField<f64>,
}
impl BuildGameliftGameSessionQueuePlayerLatencyPolicyEl {
    pub fn build(self) -> GameliftGameSessionQueuePlayerLatencyPolicyEl {
        GameliftGameSessionQueuePlayerLatencyPolicyEl {
            maximum_individual_player_latency_milliseconds: self
                .maximum_individual_player_latency_milliseconds,
            policy_duration_seconds: core::default::Default::default(),
        }
    }
}
pub struct GameliftGameSessionQueuePlayerLatencyPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GameliftGameSessionQueuePlayerLatencyPolicyElRef {
    fn new(shared: StackShared, base: String) -> GameliftGameSessionQueuePlayerLatencyPolicyElRef {
        GameliftGameSessionQueuePlayerLatencyPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GameliftGameSessionQueuePlayerLatencyPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `maximum_individual_player_latency_milliseconds` after provisioning.\n"]
    pub fn maximum_individual_player_latency_milliseconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.maximum_individual_player_latency_milliseconds",
                self.base
            ),
        )
    }
    #[doc = "Get a reference to the value of field `policy_duration_seconds` after provisioning.\n"]
    pub fn policy_duration_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_duration_seconds", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct GameliftGameSessionQueueDynamic {
    player_latency_policy: Option<DynamicBlock<GameliftGameSessionQueuePlayerLatencyPolicyEl>>,
}
