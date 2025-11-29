use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct RekognitionStreamProcessorData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_sharing_preference: Option<Vec<RekognitionStreamProcessorDataSharingPreferenceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<Vec<RekognitionStreamProcessorInputEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_channel: Option<Vec<RekognitionStreamProcessorNotificationChannelEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<Vec<RekognitionStreamProcessorOutputEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regions_of_interest: Option<Vec<RekognitionStreamProcessorRegionsOfInterestEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<Vec<RekognitionStreamProcessorSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<RekognitionStreamProcessorTimeoutsEl>,
    dynamic: RekognitionStreamProcessorDynamic,
}

struct RekognitionStreamProcessor_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RekognitionStreamProcessorData>,
}

#[derive(Clone)]
pub struct RekognitionStreamProcessor(Rc<RekognitionStreamProcessor_>);

impl RekognitionStreamProcessor {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc =
        "Set the field `kms_key_id`.\nThe identifier for your AWS Key Management Service key (AWS KMS key). You can supply the Amazon Resource Name (ARN) of your KMS key, the ID of your KMS key, an alias for your KMS key, or an alias ARN."]
    pub fn set_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `data_sharing_preference`.\n"]
    pub fn set_data_sharing_preference(
        self,
        v: impl Into<BlockAssignable<RekognitionStreamProcessorDataSharingPreferenceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_sharing_preference = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_sharing_preference = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input`.\n"]
    pub fn set_input(self, v: impl Into<BlockAssignable<RekognitionStreamProcessorInputEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().input = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.input = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `notification_channel`.\n"]
    pub fn set_notification_channel(
        self,
        v: impl Into<BlockAssignable<RekognitionStreamProcessorNotificationChannelEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification_channel = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification_channel = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output`.\n"]
    pub fn set_output(self, v: impl Into<BlockAssignable<RekognitionStreamProcessorOutputEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().output = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.output = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `regions_of_interest`.\n"]
    pub fn set_regions_of_interest(
        self,
        v: impl Into<BlockAssignable<RekognitionStreamProcessorRegionsOfInterestEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().regions_of_interest = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.regions_of_interest = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(self, v: impl Into<BlockAssignable<RekognitionStreamProcessorSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<RekognitionStreamProcessorTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_id` after provisioning.\nThe identifier for your AWS Key Management Service key (AWS KMS key). You can supply the Amazon Resource Name (ARN) of your KMS key, the ID of your KMS key, an alias for your KMS key, or an alias ARN."]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nAn identifier you assign to the stream processor."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `role_arn` after provisioning.\nThe Amazon Resource Number (ARN) of the IAM role that allows access to the stream processor."]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stream_processor_arn` after provisioning.\n"]
    pub fn stream_processor_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_processor_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_sharing_preference` after provisioning.\n"]
    pub fn data_sharing_preference(&self) -> ListRef<RekognitionStreamProcessorDataSharingPreferenceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_sharing_preference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(&self) -> ListRef<RekognitionStreamProcessorInputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.input", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_channel` after provisioning.\n"]
    pub fn notification_channel(&self) -> ListRef<RekognitionStreamProcessorNotificationChannelElRef> {
        ListRef::new(self.shared().clone(), format!("{}.notification_channel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output` after provisioning.\n"]
    pub fn output(&self) -> ListRef<RekognitionStreamProcessorOutputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.output", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regions_of_interest` after provisioning.\n"]
    pub fn regions_of_interest(&self) -> ListRef<RekognitionStreamProcessorRegionsOfInterestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.regions_of_interest", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> ListRef<RekognitionStreamProcessorSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RekognitionStreamProcessorTimeoutsElRef {
        RekognitionStreamProcessorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for RekognitionStreamProcessor {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for RekognitionStreamProcessor { }

impl ToListMappable for RekognitionStreamProcessor {
    type O = ListRef<RekognitionStreamProcessorRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for RekognitionStreamProcessor_ {
    fn extract_resource_type(&self) -> String {
        "aws_rekognition_stream_processor".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRekognitionStreamProcessor {
    pub tf_id: String,
    #[doc = "An identifier you assign to the stream processor."]
    pub name: PrimField<String>,
    #[doc = "The Amazon Resource Number (ARN) of the IAM role that allows access to the stream processor."]
    pub role_arn: PrimField<String>,
}

impl BuildRekognitionStreamProcessor {
    pub fn build(self, stack: &mut Stack) -> RekognitionStreamProcessor {
        let out = RekognitionStreamProcessor(Rc::new(RekognitionStreamProcessor_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RekognitionStreamProcessorData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                kms_key_id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                data_sharing_preference: core::default::Default::default(),
                input: core::default::Default::default(),
                notification_channel: core::default::Default::default(),
                output: core::default::Default::default(),
                regions_of_interest: core::default::Default::default(),
                settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RekognitionStreamProcessorRef {
    shared: StackShared,
    base: String,
}

impl Ref for RekognitionStreamProcessorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl RekognitionStreamProcessorRef {
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

    #[doc =
        "Get a reference to the value of field `kms_key_id` after provisioning.\nThe identifier for your AWS Key Management Service key (AWS KMS key). You can supply the Amazon Resource Name (ARN) of your KMS key, the ID of your KMS key, an alias for your KMS key, or an alias ARN."]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nAn identifier you assign to the stream processor."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `role_arn` after provisioning.\nThe Amazon Resource Number (ARN) of the IAM role that allows access to the stream processor."]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stream_processor_arn` after provisioning.\n"]
    pub fn stream_processor_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_processor_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_sharing_preference` after provisioning.\n"]
    pub fn data_sharing_preference(&self) -> ListRef<RekognitionStreamProcessorDataSharingPreferenceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_sharing_preference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(&self) -> ListRef<RekognitionStreamProcessorInputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.input", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_channel` after provisioning.\n"]
    pub fn notification_channel(&self) -> ListRef<RekognitionStreamProcessorNotificationChannelElRef> {
        ListRef::new(self.shared().clone(), format!("{}.notification_channel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output` after provisioning.\n"]
    pub fn output(&self) -> ListRef<RekognitionStreamProcessorOutputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.output", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regions_of_interest` after provisioning.\n"]
    pub fn regions_of_interest(&self) -> ListRef<RekognitionStreamProcessorRegionsOfInterestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.regions_of_interest", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> ListRef<RekognitionStreamProcessorSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RekognitionStreamProcessorTimeoutsElRef {
        RekognitionStreamProcessorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessorDataSharingPreferenceEl {
    opt_in: PrimField<bool>,
}

impl RekognitionStreamProcessorDataSharingPreferenceEl { }

impl ToListMappable for RekognitionStreamProcessorDataSharingPreferenceEl {
    type O = BlockAssignable<RekognitionStreamProcessorDataSharingPreferenceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessorDataSharingPreferenceEl {
    #[doc = "Do you want to share data with Rekognition to improve model performance."]
    pub opt_in: PrimField<bool>,
}

impl BuildRekognitionStreamProcessorDataSharingPreferenceEl {
    pub fn build(self) -> RekognitionStreamProcessorDataSharingPreferenceEl {
        RekognitionStreamProcessorDataSharingPreferenceEl { opt_in: self.opt_in }
    }
}

pub struct RekognitionStreamProcessorDataSharingPreferenceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RekognitionStreamProcessorDataSharingPreferenceElRef {
    fn new(shared: StackShared, base: String) -> RekognitionStreamProcessorDataSharingPreferenceElRef {
        RekognitionStreamProcessorDataSharingPreferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessorDataSharingPreferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `opt_in` after provisioning.\nDo you want to share data with Rekognition to improve model performance."]
    pub fn opt_in(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.opt_in", self.base))
    }
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessorInputElKinesisVideoStreamEl {
    arn: PrimField<String>,
}

impl RekognitionStreamProcessorInputElKinesisVideoStreamEl { }

impl ToListMappable for RekognitionStreamProcessorInputElKinesisVideoStreamEl {
    type O = BlockAssignable<RekognitionStreamProcessorInputElKinesisVideoStreamEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessorInputElKinesisVideoStreamEl {
    #[doc = "ARN of the Kinesis video stream stream that streams the source video."]
    pub arn: PrimField<String>,
}

impl BuildRekognitionStreamProcessorInputElKinesisVideoStreamEl {
    pub fn build(self) -> RekognitionStreamProcessorInputElKinesisVideoStreamEl {
        RekognitionStreamProcessorInputElKinesisVideoStreamEl { arn: self.arn }
    }
}

pub struct RekognitionStreamProcessorInputElKinesisVideoStreamElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RekognitionStreamProcessorInputElKinesisVideoStreamElRef {
    fn new(shared: StackShared, base: String) -> RekognitionStreamProcessorInputElKinesisVideoStreamElRef {
        RekognitionStreamProcessorInputElKinesisVideoStreamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessorInputElKinesisVideoStreamElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `arn` after provisioning.\nARN of the Kinesis video stream stream that streams the source video."]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct RekognitionStreamProcessorInputElDynamic {
    kinesis_video_stream: Option<DynamicBlock<RekognitionStreamProcessorInputElKinesisVideoStreamEl>>,
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessorInputEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_video_stream: Option<Vec<RekognitionStreamProcessorInputElKinesisVideoStreamEl>>,
    dynamic: RekognitionStreamProcessorInputElDynamic,
}

impl RekognitionStreamProcessorInputEl {
    #[doc = "Set the field `kinesis_video_stream`.\n"]
    pub fn set_kinesis_video_stream(
        mut self,
        v: impl Into<BlockAssignable<RekognitionStreamProcessorInputElKinesisVideoStreamEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kinesis_video_stream = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_video_stream = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for RekognitionStreamProcessorInputEl {
    type O = BlockAssignable<RekognitionStreamProcessorInputEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessorInputEl {}

impl BuildRekognitionStreamProcessorInputEl {
    pub fn build(self) -> RekognitionStreamProcessorInputEl {
        RekognitionStreamProcessorInputEl {
            kinesis_video_stream: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct RekognitionStreamProcessorInputElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RekognitionStreamProcessorInputElRef {
    fn new(shared: StackShared, base: String) -> RekognitionStreamProcessorInputElRef {
        RekognitionStreamProcessorInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessorInputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kinesis_video_stream` after provisioning.\n"]
    pub fn kinesis_video_stream(&self) -> ListRef<RekognitionStreamProcessorInputElKinesisVideoStreamElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kinesis_video_stream", self.base))
    }
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessorNotificationChannelEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    sns_topic_arn: Option<PrimField<String>>,
}

impl RekognitionStreamProcessorNotificationChannelEl {
    #[doc =
        "Set the field `sns_topic_arn`.\nThe Amazon Resource Number (ARN) of the Amazon Amazon Simple Notification Service topic to which Amazon Rekognition posts the completion status."]
    pub fn set_sns_topic_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sns_topic_arn = Some(v.into());
        self
    }
}

impl ToListMappable for RekognitionStreamProcessorNotificationChannelEl {
    type O = BlockAssignable<RekognitionStreamProcessorNotificationChannelEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessorNotificationChannelEl {}

impl BuildRekognitionStreamProcessorNotificationChannelEl {
    pub fn build(self) -> RekognitionStreamProcessorNotificationChannelEl {
        RekognitionStreamProcessorNotificationChannelEl { sns_topic_arn: core::default::Default::default() }
    }
}

pub struct RekognitionStreamProcessorNotificationChannelElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RekognitionStreamProcessorNotificationChannelElRef {
    fn new(shared: StackShared, base: String) -> RekognitionStreamProcessorNotificationChannelElRef {
        RekognitionStreamProcessorNotificationChannelElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessorNotificationChannelElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `sns_topic_arn` after provisioning.\nThe Amazon Resource Number (ARN) of the Amazon Amazon Simple Notification Service topic to which Amazon Rekognition posts the completion status."]
    pub fn sns_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sns_topic_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessorOutputElKinesisDataStreamEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
}

impl RekognitionStreamProcessorOutputElKinesisDataStreamEl {
    #[doc = "Set the field `arn`.\nARN of the output Amazon Kinesis Data Streams stream."]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }
}

impl ToListMappable for RekognitionStreamProcessorOutputElKinesisDataStreamEl {
    type O = BlockAssignable<RekognitionStreamProcessorOutputElKinesisDataStreamEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessorOutputElKinesisDataStreamEl {}

impl BuildRekognitionStreamProcessorOutputElKinesisDataStreamEl {
    pub fn build(self) -> RekognitionStreamProcessorOutputElKinesisDataStreamEl {
        RekognitionStreamProcessorOutputElKinesisDataStreamEl { arn: core::default::Default::default() }
    }
}

pub struct RekognitionStreamProcessorOutputElKinesisDataStreamElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RekognitionStreamProcessorOutputElKinesisDataStreamElRef {
    fn new(shared: StackShared, base: String) -> RekognitionStreamProcessorOutputElKinesisDataStreamElRef {
        RekognitionStreamProcessorOutputElKinesisDataStreamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessorOutputElKinesisDataStreamElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `arn` after provisioning.\nARN of the output Amazon Kinesis Data Streams stream."]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessorOutputElS3DestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_prefix: Option<PrimField<String>>,
}

impl RekognitionStreamProcessorOutputElS3DestinationEl {
    #[doc =
        "Set the field `bucket`.\nThe name of the Amazon S3 bucket you want to associate with the streaming video project."]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }

    #[doc =
        "Set the field `key_prefix`.\nThe prefix value of the location within the bucket that you want the information to be published to."]
    pub fn set_key_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for RekognitionStreamProcessorOutputElS3DestinationEl {
    type O = BlockAssignable<RekognitionStreamProcessorOutputElS3DestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessorOutputElS3DestinationEl {}

impl BuildRekognitionStreamProcessorOutputElS3DestinationEl {
    pub fn build(self) -> RekognitionStreamProcessorOutputElS3DestinationEl {
        RekognitionStreamProcessorOutputElS3DestinationEl {
            bucket: core::default::Default::default(),
            key_prefix: core::default::Default::default(),
        }
    }
}

pub struct RekognitionStreamProcessorOutputElS3DestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RekognitionStreamProcessorOutputElS3DestinationElRef {
    fn new(shared: StackShared, base: String) -> RekognitionStreamProcessorOutputElS3DestinationElRef {
        RekognitionStreamProcessorOutputElS3DestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessorOutputElS3DestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket` after provisioning.\nThe name of the Amazon S3 bucket you want to associate with the streaming video project."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_prefix` after provisioning.\nThe prefix value of the location within the bucket that you want the information to be published to."]
    pub fn key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct RekognitionStreamProcessorOutputElDynamic {
    kinesis_data_stream: Option<DynamicBlock<RekognitionStreamProcessorOutputElKinesisDataStreamEl>>,
    s3_destination: Option<DynamicBlock<RekognitionStreamProcessorOutputElS3DestinationEl>>,
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessorOutputEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_data_stream: Option<Vec<RekognitionStreamProcessorOutputElKinesisDataStreamEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_destination: Option<Vec<RekognitionStreamProcessorOutputElS3DestinationEl>>,
    dynamic: RekognitionStreamProcessorOutputElDynamic,
}

impl RekognitionStreamProcessorOutputEl {
    #[doc = "Set the field `kinesis_data_stream`.\n"]
    pub fn set_kinesis_data_stream(
        mut self,
        v: impl Into<BlockAssignable<RekognitionStreamProcessorOutputElKinesisDataStreamEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kinesis_data_stream = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_data_stream = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_destination`.\n"]
    pub fn set_s3_destination(
        mut self,
        v: impl Into<BlockAssignable<RekognitionStreamProcessorOutputElS3DestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_destination = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_destination = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for RekognitionStreamProcessorOutputEl {
    type O = BlockAssignable<RekognitionStreamProcessorOutputEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessorOutputEl {}

impl BuildRekognitionStreamProcessorOutputEl {
    pub fn build(self) -> RekognitionStreamProcessorOutputEl {
        RekognitionStreamProcessorOutputEl {
            kinesis_data_stream: core::default::Default::default(),
            s3_destination: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct RekognitionStreamProcessorOutputElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RekognitionStreamProcessorOutputElRef {
    fn new(shared: StackShared, base: String) -> RekognitionStreamProcessorOutputElRef {
        RekognitionStreamProcessorOutputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessorOutputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kinesis_data_stream` after provisioning.\n"]
    pub fn kinesis_data_stream(&self) -> ListRef<RekognitionStreamProcessorOutputElKinesisDataStreamElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kinesis_data_stream", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_destination` after provisioning.\n"]
    pub fn s3_destination(&self) -> ListRef<RekognitionStreamProcessorOutputElS3DestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_destination", self.base))
    }
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessorRegionsOfInterestElBoundingBoxEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<PrimField<f64>>,
}

impl RekognitionStreamProcessorRegionsOfInterestElBoundingBoxEl {
    #[doc = "Set the field `height`.\nHeight of the bounding box as a ratio of the overall image height."]
    pub fn set_height(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.height = Some(v.into());
        self
    }

    #[doc = "Set the field `left`.\nLeft coordinate of the bounding box as a ratio of overall image width."]
    pub fn set_left(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.left = Some(v.into());
        self
    }

    #[doc = "Set the field `top`.\nTop coordinate of the bounding box as a ratio of overall image height."]
    pub fn set_top(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.top = Some(v.into());
        self
    }

    #[doc = "Set the field `width`.\nWidth of the bounding box as a ratio of the overall image width."]
    pub fn set_width(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.width = Some(v.into());
        self
    }
}

impl ToListMappable for RekognitionStreamProcessorRegionsOfInterestElBoundingBoxEl {
    type O = BlockAssignable<RekognitionStreamProcessorRegionsOfInterestElBoundingBoxEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessorRegionsOfInterestElBoundingBoxEl {}

impl BuildRekognitionStreamProcessorRegionsOfInterestElBoundingBoxEl {
    pub fn build(self) -> RekognitionStreamProcessorRegionsOfInterestElBoundingBoxEl {
        RekognitionStreamProcessorRegionsOfInterestElBoundingBoxEl {
            height: core::default::Default::default(),
            left: core::default::Default::default(),
            top: core::default::Default::default(),
            width: core::default::Default::default(),
        }
    }
}

pub struct RekognitionStreamProcessorRegionsOfInterestElBoundingBoxElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RekognitionStreamProcessorRegionsOfInterestElBoundingBoxElRef {
    fn new(shared: StackShared, base: String) -> RekognitionStreamProcessorRegionsOfInterestElBoundingBoxElRef {
        RekognitionStreamProcessorRegionsOfInterestElBoundingBoxElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessorRegionsOfInterestElBoundingBoxElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `height` after provisioning.\nHeight of the bounding box as a ratio of the overall image height."]
    pub fn height(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.height", self.base))
    }

    #[doc =
        "Get a reference to the value of field `left` after provisioning.\nLeft coordinate of the bounding box as a ratio of overall image width."]
    pub fn left(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.left", self.base))
    }

    #[doc =
        "Get a reference to the value of field `top` after provisioning.\nTop coordinate of the bounding box as a ratio of overall image height."]
    pub fn top(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.top", self.base))
    }

    #[doc =
        "Get a reference to the value of field `width` after provisioning.\nWidth of the bounding box as a ratio of the overall image width."]
    pub fn width(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.width", self.base))
    }
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessorRegionsOfInterestElPolygonEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<PrimField<f64>>,
}

impl RekognitionStreamProcessorRegionsOfInterestElPolygonEl {
    #[doc = "Set the field `x`.\nThe value of the X coordinate for a point on a Polygon."]
    pub fn set_x(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.x = Some(v.into());
        self
    }

    #[doc = "Set the field `y`.\nThe value of the Y coordinate for a point on a Polygon."]
    pub fn set_y(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.y = Some(v.into());
        self
    }
}

impl ToListMappable for RekognitionStreamProcessorRegionsOfInterestElPolygonEl {
    type O = BlockAssignable<RekognitionStreamProcessorRegionsOfInterestElPolygonEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessorRegionsOfInterestElPolygonEl {}

impl BuildRekognitionStreamProcessorRegionsOfInterestElPolygonEl {
    pub fn build(self) -> RekognitionStreamProcessorRegionsOfInterestElPolygonEl {
        RekognitionStreamProcessorRegionsOfInterestElPolygonEl {
            x: core::default::Default::default(),
            y: core::default::Default::default(),
        }
    }
}

pub struct RekognitionStreamProcessorRegionsOfInterestElPolygonElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RekognitionStreamProcessorRegionsOfInterestElPolygonElRef {
    fn new(shared: StackShared, base: String) -> RekognitionStreamProcessorRegionsOfInterestElPolygonElRef {
        RekognitionStreamProcessorRegionsOfInterestElPolygonElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessorRegionsOfInterestElPolygonElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `x` after provisioning.\nThe value of the X coordinate for a point on a Polygon."]
    pub fn x(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.x", self.base))
    }

    #[doc =
        "Get a reference to the value of field `y` after provisioning.\nThe value of the Y coordinate for a point on a Polygon."]
    pub fn y(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.y", self.base))
    }
}

#[derive(Serialize, Default)]
struct RekognitionStreamProcessorRegionsOfInterestElDynamic {
    bounding_box: Option<DynamicBlock<RekognitionStreamProcessorRegionsOfInterestElBoundingBoxEl>>,
    polygon: Option<DynamicBlock<RekognitionStreamProcessorRegionsOfInterestElPolygonEl>>,
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessorRegionsOfInterestEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bounding_box: Option<Vec<RekognitionStreamProcessorRegionsOfInterestElBoundingBoxEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    polygon: Option<Vec<RekognitionStreamProcessorRegionsOfInterestElPolygonEl>>,
    dynamic: RekognitionStreamProcessorRegionsOfInterestElDynamic,
}

impl RekognitionStreamProcessorRegionsOfInterestEl {
    #[doc = "Set the field `bounding_box`.\n"]
    pub fn set_bounding_box(
        mut self,
        v: impl Into<BlockAssignable<RekognitionStreamProcessorRegionsOfInterestElBoundingBoxEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.bounding_box = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.bounding_box = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `polygon`.\n"]
    pub fn set_polygon(
        mut self,
        v: impl Into<BlockAssignable<RekognitionStreamProcessorRegionsOfInterestElPolygonEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.polygon = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.polygon = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for RekognitionStreamProcessorRegionsOfInterestEl {
    type O = BlockAssignable<RekognitionStreamProcessorRegionsOfInterestEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessorRegionsOfInterestEl {}

impl BuildRekognitionStreamProcessorRegionsOfInterestEl {
    pub fn build(self) -> RekognitionStreamProcessorRegionsOfInterestEl {
        RekognitionStreamProcessorRegionsOfInterestEl {
            bounding_box: core::default::Default::default(),
            polygon: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct RekognitionStreamProcessorRegionsOfInterestElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RekognitionStreamProcessorRegionsOfInterestElRef {
    fn new(shared: StackShared, base: String) -> RekognitionStreamProcessorRegionsOfInterestElRef {
        RekognitionStreamProcessorRegionsOfInterestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessorRegionsOfInterestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bounding_box` after provisioning.\n"]
    pub fn bounding_box(&self) -> ListRef<RekognitionStreamProcessorRegionsOfInterestElBoundingBoxElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bounding_box", self.base))
    }

    #[doc = "Get a reference to the value of field `polygon` after provisioning.\n"]
    pub fn polygon(&self) -> ListRef<RekognitionStreamProcessorRegionsOfInterestElPolygonElRef> {
        ListRef::new(self.shared().clone(), format!("{}.polygon", self.base))
    }
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessorSettingsElConnectedHomeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_confidence: Option<PrimField<f64>>,
}

impl RekognitionStreamProcessorSettingsElConnectedHomeEl {
    #[doc =
        "Set the field `labels`.\nSpecifies what you want to detect in the video, such as people, packages, or pets."]
    pub fn set_labels(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }

    #[doc = "Set the field `min_confidence`.\nThe minimum confidence required to label an object in the video."]
    pub fn set_min_confidence(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_confidence = Some(v.into());
        self
    }
}

impl ToListMappable for RekognitionStreamProcessorSettingsElConnectedHomeEl {
    type O = BlockAssignable<RekognitionStreamProcessorSettingsElConnectedHomeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessorSettingsElConnectedHomeEl {}

impl BuildRekognitionStreamProcessorSettingsElConnectedHomeEl {
    pub fn build(self) -> RekognitionStreamProcessorSettingsElConnectedHomeEl {
        RekognitionStreamProcessorSettingsElConnectedHomeEl {
            labels: core::default::Default::default(),
            min_confidence: core::default::Default::default(),
        }
    }
}

pub struct RekognitionStreamProcessorSettingsElConnectedHomeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RekognitionStreamProcessorSettingsElConnectedHomeElRef {
    fn new(shared: StackShared, base: String) -> RekognitionStreamProcessorSettingsElConnectedHomeElRef {
        RekognitionStreamProcessorSettingsElConnectedHomeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessorSettingsElConnectedHomeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSpecifies what you want to detect in the video, such as people, packages, or pets."]
    pub fn labels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_confidence` after provisioning.\nThe minimum confidence required to label an object in the video."]
    pub fn min_confidence(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_confidence", self.base))
    }
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessorSettingsElFaceSearchEl {
    collection_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    face_match_threshold: Option<PrimField<f64>>,
}

impl RekognitionStreamProcessorSettingsElFaceSearchEl {
    #[doc =
        "Set the field `face_match_threshold`.\nMinimum face match confidence score that must be met to return a result for a recognized face."]
    pub fn set_face_match_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.face_match_threshold = Some(v.into());
        self
    }
}

impl ToListMappable for RekognitionStreamProcessorSettingsElFaceSearchEl {
    type O = BlockAssignable<RekognitionStreamProcessorSettingsElFaceSearchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessorSettingsElFaceSearchEl {
    #[doc = "The ID of a collection that contains faces that you want to search for."]
    pub collection_id: PrimField<String>,
}

impl BuildRekognitionStreamProcessorSettingsElFaceSearchEl {
    pub fn build(self) -> RekognitionStreamProcessorSettingsElFaceSearchEl {
        RekognitionStreamProcessorSettingsElFaceSearchEl {
            collection_id: self.collection_id,
            face_match_threshold: core::default::Default::default(),
        }
    }
}

pub struct RekognitionStreamProcessorSettingsElFaceSearchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RekognitionStreamProcessorSettingsElFaceSearchElRef {
    fn new(shared: StackShared, base: String) -> RekognitionStreamProcessorSettingsElFaceSearchElRef {
        RekognitionStreamProcessorSettingsElFaceSearchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessorSettingsElFaceSearchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `collection_id` after provisioning.\nThe ID of a collection that contains faces that you want to search for."]
    pub fn collection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.collection_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `face_match_threshold` after provisioning.\nMinimum face match confidence score that must be met to return a result for a recognized face."]
    pub fn face_match_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.face_match_threshold", self.base))
    }
}

#[derive(Serialize, Default)]
struct RekognitionStreamProcessorSettingsElDynamic {
    connected_home: Option<DynamicBlock<RekognitionStreamProcessorSettingsElConnectedHomeEl>>,
    face_search: Option<DynamicBlock<RekognitionStreamProcessorSettingsElFaceSearchEl>>,
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessorSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connected_home: Option<Vec<RekognitionStreamProcessorSettingsElConnectedHomeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    face_search: Option<Vec<RekognitionStreamProcessorSettingsElFaceSearchEl>>,
    dynamic: RekognitionStreamProcessorSettingsElDynamic,
}

impl RekognitionStreamProcessorSettingsEl {
    #[doc = "Set the field `connected_home`.\n"]
    pub fn set_connected_home(
        mut self,
        v: impl Into<BlockAssignable<RekognitionStreamProcessorSettingsElConnectedHomeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.connected_home = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.connected_home = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `face_search`.\n"]
    pub fn set_face_search(
        mut self,
        v: impl Into<BlockAssignable<RekognitionStreamProcessorSettingsElFaceSearchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.face_search = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.face_search = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for RekognitionStreamProcessorSettingsEl {
    type O = BlockAssignable<RekognitionStreamProcessorSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessorSettingsEl {}

impl BuildRekognitionStreamProcessorSettingsEl {
    pub fn build(self) -> RekognitionStreamProcessorSettingsEl {
        RekognitionStreamProcessorSettingsEl {
            connected_home: core::default::Default::default(),
            face_search: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct RekognitionStreamProcessorSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RekognitionStreamProcessorSettingsElRef {
    fn new(shared: StackShared, base: String) -> RekognitionStreamProcessorSettingsElRef {
        RekognitionStreamProcessorSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessorSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connected_home` after provisioning.\n"]
    pub fn connected_home(&self) -> ListRef<RekognitionStreamProcessorSettingsElConnectedHomeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.connected_home", self.base))
    }

    #[doc = "Get a reference to the value of field `face_search` after provisioning.\n"]
    pub fn face_search(&self) -> ListRef<RekognitionStreamProcessorSettingsElFaceSearchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.face_search", self.base))
    }
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessorTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl RekognitionStreamProcessorTimeoutsEl {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for RekognitionStreamProcessorTimeoutsEl {
    type O = BlockAssignable<RekognitionStreamProcessorTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessorTimeoutsEl {}

impl BuildRekognitionStreamProcessorTimeoutsEl {
    pub fn build(self) -> RekognitionStreamProcessorTimeoutsEl {
        RekognitionStreamProcessorTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct RekognitionStreamProcessorTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RekognitionStreamProcessorTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> RekognitionStreamProcessorTimeoutsElRef {
        RekognitionStreamProcessorTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessorTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct RekognitionStreamProcessorDynamic {
    data_sharing_preference: Option<DynamicBlock<RekognitionStreamProcessorDataSharingPreferenceEl>>,
    input: Option<DynamicBlock<RekognitionStreamProcessorInputEl>>,
    notification_channel: Option<DynamicBlock<RekognitionStreamProcessorNotificationChannelEl>>,
    output: Option<DynamicBlock<RekognitionStreamProcessorOutputEl>>,
    regions_of_interest: Option<DynamicBlock<RekognitionStreamProcessorRegionsOfInterestEl>>,
    settings: Option<DynamicBlock<RekognitionStreamProcessorSettingsEl>>,
}
