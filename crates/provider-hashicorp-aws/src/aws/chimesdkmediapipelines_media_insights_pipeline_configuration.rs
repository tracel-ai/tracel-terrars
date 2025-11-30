use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    resource_access_role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elements: Option<Vec<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    real_time_alert_configuration: Option<
        Vec<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationTimeoutsEl>,
    dynamic: ChimesdkmediapipelinesMediaInsightsPipelineConfigurationDynamic,
}

struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationData>,
}

#[derive(Clone)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration(
    Rc<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_>,
);

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration {
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

    #[doc = "Set the field `elements`.\n"]
    pub fn set_elements(
        self,
        v: impl Into<
            BlockAssignable<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().elements = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.elements = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `real_time_alert_configuration`.\n"]
    pub fn set_real_time_alert_configuration(
        self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().real_time_alert_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .real_time_alert_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationTimeoutsEl>,
    ) -> Self {
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

    #[doc = "Get a reference to the value of field `resource_access_role_arn` after provisioning.\n"]
    pub fn resource_access_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_access_role_arn", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `elements` after provisioning.\n"]
    pub fn elements(
        &self,
    ) -> ListRef<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.elements", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `real_time_alert_configuration` after provisioning.\n"]
    pub fn real_time_alert_configuration(
        &self,
    ) -> ListRef<
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.real_time_alert_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(
        &self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationTimeoutsElRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration {}

impl ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration {
    type O = ListRef<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_chimesdkmediapipelines_media_insights_pipeline_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub resource_access_role_arn: PrimField<String>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration {
    pub fn build(
        self,
        stack: &mut Stack,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration {
        let out = ChimesdkmediapipelinesMediaInsightsPipelineConfiguration(Rc::new(
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(
                    ChimesdkmediapipelinesMediaInsightsPipelineConfigurationData {
                        depends_on: core::default::Default::default(),
                        provider: None,
                        lifecycle: core::default::Default::default(),
                        for_each: None,
                        name: self.name,
                        region: core::default::Default::default(),
                        resource_access_role_arn: self.resource_access_role_arn,
                        tags: core::default::Default::default(),
                        tags_all: core::default::Default::default(),
                        elements: core::default::Default::default(),
                        real_time_alert_configuration: core::default::Default::default(),
                        timeouts: core::default::Default::default(),
                        dynamic: Default::default(),
                    },
                ),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRef {
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

    #[doc = "Get a reference to the value of field `resource_access_role_arn` after provisioning.\n"]
    pub fn resource_access_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_access_role_arn", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `elements` after provisioning.\n"]
    pub fn elements(
        &self,
    ) -> ListRef<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.elements", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `real_time_alert_configuration` after provisioning.\n"]
    pub fn real_time_alert_configuration(
        &self,
    ) -> ListRef<
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.real_time_alert_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(
        &self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationTimeoutsElRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationElPostCallAnalyticsSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    content_redaction_output: Option<PrimField<String>>,
    data_access_role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_encryption_kms_key_id: Option<PrimField<String>>,
    output_location: PrimField<String>,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationElPostCallAnalyticsSettingsEl {
    #[doc = "Set the field `content_redaction_output`.\n"]
    pub fn set_content_redaction_output(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_redaction_output = Some(v.into());
        self
    }

    #[doc = "Set the field `output_encryption_kms_key_id`.\n"]
    pub fn set_output_encryption_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_encryption_kms_key_id = Some(v.into());
        self
    }
}

impl ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationElPostCallAnalyticsSettingsEl {
    type O =
        BlockAssignable<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationElPostCallAnalyticsSettingsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationElPostCallAnalyticsSettingsEl
{
    #[doc = ""]
    pub data_access_role_arn: PrimField<String>,
    #[doc = ""]
    pub output_location: PrimField<String>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationElPostCallAnalyticsSettingsEl {
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationElPostCallAnalyticsSettingsEl {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationElPostCallAnalyticsSettingsEl {
            content_redaction_output: core::default::Default::default(),
            data_access_role_arn: self.data_access_role_arn,
            output_encryption_kms_key_id: core::default::Default::default(),
            output_location: self.output_location,
        }
    }
}

pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationElPostCallAnalyticsSettingsElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationElPostCallAnalyticsSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationElPostCallAnalyticsSettingsElRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationElPostCallAnalyticsSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationElPostCallAnalyticsSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_redaction_output` after provisioning.\n"]
    pub fn content_redaction_output(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_redaction_output", self.base))
    }

    #[doc = "Get a reference to the value of field `data_access_role_arn` after provisioning.\n"]
    pub fn data_access_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_access_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `output_encryption_kms_key_id` after provisioning.\n"]
    pub fn output_encryption_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_encryption_kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `output_location` after provisioning.\n"]
    pub fn output_location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_location", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationElDynamic {
    post_call_analytics_settings: Option<
        DynamicBlock<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationElPostCallAnalyticsSettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    call_analytics_stream_categories: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_identification_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_redaction_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_partial_results_stabilization: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_partial_results: Option<PrimField<bool>>,
    language_code: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_model_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partial_results_stability: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pii_entity_types: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vocabulary_filter_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vocabulary_filter_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vocabulary_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_call_analytics_settings: Option<
        Vec<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationElPostCallAnalyticsSettingsEl,
        >,
    >,
    dynamic: ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationElDynamic,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationEl {
    #[doc = "Set the field `call_analytics_stream_categories`.\n"]
    pub fn set_call_analytics_stream_categories(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.call_analytics_stream_categories = Some(v.into());
        self
    }

    #[doc = "Set the field `content_identification_type`.\n"]
    pub fn set_content_identification_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_identification_type = Some(v.into());
        self
    }

    #[doc = "Set the field `content_redaction_type`.\n"]
    pub fn set_content_redaction_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_redaction_type = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_partial_results_stabilization`.\n"]
    pub fn set_enable_partial_results_stabilization(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_partial_results_stabilization = Some(v.into());
        self
    }

    #[doc = "Set the field `filter_partial_results`.\n"]
    pub fn set_filter_partial_results(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.filter_partial_results = Some(v.into());
        self
    }

    #[doc = "Set the field `language_model_name`.\n"]
    pub fn set_language_model_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.language_model_name = Some(v.into());
        self
    }

    #[doc = "Set the field `partial_results_stability`.\n"]
    pub fn set_partial_results_stability(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.partial_results_stability = Some(v.into());
        self
    }

    #[doc = "Set the field `pii_entity_types`.\n"]
    pub fn set_pii_entity_types(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pii_entity_types = Some(v.into());
        self
    }

    #[doc = "Set the field `vocabulary_filter_method`.\n"]
    pub fn set_vocabulary_filter_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vocabulary_filter_method = Some(v.into());
        self
    }

    #[doc = "Set the field `vocabulary_filter_name`.\n"]
    pub fn set_vocabulary_filter_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vocabulary_filter_name = Some(v.into());
        self
    }

    #[doc = "Set the field `vocabulary_name`.\n"]
    pub fn set_vocabulary_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vocabulary_name = Some(v.into());
        self
    }

    #[doc = "Set the field `post_call_analytics_settings`.\n"]
    pub fn set_post_call_analytics_settings(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationElPostCallAnalyticsSettingsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.post_call_analytics_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.post_call_analytics_settings = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationEl {
    type O =
        BlockAssignable<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationEl
{
    #[doc = ""]
    pub language_code: PrimField<String>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationEl {
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationEl {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationEl {
            call_analytics_stream_categories: core::default::Default::default(),
            content_identification_type: core::default::Default::default(),
            content_redaction_type: core::default::Default::default(),
            enable_partial_results_stabilization: core::default::Default::default(),
            filter_partial_results: core::default::Default::default(),
            language_code: self.language_code,
            language_model_name: core::default::Default::default(),
            partial_results_stability: core::default::Default::default(),
            pii_entity_types: core::default::Default::default(),
            vocabulary_filter_method: core::default::Default::default(),
            vocabulary_filter_name: core::default::Default::default(),
            vocabulary_name: core::default::Default::default(),
            post_call_analytics_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationElRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `call_analytics_stream_categories` after provisioning.\n"]
    pub fn call_analytics_stream_categories(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.call_analytics_stream_categories", self.base))
    }

    #[doc = "Get a reference to the value of field `content_identification_type` after provisioning.\n"]
    pub fn content_identification_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_identification_type", self.base))
    }

    #[doc = "Get a reference to the value of field `content_redaction_type` after provisioning.\n"]
    pub fn content_redaction_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_redaction_type", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_partial_results_stabilization` after provisioning.\n"]
    pub fn enable_partial_results_stabilization(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_partial_results_stabilization", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_partial_results` after provisioning.\n"]
    pub fn filter_partial_results(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter_partial_results", self.base))
    }

    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.base))
    }

    #[doc = "Get a reference to the value of field `language_model_name` after provisioning.\n"]
    pub fn language_model_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.language_model_name", self.base))
    }

    #[doc = "Get a reference to the value of field `partial_results_stability` after provisioning.\n"]
    pub fn partial_results_stability(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.partial_results_stability", self.base))
    }

    #[doc = "Get a reference to the value of field `pii_entity_types` after provisioning.\n"]
    pub fn pii_entity_types(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pii_entity_types", self.base))
    }

    #[doc = "Get a reference to the value of field `vocabulary_filter_method` after provisioning.\n"]
    pub fn vocabulary_filter_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vocabulary_filter_method", self.base))
    }

    #[doc = "Get a reference to the value of field `vocabulary_filter_name` after provisioning.\n"]
    pub fn vocabulary_filter_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vocabulary_filter_name", self.base))
    }

    #[doc = "Get a reference to the value of field `vocabulary_name` after provisioning.\n"]
    pub fn vocabulary_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vocabulary_name", self.base))
    }

    #[doc = "Get a reference to the value of field `post_call_analytics_settings` after provisioning.\n"]
    pub fn post_call_analytics_settings(
        &self,
    ) -> ListRef<
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationElPostCallAnalyticsSettingsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.post_call_analytics_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeProcessorConfigurationEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    content_identification_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_redaction_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_partial_results_stabilization: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_partial_results: Option<PrimField<bool>>,
    language_code: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_model_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partial_results_stability: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pii_entity_types: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_speaker_label: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vocabulary_filter_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vocabulary_filter_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vocabulary_name: Option<PrimField<String>>,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeProcessorConfigurationEl {
    #[doc = "Set the field `content_identification_type`.\n"]
    pub fn set_content_identification_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_identification_type = Some(v.into());
        self
    }

    #[doc = "Set the field `content_redaction_type`.\n"]
    pub fn set_content_redaction_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_redaction_type = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_partial_results_stabilization`.\n"]
    pub fn set_enable_partial_results_stabilization(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_partial_results_stabilization = Some(v.into());
        self
    }

    #[doc = "Set the field `filter_partial_results`.\n"]
    pub fn set_filter_partial_results(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.filter_partial_results = Some(v.into());
        self
    }

    #[doc = "Set the field `language_model_name`.\n"]
    pub fn set_language_model_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.language_model_name = Some(v.into());
        self
    }

    #[doc = "Set the field `partial_results_stability`.\n"]
    pub fn set_partial_results_stability(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.partial_results_stability = Some(v.into());
        self
    }

    #[doc = "Set the field `pii_entity_types`.\n"]
    pub fn set_pii_entity_types(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pii_entity_types = Some(v.into());
        self
    }

    #[doc = "Set the field `show_speaker_label`.\n"]
    pub fn set_show_speaker_label(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.show_speaker_label = Some(v.into());
        self
    }

    #[doc = "Set the field `vocabulary_filter_method`.\n"]
    pub fn set_vocabulary_filter_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vocabulary_filter_method = Some(v.into());
        self
    }

    #[doc = "Set the field `vocabulary_filter_name`.\n"]
    pub fn set_vocabulary_filter_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vocabulary_filter_name = Some(v.into());
        self
    }

    #[doc = "Set the field `vocabulary_name`.\n"]
    pub fn set_vocabulary_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vocabulary_name = Some(v.into());
        self
    }
}

impl ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeProcessorConfigurationEl {
    type O =
        BlockAssignable<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeProcessorConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeProcessorConfigurationEl
{
    #[doc = ""]
    pub language_code: PrimField<String>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeProcessorConfigurationEl {
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeProcessorConfigurationEl {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeProcessorConfigurationEl {
            content_identification_type: core::default::Default::default(),
            content_redaction_type: core::default::Default::default(),
            enable_partial_results_stabilization: core::default::Default::default(),
            filter_partial_results: core::default::Default::default(),
            language_code: self.language_code,
            language_model_name: core::default::Default::default(),
            partial_results_stability: core::default::Default::default(),
            pii_entity_types: core::default::Default::default(),
            show_speaker_label: core::default::Default::default(),
            vocabulary_filter_method: core::default::Default::default(),
            vocabulary_filter_name: core::default::Default::default(),
            vocabulary_name: core::default::Default::default(),
        }
    }
}

pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeProcessorConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeProcessorConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeProcessorConfigurationElRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeProcessorConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeProcessorConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_identification_type` after provisioning.\n"]
    pub fn content_identification_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_identification_type", self.base))
    }

    #[doc = "Get a reference to the value of field `content_redaction_type` after provisioning.\n"]
    pub fn content_redaction_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_redaction_type", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_partial_results_stabilization` after provisioning.\n"]
    pub fn enable_partial_results_stabilization(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_partial_results_stabilization", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_partial_results` after provisioning.\n"]
    pub fn filter_partial_results(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter_partial_results", self.base))
    }

    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.base))
    }

    #[doc = "Get a reference to the value of field `language_model_name` after provisioning.\n"]
    pub fn language_model_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.language_model_name", self.base))
    }

    #[doc = "Get a reference to the value of field `partial_results_stability` after provisioning.\n"]
    pub fn partial_results_stability(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.partial_results_stability", self.base))
    }

    #[doc = "Get a reference to the value of field `pii_entity_types` after provisioning.\n"]
    pub fn pii_entity_types(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pii_entity_types", self.base))
    }

    #[doc = "Get a reference to the value of field `show_speaker_label` after provisioning.\n"]
    pub fn show_speaker_label(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.show_speaker_label", self.base))
    }

    #[doc = "Get a reference to the value of field `vocabulary_filter_method` after provisioning.\n"]
    pub fn vocabulary_filter_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vocabulary_filter_method", self.base))
    }

    #[doc = "Get a reference to the value of field `vocabulary_filter_name` after provisioning.\n"]
    pub fn vocabulary_filter_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vocabulary_filter_name", self.base))
    }

    #[doc = "Get a reference to the value of field `vocabulary_name` after provisioning.\n"]
    pub fn vocabulary_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vocabulary_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElKinesisDataStreamSinkConfigurationEl
{
    insights_target: PrimField<String>,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElKinesisDataStreamSinkConfigurationEl { }

impl ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElKinesisDataStreamSinkConfigurationEl {
    type O =
        BlockAssignable<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElKinesisDataStreamSinkConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElKinesisDataStreamSinkConfigurationEl
{
    #[doc = ""]
    pub insights_target: PrimField<String>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElKinesisDataStreamSinkConfigurationEl {
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElKinesisDataStreamSinkConfigurationEl {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElKinesisDataStreamSinkConfigurationEl {
            insights_target: self.insights_target,
        }
    }
}

pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElKinesisDataStreamSinkConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElKinesisDataStreamSinkConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElKinesisDataStreamSinkConfigurationElRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElKinesisDataStreamSinkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElKinesisDataStreamSinkConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `insights_target` after provisioning.\n"]
    pub fn insights_target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.insights_target", self.base))
    }
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElLambdaFunctionSinkConfigurationEl
{
    insights_target: PrimField<String>,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElLambdaFunctionSinkConfigurationEl { }

impl ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElLambdaFunctionSinkConfigurationEl {
    type O =
        BlockAssignable<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElLambdaFunctionSinkConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElLambdaFunctionSinkConfigurationEl
{
    #[doc = ""]
    pub insights_target: PrimField<String>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElLambdaFunctionSinkConfigurationEl {
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElLambdaFunctionSinkConfigurationEl {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElLambdaFunctionSinkConfigurationEl {
            insights_target: self.insights_target,
        }
    }
}

pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElLambdaFunctionSinkConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElLambdaFunctionSinkConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElLambdaFunctionSinkConfigurationElRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElLambdaFunctionSinkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElLambdaFunctionSinkConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `insights_target` after provisioning.\n"]
    pub fn insights_target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.insights_target", self.base))
    }
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElS3RecordingSinkConfigurationEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    destination: Option<PrimField<String>>,
}

impl
    ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElS3RecordingSinkConfigurationEl
{
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination = Some(v.into());
        self
    }
}

impl ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElS3RecordingSinkConfigurationEl {
    type O =
        BlockAssignable<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElS3RecordingSinkConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElS3RecordingSinkConfigurationEl
{}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElS3RecordingSinkConfigurationEl {
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElS3RecordingSinkConfigurationEl {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElS3RecordingSinkConfigurationEl {
            destination: core::default::Default::default(),
        }
    }
}

pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElS3RecordingSinkConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElS3RecordingSinkConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElS3RecordingSinkConfigurationElRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElS3RecordingSinkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElS3RecordingSinkConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSnsTopicSinkConfigurationEl
{
    insights_target: PrimField<String>,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSnsTopicSinkConfigurationEl {}

impl ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSnsTopicSinkConfigurationEl {
    type O =
        BlockAssignable<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSnsTopicSinkConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSnsTopicSinkConfigurationEl
{
    #[doc = ""]
    pub insights_target: PrimField<String>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSnsTopicSinkConfigurationEl {
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSnsTopicSinkConfigurationEl {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSnsTopicSinkConfigurationEl {
            insights_target: self.insights_target,
        }
    }
}

pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSnsTopicSinkConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSnsTopicSinkConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSnsTopicSinkConfigurationElRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSnsTopicSinkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl
    ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSnsTopicSinkConfigurationElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `insights_target` after provisioning.\n"]
    pub fn insights_target(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.insights_target", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSqsQueueSinkConfigurationEl
{
    insights_target: PrimField<String>,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSqsQueueSinkConfigurationEl {}

impl ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSqsQueueSinkConfigurationEl {
    type O =
        BlockAssignable<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSqsQueueSinkConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSqsQueueSinkConfigurationEl
{
    #[doc = ""]
    pub insights_target: PrimField<String>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSqsQueueSinkConfigurationEl {
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSqsQueueSinkConfigurationEl {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSqsQueueSinkConfigurationEl {
            insights_target: self.insights_target,
        }
    }
}

pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSqsQueueSinkConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSqsQueueSinkConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSqsQueueSinkConfigurationElRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSqsQueueSinkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl
    ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSqsQueueSinkConfigurationElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `insights_target` after provisioning.\n"]
    pub fn insights_target(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.insights_target", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElVoiceAnalyticsProcessorConfigurationEl
{
    speaker_search_status: PrimField<String>,
    voice_tone_analysis_status: PrimField<String>,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElVoiceAnalyticsProcessorConfigurationEl { }

impl ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElVoiceAnalyticsProcessorConfigurationEl {
    type O =
        BlockAssignable<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElVoiceAnalyticsProcessorConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElVoiceAnalyticsProcessorConfigurationEl
{
    #[doc = ""]
    pub speaker_search_status: PrimField<String>,
    #[doc = ""]
    pub voice_tone_analysis_status: PrimField<String>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElVoiceAnalyticsProcessorConfigurationEl {
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElVoiceAnalyticsProcessorConfigurationEl {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElVoiceAnalyticsProcessorConfigurationEl {
            speaker_search_status: self.speaker_search_status,
            voice_tone_analysis_status: self.voice_tone_analysis_status,
        }
    }
}

pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElVoiceAnalyticsProcessorConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElVoiceAnalyticsProcessorConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElVoiceAnalyticsProcessorConfigurationElRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElVoiceAnalyticsProcessorConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElVoiceAnalyticsProcessorConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `speaker_search_status` after provisioning.\n"]
    pub fn speaker_search_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.speaker_search_status", self.base))
    }

    #[doc = "Get a reference to the value of field `voice_tone_analysis_status` after provisioning.\n"]
    pub fn voice_tone_analysis_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.voice_tone_analysis_status", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElDynamic {
    amazon_transcribe_call_analytics_processor_configuration: Option<
        DynamicBlock<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationEl,
        >,
    >,
    amazon_transcribe_processor_configuration: Option<
        DynamicBlock<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeProcessorConfigurationEl,
        >,
    >,
    kinesis_data_stream_sink_configuration: Option<
        DynamicBlock<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElKinesisDataStreamSinkConfigurationEl,
        >,
    >,
    lambda_function_sink_configuration: Option<
        DynamicBlock<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElLambdaFunctionSinkConfigurationEl,
        >,
    >,
    s3_recording_sink_configuration: Option<
        DynamicBlock<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElS3RecordingSinkConfigurationEl>,
    >,
    sns_topic_sink_configuration: Option<
        DynamicBlock<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSnsTopicSinkConfigurationEl>,
    >,
    sqs_queue_sink_configuration: Option<
        DynamicBlock<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSqsQueueSinkConfigurationEl>,
    >,
    voice_analytics_processor_configuration: Option<
        DynamicBlock<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElVoiceAnalyticsProcessorConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amazon_transcribe_call_analytics_processor_configuration: Option<
        Vec<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    amazon_transcribe_processor_configuration: Option<
        Vec<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeProcessorConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_data_stream_sink_configuration: Option<
        Vec<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElKinesisDataStreamSinkConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_function_sink_configuration: Option<
        Vec<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElLambdaFunctionSinkConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_recording_sink_configuration: Option<
        Vec<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElS3RecordingSinkConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    sns_topic_sink_configuration: Option<
        Vec<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSnsTopicSinkConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    sqs_queue_sink_configuration: Option<
        Vec<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSqsQueueSinkConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    voice_analytics_processor_configuration: Option<
        Vec<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElVoiceAnalyticsProcessorConfigurationEl>,
    >,
    dynamic: ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElDynamic,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsEl {
    #[doc = "Set the field `amazon_transcribe_call_analytics_processor_configuration`.\n"]
    pub fn set_amazon_transcribe_call_analytics_processor_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.amazon_transcribe_call_analytics_processor_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic
                    .amazon_transcribe_call_analytics_processor_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `amazon_transcribe_processor_configuration`.\n"]
    pub fn set_amazon_transcribe_processor_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeProcessorConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.amazon_transcribe_processor_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.amazon_transcribe_processor_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `kinesis_data_stream_sink_configuration`.\n"]
    pub fn set_kinesis_data_stream_sink_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElKinesisDataStreamSinkConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kinesis_data_stream_sink_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_data_stream_sink_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `lambda_function_sink_configuration`.\n"]
    pub fn set_lambda_function_sink_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElLambdaFunctionSinkConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lambda_function_sink_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_function_sink_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `s3_recording_sink_configuration`.\n"]
    pub fn set_s3_recording_sink_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElS3RecordingSinkConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_recording_sink_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_recording_sink_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `sns_topic_sink_configuration`.\n"]
    pub fn set_sns_topic_sink_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSnsTopicSinkConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sns_topic_sink_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sns_topic_sink_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `sqs_queue_sink_configuration`.\n"]
    pub fn set_sqs_queue_sink_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSqsQueueSinkConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sqs_queue_sink_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sqs_queue_sink_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `voice_analytics_processor_configuration`.\n"]
    pub fn set_voice_analytics_processor_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElVoiceAnalyticsProcessorConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.voice_analytics_processor_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.voice_analytics_processor_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsEl {
    type O = BlockAssignable<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsEl {
    pub fn build(self) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsEl {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsEl {
            type_: self.type_,
            amazon_transcribe_call_analytics_processor_configuration:
                core::default::Default::default(),
            amazon_transcribe_processor_configuration: core::default::Default::default(),
            kinesis_data_stream_sink_configuration: core::default::Default::default(),
            lambda_function_sink_configuration: core::default::Default::default(),
            s3_recording_sink_configuration: core::default::Default::default(),
            sns_topic_sink_configuration: core::default::Default::default(),
            sqs_queue_sink_configuration: core::default::Default::default(),
            voice_analytics_processor_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `amazon_transcribe_call_analytics_processor_configuration` after provisioning.\n"]
    pub fn amazon_transcribe_call_analytics_processor_configuration(
        &self,
    ) -> ListRef<
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeCallAnalyticsProcessorConfigurationElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.amazon_transcribe_call_analytics_processor_configuration",
                self.base
            ),
        )
    }

    #[doc = "Get a reference to the value of field `amazon_transcribe_processor_configuration` after provisioning.\n"]
    pub fn amazon_transcribe_processor_configuration(
        &self,
    ) -> ListRef<
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElAmazonTranscribeProcessorConfigurationElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.amazon_transcribe_processor_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `kinesis_data_stream_sink_configuration` after provisioning.\n"]
    pub fn kinesis_data_stream_sink_configuration(
        &self,
    ) -> ListRef<
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElKinesisDataStreamSinkConfigurationElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.kinesis_data_stream_sink_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `lambda_function_sink_configuration` after provisioning.\n"]
    pub fn lambda_function_sink_configuration(
        &self,
    ) -> ListRef<
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElLambdaFunctionSinkConfigurationElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.lambda_function_sink_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `s3_recording_sink_configuration` after provisioning.\n"]
    pub fn s3_recording_sink_configuration(
        &self,
    ) -> ListRef<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElS3RecordingSinkConfigurationElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_recording_sink_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sns_topic_sink_configuration` after provisioning.\n"]
    pub fn sns_topic_sink_configuration(
        &self,
    ) -> ListRef<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSnsTopicSinkConfigurationElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.sns_topic_sink_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sqs_queue_sink_configuration` after provisioning.\n"]
    pub fn sqs_queue_sink_configuration(
        &self,
    ) -> ListRef<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElSqsQueueSinkConfigurationElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.sqs_queue_sink_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `voice_analytics_processor_configuration` after provisioning.\n"]
    pub fn voice_analytics_processor_configuration(
        &self,
    ) -> ListRef<
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsElVoiceAnalyticsProcessorConfigurationElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.voice_analytics_processor_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElIssueDetectionConfigurationEl
{
    rule_name: PrimField<String>,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElIssueDetectionConfigurationEl {

}

impl ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElIssueDetectionConfigurationEl {
    type O =
        BlockAssignable<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElIssueDetectionConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElIssueDetectionConfigurationEl
{
    #[doc = ""]
    pub rule_name: PrimField<String>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElIssueDetectionConfigurationEl {
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElIssueDetectionConfigurationEl {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElIssueDetectionConfigurationEl {
            rule_name: self.rule_name,
        }
    }
}

pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElIssueDetectionConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElIssueDetectionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElIssueDetectionConfigurationElRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElIssueDetectionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElIssueDetectionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\n"]
    pub fn rule_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElKeywordMatchConfigurationEl
{
    keywords: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    negate: Option<PrimField<bool>>,
    rule_name: PrimField<String>,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElKeywordMatchConfigurationEl {
    #[doc = "Set the field `negate`.\n"]
    pub fn set_negate(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.negate = Some(v.into());
        self
    }
}

impl ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElKeywordMatchConfigurationEl {
    type O =
        BlockAssignable<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElKeywordMatchConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElKeywordMatchConfigurationEl
{
    #[doc = ""]
    pub keywords: ListField<PrimField<String>>,
    #[doc = ""]
    pub rule_name: PrimField<String>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElKeywordMatchConfigurationEl {
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElKeywordMatchConfigurationEl {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElKeywordMatchConfigurationEl {
            keywords: self.keywords,
            negate: core::default::Default::default(),
            rule_name: self.rule_name,
        }
    }
}

pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElKeywordMatchConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElKeywordMatchConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElKeywordMatchConfigurationElRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElKeywordMatchConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElKeywordMatchConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `keywords` after provisioning.\n"]
    pub fn keywords(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.keywords", self.base))
    }

    #[doc = "Get a reference to the value of field `negate` after provisioning.\n"]
    pub fn negate(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.negate", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\n"]
    pub fn rule_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElSentimentConfigurationEl
{
    rule_name: PrimField<String>,
    sentiment_type: PrimField<String>,
    time_period: PrimField<f64>,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElSentimentConfigurationEl {

}

impl ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElSentimentConfigurationEl {
    type O =
        BlockAssignable<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElSentimentConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElSentimentConfigurationEl
{
    #[doc = ""]
    pub rule_name: PrimField<String>,
    #[doc = ""]
    pub sentiment_type: PrimField<String>,
    #[doc = ""]
    pub time_period: PrimField<f64>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElSentimentConfigurationEl {
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElSentimentConfigurationEl {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElSentimentConfigurationEl {
            rule_name: self.rule_name,
            sentiment_type: self.sentiment_type,
            time_period: self.time_period,
        }
    }
}

pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElSentimentConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElSentimentConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElSentimentConfigurationElRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElSentimentConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElSentimentConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\n"]
    pub fn rule_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.base))
    }

    #[doc = "Get a reference to the value of field `sentiment_type` after provisioning.\n"]
    pub fn sentiment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sentiment_type", self.base))
    }

    #[doc = "Get a reference to the value of field `time_period` after provisioning.\n"]
    pub fn time_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_period", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElDynamic {
    issue_detection_configuration: Option<
        DynamicBlock<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElIssueDetectionConfigurationEl,
        >,
    >,
    keyword_match_configuration: Option<
        DynamicBlock<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElKeywordMatchConfigurationEl,
        >,
    >,
    sentiment_configuration: Option<
        DynamicBlock<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElSentimentConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issue_detection_configuration: Option<
        Vec<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElIssueDetectionConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    keyword_match_configuration: Option<
        Vec<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElKeywordMatchConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    sentiment_configuration: Option<
        Vec<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElSentimentConfigurationEl,
        >,
    >,
    dynamic: ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElDynamic,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesEl {
    #[doc = "Set the field `issue_detection_configuration`.\n"]
    pub fn set_issue_detection_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElIssueDetectionConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.issue_detection_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.issue_detection_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `keyword_match_configuration`.\n"]
    pub fn set_keyword_match_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElKeywordMatchConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.keyword_match_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.keyword_match_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `sentiment_configuration`.\n"]
    pub fn set_sentiment_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElSentimentConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sentiment_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sentiment_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesEl
{
    type O = BlockAssignable<
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesEl
{
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl
    BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesEl
{
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesEl
    {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesEl {
            type_: self.type_,
            issue_detection_configuration: core::default::Default::default(),
            keyword_match_configuration: core::default::Default::default(),
            sentiment_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl
    ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `issue_detection_configuration` after provisioning.\n"]
    pub fn issue_detection_configuration(
        &self,
    ) -> ListRef<
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElIssueDetectionConfigurationElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.issue_detection_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `keyword_match_configuration` after provisioning.\n"]
    pub fn keyword_match_configuration(
        &self,
    ) -> ListRef<
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElKeywordMatchConfigurationElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.keyword_match_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sentiment_configuration` after provisioning.\n"]
    pub fn sentiment_configuration(
        &self,
    ) -> ListRef<
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElSentimentConfigurationElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.sentiment_configuration", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElDynamic {
    rules: Option<
        DynamicBlock<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesEl>,
    >,
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<Vec<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesEl>>,
    dynamic: ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElDynamic,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationEl {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rules = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationEl
{
    type O = BlockAssignable<
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationEl
{}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationEl {
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationEl {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationEl {
            disabled: core::default::Default::default(),
            rules: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref
    for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRef
    {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(
        &self,
    ) -> ListRef<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationElRulesElRef>{
        ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationTimeoutsEl {
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

impl ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationTimeoutsEl {
    type O = BlockAssignable<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationTimeoutsEl {}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfigurationTimeoutsEl {
    pub fn build(self) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationTimeoutsEl {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfigurationTimeoutsElRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfigurationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationTimeoutsElRef {
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
struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationDynamic {
    elements:
        Option<DynamicBlock<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationElementsEl>>,
    real_time_alert_configuration: Option<
        DynamicBlock<
            ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRealTimeAlertConfigurationEl,
        >,
    >,
}
