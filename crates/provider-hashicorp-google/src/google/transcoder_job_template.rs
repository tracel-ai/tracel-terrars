use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct TranscoderJobTemplateData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    job_template_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Vec<TranscoderJobTemplateConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<TranscoderJobTemplateTimeoutsEl>,
    dynamic: TranscoderJobTemplateDynamic,
}
struct TranscoderJobTemplate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TranscoderJobTemplateData>,
}
#[derive(Clone)]
pub struct TranscoderJobTemplate(Rc<TranscoderJobTemplate_>);
impl TranscoderJobTemplate {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nThe labels associated with this job template. You can use these to organize and group your job templates.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(self, v: impl Into<BlockAssignable<TranscoderJobTemplateConfigEl>>) -> Self {
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
    pub fn set_timeouts(self, v: impl Into<TranscoderJobTemplateTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `job_template_id` after provisioning.\nID to use for the Transcoding job template."]
    pub fn job_template_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_template_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels associated with this job template. You can use these to organize and group your job templates.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the transcoding job template resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the job template."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<TranscoderJobTemplateConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> TranscoderJobTemplateTimeoutsElRef {
        TranscoderJobTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for TranscoderJobTemplate {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for TranscoderJobTemplate {}
impl ToListMappable for TranscoderJobTemplate {
    type O = ListRef<TranscoderJobTemplateRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for TranscoderJobTemplate_ {
    fn extract_resource_type(&self) -> String {
        "google_transcoder_job_template".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildTranscoderJobTemplate {
    pub tf_id: String,
    #[doc = "ID to use for the Transcoding job template."]
    pub job_template_id: PrimField<String>,
    #[doc = "The location of the transcoding job template resource."]
    pub location: PrimField<String>,
}
impl BuildTranscoderJobTemplate {
    pub fn build(self, stack: &mut Stack) -> TranscoderJobTemplate {
        let out = TranscoderJobTemplate(Rc::new(TranscoderJobTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TranscoderJobTemplateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                job_template_id: self.job_template_id,
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct TranscoderJobTemplateRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl TranscoderJobTemplateRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `job_template_id` after provisioning.\nID to use for the Transcoding job template."]
    pub fn job_template_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_template_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels associated with this job template. You can use these to organize and group your job templates.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the transcoding job template resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the job template."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<TranscoderJobTemplateConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> TranscoderJobTemplateTimeoutsElRef {
        TranscoderJobTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElAdBreaksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time_offset: Option<PrimField<String>>,
}
impl TranscoderJobTemplateConfigElAdBreaksEl {
    #[doc = "Set the field `start_time_offset`.\nStart time in seconds for the ad break, relative to the output file timeline"]
    pub fn set_start_time_offset(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time_offset = Some(v.into());
        self
    }
}
impl ToListMappable for TranscoderJobTemplateConfigElAdBreaksEl {
    type O = BlockAssignable<TranscoderJobTemplateConfigElAdBreaksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElAdBreaksEl {}
impl BuildTranscoderJobTemplateConfigElAdBreaksEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElAdBreaksEl {
        TranscoderJobTemplateConfigElAdBreaksEl {
            start_time_offset: core::default::Default::default(),
        }
    }
}
pub struct TranscoderJobTemplateConfigElAdBreaksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElAdBreaksElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobTemplateConfigElAdBreaksElRef {
        TranscoderJobTemplateConfigElAdBreaksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElAdBreaksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `start_time_offset` after provisioning.\nStart time in seconds for the ad break, relative to the output file timeline"]
    pub fn start_time_offset(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_time_offset", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElEditListEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    inputs: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time_offset: Option<PrimField<String>>,
}
impl TranscoderJobTemplateConfigElEditListEl {
    #[doc = "Set the field `inputs`.\nList of values identifying files that should be used in this atom."]
    pub fn set_inputs(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.inputs = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\nA unique key for this atom."]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time_offset`.\nStart time in seconds for the atom, relative to the input file timeline.  The default is '0s'."]
    pub fn set_start_time_offset(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time_offset = Some(v.into());
        self
    }
}
impl ToListMappable for TranscoderJobTemplateConfigElEditListEl {
    type O = BlockAssignable<TranscoderJobTemplateConfigElEditListEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElEditListEl {}
impl BuildTranscoderJobTemplateConfigElEditListEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElEditListEl {
        TranscoderJobTemplateConfigElEditListEl {
            inputs: core::default::Default::default(),
            key: core::default::Default::default(),
            start_time_offset: core::default::Default::default(),
        }
    }
}
pub struct TranscoderJobTemplateConfigElEditListElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElEditListElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobTemplateConfigElEditListElRef {
        TranscoderJobTemplateConfigElEditListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElEditListElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `inputs` after provisioning.\nList of values identifying files that should be used in this atom."]
    pub fn inputs(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.inputs", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nA unique key for this atom."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time_offset` after provisioning.\nStart time in seconds for the atom, relative to the input file timeline.  The default is '0s'."]
    pub fn start_time_offset(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_time_offset", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElElementaryStreamsElAudioStreamEl {
    bitrate_bps: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_layout: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    codec: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_rate_hertz: Option<PrimField<f64>>,
}
impl TranscoderJobTemplateConfigElElementaryStreamsElAudioStreamEl {
    #[doc = "Set the field `channel_count`.\nNumber of audio channels. The default is '2'."]
    pub fn set_channel_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.channel_count = Some(v.into());
        self
    }
    #[doc = "Set the field `channel_layout`.\nA list of channel names specifying layout of the audio channels.  The default is [\"fl\", \"fr\"]."]
    pub fn set_channel_layout(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.channel_layout = Some(v.into());
        self
    }
    #[doc = "Set the field `codec`.\nThe codec for this audio stream. The default is 'aac'."]
    pub fn set_codec(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.codec = Some(v.into());
        self
    }
    #[doc = "Set the field `sample_rate_hertz`.\nThe audio sample rate in Hertz. The default is '48000'."]
    pub fn set_sample_rate_hertz(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.sample_rate_hertz = Some(v.into());
        self
    }
}
impl ToListMappable for TranscoderJobTemplateConfigElElementaryStreamsElAudioStreamEl {
    type O = BlockAssignable<TranscoderJobTemplateConfigElElementaryStreamsElAudioStreamEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElElementaryStreamsElAudioStreamEl {
    #[doc = "Audio bitrate in bits per second."]
    pub bitrate_bps: PrimField<f64>,
}
impl BuildTranscoderJobTemplateConfigElElementaryStreamsElAudioStreamEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElElementaryStreamsElAudioStreamEl {
        TranscoderJobTemplateConfigElElementaryStreamsElAudioStreamEl {
            bitrate_bps: self.bitrate_bps,
            channel_count: core::default::Default::default(),
            channel_layout: core::default::Default::default(),
            codec: core::default::Default::default(),
            sample_rate_hertz: core::default::Default::default(),
        }
    }
}
pub struct TranscoderJobTemplateConfigElElementaryStreamsElAudioStreamElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElElementaryStreamsElAudioStreamElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobTemplateConfigElElementaryStreamsElAudioStreamElRef {
        TranscoderJobTemplateConfigElElementaryStreamsElAudioStreamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElElementaryStreamsElAudioStreamElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bitrate_bps` after provisioning.\nAudio bitrate in bits per second."]
    pub fn bitrate_bps(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.bitrate_bps", self.base))
    }
    #[doc = "Get a reference to the value of field `channel_count` after provisioning.\nNumber of audio channels. The default is '2'."]
    pub fn channel_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.channel_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `channel_layout` after provisioning.\nA list of channel names specifying layout of the audio channels.  The default is [\"fl\", \"fr\"]."]
    pub fn channel_layout(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.channel_layout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `codec` after provisioning.\nThe codec for this audio stream. The default is 'aac'."]
    pub fn codec(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.codec", self.base))
    }
    #[doc = "Get a reference to the value of field `sample_rate_hertz` after provisioning.\nThe audio sample rate in Hertz. The default is '48000'."]
    pub fn sample_rate_hertz(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sample_rate_hertz", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElHlgEl {}
impl TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElHlgEl {}
impl ToListMappable for TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElHlgEl {
    type O =
        BlockAssignable<TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElHlgEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElHlgEl {}
impl BuildTranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElHlgEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElHlgEl {
        TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElHlgEl {}
    }
}
pub struct TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElHlgElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElHlgElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElHlgElRef {
        TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElHlgElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElHlgElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElSdrEl {}
impl TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElSdrEl {}
impl ToListMappable for TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElSdrEl {
    type O =
        BlockAssignable<TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElSdrEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElSdrEl {}
impl BuildTranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElSdrEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElSdrEl {
        TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElSdrEl {}
    }
}
pub struct TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElSdrElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElSdrElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElSdrElRef {
        TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElSdrElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElSdrElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElDynamic {
    hlg: Option<
        DynamicBlock<TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElHlgEl>,
    >,
    sdr: Option<
        DynamicBlock<TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElSdrEl>,
    >,
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264El {
    bitrate_bps: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    crf_level: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entropy_coder: Option<PrimField<String>>,
    frame_rate: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gop_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height_pixels: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pixel_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preset: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_control_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vbv_fullness_bits: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vbv_size_bits: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width_pixels: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hlg: Option<Vec<TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElHlgEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sdr: Option<Vec<TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElSdrEl>>,
    dynamic: TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElDynamic,
}
impl TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264El {
    #[doc = "Set the field `crf_level`.\nTarget CRF level. The default is '21'."]
    pub fn set_crf_level(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.crf_level = Some(v.into());
        self
    }
    #[doc = "Set the field `entropy_coder`.\nThe entropy coder to use. The default is 'cabac'."]
    pub fn set_entropy_coder(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.entropy_coder = Some(v.into());
        self
    }
    #[doc = "Set the field `gop_duration`.\nSelect the GOP size based on the specified duration. The default is '3s'."]
    pub fn set_gop_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gop_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `height_pixels`.\nThe height of the video in pixels."]
    pub fn set_height_pixels(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.height_pixels = Some(v.into());
        self
    }
    #[doc = "Set the field `pixel_format`.\nPixel format to use. The default is 'yuv420p'."]
    pub fn set_pixel_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pixel_format = Some(v.into());
        self
    }
    #[doc = "Set the field `preset`.\nEnforces the specified codec preset. The default is 'veryfast'."]
    pub fn set_preset(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.preset = Some(v.into());
        self
    }
    #[doc = "Set the field `profile`.\nEnforces the specified codec profile."]
    pub fn set_profile(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.profile = Some(v.into());
        self
    }
    #[doc = "Set the field `rate_control_mode`.\nSpecify the mode. The default is 'vbr'."]
    pub fn set_rate_control_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rate_control_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `vbv_fullness_bits`.\nInitial fullness of the Video Buffering Verifier (VBV) buffer in bits."]
    pub fn set_vbv_fullness_bits(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.vbv_fullness_bits = Some(v.into());
        self
    }
    #[doc = "Set the field `vbv_size_bits`.\nSize of the Video Buffering Verifier (VBV) buffer in bits."]
    pub fn set_vbv_size_bits(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.vbv_size_bits = Some(v.into());
        self
    }
    #[doc = "Set the field `width_pixels`.\nThe width of the video in pixels."]
    pub fn set_width_pixels(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.width_pixels = Some(v.into());
        self
    }
    #[doc = "Set the field `hlg`.\n"]
    pub fn set_hlg(
        mut self,
        v: impl Into<
            BlockAssignable<
                TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElHlgEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hlg = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hlg = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sdr`.\n"]
    pub fn set_sdr(
        mut self,
        v: impl Into<
            BlockAssignable<
                TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElSdrEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sdr = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sdr = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264El {
    type O = BlockAssignable<TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264El {
    #[doc = "The video bitrate in bits per second."]
    pub bitrate_bps: PrimField<f64>,
    #[doc = "The target video frame rate in frames per second (FPS)."]
    pub frame_rate: PrimField<f64>,
}
impl BuildTranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264El {
    pub fn build(self) -> TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264El {
        TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264El {
            bitrate_bps: self.bitrate_bps,
            crf_level: core::default::Default::default(),
            entropy_coder: core::default::Default::default(),
            frame_rate: self.frame_rate,
            gop_duration: core::default::Default::default(),
            height_pixels: core::default::Default::default(),
            pixel_format: core::default::Default::default(),
            preset: core::default::Default::default(),
            profile: core::default::Default::default(),
            rate_control_mode: core::default::Default::default(),
            vbv_fullness_bits: core::default::Default::default(),
            vbv_size_bits: core::default::Default::default(),
            width_pixels: core::default::Default::default(),
            hlg: core::default::Default::default(),
            sdr: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElRef {
        TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bitrate_bps` after provisioning.\nThe video bitrate in bits per second."]
    pub fn bitrate_bps(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.bitrate_bps", self.base))
    }
    #[doc = "Get a reference to the value of field `crf_level` after provisioning.\nTarget CRF level. The default is '21'."]
    pub fn crf_level(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.crf_level", self.base))
    }
    #[doc = "Get a reference to the value of field `entropy_coder` after provisioning.\nThe entropy coder to use. The default is 'cabac'."]
    pub fn entropy_coder(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entropy_coder", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `frame_rate` after provisioning.\nThe target video frame rate in frames per second (FPS)."]
    pub fn frame_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.frame_rate", self.base))
    }
    #[doc = "Get a reference to the value of field `gop_duration` after provisioning.\nSelect the GOP size based on the specified duration. The default is '3s'."]
    pub fn gop_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gop_duration", self.base))
    }
    #[doc = "Get a reference to the value of field `height_pixels` after provisioning.\nThe height of the video in pixels."]
    pub fn height_pixels(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.height_pixels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pixel_format` after provisioning.\nPixel format to use. The default is 'yuv420p'."]
    pub fn pixel_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pixel_format", self.base))
    }
    #[doc = "Get a reference to the value of field `preset` after provisioning.\nEnforces the specified codec preset. The default is 'veryfast'."]
    pub fn preset(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preset", self.base))
    }
    #[doc = "Get a reference to the value of field `profile` after provisioning.\nEnforces the specified codec profile."]
    pub fn profile(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.profile", self.base))
    }
    #[doc = "Get a reference to the value of field `rate_control_mode` after provisioning.\nSpecify the mode. The default is 'vbr'."]
    pub fn rate_control_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rate_control_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vbv_fullness_bits` after provisioning.\nInitial fullness of the Video Buffering Verifier (VBV) buffer in bits."]
    pub fn vbv_fullness_bits(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vbv_fullness_bits", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vbv_size_bits` after provisioning.\nSize of the Video Buffering Verifier (VBV) buffer in bits."]
    pub fn vbv_size_bits(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vbv_size_bits", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `width_pixels` after provisioning.\nThe width of the video in pixels."]
    pub fn width_pixels(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.width_pixels", self.base))
    }
    #[doc = "Get a reference to the value of field `hlg` after provisioning.\n"]
    pub fn hlg(
        &self,
    ) -> ListRef<TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElHlgElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hlg", self.base))
    }
    #[doc = "Get a reference to the value of field `sdr` after provisioning.\n"]
    pub fn sdr(
        &self,
    ) -> ListRef<TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElSdrElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sdr", self.base))
    }
}
#[derive(Serialize, Default)]
struct TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElDynamic {
    h264: Option<DynamicBlock<TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264El>>,
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    h264: Option<Vec<TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264El>>,
    dynamic: TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElDynamic,
}
impl TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamEl {
    #[doc = "Set the field `h264`.\n"]
    pub fn set_h264(
        mut self,
        v: impl Into<
            BlockAssignable<TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264El>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.h264 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.h264 = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamEl {
    type O = BlockAssignable<TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElElementaryStreamsElVideoStreamEl {}
impl BuildTranscoderJobTemplateConfigElElementaryStreamsElVideoStreamEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamEl {
        TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamEl {
            h264: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElRef {
        TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `h264` after provisioning.\n"]
    pub fn h264(
        &self,
    ) -> ListRef<TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElH264ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.h264", self.base))
    }
}
#[derive(Serialize, Default)]
struct TranscoderJobTemplateConfigElElementaryStreamsElDynamic {
    audio_stream:
        Option<DynamicBlock<TranscoderJobTemplateConfigElElementaryStreamsElAudioStreamEl>>,
    video_stream:
        Option<DynamicBlock<TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamEl>>,
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElElementaryStreamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_stream: Option<Vec<TranscoderJobTemplateConfigElElementaryStreamsElAudioStreamEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_stream: Option<Vec<TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamEl>>,
    dynamic: TranscoderJobTemplateConfigElElementaryStreamsElDynamic,
}
impl TranscoderJobTemplateConfigElElementaryStreamsEl {
    #[doc = "Set the field `key`.\nA unique key for this atom."]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `audio_stream`.\n"]
    pub fn set_audio_stream(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobTemplateConfigElElementaryStreamsElAudioStreamEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.audio_stream = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_stream = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `video_stream`.\n"]
    pub fn set_video_stream(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.video_stream = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.video_stream = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for TranscoderJobTemplateConfigElElementaryStreamsEl {
    type O = BlockAssignable<TranscoderJobTemplateConfigElElementaryStreamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElElementaryStreamsEl {}
impl BuildTranscoderJobTemplateConfigElElementaryStreamsEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElElementaryStreamsEl {
        TranscoderJobTemplateConfigElElementaryStreamsEl {
            key: core::default::Default::default(),
            audio_stream: core::default::Default::default(),
            video_stream: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct TranscoderJobTemplateConfigElElementaryStreamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElElementaryStreamsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobTemplateConfigElElementaryStreamsElRef {
        TranscoderJobTemplateConfigElElementaryStreamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElElementaryStreamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nA unique key for this atom."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `audio_stream` after provisioning.\n"]
    pub fn audio_stream(
        &self,
    ) -> ListRef<TranscoderJobTemplateConfigElElementaryStreamsElAudioStreamElRef> {
        ListRef::new(self.shared().clone(), format!("{}.audio_stream", self.base))
    }
    #[doc = "Get a reference to the value of field `video_stream` after provisioning.\n"]
    pub fn video_stream(
        &self,
    ) -> ListRef<TranscoderJobTemplateConfigElElementaryStreamsElVideoStreamElRef> {
        ListRef::new(self.shared().clone(), format!("{}.video_stream", self.base))
    }
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElEncryptionsElAes128El {}
impl TranscoderJobTemplateConfigElEncryptionsElAes128El {}
impl ToListMappable for TranscoderJobTemplateConfigElEncryptionsElAes128El {
    type O = BlockAssignable<TranscoderJobTemplateConfigElEncryptionsElAes128El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElEncryptionsElAes128El {}
impl BuildTranscoderJobTemplateConfigElEncryptionsElAes128El {
    pub fn build(self) -> TranscoderJobTemplateConfigElEncryptionsElAes128El {
        TranscoderJobTemplateConfigElEncryptionsElAes128El {}
    }
}
pub struct TranscoderJobTemplateConfigElEncryptionsElAes128ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElEncryptionsElAes128ElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobTemplateConfigElEncryptionsElAes128ElRef {
        TranscoderJobTemplateConfigElEncryptionsElAes128ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElEncryptionsElAes128ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElClearkeyEl {}
impl TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElClearkeyEl {}
impl ToListMappable for TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElClearkeyEl {
    type O = BlockAssignable<TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElClearkeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElEncryptionsElDrmSystemsElClearkeyEl {}
impl BuildTranscoderJobTemplateConfigElEncryptionsElDrmSystemsElClearkeyEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElClearkeyEl {
        TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElClearkeyEl {}
    }
}
pub struct TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElClearkeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElClearkeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElClearkeyElRef {
        TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElClearkeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElClearkeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElFairplayEl {}
impl TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElFairplayEl {}
impl ToListMappable for TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElFairplayEl {
    type O = BlockAssignable<TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElFairplayEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElEncryptionsElDrmSystemsElFairplayEl {}
impl BuildTranscoderJobTemplateConfigElEncryptionsElDrmSystemsElFairplayEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElFairplayEl {
        TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElFairplayEl {}
    }
}
pub struct TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElFairplayElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElFairplayElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElFairplayElRef {
        TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElFairplayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElFairplayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElPlayreadyEl {}
impl TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElPlayreadyEl {}
impl ToListMappable for TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElPlayreadyEl {
    type O = BlockAssignable<TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElPlayreadyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElEncryptionsElDrmSystemsElPlayreadyEl {}
impl BuildTranscoderJobTemplateConfigElEncryptionsElDrmSystemsElPlayreadyEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElPlayreadyEl {
        TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElPlayreadyEl {}
    }
}
pub struct TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElPlayreadyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElPlayreadyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElPlayreadyElRef {
        TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElPlayreadyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElPlayreadyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElWidevineEl {}
impl TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElWidevineEl {}
impl ToListMappable for TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElWidevineEl {
    type O = BlockAssignable<TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElWidevineEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElEncryptionsElDrmSystemsElWidevineEl {}
impl BuildTranscoderJobTemplateConfigElEncryptionsElDrmSystemsElWidevineEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElWidevineEl {
        TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElWidevineEl {}
    }
}
pub struct TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElWidevineElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElWidevineElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElWidevineElRef {
        TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElWidevineElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElWidevineElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElDynamic {
    clearkey:
        Option<DynamicBlock<TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElClearkeyEl>>,
    fairplay:
        Option<DynamicBlock<TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElFairplayEl>>,
    playready:
        Option<DynamicBlock<TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElPlayreadyEl>>,
    widevine:
        Option<DynamicBlock<TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElWidevineEl>>,
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElEncryptionsElDrmSystemsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    clearkey: Option<Vec<TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElClearkeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fairplay: Option<Vec<TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElFairplayEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    playready: Option<Vec<TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElPlayreadyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    widevine: Option<Vec<TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElWidevineEl>>,
    dynamic: TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElDynamic,
}
impl TranscoderJobTemplateConfigElEncryptionsElDrmSystemsEl {
    #[doc = "Set the field `clearkey`.\n"]
    pub fn set_clearkey(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElClearkeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.clearkey = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.clearkey = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `fairplay`.\n"]
    pub fn set_fairplay(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElFairplayEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fairplay = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fairplay = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `playready`.\n"]
    pub fn set_playready(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElPlayreadyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.playready = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.playready = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `widevine`.\n"]
    pub fn set_widevine(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElWidevineEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.widevine = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.widevine = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for TranscoderJobTemplateConfigElEncryptionsElDrmSystemsEl {
    type O = BlockAssignable<TranscoderJobTemplateConfigElEncryptionsElDrmSystemsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElEncryptionsElDrmSystemsEl {}
impl BuildTranscoderJobTemplateConfigElEncryptionsElDrmSystemsEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElEncryptionsElDrmSystemsEl {
        TranscoderJobTemplateConfigElEncryptionsElDrmSystemsEl {
            clearkey: core::default::Default::default(),
            fairplay: core::default::Default::default(),
            playready: core::default::Default::default(),
            widevine: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElRef {
        TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `clearkey` after provisioning.\n"]
    pub fn clearkey(
        &self,
    ) -> ListRef<TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElClearkeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.clearkey", self.base))
    }
    #[doc = "Get a reference to the value of field `fairplay` after provisioning.\n"]
    pub fn fairplay(
        &self,
    ) -> ListRef<TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElFairplayElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fairplay", self.base))
    }
    #[doc = "Get a reference to the value of field `playready` after provisioning.\n"]
    pub fn playready(
        &self,
    ) -> ListRef<TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElPlayreadyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.playready", self.base))
    }
    #[doc = "Get a reference to the value of field `widevine` after provisioning.\n"]
    pub fn widevine(
        &self,
    ) -> ListRef<TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElWidevineElRef> {
        ListRef::new(self.shared().clone(), format!("{}.widevine", self.base))
    }
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElEncryptionsElMpegCencEl {
    scheme: PrimField<String>,
}
impl TranscoderJobTemplateConfigElEncryptionsElMpegCencEl {}
impl ToListMappable for TranscoderJobTemplateConfigElEncryptionsElMpegCencEl {
    type O = BlockAssignable<TranscoderJobTemplateConfigElEncryptionsElMpegCencEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElEncryptionsElMpegCencEl {
    #[doc = "Specify the encryption scheme."]
    pub scheme: PrimField<String>,
}
impl BuildTranscoderJobTemplateConfigElEncryptionsElMpegCencEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElEncryptionsElMpegCencEl {
        TranscoderJobTemplateConfigElEncryptionsElMpegCencEl {
            scheme: self.scheme,
        }
    }
}
pub struct TranscoderJobTemplateConfigElEncryptionsElMpegCencElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElEncryptionsElMpegCencElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobTemplateConfigElEncryptionsElMpegCencElRef {
        TranscoderJobTemplateConfigElEncryptionsElMpegCencElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElEncryptionsElMpegCencElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scheme` after provisioning.\nSpecify the encryption scheme."]
    pub fn scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scheme", self.base))
    }
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElEncryptionsElSampleAesEl {}
impl TranscoderJobTemplateConfigElEncryptionsElSampleAesEl {}
impl ToListMappable for TranscoderJobTemplateConfigElEncryptionsElSampleAesEl {
    type O = BlockAssignable<TranscoderJobTemplateConfigElEncryptionsElSampleAesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElEncryptionsElSampleAesEl {}
impl BuildTranscoderJobTemplateConfigElEncryptionsElSampleAesEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElEncryptionsElSampleAesEl {
        TranscoderJobTemplateConfigElEncryptionsElSampleAesEl {}
    }
}
pub struct TranscoderJobTemplateConfigElEncryptionsElSampleAesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElEncryptionsElSampleAesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobTemplateConfigElEncryptionsElSampleAesElRef {
        TranscoderJobTemplateConfigElEncryptionsElSampleAesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElEncryptionsElSampleAesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElEncryptionsElSecretManagerKeySourceEl {
    secret_version: PrimField<String>,
}
impl TranscoderJobTemplateConfigElEncryptionsElSecretManagerKeySourceEl {}
impl ToListMappable for TranscoderJobTemplateConfigElEncryptionsElSecretManagerKeySourceEl {
    type O = BlockAssignable<TranscoderJobTemplateConfigElEncryptionsElSecretManagerKeySourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElEncryptionsElSecretManagerKeySourceEl {
    #[doc = "The name of the Secret Version containing the encryption key in the following format: projects/{project}/secrets/{secret_id}/versions/{version_number}."]
    pub secret_version: PrimField<String>,
}
impl BuildTranscoderJobTemplateConfigElEncryptionsElSecretManagerKeySourceEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElEncryptionsElSecretManagerKeySourceEl {
        TranscoderJobTemplateConfigElEncryptionsElSecretManagerKeySourceEl {
            secret_version: self.secret_version,
        }
    }
}
pub struct TranscoderJobTemplateConfigElEncryptionsElSecretManagerKeySourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElEncryptionsElSecretManagerKeySourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobTemplateConfigElEncryptionsElSecretManagerKeySourceElRef {
        TranscoderJobTemplateConfigElEncryptionsElSecretManagerKeySourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElEncryptionsElSecretManagerKeySourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret_version` after provisioning.\nThe name of the Secret Version containing the encryption key in the following format: projects/{project}/secrets/{secret_id}/versions/{version_number}."]
    pub fn secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_version", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct TranscoderJobTemplateConfigElEncryptionsElDynamic {
    aes128: Option<DynamicBlock<TranscoderJobTemplateConfigElEncryptionsElAes128El>>,
    drm_systems: Option<DynamicBlock<TranscoderJobTemplateConfigElEncryptionsElDrmSystemsEl>>,
    mpeg_cenc: Option<DynamicBlock<TranscoderJobTemplateConfigElEncryptionsElMpegCencEl>>,
    sample_aes: Option<DynamicBlock<TranscoderJobTemplateConfigElEncryptionsElSampleAesEl>>,
    secret_manager_key_source:
        Option<DynamicBlock<TranscoderJobTemplateConfigElEncryptionsElSecretManagerKeySourceEl>>,
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElEncryptionsEl {
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aes128: Option<Vec<TranscoderJobTemplateConfigElEncryptionsElAes128El>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drm_systems: Option<Vec<TranscoderJobTemplateConfigElEncryptionsElDrmSystemsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mpeg_cenc: Option<Vec<TranscoderJobTemplateConfigElEncryptionsElMpegCencEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_aes: Option<Vec<TranscoderJobTemplateConfigElEncryptionsElSampleAesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_manager_key_source:
        Option<Vec<TranscoderJobTemplateConfigElEncryptionsElSecretManagerKeySourceEl>>,
    dynamic: TranscoderJobTemplateConfigElEncryptionsElDynamic,
}
impl TranscoderJobTemplateConfigElEncryptionsEl {
    #[doc = "Set the field `aes128`.\n"]
    pub fn set_aes128(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobTemplateConfigElEncryptionsElAes128El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aes128 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aes128 = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `drm_systems`.\n"]
    pub fn set_drm_systems(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobTemplateConfigElEncryptionsElDrmSystemsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.drm_systems = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.drm_systems = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `mpeg_cenc`.\n"]
    pub fn set_mpeg_cenc(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobTemplateConfigElEncryptionsElMpegCencEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mpeg_cenc = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mpeg_cenc = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sample_aes`.\n"]
    pub fn set_sample_aes(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobTemplateConfigElEncryptionsElSampleAesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sample_aes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sample_aes = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `secret_manager_key_source`.\n"]
    pub fn set_secret_manager_key_source(
        mut self,
        v: impl Into<
            BlockAssignable<TranscoderJobTemplateConfigElEncryptionsElSecretManagerKeySourceEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.secret_manager_key_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_manager_key_source = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for TranscoderJobTemplateConfigElEncryptionsEl {
    type O = BlockAssignable<TranscoderJobTemplateConfigElEncryptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElEncryptionsEl {
    #[doc = "Identifier for this set of encryption options."]
    pub id: PrimField<String>,
}
impl BuildTranscoderJobTemplateConfigElEncryptionsEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElEncryptionsEl {
        TranscoderJobTemplateConfigElEncryptionsEl {
            id: self.id,
            aes128: core::default::Default::default(),
            drm_systems: core::default::Default::default(),
            mpeg_cenc: core::default::Default::default(),
            sample_aes: core::default::Default::default(),
            secret_manager_key_source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct TranscoderJobTemplateConfigElEncryptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElEncryptionsElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobTemplateConfigElEncryptionsElRef {
        TranscoderJobTemplateConfigElEncryptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElEncryptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier for this set of encryption options."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `aes128` after provisioning.\n"]
    pub fn aes128(&self) -> ListRef<TranscoderJobTemplateConfigElEncryptionsElAes128ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aes128", self.base))
    }
    #[doc = "Get a reference to the value of field `drm_systems` after provisioning.\n"]
    pub fn drm_systems(
        &self,
    ) -> ListRef<TranscoderJobTemplateConfigElEncryptionsElDrmSystemsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.drm_systems", self.base))
    }
    #[doc = "Get a reference to the value of field `mpeg_cenc` after provisioning.\n"]
    pub fn mpeg_cenc(&self) -> ListRef<TranscoderJobTemplateConfigElEncryptionsElMpegCencElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mpeg_cenc", self.base))
    }
    #[doc = "Get a reference to the value of field `sample_aes` after provisioning.\n"]
    pub fn sample_aes(&self) -> ListRef<TranscoderJobTemplateConfigElEncryptionsElSampleAesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sample_aes", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_manager_key_source` after provisioning.\n"]
    pub fn secret_manager_key_source(
        &self,
    ) -> ListRef<TranscoderJobTemplateConfigElEncryptionsElSecretManagerKeySourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secret_manager_key_source", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElInputsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl TranscoderJobTemplateConfigElInputsEl {
    #[doc = "Set the field `key`.\nA unique key for this input. Must be specified when using advanced mapping and edit lists."]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `uri`.\nURI of the media. Input files must be at least 5 seconds in duration and stored in Cloud Storage (for example, gs://bucket/inputs/file.mp4).\nIf empty, the value is populated from Job.input_uri."]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for TranscoderJobTemplateConfigElInputsEl {
    type O = BlockAssignable<TranscoderJobTemplateConfigElInputsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElInputsEl {}
impl BuildTranscoderJobTemplateConfigElInputsEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElInputsEl {
        TranscoderJobTemplateConfigElInputsEl {
            key: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}
pub struct TranscoderJobTemplateConfigElInputsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElInputsElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobTemplateConfigElInputsElRef {
        TranscoderJobTemplateConfigElInputsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElInputsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nA unique key for this input. Must be specified when using advanced mapping and edit lists."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nURI of the media. Input files must be at least 5 seconds in duration and stored in Cloud Storage (for example, gs://bucket/inputs/file.mp4).\nIf empty, the value is populated from Job.input_uri."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElManifestsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mux_streams: Option<ListField<PrimField<String>>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl TranscoderJobTemplateConfigElManifestsEl {
    #[doc = "Set the field `file_name`.\nThe name of the generated file. The default is 'manifest'."]
    pub fn set_file_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_name = Some(v.into());
        self
    }
    #[doc = "Set the field `mux_streams`.\nList of user supplied MuxStream.key values that should appear in this manifest."]
    pub fn set_mux_streams(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.mux_streams = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nType of the manifest. Possible values: [\"MANIFEST_TYPE_UNSPECIFIED\", \"HLS\", \"DASH\"]"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for TranscoderJobTemplateConfigElManifestsEl {
    type O = BlockAssignable<TranscoderJobTemplateConfigElManifestsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElManifestsEl {}
impl BuildTranscoderJobTemplateConfigElManifestsEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElManifestsEl {
        TranscoderJobTemplateConfigElManifestsEl {
            file_name: core::default::Default::default(),
            mux_streams: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct TranscoderJobTemplateConfigElManifestsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElManifestsElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobTemplateConfigElManifestsElRef {
        TranscoderJobTemplateConfigElManifestsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElManifestsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `file_name` after provisioning.\nThe name of the generated file. The default is 'manifest'."]
    pub fn file_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_name", self.base))
    }
    #[doc = "Get a reference to the value of field `mux_streams` after provisioning.\nList of user supplied MuxStream.key values that should appear in this manifest."]
    pub fn mux_streams(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.mux_streams", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of the manifest. Possible values: [\"MANIFEST_TYPE_UNSPECIFIED\", \"HLS\", \"DASH\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElMuxStreamsElSegmentSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    segment_duration: Option<PrimField<String>>,
}
impl TranscoderJobTemplateConfigElMuxStreamsElSegmentSettingsEl {
    #[doc = "Set the field `segment_duration`.\nDuration of the segments in seconds. The default is '6.0s'."]
    pub fn set_segment_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.segment_duration = Some(v.into());
        self
    }
}
impl ToListMappable for TranscoderJobTemplateConfigElMuxStreamsElSegmentSettingsEl {
    type O = BlockAssignable<TranscoderJobTemplateConfigElMuxStreamsElSegmentSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElMuxStreamsElSegmentSettingsEl {}
impl BuildTranscoderJobTemplateConfigElMuxStreamsElSegmentSettingsEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElMuxStreamsElSegmentSettingsEl {
        TranscoderJobTemplateConfigElMuxStreamsElSegmentSettingsEl {
            segment_duration: core::default::Default::default(),
        }
    }
}
pub struct TranscoderJobTemplateConfigElMuxStreamsElSegmentSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElMuxStreamsElSegmentSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobTemplateConfigElMuxStreamsElSegmentSettingsElRef {
        TranscoderJobTemplateConfigElMuxStreamsElSegmentSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElMuxStreamsElSegmentSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `segment_duration` after provisioning.\nDuration of the segments in seconds. The default is '6.0s'."]
    pub fn segment_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.segment_duration", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct TranscoderJobTemplateConfigElMuxStreamsElDynamic {
    segment_settings:
        Option<DynamicBlock<TranscoderJobTemplateConfigElMuxStreamsElSegmentSettingsEl>>,
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElMuxStreamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elementary_streams: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    segment_settings: Option<Vec<TranscoderJobTemplateConfigElMuxStreamsElSegmentSettingsEl>>,
    dynamic: TranscoderJobTemplateConfigElMuxStreamsElDynamic,
}
impl TranscoderJobTemplateConfigElMuxStreamsEl {
    #[doc = "Set the field `container`.\nThe container format. The default is 'mp4'."]
    pub fn set_container(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container = Some(v.into());
        self
    }
    #[doc = "Set the field `elementary_streams`.\nList of ElementaryStream.key values multiplexed in this stream."]
    pub fn set_elementary_streams(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.elementary_streams = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_id`.\nIdentifier of the encryption configuration to use."]
    pub fn set_encryption_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_id = Some(v.into());
        self
    }
    #[doc = "Set the field `file_name`.\nThe name of the generated file."]
    pub fn set_file_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_name = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\nA unique key for this multiplexed stream."]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `segment_settings`.\n"]
    pub fn set_segment_settings(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobTemplateConfigElMuxStreamsElSegmentSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.segment_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.segment_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for TranscoderJobTemplateConfigElMuxStreamsEl {
    type O = BlockAssignable<TranscoderJobTemplateConfigElMuxStreamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElMuxStreamsEl {}
impl BuildTranscoderJobTemplateConfigElMuxStreamsEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElMuxStreamsEl {
        TranscoderJobTemplateConfigElMuxStreamsEl {
            container: core::default::Default::default(),
            elementary_streams: core::default::Default::default(),
            encryption_id: core::default::Default::default(),
            file_name: core::default::Default::default(),
            key: core::default::Default::default(),
            segment_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct TranscoderJobTemplateConfigElMuxStreamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElMuxStreamsElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobTemplateConfigElMuxStreamsElRef {
        TranscoderJobTemplateConfigElMuxStreamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElMuxStreamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container` after provisioning.\nThe container format. The default is 'mp4'."]
    pub fn container(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container", self.base))
    }
    #[doc = "Get a reference to the value of field `elementary_streams` after provisioning.\nList of ElementaryStream.key values multiplexed in this stream."]
    pub fn elementary_streams(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.elementary_streams", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_id` after provisioning.\nIdentifier of the encryption configuration to use."]
    pub fn encryption_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `file_name` after provisioning.\nThe name of the generated file."]
    pub fn file_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_name", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nA unique key for this multiplexed stream."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `segment_settings` after provisioning.\n"]
    pub fn segment_settings(
        &self,
    ) -> ListRef<TranscoderJobTemplateConfigElMuxStreamsElSegmentSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.segment_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElOutputEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl TranscoderJobTemplateConfigElOutputEl {
    #[doc = "Set the field `uri`.\nURI for the output file(s). For example, gs://my-bucket/outputs/."]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for TranscoderJobTemplateConfigElOutputEl {
    type O = BlockAssignable<TranscoderJobTemplateConfigElOutputEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElOutputEl {}
impl BuildTranscoderJobTemplateConfigElOutputEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElOutputEl {
        TranscoderJobTemplateConfigElOutputEl {
            uri: core::default::Default::default(),
        }
    }
}
pub struct TranscoderJobTemplateConfigElOutputElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElOutputElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobTemplateConfigElOutputElRef {
        TranscoderJobTemplateConfigElOutputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElOutputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nURI for the output file(s). For example, gs://my-bucket/outputs/."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeElXyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<PrimField<f64>>,
}
impl TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeElXyEl {
    #[doc = "Set the field `x`.\nNormalized x coordinate."]
    pub fn set_x(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.x = Some(v.into());
        self
    }
    #[doc = "Set the field `y`.\nNormalized y coordinate."]
    pub fn set_y(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.y = Some(v.into());
        self
    }
}
impl ToListMappable for TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeElXyEl {
    type O =
        BlockAssignable<TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeElXyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeElXyEl {}
impl BuildTranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeElXyEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeElXyEl {
        TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeElXyEl {
            x: core::default::Default::default(),
            y: core::default::Default::default(),
        }
    }
}
pub struct TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeElXyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeElXyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeElXyElRef {
        TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeElXyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeElXyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `x` after provisioning.\nNormalized x coordinate."]
    pub fn x(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.x", self.base))
    }
    #[doc = "Get a reference to the value of field `y` after provisioning.\nNormalized y coordinate."]
    pub fn y(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.y", self.base))
    }
}
#[derive(Serialize, Default)]
struct TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeElDynamic {
    xy: Option<
        DynamicBlock<TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeElXyEl>,
    >,
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time_offset: Option<PrimField<String>>,
    fade_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time_offset: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    xy: Option<Vec<TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeElXyEl>>,
    dynamic: TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeElDynamic,
}
impl TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeEl {
    #[doc = "Set the field `end_time_offset`.\nThe time to end the fade animation, in seconds."]
    pub fn set_end_time_offset(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time_offset = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time_offset`.\nThe time to start the fade animation, in seconds."]
    pub fn set_start_time_offset(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time_offset = Some(v.into());
        self
    }
    #[doc = "Set the field `xy`.\n"]
    pub fn set_xy(
        mut self,
        v: impl Into<
            BlockAssignable<TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeElXyEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.xy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.xy = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeEl {
    type O = BlockAssignable<TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeEl {
    #[doc = "Required. Type of fade animation: 'FADE_IN' or 'FADE_OUT'.\nThe possible values are:\n\n* 'FADE_TYPE_UNSPECIFIED': The fade type is not specified.\n\n* 'FADE_IN': Fade the overlay object into view.\n\n* 'FADE_OUT': Fade the overlay object out of view. Possible values: [\"FADE_TYPE_UNSPECIFIED\", \"FADE_IN\", \"FADE_OUT\"]"]
    pub fade_type: PrimField<String>,
}
impl BuildTranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeEl {
        TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeEl {
            end_time_offset: core::default::Default::default(),
            fade_type: self.fade_type,
            start_time_offset: core::default::Default::default(),
            xy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeElRef {
        TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_time_offset` after provisioning.\nThe time to end the fade animation, in seconds."]
    pub fn end_time_offset(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.end_time_offset", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fade_type` after provisioning.\nRequired. Type of fade animation: 'FADE_IN' or 'FADE_OUT'.\nThe possible values are:\n\n* 'FADE_TYPE_UNSPECIFIED': The fade type is not specified.\n\n* 'FADE_IN': Fade the overlay object into view.\n\n* 'FADE_OUT': Fade the overlay object out of view. Possible values: [\"FADE_TYPE_UNSPECIFIED\", \"FADE_IN\", \"FADE_OUT\"]"]
    pub fn fade_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fade_type", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time_offset` after provisioning.\nThe time to start the fade animation, in seconds."]
    pub fn start_time_offset(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_time_offset", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `xy` after provisioning.\n"]
    pub fn xy(
        &self,
    ) -> ListRef<TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeElXyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.xy", self.base))
    }
}
#[derive(Serialize, Default)]
struct TranscoderJobTemplateConfigElOverlaysElAnimationsElDynamic {
    animation_fade:
        Option<DynamicBlock<TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeEl>>,
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElOverlaysElAnimationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    animation_fade: Option<Vec<TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeEl>>,
    dynamic: TranscoderJobTemplateConfigElOverlaysElAnimationsElDynamic,
}
impl TranscoderJobTemplateConfigElOverlaysElAnimationsEl {
    #[doc = "Set the field `animation_fade`.\n"]
    pub fn set_animation_fade(
        mut self,
        v: impl Into<
            BlockAssignable<TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.animation_fade = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.animation_fade = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for TranscoderJobTemplateConfigElOverlaysElAnimationsEl {
    type O = BlockAssignable<TranscoderJobTemplateConfigElOverlaysElAnimationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElOverlaysElAnimationsEl {}
impl BuildTranscoderJobTemplateConfigElOverlaysElAnimationsEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElOverlaysElAnimationsEl {
        TranscoderJobTemplateConfigElOverlaysElAnimationsEl {
            animation_fade: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct TranscoderJobTemplateConfigElOverlaysElAnimationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElOverlaysElAnimationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobTemplateConfigElOverlaysElAnimationsElRef {
        TranscoderJobTemplateConfigElOverlaysElAnimationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElOverlaysElAnimationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `animation_fade` after provisioning.\n"]
    pub fn animation_fade(
        &self,
    ) -> ListRef<TranscoderJobTemplateConfigElOverlaysElAnimationsElAnimationFadeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.animation_fade", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElOverlaysElImageEl {
    uri: PrimField<String>,
}
impl TranscoderJobTemplateConfigElOverlaysElImageEl {}
impl ToListMappable for TranscoderJobTemplateConfigElOverlaysElImageEl {
    type O = BlockAssignable<TranscoderJobTemplateConfigElOverlaysElImageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElOverlaysElImageEl {
    #[doc = "URI of the image in Cloud Storage. For example, gs://bucket/inputs/image.png."]
    pub uri: PrimField<String>,
}
impl BuildTranscoderJobTemplateConfigElOverlaysElImageEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElOverlaysElImageEl {
        TranscoderJobTemplateConfigElOverlaysElImageEl { uri: self.uri }
    }
}
pub struct TranscoderJobTemplateConfigElOverlaysElImageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElOverlaysElImageElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobTemplateConfigElOverlaysElImageElRef {
        TranscoderJobTemplateConfigElOverlaysElImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElOverlaysElImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nURI of the image in Cloud Storage. For example, gs://bucket/inputs/image.png."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct TranscoderJobTemplateConfigElOverlaysElDynamic {
    animations: Option<DynamicBlock<TranscoderJobTemplateConfigElOverlaysElAnimationsEl>>,
    image: Option<DynamicBlock<TranscoderJobTemplateConfigElOverlaysElImageEl>>,
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElOverlaysEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    animations: Option<Vec<TranscoderJobTemplateConfigElOverlaysElAnimationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Vec<TranscoderJobTemplateConfigElOverlaysElImageEl>>,
    dynamic: TranscoderJobTemplateConfigElOverlaysElDynamic,
}
impl TranscoderJobTemplateConfigElOverlaysEl {
    #[doc = "Set the field `animations`.\n"]
    pub fn set_animations(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobTemplateConfigElOverlaysElAnimationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.animations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.animations = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `image`.\n"]
    pub fn set_image(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobTemplateConfigElOverlaysElImageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.image = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.image = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for TranscoderJobTemplateConfigElOverlaysEl {
    type O = BlockAssignable<TranscoderJobTemplateConfigElOverlaysEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElOverlaysEl {}
impl BuildTranscoderJobTemplateConfigElOverlaysEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElOverlaysEl {
        TranscoderJobTemplateConfigElOverlaysEl {
            animations: core::default::Default::default(),
            image: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct TranscoderJobTemplateConfigElOverlaysElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElOverlaysElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobTemplateConfigElOverlaysElRef {
        TranscoderJobTemplateConfigElOverlaysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElOverlaysElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `animations` after provisioning.\n"]
    pub fn animations(&self) -> ListRef<TranscoderJobTemplateConfigElOverlaysElAnimationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.animations", self.base))
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> ListRef<TranscoderJobTemplateConfigElOverlaysElImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image", self.base))
    }
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigElPubsubDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<PrimField<String>>,
}
impl TranscoderJobTemplateConfigElPubsubDestinationEl {
    #[doc = "Set the field `topic`.\nThe name of the Pub/Sub topic to publish job completion notification to. For example: projects/{project}/topics/{topic}."]
    pub fn set_topic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.topic = Some(v.into());
        self
    }
}
impl ToListMappable for TranscoderJobTemplateConfigElPubsubDestinationEl {
    type O = BlockAssignable<TranscoderJobTemplateConfigElPubsubDestinationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigElPubsubDestinationEl {}
impl BuildTranscoderJobTemplateConfigElPubsubDestinationEl {
    pub fn build(self) -> TranscoderJobTemplateConfigElPubsubDestinationEl {
        TranscoderJobTemplateConfigElPubsubDestinationEl {
            topic: core::default::Default::default(),
        }
    }
}
pub struct TranscoderJobTemplateConfigElPubsubDestinationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElPubsubDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobTemplateConfigElPubsubDestinationElRef {
        TranscoderJobTemplateConfigElPubsubDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElPubsubDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\nThe name of the Pub/Sub topic to publish job completion notification to. For example: projects/{project}/topics/{topic}."]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}
#[derive(Serialize, Default)]
struct TranscoderJobTemplateConfigElDynamic {
    ad_breaks: Option<DynamicBlock<TranscoderJobTemplateConfigElAdBreaksEl>>,
    edit_list: Option<DynamicBlock<TranscoderJobTemplateConfigElEditListEl>>,
    elementary_streams: Option<DynamicBlock<TranscoderJobTemplateConfigElElementaryStreamsEl>>,
    encryptions: Option<DynamicBlock<TranscoderJobTemplateConfigElEncryptionsEl>>,
    inputs: Option<DynamicBlock<TranscoderJobTemplateConfigElInputsEl>>,
    manifests: Option<DynamicBlock<TranscoderJobTemplateConfigElManifestsEl>>,
    mux_streams: Option<DynamicBlock<TranscoderJobTemplateConfigElMuxStreamsEl>>,
    output: Option<DynamicBlock<TranscoderJobTemplateConfigElOutputEl>>,
    overlays: Option<DynamicBlock<TranscoderJobTemplateConfigElOverlaysEl>>,
    pubsub_destination: Option<DynamicBlock<TranscoderJobTemplateConfigElPubsubDestinationEl>>,
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ad_breaks: Option<Vec<TranscoderJobTemplateConfigElAdBreaksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edit_list: Option<Vec<TranscoderJobTemplateConfigElEditListEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elementary_streams: Option<Vec<TranscoderJobTemplateConfigElElementaryStreamsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryptions: Option<Vec<TranscoderJobTemplateConfigElEncryptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inputs: Option<Vec<TranscoderJobTemplateConfigElInputsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manifests: Option<Vec<TranscoderJobTemplateConfigElManifestsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mux_streams: Option<Vec<TranscoderJobTemplateConfigElMuxStreamsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<Vec<TranscoderJobTemplateConfigElOutputEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overlays: Option<Vec<TranscoderJobTemplateConfigElOverlaysEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pubsub_destination: Option<Vec<TranscoderJobTemplateConfigElPubsubDestinationEl>>,
    dynamic: TranscoderJobTemplateConfigElDynamic,
}
impl TranscoderJobTemplateConfigEl {
    #[doc = "Set the field `ad_breaks`.\n"]
    pub fn set_ad_breaks(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobTemplateConfigElAdBreaksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ad_breaks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ad_breaks = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `edit_list`.\n"]
    pub fn set_edit_list(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobTemplateConfigElEditListEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.edit_list = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.edit_list = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `elementary_streams`.\n"]
    pub fn set_elementary_streams(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobTemplateConfigElElementaryStreamsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.elementary_streams = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.elementary_streams = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `encryptions`.\n"]
    pub fn set_encryptions(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobTemplateConfigElEncryptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.encryptions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.encryptions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `inputs`.\n"]
    pub fn set_inputs(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobTemplateConfigElInputsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.inputs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.inputs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `manifests`.\n"]
    pub fn set_manifests(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobTemplateConfigElManifestsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.manifests = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.manifests = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `mux_streams`.\n"]
    pub fn set_mux_streams(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobTemplateConfigElMuxStreamsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mux_streams = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mux_streams = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `output`.\n"]
    pub fn set_output(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobTemplateConfigElOutputEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.output = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.output = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `overlays`.\n"]
    pub fn set_overlays(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobTemplateConfigElOverlaysEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.overlays = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.overlays = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `pubsub_destination`.\n"]
    pub fn set_pubsub_destination(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobTemplateConfigElPubsubDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pubsub_destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pubsub_destination = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for TranscoderJobTemplateConfigEl {
    type O = BlockAssignable<TranscoderJobTemplateConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateConfigEl {}
impl BuildTranscoderJobTemplateConfigEl {
    pub fn build(self) -> TranscoderJobTemplateConfigEl {
        TranscoderJobTemplateConfigEl {
            ad_breaks: core::default::Default::default(),
            edit_list: core::default::Default::default(),
            elementary_streams: core::default::Default::default(),
            encryptions: core::default::Default::default(),
            inputs: core::default::Default::default(),
            manifests: core::default::Default::default(),
            mux_streams: core::default::Default::default(),
            output: core::default::Default::default(),
            overlays: core::default::Default::default(),
            pubsub_destination: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct TranscoderJobTemplateConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateConfigElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobTemplateConfigElRef {
        TranscoderJobTemplateConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ad_breaks` after provisioning.\n"]
    pub fn ad_breaks(&self) -> ListRef<TranscoderJobTemplateConfigElAdBreaksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ad_breaks", self.base))
    }
    #[doc = "Get a reference to the value of field `edit_list` after provisioning.\n"]
    pub fn edit_list(&self) -> ListRef<TranscoderJobTemplateConfigElEditListElRef> {
        ListRef::new(self.shared().clone(), format!("{}.edit_list", self.base))
    }
    #[doc = "Get a reference to the value of field `elementary_streams` after provisioning.\n"]
    pub fn elementary_streams(
        &self,
    ) -> ListRef<TranscoderJobTemplateConfigElElementaryStreamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.elementary_streams", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encryptions` after provisioning.\n"]
    pub fn encryptions(&self) -> ListRef<TranscoderJobTemplateConfigElEncryptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryptions", self.base))
    }
    #[doc = "Get a reference to the value of field `inputs` after provisioning.\n"]
    pub fn inputs(&self) -> ListRef<TranscoderJobTemplateConfigElInputsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.inputs", self.base))
    }
    #[doc = "Get a reference to the value of field `manifests` after provisioning.\n"]
    pub fn manifests(&self) -> ListRef<TranscoderJobTemplateConfigElManifestsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.manifests", self.base))
    }
    #[doc = "Get a reference to the value of field `mux_streams` after provisioning.\n"]
    pub fn mux_streams(&self) -> ListRef<TranscoderJobTemplateConfigElMuxStreamsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mux_streams", self.base))
    }
    #[doc = "Get a reference to the value of field `output` after provisioning.\n"]
    pub fn output(&self) -> ListRef<TranscoderJobTemplateConfigElOutputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.output", self.base))
    }
    #[doc = "Get a reference to the value of field `overlays` after provisioning.\n"]
    pub fn overlays(&self) -> ListRef<TranscoderJobTemplateConfigElOverlaysElRef> {
        ListRef::new(self.shared().clone(), format!("{}.overlays", self.base))
    }
    #[doc = "Get a reference to the value of field `pubsub_destination` after provisioning.\n"]
    pub fn pubsub_destination(
        &self,
    ) -> ListRef<TranscoderJobTemplateConfigElPubsubDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pubsub_destination", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct TranscoderJobTemplateTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl TranscoderJobTemplateTimeoutsEl {
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
impl ToListMappable for TranscoderJobTemplateTimeoutsEl {
    type O = BlockAssignable<TranscoderJobTemplateTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTemplateTimeoutsEl {}
impl BuildTranscoderJobTemplateTimeoutsEl {
    pub fn build(self) -> TranscoderJobTemplateTimeoutsEl {
        TranscoderJobTemplateTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct TranscoderJobTemplateTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTemplateTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobTemplateTimeoutsElRef {
        TranscoderJobTemplateTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTemplateTimeoutsElRef {
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
struct TranscoderJobTemplateDynamic {
    config: Option<DynamicBlock<TranscoderJobTemplateConfigEl>>,
}
