use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct TranscoderJobData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Vec<TranscoderJobConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<TranscoderJobTimeoutsEl>,
    dynamic: TranscoderJobDynamic,
}
struct TranscoderJob_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TranscoderJobData>,
}
#[derive(Clone)]
pub struct TranscoderJob(Rc<TranscoderJob_>);
impl TranscoderJob {
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
    #[doc = "Set the field `labels`.\nThe labels associated with this job. You can use these to organize and group your jobs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `template_id`.\nSpecify the templateId to use for populating Job.config.\nThe default is preset/web-hd, which is the only supported preset."]
    pub fn set_template_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().template_id = Some(v.into());
        self
    }
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(self, v: impl Into<BlockAssignable<TranscoderJobConfigEl>>) -> Self {
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
    pub fn set_timeouts(self, v: impl Into<TranscoderJobTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time the job was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\nThe time the transcoding finished."]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.end_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels associated with this job. You can use these to organize and group your jobs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the transcoding job resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the job."]
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
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\nThe time the transcoding started."]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the job."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template_id` after provisioning.\nSpecify the templateId to use for populating Job.config.\nThe default is preset/web-hd, which is the only supported preset."]
    pub fn template_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_id", self.extract_ref()),
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
    pub fn config(&self) -> ListRef<TranscoderJobConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> TranscoderJobTimeoutsElRef {
        TranscoderJobTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for TranscoderJob {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for TranscoderJob {}
impl ToListMappable for TranscoderJob {
    type O = ListRef<TranscoderJobRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for TranscoderJob_ {
    fn extract_resource_type(&self) -> String {
        "google_transcoder_job".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildTranscoderJob {
    pub tf_id: String,
    #[doc = "The location of the transcoding job resource."]
    pub location: PrimField<String>,
}
impl BuildTranscoderJob {
    pub fn build(self, stack: &mut Stack) -> TranscoderJob {
        let out = TranscoderJob(Rc::new(TranscoderJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TranscoderJobData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                template_id: core::default::Default::default(),
                config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct TranscoderJobRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl TranscoderJobRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time the job was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\nThe time the transcoding finished."]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.end_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels associated with this job. You can use these to organize and group your jobs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the transcoding job resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the job."]
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
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\nThe time the transcoding started."]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the job."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template_id` after provisioning.\nSpecify the templateId to use for populating Job.config.\nThe default is preset/web-hd, which is the only supported preset."]
    pub fn template_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_id", self.extract_ref()),
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
    pub fn config(&self) -> ListRef<TranscoderJobConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> TranscoderJobTimeoutsElRef {
        TranscoderJobTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct TranscoderJobConfigElAdBreaksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time_offset: Option<PrimField<String>>,
}
impl TranscoderJobConfigElAdBreaksEl {
    #[doc = "Set the field `start_time_offset`.\nStart time in seconds for the ad break, relative to the output file timeline"]
    pub fn set_start_time_offset(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time_offset = Some(v.into());
        self
    }
}
impl ToListMappable for TranscoderJobConfigElAdBreaksEl {
    type O = BlockAssignable<TranscoderJobConfigElAdBreaksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElAdBreaksEl {}
impl BuildTranscoderJobConfigElAdBreaksEl {
    pub fn build(self) -> TranscoderJobConfigElAdBreaksEl {
        TranscoderJobConfigElAdBreaksEl {
            start_time_offset: core::default::Default::default(),
        }
    }
}
pub struct TranscoderJobConfigElAdBreaksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElAdBreaksElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobConfigElAdBreaksElRef {
        TranscoderJobConfigElAdBreaksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElAdBreaksElRef {
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
pub struct TranscoderJobConfigElEditListEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    inputs: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time_offset: Option<PrimField<String>>,
}
impl TranscoderJobConfigElEditListEl {
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
    #[doc = "Set the field `start_time_offset`.\nStart time in seconds for the atom, relative to the input file timeline. The default is '0s'."]
    pub fn set_start_time_offset(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time_offset = Some(v.into());
        self
    }
}
impl ToListMappable for TranscoderJobConfigElEditListEl {
    type O = BlockAssignable<TranscoderJobConfigElEditListEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElEditListEl {}
impl BuildTranscoderJobConfigElEditListEl {
    pub fn build(self) -> TranscoderJobConfigElEditListEl {
        TranscoderJobConfigElEditListEl {
            inputs: core::default::Default::default(),
            key: core::default::Default::default(),
            start_time_offset: core::default::Default::default(),
        }
    }
}
pub struct TranscoderJobConfigElEditListElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElEditListElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobConfigElEditListElRef {
        TranscoderJobConfigElEditListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElEditListElRef {
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
    #[doc = "Get a reference to the value of field `start_time_offset` after provisioning.\nStart time in seconds for the atom, relative to the input file timeline. The default is '0s'."]
    pub fn start_time_offset(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_time_offset", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct TranscoderJobConfigElElementaryStreamsElAudioStreamEl {
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
impl TranscoderJobConfigElElementaryStreamsElAudioStreamEl {
    #[doc = "Set the field `channel_count`.\nNumber of audio channels. The default is '2'."]
    pub fn set_channel_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.channel_count = Some(v.into());
        self
    }
    #[doc = "Set the field `channel_layout`.\nA list of channel names specifying layout of the audio channels. The default is [\"fl\", \"fr\"]."]
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
impl ToListMappable for TranscoderJobConfigElElementaryStreamsElAudioStreamEl {
    type O = BlockAssignable<TranscoderJobConfigElElementaryStreamsElAudioStreamEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElElementaryStreamsElAudioStreamEl {
    #[doc = "Audio bitrate in bits per second."]
    pub bitrate_bps: PrimField<f64>,
}
impl BuildTranscoderJobConfigElElementaryStreamsElAudioStreamEl {
    pub fn build(self) -> TranscoderJobConfigElElementaryStreamsElAudioStreamEl {
        TranscoderJobConfigElElementaryStreamsElAudioStreamEl {
            bitrate_bps: self.bitrate_bps,
            channel_count: core::default::Default::default(),
            channel_layout: core::default::Default::default(),
            codec: core::default::Default::default(),
            sample_rate_hertz: core::default::Default::default(),
        }
    }
}
pub struct TranscoderJobConfigElElementaryStreamsElAudioStreamElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElElementaryStreamsElAudioStreamElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobConfigElElementaryStreamsElAudioStreamElRef {
        TranscoderJobConfigElElementaryStreamsElAudioStreamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElElementaryStreamsElAudioStreamElRef {
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
    #[doc = "Get a reference to the value of field `channel_layout` after provisioning.\nA list of channel names specifying layout of the audio channels. The default is [\"fl\", \"fr\"]."]
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
pub struct TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElHlgEl {}
impl TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElHlgEl {}
impl ToListMappable for TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElHlgEl {
    type O = BlockAssignable<TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElHlgEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElHlgEl {}
impl BuildTranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElHlgEl {
    pub fn build(self) -> TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElHlgEl {
        TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElHlgEl {}
    }
}
pub struct TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElHlgElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElHlgElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElHlgElRef {
        TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElHlgElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElHlgElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElSdrEl {}
impl TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElSdrEl {}
impl ToListMappable for TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElSdrEl {
    type O = BlockAssignable<TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElSdrEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElSdrEl {}
impl BuildTranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElSdrEl {
    pub fn build(self) -> TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElSdrEl {
        TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElSdrEl {}
    }
}
pub struct TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElSdrElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElSdrElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElSdrElRef {
        TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElSdrElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElSdrElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElDynamic {
    hlg: Option<DynamicBlock<TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElHlgEl>>,
    sdr: Option<DynamicBlock<TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElSdrEl>>,
}
#[derive(Serialize)]
pub struct TranscoderJobConfigElElementaryStreamsElVideoStreamElH264El {
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
    hlg: Option<Vec<TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElHlgEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sdr: Option<Vec<TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElSdrEl>>,
    dynamic: TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElDynamic,
}
impl TranscoderJobConfigElElementaryStreamsElVideoStreamElH264El {
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
        v: impl Into<BlockAssignable<TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElHlgEl>>,
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
        v: impl Into<BlockAssignable<TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElSdrEl>>,
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
impl ToListMappable for TranscoderJobConfigElElementaryStreamsElVideoStreamElH264El {
    type O = BlockAssignable<TranscoderJobConfigElElementaryStreamsElVideoStreamElH264El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElElementaryStreamsElVideoStreamElH264El {
    #[doc = "The video bitrate in bits per second."]
    pub bitrate_bps: PrimField<f64>,
    #[doc = "The target video frame rate in frames per second (FPS)."]
    pub frame_rate: PrimField<f64>,
}
impl BuildTranscoderJobConfigElElementaryStreamsElVideoStreamElH264El {
    pub fn build(self) -> TranscoderJobConfigElElementaryStreamsElVideoStreamElH264El {
        TranscoderJobConfigElElementaryStreamsElVideoStreamElH264El {
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
pub struct TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElRef {
        TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElRef {
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
    ) -> ListRef<TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElHlgElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hlg", self.base))
    }
    #[doc = "Get a reference to the value of field `sdr` after provisioning.\n"]
    pub fn sdr(
        &self,
    ) -> ListRef<TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElSdrElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sdr", self.base))
    }
}
#[derive(Serialize, Default)]
struct TranscoderJobConfigElElementaryStreamsElVideoStreamElDynamic {
    h264: Option<DynamicBlock<TranscoderJobConfigElElementaryStreamsElVideoStreamElH264El>>,
}
#[derive(Serialize)]
pub struct TranscoderJobConfigElElementaryStreamsElVideoStreamEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    h264: Option<Vec<TranscoderJobConfigElElementaryStreamsElVideoStreamElH264El>>,
    dynamic: TranscoderJobConfigElElementaryStreamsElVideoStreamElDynamic,
}
impl TranscoderJobConfigElElementaryStreamsElVideoStreamEl {
    #[doc = "Set the field `h264`.\n"]
    pub fn set_h264(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobConfigElElementaryStreamsElVideoStreamElH264El>>,
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
impl ToListMappable for TranscoderJobConfigElElementaryStreamsElVideoStreamEl {
    type O = BlockAssignable<TranscoderJobConfigElElementaryStreamsElVideoStreamEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElElementaryStreamsElVideoStreamEl {}
impl BuildTranscoderJobConfigElElementaryStreamsElVideoStreamEl {
    pub fn build(self) -> TranscoderJobConfigElElementaryStreamsElVideoStreamEl {
        TranscoderJobConfigElElementaryStreamsElVideoStreamEl {
            h264: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct TranscoderJobConfigElElementaryStreamsElVideoStreamElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElElementaryStreamsElVideoStreamElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobConfigElElementaryStreamsElVideoStreamElRef {
        TranscoderJobConfigElElementaryStreamsElVideoStreamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElElementaryStreamsElVideoStreamElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `h264` after provisioning.\n"]
    pub fn h264(&self) -> ListRef<TranscoderJobConfigElElementaryStreamsElVideoStreamElH264ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.h264", self.base))
    }
}
#[derive(Serialize, Default)]
struct TranscoderJobConfigElElementaryStreamsElDynamic {
    audio_stream: Option<DynamicBlock<TranscoderJobConfigElElementaryStreamsElAudioStreamEl>>,
    video_stream: Option<DynamicBlock<TranscoderJobConfigElElementaryStreamsElVideoStreamEl>>,
}
#[derive(Serialize)]
pub struct TranscoderJobConfigElElementaryStreamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_stream: Option<Vec<TranscoderJobConfigElElementaryStreamsElAudioStreamEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_stream: Option<Vec<TranscoderJobConfigElElementaryStreamsElVideoStreamEl>>,
    dynamic: TranscoderJobConfigElElementaryStreamsElDynamic,
}
impl TranscoderJobConfigElElementaryStreamsEl {
    #[doc = "Set the field `key`.\nA unique key for this atom."]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `audio_stream`.\n"]
    pub fn set_audio_stream(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobConfigElElementaryStreamsElAudioStreamEl>>,
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
        v: impl Into<BlockAssignable<TranscoderJobConfigElElementaryStreamsElVideoStreamEl>>,
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
impl ToListMappable for TranscoderJobConfigElElementaryStreamsEl {
    type O = BlockAssignable<TranscoderJobConfigElElementaryStreamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElElementaryStreamsEl {}
impl BuildTranscoderJobConfigElElementaryStreamsEl {
    pub fn build(self) -> TranscoderJobConfigElElementaryStreamsEl {
        TranscoderJobConfigElElementaryStreamsEl {
            key: core::default::Default::default(),
            audio_stream: core::default::Default::default(),
            video_stream: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct TranscoderJobConfigElElementaryStreamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElElementaryStreamsElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobConfigElElementaryStreamsElRef {
        TranscoderJobConfigElElementaryStreamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElElementaryStreamsElRef {
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
    ) -> ListRef<TranscoderJobConfigElElementaryStreamsElAudioStreamElRef> {
        ListRef::new(self.shared().clone(), format!("{}.audio_stream", self.base))
    }
    #[doc = "Get a reference to the value of field `video_stream` after provisioning.\n"]
    pub fn video_stream(
        &self,
    ) -> ListRef<TranscoderJobConfigElElementaryStreamsElVideoStreamElRef> {
        ListRef::new(self.shared().clone(), format!("{}.video_stream", self.base))
    }
}
#[derive(Serialize)]
pub struct TranscoderJobConfigElEncryptionsElAes128El {}
impl TranscoderJobConfigElEncryptionsElAes128El {}
impl ToListMappable for TranscoderJobConfigElEncryptionsElAes128El {
    type O = BlockAssignable<TranscoderJobConfigElEncryptionsElAes128El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElEncryptionsElAes128El {}
impl BuildTranscoderJobConfigElEncryptionsElAes128El {
    pub fn build(self) -> TranscoderJobConfigElEncryptionsElAes128El {
        TranscoderJobConfigElEncryptionsElAes128El {}
    }
}
pub struct TranscoderJobConfigElEncryptionsElAes128ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElEncryptionsElAes128ElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobConfigElEncryptionsElAes128ElRef {
        TranscoderJobConfigElEncryptionsElAes128ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElEncryptionsElAes128ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct TranscoderJobConfigElEncryptionsElDrmSystemsElClearkeyEl {}
impl TranscoderJobConfigElEncryptionsElDrmSystemsElClearkeyEl {}
impl ToListMappable for TranscoderJobConfigElEncryptionsElDrmSystemsElClearkeyEl {
    type O = BlockAssignable<TranscoderJobConfigElEncryptionsElDrmSystemsElClearkeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElEncryptionsElDrmSystemsElClearkeyEl {}
impl BuildTranscoderJobConfigElEncryptionsElDrmSystemsElClearkeyEl {
    pub fn build(self) -> TranscoderJobConfigElEncryptionsElDrmSystemsElClearkeyEl {
        TranscoderJobConfigElEncryptionsElDrmSystemsElClearkeyEl {}
    }
}
pub struct TranscoderJobConfigElEncryptionsElDrmSystemsElClearkeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElEncryptionsElDrmSystemsElClearkeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobConfigElEncryptionsElDrmSystemsElClearkeyElRef {
        TranscoderJobConfigElEncryptionsElDrmSystemsElClearkeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElEncryptionsElDrmSystemsElClearkeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct TranscoderJobConfigElEncryptionsElDrmSystemsElFairplayEl {}
impl TranscoderJobConfigElEncryptionsElDrmSystemsElFairplayEl {}
impl ToListMappable for TranscoderJobConfigElEncryptionsElDrmSystemsElFairplayEl {
    type O = BlockAssignable<TranscoderJobConfigElEncryptionsElDrmSystemsElFairplayEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElEncryptionsElDrmSystemsElFairplayEl {}
impl BuildTranscoderJobConfigElEncryptionsElDrmSystemsElFairplayEl {
    pub fn build(self) -> TranscoderJobConfigElEncryptionsElDrmSystemsElFairplayEl {
        TranscoderJobConfigElEncryptionsElDrmSystemsElFairplayEl {}
    }
}
pub struct TranscoderJobConfigElEncryptionsElDrmSystemsElFairplayElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElEncryptionsElDrmSystemsElFairplayElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobConfigElEncryptionsElDrmSystemsElFairplayElRef {
        TranscoderJobConfigElEncryptionsElDrmSystemsElFairplayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElEncryptionsElDrmSystemsElFairplayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct TranscoderJobConfigElEncryptionsElDrmSystemsElPlayreadyEl {}
impl TranscoderJobConfigElEncryptionsElDrmSystemsElPlayreadyEl {}
impl ToListMappable for TranscoderJobConfigElEncryptionsElDrmSystemsElPlayreadyEl {
    type O = BlockAssignable<TranscoderJobConfigElEncryptionsElDrmSystemsElPlayreadyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElEncryptionsElDrmSystemsElPlayreadyEl {}
impl BuildTranscoderJobConfigElEncryptionsElDrmSystemsElPlayreadyEl {
    pub fn build(self) -> TranscoderJobConfigElEncryptionsElDrmSystemsElPlayreadyEl {
        TranscoderJobConfigElEncryptionsElDrmSystemsElPlayreadyEl {}
    }
}
pub struct TranscoderJobConfigElEncryptionsElDrmSystemsElPlayreadyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElEncryptionsElDrmSystemsElPlayreadyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobConfigElEncryptionsElDrmSystemsElPlayreadyElRef {
        TranscoderJobConfigElEncryptionsElDrmSystemsElPlayreadyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElEncryptionsElDrmSystemsElPlayreadyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct TranscoderJobConfigElEncryptionsElDrmSystemsElWidevineEl {}
impl TranscoderJobConfigElEncryptionsElDrmSystemsElWidevineEl {}
impl ToListMappable for TranscoderJobConfigElEncryptionsElDrmSystemsElWidevineEl {
    type O = BlockAssignable<TranscoderJobConfigElEncryptionsElDrmSystemsElWidevineEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElEncryptionsElDrmSystemsElWidevineEl {}
impl BuildTranscoderJobConfigElEncryptionsElDrmSystemsElWidevineEl {
    pub fn build(self) -> TranscoderJobConfigElEncryptionsElDrmSystemsElWidevineEl {
        TranscoderJobConfigElEncryptionsElDrmSystemsElWidevineEl {}
    }
}
pub struct TranscoderJobConfigElEncryptionsElDrmSystemsElWidevineElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElEncryptionsElDrmSystemsElWidevineElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobConfigElEncryptionsElDrmSystemsElWidevineElRef {
        TranscoderJobConfigElEncryptionsElDrmSystemsElWidevineElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElEncryptionsElDrmSystemsElWidevineElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct TranscoderJobConfigElEncryptionsElDrmSystemsElDynamic {
    clearkey: Option<DynamicBlock<TranscoderJobConfigElEncryptionsElDrmSystemsElClearkeyEl>>,
    fairplay: Option<DynamicBlock<TranscoderJobConfigElEncryptionsElDrmSystemsElFairplayEl>>,
    playready: Option<DynamicBlock<TranscoderJobConfigElEncryptionsElDrmSystemsElPlayreadyEl>>,
    widevine: Option<DynamicBlock<TranscoderJobConfigElEncryptionsElDrmSystemsElWidevineEl>>,
}
#[derive(Serialize)]
pub struct TranscoderJobConfigElEncryptionsElDrmSystemsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    clearkey: Option<Vec<TranscoderJobConfigElEncryptionsElDrmSystemsElClearkeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fairplay: Option<Vec<TranscoderJobConfigElEncryptionsElDrmSystemsElFairplayEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    playready: Option<Vec<TranscoderJobConfigElEncryptionsElDrmSystemsElPlayreadyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    widevine: Option<Vec<TranscoderJobConfigElEncryptionsElDrmSystemsElWidevineEl>>,
    dynamic: TranscoderJobConfigElEncryptionsElDrmSystemsElDynamic,
}
impl TranscoderJobConfigElEncryptionsElDrmSystemsEl {
    #[doc = "Set the field `clearkey`.\n"]
    pub fn set_clearkey(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobConfigElEncryptionsElDrmSystemsElClearkeyEl>>,
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
        v: impl Into<BlockAssignable<TranscoderJobConfigElEncryptionsElDrmSystemsElFairplayEl>>,
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
        v: impl Into<BlockAssignable<TranscoderJobConfigElEncryptionsElDrmSystemsElPlayreadyEl>>,
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
        v: impl Into<BlockAssignable<TranscoderJobConfigElEncryptionsElDrmSystemsElWidevineEl>>,
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
impl ToListMappable for TranscoderJobConfigElEncryptionsElDrmSystemsEl {
    type O = BlockAssignable<TranscoderJobConfigElEncryptionsElDrmSystemsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElEncryptionsElDrmSystemsEl {}
impl BuildTranscoderJobConfigElEncryptionsElDrmSystemsEl {
    pub fn build(self) -> TranscoderJobConfigElEncryptionsElDrmSystemsEl {
        TranscoderJobConfigElEncryptionsElDrmSystemsEl {
            clearkey: core::default::Default::default(),
            fairplay: core::default::Default::default(),
            playready: core::default::Default::default(),
            widevine: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct TranscoderJobConfigElEncryptionsElDrmSystemsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElEncryptionsElDrmSystemsElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobConfigElEncryptionsElDrmSystemsElRef {
        TranscoderJobConfigElEncryptionsElDrmSystemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElEncryptionsElDrmSystemsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `clearkey` after provisioning.\n"]
    pub fn clearkey(&self) -> ListRef<TranscoderJobConfigElEncryptionsElDrmSystemsElClearkeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.clearkey", self.base))
    }
    #[doc = "Get a reference to the value of field `fairplay` after provisioning.\n"]
    pub fn fairplay(&self) -> ListRef<TranscoderJobConfigElEncryptionsElDrmSystemsElFairplayElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fairplay", self.base))
    }
    #[doc = "Get a reference to the value of field `playready` after provisioning.\n"]
    pub fn playready(
        &self,
    ) -> ListRef<TranscoderJobConfigElEncryptionsElDrmSystemsElPlayreadyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.playready", self.base))
    }
    #[doc = "Get a reference to the value of field `widevine` after provisioning.\n"]
    pub fn widevine(&self) -> ListRef<TranscoderJobConfigElEncryptionsElDrmSystemsElWidevineElRef> {
        ListRef::new(self.shared().clone(), format!("{}.widevine", self.base))
    }
}
#[derive(Serialize)]
pub struct TranscoderJobConfigElEncryptionsElMpegCencEl {
    scheme: PrimField<String>,
}
impl TranscoderJobConfigElEncryptionsElMpegCencEl {}
impl ToListMappable for TranscoderJobConfigElEncryptionsElMpegCencEl {
    type O = BlockAssignable<TranscoderJobConfigElEncryptionsElMpegCencEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElEncryptionsElMpegCencEl {
    #[doc = "Specify the encryption scheme."]
    pub scheme: PrimField<String>,
}
impl BuildTranscoderJobConfigElEncryptionsElMpegCencEl {
    pub fn build(self) -> TranscoderJobConfigElEncryptionsElMpegCencEl {
        TranscoderJobConfigElEncryptionsElMpegCencEl {
            scheme: self.scheme,
        }
    }
}
pub struct TranscoderJobConfigElEncryptionsElMpegCencElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElEncryptionsElMpegCencElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobConfigElEncryptionsElMpegCencElRef {
        TranscoderJobConfigElEncryptionsElMpegCencElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElEncryptionsElMpegCencElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scheme` after provisioning.\nSpecify the encryption scheme."]
    pub fn scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scheme", self.base))
    }
}
#[derive(Serialize)]
pub struct TranscoderJobConfigElEncryptionsElSampleAesEl {}
impl TranscoderJobConfigElEncryptionsElSampleAesEl {}
impl ToListMappable for TranscoderJobConfigElEncryptionsElSampleAesEl {
    type O = BlockAssignable<TranscoderJobConfigElEncryptionsElSampleAesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElEncryptionsElSampleAesEl {}
impl BuildTranscoderJobConfigElEncryptionsElSampleAesEl {
    pub fn build(self) -> TranscoderJobConfigElEncryptionsElSampleAesEl {
        TranscoderJobConfigElEncryptionsElSampleAesEl {}
    }
}
pub struct TranscoderJobConfigElEncryptionsElSampleAesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElEncryptionsElSampleAesElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobConfigElEncryptionsElSampleAesElRef {
        TranscoderJobConfigElEncryptionsElSampleAesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElEncryptionsElSampleAesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct TranscoderJobConfigElEncryptionsElSecretManagerKeySourceEl {
    secret_version: PrimField<String>,
}
impl TranscoderJobConfigElEncryptionsElSecretManagerKeySourceEl {}
impl ToListMappable for TranscoderJobConfigElEncryptionsElSecretManagerKeySourceEl {
    type O = BlockAssignable<TranscoderJobConfigElEncryptionsElSecretManagerKeySourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElEncryptionsElSecretManagerKeySourceEl {
    #[doc = "The name of the Secret Version containing the encryption key in the following format: projects/{project}/secrets/{secret_id}/versions/{version_number}."]
    pub secret_version: PrimField<String>,
}
impl BuildTranscoderJobConfigElEncryptionsElSecretManagerKeySourceEl {
    pub fn build(self) -> TranscoderJobConfigElEncryptionsElSecretManagerKeySourceEl {
        TranscoderJobConfigElEncryptionsElSecretManagerKeySourceEl {
            secret_version: self.secret_version,
        }
    }
}
pub struct TranscoderJobConfigElEncryptionsElSecretManagerKeySourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElEncryptionsElSecretManagerKeySourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobConfigElEncryptionsElSecretManagerKeySourceElRef {
        TranscoderJobConfigElEncryptionsElSecretManagerKeySourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElEncryptionsElSecretManagerKeySourceElRef {
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
struct TranscoderJobConfigElEncryptionsElDynamic {
    aes128: Option<DynamicBlock<TranscoderJobConfigElEncryptionsElAes128El>>,
    drm_systems: Option<DynamicBlock<TranscoderJobConfigElEncryptionsElDrmSystemsEl>>,
    mpeg_cenc: Option<DynamicBlock<TranscoderJobConfigElEncryptionsElMpegCencEl>>,
    sample_aes: Option<DynamicBlock<TranscoderJobConfigElEncryptionsElSampleAesEl>>,
    secret_manager_key_source:
        Option<DynamicBlock<TranscoderJobConfigElEncryptionsElSecretManagerKeySourceEl>>,
}
#[derive(Serialize)]
pub struct TranscoderJobConfigElEncryptionsEl {
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aes128: Option<Vec<TranscoderJobConfigElEncryptionsElAes128El>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drm_systems: Option<Vec<TranscoderJobConfigElEncryptionsElDrmSystemsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mpeg_cenc: Option<Vec<TranscoderJobConfigElEncryptionsElMpegCencEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_aes: Option<Vec<TranscoderJobConfigElEncryptionsElSampleAesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_manager_key_source:
        Option<Vec<TranscoderJobConfigElEncryptionsElSecretManagerKeySourceEl>>,
    dynamic: TranscoderJobConfigElEncryptionsElDynamic,
}
impl TranscoderJobConfigElEncryptionsEl {
    #[doc = "Set the field `aes128`.\n"]
    pub fn set_aes128(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobConfigElEncryptionsElAes128El>>,
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
        v: impl Into<BlockAssignable<TranscoderJobConfigElEncryptionsElDrmSystemsEl>>,
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
        v: impl Into<BlockAssignable<TranscoderJobConfigElEncryptionsElMpegCencEl>>,
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
        v: impl Into<BlockAssignable<TranscoderJobConfigElEncryptionsElSampleAesEl>>,
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
        v: impl Into<BlockAssignable<TranscoderJobConfigElEncryptionsElSecretManagerKeySourceEl>>,
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
impl ToListMappable for TranscoderJobConfigElEncryptionsEl {
    type O = BlockAssignable<TranscoderJobConfigElEncryptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElEncryptionsEl {
    #[doc = "Identifier for this set of encryption options."]
    pub id: PrimField<String>,
}
impl BuildTranscoderJobConfigElEncryptionsEl {
    pub fn build(self) -> TranscoderJobConfigElEncryptionsEl {
        TranscoderJobConfigElEncryptionsEl {
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
pub struct TranscoderJobConfigElEncryptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElEncryptionsElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobConfigElEncryptionsElRef {
        TranscoderJobConfigElEncryptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElEncryptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier for this set of encryption options."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `aes128` after provisioning.\n"]
    pub fn aes128(&self) -> ListRef<TranscoderJobConfigElEncryptionsElAes128ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aes128", self.base))
    }
    #[doc = "Get a reference to the value of field `drm_systems` after provisioning.\n"]
    pub fn drm_systems(&self) -> ListRef<TranscoderJobConfigElEncryptionsElDrmSystemsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.drm_systems", self.base))
    }
    #[doc = "Get a reference to the value of field `mpeg_cenc` after provisioning.\n"]
    pub fn mpeg_cenc(&self) -> ListRef<TranscoderJobConfigElEncryptionsElMpegCencElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mpeg_cenc", self.base))
    }
    #[doc = "Get a reference to the value of field `sample_aes` after provisioning.\n"]
    pub fn sample_aes(&self) -> ListRef<TranscoderJobConfigElEncryptionsElSampleAesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sample_aes", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_manager_key_source` after provisioning.\n"]
    pub fn secret_manager_key_source(
        &self,
    ) -> ListRef<TranscoderJobConfigElEncryptionsElSecretManagerKeySourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secret_manager_key_source", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct TranscoderJobConfigElInputsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl TranscoderJobConfigElInputsEl {
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
impl ToListMappable for TranscoderJobConfigElInputsEl {
    type O = BlockAssignable<TranscoderJobConfigElInputsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElInputsEl {}
impl BuildTranscoderJobConfigElInputsEl {
    pub fn build(self) -> TranscoderJobConfigElInputsEl {
        TranscoderJobConfigElInputsEl {
            key: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}
pub struct TranscoderJobConfigElInputsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElInputsElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobConfigElInputsElRef {
        TranscoderJobConfigElInputsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElInputsElRef {
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
pub struct TranscoderJobConfigElManifestsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mux_streams: Option<ListField<PrimField<String>>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl TranscoderJobConfigElManifestsEl {
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
impl ToListMappable for TranscoderJobConfigElManifestsEl {
    type O = BlockAssignable<TranscoderJobConfigElManifestsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElManifestsEl {}
impl BuildTranscoderJobConfigElManifestsEl {
    pub fn build(self) -> TranscoderJobConfigElManifestsEl {
        TranscoderJobConfigElManifestsEl {
            file_name: core::default::Default::default(),
            mux_streams: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct TranscoderJobConfigElManifestsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElManifestsElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobConfigElManifestsElRef {
        TranscoderJobConfigElManifestsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElManifestsElRef {
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
pub struct TranscoderJobConfigElMuxStreamsElSegmentSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    segment_duration: Option<PrimField<String>>,
}
impl TranscoderJobConfigElMuxStreamsElSegmentSettingsEl {
    #[doc = "Set the field `segment_duration`.\nDuration of the segments in seconds. The default is '6.0s'."]
    pub fn set_segment_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.segment_duration = Some(v.into());
        self
    }
}
impl ToListMappable for TranscoderJobConfigElMuxStreamsElSegmentSettingsEl {
    type O = BlockAssignable<TranscoderJobConfigElMuxStreamsElSegmentSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElMuxStreamsElSegmentSettingsEl {}
impl BuildTranscoderJobConfigElMuxStreamsElSegmentSettingsEl {
    pub fn build(self) -> TranscoderJobConfigElMuxStreamsElSegmentSettingsEl {
        TranscoderJobConfigElMuxStreamsElSegmentSettingsEl {
            segment_duration: core::default::Default::default(),
        }
    }
}
pub struct TranscoderJobConfigElMuxStreamsElSegmentSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElMuxStreamsElSegmentSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobConfigElMuxStreamsElSegmentSettingsElRef {
        TranscoderJobConfigElMuxStreamsElSegmentSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElMuxStreamsElSegmentSettingsElRef {
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
struct TranscoderJobConfigElMuxStreamsElDynamic {
    segment_settings: Option<DynamicBlock<TranscoderJobConfigElMuxStreamsElSegmentSettingsEl>>,
}
#[derive(Serialize)]
pub struct TranscoderJobConfigElMuxStreamsEl {
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
    segment_settings: Option<Vec<TranscoderJobConfigElMuxStreamsElSegmentSettingsEl>>,
    dynamic: TranscoderJobConfigElMuxStreamsElDynamic,
}
impl TranscoderJobConfigElMuxStreamsEl {
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
        v: impl Into<BlockAssignable<TranscoderJobConfigElMuxStreamsElSegmentSettingsEl>>,
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
impl ToListMappable for TranscoderJobConfigElMuxStreamsEl {
    type O = BlockAssignable<TranscoderJobConfigElMuxStreamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElMuxStreamsEl {}
impl BuildTranscoderJobConfigElMuxStreamsEl {
    pub fn build(self) -> TranscoderJobConfigElMuxStreamsEl {
        TranscoderJobConfigElMuxStreamsEl {
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
pub struct TranscoderJobConfigElMuxStreamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElMuxStreamsElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobConfigElMuxStreamsElRef {
        TranscoderJobConfigElMuxStreamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElMuxStreamsElRef {
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
    ) -> ListRef<TranscoderJobConfigElMuxStreamsElSegmentSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.segment_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct TranscoderJobConfigElOutputEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl TranscoderJobConfigElOutputEl {
    #[doc = "Set the field `uri`.\nURI for the output file(s). For example, gs://my-bucket/outputs/."]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for TranscoderJobConfigElOutputEl {
    type O = BlockAssignable<TranscoderJobConfigElOutputEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElOutputEl {}
impl BuildTranscoderJobConfigElOutputEl {
    pub fn build(self) -> TranscoderJobConfigElOutputEl {
        TranscoderJobConfigElOutputEl {
            uri: core::default::Default::default(),
        }
    }
}
pub struct TranscoderJobConfigElOutputElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElOutputElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobConfigElOutputElRef {
        TranscoderJobConfigElOutputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElOutputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nURI for the output file(s). For example, gs://my-bucket/outputs/."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeElXyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<PrimField<f64>>,
}
impl TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeElXyEl {
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
impl ToListMappable for TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeElXyEl {
    type O = BlockAssignable<TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeElXyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElOverlaysElAnimationsElAnimationFadeElXyEl {}
impl BuildTranscoderJobConfigElOverlaysElAnimationsElAnimationFadeElXyEl {
    pub fn build(self) -> TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeElXyEl {
        TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeElXyEl {
            x: core::default::Default::default(),
            y: core::default::Default::default(),
        }
    }
}
pub struct TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeElXyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeElXyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeElXyElRef {
        TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeElXyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeElXyElRef {
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
struct TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeElDynamic {
    xy: Option<DynamicBlock<TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeElXyEl>>,
}
#[derive(Serialize)]
pub struct TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time_offset: Option<PrimField<String>>,
    fade_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time_offset: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    xy: Option<Vec<TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeElXyEl>>,
    dynamic: TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeElDynamic,
}
impl TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeEl {
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
        v: impl Into<BlockAssignable<TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeElXyEl>>,
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
impl ToListMappable for TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeEl {
    type O = BlockAssignable<TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElOverlaysElAnimationsElAnimationFadeEl {
    #[doc = "Required. Type of fade animation: 'FADE_IN' or 'FADE_OUT'.\nThe possible values are:\n\n* 'FADE_TYPE_UNSPECIFIED': The fade type is not specified.\n\n* 'FADE_IN': Fade the overlay object into view.\n\n* 'FADE_OUT': Fade the overlay object out of view. Possible values: [\"FADE_TYPE_UNSPECIFIED\", \"FADE_IN\", \"FADE_OUT\"]"]
    pub fade_type: PrimField<String>,
}
impl BuildTranscoderJobConfigElOverlaysElAnimationsElAnimationFadeEl {
    pub fn build(self) -> TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeEl {
        TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeEl {
            end_time_offset: core::default::Default::default(),
            fade_type: self.fade_type,
            start_time_offset: core::default::Default::default(),
            xy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeElRef {
        TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeElRef {
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
    pub fn xy(&self) -> ListRef<TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeElXyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.xy", self.base))
    }
}
#[derive(Serialize, Default)]
struct TranscoderJobConfigElOverlaysElAnimationsElDynamic {
    animation_fade:
        Option<DynamicBlock<TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeEl>>,
}
#[derive(Serialize)]
pub struct TranscoderJobConfigElOverlaysElAnimationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    animation_fade: Option<Vec<TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeEl>>,
    dynamic: TranscoderJobConfigElOverlaysElAnimationsElDynamic,
}
impl TranscoderJobConfigElOverlaysElAnimationsEl {
    #[doc = "Set the field `animation_fade`.\n"]
    pub fn set_animation_fade(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeEl>>,
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
impl ToListMappable for TranscoderJobConfigElOverlaysElAnimationsEl {
    type O = BlockAssignable<TranscoderJobConfigElOverlaysElAnimationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElOverlaysElAnimationsEl {}
impl BuildTranscoderJobConfigElOverlaysElAnimationsEl {
    pub fn build(self) -> TranscoderJobConfigElOverlaysElAnimationsEl {
        TranscoderJobConfigElOverlaysElAnimationsEl {
            animation_fade: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct TranscoderJobConfigElOverlaysElAnimationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElOverlaysElAnimationsElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobConfigElOverlaysElAnimationsElRef {
        TranscoderJobConfigElOverlaysElAnimationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElOverlaysElAnimationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `animation_fade` after provisioning.\n"]
    pub fn animation_fade(
        &self,
    ) -> ListRef<TranscoderJobConfigElOverlaysElAnimationsElAnimationFadeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.animation_fade", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct TranscoderJobConfigElOverlaysElImageEl {
    uri: PrimField<String>,
}
impl TranscoderJobConfigElOverlaysElImageEl {}
impl ToListMappable for TranscoderJobConfigElOverlaysElImageEl {
    type O = BlockAssignable<TranscoderJobConfigElOverlaysElImageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElOverlaysElImageEl {
    #[doc = "URI of the image in Cloud Storage. For example, gs://bucket/inputs/image.png."]
    pub uri: PrimField<String>,
}
impl BuildTranscoderJobConfigElOverlaysElImageEl {
    pub fn build(self) -> TranscoderJobConfigElOverlaysElImageEl {
        TranscoderJobConfigElOverlaysElImageEl { uri: self.uri }
    }
}
pub struct TranscoderJobConfigElOverlaysElImageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElOverlaysElImageElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobConfigElOverlaysElImageElRef {
        TranscoderJobConfigElOverlaysElImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElOverlaysElImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nURI of the image in Cloud Storage. For example, gs://bucket/inputs/image.png."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct TranscoderJobConfigElOverlaysElDynamic {
    animations: Option<DynamicBlock<TranscoderJobConfigElOverlaysElAnimationsEl>>,
    image: Option<DynamicBlock<TranscoderJobConfigElOverlaysElImageEl>>,
}
#[derive(Serialize)]
pub struct TranscoderJobConfigElOverlaysEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    animations: Option<Vec<TranscoderJobConfigElOverlaysElAnimationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Vec<TranscoderJobConfigElOverlaysElImageEl>>,
    dynamic: TranscoderJobConfigElOverlaysElDynamic,
}
impl TranscoderJobConfigElOverlaysEl {
    #[doc = "Set the field `animations`.\n"]
    pub fn set_animations(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobConfigElOverlaysElAnimationsEl>>,
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
        v: impl Into<BlockAssignable<TranscoderJobConfigElOverlaysElImageEl>>,
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
impl ToListMappable for TranscoderJobConfigElOverlaysEl {
    type O = BlockAssignable<TranscoderJobConfigElOverlaysEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElOverlaysEl {}
impl BuildTranscoderJobConfigElOverlaysEl {
    pub fn build(self) -> TranscoderJobConfigElOverlaysEl {
        TranscoderJobConfigElOverlaysEl {
            animations: core::default::Default::default(),
            image: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct TranscoderJobConfigElOverlaysElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElOverlaysElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobConfigElOverlaysElRef {
        TranscoderJobConfigElOverlaysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElOverlaysElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `animations` after provisioning.\n"]
    pub fn animations(&self) -> ListRef<TranscoderJobConfigElOverlaysElAnimationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.animations", self.base))
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> ListRef<TranscoderJobConfigElOverlaysElImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image", self.base))
    }
}
#[derive(Serialize)]
pub struct TranscoderJobConfigElPubsubDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<PrimField<String>>,
}
impl TranscoderJobConfigElPubsubDestinationEl {
    #[doc = "Set the field `topic`.\nThe name of the Pub/Sub topic to publish job completion notification to. For example: projects/{project}/topics/{topic}."]
    pub fn set_topic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.topic = Some(v.into());
        self
    }
}
impl ToListMappable for TranscoderJobConfigElPubsubDestinationEl {
    type O = BlockAssignable<TranscoderJobConfigElPubsubDestinationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigElPubsubDestinationEl {}
impl BuildTranscoderJobConfigElPubsubDestinationEl {
    pub fn build(self) -> TranscoderJobConfigElPubsubDestinationEl {
        TranscoderJobConfigElPubsubDestinationEl {
            topic: core::default::Default::default(),
        }
    }
}
pub struct TranscoderJobConfigElPubsubDestinationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElPubsubDestinationElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobConfigElPubsubDestinationElRef {
        TranscoderJobConfigElPubsubDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElPubsubDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\nThe name of the Pub/Sub topic to publish job completion notification to. For example: projects/{project}/topics/{topic}."]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}
#[derive(Serialize, Default)]
struct TranscoderJobConfigElDynamic {
    ad_breaks: Option<DynamicBlock<TranscoderJobConfigElAdBreaksEl>>,
    edit_list: Option<DynamicBlock<TranscoderJobConfigElEditListEl>>,
    elementary_streams: Option<DynamicBlock<TranscoderJobConfigElElementaryStreamsEl>>,
    encryptions: Option<DynamicBlock<TranscoderJobConfigElEncryptionsEl>>,
    inputs: Option<DynamicBlock<TranscoderJobConfigElInputsEl>>,
    manifests: Option<DynamicBlock<TranscoderJobConfigElManifestsEl>>,
    mux_streams: Option<DynamicBlock<TranscoderJobConfigElMuxStreamsEl>>,
    output: Option<DynamicBlock<TranscoderJobConfigElOutputEl>>,
    overlays: Option<DynamicBlock<TranscoderJobConfigElOverlaysEl>>,
    pubsub_destination: Option<DynamicBlock<TranscoderJobConfigElPubsubDestinationEl>>,
}
#[derive(Serialize)]
pub struct TranscoderJobConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ad_breaks: Option<Vec<TranscoderJobConfigElAdBreaksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edit_list: Option<Vec<TranscoderJobConfigElEditListEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elementary_streams: Option<Vec<TranscoderJobConfigElElementaryStreamsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryptions: Option<Vec<TranscoderJobConfigElEncryptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inputs: Option<Vec<TranscoderJobConfigElInputsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manifests: Option<Vec<TranscoderJobConfigElManifestsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mux_streams: Option<Vec<TranscoderJobConfigElMuxStreamsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<Vec<TranscoderJobConfigElOutputEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overlays: Option<Vec<TranscoderJobConfigElOverlaysEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pubsub_destination: Option<Vec<TranscoderJobConfigElPubsubDestinationEl>>,
    dynamic: TranscoderJobConfigElDynamic,
}
impl TranscoderJobConfigEl {
    #[doc = "Set the field `ad_breaks`.\n"]
    pub fn set_ad_breaks(
        mut self,
        v: impl Into<BlockAssignable<TranscoderJobConfigElAdBreaksEl>>,
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
        v: impl Into<BlockAssignable<TranscoderJobConfigElEditListEl>>,
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
        v: impl Into<BlockAssignable<TranscoderJobConfigElElementaryStreamsEl>>,
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
        v: impl Into<BlockAssignable<TranscoderJobConfigElEncryptionsEl>>,
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
        v: impl Into<BlockAssignable<TranscoderJobConfigElInputsEl>>,
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
        v: impl Into<BlockAssignable<TranscoderJobConfigElManifestsEl>>,
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
        v: impl Into<BlockAssignable<TranscoderJobConfigElMuxStreamsEl>>,
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
        v: impl Into<BlockAssignable<TranscoderJobConfigElOutputEl>>,
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
        v: impl Into<BlockAssignable<TranscoderJobConfigElOverlaysEl>>,
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
        v: impl Into<BlockAssignable<TranscoderJobConfigElPubsubDestinationEl>>,
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
impl ToListMappable for TranscoderJobConfigEl {
    type O = BlockAssignable<TranscoderJobConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobConfigEl {}
impl BuildTranscoderJobConfigEl {
    pub fn build(self) -> TranscoderJobConfigEl {
        TranscoderJobConfigEl {
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
pub struct TranscoderJobConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobConfigElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobConfigElRef {
        TranscoderJobConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ad_breaks` after provisioning.\n"]
    pub fn ad_breaks(&self) -> ListRef<TranscoderJobConfigElAdBreaksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ad_breaks", self.base))
    }
    #[doc = "Get a reference to the value of field `edit_list` after provisioning.\n"]
    pub fn edit_list(&self) -> ListRef<TranscoderJobConfigElEditListElRef> {
        ListRef::new(self.shared().clone(), format!("{}.edit_list", self.base))
    }
    #[doc = "Get a reference to the value of field `elementary_streams` after provisioning.\n"]
    pub fn elementary_streams(&self) -> ListRef<TranscoderJobConfigElElementaryStreamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.elementary_streams", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encryptions` after provisioning.\n"]
    pub fn encryptions(&self) -> ListRef<TranscoderJobConfigElEncryptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryptions", self.base))
    }
    #[doc = "Get a reference to the value of field `inputs` after provisioning.\n"]
    pub fn inputs(&self) -> ListRef<TranscoderJobConfigElInputsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.inputs", self.base))
    }
    #[doc = "Get a reference to the value of field `manifests` after provisioning.\n"]
    pub fn manifests(&self) -> ListRef<TranscoderJobConfigElManifestsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.manifests", self.base))
    }
    #[doc = "Get a reference to the value of field `mux_streams` after provisioning.\n"]
    pub fn mux_streams(&self) -> ListRef<TranscoderJobConfigElMuxStreamsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mux_streams", self.base))
    }
    #[doc = "Get a reference to the value of field `output` after provisioning.\n"]
    pub fn output(&self) -> ListRef<TranscoderJobConfigElOutputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.output", self.base))
    }
    #[doc = "Get a reference to the value of field `overlays` after provisioning.\n"]
    pub fn overlays(&self) -> ListRef<TranscoderJobConfigElOverlaysElRef> {
        ListRef::new(self.shared().clone(), format!("{}.overlays", self.base))
    }
    #[doc = "Get a reference to the value of field `pubsub_destination` after provisioning.\n"]
    pub fn pubsub_destination(&self) -> ListRef<TranscoderJobConfigElPubsubDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pubsub_destination", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct TranscoderJobTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl TranscoderJobTimeoutsEl {
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
impl ToListMappable for TranscoderJobTimeoutsEl {
    type O = BlockAssignable<TranscoderJobTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTranscoderJobTimeoutsEl {}
impl BuildTranscoderJobTimeoutsEl {
    pub fn build(self) -> TranscoderJobTimeoutsEl {
        TranscoderJobTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct TranscoderJobTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TranscoderJobTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> TranscoderJobTimeoutsElRef {
        TranscoderJobTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TranscoderJobTimeoutsElRef {
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
struct TranscoderJobDynamic {
    config: Option<DynamicBlock<TranscoderJobConfigEl>>,
}
