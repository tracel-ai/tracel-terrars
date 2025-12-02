use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataConnectRoutingProfileData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_profile_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}
struct DataConnectRoutingProfile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataConnectRoutingProfileData>,
}
#[derive(Clone)]
pub struct DataConnectRoutingProfile(Rc<DataConnectRoutingProfile_>);
impl DataConnectRoutingProfile {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `routing_profile_id`.\n"]
    pub fn set_routing_profile_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().routing_profile_id = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
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
    #[doc = "Get a reference to the value of field `media_concurrencies` after provisioning.\n"]
    pub fn media_concurrencies(&self) -> SetRef<DataConnectRoutingProfileMediaConcurrenciesElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.media_concurrencies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `queue_configs` after provisioning.\n"]
    pub fn queue_configs(&self) -> SetRef<DataConnectRoutingProfileQueueConfigsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.queue_configs", self.extract_ref()),
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
}
impl Referable for DataConnectRoutingProfile {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataConnectRoutingProfile {}
impl ToListMappable for DataConnectRoutingProfile {
    type O = ListRef<DataConnectRoutingProfileRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataConnectRoutingProfile_ {
    fn extract_datasource_type(&self) -> String {
        "aws_connect_routing_profile".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataConnectRoutingProfile {
    pub tf_id: String,
    #[doc = ""]
    pub instance_id: PrimField<String>,
}
impl BuildDataConnectRoutingProfile {
    pub fn build(self, stack: &mut Stack) -> DataConnectRoutingProfile {
        let out = DataConnectRoutingProfile(Rc::new(DataConnectRoutingProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataConnectRoutingProfileData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                routing_profile_id: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataConnectRoutingProfileRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataConnectRoutingProfileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataConnectRoutingProfileRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
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
    #[doc = "Get a reference to the value of field `media_concurrencies` after provisioning.\n"]
    pub fn media_concurrencies(&self) -> SetRef<DataConnectRoutingProfileMediaConcurrenciesElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.media_concurrencies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `queue_configs` after provisioning.\n"]
    pub fn queue_configs(&self) -> SetRef<DataConnectRoutingProfileQueueConfigsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.queue_configs", self.extract_ref()),
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
}
#[derive(Serialize)]
pub struct DataConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    behavior_type: Option<PrimField<String>>,
}
impl DataConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorEl {
    #[doc = "Set the field `behavior_type`.\n"]
    pub fn set_behavior_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.behavior_type = Some(v.into());
        self
    }
}
impl ToListMappable for DataConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorEl {
    type O = BlockAssignable<DataConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorEl {}
impl BuildDataConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorEl {
    pub fn build(self) -> DataConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorEl {
        DataConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorEl {
            behavior_type: core::default::Default::default(),
        }
    }
}
pub struct DataConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorElRef {
        DataConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorElRef {
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
#[derive(Serialize)]
pub struct DataConnectRoutingProfileMediaConcurrenciesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    concurrency: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cross_channel_behavior:
        Option<ListField<DataConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorEl>>,
}
impl DataConnectRoutingProfileMediaConcurrenciesEl {
    #[doc = "Set the field `channel`.\n"]
    pub fn set_channel(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.channel = Some(v.into());
        self
    }
    #[doc = "Set the field `concurrency`.\n"]
    pub fn set_concurrency(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.concurrency = Some(v.into());
        self
    }
    #[doc = "Set the field `cross_channel_behavior`.\n"]
    pub fn set_cross_channel_behavior(
        mut self,
        v: impl Into<ListField<DataConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorEl>>,
    ) -> Self {
        self.cross_channel_behavior = Some(v.into());
        self
    }
}
impl ToListMappable for DataConnectRoutingProfileMediaConcurrenciesEl {
    type O = BlockAssignable<DataConnectRoutingProfileMediaConcurrenciesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataConnectRoutingProfileMediaConcurrenciesEl {}
impl BuildDataConnectRoutingProfileMediaConcurrenciesEl {
    pub fn build(self) -> DataConnectRoutingProfileMediaConcurrenciesEl {
        DataConnectRoutingProfileMediaConcurrenciesEl {
            channel: core::default::Default::default(),
            concurrency: core::default::Default::default(),
            cross_channel_behavior: core::default::Default::default(),
        }
    }
}
pub struct DataConnectRoutingProfileMediaConcurrenciesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataConnectRoutingProfileMediaConcurrenciesElRef {
    fn new(shared: StackShared, base: String) -> DataConnectRoutingProfileMediaConcurrenciesElRef {
        DataConnectRoutingProfileMediaConcurrenciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataConnectRoutingProfileMediaConcurrenciesElRef {
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
    ) -> ListRef<DataConnectRoutingProfileMediaConcurrenciesElCrossChannelBehaviorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cross_channel_behavior", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataConnectRoutingProfileQueueConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delay: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queue_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queue_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queue_name: Option<PrimField<String>>,
}
impl DataConnectRoutingProfileQueueConfigsEl {
    #[doc = "Set the field `channel`.\n"]
    pub fn set_channel(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.channel = Some(v.into());
        self
    }
    #[doc = "Set the field `delay`.\n"]
    pub fn set_delay(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.delay = Some(v.into());
        self
    }
    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }
    #[doc = "Set the field `queue_arn`.\n"]
    pub fn set_queue_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.queue_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `queue_id`.\n"]
    pub fn set_queue_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.queue_id = Some(v.into());
        self
    }
    #[doc = "Set the field `queue_name`.\n"]
    pub fn set_queue_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.queue_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataConnectRoutingProfileQueueConfigsEl {
    type O = BlockAssignable<DataConnectRoutingProfileQueueConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataConnectRoutingProfileQueueConfigsEl {}
impl BuildDataConnectRoutingProfileQueueConfigsEl {
    pub fn build(self) -> DataConnectRoutingProfileQueueConfigsEl {
        DataConnectRoutingProfileQueueConfigsEl {
            channel: core::default::Default::default(),
            delay: core::default::Default::default(),
            priority: core::default::Default::default(),
            queue_arn: core::default::Default::default(),
            queue_id: core::default::Default::default(),
            queue_name: core::default::Default::default(),
        }
    }
}
pub struct DataConnectRoutingProfileQueueConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataConnectRoutingProfileQueueConfigsElRef {
    fn new(shared: StackShared, base: String) -> DataConnectRoutingProfileQueueConfigsElRef {
        DataConnectRoutingProfileQueueConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataConnectRoutingProfileQueueConfigsElRef {
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
