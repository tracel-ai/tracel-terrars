use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ElastictranscoderPresetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    container: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_codec_options: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio: Option<Vec<ElastictranscoderPresetAudioEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_codec_options: Option<Vec<ElastictranscoderPresetAudioCodecOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnails: Option<Vec<ElastictranscoderPresetThumbnailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video: Option<Vec<ElastictranscoderPresetVideoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_watermarks: Option<Vec<ElastictranscoderPresetVideoWatermarksEl>>,
    dynamic: ElastictranscoderPresetDynamic,
}
struct ElastictranscoderPreset_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ElastictranscoderPresetData>,
}
#[derive(Clone)]
pub struct ElastictranscoderPreset(Rc<ElastictranscoderPreset_>);
impl ElastictranscoderPreset {
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
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
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
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `video_codec_options`.\n"]
    pub fn set_video_codec_options(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().video_codec_options = Some(v.into());
        self
    }
    #[doc = "Set the field `audio`.\n"]
    pub fn set_audio(self, v: impl Into<BlockAssignable<ElastictranscoderPresetAudioEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().audio = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.audio = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `audio_codec_options`.\n"]
    pub fn set_audio_codec_options(
        self,
        v: impl Into<BlockAssignable<ElastictranscoderPresetAudioCodecOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().audio_codec_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.audio_codec_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `thumbnails`.\n"]
    pub fn set_thumbnails(
        self,
        v: impl Into<BlockAssignable<ElastictranscoderPresetThumbnailsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().thumbnails = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.thumbnails = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `video`.\n"]
    pub fn set_video(self, v: impl Into<BlockAssignable<ElastictranscoderPresetVideoEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().video = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.video = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `video_watermarks`.\n"]
    pub fn set_video_watermarks(
        self,
        v: impl Into<BlockAssignable<ElastictranscoderPresetVideoWatermarksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().video_watermarks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.video_watermarks = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `video_codec_options` after provisioning.\n"]
    pub fn video_codec_options(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.video_codec_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `audio` after provisioning.\n"]
    pub fn audio(&self) -> ListRef<ElastictranscoderPresetAudioElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.audio", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `audio_codec_options` after provisioning.\n"]
    pub fn audio_codec_options(&self) -> ListRef<ElastictranscoderPresetAudioCodecOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.audio_codec_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `thumbnails` after provisioning.\n"]
    pub fn thumbnails(&self) -> ListRef<ElastictranscoderPresetThumbnailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.thumbnails", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `video` after provisioning.\n"]
    pub fn video(&self) -> ListRef<ElastictranscoderPresetVideoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.video", self.extract_ref()),
        )
    }
}
impl Referable for ElastictranscoderPreset {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ElastictranscoderPreset {}
impl ToListMappable for ElastictranscoderPreset {
    type O = ListRef<ElastictranscoderPresetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ElastictranscoderPreset_ {
    fn extract_resource_type(&self) -> String {
        "aws_elastictranscoder_preset".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildElastictranscoderPreset {
    pub tf_id: String,
    #[doc = ""]
    pub container: PrimField<String>,
}
impl BuildElastictranscoderPreset {
    pub fn build(self, stack: &mut Stack) -> ElastictranscoderPreset {
        let out = ElastictranscoderPreset(Rc::new(ElastictranscoderPreset_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ElastictranscoderPresetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                container: self.container,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                type_: core::default::Default::default(),
                video_codec_options: core::default::Default::default(),
                audio: core::default::Default::default(),
                audio_codec_options: core::default::Default::default(),
                thumbnails: core::default::Default::default(),
                video: core::default::Default::default(),
                video_watermarks: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ElastictranscoderPresetRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElastictranscoderPresetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ElastictranscoderPresetRef {
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
    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `video_codec_options` after provisioning.\n"]
    pub fn video_codec_options(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.video_codec_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `audio` after provisioning.\n"]
    pub fn audio(&self) -> ListRef<ElastictranscoderPresetAudioElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.audio", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `audio_codec_options` after provisioning.\n"]
    pub fn audio_codec_options(&self) -> ListRef<ElastictranscoderPresetAudioCodecOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.audio_codec_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `thumbnails` after provisioning.\n"]
    pub fn thumbnails(&self) -> ListRef<ElastictranscoderPresetThumbnailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.thumbnails", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `video` after provisioning.\n"]
    pub fn video(&self) -> ListRef<ElastictranscoderPresetVideoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.video", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ElastictranscoderPresetAudioEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_packing_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bit_rate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channels: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    codec: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_rate: Option<PrimField<String>>,
}
impl ElastictranscoderPresetAudioEl {
    #[doc = "Set the field `audio_packing_mode`.\n"]
    pub fn set_audio_packing_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.audio_packing_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `bit_rate`.\n"]
    pub fn set_bit_rate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bit_rate = Some(v.into());
        self
    }
    #[doc = "Set the field `channels`.\n"]
    pub fn set_channels(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.channels = Some(v.into());
        self
    }
    #[doc = "Set the field `codec`.\n"]
    pub fn set_codec(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.codec = Some(v.into());
        self
    }
    #[doc = "Set the field `sample_rate`.\n"]
    pub fn set_sample_rate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sample_rate = Some(v.into());
        self
    }
}
impl ToListMappable for ElastictranscoderPresetAudioEl {
    type O = BlockAssignable<ElastictranscoderPresetAudioEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElastictranscoderPresetAudioEl {}
impl BuildElastictranscoderPresetAudioEl {
    pub fn build(self) -> ElastictranscoderPresetAudioEl {
        ElastictranscoderPresetAudioEl {
            audio_packing_mode: core::default::Default::default(),
            bit_rate: core::default::Default::default(),
            channels: core::default::Default::default(),
            codec: core::default::Default::default(),
            sample_rate: core::default::Default::default(),
        }
    }
}
pub struct ElastictranscoderPresetAudioElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElastictranscoderPresetAudioElRef {
    fn new(shared: StackShared, base: String) -> ElastictranscoderPresetAudioElRef {
        ElastictranscoderPresetAudioElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElastictranscoderPresetAudioElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `audio_packing_mode` after provisioning.\n"]
    pub fn audio_packing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.audio_packing_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bit_rate` after provisioning.\n"]
    pub fn bit_rate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bit_rate", self.base))
    }
    #[doc = "Get a reference to the value of field `channels` after provisioning.\n"]
    pub fn channels(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.channels", self.base))
    }
    #[doc = "Get a reference to the value of field `codec` after provisioning.\n"]
    pub fn codec(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.codec", self.base))
    }
    #[doc = "Get a reference to the value of field `sample_rate` after provisioning.\n"]
    pub fn sample_rate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sample_rate", self.base))
    }
}
#[derive(Serialize)]
pub struct ElastictranscoderPresetAudioCodecOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bit_depth: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bit_order: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signed: Option<PrimField<String>>,
}
impl ElastictranscoderPresetAudioCodecOptionsEl {
    #[doc = "Set the field `bit_depth`.\n"]
    pub fn set_bit_depth(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bit_depth = Some(v.into());
        self
    }
    #[doc = "Set the field `bit_order`.\n"]
    pub fn set_bit_order(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bit_order = Some(v.into());
        self
    }
    #[doc = "Set the field `profile`.\n"]
    pub fn set_profile(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.profile = Some(v.into());
        self
    }
    #[doc = "Set the field `signed`.\n"]
    pub fn set_signed(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.signed = Some(v.into());
        self
    }
}
impl ToListMappable for ElastictranscoderPresetAudioCodecOptionsEl {
    type O = BlockAssignable<ElastictranscoderPresetAudioCodecOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElastictranscoderPresetAudioCodecOptionsEl {}
impl BuildElastictranscoderPresetAudioCodecOptionsEl {
    pub fn build(self) -> ElastictranscoderPresetAudioCodecOptionsEl {
        ElastictranscoderPresetAudioCodecOptionsEl {
            bit_depth: core::default::Default::default(),
            bit_order: core::default::Default::default(),
            profile: core::default::Default::default(),
            signed: core::default::Default::default(),
        }
    }
}
pub struct ElastictranscoderPresetAudioCodecOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElastictranscoderPresetAudioCodecOptionsElRef {
    fn new(shared: StackShared, base: String) -> ElastictranscoderPresetAudioCodecOptionsElRef {
        ElastictranscoderPresetAudioCodecOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElastictranscoderPresetAudioCodecOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bit_depth` after provisioning.\n"]
    pub fn bit_depth(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bit_depth", self.base))
    }
    #[doc = "Get a reference to the value of field `bit_order` after provisioning.\n"]
    pub fn bit_order(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bit_order", self.base))
    }
    #[doc = "Get a reference to the value of field `profile` after provisioning.\n"]
    pub fn profile(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.profile", self.base))
    }
    #[doc = "Get a reference to the value of field `signed` after provisioning.\n"]
    pub fn signed(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.signed", self.base))
    }
}
#[derive(Serialize)]
pub struct ElastictranscoderPresetThumbnailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aspect_ratio: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_height: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_width: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    padding_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resolution: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sizing_policy: Option<PrimField<String>>,
}
impl ElastictranscoderPresetThumbnailsEl {
    #[doc = "Set the field `aspect_ratio`.\n"]
    pub fn set_aspect_ratio(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.aspect_ratio = Some(v.into());
        self
    }
    #[doc = "Set the field `format`.\n"]
    pub fn set_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.format = Some(v.into());
        self
    }
    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interval = Some(v.into());
        self
    }
    #[doc = "Set the field `max_height`.\n"]
    pub fn set_max_height(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_height = Some(v.into());
        self
    }
    #[doc = "Set the field `max_width`.\n"]
    pub fn set_max_width(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_width = Some(v.into());
        self
    }
    #[doc = "Set the field `padding_policy`.\n"]
    pub fn set_padding_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.padding_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `resolution`.\n"]
    pub fn set_resolution(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resolution = Some(v.into());
        self
    }
    #[doc = "Set the field `sizing_policy`.\n"]
    pub fn set_sizing_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sizing_policy = Some(v.into());
        self
    }
}
impl ToListMappable for ElastictranscoderPresetThumbnailsEl {
    type O = BlockAssignable<ElastictranscoderPresetThumbnailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElastictranscoderPresetThumbnailsEl {}
impl BuildElastictranscoderPresetThumbnailsEl {
    pub fn build(self) -> ElastictranscoderPresetThumbnailsEl {
        ElastictranscoderPresetThumbnailsEl {
            aspect_ratio: core::default::Default::default(),
            format: core::default::Default::default(),
            interval: core::default::Default::default(),
            max_height: core::default::Default::default(),
            max_width: core::default::Default::default(),
            padding_policy: core::default::Default::default(),
            resolution: core::default::Default::default(),
            sizing_policy: core::default::Default::default(),
        }
    }
}
pub struct ElastictranscoderPresetThumbnailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElastictranscoderPresetThumbnailsElRef {
    fn new(shared: StackShared, base: String) -> ElastictranscoderPresetThumbnailsElRef {
        ElastictranscoderPresetThumbnailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElastictranscoderPresetThumbnailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aspect_ratio` after provisioning.\n"]
    pub fn aspect_ratio(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aspect_ratio", self.base))
    }
    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }
    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }
    #[doc = "Get a reference to the value of field `max_height` after provisioning.\n"]
    pub fn max_height(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_height", self.base))
    }
    #[doc = "Get a reference to the value of field `max_width` after provisioning.\n"]
    pub fn max_width(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_width", self.base))
    }
    #[doc = "Get a reference to the value of field `padding_policy` after provisioning.\n"]
    pub fn padding_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.padding_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resolution` after provisioning.\n"]
    pub fn resolution(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resolution", self.base))
    }
    #[doc = "Get a reference to the value of field `sizing_policy` after provisioning.\n"]
    pub fn sizing_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sizing_policy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ElastictranscoderPresetVideoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aspect_ratio: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bit_rate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    codec: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_aspect_ratio: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_gop: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    frame_rate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keyframes_max_dist: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_frame_rate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_height: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_width: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    padding_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resolution: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sizing_policy: Option<PrimField<String>>,
}
impl ElastictranscoderPresetVideoEl {
    #[doc = "Set the field `aspect_ratio`.\n"]
    pub fn set_aspect_ratio(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.aspect_ratio = Some(v.into());
        self
    }
    #[doc = "Set the field `bit_rate`.\n"]
    pub fn set_bit_rate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bit_rate = Some(v.into());
        self
    }
    #[doc = "Set the field `codec`.\n"]
    pub fn set_codec(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.codec = Some(v.into());
        self
    }
    #[doc = "Set the field `display_aspect_ratio`.\n"]
    pub fn set_display_aspect_ratio(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_aspect_ratio = Some(v.into());
        self
    }
    #[doc = "Set the field `fixed_gop`.\n"]
    pub fn set_fixed_gop(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fixed_gop = Some(v.into());
        self
    }
    #[doc = "Set the field `frame_rate`.\n"]
    pub fn set_frame_rate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.frame_rate = Some(v.into());
        self
    }
    #[doc = "Set the field `keyframes_max_dist`.\n"]
    pub fn set_keyframes_max_dist(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.keyframes_max_dist = Some(v.into());
        self
    }
    #[doc = "Set the field `max_frame_rate`.\n"]
    pub fn set_max_frame_rate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_frame_rate = Some(v.into());
        self
    }
    #[doc = "Set the field `max_height`.\n"]
    pub fn set_max_height(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_height = Some(v.into());
        self
    }
    #[doc = "Set the field `max_width`.\n"]
    pub fn set_max_width(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_width = Some(v.into());
        self
    }
    #[doc = "Set the field `padding_policy`.\n"]
    pub fn set_padding_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.padding_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `resolution`.\n"]
    pub fn set_resolution(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resolution = Some(v.into());
        self
    }
    #[doc = "Set the field `sizing_policy`.\n"]
    pub fn set_sizing_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sizing_policy = Some(v.into());
        self
    }
}
impl ToListMappable for ElastictranscoderPresetVideoEl {
    type O = BlockAssignable<ElastictranscoderPresetVideoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElastictranscoderPresetVideoEl {}
impl BuildElastictranscoderPresetVideoEl {
    pub fn build(self) -> ElastictranscoderPresetVideoEl {
        ElastictranscoderPresetVideoEl {
            aspect_ratio: core::default::Default::default(),
            bit_rate: core::default::Default::default(),
            codec: core::default::Default::default(),
            display_aspect_ratio: core::default::Default::default(),
            fixed_gop: core::default::Default::default(),
            frame_rate: core::default::Default::default(),
            keyframes_max_dist: core::default::Default::default(),
            max_frame_rate: core::default::Default::default(),
            max_height: core::default::Default::default(),
            max_width: core::default::Default::default(),
            padding_policy: core::default::Default::default(),
            resolution: core::default::Default::default(),
            sizing_policy: core::default::Default::default(),
        }
    }
}
pub struct ElastictranscoderPresetVideoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElastictranscoderPresetVideoElRef {
    fn new(shared: StackShared, base: String) -> ElastictranscoderPresetVideoElRef {
        ElastictranscoderPresetVideoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElastictranscoderPresetVideoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aspect_ratio` after provisioning.\n"]
    pub fn aspect_ratio(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aspect_ratio", self.base))
    }
    #[doc = "Get a reference to the value of field `bit_rate` after provisioning.\n"]
    pub fn bit_rate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bit_rate", self.base))
    }
    #[doc = "Get a reference to the value of field `codec` after provisioning.\n"]
    pub fn codec(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.codec", self.base))
    }
    #[doc = "Get a reference to the value of field `display_aspect_ratio` after provisioning.\n"]
    pub fn display_aspect_ratio(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_aspect_ratio", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fixed_gop` after provisioning.\n"]
    pub fn fixed_gop(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fixed_gop", self.base))
    }
    #[doc = "Get a reference to the value of field `frame_rate` after provisioning.\n"]
    pub fn frame_rate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.frame_rate", self.base))
    }
    #[doc = "Get a reference to the value of field `keyframes_max_dist` after provisioning.\n"]
    pub fn keyframes_max_dist(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.keyframes_max_dist", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_frame_rate` after provisioning.\n"]
    pub fn max_frame_rate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_frame_rate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_height` after provisioning.\n"]
    pub fn max_height(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_height", self.base))
    }
    #[doc = "Get a reference to the value of field `max_width` after provisioning.\n"]
    pub fn max_width(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_width", self.base))
    }
    #[doc = "Get a reference to the value of field `padding_policy` after provisioning.\n"]
    pub fn padding_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.padding_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resolution` after provisioning.\n"]
    pub fn resolution(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resolution", self.base))
    }
    #[doc = "Get a reference to the value of field `sizing_policy` after provisioning.\n"]
    pub fn sizing_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sizing_policy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ElastictranscoderPresetVideoWatermarksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_align: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_offset: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_height: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_width: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sizing_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vertical_align: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vertical_offset: Option<PrimField<String>>,
}
impl ElastictranscoderPresetVideoWatermarksEl {
    #[doc = "Set the field `horizontal_align`.\n"]
    pub fn set_horizontal_align(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.horizontal_align = Some(v.into());
        self
    }
    #[doc = "Set the field `horizontal_offset`.\n"]
    pub fn set_horizontal_offset(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.horizontal_offset = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `max_height`.\n"]
    pub fn set_max_height(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_height = Some(v.into());
        self
    }
    #[doc = "Set the field `max_width`.\n"]
    pub fn set_max_width(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_width = Some(v.into());
        self
    }
    #[doc = "Set the field `opacity`.\n"]
    pub fn set_opacity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.opacity = Some(v.into());
        self
    }
    #[doc = "Set the field `sizing_policy`.\n"]
    pub fn set_sizing_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sizing_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `target`.\n"]
    pub fn set_target(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target = Some(v.into());
        self
    }
    #[doc = "Set the field `vertical_align`.\n"]
    pub fn set_vertical_align(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vertical_align = Some(v.into());
        self
    }
    #[doc = "Set the field `vertical_offset`.\n"]
    pub fn set_vertical_offset(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vertical_offset = Some(v.into());
        self
    }
}
impl ToListMappable for ElastictranscoderPresetVideoWatermarksEl {
    type O = BlockAssignable<ElastictranscoderPresetVideoWatermarksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElastictranscoderPresetVideoWatermarksEl {}
impl BuildElastictranscoderPresetVideoWatermarksEl {
    pub fn build(self) -> ElastictranscoderPresetVideoWatermarksEl {
        ElastictranscoderPresetVideoWatermarksEl {
            horizontal_align: core::default::Default::default(),
            horizontal_offset: core::default::Default::default(),
            id: core::default::Default::default(),
            max_height: core::default::Default::default(),
            max_width: core::default::Default::default(),
            opacity: core::default::Default::default(),
            sizing_policy: core::default::Default::default(),
            target: core::default::Default::default(),
            vertical_align: core::default::Default::default(),
            vertical_offset: core::default::Default::default(),
        }
    }
}
pub struct ElastictranscoderPresetVideoWatermarksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElastictranscoderPresetVideoWatermarksElRef {
    fn new(shared: StackShared, base: String) -> ElastictranscoderPresetVideoWatermarksElRef {
        ElastictranscoderPresetVideoWatermarksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElastictranscoderPresetVideoWatermarksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `horizontal_align` after provisioning.\n"]
    pub fn horizontal_align(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.horizontal_align", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `horizontal_offset` after provisioning.\n"]
    pub fn horizontal_offset(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.horizontal_offset", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `max_height` after provisioning.\n"]
    pub fn max_height(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_height", self.base))
    }
    #[doc = "Get a reference to the value of field `max_width` after provisioning.\n"]
    pub fn max_width(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_width", self.base))
    }
    #[doc = "Get a reference to the value of field `opacity` after provisioning.\n"]
    pub fn opacity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.opacity", self.base))
    }
    #[doc = "Get a reference to the value of field `sizing_policy` after provisioning.\n"]
    pub fn sizing_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sizing_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
    #[doc = "Get a reference to the value of field `vertical_align` after provisioning.\n"]
    pub fn vertical_align(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vertical_align", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vertical_offset` after provisioning.\n"]
    pub fn vertical_offset(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vertical_offset", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ElastictranscoderPresetDynamic {
    audio: Option<DynamicBlock<ElastictranscoderPresetAudioEl>>,
    audio_codec_options: Option<DynamicBlock<ElastictranscoderPresetAudioCodecOptionsEl>>,
    thumbnails: Option<DynamicBlock<ElastictranscoderPresetThumbnailsEl>>,
    video: Option<DynamicBlock<ElastictranscoderPresetVideoEl>>,
    video_watermarks: Option<DynamicBlock<ElastictranscoderPresetVideoWatermarksEl>>,
}
