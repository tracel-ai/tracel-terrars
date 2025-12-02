use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ChimeVoiceConnectorStreamingData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    data_retention: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    streaming_notification_targets: Option<SetField<PrimField<String>>>,
    voice_connector_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    media_insights_configuration:
        Option<Vec<ChimeVoiceConnectorStreamingMediaInsightsConfigurationEl>>,
    dynamic: ChimeVoiceConnectorStreamingDynamic,
}
struct ChimeVoiceConnectorStreaming_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ChimeVoiceConnectorStreamingData>,
}
#[derive(Clone)]
pub struct ChimeVoiceConnectorStreaming(Rc<ChimeVoiceConnectorStreaming_>);
impl ChimeVoiceConnectorStreaming {
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
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = Some(v.into());
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
    #[doc = "Set the field `streaming_notification_targets`.\n"]
    pub fn set_streaming_notification_targets(
        self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.0.data.borrow_mut().streaming_notification_targets = Some(v.into());
        self
    }
    #[doc = "Set the field `media_insights_configuration`.\n"]
    pub fn set_media_insights_configuration(
        self,
        v: impl Into<BlockAssignable<ChimeVoiceConnectorStreamingMediaInsightsConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().media_insights_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .media_insights_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `data_retention` after provisioning.\n"]
    pub fn data_retention(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_retention", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `streaming_notification_targets` after provisioning.\n"]
    pub fn streaming_notification_targets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.streaming_notification_targets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `voice_connector_id` after provisioning.\n"]
    pub fn voice_connector_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.voice_connector_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `media_insights_configuration` after provisioning.\n"]
    pub fn media_insights_configuration(
        &self,
    ) -> ListRef<ChimeVoiceConnectorStreamingMediaInsightsConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.media_insights_configuration", self.extract_ref()),
        )
    }
}
impl Referable for ChimeVoiceConnectorStreaming {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ChimeVoiceConnectorStreaming {}
impl ToListMappable for ChimeVoiceConnectorStreaming {
    type O = ListRef<ChimeVoiceConnectorStreamingRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ChimeVoiceConnectorStreaming_ {
    fn extract_resource_type(&self) -> String {
        "aws_chime_voice_connector_streaming".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildChimeVoiceConnectorStreaming {
    pub tf_id: String,
    #[doc = ""]
    pub data_retention: PrimField<f64>,
    #[doc = ""]
    pub voice_connector_id: PrimField<String>,
}
impl BuildChimeVoiceConnectorStreaming {
    pub fn build(self, stack: &mut Stack) -> ChimeVoiceConnectorStreaming {
        let out = ChimeVoiceConnectorStreaming(Rc::new(ChimeVoiceConnectorStreaming_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ChimeVoiceConnectorStreamingData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                data_retention: self.data_retention,
                disabled: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                streaming_notification_targets: core::default::Default::default(),
                voice_connector_id: self.voice_connector_id,
                media_insights_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ChimeVoiceConnectorStreamingRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChimeVoiceConnectorStreamingRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ChimeVoiceConnectorStreamingRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_retention` after provisioning.\n"]
    pub fn data_retention(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_retention", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `streaming_notification_targets` after provisioning.\n"]
    pub fn streaming_notification_targets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.streaming_notification_targets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `voice_connector_id` after provisioning.\n"]
    pub fn voice_connector_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.voice_connector_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `media_insights_configuration` after provisioning.\n"]
    pub fn media_insights_configuration(
        &self,
    ) -> ListRef<ChimeVoiceConnectorStreamingMediaInsightsConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.media_insights_configuration", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ChimeVoiceConnectorStreamingMediaInsightsConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
}
impl ChimeVoiceConnectorStreamingMediaInsightsConfigurationEl {
    #[doc = "Set the field `configuration_arn`.\n"]
    pub fn set_configuration_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.configuration_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}
impl ToListMappable for ChimeVoiceConnectorStreamingMediaInsightsConfigurationEl {
    type O = BlockAssignable<ChimeVoiceConnectorStreamingMediaInsightsConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChimeVoiceConnectorStreamingMediaInsightsConfigurationEl {}
impl BuildChimeVoiceConnectorStreamingMediaInsightsConfigurationEl {
    pub fn build(self) -> ChimeVoiceConnectorStreamingMediaInsightsConfigurationEl {
        ChimeVoiceConnectorStreamingMediaInsightsConfigurationEl {
            configuration_arn: core::default::Default::default(),
            disabled: core::default::Default::default(),
        }
    }
}
pub struct ChimeVoiceConnectorStreamingMediaInsightsConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChimeVoiceConnectorStreamingMediaInsightsConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChimeVoiceConnectorStreamingMediaInsightsConfigurationElRef {
        ChimeVoiceConnectorStreamingMediaInsightsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChimeVoiceConnectorStreamingMediaInsightsConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `configuration_arn` after provisioning.\n"]
    pub fn configuration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}
#[derive(Serialize, Default)]
struct ChimeVoiceConnectorStreamingDynamic {
    media_insights_configuration:
        Option<DynamicBlock<ChimeVoiceConnectorStreamingMediaInsightsConfigurationEl>>,
}
