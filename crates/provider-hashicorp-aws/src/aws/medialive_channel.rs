use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct MedialiveChannelData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    channel_class: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_level: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_channel: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cdi_input_specification: Option<Vec<MedialiveChannelCdiInputSpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destinations: Option<Vec<MedialiveChannelDestinationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoder_settings: Option<Vec<MedialiveChannelEncoderSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_attachments: Option<Vec<MedialiveChannelInputAttachmentsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_specification: Option<Vec<MedialiveChannelInputSpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance: Option<Vec<MedialiveChannelMaintenanceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<MedialiveChannelTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc: Option<Vec<MedialiveChannelVpcEl>>,
    dynamic: MedialiveChannelDynamic,
}
struct MedialiveChannel_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<MedialiveChannelData>,
}
#[derive(Clone)]
pub struct MedialiveChannel(Rc<MedialiveChannel_>);
impl MedialiveChannel {
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
    #[doc = "Set the field `log_level`.\n"]
    pub fn set_log_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().log_level = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `start_channel`.\n"]
    pub fn set_start_channel(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().start_channel = Some(v.into());
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
    #[doc = "Set the field `cdi_input_specification`.\n"]
    pub fn set_cdi_input_specification(
        self,
        v: impl Into<BlockAssignable<MedialiveChannelCdiInputSpecificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cdi_input_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cdi_input_specification = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `destinations`.\n"]
    pub fn set_destinations(
        self,
        v: impl Into<BlockAssignable<MedialiveChannelDestinationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destinations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destinations = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `encoder_settings`.\n"]
    pub fn set_encoder_settings(
        self,
        v: impl Into<BlockAssignable<MedialiveChannelEncoderSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encoder_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encoder_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `input_attachments`.\n"]
    pub fn set_input_attachments(
        self,
        v: impl Into<BlockAssignable<MedialiveChannelInputAttachmentsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().input_attachments = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.input_attachments = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `input_specification`.\n"]
    pub fn set_input_specification(
        self,
        v: impl Into<BlockAssignable<MedialiveChannelInputSpecificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().input_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.input_specification = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `maintenance`.\n"]
    pub fn set_maintenance(
        self,
        v: impl Into<BlockAssignable<MedialiveChannelMaintenanceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<MedialiveChannelTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(self, v: impl Into<BlockAssignable<MedialiveChannelVpcEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `channel_class` after provisioning.\n"]
    pub fn channel_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.channel_class", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `channel_id` after provisioning.\n"]
    pub fn channel_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.channel_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `log_level` after provisioning.\n"]
    pub fn log_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_level", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `start_channel` after provisioning.\n"]
    pub fn start_channel(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_channel", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `cdi_input_specification` after provisioning.\n"]
    pub fn cdi_input_specification(&self) -> ListRef<MedialiveChannelCdiInputSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cdi_input_specification", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encoder_settings` after provisioning.\n"]
    pub fn encoder_settings(&self) -> ListRef<MedialiveChannelEncoderSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encoder_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `input_specification` after provisioning.\n"]
    pub fn input_specification(&self) -> ListRef<MedialiveChannelInputSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_specification", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance` after provisioning.\n"]
    pub fn maintenance(&self) -> ListRef<MedialiveChannelMaintenanceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MedialiveChannelTimeoutsElRef {
        MedialiveChannelTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> ListRef<MedialiveChannelVpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc", self.extract_ref()))
    }
}
impl Referable for MedialiveChannel {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for MedialiveChannel {}
impl ToListMappable for MedialiveChannel {
    type O = ListRef<MedialiveChannelRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for MedialiveChannel_ {
    fn extract_resource_type(&self) -> String {
        "aws_medialive_channel".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildMedialiveChannel {
    pub tf_id: String,
    #[doc = ""]
    pub channel_class: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildMedialiveChannel {
    pub fn build(self, stack: &mut Stack) -> MedialiveChannel {
        let out = MedialiveChannel(Rc::new(MedialiveChannel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(MedialiveChannelData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                channel_class: self.channel_class,
                id: core::default::Default::default(),
                log_level: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                role_arn: core::default::Default::default(),
                start_channel: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                cdi_input_specification: core::default::Default::default(),
                destinations: core::default::Default::default(),
                encoder_settings: core::default::Default::default(),
                input_attachments: core::default::Default::default(),
                input_specification: core::default::Default::default(),
                maintenance: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct MedialiveChannelRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl MedialiveChannelRef {
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
    #[doc = "Get a reference to the value of field `channel_class` after provisioning.\n"]
    pub fn channel_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.channel_class", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `channel_id` after provisioning.\n"]
    pub fn channel_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.channel_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `log_level` after provisioning.\n"]
    pub fn log_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_level", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `start_channel` after provisioning.\n"]
    pub fn start_channel(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_channel", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `cdi_input_specification` after provisioning.\n"]
    pub fn cdi_input_specification(&self) -> ListRef<MedialiveChannelCdiInputSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cdi_input_specification", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encoder_settings` after provisioning.\n"]
    pub fn encoder_settings(&self) -> ListRef<MedialiveChannelEncoderSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encoder_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `input_specification` after provisioning.\n"]
    pub fn input_specification(&self) -> ListRef<MedialiveChannelInputSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_specification", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance` after provisioning.\n"]
    pub fn maintenance(&self) -> ListRef<MedialiveChannelMaintenanceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MedialiveChannelTimeoutsElRef {
        MedialiveChannelTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> ListRef<MedialiveChannelVpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc", self.extract_ref()))
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelCdiInputSpecificationEl {
    resolution: PrimField<String>,
}
impl MedialiveChannelCdiInputSpecificationEl {}
impl ToListMappable for MedialiveChannelCdiInputSpecificationEl {
    type O = BlockAssignable<MedialiveChannelCdiInputSpecificationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelCdiInputSpecificationEl {
    #[doc = ""]
    pub resolution: PrimField<String>,
}
impl BuildMedialiveChannelCdiInputSpecificationEl {
    pub fn build(self) -> MedialiveChannelCdiInputSpecificationEl {
        MedialiveChannelCdiInputSpecificationEl {
            resolution: self.resolution,
        }
    }
}
pub struct MedialiveChannelCdiInputSpecificationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelCdiInputSpecificationElRef {
    fn new(shared: StackShared, base: String) -> MedialiveChannelCdiInputSpecificationElRef {
        MedialiveChannelCdiInputSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelCdiInputSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resolution` after provisioning.\n"]
    pub fn resolution(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resolution", self.base))
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelDestinationsElMediaPackageSettingsEl {
    channel_id: PrimField<String>,
}
impl MedialiveChannelDestinationsElMediaPackageSettingsEl {}
impl ToListMappable for MedialiveChannelDestinationsElMediaPackageSettingsEl {
    type O = BlockAssignable<MedialiveChannelDestinationsElMediaPackageSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelDestinationsElMediaPackageSettingsEl {
    #[doc = ""]
    pub channel_id: PrimField<String>,
}
impl BuildMedialiveChannelDestinationsElMediaPackageSettingsEl {
    pub fn build(self) -> MedialiveChannelDestinationsElMediaPackageSettingsEl {
        MedialiveChannelDestinationsElMediaPackageSettingsEl {
            channel_id: self.channel_id,
        }
    }
}
pub struct MedialiveChannelDestinationsElMediaPackageSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelDestinationsElMediaPackageSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelDestinationsElMediaPackageSettingsElRef {
        MedialiveChannelDestinationsElMediaPackageSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelDestinationsElMediaPackageSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `channel_id` after provisioning.\n"]
    pub fn channel_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.channel_id", self.base))
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelDestinationsElMultiplexSettingsEl {
    multiplex_id: PrimField<String>,
    program_name: PrimField<String>,
}
impl MedialiveChannelDestinationsElMultiplexSettingsEl {}
impl ToListMappable for MedialiveChannelDestinationsElMultiplexSettingsEl {
    type O = BlockAssignable<MedialiveChannelDestinationsElMultiplexSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelDestinationsElMultiplexSettingsEl {
    #[doc = ""]
    pub multiplex_id: PrimField<String>,
    #[doc = ""]
    pub program_name: PrimField<String>,
}
impl BuildMedialiveChannelDestinationsElMultiplexSettingsEl {
    pub fn build(self) -> MedialiveChannelDestinationsElMultiplexSettingsEl {
        MedialiveChannelDestinationsElMultiplexSettingsEl {
            multiplex_id: self.multiplex_id,
            program_name: self.program_name,
        }
    }
}
pub struct MedialiveChannelDestinationsElMultiplexSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelDestinationsElMultiplexSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelDestinationsElMultiplexSettingsElRef {
        MedialiveChannelDestinationsElMultiplexSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelDestinationsElMultiplexSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `multiplex_id` after provisioning.\n"]
    pub fn multiplex_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.multiplex_id", self.base))
    }
    #[doc = "Get a reference to the value of field `program_name` after provisioning.\n"]
    pub fn program_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.program_name", self.base))
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelDestinationsElSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    password_param: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
}
impl MedialiveChannelDestinationsElSettingsEl {
    #[doc = "Set the field `password_param`.\n"]
    pub fn set_password_param(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password_param = Some(v.into());
        self
    }
    #[doc = "Set the field `stream_name`.\n"]
    pub fn set_stream_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stream_name = Some(v.into());
        self
    }
    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }
    #[doc = "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username = Some(v.into());
        self
    }
}
impl ToListMappable for MedialiveChannelDestinationsElSettingsEl {
    type O = BlockAssignable<MedialiveChannelDestinationsElSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelDestinationsElSettingsEl {}
impl BuildMedialiveChannelDestinationsElSettingsEl {
    pub fn build(self) -> MedialiveChannelDestinationsElSettingsEl {
        MedialiveChannelDestinationsElSettingsEl {
            password_param: core::default::Default::default(),
            stream_name: core::default::Default::default(),
            url: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}
pub struct MedialiveChannelDestinationsElSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelDestinationsElSettingsElRef {
    fn new(shared: StackShared, base: String) -> MedialiveChannelDestinationsElSettingsElRef {
        MedialiveChannelDestinationsElSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelDestinationsElSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `password_param` after provisioning.\n"]
    pub fn password_param(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password_param", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `stream_name` after provisioning.\n"]
    pub fn stream_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_name", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize, Default)]
struct MedialiveChannelDestinationsElDynamic {
    media_package_settings:
        Option<DynamicBlock<MedialiveChannelDestinationsElMediaPackageSettingsEl>>,
    multiplex_settings: Option<DynamicBlock<MedialiveChannelDestinationsElMultiplexSettingsEl>>,
    settings: Option<DynamicBlock<MedialiveChannelDestinationsElSettingsEl>>,
}
#[derive(Serialize)]
pub struct MedialiveChannelDestinationsEl {
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    media_package_settings: Option<Vec<MedialiveChannelDestinationsElMediaPackageSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multiplex_settings: Option<Vec<MedialiveChannelDestinationsElMultiplexSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<Vec<MedialiveChannelDestinationsElSettingsEl>>,
    dynamic: MedialiveChannelDestinationsElDynamic,
}
impl MedialiveChannelDestinationsEl {
    #[doc = "Set the field `media_package_settings`.\n"]
    pub fn set_media_package_settings(
        mut self,
        v: impl Into<BlockAssignable<MedialiveChannelDestinationsElMediaPackageSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.media_package_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.media_package_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `multiplex_settings`.\n"]
    pub fn set_multiplex_settings(
        mut self,
        v: impl Into<BlockAssignable<MedialiveChannelDestinationsElMultiplexSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.multiplex_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.multiplex_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(
        mut self,
        v: impl Into<BlockAssignable<MedialiveChannelDestinationsElSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveChannelDestinationsEl {
    type O = BlockAssignable<MedialiveChannelDestinationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelDestinationsEl {
    #[doc = ""]
    pub id: PrimField<String>,
}
impl BuildMedialiveChannelDestinationsEl {
    pub fn build(self) -> MedialiveChannelDestinationsEl {
        MedialiveChannelDestinationsEl {
            id: self.id,
            media_package_settings: core::default::Default::default(),
            multiplex_settings: core::default::Default::default(),
            settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelDestinationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelDestinationsElRef {
    fn new(shared: StackShared, base: String) -> MedialiveChannelDestinationsElRef {
        MedialiveChannelDestinationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelDestinationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `multiplex_settings` after provisioning.\n"]
    pub fn multiplex_settings(
        &self,
    ) -> ListRef<MedialiveChannelDestinationsElMultiplexSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multiplex_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm_control: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_lkfs: Option<PrimField<f64>>,
}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsEl {
    #[doc = "Set the field `algorithm`.\n"]
    pub fn set_algorithm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.algorithm = Some(v.into());
        self
    }
    #[doc = "Set the field `algorithm_control`.\n"]
    pub fn set_algorithm_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.algorithm_control = Some(v.into());
        self
    }
    #[doc = "Set the field `target_lkfs`.\n"]
    pub fn set_target_lkfs(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target_lkfs = Some(v.into());
        self
    }
}
impl ToListMappable
    for MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsEl
{
    type O = BlockAssignable<
        MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsEl {
}
impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsEl {
            algorithm: core::default::Default::default(),
            algorithm_control: core::default::Default::default(),
            target_lkfs: core::default::Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }
    #[doc = "Get a reference to the value of field `algorithm_control` after provisioning.\n"]
    pub fn algorithm_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.algorithm_control", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_lkfs` after provisioning.\n"]
    pub fn target_lkfs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_lkfs", self.base))
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsEl
{
    cbet_check_digit_string: PrimField<String>,
    cbet_stepaside: PrimField<String>,
    csid: PrimField<String>,
}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsEl
{
    #[doc = ""]
    pub cbet_check_digit_string: PrimField<String>,
    #[doc = ""]
    pub cbet_stepaside: PrimField<String>,
    #[doc = ""]
    pub csid: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsEl { MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsEl { cbet_check_digit_string : self . cbet_check_digit_string , cbet_stepaside : self . cbet_stepaside , csid : self . csid , } } }
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsElRef { MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `cbet_check_digit_string` after provisioning.\n"] pub fn cbet_check_digit_string (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.cbet_check_digit_string" , self . base)) } # [doc = "Get a reference to the value of field `cbet_stepaside` after provisioning.\n"] pub fn cbet_stepaside (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.cbet_stepaside" , self . base)) } # [doc = "Get a reference to the value of field `csid` after provisioning.\n"] pub fn csid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.csid" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsEl
{
    check_digit_string: PrimField<String>,
    sid: PrimField<f64>,
}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsEl
{
    #[doc = ""]
    pub check_digit_string: PrimField<String>,
    #[doc = ""]
    pub sid: PrimField<f64>,
}
impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsEl { MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsEl { check_digit_string : self . check_digit_string , sid : self . sid , } } }
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsElRef { MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `check_digit_string` after provisioning.\n"] pub fn check_digit_string (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.check_digit_string" , self . base)) } # [doc = "Get a reference to the value of field `sid` after provisioning.\n"] pub fn sid (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.sid" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElDynamic { nielsen_cbet_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsEl >> , nielsen_naes_ii_nw_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] nielsen_distribution_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] nielsen_cbet_settings : Option < Vec < MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] nielsen_naes_ii_nw_settings : Option < Vec < MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsEl { # [doc = "Set the field `nielsen_distribution_type`.\n"] pub fn set_nielsen_distribution_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . nielsen_distribution_type = Some (v . into ()) ; self } # [doc = "Set the field `nielsen_cbet_settings`.\n"] pub fn set_nielsen_cbet_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . nielsen_cbet_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . nielsen_cbet_settings = Some (d) ; } } self } # [doc = "Set the field `nielsen_naes_ii_nw_settings`.\n"] pub fn set_nielsen_naes_ii_nw_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . nielsen_naes_ii_nw_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . nielsen_naes_ii_nw_settings = Some (d) ; } } self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsEl { MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsEl { nielsen_distribution_type : core :: default :: Default :: default () , nielsen_cbet_settings : core :: default :: Default :: default () , nielsen_naes_ii_nw_settings : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElRef { MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `nielsen_distribution_type` after provisioning.\n"] pub fn nielsen_distribution_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.nielsen_distribution_type" , self . base)) } # [doc = "Get a reference to the value of field `nielsen_cbet_settings` after provisioning.\n"] pub fn nielsen_cbet_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.nielsen_cbet_settings" , self . base)) } # [doc = "Get a reference to the value of field `nielsen_naes_ii_nw_settings` after provisioning.\n"] pub fn nielsen_naes_ii_nw_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.nielsen_naes_ii_nw_settings" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElDynamic { nielsen_watermarks_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] nielsen_watermarks_settings : Option < Vec < MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsEl {
    #[doc = "Set the field `nielsen_watermarks_settings`.\n"]
    pub fn set_nielsen_watermarks_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.nielsen_watermarks_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.nielsen_watermarks_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsEl
{
    type O = BlockAssignable<
        MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsEl {}
impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsEl {
            nielsen_watermarks_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nielsen_watermarks_settings` after provisioning.\n"]    pub fn nielsen_watermarks_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.nielsen_watermarks_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bitrate: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coding_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_control_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_rate: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spec: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vbr_quality: Option<PrimField<String>>,
}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsEl {
    #[doc = "Set the field `bitrate`.\n"]
    pub fn set_bitrate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.bitrate = Some(v.into());
        self
    }
    #[doc = "Set the field `coding_mode`.\n"]
    pub fn set_coding_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.coding_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `input_type`.\n"]
    pub fn set_input_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.input_type = Some(v.into());
        self
    }
    #[doc = "Set the field `profile`.\n"]
    pub fn set_profile(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.profile = Some(v.into());
        self
    }
    #[doc = "Set the field `rate_control_mode`.\n"]
    pub fn set_rate_control_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rate_control_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `raw_format`.\n"]
    pub fn set_raw_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.raw_format = Some(v.into());
        self
    }
    #[doc = "Set the field `sample_rate`.\n"]
    pub fn set_sample_rate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.sample_rate = Some(v.into());
        self
    }
    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.spec = Some(v.into());
        self
    }
    #[doc = "Set the field `vbr_quality`.\n"]
    pub fn set_vbr_quality(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vbr_quality = Some(v.into());
        self
    }
}
impl ToListMappable
    for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsEl
{
    type O = BlockAssignable<
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsEl {
}
impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsEl {
            bitrate: core::default::Default::default(),
            coding_mode: core::default::Default::default(),
            input_type: core::default::Default::default(),
            profile: core::default::Default::default(),
            rate_control_mode: core::default::Default::default(),
            raw_format: core::default::Default::default(),
            sample_rate: core::default::Default::default(),
            spec: core::default::Default::default(),
            vbr_quality: core::default::Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bitrate` after provisioning.\n"]
    pub fn bitrate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.bitrate", self.base))
    }
    #[doc = "Get a reference to the value of field `coding_mode` after provisioning.\n"]
    pub fn coding_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.coding_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `input_type` after provisioning.\n"]
    pub fn input_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.input_type", self.base))
    }
    #[doc = "Get a reference to the value of field `profile` after provisioning.\n"]
    pub fn profile(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.profile", self.base))
    }
    #[doc = "Get a reference to the value of field `rate_control_mode` after provisioning.\n"]
    pub fn rate_control_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rate_control_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `raw_format` after provisioning.\n"]
    pub fn raw_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.raw_format", self.base))
    }
    #[doc = "Get a reference to the value of field `sample_rate` after provisioning.\n"]
    pub fn sample_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.sample_rate", self.base))
    }
    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.spec", self.base))
    }
    #[doc = "Get a reference to the value of field `vbr_quality` after provisioning.\n"]
    pub fn vbr_quality(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vbr_quality", self.base))
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bitrate: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bitstream_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coding_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dialnorm: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drc_profile: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lfe_filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_control: Option<PrimField<String>>,
}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsEl {
    #[doc = "Set the field `bitrate`.\n"]
    pub fn set_bitrate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.bitrate = Some(v.into());
        self
    }
    #[doc = "Set the field `bitstream_mode`.\n"]
    pub fn set_bitstream_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bitstream_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `coding_mode`.\n"]
    pub fn set_coding_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.coding_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `dialnorm`.\n"]
    pub fn set_dialnorm(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.dialnorm = Some(v.into());
        self
    }
    #[doc = "Set the field `drc_profile`.\n"]
    pub fn set_drc_profile(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.drc_profile = Some(v.into());
        self
    }
    #[doc = "Set the field `lfe_filter`.\n"]
    pub fn set_lfe_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lfe_filter = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata_control`.\n"]
    pub fn set_metadata_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metadata_control = Some(v.into());
        self
    }
}
impl ToListMappable
    for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsEl
{
    type O = BlockAssignable<
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsEl {
}
impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsEl {
            bitrate: core::default::Default::default(),
            bitstream_mode: core::default::Default::default(),
            coding_mode: core::default::Default::default(),
            dialnorm: core::default::Default::default(),
            drc_profile: core::default::Default::default(),
            lfe_filter: core::default::Default::default(),
            metadata_control: core::default::Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bitrate` after provisioning.\n"]
    pub fn bitrate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.bitrate", self.base))
    }
    #[doc = "Get a reference to the value of field `bitstream_mode` after provisioning.\n"]
    pub fn bitstream_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bitstream_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `coding_mode` after provisioning.\n"]
    pub fn coding_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.coding_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `dialnorm` after provisioning.\n"]
    pub fn dialnorm(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.dialnorm", self.base))
    }
    #[doc = "Get a reference to the value of field `drc_profile` after provisioning.\n"]
    pub fn drc_profile(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.drc_profile", self.base))
    }
    #[doc = "Get a reference to the value of field `lfe_filter` after provisioning.\n"]
    pub fn lfe_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lfe_filter", self.base))
    }
    #[doc = "Get a reference to the value of field `metadata_control` after provisioning.\n"]
    pub fn metadata_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metadata_control", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bitrate: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coding_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dialnorm: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drc_line: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drc_rf: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height_trim: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    surround_trim: Option<PrimField<f64>>,
}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsEl {
    #[doc = "Set the field `bitrate`.\n"]
    pub fn set_bitrate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.bitrate = Some(v.into());
        self
    }
    #[doc = "Set the field `coding_mode`.\n"]
    pub fn set_coding_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.coding_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `dialnorm`.\n"]
    pub fn set_dialnorm(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.dialnorm = Some(v.into());
        self
    }
    #[doc = "Set the field `drc_line`.\n"]
    pub fn set_drc_line(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.drc_line = Some(v.into());
        self
    }
    #[doc = "Set the field `drc_rf`.\n"]
    pub fn set_drc_rf(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.drc_rf = Some(v.into());
        self
    }
    #[doc = "Set the field `height_trim`.\n"]
    pub fn set_height_trim(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.height_trim = Some(v.into());
        self
    }
    #[doc = "Set the field `surround_trim`.\n"]
    pub fn set_surround_trim(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.surround_trim = Some(v.into());
        self
    }
}
impl ToListMappable
    for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsEl
{
    type O = BlockAssignable<
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsEl
    {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsEl {
            bitrate: core::default::Default::default(),
            coding_mode: core::default::Default::default(),
            dialnorm: core::default::Default::default(),
            drc_line: core::default::Default::default(),
            drc_rf: core::default::Default::default(),
            height_trim: core::default::Default::default(),
            surround_trim: core::default::Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsElRef
    {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bitrate` after provisioning.\n"]
    pub fn bitrate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.bitrate", self.base))
    }
    #[doc = "Get a reference to the value of field `coding_mode` after provisioning.\n"]
    pub fn coding_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.coding_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `dialnorm` after provisioning.\n"]
    pub fn dialnorm(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.dialnorm", self.base))
    }
    #[doc = "Get a reference to the value of field `drc_line` after provisioning.\n"]
    pub fn drc_line(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.drc_line", self.base))
    }
    #[doc = "Get a reference to the value of field `drc_rf` after provisioning.\n"]
    pub fn drc_rf(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.drc_rf", self.base))
    }
    #[doc = "Get a reference to the value of field `height_trim` after provisioning.\n"]
    pub fn height_trim(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.height_trim", self.base))
    }
    #[doc = "Get a reference to the value of field `surround_trim` after provisioning.\n"]
    pub fn surround_trim(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.surround_trim", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attenuation_control: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bitrate: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bitstream_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coding_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dc_filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dialnorm: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drc_line: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drc_rf: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lfe_control: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lfe_filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lo_ro_center_mix_level: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lo_ro_surround_mix_level: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lt_rt_center_mix_level: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lt_rt_surround_mix_level: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_control: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    passthrough_control: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phase_control: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stereo_downmix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    surround_ex_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    surround_mode: Option<PrimField<String>>,
}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsEl {
    #[doc = "Set the field `attenuation_control`.\n"]
    pub fn set_attenuation_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attenuation_control = Some(v.into());
        self
    }
    #[doc = "Set the field `bitrate`.\n"]
    pub fn set_bitrate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.bitrate = Some(v.into());
        self
    }
    #[doc = "Set the field `bitstream_mode`.\n"]
    pub fn set_bitstream_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bitstream_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `coding_mode`.\n"]
    pub fn set_coding_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.coding_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `dc_filter`.\n"]
    pub fn set_dc_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dc_filter = Some(v.into());
        self
    }
    #[doc = "Set the field `dialnorm`.\n"]
    pub fn set_dialnorm(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.dialnorm = Some(v.into());
        self
    }
    #[doc = "Set the field `drc_line`.\n"]
    pub fn set_drc_line(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.drc_line = Some(v.into());
        self
    }
    #[doc = "Set the field `drc_rf`.\n"]
    pub fn set_drc_rf(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.drc_rf = Some(v.into());
        self
    }
    #[doc = "Set the field `lfe_control`.\n"]
    pub fn set_lfe_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lfe_control = Some(v.into());
        self
    }
    #[doc = "Set the field `lfe_filter`.\n"]
    pub fn set_lfe_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lfe_filter = Some(v.into());
        self
    }
    #[doc = "Set the field `lo_ro_center_mix_level`.\n"]
    pub fn set_lo_ro_center_mix_level(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.lo_ro_center_mix_level = Some(v.into());
        self
    }
    #[doc = "Set the field `lo_ro_surround_mix_level`.\n"]
    pub fn set_lo_ro_surround_mix_level(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.lo_ro_surround_mix_level = Some(v.into());
        self
    }
    #[doc = "Set the field `lt_rt_center_mix_level`.\n"]
    pub fn set_lt_rt_center_mix_level(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.lt_rt_center_mix_level = Some(v.into());
        self
    }
    #[doc = "Set the field `lt_rt_surround_mix_level`.\n"]
    pub fn set_lt_rt_surround_mix_level(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.lt_rt_surround_mix_level = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata_control`.\n"]
    pub fn set_metadata_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metadata_control = Some(v.into());
        self
    }
    #[doc = "Set the field `passthrough_control`.\n"]
    pub fn set_passthrough_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.passthrough_control = Some(v.into());
        self
    }
    #[doc = "Set the field `phase_control`.\n"]
    pub fn set_phase_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.phase_control = Some(v.into());
        self
    }
    #[doc = "Set the field `stereo_downmix`.\n"]
    pub fn set_stereo_downmix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stereo_downmix = Some(v.into());
        self
    }
    #[doc = "Set the field `surround_ex_mode`.\n"]
    pub fn set_surround_ex_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.surround_ex_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `surround_mode`.\n"]
    pub fn set_surround_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.surround_mode = Some(v.into());
        self
    }
}
impl ToListMappable
    for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsEl
{
    type O = BlockAssignable<
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsEl {
}
impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsEl {
            attenuation_control: core::default::Default::default(),
            bitrate: core::default::Default::default(),
            bitstream_mode: core::default::Default::default(),
            coding_mode: core::default::Default::default(),
            dc_filter: core::default::Default::default(),
            dialnorm: core::default::Default::default(),
            drc_line: core::default::Default::default(),
            drc_rf: core::default::Default::default(),
            lfe_control: core::default::Default::default(),
            lfe_filter: core::default::Default::default(),
            lo_ro_center_mix_level: core::default::Default::default(),
            lo_ro_surround_mix_level: core::default::Default::default(),
            lt_rt_center_mix_level: core::default::Default::default(),
            lt_rt_surround_mix_level: core::default::Default::default(),
            metadata_control: core::default::Default::default(),
            passthrough_control: core::default::Default::default(),
            phase_control: core::default::Default::default(),
            stereo_downmix: core::default::Default::default(),
            surround_ex_mode: core::default::Default::default(),
            surround_mode: core::default::Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `attenuation_control` after provisioning.\n"]
    pub fn attenuation_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.attenuation_control", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bitrate` after provisioning.\n"]
    pub fn bitrate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.bitrate", self.base))
    }
    #[doc = "Get a reference to the value of field `bitstream_mode` after provisioning.\n"]
    pub fn bitstream_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bitstream_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `coding_mode` after provisioning.\n"]
    pub fn coding_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.coding_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `dc_filter` after provisioning.\n"]
    pub fn dc_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dc_filter", self.base))
    }
    #[doc = "Get a reference to the value of field `dialnorm` after provisioning.\n"]
    pub fn dialnorm(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.dialnorm", self.base))
    }
    #[doc = "Get a reference to the value of field `drc_line` after provisioning.\n"]
    pub fn drc_line(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.drc_line", self.base))
    }
    #[doc = "Get a reference to the value of field `drc_rf` after provisioning.\n"]
    pub fn drc_rf(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.drc_rf", self.base))
    }
    #[doc = "Get a reference to the value of field `lfe_control` after provisioning.\n"]
    pub fn lfe_control(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lfe_control", self.base))
    }
    #[doc = "Get a reference to the value of field `lfe_filter` after provisioning.\n"]
    pub fn lfe_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lfe_filter", self.base))
    }
    #[doc = "Get a reference to the value of field `lo_ro_center_mix_level` after provisioning.\n"]
    pub fn lo_ro_center_mix_level(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lo_ro_center_mix_level", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lo_ro_surround_mix_level` after provisioning.\n"]
    pub fn lo_ro_surround_mix_level(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lo_ro_surround_mix_level", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lt_rt_center_mix_level` after provisioning.\n"]
    pub fn lt_rt_center_mix_level(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lt_rt_center_mix_level", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lt_rt_surround_mix_level` after provisioning.\n"]
    pub fn lt_rt_surround_mix_level(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lt_rt_surround_mix_level", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metadata_control` after provisioning.\n"]
    pub fn metadata_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metadata_control", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `passthrough_control` after provisioning.\n"]
    pub fn passthrough_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.passthrough_control", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `phase_control` after provisioning.\n"]
    pub fn phase_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.phase_control", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `stereo_downmix` after provisioning.\n"]
    pub fn stereo_downmix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stereo_downmix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `surround_ex_mode` after provisioning.\n"]
    pub fn surround_ex_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.surround_ex_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `surround_mode` after provisioning.\n"]
    pub fn surround_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.surround_mode", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bitrate: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coding_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_rate: Option<PrimField<f64>>,
}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsEl {
    #[doc = "Set the field `bitrate`.\n"]
    pub fn set_bitrate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.bitrate = Some(v.into());
        self
    }
    #[doc = "Set the field `coding_mode`.\n"]
    pub fn set_coding_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.coding_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `sample_rate`.\n"]
    pub fn set_sample_rate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.sample_rate = Some(v.into());
        self
    }
}
impl ToListMappable
    for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsEl
{
    type O = BlockAssignable<
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsEl {
}
impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsEl {
            bitrate: core::default::Default::default(),
            coding_mode: core::default::Default::default(),
            sample_rate: core::default::Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bitrate` after provisioning.\n"]
    pub fn bitrate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.bitrate", self.base))
    }
    #[doc = "Get a reference to the value of field `coding_mode` after provisioning.\n"]
    pub fn coding_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.coding_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `sample_rate` after provisioning.\n"]
    pub fn sample_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.sample_rate", self.base))
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsEl
{}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsEl {}
impl ToListMappable
    for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsEl
{
    type O = BlockAssignable<
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsEl
    {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsEl {}
    }
}
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsElRef
    {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsElRef { shared : shared , base : base . to_string () , }
    }
}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bit_depth: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coding_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_rate: Option<PrimField<f64>>,
}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsEl {
    #[doc = "Set the field `bit_depth`.\n"]
    pub fn set_bit_depth(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.bit_depth = Some(v.into());
        self
    }
    #[doc = "Set the field `coding_mode`.\n"]
    pub fn set_coding_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.coding_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `sample_rate`.\n"]
    pub fn set_sample_rate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.sample_rate = Some(v.into());
        self
    }
}
impl ToListMappable
    for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsEl
{
    type O = BlockAssignable<
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsEl {
}
impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsEl {
            bit_depth: core::default::Default::default(),
            coding_mode: core::default::Default::default(),
            sample_rate: core::default::Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bit_depth` after provisioning.\n"]
    pub fn bit_depth(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.bit_depth", self.base))
    }
    #[doc = "Get a reference to the value of field `coding_mode` after provisioning.\n"]
    pub fn coding_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.coding_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `sample_rate` after provisioning.\n"]
    pub fn sample_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.sample_rate", self.base))
    }
}
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElDynamic { aac_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsEl >> , ac3_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsEl >> , eac3_atmos_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsEl >> , eac3_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsEl >> , mp2_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsEl >> , pass_through_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsEl >> , wav_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] aac_settings : Option < Vec < MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] ac3_settings : Option < Vec < MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] eac3_atmos_settings : Option < Vec < MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] eac3_settings : Option < Vec < MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] mp2_settings : Option < Vec < MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] pass_through_settings : Option < Vec < MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] wav_settings : Option < Vec < MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsEl {
    #[doc = "Set the field `aac_settings`.\n"]
    pub fn set_aac_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aac_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aac_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ac3_settings`.\n"]
    pub fn set_ac3_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ac3_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ac3_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `eac3_atmos_settings`.\n"]
    pub fn set_eac3_atmos_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.eac3_atmos_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.eac3_atmos_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `eac3_settings`.\n"]
    pub fn set_eac3_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.eac3_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.eac3_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `mp2_settings`.\n"]
    pub fn set_mp2_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mp2_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mp2_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `pass_through_settings`.\n"]
    pub fn set_pass_through_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pass_through_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pass_through_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `wav_settings`.\n"]
    pub fn set_wav_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.wav_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.wav_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsEl {
    type O = BlockAssignable<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsEl {}
impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsEl {
            aac_settings: core::default::Default::default(),
            ac3_settings: core::default::Default::default(),
            eac3_atmos_settings: core::default::Default::default(),
            eac3_settings: core::default::Default::default(),
            mp2_settings: core::default::Default::default(),
            pass_through_settings: core::default::Default::default(),
            wav_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aac_settings` after provisioning.\n"]
    pub fn aac_settings(
        &self,
    ) -> ListRef<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.aac_settings", self.base))
    }
    #[doc = "Get a reference to the value of field `ac3_settings` after provisioning.\n"]
    pub fn ac3_settings(
        &self,
    ) -> ListRef<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.ac3_settings", self.base))
    }
    #[doc = "Get a reference to the value of field `eac3_atmos_settings` after provisioning.\n"]
    pub fn eac3_atmos_settings(
        &self,
    ) -> ListRef<
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.eac3_atmos_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `eac3_settings` after provisioning.\n"]
    pub fn eac3_settings(
        &self,
    ) -> ListRef<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.eac3_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mp2_settings` after provisioning.\n"]
    pub fn mp2_settings(
        &self,
    ) -> ListRef<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.mp2_settings", self.base))
    }
    #[doc = "Get a reference to the value of field `pass_through_settings` after provisioning.\n"]
    pub fn pass_through_settings(
        &self,
    ) -> ListRef<
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pass_through_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `wav_settings` after provisioning.\n"]
    pub fn wav_settings(
        &self,
    ) -> ListRef<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.wav_settings", self.base))
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsEl
{
    gain: PrimField<f64>,
    input_channel: PrimField<f64>,
}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsEl
{
    #[doc = ""]
    pub gain: PrimField<f64>,
    #[doc = ""]
    pub input_channel: PrimField<f64>,
}
impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsEl { MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsEl { gain : self . gain , input_channel : self . input_channel , } } }
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsElRef { MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `gain` after provisioning.\n"] pub fn gain (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.gain" , self . base)) } # [doc = "Get a reference to the value of field `input_channel` after provisioning.\n"] pub fn input_channel (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.input_channel" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElDynamic { input_channel_levels : Option < DynamicBlock < MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsEl { output_channel : PrimField < f64 > , # [serde (skip_serializing_if = "Option::is_none")] input_channel_levels : Option < Vec < MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsEl > > , dynamic : MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElDynamic , }
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsEl {
    #[doc = "Set the field `input_channel_levels`.\n"]
    pub fn set_input_channel_levels(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.input_channel_levels = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.input_channel_levels = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsEl
{
    type O = BlockAssignable<
        MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsEl
{
    #[doc = ""]
    pub output_channel: PrimField<f64>,
}
impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsEl {
            output_channel: self.output_channel,
            input_channel_levels: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElRef
    {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `output_channel` after provisioning.\n"]
    pub fn output_channel(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_channel", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElDynamic {
    channel_mappings: Option<
        DynamicBlock<
            MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    channels_in: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channels_out: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_mappings: Option<
        Vec<MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsEl>,
    >,
    dynamic: MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElDynamic,
}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsEl {
    #[doc = "Set the field `channels_in`.\n"]
    pub fn set_channels_in(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.channels_in = Some(v.into());
        self
    }
    #[doc = "Set the field `channels_out`.\n"]
    pub fn set_channels_out(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.channels_out = Some(v.into());
        self
    }
    #[doc = "Set the field `channel_mappings`.\n"]
    pub fn set_channel_mappings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.channel_mappings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.channel_mappings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsEl {
    type O = BlockAssignable<MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsEl {}
impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsEl {
            channels_in: core::default::Default::default(),
            channels_out: core::default::Default::default(),
            channel_mappings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `channels_in` after provisioning.\n"]
    pub fn channels_in(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.channels_in", self.base))
    }
    #[doc = "Get a reference to the value of field `channels_out` after provisioning.\n"]
    pub fn channels_out(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.channels_out", self.base))
    }
}
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElAudioDescriptionsElDynamic {
    audio_normalization_settings: Option<
        DynamicBlock<
            MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsEl,
        >,
    >,
    audio_watermark_settings: Option<
        DynamicBlock<MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsEl>,
    >,
    codec_settings:
        Option<DynamicBlock<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsEl>>,
    remix_settings:
        Option<DynamicBlock<MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsEl>>,
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsEl {
    audio_selector_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_type_control: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code_control: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_normalization_settings: Option<
        Vec<MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_watermark_settings:
        Option<Vec<MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    codec_settings:
        Option<Vec<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remix_settings:
        Option<Vec<MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsEl>>,
    dynamic: MedialiveChannelEncoderSettingsElAudioDescriptionsElDynamic,
}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsEl {
    #[doc = "Set the field `audio_type`.\n"]
    pub fn set_audio_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.audio_type = Some(v.into());
        self
    }
    #[doc = "Set the field `audio_type_control`.\n"]
    pub fn set_audio_type_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.audio_type_control = Some(v.into());
        self
    }
    #[doc = "Set the field `language_code`.\n"]
    pub fn set_language_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.language_code = Some(v.into());
        self
    }
    #[doc = "Set the field `language_code_control`.\n"]
    pub fn set_language_code_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.language_code_control = Some(v.into());
        self
    }
    #[doc = "Set the field `stream_name`.\n"]
    pub fn set_stream_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stream_name = Some(v.into());
        self
    }
    #[doc = "Set the field `audio_normalization_settings`.\n"]
    pub fn set_audio_normalization_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.audio_normalization_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_normalization_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `audio_watermark_settings`.\n"]
    pub fn set_audio_watermark_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.audio_watermark_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_watermark_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `codec_settings`.\n"]
    pub fn set_codec_settings(
        mut self,
        v: impl Into<
            BlockAssignable<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.codec_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.codec_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `remix_settings`.\n"]
    pub fn set_remix_settings(
        mut self,
        v: impl Into<
            BlockAssignable<MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.remix_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.remix_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveChannelEncoderSettingsElAudioDescriptionsEl {
    type O = BlockAssignable<MedialiveChannelEncoderSettingsElAudioDescriptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsEl {
    #[doc = ""]
    pub audio_selector_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElAudioDescriptionsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsEl {
            audio_selector_name: self.audio_selector_name,
            audio_type: core::default::Default::default(),
            audio_type_control: core::default::Default::default(),
            language_code: core::default::Default::default(),
            language_code_control: core::default::Default::default(),
            name: self.name,
            stream_name: core::default::Default::default(),
            audio_normalization_settings: core::default::Default::default(),
            audio_watermark_settings: core::default::Default::default(),
            codec_settings: core::default::Default::default(),
            remix_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElAudioDescriptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `audio_selector_name` after provisioning.\n"]
    pub fn audio_selector_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.audio_selector_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `audio_type` after provisioning.\n"]
    pub fn audio_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.audio_type", self.base))
    }
    #[doc = "Get a reference to the value of field `audio_type_control` after provisioning.\n"]
    pub fn audio_type_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.audio_type_control", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `language_code_control` after provisioning.\n"]
    pub fn language_code_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code_control", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `stream_name` after provisioning.\n"]
    pub fn stream_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_name", self.base))
    }
    #[doc = "Get a reference to the value of field `audio_normalization_settings` after provisioning.\n"]
    pub fn audio_normalization_settings(
        &self,
    ) -> ListRef<MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.audio_normalization_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `audio_watermark_settings` after provisioning.\n"]
    pub fn audio_watermark_settings(
        &self,
    ) -> ListRef<MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.audio_watermark_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `codec_settings` after provisioning.\n"]
    pub fn codec_settings(
        &self,
    ) -> ListRef<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.codec_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `remix_settings` after provisioning.\n"]
    pub fn remix_settings(
        &self,
    ) -> ListRef<MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.remix_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    password_param: Option<PrimField<String>>,
    uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
}
impl MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageEl {
    #[doc = "Set the field `password_param`.\n"]
    pub fn set_password_param(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password_param = Some(v.into());
        self
    }
    #[doc = "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username = Some(v.into());
        self
    }
}
impl ToListMappable for MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageEl {
    type O = BlockAssignable<MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageEl {
    #[doc = ""]
    pub uri: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageEl {
        MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageEl {
            password_param: core::default::Default::default(),
            uri: self.uri,
            username: core::default::Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageElRef {
        MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `password_param` after provisioning.\n"]
    pub fn password_param(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password_param", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElAvailBlankingElDynamic {
    avail_blanking_image:
        Option<DynamicBlock<MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageEl>>,
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAvailBlankingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avail_blanking_image:
        Option<Vec<MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageEl>>,
    dynamic: MedialiveChannelEncoderSettingsElAvailBlankingElDynamic,
}
impl MedialiveChannelEncoderSettingsElAvailBlankingEl {
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `avail_blanking_image`.\n"]
    pub fn set_avail_blanking_image(
        mut self,
        v: impl Into<
            BlockAssignable<MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.avail_blanking_image = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.avail_blanking_image = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveChannelEncoderSettingsElAvailBlankingEl {
    type O = BlockAssignable<MedialiveChannelEncoderSettingsElAvailBlankingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElAvailBlankingEl {}
impl BuildMedialiveChannelEncoderSettingsElAvailBlankingEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElAvailBlankingEl {
        MedialiveChannelEncoderSettingsElAvailBlankingEl {
            state: core::default::Default::default(),
            avail_blanking_image: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElAvailBlankingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElAvailBlankingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElAvailBlankingElRef {
        MedialiveChannelEncoderSettingsElAvailBlankingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElAvailBlankingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `avail_blanking_image` after provisioning.\n"]
    pub fn avail_blanking_image(
        &self,
    ) -> ListRef<MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.avail_blanking_image", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElAribDestinationSettingsEl
{}
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElAribDestinationSettingsEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElAribDestinationSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElAribDestinationSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElAribDestinationSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElAribDestinationSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElAribDestinationSettingsEl { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElAribDestinationSettingsEl { } } }
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElAribDestinationSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElAribDestinationSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElAribDestinationSettingsElRef { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElAribDestinationSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElAribDestinationSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsElFontEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    password_param: Option<PrimField<String>>,
    uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
}
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsElFontEl { # [doc = "Set the field `password_param`.\n"] pub fn set_password_param (mut self , v : impl Into < PrimField < String > >) -> Self { self . password_param = Some (v . into ()) ; self } # [doc = "Set the field `username`.\n"] pub fn set_username (mut self , v : impl Into < PrimField < String > >) -> Self { self . username = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsElFontEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsElFontEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsElFontEl
{
    #[doc = ""]
    pub uri: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsElFontEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsElFontEl { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsElFontEl { password_param : core :: default :: Default :: default () , uri : self . uri , username : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsElFontElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsElFontElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsElFontElRef { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsElFontElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsElFontElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `password_param` after provisioning.\n"] pub fn password_param (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.password_param" , self . base)) } # [doc = "Get a reference to the value of field `uri` after provisioning.\n"] pub fn uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.uri" , self . base)) } # [doc = "Get a reference to the value of field `username` after provisioning.\n"] pub fn username (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.username" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsElDynamic { font : Option < DynamicBlock < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsElFontEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] alignment : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] background_color : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] background_opacity : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] font_color : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] font_opacity : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] font_resolution : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] font_size : Option < PrimField < String > > , outline_color : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] outline_size : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] shadow_color : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] shadow_opacity : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] shadow_x_offset : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] shadow_y_offset : Option < PrimField < f64 > > , teletext_grid_control : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] x_position : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] y_position : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] font : Option < Vec < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsElFontEl > > , dynamic : MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsEl { # [doc = "Set the field `alignment`.\n"] pub fn set_alignment (mut self , v : impl Into < PrimField < String > >) -> Self { self . alignment = Some (v . into ()) ; self } # [doc = "Set the field `background_color`.\n"] pub fn set_background_color (mut self , v : impl Into < PrimField < String > >) -> Self { self . background_color = Some (v . into ()) ; self } # [doc = "Set the field `background_opacity`.\n"] pub fn set_background_opacity (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . background_opacity = Some (v . into ()) ; self } # [doc = "Set the field `font_color`.\n"] pub fn set_font_color (mut self , v : impl Into < PrimField < String > >) -> Self { self . font_color = Some (v . into ()) ; self } # [doc = "Set the field `font_opacity`.\n"] pub fn set_font_opacity (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . font_opacity = Some (v . into ()) ; self } # [doc = "Set the field `font_resolution`.\n"] pub fn set_font_resolution (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . font_resolution = Some (v . into ()) ; self } # [doc = "Set the field `font_size`.\n"] pub fn set_font_size (mut self , v : impl Into < PrimField < String > >) -> Self { self . font_size = Some (v . into ()) ; self } # [doc = "Set the field `outline_size`.\n"] pub fn set_outline_size (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . outline_size = Some (v . into ()) ; self } # [doc = "Set the field `shadow_color`.\n"] pub fn set_shadow_color (mut self , v : impl Into < PrimField < String > >) -> Self { self . shadow_color = Some (v . into ()) ; self } # [doc = "Set the field `shadow_opacity`.\n"] pub fn set_shadow_opacity (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . shadow_opacity = Some (v . into ()) ; self } # [doc = "Set the field `shadow_x_offset`.\n"] pub fn set_shadow_x_offset (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . shadow_x_offset = Some (v . into ()) ; self } # [doc = "Set the field `shadow_y_offset`.\n"] pub fn set_shadow_y_offset (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . shadow_y_offset = Some (v . into ()) ; self } # [doc = "Set the field `x_position`.\n"] pub fn set_x_position (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . x_position = Some (v . into ()) ; self } # [doc = "Set the field `y_position`.\n"] pub fn set_y_position (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . y_position = Some (v . into ()) ; self } # [doc = "Set the field `font`.\n"] pub fn set_font (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsElFontEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . font = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . font = Some (d) ; } } self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsEl
{
    #[doc = ""]
    pub outline_color: PrimField<String>,
    #[doc = ""]
    pub teletext_grid_control: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsEl { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsEl { alignment : core :: default :: Default :: default () , background_color : core :: default :: Default :: default () , background_opacity : core :: default :: Default :: default () , font_color : core :: default :: Default :: default () , font_opacity : core :: default :: Default :: default () , font_resolution : core :: default :: Default :: default () , font_size : core :: default :: Default :: default () , outline_color : self . outline_color , outline_size : core :: default :: Default :: default () , shadow_color : core :: default :: Default :: default () , shadow_opacity : core :: default :: Default :: default () , shadow_x_offset : core :: default :: Default :: default () , shadow_y_offset : core :: default :: Default :: default () , teletext_grid_control : self . teletext_grid_control , x_position : core :: default :: Default :: default () , y_position : core :: default :: Default :: default () , font : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsElRef { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `alignment` after provisioning.\n"] pub fn alignment (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.alignment" , self . base)) } # [doc = "Get a reference to the value of field `background_color` after provisioning.\n"] pub fn background_color (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.background_color" , self . base)) } # [doc = "Get a reference to the value of field `background_opacity` after provisioning.\n"] pub fn background_opacity (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.background_opacity" , self . base)) } # [doc = "Get a reference to the value of field `font_color` after provisioning.\n"] pub fn font_color (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.font_color" , self . base)) } # [doc = "Get a reference to the value of field `font_opacity` after provisioning.\n"] pub fn font_opacity (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.font_opacity" , self . base)) } # [doc = "Get a reference to the value of field `font_resolution` after provisioning.\n"] pub fn font_resolution (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.font_resolution" , self . base)) } # [doc = "Get a reference to the value of field `font_size` after provisioning.\n"] pub fn font_size (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.font_size" , self . base)) } # [doc = "Get a reference to the value of field `outline_color` after provisioning.\n"] pub fn outline_color (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.outline_color" , self . base)) } # [doc = "Get a reference to the value of field `outline_size` after provisioning.\n"] pub fn outline_size (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.outline_size" , self . base)) } # [doc = "Get a reference to the value of field `shadow_color` after provisioning.\n"] pub fn shadow_color (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.shadow_color" , self . base)) } # [doc = "Get a reference to the value of field `shadow_opacity` after provisioning.\n"] pub fn shadow_opacity (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.shadow_opacity" , self . base)) } # [doc = "Get a reference to the value of field `shadow_x_offset` after provisioning.\n"] pub fn shadow_x_offset (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.shadow_x_offset" , self . base)) } # [doc = "Get a reference to the value of field `shadow_y_offset` after provisioning.\n"] pub fn shadow_y_offset (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.shadow_y_offset" , self . base)) } # [doc = "Get a reference to the value of field `teletext_grid_control` after provisioning.\n"] pub fn teletext_grid_control (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.teletext_grid_control" , self . base)) } # [doc = "Get a reference to the value of field `x_position` after provisioning.\n"] pub fn x_position (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.x_position" , self . base)) } # [doc = "Get a reference to the value of field `y_position` after provisioning.\n"] pub fn y_position (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.y_position" , self . base)) } # [doc = "Get a reference to the value of field `font` after provisioning.\n"] pub fn font (& self) -> ListRef < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsElFontElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.font" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsElFontEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    password_param: Option<PrimField<String>>,
    uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
}
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsElFontEl { # [doc = "Set the field `password_param`.\n"] pub fn set_password_param (mut self , v : impl Into < PrimField < String > >) -> Self { self . password_param = Some (v . into ()) ; self } # [doc = "Set the field `username`.\n"] pub fn set_username (mut self , v : impl Into < PrimField < String > >) -> Self { self . username = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsElFontEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsElFontEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsElFontEl
{
    #[doc = ""]
    pub uri: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsElFontEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsElFontEl { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsElFontEl { password_param : core :: default :: Default :: default () , uri : self . uri , username : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsElFontElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsElFontElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsElFontElRef { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsElFontElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsElFontElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `password_param` after provisioning.\n"] pub fn password_param (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.password_param" , self . base)) } # [doc = "Get a reference to the value of field `uri` after provisioning.\n"] pub fn uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.uri" , self . base)) } # [doc = "Get a reference to the value of field `username` after provisioning.\n"] pub fn username (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.username" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsElDynamic { font : Option < DynamicBlock < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsElFontEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] alignment : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] background_color : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] background_opacity : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] font_color : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] font_opacity : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] font_resolution : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] font_size : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] outline_color : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] outline_size : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] shadow_color : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] shadow_opacity : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] shadow_x_offset : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] shadow_y_offset : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] teletext_grid_control : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] x_position : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] y_position : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] font : Option < Vec < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsElFontEl > > , dynamic : MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsEl { # [doc = "Set the field `alignment`.\n"] pub fn set_alignment (mut self , v : impl Into < PrimField < String > >) -> Self { self . alignment = Some (v . into ()) ; self } # [doc = "Set the field `background_color`.\n"] pub fn set_background_color (mut self , v : impl Into < PrimField < String > >) -> Self { self . background_color = Some (v . into ()) ; self } # [doc = "Set the field `background_opacity`.\n"] pub fn set_background_opacity (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . background_opacity = Some (v . into ()) ; self } # [doc = "Set the field `font_color`.\n"] pub fn set_font_color (mut self , v : impl Into < PrimField < String > >) -> Self { self . font_color = Some (v . into ()) ; self } # [doc = "Set the field `font_opacity`.\n"] pub fn set_font_opacity (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . font_opacity = Some (v . into ()) ; self } # [doc = "Set the field `font_resolution`.\n"] pub fn set_font_resolution (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . font_resolution = Some (v . into ()) ; self } # [doc = "Set the field `font_size`.\n"] pub fn set_font_size (mut self , v : impl Into < PrimField < String > >) -> Self { self . font_size = Some (v . into ()) ; self } # [doc = "Set the field `outline_color`.\n"] pub fn set_outline_color (mut self , v : impl Into < PrimField < String > >) -> Self { self . outline_color = Some (v . into ()) ; self } # [doc = "Set the field `outline_size`.\n"] pub fn set_outline_size (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . outline_size = Some (v . into ()) ; self } # [doc = "Set the field `shadow_color`.\n"] pub fn set_shadow_color (mut self , v : impl Into < PrimField < String > >) -> Self { self . shadow_color = Some (v . into ()) ; self } # [doc = "Set the field `shadow_opacity`.\n"] pub fn set_shadow_opacity (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . shadow_opacity = Some (v . into ()) ; self } # [doc = "Set the field `shadow_x_offset`.\n"] pub fn set_shadow_x_offset (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . shadow_x_offset = Some (v . into ()) ; self } # [doc = "Set the field `shadow_y_offset`.\n"] pub fn set_shadow_y_offset (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . shadow_y_offset = Some (v . into ()) ; self } # [doc = "Set the field `teletext_grid_control`.\n"] pub fn set_teletext_grid_control (mut self , v : impl Into < PrimField < String > >) -> Self { self . teletext_grid_control = Some (v . into ()) ; self } # [doc = "Set the field `x_position`.\n"] pub fn set_x_position (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . x_position = Some (v . into ()) ; self } # [doc = "Set the field `y_position`.\n"] pub fn set_y_position (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . y_position = Some (v . into ()) ; self } # [doc = "Set the field `font`.\n"] pub fn set_font (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsElFontEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . font = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . font = Some (d) ; } } self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsEl { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsEl { alignment : core :: default :: Default :: default () , background_color : core :: default :: Default :: default () , background_opacity : core :: default :: Default :: default () , font_color : core :: default :: Default :: default () , font_opacity : core :: default :: Default :: default () , font_resolution : core :: default :: Default :: default () , font_size : core :: default :: Default :: default () , outline_color : core :: default :: Default :: default () , outline_size : core :: default :: Default :: default () , shadow_color : core :: default :: Default :: default () , shadow_opacity : core :: default :: Default :: default () , shadow_x_offset : core :: default :: Default :: default () , shadow_y_offset : core :: default :: Default :: default () , teletext_grid_control : core :: default :: Default :: default () , x_position : core :: default :: Default :: default () , y_position : core :: default :: Default :: default () , font : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsElRef { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `alignment` after provisioning.\n"] pub fn alignment (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.alignment" , self . base)) } # [doc = "Get a reference to the value of field `background_color` after provisioning.\n"] pub fn background_color (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.background_color" , self . base)) } # [doc = "Get a reference to the value of field `background_opacity` after provisioning.\n"] pub fn background_opacity (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.background_opacity" , self . base)) } # [doc = "Get a reference to the value of field `font_color` after provisioning.\n"] pub fn font_color (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.font_color" , self . base)) } # [doc = "Get a reference to the value of field `font_opacity` after provisioning.\n"] pub fn font_opacity (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.font_opacity" , self . base)) } # [doc = "Get a reference to the value of field `font_resolution` after provisioning.\n"] pub fn font_resolution (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.font_resolution" , self . base)) } # [doc = "Get a reference to the value of field `font_size` after provisioning.\n"] pub fn font_size (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.font_size" , self . base)) } # [doc = "Get a reference to the value of field `outline_color` after provisioning.\n"] pub fn outline_color (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.outline_color" , self . base)) } # [doc = "Get a reference to the value of field `outline_size` after provisioning.\n"] pub fn outline_size (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.outline_size" , self . base)) } # [doc = "Get a reference to the value of field `shadow_color` after provisioning.\n"] pub fn shadow_color (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.shadow_color" , self . base)) } # [doc = "Get a reference to the value of field `shadow_opacity` after provisioning.\n"] pub fn shadow_opacity (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.shadow_opacity" , self . base)) } # [doc = "Get a reference to the value of field `shadow_x_offset` after provisioning.\n"] pub fn shadow_x_offset (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.shadow_x_offset" , self . base)) } # [doc = "Get a reference to the value of field `shadow_y_offset` after provisioning.\n"] pub fn shadow_y_offset (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.shadow_y_offset" , self . base)) } # [doc = "Get a reference to the value of field `teletext_grid_control` after provisioning.\n"] pub fn teletext_grid_control (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.teletext_grid_control" , self . base)) } # [doc = "Get a reference to the value of field `x_position` after provisioning.\n"] pub fn x_position (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.x_position" , self . base)) } # [doc = "Get a reference to the value of field `y_position` after provisioning.\n"] pub fn y_position (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.y_position" , self . base)) } # [doc = "Get a reference to the value of field `font` after provisioning.\n"] pub fn font (& self) -> ListRef < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsElFontElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.font" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEbuTtDDestinationSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    copyright_holder: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fill_line_gap: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    font_family: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    style_control: Option<PrimField<String>>,
}
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEbuTtDDestinationSettingsEl { # [doc = "Set the field `copyright_holder`.\n"] pub fn set_copyright_holder (mut self , v : impl Into < PrimField < String > >) -> Self { self . copyright_holder = Some (v . into ()) ; self } # [doc = "Set the field `fill_line_gap`.\n"] pub fn set_fill_line_gap (mut self , v : impl Into < PrimField < String > >) -> Self { self . fill_line_gap = Some (v . into ()) ; self } # [doc = "Set the field `font_family`.\n"] pub fn set_font_family (mut self , v : impl Into < PrimField < String > >) -> Self { self . font_family = Some (v . into ()) ; self } # [doc = "Set the field `style_control`.\n"] pub fn set_style_control (mut self , v : impl Into < PrimField < String > >) -> Self { self . style_control = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEbuTtDDestinationSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEbuTtDDestinationSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEbuTtDDestinationSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEbuTtDDestinationSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEbuTtDDestinationSettingsEl { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEbuTtDDestinationSettingsEl { copyright_holder : core :: default :: Default :: default () , fill_line_gap : core :: default :: Default :: default () , font_family : core :: default :: Default :: default () , style_control : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEbuTtDDestinationSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEbuTtDDestinationSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEbuTtDDestinationSettingsElRef { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEbuTtDDestinationSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEbuTtDDestinationSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `copyright_holder` after provisioning.\n"] pub fn copyright_holder (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.copyright_holder" , self . base)) } # [doc = "Get a reference to the value of field `fill_line_gap` after provisioning.\n"] pub fn fill_line_gap (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.fill_line_gap" , self . base)) } # [doc = "Get a reference to the value of field `font_family` after provisioning.\n"] pub fn font_family (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.font_family" , self . base)) } # [doc = "Get a reference to the value of field `style_control` after provisioning.\n"] pub fn style_control (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.style_control" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedDestinationSettingsEl
{}
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedDestinationSettingsEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedDestinationSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedDestinationSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedDestinationSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedDestinationSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedDestinationSettingsEl { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedDestinationSettingsEl { } } }
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedDestinationSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedDestinationSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedDestinationSettingsElRef { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedDestinationSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedDestinationSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedPlusScte20DestinationSettingsEl
{}
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedPlusScte20DestinationSettingsEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedPlusScte20DestinationSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedPlusScte20DestinationSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedPlusScte20DestinationSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedPlusScte20DestinationSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedPlusScte20DestinationSettingsEl { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedPlusScte20DestinationSettingsEl { } } }
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedPlusScte20DestinationSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedPlusScte20DestinationSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedPlusScte20DestinationSettingsElRef { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedPlusScte20DestinationSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedPlusScte20DestinationSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElRtmpCaptionInfoDestinationSettingsEl
{}
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElRtmpCaptionInfoDestinationSettingsEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElRtmpCaptionInfoDestinationSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElRtmpCaptionInfoDestinationSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElRtmpCaptionInfoDestinationSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElRtmpCaptionInfoDestinationSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElRtmpCaptionInfoDestinationSettingsEl { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElRtmpCaptionInfoDestinationSettingsEl { } } }
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElRtmpCaptionInfoDestinationSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElRtmpCaptionInfoDestinationSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElRtmpCaptionInfoDestinationSettingsElRef { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElRtmpCaptionInfoDestinationSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElRtmpCaptionInfoDestinationSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte20PlusEmbeddedDestinationSettingsEl
{}
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte20PlusEmbeddedDestinationSettingsEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte20PlusEmbeddedDestinationSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte20PlusEmbeddedDestinationSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte20PlusEmbeddedDestinationSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte20PlusEmbeddedDestinationSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte20PlusEmbeddedDestinationSettingsEl { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte20PlusEmbeddedDestinationSettingsEl { } } }
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte20PlusEmbeddedDestinationSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte20PlusEmbeddedDestinationSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte20PlusEmbeddedDestinationSettingsElRef { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte20PlusEmbeddedDestinationSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte20PlusEmbeddedDestinationSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte27DestinationSettingsEl
{}
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte27DestinationSettingsEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte27DestinationSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte27DestinationSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte27DestinationSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte27DestinationSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte27DestinationSettingsEl { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte27DestinationSettingsEl { } } }
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte27DestinationSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte27DestinationSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte27DestinationSettingsElRef { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte27DestinationSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte27DestinationSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElSmpteTtDestinationSettingsEl
{}
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElSmpteTtDestinationSettingsEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElSmpteTtDestinationSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElSmpteTtDestinationSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElSmpteTtDestinationSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElSmpteTtDestinationSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElSmpteTtDestinationSettingsEl { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElSmpteTtDestinationSettingsEl { } } }
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElSmpteTtDestinationSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElSmpteTtDestinationSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElSmpteTtDestinationSettingsElRef { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElSmpteTtDestinationSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElSmpteTtDestinationSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTeletextDestinationSettingsEl
{}
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTeletextDestinationSettingsEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTeletextDestinationSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTeletextDestinationSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTeletextDestinationSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTeletextDestinationSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTeletextDestinationSettingsEl { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTeletextDestinationSettingsEl { } } }
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTeletextDestinationSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTeletextDestinationSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTeletextDestinationSettingsElRef { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTeletextDestinationSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTeletextDestinationSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTtmlDestinationSettingsEl
{
    style_control: PrimField<String>,
}
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTtmlDestinationSettingsEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTtmlDestinationSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTtmlDestinationSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTtmlDestinationSettingsEl
{
    #[doc = ""]
    pub style_control: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTtmlDestinationSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTtmlDestinationSettingsEl { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTtmlDestinationSettingsEl { style_control : self . style_control , } } }
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTtmlDestinationSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTtmlDestinationSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTtmlDestinationSettingsElRef { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTtmlDestinationSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTtmlDestinationSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `style_control` after provisioning.\n"] pub fn style_control (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.style_control" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElWebvttDestinationSettingsEl
{
    style_control: PrimField<String>,
}
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElWebvttDestinationSettingsEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElWebvttDestinationSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElWebvttDestinationSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElWebvttDestinationSettingsEl
{
    #[doc = ""]
    pub style_control: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElWebvttDestinationSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElWebvttDestinationSettingsEl { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElWebvttDestinationSettingsEl { style_control : self . style_control , } } }
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElWebvttDestinationSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElWebvttDestinationSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElWebvttDestinationSettingsElRef { MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElWebvttDestinationSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElWebvttDestinationSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `style_control` after provisioning.\n"] pub fn style_control (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.style_control" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDynamic { arib_destination_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElAribDestinationSettingsEl >> , burn_in_destination_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsEl >> , dvb_sub_destination_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsEl >> , ebu_tt_d_destination_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEbuTtDDestinationSettingsEl >> , embedded_destination_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedDestinationSettingsEl >> , embedded_plus_scte20_destination_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedPlusScte20DestinationSettingsEl >> , rtmp_caption_info_destination_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElRtmpCaptionInfoDestinationSettingsEl >> , scte20_plus_embedded_destination_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte20PlusEmbeddedDestinationSettingsEl >> , scte27_destination_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte27DestinationSettingsEl >> , smpte_tt_destination_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElSmpteTtDestinationSettingsEl >> , teletext_destination_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTeletextDestinationSettingsEl >> , ttml_destination_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTtmlDestinationSettingsEl >> , webvtt_destination_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElWebvttDestinationSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] arib_destination_settings : Option < Vec < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElAribDestinationSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] burn_in_destination_settings : Option < Vec < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] dvb_sub_destination_settings : Option < Vec < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] ebu_tt_d_destination_settings : Option < Vec < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEbuTtDDestinationSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] embedded_destination_settings : Option < Vec < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedDestinationSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] embedded_plus_scte20_destination_settings : Option < Vec < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedPlusScte20DestinationSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] rtmp_caption_info_destination_settings : Option < Vec < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElRtmpCaptionInfoDestinationSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] scte20_plus_embedded_destination_settings : Option < Vec < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte20PlusEmbeddedDestinationSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] scte27_destination_settings : Option < Vec < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte27DestinationSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] smpte_tt_destination_settings : Option < Vec < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElSmpteTtDestinationSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] teletext_destination_settings : Option < Vec < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTeletextDestinationSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] ttml_destination_settings : Option < Vec < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTtmlDestinationSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] webvtt_destination_settings : Option < Vec < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElWebvttDestinationSettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsEl {
    #[doc = "Set the field `arib_destination_settings`.\n"]
    pub fn set_arib_destination_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElAribDestinationSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.arib_destination_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.arib_destination_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `burn_in_destination_settings`.\n"]
    pub fn set_burn_in_destination_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.burn_in_destination_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.burn_in_destination_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dvb_sub_destination_settings`.\n"]
    pub fn set_dvb_sub_destination_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dvb_sub_destination_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dvb_sub_destination_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ebu_tt_d_destination_settings`.\n"]
    pub fn set_ebu_tt_d_destination_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEbuTtDDestinationSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ebu_tt_d_destination_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ebu_tt_d_destination_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `embedded_destination_settings`.\n"]
    pub fn set_embedded_destination_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedDestinationSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.embedded_destination_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.embedded_destination_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `embedded_plus_scte20_destination_settings`.\n"]
    pub fn set_embedded_plus_scte20_destination_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedPlusScte20DestinationSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.embedded_plus_scte20_destination_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.embedded_plus_scte20_destination_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rtmp_caption_info_destination_settings`.\n"]
    pub fn set_rtmp_caption_info_destination_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElRtmpCaptionInfoDestinationSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rtmp_caption_info_destination_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rtmp_caption_info_destination_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scte20_plus_embedded_destination_settings`.\n"]
    pub fn set_scte20_plus_embedded_destination_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte20PlusEmbeddedDestinationSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.scte20_plus_embedded_destination_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.scte20_plus_embedded_destination_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scte27_destination_settings`.\n"]
    pub fn set_scte27_destination_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte27DestinationSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.scte27_destination_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.scte27_destination_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `smpte_tt_destination_settings`.\n"]
    pub fn set_smpte_tt_destination_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElSmpteTtDestinationSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.smpte_tt_destination_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.smpte_tt_destination_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `teletext_destination_settings`.\n"]
    pub fn set_teletext_destination_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTeletextDestinationSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.teletext_destination_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.teletext_destination_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ttml_destination_settings`.\n"]
    pub fn set_ttml_destination_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTtmlDestinationSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ttml_destination_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ttml_destination_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `webvtt_destination_settings`.\n"]
    pub fn set_webvtt_destination_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElWebvttDestinationSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.webvtt_destination_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.webvtt_destination_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsEl
{
    type O = BlockAssignable<
        MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsEl {}
impl BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsEl {
        MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsEl {
            arib_destination_settings: core::default::Default::default(),
            burn_in_destination_settings: core::default::Default::default(),
            dvb_sub_destination_settings: core::default::Default::default(),
            ebu_tt_d_destination_settings: core::default::Default::default(),
            embedded_destination_settings: core::default::Default::default(),
            embedded_plus_scte20_destination_settings: core::default::Default::default(),
            rtmp_caption_info_destination_settings: core::default::Default::default(),
            scte20_plus_embedded_destination_settings: core::default::Default::default(),
            scte27_destination_settings: core::default::Default::default(),
            smpte_tt_destination_settings: core::default::Default::default(),
            teletext_destination_settings: core::default::Default::default(),
            ttml_destination_settings: core::default::Default::default(),
            webvtt_destination_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElRef {
        MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arib_destination_settings` after provisioning.\n"]    pub fn arib_destination_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElAribDestinationSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.arib_destination_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `burn_in_destination_settings` after provisioning.\n"]    pub fn burn_in_destination_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElBurnInDestinationSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.burn_in_destination_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dvb_sub_destination_settings` after provisioning.\n"]    pub fn dvb_sub_destination_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElDvbSubDestinationSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.dvb_sub_destination_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ebu_tt_d_destination_settings` after provisioning.\n"]    pub fn ebu_tt_d_destination_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEbuTtDDestinationSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.ebu_tt_d_destination_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `embedded_destination_settings` after provisioning.\n"]    pub fn embedded_destination_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedDestinationSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.embedded_destination_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `embedded_plus_scte20_destination_settings` after provisioning.\n"]    pub fn embedded_plus_scte20_destination_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElEmbeddedPlusScte20DestinationSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.embedded_plus_scte20_destination_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rtmp_caption_info_destination_settings` after provisioning.\n"]    pub fn rtmp_caption_info_destination_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElRtmpCaptionInfoDestinationSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.rtmp_caption_info_destination_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scte20_plus_embedded_destination_settings` after provisioning.\n"]    pub fn scte20_plus_embedded_destination_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte20PlusEmbeddedDestinationSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.scte20_plus_embedded_destination_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scte27_destination_settings` after provisioning.\n"]    pub fn scte27_destination_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElScte27DestinationSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.scte27_destination_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `smpte_tt_destination_settings` after provisioning.\n"]    pub fn smpte_tt_destination_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElSmpteTtDestinationSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.smpte_tt_destination_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `teletext_destination_settings` after provisioning.\n"]    pub fn teletext_destination_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTeletextDestinationSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.teletext_destination_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ttml_destination_settings` after provisioning.\n"]    pub fn ttml_destination_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElTtmlDestinationSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.ttml_destination_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `webvtt_destination_settings` after provisioning.\n"]    pub fn webvtt_destination_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElWebvttDestinationSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.webvtt_destination_settings", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElDynamic {
    destination_settings: Option<
        DynamicBlock<MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsEl>,
    >,
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accessibility: Option<PrimField<String>>,
    caption_selector_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_description: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_settings:
        Option<Vec<MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsEl>>,
    dynamic: MedialiveChannelEncoderSettingsElCaptionDescriptionsElDynamic,
}
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsEl {
    #[doc = "Set the field `accessibility`.\n"]
    pub fn set_accessibility(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accessibility = Some(v.into());
        self
    }
    #[doc = "Set the field `language_code`.\n"]
    pub fn set_language_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.language_code = Some(v.into());
        self
    }
    #[doc = "Set the field `language_description`.\n"]
    pub fn set_language_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.language_description = Some(v.into());
        self
    }
    #[doc = "Set the field `destination_settings`.\n"]
    pub fn set_destination_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveChannelEncoderSettingsElCaptionDescriptionsEl {
    type O = BlockAssignable<MedialiveChannelEncoderSettingsElCaptionDescriptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsEl {
    #[doc = ""]
    pub caption_selector_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElCaptionDescriptionsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsEl {
        MedialiveChannelEncoderSettingsElCaptionDescriptionsEl {
            accessibility: core::default::Default::default(),
            caption_selector_name: self.caption_selector_name,
            language_code: core::default::Default::default(),
            language_description: core::default::Default::default(),
            name: self.name,
            destination_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElCaptionDescriptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElCaptionDescriptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElCaptionDescriptionsElRef {
        MedialiveChannelEncoderSettingsElCaptionDescriptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElCaptionDescriptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accessibility` after provisioning.\n"]
    pub fn accessibility(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accessibility", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `caption_selector_name` after provisioning.\n"]
    pub fn caption_selector_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.caption_selector_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `language_description` after provisioning.\n"]
    pub fn language_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_description", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `destination_settings` after provisioning.\n"]
    pub fn destination_settings(
        &self,
    ) -> ListRef<MedialiveChannelEncoderSettingsElCaptionDescriptionsElDestinationSettingsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorElInputLossImageSlateEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    password_param: Option<PrimField<String>>,
    uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
}
impl
    MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorElInputLossImageSlateEl
{
    #[doc = "Set the field `password_param`.\n"]
    pub fn set_password_param(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password_param = Some(v.into());
        self
    }
    #[doc = "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username = Some(v.into());
        self
    }
}
impl ToListMappable for MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorElInputLossImageSlateEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorElInputLossImageSlateEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorElInputLossImageSlateEl
{
    #[doc = ""]
    pub uri: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorElInputLossImageSlateEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorElInputLossImageSlateEl { MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorElInputLossImageSlateEl { password_param : core :: default :: Default :: default () , uri : self . uri , username : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorElInputLossImageSlateElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorElInputLossImageSlateElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorElInputLossImageSlateElRef { MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorElInputLossImageSlateElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorElInputLossImageSlateElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `password_param` after provisioning.\n"] pub fn password_param (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.password_param" , self . base)) } # [doc = "Get a reference to the value of field `uri` after provisioning.\n"] pub fn uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.uri" , self . base)) } # [doc = "Get a reference to the value of field `username` after provisioning.\n"] pub fn username (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.username" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorElDynamic { input_loss_image_slate : Option < DynamicBlock < MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorElInputLossImageSlateEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorEl { # [serde (skip_serializing_if = "Option::is_none")] black_frame_msec : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] input_loss_image_color : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] input_loss_image_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] repeat_frame_msec : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] input_loss_image_slate : Option < Vec < MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorElInputLossImageSlateEl > > , dynamic : MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorElDynamic , }
impl MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorEl {
    #[doc = "Set the field `black_frame_msec`.\n"]
    pub fn set_black_frame_msec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.black_frame_msec = Some(v.into());
        self
    }
    #[doc = "Set the field `input_loss_image_color`.\n"]
    pub fn set_input_loss_image_color(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.input_loss_image_color = Some(v.into());
        self
    }
    #[doc = "Set the field `input_loss_image_type`.\n"]
    pub fn set_input_loss_image_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.input_loss_image_type = Some(v.into());
        self
    }
    #[doc = "Set the field `repeat_frame_msec`.\n"]
    pub fn set_repeat_frame_msec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.repeat_frame_msec = Some(v.into());
        self
    }
    #[doc = "Set the field `input_loss_image_slate`.\n"]
    pub fn set_input_loss_image_slate(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorElInputLossImageSlateEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.input_loss_image_slate = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.input_loss_image_slate = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorEl {
    type O =
        BlockAssignable<MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorEl {}
impl BuildMedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorEl {
        MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorEl {
            black_frame_msec: core::default::Default::default(),
            input_loss_image_color: core::default::Default::default(),
            input_loss_image_type: core::default::Default::default(),
            repeat_frame_msec: core::default::Default::default(),
            input_loss_image_slate: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorElRef {
        MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `black_frame_msec` after provisioning.\n"]
    pub fn black_frame_msec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.black_frame_msec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `input_loss_image_color` after provisioning.\n"]
    pub fn input_loss_image_color(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_loss_image_color", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `input_loss_image_type` after provisioning.\n"]
    pub fn input_loss_image_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_loss_image_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `repeat_frame_msec` after provisioning.\n"]
    pub fn repeat_frame_msec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repeat_frame_msec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `input_loss_image_slate` after provisioning.\n"]    pub fn input_loss_image_slate (& self) -> ListRef < MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorElInputLossImageSlateElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_loss_image_slate", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElGlobalConfigurationElDynamic {
    input_loss_behavior: Option<
        DynamicBlock<MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorEl>,
    >,
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElGlobalConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_audio_gain: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_end_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_locking_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_timing_source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    support_low_framerate_inputs: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_loss_behavior:
        Option<Vec<MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorEl>>,
    dynamic: MedialiveChannelEncoderSettingsElGlobalConfigurationElDynamic,
}
impl MedialiveChannelEncoderSettingsElGlobalConfigurationEl {
    #[doc = "Set the field `initial_audio_gain`.\n"]
    pub fn set_initial_audio_gain(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.initial_audio_gain = Some(v.into());
        self
    }
    #[doc = "Set the field `input_end_action`.\n"]
    pub fn set_input_end_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.input_end_action = Some(v.into());
        self
    }
    #[doc = "Set the field `output_locking_mode`.\n"]
    pub fn set_output_locking_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_locking_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `output_timing_source`.\n"]
    pub fn set_output_timing_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_timing_source = Some(v.into());
        self
    }
    #[doc = "Set the field `support_low_framerate_inputs`.\n"]
    pub fn set_support_low_framerate_inputs(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.support_low_framerate_inputs = Some(v.into());
        self
    }
    #[doc = "Set the field `input_loss_behavior`.\n"]
    pub fn set_input_loss_behavior(
        mut self,
        v: impl Into<
            BlockAssignable<
                MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.input_loss_behavior = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.input_loss_behavior = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveChannelEncoderSettingsElGlobalConfigurationEl {
    type O = BlockAssignable<MedialiveChannelEncoderSettingsElGlobalConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElGlobalConfigurationEl {}
impl BuildMedialiveChannelEncoderSettingsElGlobalConfigurationEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElGlobalConfigurationEl {
        MedialiveChannelEncoderSettingsElGlobalConfigurationEl {
            initial_audio_gain: core::default::Default::default(),
            input_end_action: core::default::Default::default(),
            output_locking_mode: core::default::Default::default(),
            output_timing_source: core::default::Default::default(),
            support_low_framerate_inputs: core::default::Default::default(),
            input_loss_behavior: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElGlobalConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElGlobalConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElGlobalConfigurationElRef {
        MedialiveChannelEncoderSettingsElGlobalConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElGlobalConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `initial_audio_gain` after provisioning.\n"]
    pub fn initial_audio_gain(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_audio_gain", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `input_end_action` after provisioning.\n"]
    pub fn input_end_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_end_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `output_locking_mode` after provisioning.\n"]
    pub fn output_locking_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_locking_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `output_timing_source` after provisioning.\n"]
    pub fn output_timing_source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_timing_source", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `support_low_framerate_inputs` after provisioning.\n"]
    pub fn support_low_framerate_inputs(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.support_low_framerate_inputs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `input_loss_behavior` after provisioning.\n"]
    pub fn input_loss_behavior(
        &self,
    ) -> ListRef<MedialiveChannelEncoderSettingsElGlobalConfigurationElInputLossBehaviorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_loss_behavior", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsElHtmlMotionGraphicsSettingsEl
{}
impl MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsElHtmlMotionGraphicsSettingsEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsElHtmlMotionGraphicsSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsElHtmlMotionGraphicsSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsElHtmlMotionGraphicsSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsElHtmlMotionGraphicsSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsElHtmlMotionGraphicsSettingsEl { MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsElHtmlMotionGraphicsSettingsEl { } } }
pub struct MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsElHtmlMotionGraphicsSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsElHtmlMotionGraphicsSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsElHtmlMotionGraphicsSettingsElRef { MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsElHtmlMotionGraphicsSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsElHtmlMotionGraphicsSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsElDynamic { html_motion_graphics_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsElHtmlMotionGraphicsSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] html_motion_graphics_settings : Option < Vec < MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsElHtmlMotionGraphicsSettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsEl {
    #[doc = "Set the field `html_motion_graphics_settings`.\n"]
    pub fn set_html_motion_graphics_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsElHtmlMotionGraphicsSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.html_motion_graphics_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.html_motion_graphics_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsEl
{
    type O = BlockAssignable<
        MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsEl
    {
        MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsEl {
            html_motion_graphics_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsElRef
    {
        MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `html_motion_graphics_settings` after provisioning.\n"]    pub fn html_motion_graphics_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsElHtmlMotionGraphicsSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.html_motion_graphics_settings", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElDynamic {
    motion_graphics_settings: Option<
        DynamicBlock<
            MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    motion_graphics_insertion: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    motion_graphics_settings: Option<
        Vec<MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsEl>,
    >,
    dynamic: MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElDynamic,
}
impl MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationEl {
    #[doc = "Set the field `motion_graphics_insertion`.\n"]
    pub fn set_motion_graphics_insertion(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.motion_graphics_insertion = Some(v.into());
        self
    }
    #[doc = "Set the field `motion_graphics_settings`.\n"]
    pub fn set_motion_graphics_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.motion_graphics_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.motion_graphics_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationEl {
    type O = BlockAssignable<MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElMotionGraphicsConfigurationEl {}
impl BuildMedialiveChannelEncoderSettingsElMotionGraphicsConfigurationEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationEl {
        MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationEl {
            motion_graphics_insertion: core::default::Default::default(),
            motion_graphics_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElRef {
        MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `motion_graphics_insertion` after provisioning.\n"]
    pub fn motion_graphics_insertion(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.motion_graphics_insertion", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `motion_graphics_settings` after provisioning.\n"]
    pub fn motion_graphics_settings(
        &self,
    ) -> ListRef<
        MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElMotionGraphicsSettingsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.motion_graphics_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElNielsenConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    distributor_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nielsen_pcm_to_id3_tagging: Option<PrimField<String>>,
}
impl MedialiveChannelEncoderSettingsElNielsenConfigurationEl {
    #[doc = "Set the field `distributor_id`.\n"]
    pub fn set_distributor_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.distributor_id = Some(v.into());
        self
    }
    #[doc = "Set the field `nielsen_pcm_to_id3_tagging`.\n"]
    pub fn set_nielsen_pcm_to_id3_tagging(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nielsen_pcm_to_id3_tagging = Some(v.into());
        self
    }
}
impl ToListMappable for MedialiveChannelEncoderSettingsElNielsenConfigurationEl {
    type O = BlockAssignable<MedialiveChannelEncoderSettingsElNielsenConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElNielsenConfigurationEl {}
impl BuildMedialiveChannelEncoderSettingsElNielsenConfigurationEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElNielsenConfigurationEl {
        MedialiveChannelEncoderSettingsElNielsenConfigurationEl {
            distributor_id: core::default::Default::default(),
            nielsen_pcm_to_id3_tagging: core::default::Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElNielsenConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElNielsenConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElNielsenConfigurationElRef {
        MedialiveChannelEncoderSettingsElNielsenConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElNielsenConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `distributor_id` after provisioning.\n"]
    pub fn distributor_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.distributor_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nielsen_pcm_to_id3_tagging` after provisioning.\n"]
    pub fn nielsen_pcm_to_id3_tagging(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nielsen_pcm_to_id3_tagging", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    canned_acl: Option<PrimField<String>>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsEl { # [doc = "Set the field `canned_acl`.\n"] pub fn set_canned_acl (mut self , v : impl Into < PrimField < String > >) -> Self { self . canned_acl = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsEl { canned_acl : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `canned_acl` after provisioning.\n"] pub fn canned_acl (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.canned_acl" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElDynamic { archive_s3_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] archive_s3_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsEl { # [doc = "Set the field `archive_s3_settings`.\n"] pub fn set_archive_s3_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . archive_s3_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . archive_s3_settings = Some (d) ; } } self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsEl { archive_s3_settings : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `archive_s3_settings` after provisioning.\n"] pub fn archive_s3_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.archive_s3_settings" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationEl
{
    destination_ref_id: PrimField<String>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationEl
{
    #[doc = ""]
    pub destination_ref_id: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationEl { destination_ref_id : self . destination_ref_id , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `destination_ref_id` after provisioning.\n"] pub fn destination_ref_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.destination_ref_id" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDynamic { archive_cdn_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsEl >> , destination : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] rollover_interval : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] archive_cdn_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] destination : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationEl > > , dynamic : MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsEl {
    #[doc = "Set the field `rollover_interval`.\n"]
    pub fn set_rollover_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.rollover_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `archive_cdn_settings`.\n"]
    pub fn set_archive_cdn_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.archive_cdn_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.archive_cdn_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsEl
{
    type O = BlockAssignable<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsEl
{}
impl
    BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsEl
{
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsEl
    {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsEl {
            rollover_interval: core::default::Default::default(),
            archive_cdn_settings: core::default::Default::default(),
            destination: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `rollover_interval` after provisioning.\n"]
    pub fn rollover_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rollover_interval", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `archive_cdn_settings` after provisioning.\n"]    pub fn archive_cdn_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.archive_cdn_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]    pub fn destination (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationElRef >{
        ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationEl
{
    destination_ref_id: PrimField<String>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationEl
{
    #[doc = ""]
    pub destination_ref_id: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationEl { destination_ref_id : self . destination_ref_id , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `destination_ref_id` after provisioning.\n"] pub fn destination_ref_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.destination_ref_id" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    canned_acl: Option<PrimField<String>>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsEl { # [doc = "Set the field `canned_acl`.\n"] pub fn set_canned_acl (mut self , v : impl Into < PrimField < String > >) -> Self { self . canned_acl = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsEl { canned_acl : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `canned_acl` after provisioning.\n"] pub fn canned_acl (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.canned_acl" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElDynamic { frame_capture_s3_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] frame_capture_s3_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsEl { # [doc = "Set the field `frame_capture_s3_settings`.\n"] pub fn set_frame_capture_s3_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . frame_capture_s3_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . frame_capture_s3_settings = Some (d) ; } } self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsEl { frame_capture_s3_settings : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `frame_capture_s3_settings` after provisioning.\n"] pub fn frame_capture_s3_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.frame_capture_s3_settings" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDynamic { destination : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationEl >> , frame_capture_cdn_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] destination : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationEl > > , # [serde (skip_serializing_if = "Option::is_none")] frame_capture_cdn_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDynamic , }
impl
    MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsEl
{
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `frame_capture_cdn_settings`.\n"]
    pub fn set_frame_capture_cdn_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.frame_capture_cdn_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.frame_capture_cdn_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsEl { destination : core :: default :: Default :: default () , frame_capture_cdn_settings : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `destination` after provisioning.\n"] pub fn destination (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.destination" , self . base)) } # [doc = "Get a reference to the value of field `frame_capture_cdn_settings` after provisioning.\n"] pub fn frame_capture_cdn_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.frame_capture_cdn_settings" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsEl
{
    caption_channel: PrimField<f64>,
    language_code: PrimField<String>,
    language_description: PrimField<String>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsEl
{
    #[doc = ""]
    pub caption_channel: PrimField<f64>,
    #[doc = ""]
    pub language_code: PrimField<String>,
    #[doc = ""]
    pub language_description: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsEl { caption_channel : self . caption_channel , language_code : self . language_code , language_description : self . language_description , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `caption_channel` after provisioning.\n"] pub fn caption_channel (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.caption_channel" , self . base)) } # [doc = "Get a reference to the value of field `language_code` after provisioning.\n"] pub fn language_code (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.language_code" , self . base)) } # [doc = "Get a reference to the value of field `language_description` after provisioning.\n"] pub fn language_description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.language_description" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationEl
{
    destination_ref_id: PrimField<String>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationEl
{
    #[doc = ""]
    pub destination_ref_id: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationEl { destination_ref_id : self . destination_ref_id , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `destination_ref_id` after provisioning.\n"] pub fn destination_ref_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.destination_ref_id" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_retry_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filecache_duration: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_transfer_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restart_delay: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    salt: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<PrimField<String>>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsEl { # [doc = "Set the field `connection_retry_interval`.\n"] pub fn set_connection_retry_interval (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . connection_retry_interval = Some (v . into ()) ; self } # [doc = "Set the field `filecache_duration`.\n"] pub fn set_filecache_duration (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . filecache_duration = Some (v . into ()) ; self } # [doc = "Set the field `http_transfer_mode`.\n"] pub fn set_http_transfer_mode (mut self , v : impl Into < PrimField < String > >) -> Self { self . http_transfer_mode = Some (v . into ()) ; self } # [doc = "Set the field `num_retries`.\n"] pub fn set_num_retries (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . num_retries = Some (v . into ()) ; self } # [doc = "Set the field `restart_delay`.\n"] pub fn set_restart_delay (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . restart_delay = Some (v . into ()) ; self } # [doc = "Set the field `salt`.\n"] pub fn set_salt (mut self , v : impl Into < PrimField < String > >) -> Self { self . salt = Some (v . into ()) ; self } # [doc = "Set the field `token`.\n"] pub fn set_token (mut self , v : impl Into < PrimField < String > >) -> Self { self . token = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsEl { connection_retry_interval : core :: default :: Default :: default () , filecache_duration : core :: default :: Default :: default () , http_transfer_mode : core :: default :: Default :: default () , num_retries : core :: default :: Default :: default () , restart_delay : core :: default :: Default :: default () , salt : core :: default :: Default :: default () , token : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `connection_retry_interval` after provisioning.\n"] pub fn connection_retry_interval (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.connection_retry_interval" , self . base)) } # [doc = "Get a reference to the value of field `filecache_duration` after provisioning.\n"] pub fn filecache_duration (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.filecache_duration" , self . base)) } # [doc = "Get a reference to the value of field `http_transfer_mode` after provisioning.\n"] pub fn http_transfer_mode (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.http_transfer_mode" , self . base)) } # [doc = "Get a reference to the value of field `num_retries` after provisioning.\n"] pub fn num_retries (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.num_retries" , self . base)) } # [doc = "Get a reference to the value of field `restart_delay` after provisioning.\n"] pub fn restart_delay (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.restart_delay" , self . base)) } # [doc = "Get a reference to the value of field `salt` after provisioning.\n"] pub fn salt (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.salt" , self . base)) } # [doc = "Get a reference to the value of field `token` after provisioning.\n"] pub fn token (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.token" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_retry_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filecache_duration: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restart_delay: Option<PrimField<f64>>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsEl { # [doc = "Set the field `connection_retry_interval`.\n"] pub fn set_connection_retry_interval (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . connection_retry_interval = Some (v . into ()) ; self } # [doc = "Set the field `filecache_duration`.\n"] pub fn set_filecache_duration (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . filecache_duration = Some (v . into ()) ; self } # [doc = "Set the field `num_retries`.\n"] pub fn set_num_retries (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . num_retries = Some (v . into ()) ; self } # [doc = "Set the field `restart_delay`.\n"] pub fn set_restart_delay (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . restart_delay = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsEl { connection_retry_interval : core :: default :: Default :: default () , filecache_duration : core :: default :: Default :: default () , num_retries : core :: default :: Default :: default () , restart_delay : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `connection_retry_interval` after provisioning.\n"] pub fn connection_retry_interval (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.connection_retry_interval" , self . base)) } # [doc = "Get a reference to the value of field `filecache_duration` after provisioning.\n"] pub fn filecache_duration (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.filecache_duration" , self . base)) } # [doc = "Get a reference to the value of field `num_retries` after provisioning.\n"] pub fn num_retries (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.num_retries" , self . base)) } # [doc = "Get a reference to the value of field `restart_delay` after provisioning.\n"] pub fn restart_delay (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.restart_delay" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_retry_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filecache_duration: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    media_store_storage_class: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restart_delay: Option<PrimField<f64>>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsEl { # [doc = "Set the field `connection_retry_interval`.\n"] pub fn set_connection_retry_interval (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . connection_retry_interval = Some (v . into ()) ; self } # [doc = "Set the field `filecache_duration`.\n"] pub fn set_filecache_duration (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . filecache_duration = Some (v . into ()) ; self } # [doc = "Set the field `media_store_storage_class`.\n"] pub fn set_media_store_storage_class (mut self , v : impl Into < PrimField < String > >) -> Self { self . media_store_storage_class = Some (v . into ()) ; self } # [doc = "Set the field `num_retries`.\n"] pub fn set_num_retries (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . num_retries = Some (v . into ()) ; self } # [doc = "Set the field `restart_delay`.\n"] pub fn set_restart_delay (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . restart_delay = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsEl { connection_retry_interval : core :: default :: Default :: default () , filecache_duration : core :: default :: Default :: default () , media_store_storage_class : core :: default :: Default :: default () , num_retries : core :: default :: Default :: default () , restart_delay : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `connection_retry_interval` after provisioning.\n"] pub fn connection_retry_interval (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.connection_retry_interval" , self . base)) } # [doc = "Get a reference to the value of field `filecache_duration` after provisioning.\n"] pub fn filecache_duration (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.filecache_duration" , self . base)) } # [doc = "Get a reference to the value of field `media_store_storage_class` after provisioning.\n"] pub fn media_store_storage_class (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.media_store_storage_class" , self . base)) } # [doc = "Get a reference to the value of field `num_retries` after provisioning.\n"] pub fn num_retries (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.num_retries" , self . base)) } # [doc = "Get a reference to the value of field `restart_delay` after provisioning.\n"] pub fn restart_delay (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.restart_delay" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    canned_acl: Option<PrimField<String>>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsEl { # [doc = "Set the field `canned_acl`.\n"] pub fn set_canned_acl (mut self , v : impl Into < PrimField < String > >) -> Self { self . canned_acl = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsEl { canned_acl : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `canned_acl` after provisioning.\n"] pub fn canned_acl (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.canned_acl" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_retry_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filecache_duration: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_transfer_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restart_delay: Option<PrimField<f64>>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsEl { # [doc = "Set the field `connection_retry_interval`.\n"] pub fn set_connection_retry_interval (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . connection_retry_interval = Some (v . into ()) ; self } # [doc = "Set the field `filecache_duration`.\n"] pub fn set_filecache_duration (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . filecache_duration = Some (v . into ()) ; self } # [doc = "Set the field `http_transfer_mode`.\n"] pub fn set_http_transfer_mode (mut self , v : impl Into < PrimField < String > >) -> Self { self . http_transfer_mode = Some (v . into ()) ; self } # [doc = "Set the field `num_retries`.\n"] pub fn set_num_retries (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . num_retries = Some (v . into ()) ; self } # [doc = "Set the field `restart_delay`.\n"] pub fn set_restart_delay (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . restart_delay = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsEl { connection_retry_interval : core :: default :: Default :: default () , filecache_duration : core :: default :: Default :: default () , http_transfer_mode : core :: default :: Default :: default () , num_retries : core :: default :: Default :: default () , restart_delay : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `connection_retry_interval` after provisioning.\n"] pub fn connection_retry_interval (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.connection_retry_interval" , self . base)) } # [doc = "Get a reference to the value of field `filecache_duration` after provisioning.\n"] pub fn filecache_duration (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.filecache_duration" , self . base)) } # [doc = "Get a reference to the value of field `http_transfer_mode` after provisioning.\n"] pub fn http_transfer_mode (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.http_transfer_mode" , self . base)) } # [doc = "Get a reference to the value of field `num_retries` after provisioning.\n"] pub fn num_retries (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.num_retries" , self . base)) } # [doc = "Get a reference to the value of field `restart_delay` after provisioning.\n"] pub fn restart_delay (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.restart_delay" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElDynamic { hls_akamai_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsEl >> , hls_basic_put_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsEl >> , hls_media_store_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsEl >> , hls_s3_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsEl >> , hls_webdav_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] hls_akamai_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] hls_basic_put_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] hls_media_store_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] hls_s3_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] hls_webdav_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsEl { # [doc = "Set the field `hls_akamai_settings`.\n"] pub fn set_hls_akamai_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . hls_akamai_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . hls_akamai_settings = Some (d) ; } } self } # [doc = "Set the field `hls_basic_put_settings`.\n"] pub fn set_hls_basic_put_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . hls_basic_put_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . hls_basic_put_settings = Some (d) ; } } self } # [doc = "Set the field `hls_media_store_settings`.\n"] pub fn set_hls_media_store_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . hls_media_store_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . hls_media_store_settings = Some (d) ; } } self } # [doc = "Set the field `hls_s3_settings`.\n"] pub fn set_hls_s3_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . hls_s3_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . hls_s3_settings = Some (d) ; } } self } # [doc = "Set the field `hls_webdav_settings`.\n"] pub fn set_hls_webdav_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . hls_webdav_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . hls_webdav_settings = Some (d) ; } } self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsEl { hls_akamai_settings : core :: default :: Default :: default () , hls_basic_put_settings : core :: default :: Default :: default () , hls_media_store_settings : core :: default :: Default :: default () , hls_s3_settings : core :: default :: Default :: default () , hls_webdav_settings : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `hls_akamai_settings` after provisioning.\n"] pub fn hls_akamai_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.hls_akamai_settings" , self . base)) } # [doc = "Get a reference to the value of field `hls_basic_put_settings` after provisioning.\n"] pub fn hls_basic_put_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.hls_basic_put_settings" , self . base)) } # [doc = "Get a reference to the value of field `hls_media_store_settings` after provisioning.\n"] pub fn hls_media_store_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.hls_media_store_settings" , self . base)) } # [doc = "Get a reference to the value of field `hls_s3_settings` after provisioning.\n"] pub fn hls_s3_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.hls_s3_settings" , self . base)) } # [doc = "Get a reference to the value of field `hls_webdav_settings` after provisioning.\n"] pub fn hls_webdav_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.hls_webdav_settings" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    password_param: Option<PrimField<String>>,
    uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerEl { # [doc = "Set the field `password_param`.\n"] pub fn set_password_param (mut self , v : impl Into < PrimField < String > >) -> Self { self . password_param = Some (v . into ()) ; self } # [doc = "Set the field `username`.\n"] pub fn set_username (mut self , v : impl Into < PrimField < String > >) -> Self { self . username = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerEl
{
    #[doc = ""]
    pub uri: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerEl { password_param : core :: default :: Default :: default () , uri : self . uri , username : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `password_param` after provisioning.\n"] pub fn password_param (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.password_param" , self . base)) } # [doc = "Get a reference to the value of field `uri` after provisioning.\n"] pub fn uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.uri" , self . base)) } # [doc = "Get a reference to the value of field `username` after provisioning.\n"] pub fn username (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.username" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElDynamic { key_provider_server : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsEl { static_key_value : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] key_provider_server : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerEl > > , dynamic : MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsEl { # [doc = "Set the field `key_provider_server`.\n"] pub fn set_key_provider_server (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . key_provider_server = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . key_provider_server = Some (d) ; } } self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsEl
{
    #[doc = ""]
    pub static_key_value: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsEl { static_key_value : self . static_key_value , key_provider_server : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `static_key_value` after provisioning.\n"] pub fn static_key_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.static_key_value" , self . base)) } # [doc = "Get a reference to the value of field `key_provider_server` after provisioning.\n"] pub fn key_provider_server (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.key_provider_server" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElDynamic { static_key_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] static_key_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsEl { # [doc = "Set the field `static_key_settings`.\n"] pub fn set_static_key_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . static_key_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . static_key_settings = Some (d) ; } } self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsEl { static_key_settings : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `static_key_settings` after provisioning.\n"] pub fn static_key_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.static_key_settings" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDynamic { caption_language_mappings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsEl >> , destination : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationEl >> , hls_cdn_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsEl >> , key_provider_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] ad_markers : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] base_url_content : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] base_url_content1 : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] base_url_manifest : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] base_url_manifest1 : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] caption_language_setting : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] client_cache : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] codec_specification : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] constant_iv : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] directory_structure : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] discontinuity_tags : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] encryption_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] hls_id3_segment_tagging : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] iframe_only_playlists : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] incomplete_segment_behavior : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] index_n_segments : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] input_loss_action : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] iv_in_manifest : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] iv_source : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] keep_segments : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] key_format : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] key_format_versions : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] manifest_compression : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] manifest_duration_format : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] min_segment_length : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] mode : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] output_selection : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] program_date_time : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] program_date_time_clock : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] program_date_time_period : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] redundant_manifest : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] segment_length : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] segments_per_subdirectory : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] stream_inf_resolution : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] timed_metadata_id3_frame : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] timed_metadata_id3_period : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] timestamp_delta_milliseconds : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] ts_file_mode : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] caption_language_mappings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] destination : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationEl > > , # [serde (skip_serializing_if = "Option::is_none")] hls_cdn_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] key_provider_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsEl {
    #[doc = "Set the field `ad_markers`.\n"]
    pub fn set_ad_markers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ad_markers = Some(v.into());
        self
    }
    #[doc = "Set the field `base_url_content`.\n"]
    pub fn set_base_url_content(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.base_url_content = Some(v.into());
        self
    }
    #[doc = "Set the field `base_url_content1`.\n"]
    pub fn set_base_url_content1(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.base_url_content1 = Some(v.into());
        self
    }
    #[doc = "Set the field `base_url_manifest`.\n"]
    pub fn set_base_url_manifest(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.base_url_manifest = Some(v.into());
        self
    }
    #[doc = "Set the field `base_url_manifest1`.\n"]
    pub fn set_base_url_manifest1(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.base_url_manifest1 = Some(v.into());
        self
    }
    #[doc = "Set the field `caption_language_setting`.\n"]
    pub fn set_caption_language_setting(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.caption_language_setting = Some(v.into());
        self
    }
    #[doc = "Set the field `client_cache`.\n"]
    pub fn set_client_cache(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_cache = Some(v.into());
        self
    }
    #[doc = "Set the field `codec_specification`.\n"]
    pub fn set_codec_specification(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.codec_specification = Some(v.into());
        self
    }
    #[doc = "Set the field `constant_iv`.\n"]
    pub fn set_constant_iv(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.constant_iv = Some(v.into());
        self
    }
    #[doc = "Set the field `directory_structure`.\n"]
    pub fn set_directory_structure(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.directory_structure = Some(v.into());
        self
    }
    #[doc = "Set the field `discontinuity_tags`.\n"]
    pub fn set_discontinuity_tags(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.discontinuity_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_type`.\n"]
    pub fn set_encryption_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_type = Some(v.into());
        self
    }
    #[doc = "Set the field `hls_id3_segment_tagging`.\n"]
    pub fn set_hls_id3_segment_tagging(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hls_id3_segment_tagging = Some(v.into());
        self
    }
    #[doc = "Set the field `iframe_only_playlists`.\n"]
    pub fn set_iframe_only_playlists(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.iframe_only_playlists = Some(v.into());
        self
    }
    #[doc = "Set the field `incomplete_segment_behavior`.\n"]
    pub fn set_incomplete_segment_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.incomplete_segment_behavior = Some(v.into());
        self
    }
    #[doc = "Set the field `index_n_segments`.\n"]
    pub fn set_index_n_segments(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.index_n_segments = Some(v.into());
        self
    }
    #[doc = "Set the field `input_loss_action`.\n"]
    pub fn set_input_loss_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.input_loss_action = Some(v.into());
        self
    }
    #[doc = "Set the field `iv_in_manifest`.\n"]
    pub fn set_iv_in_manifest(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.iv_in_manifest = Some(v.into());
        self
    }
    #[doc = "Set the field `iv_source`.\n"]
    pub fn set_iv_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.iv_source = Some(v.into());
        self
    }
    #[doc = "Set the field `keep_segments`.\n"]
    pub fn set_keep_segments(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.keep_segments = Some(v.into());
        self
    }
    #[doc = "Set the field `key_format`.\n"]
    pub fn set_key_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_format = Some(v.into());
        self
    }
    #[doc = "Set the field `key_format_versions`.\n"]
    pub fn set_key_format_versions(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_format_versions = Some(v.into());
        self
    }
    #[doc = "Set the field `manifest_compression`.\n"]
    pub fn set_manifest_compression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.manifest_compression = Some(v.into());
        self
    }
    #[doc = "Set the field `manifest_duration_format`.\n"]
    pub fn set_manifest_duration_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.manifest_duration_format = Some(v.into());
        self
    }
    #[doc = "Set the field `min_segment_length`.\n"]
    pub fn set_min_segment_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_segment_length = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `output_selection`.\n"]
    pub fn set_output_selection(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_selection = Some(v.into());
        self
    }
    #[doc = "Set the field `program_date_time`.\n"]
    pub fn set_program_date_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.program_date_time = Some(v.into());
        self
    }
    #[doc = "Set the field `program_date_time_clock`.\n"]
    pub fn set_program_date_time_clock(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.program_date_time_clock = Some(v.into());
        self
    }
    #[doc = "Set the field `program_date_time_period`.\n"]
    pub fn set_program_date_time_period(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.program_date_time_period = Some(v.into());
        self
    }
    #[doc = "Set the field `redundant_manifest`.\n"]
    pub fn set_redundant_manifest(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.redundant_manifest = Some(v.into());
        self
    }
    #[doc = "Set the field `segment_length`.\n"]
    pub fn set_segment_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.segment_length = Some(v.into());
        self
    }
    #[doc = "Set the field `segments_per_subdirectory`.\n"]
    pub fn set_segments_per_subdirectory(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.segments_per_subdirectory = Some(v.into());
        self
    }
    #[doc = "Set the field `stream_inf_resolution`.\n"]
    pub fn set_stream_inf_resolution(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stream_inf_resolution = Some(v.into());
        self
    }
    #[doc = "Set the field `timed_metadata_id3_frame`.\n"]
    pub fn set_timed_metadata_id3_frame(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timed_metadata_id3_frame = Some(v.into());
        self
    }
    #[doc = "Set the field `timed_metadata_id3_period`.\n"]
    pub fn set_timed_metadata_id3_period(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timed_metadata_id3_period = Some(v.into());
        self
    }
    #[doc = "Set the field `timestamp_delta_milliseconds`.\n"]
    pub fn set_timestamp_delta_milliseconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timestamp_delta_milliseconds = Some(v.into());
        self
    }
    #[doc = "Set the field `ts_file_mode`.\n"]
    pub fn set_ts_file_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ts_file_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `caption_language_mappings`.\n"]
    pub fn set_caption_language_mappings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.caption_language_mappings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.caption_language_mappings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `hls_cdn_settings`.\n"]
    pub fn set_hls_cdn_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hls_cdn_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hls_cdn_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `key_provider_settings`.\n"]
    pub fn set_key_provider_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.key_provider_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.key_provider_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsEl
{
    type O = BlockAssignable<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsEl
    {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsEl {
            ad_markers: core::default::Default::default(),
            base_url_content: core::default::Default::default(),
            base_url_content1: core::default::Default::default(),
            base_url_manifest: core::default::Default::default(),
            base_url_manifest1: core::default::Default::default(),
            caption_language_setting: core::default::Default::default(),
            client_cache: core::default::Default::default(),
            codec_specification: core::default::Default::default(),
            constant_iv: core::default::Default::default(),
            directory_structure: core::default::Default::default(),
            discontinuity_tags: core::default::Default::default(),
            encryption_type: core::default::Default::default(),
            hls_id3_segment_tagging: core::default::Default::default(),
            iframe_only_playlists: core::default::Default::default(),
            incomplete_segment_behavior: core::default::Default::default(),
            index_n_segments: core::default::Default::default(),
            input_loss_action: core::default::Default::default(),
            iv_in_manifest: core::default::Default::default(),
            iv_source: core::default::Default::default(),
            keep_segments: core::default::Default::default(),
            key_format: core::default::Default::default(),
            key_format_versions: core::default::Default::default(),
            manifest_compression: core::default::Default::default(),
            manifest_duration_format: core::default::Default::default(),
            min_segment_length: core::default::Default::default(),
            mode: core::default::Default::default(),
            output_selection: core::default::Default::default(),
            program_date_time: core::default::Default::default(),
            program_date_time_clock: core::default::Default::default(),
            program_date_time_period: core::default::Default::default(),
            redundant_manifest: core::default::Default::default(),
            segment_length: core::default::Default::default(),
            segments_per_subdirectory: core::default::Default::default(),
            stream_inf_resolution: core::default::Default::default(),
            timed_metadata_id3_frame: core::default::Default::default(),
            timed_metadata_id3_period: core::default::Default::default(),
            timestamp_delta_milliseconds: core::default::Default::default(),
            ts_file_mode: core::default::Default::default(),
            caption_language_mappings: core::default::Default::default(),
            destination: core::default::Default::default(),
            hls_cdn_settings: core::default::Default::default(),
            key_provider_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElRef
    {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ad_markers` after provisioning.\n"]
    pub fn ad_markers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ad_markers", self.base))
    }
    #[doc = "Get a reference to the value of field `base_url_content` after provisioning.\n"]
    pub fn base_url_content(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_url_content", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `base_url_content1` after provisioning.\n"]
    pub fn base_url_content1(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_url_content1", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `base_url_manifest` after provisioning.\n"]
    pub fn base_url_manifest(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_url_manifest", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `base_url_manifest1` after provisioning.\n"]
    pub fn base_url_manifest1(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_url_manifest1", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `caption_language_setting` after provisioning.\n"]
    pub fn caption_language_setting(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.caption_language_setting", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_cache` after provisioning.\n"]
    pub fn client_cache(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_cache", self.base))
    }
    #[doc = "Get a reference to the value of field `codec_specification` after provisioning.\n"]
    pub fn codec_specification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.codec_specification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `constant_iv` after provisioning.\n"]
    pub fn constant_iv(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.constant_iv", self.base))
    }
    #[doc = "Get a reference to the value of field `directory_structure` after provisioning.\n"]
    pub fn directory_structure(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.directory_structure", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `discontinuity_tags` after provisioning.\n"]
    pub fn discontinuity_tags(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.discontinuity_tags", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_type` after provisioning.\n"]
    pub fn encryption_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hls_id3_segment_tagging` after provisioning.\n"]
    pub fn hls_id3_segment_tagging(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hls_id3_segment_tagging", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `iframe_only_playlists` after provisioning.\n"]
    pub fn iframe_only_playlists(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iframe_only_playlists", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `incomplete_segment_behavior` after provisioning.\n"]
    pub fn incomplete_segment_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.incomplete_segment_behavior", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `index_n_segments` after provisioning.\n"]
    pub fn index_n_segments(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.index_n_segments", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `input_loss_action` after provisioning.\n"]
    pub fn input_loss_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_loss_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `iv_in_manifest` after provisioning.\n"]
    pub fn iv_in_manifest(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iv_in_manifest", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `iv_source` after provisioning.\n"]
    pub fn iv_source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iv_source", self.base))
    }
    #[doc = "Get a reference to the value of field `keep_segments` after provisioning.\n"]
    pub fn keep_segments(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.keep_segments", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key_format` after provisioning.\n"]
    pub fn key_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_format", self.base))
    }
    #[doc = "Get a reference to the value of field `key_format_versions` after provisioning.\n"]
    pub fn key_format_versions(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_format_versions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `manifest_compression` after provisioning.\n"]
    pub fn manifest_compression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.manifest_compression", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `manifest_duration_format` after provisioning.\n"]
    pub fn manifest_duration_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.manifest_duration_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_segment_length` after provisioning.\n"]
    pub fn min_segment_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_segment_length", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `output_selection` after provisioning.\n"]
    pub fn output_selection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_selection", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `program_date_time` after provisioning.\n"]
    pub fn program_date_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.program_date_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `program_date_time_clock` after provisioning.\n"]
    pub fn program_date_time_clock(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.program_date_time_clock", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `program_date_time_period` after provisioning.\n"]
    pub fn program_date_time_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.program_date_time_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `redundant_manifest` after provisioning.\n"]
    pub fn redundant_manifest(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redundant_manifest", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `segment_length` after provisioning.\n"]
    pub fn segment_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.segment_length", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `segments_per_subdirectory` after provisioning.\n"]
    pub fn segments_per_subdirectory(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.segments_per_subdirectory", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `stream_inf_resolution` after provisioning.\n"]
    pub fn stream_inf_resolution(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stream_inf_resolution", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timed_metadata_id3_frame` after provisioning.\n"]
    pub fn timed_metadata_id3_frame(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timed_metadata_id3_frame", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timed_metadata_id3_period` after provisioning.\n"]
    pub fn timed_metadata_id3_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timed_metadata_id3_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timestamp_delta_milliseconds` after provisioning.\n"]
    pub fn timestamp_delta_milliseconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timestamp_delta_milliseconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ts_file_mode` after provisioning.\n"]
    pub fn ts_file_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ts_file_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]    pub fn destination (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationElRef >{
        ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
    #[doc = "Get a reference to the value of field `hls_cdn_settings` after provisioning.\n"]    pub fn hls_cdn_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.hls_cdn_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key_provider_settings` after provisioning.\n"]    pub fn key_provider_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.key_provider_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationEl
{
    destination_ref_id: PrimField<String>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationEl
{
    #[doc = ""]
    pub destination_ref_id: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationEl { destination_ref_id : self . destination_ref_id , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `destination_ref_id` after provisioning.\n"] pub fn destination_ref_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.destination_ref_id" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDynamic { destination : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] destination : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationEl > > , dynamic : MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDynamic , }
impl
    MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsEl
{
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsEl { destination : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `destination` after provisioning.\n"] pub fn destination (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.destination" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationEl
{
    destination_ref_id: PrimField<String>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationEl
{
    #[doc = ""]
    pub destination_ref_id: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationEl { destination_ref_id : self . destination_ref_id , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `destination_ref_id` after provisioning.\n"] pub fn destination_ref_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.destination_ref_id" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDynamic { destination : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] acquisition_point_id : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] audio_only_timecode_control : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] certificate_mode : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] connection_retry_interval : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] event_id : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] event_id_mode : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] event_stop_behavior : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] filecache_duration : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] fragment_length : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] input_loss_action : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] num_retries : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] restart_delay : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] segmentation_mode : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] send_delay_ms : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] sparse_track_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] stream_manifest_behavior : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] timestamp_offset : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] timestamp_offset_mode : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] destination : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationEl > > , dynamic : MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsEl {
    #[doc = "Set the field `acquisition_point_id`.\n"]
    pub fn set_acquisition_point_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.acquisition_point_id = Some(v.into());
        self
    }
    #[doc = "Set the field `audio_only_timecode_control`.\n"]
    pub fn set_audio_only_timecode_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.audio_only_timecode_control = Some(v.into());
        self
    }
    #[doc = "Set the field `certificate_mode`.\n"]
    pub fn set_certificate_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `connection_retry_interval`.\n"]
    pub fn set_connection_retry_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.connection_retry_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `event_id`.\n"]
    pub fn set_event_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.event_id = Some(v.into());
        self
    }
    #[doc = "Set the field `event_id_mode`.\n"]
    pub fn set_event_id_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.event_id_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `event_stop_behavior`.\n"]
    pub fn set_event_stop_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.event_stop_behavior = Some(v.into());
        self
    }
    #[doc = "Set the field `filecache_duration`.\n"]
    pub fn set_filecache_duration(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.filecache_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `fragment_length`.\n"]
    pub fn set_fragment_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.fragment_length = Some(v.into());
        self
    }
    #[doc = "Set the field `input_loss_action`.\n"]
    pub fn set_input_loss_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.input_loss_action = Some(v.into());
        self
    }
    #[doc = "Set the field `num_retries`.\n"]
    pub fn set_num_retries(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_retries = Some(v.into());
        self
    }
    #[doc = "Set the field `restart_delay`.\n"]
    pub fn set_restart_delay(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.restart_delay = Some(v.into());
        self
    }
    #[doc = "Set the field `segmentation_mode`.\n"]
    pub fn set_segmentation_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.segmentation_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `send_delay_ms`.\n"]
    pub fn set_send_delay_ms(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.send_delay_ms = Some(v.into());
        self
    }
    #[doc = "Set the field `sparse_track_type`.\n"]
    pub fn set_sparse_track_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sparse_track_type = Some(v.into());
        self
    }
    #[doc = "Set the field `stream_manifest_behavior`.\n"]
    pub fn set_stream_manifest_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stream_manifest_behavior = Some(v.into());
        self
    }
    #[doc = "Set the field `timestamp_offset`.\n"]
    pub fn set_timestamp_offset(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timestamp_offset = Some(v.into());
        self
    }
    #[doc = "Set the field `timestamp_offset_mode`.\n"]
    pub fn set_timestamp_offset_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timestamp_offset_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsEl
{
    type O = BlockAssignable<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsEl
{}
impl
    BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsEl
{
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsEl
    {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsEl { acquisition_point_id : core :: default :: Default :: default () , audio_only_timecode_control : core :: default :: Default :: default () , certificate_mode : core :: default :: Default :: default () , connection_retry_interval : core :: default :: Default :: default () , event_id : core :: default :: Default :: default () , event_id_mode : core :: default :: Default :: default () , event_stop_behavior : core :: default :: Default :: default () , filecache_duration : core :: default :: Default :: default () , fragment_length : core :: default :: Default :: default () , input_loss_action : core :: default :: Default :: default () , num_retries : core :: default :: Default :: default () , restart_delay : core :: default :: Default :: default () , segmentation_mode : core :: default :: Default :: default () , send_delay_ms : core :: default :: Default :: default () , sparse_track_type : core :: default :: Default :: default () , stream_manifest_behavior : core :: default :: Default :: default () , timestamp_offset : core :: default :: Default :: default () , timestamp_offset_mode : core :: default :: Default :: default () , destination : core :: default :: Default :: default () , dynamic : Default :: default () , }
    }
}
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElRef { shared : shared , base : base . to_string () , } } }
impl
    MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `acquisition_point_id` after provisioning.\n"]
    pub fn acquisition_point_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.acquisition_point_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `audio_only_timecode_control` after provisioning.\n"]
    pub fn audio_only_timecode_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.audio_only_timecode_control", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `certificate_mode` after provisioning.\n"]
    pub fn certificate_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `connection_retry_interval` after provisioning.\n"]
    pub fn connection_retry_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_retry_interval", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `event_id` after provisioning.\n"]
    pub fn event_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_id", self.base))
    }
    #[doc = "Get a reference to the value of field `event_id_mode` after provisioning.\n"]
    pub fn event_id_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_id_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `event_stop_behavior` after provisioning.\n"]
    pub fn event_stop_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_stop_behavior", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `filecache_duration` after provisioning.\n"]
    pub fn filecache_duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filecache_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fragment_length` after provisioning.\n"]
    pub fn fragment_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fragment_length", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `input_loss_action` after provisioning.\n"]
    pub fn input_loss_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_loss_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `num_retries` after provisioning.\n"]
    pub fn num_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_retries", self.base))
    }
    #[doc = "Get a reference to the value of field `restart_delay` after provisioning.\n"]
    pub fn restart_delay(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.restart_delay", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `segmentation_mode` after provisioning.\n"]
    pub fn segmentation_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.segmentation_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `send_delay_ms` after provisioning.\n"]
    pub fn send_delay_ms(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.send_delay_ms", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sparse_track_type` after provisioning.\n"]
    pub fn sparse_track_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sparse_track_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `stream_manifest_behavior` after provisioning.\n"]
    pub fn stream_manifest_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stream_manifest_behavior", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timestamp_offset` after provisioning.\n"]
    pub fn timestamp_offset(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timestamp_offset", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timestamp_offset_mode` after provisioning.\n"]
    pub fn timestamp_offset_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timestamp_offset_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]    pub fn destination (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationElRef >{
        ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsEl
{}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsEl {}
impl ToListMappable
    for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsEl
{
    type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsEl { } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsElRef { shared : shared , base : base . to_string () , } } }
impl
    MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ad_markers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_scheme: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_full_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_data: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_loss_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restart_delay: Option<PrimField<f64>>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsEl {
    #[doc = "Set the field `ad_markers`.\n"]
    pub fn set_ad_markers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ad_markers = Some(v.into());
        self
    }
    #[doc = "Set the field `authentication_scheme`.\n"]
    pub fn set_authentication_scheme(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authentication_scheme = Some(v.into());
        self
    }
    #[doc = "Set the field `cache_full_behavior`.\n"]
    pub fn set_cache_full_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cache_full_behavior = Some(v.into());
        self
    }
    #[doc = "Set the field `cache_length`.\n"]
    pub fn set_cache_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cache_length = Some(v.into());
        self
    }
    #[doc = "Set the field `caption_data`.\n"]
    pub fn set_caption_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.caption_data = Some(v.into());
        self
    }
    #[doc = "Set the field `input_loss_action`.\n"]
    pub fn set_input_loss_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.input_loss_action = Some(v.into());
        self
    }
    #[doc = "Set the field `restart_delay`.\n"]
    pub fn set_restart_delay(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.restart_delay = Some(v.into());
        self
    }
}
impl ToListMappable
    for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsEl
{
    type O = BlockAssignable<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsEl
    {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsEl {
            ad_markers: core::default::Default::default(),
            authentication_scheme: core::default::Default::default(),
            cache_full_behavior: core::default::Default::default(),
            cache_length: core::default::Default::default(),
            caption_data: core::default::Default::default(),
            input_loss_action: core::default::Default::default(),
            restart_delay: core::default::Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsElRef
    {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ad_markers` after provisioning.\n"]
    pub fn ad_markers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ad_markers", self.base))
    }
    #[doc = "Get a reference to the value of field `authentication_scheme` after provisioning.\n"]
    pub fn authentication_scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authentication_scheme", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cache_full_behavior` after provisioning.\n"]
    pub fn cache_full_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cache_full_behavior", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cache_length` after provisioning.\n"]
    pub fn cache_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cache_length", self.base))
    }
    #[doc = "Get a reference to the value of field `caption_data` after provisioning.\n"]
    pub fn caption_data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.caption_data", self.base))
    }
    #[doc = "Get a reference to the value of field `input_loss_action` after provisioning.\n"]
    pub fn input_loss_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_loss_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `restart_delay` after provisioning.\n"]
    pub fn restart_delay(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.restart_delay", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    input_loss_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timed_metadata_id3_frame: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timed_metadata_id3_period: Option<PrimField<f64>>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsEl {
    #[doc = "Set the field `input_loss_action`.\n"]
    pub fn set_input_loss_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.input_loss_action = Some(v.into());
        self
    }
    #[doc = "Set the field `timed_metadata_id3_frame`.\n"]
    pub fn set_timed_metadata_id3_frame(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timed_metadata_id3_frame = Some(v.into());
        self
    }
    #[doc = "Set the field `timed_metadata_id3_period`.\n"]
    pub fn set_timed_metadata_id3_period(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timed_metadata_id3_period = Some(v.into());
        self
    }
}
impl ToListMappable
    for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsEl
{
    type O = BlockAssignable<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsEl
    {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsEl {
            input_loss_action: core::default::Default::default(),
            timed_metadata_id3_frame: core::default::Default::default(),
            timed_metadata_id3_period: core::default::Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsElRef
    {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `input_loss_action` after provisioning.\n"]
    pub fn input_loss_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_loss_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timed_metadata_id3_frame` after provisioning.\n"]
    pub fn timed_metadata_id3_frame(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timed_metadata_id3_frame", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timed_metadata_id3_period` after provisioning.\n"]
    pub fn timed_metadata_id3_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timed_metadata_id3_period", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElDynamic { archive_group_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsEl >> , frame_capture_group_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsEl >> , hls_group_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsEl >> , media_package_group_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsEl >> , ms_smooth_group_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsEl >> , multiplex_group_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsEl >> , rtmp_group_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsEl >> , udp_group_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] archive_group_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] frame_capture_group_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] hls_group_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] media_package_group_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] ms_smooth_group_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] multiplex_group_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] rtmp_group_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] udp_group_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsEl {
    #[doc = "Set the field `archive_group_settings`.\n"]
    pub fn set_archive_group_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.archive_group_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.archive_group_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `frame_capture_group_settings`.\n"]
    pub fn set_frame_capture_group_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.frame_capture_group_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.frame_capture_group_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `hls_group_settings`.\n"]
    pub fn set_hls_group_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hls_group_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hls_group_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `media_package_group_settings`.\n"]
    pub fn set_media_package_group_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.media_package_group_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.media_package_group_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ms_smooth_group_settings`.\n"]
    pub fn set_ms_smooth_group_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ms_smooth_group_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ms_smooth_group_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `multiplex_group_settings`.\n"]
    pub fn set_multiplex_group_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.multiplex_group_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.multiplex_group_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rtmp_group_settings`.\n"]
    pub fn set_rtmp_group_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rtmp_group_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rtmp_group_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `udp_group_settings`.\n"]
    pub fn set_udp_group_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.udp_group_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.udp_group_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsEl {
    type O = BlockAssignable<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsEl {}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsEl {
            archive_group_settings: core::default::Default::default(),
            frame_capture_group_settings: core::default::Default::default(),
            hls_group_settings: core::default::Default::default(),
            media_package_group_settings: core::default::Default::default(),
            ms_smooth_group_settings: core::default::Default::default(),
            multiplex_group_settings: core::default::Default::default(),
            rtmp_group_settings: core::default::Default::default(),
            udp_group_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `archive_group_settings` after provisioning.\n"]    pub fn archive_group_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.archive_group_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `frame_capture_group_settings` after provisioning.\n"]    pub fn frame_capture_group_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.frame_capture_group_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hls_group_settings` after provisioning.\n"]
    pub fn hls_group_settings(
        &self,
    ) -> ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hls_group_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `media_package_group_settings` after provisioning.\n"]    pub fn media_package_group_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.media_package_group_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ms_smooth_group_settings` after provisioning.\n"]    pub fn ms_smooth_group_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.ms_smooth_group_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `multiplex_group_settings` after provisioning.\n"]    pub fn multiplex_group_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.multiplex_group_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rtmp_group_settings` after provisioning.\n"]
    pub fn rtmp_group_settings(
        &self,
    ) -> ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rtmp_group_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `udp_group_settings` after provisioning.\n"]
    pub fn udp_group_settings(
        &self,
    ) -> ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.udp_group_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl
{
    network_id: PrimField<f64>,
    network_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rep_interval: Option<PrimField<f64>>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl { # [doc = "Set the field `rep_interval`.\n"] pub fn set_rep_interval (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . rep_interval = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl
{
    #[doc = ""]
    pub network_id: PrimField<f64>,
    #[doc = ""]
    pub network_name: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl { network_id : self . network_id , network_name : self . network_name , rep_interval : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `network_id` after provisioning.\n"] pub fn network_id (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.network_id" , self . base)) } # [doc = "Get a reference to the value of field `network_name` after provisioning.\n"] pub fn network_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.network_name" , self . base)) } # [doc = "Get a reference to the value of field `rep_interval` after provisioning.\n"] pub fn rep_interval (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.rep_interval" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    output_sdt: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rep_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_provider_name: Option<PrimField<String>>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl { # [doc = "Set the field `output_sdt`.\n"] pub fn set_output_sdt (mut self , v : impl Into < PrimField < String > >) -> Self { self . output_sdt = Some (v . into ()) ; self } # [doc = "Set the field `rep_interval`.\n"] pub fn set_rep_interval (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . rep_interval = Some (v . into ()) ; self } # [doc = "Set the field `service_name`.\n"] pub fn set_service_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . service_name = Some (v . into ()) ; self } # [doc = "Set the field `service_provider_name`.\n"] pub fn set_service_provider_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . service_provider_name = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl { output_sdt : core :: default :: Default :: default () , rep_interval : core :: default :: Default :: default () , service_name : core :: default :: Default :: default () , service_provider_name : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `output_sdt` after provisioning.\n"] pub fn output_sdt (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.output_sdt" , self . base)) } # [doc = "Get a reference to the value of field `rep_interval` after provisioning.\n"] pub fn rep_interval (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.rep_interval" , self . base)) } # [doc = "Get a reference to the value of field `service_name` after provisioning.\n"] pub fn service_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.service_name" , self . base)) } # [doc = "Get a reference to the value of field `service_provider_name` after provisioning.\n"] pub fn service_provider_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.service_provider_name" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    rep_interval: Option<PrimField<f64>>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl { # [doc = "Set the field `rep_interval`.\n"] pub fn set_rep_interval (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . rep_interval = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl { rep_interval : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `rep_interval` after provisioning.\n"] pub fn rep_interval (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.rep_interval" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDynamic { dvb_nit_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl >> , dvb_sdt_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl >> , dvb_tdt_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] absent_input_audio_behavior : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] arib : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] arib_captions_pid : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] arib_captions_pid_control : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] audio_buffer_model : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] audio_frames_per_pes : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] audio_pids : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] audio_stream_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] bitrate : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] buffer_model : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] cc_descriptor : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] dvb_sub_pids : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] dvb_teletext_pid : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] ebif : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] ebp_audio_interval : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] ebp_lookahead_ms : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] ebp_placement : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] ecm_pid : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] es_rate_in_pes : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] etv_platform_pid : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] etv_signal_pid : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] fragment_time : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] klv : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] klv_data_pids : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] nielsen_id3_behavior : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] null_packet_bitrate : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] pat_interval : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] pcr_control : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] pcr_period : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] pcr_pid : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] pmt_interval : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] pmt_pid : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] program_num : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] rate_mode : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] scte27_pids : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] scte35_control : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] scte35_pid : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] segmentation_markers : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] segmentation_style : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] segmentation_time : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] timed_metadata_behavior : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] timed_metadata_pid : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] transport_stream_id : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] video_pid : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] dvb_nit_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] dvb_sdt_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] dvb_tdt_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsEl { # [doc = "Set the field `absent_input_audio_behavior`.\n"] pub fn set_absent_input_audio_behavior (mut self , v : impl Into < PrimField < String > >) -> Self { self . absent_input_audio_behavior = Some (v . into ()) ; self } # [doc = "Set the field `arib`.\n"] pub fn set_arib (mut self , v : impl Into < PrimField < String > >) -> Self { self . arib = Some (v . into ()) ; self } # [doc = "Set the field `arib_captions_pid`.\n"] pub fn set_arib_captions_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . arib_captions_pid = Some (v . into ()) ; self } # [doc = "Set the field `arib_captions_pid_control`.\n"] pub fn set_arib_captions_pid_control (mut self , v : impl Into < PrimField < String > >) -> Self { self . arib_captions_pid_control = Some (v . into ()) ; self } # [doc = "Set the field `audio_buffer_model`.\n"] pub fn set_audio_buffer_model (mut self , v : impl Into < PrimField < String > >) -> Self { self . audio_buffer_model = Some (v . into ()) ; self } # [doc = "Set the field `audio_frames_per_pes`.\n"] pub fn set_audio_frames_per_pes (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . audio_frames_per_pes = Some (v . into ()) ; self } # [doc = "Set the field `audio_pids`.\n"] pub fn set_audio_pids (mut self , v : impl Into < PrimField < String > >) -> Self { self . audio_pids = Some (v . into ()) ; self } # [doc = "Set the field `audio_stream_type`.\n"] pub fn set_audio_stream_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . audio_stream_type = Some (v . into ()) ; self } # [doc = "Set the field `bitrate`.\n"] pub fn set_bitrate (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . bitrate = Some (v . into ()) ; self } # [doc = "Set the field `buffer_model`.\n"] pub fn set_buffer_model (mut self , v : impl Into < PrimField < String > >) -> Self { self . buffer_model = Some (v . into ()) ; self } # [doc = "Set the field `cc_descriptor`.\n"] pub fn set_cc_descriptor (mut self , v : impl Into < PrimField < String > >) -> Self { self . cc_descriptor = Some (v . into ()) ; self } # [doc = "Set the field `dvb_sub_pids`.\n"] pub fn set_dvb_sub_pids (mut self , v : impl Into < PrimField < String > >) -> Self { self . dvb_sub_pids = Some (v . into ()) ; self } # [doc = "Set the field `dvb_teletext_pid`.\n"] pub fn set_dvb_teletext_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . dvb_teletext_pid = Some (v . into ()) ; self } # [doc = "Set the field `ebif`.\n"] pub fn set_ebif (mut self , v : impl Into < PrimField < String > >) -> Self { self . ebif = Some (v . into ()) ; self } # [doc = "Set the field `ebp_audio_interval`.\n"] pub fn set_ebp_audio_interval (mut self , v : impl Into < PrimField < String > >) -> Self { self . ebp_audio_interval = Some (v . into ()) ; self } # [doc = "Set the field `ebp_lookahead_ms`.\n"] pub fn set_ebp_lookahead_ms (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . ebp_lookahead_ms = Some (v . into ()) ; self } # [doc = "Set the field `ebp_placement`.\n"] pub fn set_ebp_placement (mut self , v : impl Into < PrimField < String > >) -> Self { self . ebp_placement = Some (v . into ()) ; self } # [doc = "Set the field `ecm_pid`.\n"] pub fn set_ecm_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . ecm_pid = Some (v . into ()) ; self } # [doc = "Set the field `es_rate_in_pes`.\n"] pub fn set_es_rate_in_pes (mut self , v : impl Into < PrimField < String > >) -> Self { self . es_rate_in_pes = Some (v . into ()) ; self } # [doc = "Set the field `etv_platform_pid`.\n"] pub fn set_etv_platform_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . etv_platform_pid = Some (v . into ()) ; self } # [doc = "Set the field `etv_signal_pid`.\n"] pub fn set_etv_signal_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . etv_signal_pid = Some (v . into ()) ; self } # [doc = "Set the field `fragment_time`.\n"] pub fn set_fragment_time (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . fragment_time = Some (v . into ()) ; self } # [doc = "Set the field `klv`.\n"] pub fn set_klv (mut self , v : impl Into < PrimField < String > >) -> Self { self . klv = Some (v . into ()) ; self } # [doc = "Set the field `klv_data_pids`.\n"] pub fn set_klv_data_pids (mut self , v : impl Into < PrimField < String > >) -> Self { self . klv_data_pids = Some (v . into ()) ; self } # [doc = "Set the field `nielsen_id3_behavior`.\n"] pub fn set_nielsen_id3_behavior (mut self , v : impl Into < PrimField < String > >) -> Self { self . nielsen_id3_behavior = Some (v . into ()) ; self } # [doc = "Set the field `null_packet_bitrate`.\n"] pub fn set_null_packet_bitrate (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . null_packet_bitrate = Some (v . into ()) ; self } # [doc = "Set the field `pat_interval`.\n"] pub fn set_pat_interval (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . pat_interval = Some (v . into ()) ; self } # [doc = "Set the field `pcr_control`.\n"] pub fn set_pcr_control (mut self , v : impl Into < PrimField < String > >) -> Self { self . pcr_control = Some (v . into ()) ; self } # [doc = "Set the field `pcr_period`.\n"] pub fn set_pcr_period (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . pcr_period = Some (v . into ()) ; self } # [doc = "Set the field `pcr_pid`.\n"] pub fn set_pcr_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . pcr_pid = Some (v . into ()) ; self } # [doc = "Set the field `pmt_interval`.\n"] pub fn set_pmt_interval (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . pmt_interval = Some (v . into ()) ; self } # [doc = "Set the field `pmt_pid`.\n"] pub fn set_pmt_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . pmt_pid = Some (v . into ()) ; self } # [doc = "Set the field `program_num`.\n"] pub fn set_program_num (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . program_num = Some (v . into ()) ; self } # [doc = "Set the field `rate_mode`.\n"] pub fn set_rate_mode (mut self , v : impl Into < PrimField < String > >) -> Self { self . rate_mode = Some (v . into ()) ; self } # [doc = "Set the field `scte27_pids`.\n"] pub fn set_scte27_pids (mut self , v : impl Into < PrimField < String > >) -> Self { self . scte27_pids = Some (v . into ()) ; self } # [doc = "Set the field `scte35_control`.\n"] pub fn set_scte35_control (mut self , v : impl Into < PrimField < String > >) -> Self { self . scte35_control = Some (v . into ()) ; self } # [doc = "Set the field `scte35_pid`.\n"] pub fn set_scte35_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . scte35_pid = Some (v . into ()) ; self } # [doc = "Set the field `segmentation_markers`.\n"] pub fn set_segmentation_markers (mut self , v : impl Into < PrimField < String > >) -> Self { self . segmentation_markers = Some (v . into ()) ; self } # [doc = "Set the field `segmentation_style`.\n"] pub fn set_segmentation_style (mut self , v : impl Into < PrimField < String > >) -> Self { self . segmentation_style = Some (v . into ()) ; self } # [doc = "Set the field `segmentation_time`.\n"] pub fn set_segmentation_time (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . segmentation_time = Some (v . into ()) ; self } # [doc = "Set the field `timed_metadata_behavior`.\n"] pub fn set_timed_metadata_behavior (mut self , v : impl Into < PrimField < String > >) -> Self { self . timed_metadata_behavior = Some (v . into ()) ; self } # [doc = "Set the field `timed_metadata_pid`.\n"] pub fn set_timed_metadata_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . timed_metadata_pid = Some (v . into ()) ; self } # [doc = "Set the field `transport_stream_id`.\n"] pub fn set_transport_stream_id (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . transport_stream_id = Some (v . into ()) ; self } # [doc = "Set the field `video_pid`.\n"] pub fn set_video_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . video_pid = Some (v . into ()) ; self } # [doc = "Set the field `dvb_nit_settings`.\n"] pub fn set_dvb_nit_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . dvb_nit_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . dvb_nit_settings = Some (d) ; } } self } # [doc = "Set the field `dvb_sdt_settings`.\n"] pub fn set_dvb_sdt_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . dvb_sdt_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . dvb_sdt_settings = Some (d) ; } } self } # [doc = "Set the field `dvb_tdt_settings`.\n"] pub fn set_dvb_tdt_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . dvb_tdt_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . dvb_tdt_settings = Some (d) ; } } self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsEl { absent_input_audio_behavior : core :: default :: Default :: default () , arib : core :: default :: Default :: default () , arib_captions_pid : core :: default :: Default :: default () , arib_captions_pid_control : core :: default :: Default :: default () , audio_buffer_model : core :: default :: Default :: default () , audio_frames_per_pes : core :: default :: Default :: default () , audio_pids : core :: default :: Default :: default () , audio_stream_type : core :: default :: Default :: default () , bitrate : core :: default :: Default :: default () , buffer_model : core :: default :: Default :: default () , cc_descriptor : core :: default :: Default :: default () , dvb_sub_pids : core :: default :: Default :: default () , dvb_teletext_pid : core :: default :: Default :: default () , ebif : core :: default :: Default :: default () , ebp_audio_interval : core :: default :: Default :: default () , ebp_lookahead_ms : core :: default :: Default :: default () , ebp_placement : core :: default :: Default :: default () , ecm_pid : core :: default :: Default :: default () , es_rate_in_pes : core :: default :: Default :: default () , etv_platform_pid : core :: default :: Default :: default () , etv_signal_pid : core :: default :: Default :: default () , fragment_time : core :: default :: Default :: default () , klv : core :: default :: Default :: default () , klv_data_pids : core :: default :: Default :: default () , nielsen_id3_behavior : core :: default :: Default :: default () , null_packet_bitrate : core :: default :: Default :: default () , pat_interval : core :: default :: Default :: default () , pcr_control : core :: default :: Default :: default () , pcr_period : core :: default :: Default :: default () , pcr_pid : core :: default :: Default :: default () , pmt_interval : core :: default :: Default :: default () , pmt_pid : core :: default :: Default :: default () , program_num : core :: default :: Default :: default () , rate_mode : core :: default :: Default :: default () , scte27_pids : core :: default :: Default :: default () , scte35_control : core :: default :: Default :: default () , scte35_pid : core :: default :: Default :: default () , segmentation_markers : core :: default :: Default :: default () , segmentation_style : core :: default :: Default :: default () , segmentation_time : core :: default :: Default :: default () , timed_metadata_behavior : core :: default :: Default :: default () , timed_metadata_pid : core :: default :: Default :: default () , transport_stream_id : core :: default :: Default :: default () , video_pid : core :: default :: Default :: default () , dvb_nit_settings : core :: default :: Default :: default () , dvb_sdt_settings : core :: default :: Default :: default () , dvb_tdt_settings : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `absent_input_audio_behavior` after provisioning.\n"] pub fn absent_input_audio_behavior (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.absent_input_audio_behavior" , self . base)) } # [doc = "Get a reference to the value of field `arib` after provisioning.\n"] pub fn arib (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.arib" , self . base)) } # [doc = "Get a reference to the value of field `arib_captions_pid` after provisioning.\n"] pub fn arib_captions_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.arib_captions_pid" , self . base)) } # [doc = "Get a reference to the value of field `arib_captions_pid_control` after provisioning.\n"] pub fn arib_captions_pid_control (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.arib_captions_pid_control" , self . base)) } # [doc = "Get a reference to the value of field `audio_buffer_model` after provisioning.\n"] pub fn audio_buffer_model (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.audio_buffer_model" , self . base)) } # [doc = "Get a reference to the value of field `audio_frames_per_pes` after provisioning.\n"] pub fn audio_frames_per_pes (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.audio_frames_per_pes" , self . base)) } # [doc = "Get a reference to the value of field `audio_pids` after provisioning.\n"] pub fn audio_pids (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.audio_pids" , self . base)) } # [doc = "Get a reference to the value of field `audio_stream_type` after provisioning.\n"] pub fn audio_stream_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.audio_stream_type" , self . base)) } # [doc = "Get a reference to the value of field `bitrate` after provisioning.\n"] pub fn bitrate (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bitrate" , self . base)) } # [doc = "Get a reference to the value of field `buffer_model` after provisioning.\n"] pub fn buffer_model (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.buffer_model" , self . base)) } # [doc = "Get a reference to the value of field `cc_descriptor` after provisioning.\n"] pub fn cc_descriptor (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.cc_descriptor" , self . base)) } # [doc = "Get a reference to the value of field `dvb_sub_pids` after provisioning.\n"] pub fn dvb_sub_pids (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.dvb_sub_pids" , self . base)) } # [doc = "Get a reference to the value of field `dvb_teletext_pid` after provisioning.\n"] pub fn dvb_teletext_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.dvb_teletext_pid" , self . base)) } # [doc = "Get a reference to the value of field `ebif` after provisioning.\n"] pub fn ebif (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ebif" , self . base)) } # [doc = "Get a reference to the value of field `ebp_audio_interval` after provisioning.\n"] pub fn ebp_audio_interval (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ebp_audio_interval" , self . base)) } # [doc = "Get a reference to the value of field `ebp_lookahead_ms` after provisioning.\n"] pub fn ebp_lookahead_ms (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ebp_lookahead_ms" , self . base)) } # [doc = "Get a reference to the value of field `ebp_placement` after provisioning.\n"] pub fn ebp_placement (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ebp_placement" , self . base)) } # [doc = "Get a reference to the value of field `ecm_pid` after provisioning.\n"] pub fn ecm_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ecm_pid" , self . base)) } # [doc = "Get a reference to the value of field `es_rate_in_pes` after provisioning.\n"] pub fn es_rate_in_pes (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.es_rate_in_pes" , self . base)) } # [doc = "Get a reference to the value of field `etv_platform_pid` after provisioning.\n"] pub fn etv_platform_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.etv_platform_pid" , self . base)) } # [doc = "Get a reference to the value of field `etv_signal_pid` after provisioning.\n"] pub fn etv_signal_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.etv_signal_pid" , self . base)) } # [doc = "Get a reference to the value of field `fragment_time` after provisioning.\n"] pub fn fragment_time (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.fragment_time" , self . base)) } # [doc = "Get a reference to the value of field `klv` after provisioning.\n"] pub fn klv (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.klv" , self . base)) } # [doc = "Get a reference to the value of field `klv_data_pids` after provisioning.\n"] pub fn klv_data_pids (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.klv_data_pids" , self . base)) } # [doc = "Get a reference to the value of field `nielsen_id3_behavior` after provisioning.\n"] pub fn nielsen_id3_behavior (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.nielsen_id3_behavior" , self . base)) } # [doc = "Get a reference to the value of field `null_packet_bitrate` after provisioning.\n"] pub fn null_packet_bitrate (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.null_packet_bitrate" , self . base)) } # [doc = "Get a reference to the value of field `pat_interval` after provisioning.\n"] pub fn pat_interval (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pat_interval" , self . base)) } # [doc = "Get a reference to the value of field `pcr_control` after provisioning.\n"] pub fn pcr_control (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pcr_control" , self . base)) } # [doc = "Get a reference to the value of field `pcr_period` after provisioning.\n"] pub fn pcr_period (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pcr_period" , self . base)) } # [doc = "Get a reference to the value of field `pcr_pid` after provisioning.\n"] pub fn pcr_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pcr_pid" , self . base)) } # [doc = "Get a reference to the value of field `pmt_interval` after provisioning.\n"] pub fn pmt_interval (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pmt_interval" , self . base)) } # [doc = "Get a reference to the value of field `pmt_pid` after provisioning.\n"] pub fn pmt_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pmt_pid" , self . base)) } # [doc = "Get a reference to the value of field `program_num` after provisioning.\n"] pub fn program_num (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.program_num" , self . base)) } # [doc = "Get a reference to the value of field `rate_mode` after provisioning.\n"] pub fn rate_mode (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.rate_mode" , self . base)) } # [doc = "Get a reference to the value of field `scte27_pids` after provisioning.\n"] pub fn scte27_pids (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.scte27_pids" , self . base)) } # [doc = "Get a reference to the value of field `scte35_control` after provisioning.\n"] pub fn scte35_control (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.scte35_control" , self . base)) } # [doc = "Get a reference to the value of field `scte35_pid` after provisioning.\n"] pub fn scte35_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.scte35_pid" , self . base)) } # [doc = "Get a reference to the value of field `segmentation_markers` after provisioning.\n"] pub fn segmentation_markers (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.segmentation_markers" , self . base)) } # [doc = "Get a reference to the value of field `segmentation_style` after provisioning.\n"] pub fn segmentation_style (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.segmentation_style" , self . base)) } # [doc = "Get a reference to the value of field `segmentation_time` after provisioning.\n"] pub fn segmentation_time (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.segmentation_time" , self . base)) } # [doc = "Get a reference to the value of field `timed_metadata_behavior` after provisioning.\n"] pub fn timed_metadata_behavior (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.timed_metadata_behavior" , self . base)) } # [doc = "Get a reference to the value of field `timed_metadata_pid` after provisioning.\n"] pub fn timed_metadata_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.timed_metadata_pid" , self . base)) } # [doc = "Get a reference to the value of field `transport_stream_id` after provisioning.\n"] pub fn transport_stream_id (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.transport_stream_id" , self . base)) } # [doc = "Get a reference to the value of field `video_pid` after provisioning.\n"] pub fn video_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.video_pid" , self . base)) } # [doc = "Get a reference to the value of field `dvb_nit_settings` after provisioning.\n"] pub fn dvb_nit_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.dvb_nit_settings" , self . base)) } # [doc = "Get a reference to the value of field `dvb_sdt_settings` after provisioning.\n"] pub fn dvb_sdt_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.dvb_sdt_settings" , self . base)) } # [doc = "Get a reference to the value of field `dvb_tdt_settings` after provisioning.\n"] pub fn dvb_tdt_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.dvb_tdt_settings" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsEl
{}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsEl { } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElDynamic { m2ts_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsEl >> , raw_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] m2ts_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] raw_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsEl { # [doc = "Set the field `m2ts_settings`.\n"] pub fn set_m2ts_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . m2ts_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . m2ts_settings = Some (d) ; } } self } # [doc = "Set the field `raw_settings`.\n"] pub fn set_raw_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . raw_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . raw_settings = Some (d) ; } } self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsEl { m2ts_settings : core :: default :: Default :: default () , raw_settings : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `m2ts_settings` after provisioning.\n"] pub fn m2ts_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.m2ts_settings" , self . base)) } # [doc = "Get a reference to the value of field `raw_settings` after provisioning.\n"] pub fn raw_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.raw_settings" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElDynamic { container_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] extension : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] name_modifier : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] container_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElDynamic , }
impl
    MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsEl
{
    #[doc = "Set the field `extension`.\n"]
    pub fn set_extension(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.extension = Some(v.into());
        self
    }
    #[doc = "Set the field `name_modifier`.\n"]
    pub fn set_name_modifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name_modifier = Some(v.into());
        self
    }
    #[doc = "Set the field `container_settings`.\n"]
    pub fn set_container_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.container_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.container_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsEl { extension : core :: default :: Default :: default () , name_modifier : core :: default :: Default :: default () , container_settings : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `extension` after provisioning.\n"] pub fn extension (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.extension" , self . base)) } # [doc = "Get a reference to the value of field `name_modifier` after provisioning.\n"] pub fn name_modifier (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name_modifier" , self . base)) } # [doc = "Get a reference to the value of field `container_settings` after provisioning.\n"] pub fn container_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.container_settings" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    name_modifier: Option<PrimField<String>>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsEl { # [doc = "Set the field `name_modifier`.\n"] pub fn set_name_modifier (mut self , v : impl Into < PrimField < String > >) -> Self { self . name_modifier = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsEl { name_modifier : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name_modifier` after provisioning.\n"] pub fn name_modifier (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name_modifier" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    password_param: Option<PrimField<String>>,
    uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageEl { # [doc = "Set the field `password_param`.\n"] pub fn set_password_param (mut self , v : impl Into < PrimField < String > >) -> Self { self . password_param = Some (v . into ()) ; self } # [doc = "Set the field `username`.\n"] pub fn set_username (mut self , v : impl Into < PrimField < String > >) -> Self { self . username = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageEl
{
    #[doc = ""]
    pub uri: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageEl { password_param : core :: default :: Default :: default () , uri : self . uri , username : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `password_param` after provisioning.\n"] pub fn password_param (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.password_param" , self . base)) } # [doc = "Get a reference to the value of field `uri` after provisioning.\n"] pub fn uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.uri" , self . base)) } # [doc = "Get a reference to the value of field `username` after provisioning.\n"] pub fn username (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.username" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElDynamic { audio_only_image : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] audio_group_id : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] audio_track_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] segment_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] audio_only_image : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageEl > > , dynamic : MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsEl { # [doc = "Set the field `audio_group_id`.\n"] pub fn set_audio_group_id (mut self , v : impl Into < PrimField < String > >) -> Self { self . audio_group_id = Some (v . into ()) ; self } # [doc = "Set the field `audio_track_type`.\n"] pub fn set_audio_track_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . audio_track_type = Some (v . into ()) ; self } # [doc = "Set the field `segment_type`.\n"] pub fn set_segment_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . segment_type = Some (v . into ()) ; self } # [doc = "Set the field `audio_only_image`.\n"] pub fn set_audio_only_image (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . audio_only_image = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . audio_only_image = Some (d) ; } } self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsEl { audio_group_id : core :: default :: Default :: default () , audio_track_type : core :: default :: Default :: default () , segment_type : core :: default :: Default :: default () , audio_only_image : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `audio_group_id` after provisioning.\n"] pub fn audio_group_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.audio_group_id" , self . base)) } # [doc = "Get a reference to the value of field `audio_track_type` after provisioning.\n"] pub fn audio_track_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.audio_track_type" , self . base)) } # [doc = "Get a reference to the value of field `segment_type` after provisioning.\n"] pub fn segment_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.segment_type" , self . base)) } # [doc = "Get a reference to the value of field `audio_only_image` after provisioning.\n"] pub fn audio_only_image (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.audio_only_image" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_rendition_sets: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nielsen_id3_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timed_metadata_behavior: Option<PrimField<String>>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsEl { # [doc = "Set the field `audio_rendition_sets`.\n"] pub fn set_audio_rendition_sets (mut self , v : impl Into < PrimField < String > >) -> Self { self . audio_rendition_sets = Some (v . into ()) ; self } # [doc = "Set the field `nielsen_id3_behavior`.\n"] pub fn set_nielsen_id3_behavior (mut self , v : impl Into < PrimField < String > >) -> Self { self . nielsen_id3_behavior = Some (v . into ()) ; self } # [doc = "Set the field `timed_metadata_behavior`.\n"] pub fn set_timed_metadata_behavior (mut self , v : impl Into < PrimField < String > >) -> Self { self . timed_metadata_behavior = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsEl { audio_rendition_sets : core :: default :: Default :: default () , nielsen_id3_behavior : core :: default :: Default :: default () , timed_metadata_behavior : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `audio_rendition_sets` after provisioning.\n"] pub fn audio_rendition_sets (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.audio_rendition_sets" , self . base)) } # [doc = "Get a reference to the value of field `nielsen_id3_behavior` after provisioning.\n"] pub fn nielsen_id3_behavior (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.nielsen_id3_behavior" , self . base)) } # [doc = "Get a reference to the value of field `timed_metadata_behavior` after provisioning.\n"] pub fn timed_metadata_behavior (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.timed_metadata_behavior" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsEl
{}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsEl { } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_frames_per_pes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_pids: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecm_pid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nielsen_id3_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pat_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pcr_control: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pcr_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pcr_pid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pmt_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pmt_pid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    program_num: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scte35_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scte35_pid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timed_metadata_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timed_metadata_pid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transport_stream_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_pid: Option<PrimField<String>>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsEl { # [doc = "Set the field `audio_frames_per_pes`.\n"] pub fn set_audio_frames_per_pes (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . audio_frames_per_pes = Some (v . into ()) ; self } # [doc = "Set the field `audio_pids`.\n"] pub fn set_audio_pids (mut self , v : impl Into < PrimField < String > >) -> Self { self . audio_pids = Some (v . into ()) ; self } # [doc = "Set the field `ecm_pid`.\n"] pub fn set_ecm_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . ecm_pid = Some (v . into ()) ; self } # [doc = "Set the field `nielsen_id3_behavior`.\n"] pub fn set_nielsen_id3_behavior (mut self , v : impl Into < PrimField < String > >) -> Self { self . nielsen_id3_behavior = Some (v . into ()) ; self } # [doc = "Set the field `pat_interval`.\n"] pub fn set_pat_interval (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . pat_interval = Some (v . into ()) ; self } # [doc = "Set the field `pcr_control`.\n"] pub fn set_pcr_control (mut self , v : impl Into < PrimField < String > >) -> Self { self . pcr_control = Some (v . into ()) ; self } # [doc = "Set the field `pcr_period`.\n"] pub fn set_pcr_period (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . pcr_period = Some (v . into ()) ; self } # [doc = "Set the field `pcr_pid`.\n"] pub fn set_pcr_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . pcr_pid = Some (v . into ()) ; self } # [doc = "Set the field `pmt_interval`.\n"] pub fn set_pmt_interval (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . pmt_interval = Some (v . into ()) ; self } # [doc = "Set the field `pmt_pid`.\n"] pub fn set_pmt_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . pmt_pid = Some (v . into ()) ; self } # [doc = "Set the field `program_num`.\n"] pub fn set_program_num (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . program_num = Some (v . into ()) ; self } # [doc = "Set the field `scte35_behavior`.\n"] pub fn set_scte35_behavior (mut self , v : impl Into < PrimField < String > >) -> Self { self . scte35_behavior = Some (v . into ()) ; self } # [doc = "Set the field `scte35_pid`.\n"] pub fn set_scte35_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . scte35_pid = Some (v . into ()) ; self } # [doc = "Set the field `timed_metadata_behavior`.\n"] pub fn set_timed_metadata_behavior (mut self , v : impl Into < PrimField < String > >) -> Self { self . timed_metadata_behavior = Some (v . into ()) ; self } # [doc = "Set the field `timed_metadata_pid`.\n"] pub fn set_timed_metadata_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . timed_metadata_pid = Some (v . into ()) ; self } # [doc = "Set the field `transport_stream_id`.\n"] pub fn set_transport_stream_id (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . transport_stream_id = Some (v . into ()) ; self } # [doc = "Set the field `video_pid`.\n"] pub fn set_video_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . video_pid = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsEl { audio_frames_per_pes : core :: default :: Default :: default () , audio_pids : core :: default :: Default :: default () , ecm_pid : core :: default :: Default :: default () , nielsen_id3_behavior : core :: default :: Default :: default () , pat_interval : core :: default :: Default :: default () , pcr_control : core :: default :: Default :: default () , pcr_period : core :: default :: Default :: default () , pcr_pid : core :: default :: Default :: default () , pmt_interval : core :: default :: Default :: default () , pmt_pid : core :: default :: Default :: default () , program_num : core :: default :: Default :: default () , scte35_behavior : core :: default :: Default :: default () , scte35_pid : core :: default :: Default :: default () , timed_metadata_behavior : core :: default :: Default :: default () , timed_metadata_pid : core :: default :: Default :: default () , transport_stream_id : core :: default :: Default :: default () , video_pid : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `audio_frames_per_pes` after provisioning.\n"] pub fn audio_frames_per_pes (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.audio_frames_per_pes" , self . base)) } # [doc = "Get a reference to the value of field `audio_pids` after provisioning.\n"] pub fn audio_pids (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.audio_pids" , self . base)) } # [doc = "Get a reference to the value of field `ecm_pid` after provisioning.\n"] pub fn ecm_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ecm_pid" , self . base)) } # [doc = "Get a reference to the value of field `nielsen_id3_behavior` after provisioning.\n"] pub fn nielsen_id3_behavior (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.nielsen_id3_behavior" , self . base)) } # [doc = "Get a reference to the value of field `pat_interval` after provisioning.\n"] pub fn pat_interval (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pat_interval" , self . base)) } # [doc = "Get a reference to the value of field `pcr_control` after provisioning.\n"] pub fn pcr_control (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pcr_control" , self . base)) } # [doc = "Get a reference to the value of field `pcr_period` after provisioning.\n"] pub fn pcr_period (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pcr_period" , self . base)) } # [doc = "Get a reference to the value of field `pcr_pid` after provisioning.\n"] pub fn pcr_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pcr_pid" , self . base)) } # [doc = "Get a reference to the value of field `pmt_interval` after provisioning.\n"] pub fn pmt_interval (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pmt_interval" , self . base)) } # [doc = "Get a reference to the value of field `pmt_pid` after provisioning.\n"] pub fn pmt_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pmt_pid" , self . base)) } # [doc = "Get a reference to the value of field `program_num` after provisioning.\n"] pub fn program_num (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.program_num" , self . base)) } # [doc = "Get a reference to the value of field `scte35_behavior` after provisioning.\n"] pub fn scte35_behavior (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.scte35_behavior" , self . base)) } # [doc = "Get a reference to the value of field `scte35_pid` after provisioning.\n"] pub fn scte35_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.scte35_pid" , self . base)) } # [doc = "Get a reference to the value of field `timed_metadata_behavior` after provisioning.\n"] pub fn timed_metadata_behavior (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.timed_metadata_behavior" , self . base)) } # [doc = "Get a reference to the value of field `timed_metadata_pid` after provisioning.\n"] pub fn timed_metadata_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.timed_metadata_pid" , self . base)) } # [doc = "Get a reference to the value of field `transport_stream_id` after provisioning.\n"] pub fn transport_stream_id (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.transport_stream_id" , self . base)) } # [doc = "Get a reference to the value of field `video_pid` after provisioning.\n"] pub fn video_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.video_pid" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElDynamic { m3u8_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] audio_rendition_sets : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] m3u8_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsEl { # [doc = "Set the field `audio_rendition_sets`.\n"] pub fn set_audio_rendition_sets (mut self , v : impl Into < PrimField < String > >) -> Self { self . audio_rendition_sets = Some (v . into ()) ; self } # [doc = "Set the field `m3u8_settings`.\n"] pub fn set_m3u8_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . m3u8_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . m3u8_settings = Some (d) ; } } self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsEl { audio_rendition_sets : core :: default :: Default :: default () , m3u8_settings : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `audio_rendition_sets` after provisioning.\n"] pub fn audio_rendition_sets (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.audio_rendition_sets" , self . base)) } # [doc = "Get a reference to the value of field `m3u8_settings` after provisioning.\n"] pub fn m3u8_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.m3u8_settings" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElDynamic { audio_only_hls_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsEl >> , fmp4_hls_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsEl >> , frame_capture_hls_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsEl >> , standard_hls_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] audio_only_hls_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] fmp4_hls_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] frame_capture_hls_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] standard_hls_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsEl { # [doc = "Set the field `audio_only_hls_settings`.\n"] pub fn set_audio_only_hls_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . audio_only_hls_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . audio_only_hls_settings = Some (d) ; } } self } # [doc = "Set the field `fmp4_hls_settings`.\n"] pub fn set_fmp4_hls_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . fmp4_hls_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . fmp4_hls_settings = Some (d) ; } } self } # [doc = "Set the field `frame_capture_hls_settings`.\n"] pub fn set_frame_capture_hls_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . frame_capture_hls_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . frame_capture_hls_settings = Some (d) ; } } self } # [doc = "Set the field `standard_hls_settings`.\n"] pub fn set_standard_hls_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . standard_hls_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . standard_hls_settings = Some (d) ; } } self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsEl { audio_only_hls_settings : core :: default :: Default :: default () , fmp4_hls_settings : core :: default :: Default :: default () , frame_capture_hls_settings : core :: default :: Default :: default () , standard_hls_settings : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `audio_only_hls_settings` after provisioning.\n"] pub fn audio_only_hls_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.audio_only_hls_settings" , self . base)) } # [doc = "Get a reference to the value of field `fmp4_hls_settings` after provisioning.\n"] pub fn fmp4_hls_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.fmp4_hls_settings" , self . base)) } # [doc = "Get a reference to the value of field `frame_capture_hls_settings` after provisioning.\n"] pub fn frame_capture_hls_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.frame_capture_hls_settings" , self . base)) } # [doc = "Get a reference to the value of field `standard_hls_settings` after provisioning.\n"] pub fn standard_hls_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.standard_hls_settings" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElDynamic { hls_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] h265_packaging_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] name_modifier : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] segment_modifier : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] hls_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsEl {
    #[doc = "Set the field `h265_packaging_type`.\n"]
    pub fn set_h265_packaging_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.h265_packaging_type = Some(v.into());
        self
    }
    #[doc = "Set the field `name_modifier`.\n"]
    pub fn set_name_modifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name_modifier = Some(v.into());
        self
    }
    #[doc = "Set the field `segment_modifier`.\n"]
    pub fn set_segment_modifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.segment_modifier = Some(v.into());
        self
    }
    #[doc = "Set the field `hls_settings`.\n"]
    pub fn set_hls_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hls_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hls_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsEl
{
    type O = BlockAssignable<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsEl
{}
impl
    BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsEl
{
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsEl
    {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsEl { h265_packaging_type : core :: default :: Default :: default () , name_modifier : core :: default :: Default :: default () , segment_modifier : core :: default :: Default :: default () , hls_settings : core :: default :: Default :: default () , dynamic : Default :: default () , }
    }
}
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElRef { shared : shared , base : base . to_string () , } } }
impl
    MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `h265_packaging_type` after provisioning.\n"]
    pub fn h265_packaging_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.h265_packaging_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name_modifier` after provisioning.\n"]
    pub fn name_modifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_modifier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `segment_modifier` after provisioning.\n"]
    pub fn segment_modifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.segment_modifier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hls_settings` after provisioning.\n"]    pub fn hls_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.hls_settings", self.base))
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsEl
{}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsEl { } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    h265_packaging_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_modifier: Option<PrimField<String>>,
}
impl
    MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsEl
{
    #[doc = "Set the field `h265_packaging_type`.\n"]
    pub fn set_h265_packaging_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.h265_packaging_type = Some(v.into());
        self
    }
    #[doc = "Set the field `name_modifier`.\n"]
    pub fn set_name_modifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name_modifier = Some(v.into());
        self
    }
}
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsEl { h265_packaging_type : core :: default :: Default :: default () , name_modifier : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `h265_packaging_type` after provisioning.\n"] pub fn h265_packaging_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.h265_packaging_type" , self . base)) } # [doc = "Get a reference to the value of field `name_modifier` after provisioning.\n"] pub fn name_modifier (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name_modifier" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationEl
{
    destination_ref_id: PrimField<String>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationEl
{
    #[doc = ""]
    pub destination_ref_id: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationEl { destination_ref_id : self . destination_ref_id , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `destination_ref_id` after provisioning.\n"] pub fn destination_ref_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.destination_ref_id" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDynamic { destination : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] destination : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationEl > > , dynamic : MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsEl { # [doc = "Set the field `destination`.\n"] pub fn set_destination (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . destination = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . destination = Some (d) ; } } self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsEl { destination : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `destination` after provisioning.\n"] pub fn destination (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.destination" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationEl
{
    destination_ref_id: PrimField<String>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationEl
{
    #[doc = ""]
    pub destination_ref_id: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationEl { destination_ref_id : self . destination_ref_id , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `destination_ref_id` after provisioning.\n"] pub fn destination_ref_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.destination_ref_id" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDynamic { destination : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] certificate_mode : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] connection_retry_interval : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] num_retries : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] destination : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationEl > > , dynamic : MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsEl {
    #[doc = "Set the field `certificate_mode`.\n"]
    pub fn set_certificate_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `connection_retry_interval`.\n"]
    pub fn set_connection_retry_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.connection_retry_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `num_retries`.\n"]
    pub fn set_num_retries(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_retries = Some(v.into());
        self
    }
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsEl
{
    type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsEl { certificate_mode : core :: default :: Default :: default () , connection_retry_interval : core :: default :: Default :: default () , num_retries : core :: default :: Default :: default () , destination : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElRef { shared : shared , base : base . to_string () , } } }
impl
    MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificate_mode` after provisioning.\n"]
    pub fn certificate_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `connection_retry_interval` after provisioning.\n"]
    pub fn connection_retry_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_retry_interval", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `num_retries` after provisioning.\n"]
    pub fn num_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_retries", self.base))
    }
    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]    pub fn destination (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationElRef >{
        ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl
{
    network_id: PrimField<f64>,
    network_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rep_interval: Option<PrimField<f64>>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl { # [doc = "Set the field `rep_interval`.\n"] pub fn set_rep_interval (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . rep_interval = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl
{
    #[doc = ""]
    pub network_id: PrimField<f64>,
    #[doc = ""]
    pub network_name: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl { network_id : self . network_id , network_name : self . network_name , rep_interval : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `network_id` after provisioning.\n"] pub fn network_id (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.network_id" , self . base)) } # [doc = "Get a reference to the value of field `network_name` after provisioning.\n"] pub fn network_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.network_name" , self . base)) } # [doc = "Get a reference to the value of field `rep_interval` after provisioning.\n"] pub fn rep_interval (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.rep_interval" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    output_sdt: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rep_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_provider_name: Option<PrimField<String>>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl { # [doc = "Set the field `output_sdt`.\n"] pub fn set_output_sdt (mut self , v : impl Into < PrimField < String > >) -> Self { self . output_sdt = Some (v . into ()) ; self } # [doc = "Set the field `rep_interval`.\n"] pub fn set_rep_interval (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . rep_interval = Some (v . into ()) ; self } # [doc = "Set the field `service_name`.\n"] pub fn set_service_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . service_name = Some (v . into ()) ; self } # [doc = "Set the field `service_provider_name`.\n"] pub fn set_service_provider_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . service_provider_name = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl { output_sdt : core :: default :: Default :: default () , rep_interval : core :: default :: Default :: default () , service_name : core :: default :: Default :: default () , service_provider_name : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `output_sdt` after provisioning.\n"] pub fn output_sdt (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.output_sdt" , self . base)) } # [doc = "Get a reference to the value of field `rep_interval` after provisioning.\n"] pub fn rep_interval (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.rep_interval" , self . base)) } # [doc = "Get a reference to the value of field `service_name` after provisioning.\n"] pub fn service_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.service_name" , self . base)) } # [doc = "Get a reference to the value of field `service_provider_name` after provisioning.\n"] pub fn service_provider_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.service_provider_name" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    rep_interval: Option<PrimField<f64>>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl { # [doc = "Set the field `rep_interval`.\n"] pub fn set_rep_interval (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . rep_interval = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl { rep_interval : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `rep_interval` after provisioning.\n"] pub fn rep_interval (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.rep_interval" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDynamic { dvb_nit_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl >> , dvb_sdt_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl >> , dvb_tdt_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] absent_input_audio_behavior : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] arib : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] arib_captions_pid : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] arib_captions_pid_control : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] audio_buffer_model : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] audio_frames_per_pes : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] audio_pids : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] audio_stream_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] bitrate : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] buffer_model : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] cc_descriptor : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] dvb_sub_pids : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] dvb_teletext_pid : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] ebif : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] ebp_audio_interval : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] ebp_lookahead_ms : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] ebp_placement : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] ecm_pid : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] es_rate_in_pes : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] etv_platform_pid : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] etv_signal_pid : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] fragment_time : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] klv : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] klv_data_pids : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] nielsen_id3_behavior : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] null_packet_bitrate : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] pat_interval : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] pcr_control : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] pcr_period : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] pcr_pid : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] pmt_interval : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] pmt_pid : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] program_num : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] rate_mode : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] scte27_pids : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] scte35_control : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] scte35_pid : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] segmentation_markers : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] segmentation_style : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] segmentation_time : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] timed_metadata_behavior : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] timed_metadata_pid : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] transport_stream_id : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] video_pid : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] dvb_nit_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] dvb_sdt_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] dvb_tdt_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsEl { # [doc = "Set the field `absent_input_audio_behavior`.\n"] pub fn set_absent_input_audio_behavior (mut self , v : impl Into < PrimField < String > >) -> Self { self . absent_input_audio_behavior = Some (v . into ()) ; self } # [doc = "Set the field `arib`.\n"] pub fn set_arib (mut self , v : impl Into < PrimField < String > >) -> Self { self . arib = Some (v . into ()) ; self } # [doc = "Set the field `arib_captions_pid`.\n"] pub fn set_arib_captions_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . arib_captions_pid = Some (v . into ()) ; self } # [doc = "Set the field `arib_captions_pid_control`.\n"] pub fn set_arib_captions_pid_control (mut self , v : impl Into < PrimField < String > >) -> Self { self . arib_captions_pid_control = Some (v . into ()) ; self } # [doc = "Set the field `audio_buffer_model`.\n"] pub fn set_audio_buffer_model (mut self , v : impl Into < PrimField < String > >) -> Self { self . audio_buffer_model = Some (v . into ()) ; self } # [doc = "Set the field `audio_frames_per_pes`.\n"] pub fn set_audio_frames_per_pes (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . audio_frames_per_pes = Some (v . into ()) ; self } # [doc = "Set the field `audio_pids`.\n"] pub fn set_audio_pids (mut self , v : impl Into < PrimField < String > >) -> Self { self . audio_pids = Some (v . into ()) ; self } # [doc = "Set the field `audio_stream_type`.\n"] pub fn set_audio_stream_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . audio_stream_type = Some (v . into ()) ; self } # [doc = "Set the field `bitrate`.\n"] pub fn set_bitrate (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . bitrate = Some (v . into ()) ; self } # [doc = "Set the field `buffer_model`.\n"] pub fn set_buffer_model (mut self , v : impl Into < PrimField < String > >) -> Self { self . buffer_model = Some (v . into ()) ; self } # [doc = "Set the field `cc_descriptor`.\n"] pub fn set_cc_descriptor (mut self , v : impl Into < PrimField < String > >) -> Self { self . cc_descriptor = Some (v . into ()) ; self } # [doc = "Set the field `dvb_sub_pids`.\n"] pub fn set_dvb_sub_pids (mut self , v : impl Into < PrimField < String > >) -> Self { self . dvb_sub_pids = Some (v . into ()) ; self } # [doc = "Set the field `dvb_teletext_pid`.\n"] pub fn set_dvb_teletext_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . dvb_teletext_pid = Some (v . into ()) ; self } # [doc = "Set the field `ebif`.\n"] pub fn set_ebif (mut self , v : impl Into < PrimField < String > >) -> Self { self . ebif = Some (v . into ()) ; self } # [doc = "Set the field `ebp_audio_interval`.\n"] pub fn set_ebp_audio_interval (mut self , v : impl Into < PrimField < String > >) -> Self { self . ebp_audio_interval = Some (v . into ()) ; self } # [doc = "Set the field `ebp_lookahead_ms`.\n"] pub fn set_ebp_lookahead_ms (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . ebp_lookahead_ms = Some (v . into ()) ; self } # [doc = "Set the field `ebp_placement`.\n"] pub fn set_ebp_placement (mut self , v : impl Into < PrimField < String > >) -> Self { self . ebp_placement = Some (v . into ()) ; self } # [doc = "Set the field `ecm_pid`.\n"] pub fn set_ecm_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . ecm_pid = Some (v . into ()) ; self } # [doc = "Set the field `es_rate_in_pes`.\n"] pub fn set_es_rate_in_pes (mut self , v : impl Into < PrimField < String > >) -> Self { self . es_rate_in_pes = Some (v . into ()) ; self } # [doc = "Set the field `etv_platform_pid`.\n"] pub fn set_etv_platform_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . etv_platform_pid = Some (v . into ()) ; self } # [doc = "Set the field `etv_signal_pid`.\n"] pub fn set_etv_signal_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . etv_signal_pid = Some (v . into ()) ; self } # [doc = "Set the field `fragment_time`.\n"] pub fn set_fragment_time (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . fragment_time = Some (v . into ()) ; self } # [doc = "Set the field `klv`.\n"] pub fn set_klv (mut self , v : impl Into < PrimField < String > >) -> Self { self . klv = Some (v . into ()) ; self } # [doc = "Set the field `klv_data_pids`.\n"] pub fn set_klv_data_pids (mut self , v : impl Into < PrimField < String > >) -> Self { self . klv_data_pids = Some (v . into ()) ; self } # [doc = "Set the field `nielsen_id3_behavior`.\n"] pub fn set_nielsen_id3_behavior (mut self , v : impl Into < PrimField < String > >) -> Self { self . nielsen_id3_behavior = Some (v . into ()) ; self } # [doc = "Set the field `null_packet_bitrate`.\n"] pub fn set_null_packet_bitrate (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . null_packet_bitrate = Some (v . into ()) ; self } # [doc = "Set the field `pat_interval`.\n"] pub fn set_pat_interval (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . pat_interval = Some (v . into ()) ; self } # [doc = "Set the field `pcr_control`.\n"] pub fn set_pcr_control (mut self , v : impl Into < PrimField < String > >) -> Self { self . pcr_control = Some (v . into ()) ; self } # [doc = "Set the field `pcr_period`.\n"] pub fn set_pcr_period (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . pcr_period = Some (v . into ()) ; self } # [doc = "Set the field `pcr_pid`.\n"] pub fn set_pcr_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . pcr_pid = Some (v . into ()) ; self } # [doc = "Set the field `pmt_interval`.\n"] pub fn set_pmt_interval (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . pmt_interval = Some (v . into ()) ; self } # [doc = "Set the field `pmt_pid`.\n"] pub fn set_pmt_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . pmt_pid = Some (v . into ()) ; self } # [doc = "Set the field `program_num`.\n"] pub fn set_program_num (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . program_num = Some (v . into ()) ; self } # [doc = "Set the field `rate_mode`.\n"] pub fn set_rate_mode (mut self , v : impl Into < PrimField < String > >) -> Self { self . rate_mode = Some (v . into ()) ; self } # [doc = "Set the field `scte27_pids`.\n"] pub fn set_scte27_pids (mut self , v : impl Into < PrimField < String > >) -> Self { self . scte27_pids = Some (v . into ()) ; self } # [doc = "Set the field `scte35_control`.\n"] pub fn set_scte35_control (mut self , v : impl Into < PrimField < String > >) -> Self { self . scte35_control = Some (v . into ()) ; self } # [doc = "Set the field `scte35_pid`.\n"] pub fn set_scte35_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . scte35_pid = Some (v . into ()) ; self } # [doc = "Set the field `segmentation_markers`.\n"] pub fn set_segmentation_markers (mut self , v : impl Into < PrimField < String > >) -> Self { self . segmentation_markers = Some (v . into ()) ; self } # [doc = "Set the field `segmentation_style`.\n"] pub fn set_segmentation_style (mut self , v : impl Into < PrimField < String > >) -> Self { self . segmentation_style = Some (v . into ()) ; self } # [doc = "Set the field `segmentation_time`.\n"] pub fn set_segmentation_time (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . segmentation_time = Some (v . into ()) ; self } # [doc = "Set the field `timed_metadata_behavior`.\n"] pub fn set_timed_metadata_behavior (mut self , v : impl Into < PrimField < String > >) -> Self { self . timed_metadata_behavior = Some (v . into ()) ; self } # [doc = "Set the field `timed_metadata_pid`.\n"] pub fn set_timed_metadata_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . timed_metadata_pid = Some (v . into ()) ; self } # [doc = "Set the field `transport_stream_id`.\n"] pub fn set_transport_stream_id (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . transport_stream_id = Some (v . into ()) ; self } # [doc = "Set the field `video_pid`.\n"] pub fn set_video_pid (mut self , v : impl Into < PrimField < String > >) -> Self { self . video_pid = Some (v . into ()) ; self } # [doc = "Set the field `dvb_nit_settings`.\n"] pub fn set_dvb_nit_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . dvb_nit_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . dvb_nit_settings = Some (d) ; } } self } # [doc = "Set the field `dvb_sdt_settings`.\n"] pub fn set_dvb_sdt_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . dvb_sdt_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . dvb_sdt_settings = Some (d) ; } } self } # [doc = "Set the field `dvb_tdt_settings`.\n"] pub fn set_dvb_tdt_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . dvb_tdt_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . dvb_tdt_settings = Some (d) ; } } self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsEl { absent_input_audio_behavior : core :: default :: Default :: default () , arib : core :: default :: Default :: default () , arib_captions_pid : core :: default :: Default :: default () , arib_captions_pid_control : core :: default :: Default :: default () , audio_buffer_model : core :: default :: Default :: default () , audio_frames_per_pes : core :: default :: Default :: default () , audio_pids : core :: default :: Default :: default () , audio_stream_type : core :: default :: Default :: default () , bitrate : core :: default :: Default :: default () , buffer_model : core :: default :: Default :: default () , cc_descriptor : core :: default :: Default :: default () , dvb_sub_pids : core :: default :: Default :: default () , dvb_teletext_pid : core :: default :: Default :: default () , ebif : core :: default :: Default :: default () , ebp_audio_interval : core :: default :: Default :: default () , ebp_lookahead_ms : core :: default :: Default :: default () , ebp_placement : core :: default :: Default :: default () , ecm_pid : core :: default :: Default :: default () , es_rate_in_pes : core :: default :: Default :: default () , etv_platform_pid : core :: default :: Default :: default () , etv_signal_pid : core :: default :: Default :: default () , fragment_time : core :: default :: Default :: default () , klv : core :: default :: Default :: default () , klv_data_pids : core :: default :: Default :: default () , nielsen_id3_behavior : core :: default :: Default :: default () , null_packet_bitrate : core :: default :: Default :: default () , pat_interval : core :: default :: Default :: default () , pcr_control : core :: default :: Default :: default () , pcr_period : core :: default :: Default :: default () , pcr_pid : core :: default :: Default :: default () , pmt_interval : core :: default :: Default :: default () , pmt_pid : core :: default :: Default :: default () , program_num : core :: default :: Default :: default () , rate_mode : core :: default :: Default :: default () , scte27_pids : core :: default :: Default :: default () , scte35_control : core :: default :: Default :: default () , scte35_pid : core :: default :: Default :: default () , segmentation_markers : core :: default :: Default :: default () , segmentation_style : core :: default :: Default :: default () , segmentation_time : core :: default :: Default :: default () , timed_metadata_behavior : core :: default :: Default :: default () , timed_metadata_pid : core :: default :: Default :: default () , transport_stream_id : core :: default :: Default :: default () , video_pid : core :: default :: Default :: default () , dvb_nit_settings : core :: default :: Default :: default () , dvb_sdt_settings : core :: default :: Default :: default () , dvb_tdt_settings : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `absent_input_audio_behavior` after provisioning.\n"] pub fn absent_input_audio_behavior (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.absent_input_audio_behavior" , self . base)) } # [doc = "Get a reference to the value of field `arib` after provisioning.\n"] pub fn arib (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.arib" , self . base)) } # [doc = "Get a reference to the value of field `arib_captions_pid` after provisioning.\n"] pub fn arib_captions_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.arib_captions_pid" , self . base)) } # [doc = "Get a reference to the value of field `arib_captions_pid_control` after provisioning.\n"] pub fn arib_captions_pid_control (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.arib_captions_pid_control" , self . base)) } # [doc = "Get a reference to the value of field `audio_buffer_model` after provisioning.\n"] pub fn audio_buffer_model (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.audio_buffer_model" , self . base)) } # [doc = "Get a reference to the value of field `audio_frames_per_pes` after provisioning.\n"] pub fn audio_frames_per_pes (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.audio_frames_per_pes" , self . base)) } # [doc = "Get a reference to the value of field `audio_pids` after provisioning.\n"] pub fn audio_pids (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.audio_pids" , self . base)) } # [doc = "Get a reference to the value of field `audio_stream_type` after provisioning.\n"] pub fn audio_stream_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.audio_stream_type" , self . base)) } # [doc = "Get a reference to the value of field `bitrate` after provisioning.\n"] pub fn bitrate (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bitrate" , self . base)) } # [doc = "Get a reference to the value of field `buffer_model` after provisioning.\n"] pub fn buffer_model (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.buffer_model" , self . base)) } # [doc = "Get a reference to the value of field `cc_descriptor` after provisioning.\n"] pub fn cc_descriptor (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.cc_descriptor" , self . base)) } # [doc = "Get a reference to the value of field `dvb_sub_pids` after provisioning.\n"] pub fn dvb_sub_pids (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.dvb_sub_pids" , self . base)) } # [doc = "Get a reference to the value of field `dvb_teletext_pid` after provisioning.\n"] pub fn dvb_teletext_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.dvb_teletext_pid" , self . base)) } # [doc = "Get a reference to the value of field `ebif` after provisioning.\n"] pub fn ebif (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ebif" , self . base)) } # [doc = "Get a reference to the value of field `ebp_audio_interval` after provisioning.\n"] pub fn ebp_audio_interval (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ebp_audio_interval" , self . base)) } # [doc = "Get a reference to the value of field `ebp_lookahead_ms` after provisioning.\n"] pub fn ebp_lookahead_ms (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ebp_lookahead_ms" , self . base)) } # [doc = "Get a reference to the value of field `ebp_placement` after provisioning.\n"] pub fn ebp_placement (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ebp_placement" , self . base)) } # [doc = "Get a reference to the value of field `ecm_pid` after provisioning.\n"] pub fn ecm_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ecm_pid" , self . base)) } # [doc = "Get a reference to the value of field `es_rate_in_pes` after provisioning.\n"] pub fn es_rate_in_pes (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.es_rate_in_pes" , self . base)) } # [doc = "Get a reference to the value of field `etv_platform_pid` after provisioning.\n"] pub fn etv_platform_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.etv_platform_pid" , self . base)) } # [doc = "Get a reference to the value of field `etv_signal_pid` after provisioning.\n"] pub fn etv_signal_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.etv_signal_pid" , self . base)) } # [doc = "Get a reference to the value of field `fragment_time` after provisioning.\n"] pub fn fragment_time (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.fragment_time" , self . base)) } # [doc = "Get a reference to the value of field `klv` after provisioning.\n"] pub fn klv (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.klv" , self . base)) } # [doc = "Get a reference to the value of field `klv_data_pids` after provisioning.\n"] pub fn klv_data_pids (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.klv_data_pids" , self . base)) } # [doc = "Get a reference to the value of field `nielsen_id3_behavior` after provisioning.\n"] pub fn nielsen_id3_behavior (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.nielsen_id3_behavior" , self . base)) } # [doc = "Get a reference to the value of field `null_packet_bitrate` after provisioning.\n"] pub fn null_packet_bitrate (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.null_packet_bitrate" , self . base)) } # [doc = "Get a reference to the value of field `pat_interval` after provisioning.\n"] pub fn pat_interval (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pat_interval" , self . base)) } # [doc = "Get a reference to the value of field `pcr_control` after provisioning.\n"] pub fn pcr_control (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pcr_control" , self . base)) } # [doc = "Get a reference to the value of field `pcr_period` after provisioning.\n"] pub fn pcr_period (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pcr_period" , self . base)) } # [doc = "Get a reference to the value of field `pcr_pid` after provisioning.\n"] pub fn pcr_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pcr_pid" , self . base)) } # [doc = "Get a reference to the value of field `pmt_interval` after provisioning.\n"] pub fn pmt_interval (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pmt_interval" , self . base)) } # [doc = "Get a reference to the value of field `pmt_pid` after provisioning.\n"] pub fn pmt_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pmt_pid" , self . base)) } # [doc = "Get a reference to the value of field `program_num` after provisioning.\n"] pub fn program_num (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.program_num" , self . base)) } # [doc = "Get a reference to the value of field `rate_mode` after provisioning.\n"] pub fn rate_mode (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.rate_mode" , self . base)) } # [doc = "Get a reference to the value of field `scte27_pids` after provisioning.\n"] pub fn scte27_pids (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.scte27_pids" , self . base)) } # [doc = "Get a reference to the value of field `scte35_control` after provisioning.\n"] pub fn scte35_control (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.scte35_control" , self . base)) } # [doc = "Get a reference to the value of field `scte35_pid` after provisioning.\n"] pub fn scte35_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.scte35_pid" , self . base)) } # [doc = "Get a reference to the value of field `segmentation_markers` after provisioning.\n"] pub fn segmentation_markers (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.segmentation_markers" , self . base)) } # [doc = "Get a reference to the value of field `segmentation_style` after provisioning.\n"] pub fn segmentation_style (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.segmentation_style" , self . base)) } # [doc = "Get a reference to the value of field `segmentation_time` after provisioning.\n"] pub fn segmentation_time (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.segmentation_time" , self . base)) } # [doc = "Get a reference to the value of field `timed_metadata_behavior` after provisioning.\n"] pub fn timed_metadata_behavior (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.timed_metadata_behavior" , self . base)) } # [doc = "Get a reference to the value of field `timed_metadata_pid` after provisioning.\n"] pub fn timed_metadata_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.timed_metadata_pid" , self . base)) } # [doc = "Get a reference to the value of field `transport_stream_id` after provisioning.\n"] pub fn transport_stream_id (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.transport_stream_id" , self . base)) } # [doc = "Get a reference to the value of field `video_pid` after provisioning.\n"] pub fn video_pid (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.video_pid" , self . base)) } # [doc = "Get a reference to the value of field `dvb_nit_settings` after provisioning.\n"] pub fn dvb_nit_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.dvb_nit_settings" , self . base)) } # [doc = "Get a reference to the value of field `dvb_sdt_settings` after provisioning.\n"] pub fn dvb_sdt_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.dvb_sdt_settings" , self . base)) } # [doc = "Get a reference to the value of field `dvb_tdt_settings` after provisioning.\n"] pub fn dvb_tdt_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.dvb_tdt_settings" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElDynamic { m2ts_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] m2ts_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsEl { # [doc = "Set the field `m2ts_settings`.\n"] pub fn set_m2ts_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . m2ts_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . m2ts_settings = Some (d) ; } } self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsEl { m2ts_settings : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `m2ts_settings` after provisioning.\n"] pub fn m2ts_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.m2ts_settings" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationEl
{
    destination_ref_id: PrimField<String>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationEl
{
    #[doc = ""]
    pub destination_ref_id: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationEl { destination_ref_id : self . destination_ref_id , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `destination_ref_id` after provisioning.\n"] pub fn destination_ref_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.destination_ref_id" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    column_depth: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_fec: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    row_length: Option<PrimField<f64>>,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsEl { # [doc = "Set the field `column_depth`.\n"] pub fn set_column_depth (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . column_depth = Some (v . into ()) ; self } # [doc = "Set the field `include_fec`.\n"] pub fn set_include_fec (mut self , v : impl Into < PrimField < String > >) -> Self { self . include_fec = Some (v . into ()) ; self } # [doc = "Set the field `row_length`.\n"] pub fn set_row_length (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . row_length = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsEl { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsEl { column_depth : core :: default :: Default :: default () , include_fec : core :: default :: Default :: default () , row_length : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `column_depth` after provisioning.\n"] pub fn column_depth (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.column_depth" , self . base)) } # [doc = "Get a reference to the value of field `include_fec` after provisioning.\n"] pub fn include_fec (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.include_fec" , self . base)) } # [doc = "Get a reference to the value of field `row_length` after provisioning.\n"] pub fn row_length (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.row_length" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDynamic { container_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsEl >> , destination : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationEl >> , fec_output_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] buffer_msec : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] container_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] destination : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationEl > > , # [serde (skip_serializing_if = "Option::is_none")] fec_output_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsEl {
    #[doc = "Set the field `buffer_msec`.\n"]
    pub fn set_buffer_msec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.buffer_msec = Some(v.into());
        self
    }
    #[doc = "Set the field `container_settings`.\n"]
    pub fn set_container_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.container_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.container_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `fec_output_settings`.\n"]
    pub fn set_fec_output_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fec_output_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fec_output_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsEl
{
    type O = BlockAssignable<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsEl
{}
impl
    BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsEl
{
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsEl
    {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsEl { buffer_msec : core :: default :: Default :: default () , container_settings : core :: default :: Default :: default () , destination : core :: default :: Default :: default () , fec_output_settings : core :: default :: Default :: default () , dynamic : Default :: default () , }
    }
}
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElRef { MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElRef { shared : shared , base : base . to_string () , } } }
impl
    MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `buffer_msec` after provisioning.\n"]
    pub fn buffer_msec(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.buffer_msec", self.base))
    }
    #[doc = "Get a reference to the value of field `container_settings` after provisioning.\n"]    pub fn container_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.container_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]    pub fn destination (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationElRef >{
        ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
    #[doc = "Get a reference to the value of field `fec_output_settings` after provisioning.\n"]    pub fn fec_output_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.fec_output_settings", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElDynamic { archive_output_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsEl >> , frame_capture_output_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsEl >> , hls_output_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsEl >> , media_package_output_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsEl >> , ms_smooth_output_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsEl >> , multiplex_output_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsEl >> , rtmp_output_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsEl >> , udp_output_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] archive_output_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] frame_capture_output_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] hls_output_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] media_package_output_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] ms_smooth_output_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] multiplex_output_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] rtmp_output_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] udp_output_settings : Option < Vec < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsEl {
    #[doc = "Set the field `archive_output_settings`.\n"]
    pub fn set_archive_output_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.archive_output_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.archive_output_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `frame_capture_output_settings`.\n"]
    pub fn set_frame_capture_output_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.frame_capture_output_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.frame_capture_output_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `hls_output_settings`.\n"]
    pub fn set_hls_output_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hls_output_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hls_output_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `media_package_output_settings`.\n"]
    pub fn set_media_package_output_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.media_package_output_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.media_package_output_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ms_smooth_output_settings`.\n"]
    pub fn set_ms_smooth_output_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ms_smooth_output_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ms_smooth_output_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `multiplex_output_settings`.\n"]
    pub fn set_multiplex_output_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.multiplex_output_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.multiplex_output_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rtmp_output_settings`.\n"]
    pub fn set_rtmp_output_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rtmp_output_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rtmp_output_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `udp_output_settings`.\n"]
    pub fn set_udp_output_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.udp_output_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.udp_output_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsEl {
    type O =
        BlockAssignable<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsEl {}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsEl {
            archive_output_settings: core::default::Default::default(),
            frame_capture_output_settings: core::default::Default::default(),
            hls_output_settings: core::default::Default::default(),
            media_package_output_settings: core::default::Default::default(),
            ms_smooth_output_settings: core::default::Default::default(),
            multiplex_output_settings: core::default::Default::default(),
            rtmp_output_settings: core::default::Default::default(),
            udp_output_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `archive_output_settings` after provisioning.\n"]    pub fn archive_output_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.archive_output_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `frame_capture_output_settings` after provisioning.\n"]    pub fn frame_capture_output_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.frame_capture_output_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hls_output_settings` after provisioning.\n"]    pub fn hls_output_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.hls_output_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `media_package_output_settings` after provisioning.\n"]    pub fn media_package_output_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.media_package_output_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ms_smooth_output_settings` after provisioning.\n"]    pub fn ms_smooth_output_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.ms_smooth_output_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `multiplex_output_settings` after provisioning.\n"]    pub fn multiplex_output_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.multiplex_output_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rtmp_output_settings` after provisioning.\n"]    pub fn rtmp_output_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.rtmp_output_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `udp_output_settings` after provisioning.\n"]    pub fn udp_output_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.udp_output_settings", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElDynamic {
    output_settings: Option<
        DynamicBlock<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsEl>,
    >,
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_description_names: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_description_names: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_description_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_settings:
        Option<Vec<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsEl>>,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElDynamic,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsEl {
    #[doc = "Set the field `audio_description_names`.\n"]
    pub fn set_audio_description_names(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.audio_description_names = Some(v.into());
        self
    }
    #[doc = "Set the field `caption_description_names`.\n"]
    pub fn set_caption_description_names(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.caption_description_names = Some(v.into());
        self
    }
    #[doc = "Set the field `output_name`.\n"]
    pub fn set_output_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_name = Some(v.into());
        self
    }
    #[doc = "Set the field `video_description_name`.\n"]
    pub fn set_video_description_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.video_description_name = Some(v.into());
        self
    }
    #[doc = "Set the field `output_settings`.\n"]
    pub fn set_output_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.output_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.output_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsEl {
    type O = BlockAssignable<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsEl {}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsEl {
            audio_description_names: core::default::Default::default(),
            caption_description_names: core::default::Default::default(),
            output_name: core::default::Default::default(),
            video_description_name: core::default::Default::default(),
            output_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `audio_description_names` after provisioning.\n"]
    pub fn audio_description_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.audio_description_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `caption_description_names` after provisioning.\n"]
    pub fn caption_description_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.caption_description_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `output_name` after provisioning.\n"]
    pub fn output_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_name", self.base))
    }
    #[doc = "Get a reference to the value of field `video_description_name` after provisioning.\n"]
    pub fn video_description_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.video_description_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `output_settings` after provisioning.\n"]
    pub fn output_settings(
        &self,
    ) -> ListRef<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_settings", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElDynamic {
    output_group_settings:
        Option<DynamicBlock<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsEl>>,
    outputs: Option<DynamicBlock<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsEl>>,
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_group_settings:
        Option<Vec<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outputs: Option<Vec<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsEl>>,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElDynamic,
}
impl MedialiveChannelEncoderSettingsElOutputGroupsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `output_group_settings`.\n"]
    pub fn set_output_group_settings(
        mut self,
        v: impl Into<
            BlockAssignable<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.output_group_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.output_group_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `outputs`.\n"]
    pub fn set_outputs(
        mut self,
        v: impl Into<BlockAssignable<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.outputs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.outputs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsEl {
    type O = BlockAssignable<MedialiveChannelEncoderSettingsElOutputGroupsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsEl {}
impl BuildMedialiveChannelEncoderSettingsElOutputGroupsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElOutputGroupsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsEl {
            name: core::default::Default::default(),
            output_group_settings: core::default::Default::default(),
            outputs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElOutputGroupsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElOutputGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `output_group_settings` after provisioning.\n"]
    pub fn output_group_settings(
        &self,
    ) -> ListRef<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_group_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `outputs` after provisioning.\n"]
    pub fn outputs(&self) -> ListRef<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.outputs", self.base))
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElTimecodeConfigEl {
    source: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_threshold: Option<PrimField<f64>>,
}
impl MedialiveChannelEncoderSettingsElTimecodeConfigEl {
    #[doc = "Set the field `sync_threshold`.\n"]
    pub fn set_sync_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.sync_threshold = Some(v.into());
        self
    }
}
impl ToListMappable for MedialiveChannelEncoderSettingsElTimecodeConfigEl {
    type O = BlockAssignable<MedialiveChannelEncoderSettingsElTimecodeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElTimecodeConfigEl {
    #[doc = ""]
    pub source: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElTimecodeConfigEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElTimecodeConfigEl {
        MedialiveChannelEncoderSettingsElTimecodeConfigEl {
            source: self.source,
            sync_threshold: core::default::Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElTimecodeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElTimecodeConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElTimecodeConfigElRef {
        MedialiveChannelEncoderSettingsElTimecodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElTimecodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
    #[doc = "Get a reference to the value of field `sync_threshold` after provisioning.\n"]
    pub fn sync_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sync_threshold", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    capture_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capture_interval_units: Option<PrimField<String>>,
}
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsEl {
    #[doc = "Set the field `capture_interval`.\n"]
    pub fn set_capture_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.capture_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `capture_interval_units`.\n"]
    pub fn set_capture_interval_units(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.capture_interval_units = Some(v.into());
        self
    }
}
impl ToListMappable
    for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsEl
{
    type O = BlockAssignable<
        MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsEl
{}
impl
    BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsEl
{
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsEl
    {
        MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsEl {
            capture_interval: core::default::Default::default(),
            capture_interval_units: core::default::Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsElRef
    {
        MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsElRef { shared : shared , base : base . to_string () , }
    }
}
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `capture_interval` after provisioning.\n"]
    pub fn capture_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capture_interval", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `capture_interval_units` after provisioning.\n"]
    pub fn capture_interval_units(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capture_interval_units", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    post_filter_sharpening: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strength: Option<PrimField<String>>,
}
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsEl { # [doc = "Set the field `post_filter_sharpening`.\n"] pub fn set_post_filter_sharpening (mut self , v : impl Into < PrimField < String > >) -> Self { self . post_filter_sharpening = Some (v . into ()) ; self } # [doc = "Set the field `strength`.\n"] pub fn set_strength (mut self , v : impl Into < PrimField < String > >) -> Self { self . strength = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsEl { MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsEl { post_filter_sharpening : core :: default :: Default :: default () , strength : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsElRef { MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `post_filter_sharpening` after provisioning.\n"] pub fn post_filter_sharpening (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.post_filter_sharpening" , self . base)) } # [doc = "Get a reference to the value of field `strength` after provisioning.\n"] pub fn strength (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.strength" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElDynamic { temporal_filter_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] temporal_filter_settings : Option < Vec < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsEl { # [doc = "Set the field `temporal_filter_settings`.\n"] pub fn set_temporal_filter_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . temporal_filter_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . temporal_filter_settings = Some (d) ; } } self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsEl { MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsEl { temporal_filter_settings : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElRef { MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `temporal_filter_settings` after provisioning.\n"] pub fn temporal_filter_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.temporal_filter_settings" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElDynamic { filter_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsEl { # [serde (skip_serializing_if = "Option::is_none")] adaptive_quantization : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] afd_signaling : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] bitrate : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] buf_fill_pct : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] buf_size : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] color_metadata : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] entropy_encoding : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] fixed_afd : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] flicker_aq : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] force_field_pictures : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] framerate_control : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] framerate_denominator : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] framerate_numerator : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] gop_b_reference : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] gop_closed_cadence : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] gop_num_b_frames : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] gop_size : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] gop_size_units : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] level : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] look_ahead_rate_control : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] max_bitrate : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] min_i_interval : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] num_ref_frames : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] par_control : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] par_denominator : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] par_numerator : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] profile : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] quality_level : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] qvbr_quality_level : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] rate_control_mode : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] scan_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] scene_change_detect : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] slices : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] softness : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] spatial_aq : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] subgop_length : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] syntax : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] temporal_aq : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] timecode_insertion : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] filter_settings : Option < Vec < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsEl {
    #[doc = "Set the field `adaptive_quantization`.\n"]
    pub fn set_adaptive_quantization(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.adaptive_quantization = Some(v.into());
        self
    }
    #[doc = "Set the field `afd_signaling`.\n"]
    pub fn set_afd_signaling(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.afd_signaling = Some(v.into());
        self
    }
    #[doc = "Set the field `bitrate`.\n"]
    pub fn set_bitrate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.bitrate = Some(v.into());
        self
    }
    #[doc = "Set the field `buf_fill_pct`.\n"]
    pub fn set_buf_fill_pct(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.buf_fill_pct = Some(v.into());
        self
    }
    #[doc = "Set the field `buf_size`.\n"]
    pub fn set_buf_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.buf_size = Some(v.into());
        self
    }
    #[doc = "Set the field `color_metadata`.\n"]
    pub fn set_color_metadata(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.color_metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `entropy_encoding`.\n"]
    pub fn set_entropy_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.entropy_encoding = Some(v.into());
        self
    }
    #[doc = "Set the field `fixed_afd`.\n"]
    pub fn set_fixed_afd(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fixed_afd = Some(v.into());
        self
    }
    #[doc = "Set the field `flicker_aq`.\n"]
    pub fn set_flicker_aq(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.flicker_aq = Some(v.into());
        self
    }
    #[doc = "Set the field `force_field_pictures`.\n"]
    pub fn set_force_field_pictures(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.force_field_pictures = Some(v.into());
        self
    }
    #[doc = "Set the field `framerate_control`.\n"]
    pub fn set_framerate_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.framerate_control = Some(v.into());
        self
    }
    #[doc = "Set the field `framerate_denominator`.\n"]
    pub fn set_framerate_denominator(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.framerate_denominator = Some(v.into());
        self
    }
    #[doc = "Set the field `framerate_numerator`.\n"]
    pub fn set_framerate_numerator(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.framerate_numerator = Some(v.into());
        self
    }
    #[doc = "Set the field `gop_b_reference`.\n"]
    pub fn set_gop_b_reference(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gop_b_reference = Some(v.into());
        self
    }
    #[doc = "Set the field `gop_closed_cadence`.\n"]
    pub fn set_gop_closed_cadence(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.gop_closed_cadence = Some(v.into());
        self
    }
    #[doc = "Set the field `gop_num_b_frames`.\n"]
    pub fn set_gop_num_b_frames(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.gop_num_b_frames = Some(v.into());
        self
    }
    #[doc = "Set the field `gop_size`.\n"]
    pub fn set_gop_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.gop_size = Some(v.into());
        self
    }
    #[doc = "Set the field `gop_size_units`.\n"]
    pub fn set_gop_size_units(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gop_size_units = Some(v.into());
        self
    }
    #[doc = "Set the field `level`.\n"]
    pub fn set_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.level = Some(v.into());
        self
    }
    #[doc = "Set the field `look_ahead_rate_control`.\n"]
    pub fn set_look_ahead_rate_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.look_ahead_rate_control = Some(v.into());
        self
    }
    #[doc = "Set the field `max_bitrate`.\n"]
    pub fn set_max_bitrate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_bitrate = Some(v.into());
        self
    }
    #[doc = "Set the field `min_i_interval`.\n"]
    pub fn set_min_i_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_i_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `num_ref_frames`.\n"]
    pub fn set_num_ref_frames(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_ref_frames = Some(v.into());
        self
    }
    #[doc = "Set the field `par_control`.\n"]
    pub fn set_par_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.par_control = Some(v.into());
        self
    }
    #[doc = "Set the field `par_denominator`.\n"]
    pub fn set_par_denominator(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.par_denominator = Some(v.into());
        self
    }
    #[doc = "Set the field `par_numerator`.\n"]
    pub fn set_par_numerator(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.par_numerator = Some(v.into());
        self
    }
    #[doc = "Set the field `profile`.\n"]
    pub fn set_profile(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.profile = Some(v.into());
        self
    }
    #[doc = "Set the field `quality_level`.\n"]
    pub fn set_quality_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.quality_level = Some(v.into());
        self
    }
    #[doc = "Set the field `qvbr_quality_level`.\n"]
    pub fn set_qvbr_quality_level(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.qvbr_quality_level = Some(v.into());
        self
    }
    #[doc = "Set the field `rate_control_mode`.\n"]
    pub fn set_rate_control_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rate_control_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `scan_type`.\n"]
    pub fn set_scan_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scan_type = Some(v.into());
        self
    }
    #[doc = "Set the field `scene_change_detect`.\n"]
    pub fn set_scene_change_detect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scene_change_detect = Some(v.into());
        self
    }
    #[doc = "Set the field `slices`.\n"]
    pub fn set_slices(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.slices = Some(v.into());
        self
    }
    #[doc = "Set the field `softness`.\n"]
    pub fn set_softness(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.softness = Some(v.into());
        self
    }
    #[doc = "Set the field `spatial_aq`.\n"]
    pub fn set_spatial_aq(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.spatial_aq = Some(v.into());
        self
    }
    #[doc = "Set the field `subgop_length`.\n"]
    pub fn set_subgop_length(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subgop_length = Some(v.into());
        self
    }
    #[doc = "Set the field `syntax`.\n"]
    pub fn set_syntax(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.syntax = Some(v.into());
        self
    }
    #[doc = "Set the field `temporal_aq`.\n"]
    pub fn set_temporal_aq(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.temporal_aq = Some(v.into());
        self
    }
    #[doc = "Set the field `timecode_insertion`.\n"]
    pub fn set_timecode_insertion(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timecode_insertion = Some(v.into());
        self
    }
    #[doc = "Set the field `filter_settings`.\n"]
    pub fn set_filter_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsEl
{
    type O = BlockAssignable<
        MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsEl {
}
impl BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsEl {
        MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsEl {
            adaptive_quantization: core::default::Default::default(),
            afd_signaling: core::default::Default::default(),
            bitrate: core::default::Default::default(),
            buf_fill_pct: core::default::Default::default(),
            buf_size: core::default::Default::default(),
            color_metadata: core::default::Default::default(),
            entropy_encoding: core::default::Default::default(),
            fixed_afd: core::default::Default::default(),
            flicker_aq: core::default::Default::default(),
            force_field_pictures: core::default::Default::default(),
            framerate_control: core::default::Default::default(),
            framerate_denominator: core::default::Default::default(),
            framerate_numerator: core::default::Default::default(),
            gop_b_reference: core::default::Default::default(),
            gop_closed_cadence: core::default::Default::default(),
            gop_num_b_frames: core::default::Default::default(),
            gop_size: core::default::Default::default(),
            gop_size_units: core::default::Default::default(),
            level: core::default::Default::default(),
            look_ahead_rate_control: core::default::Default::default(),
            max_bitrate: core::default::Default::default(),
            min_i_interval: core::default::Default::default(),
            num_ref_frames: core::default::Default::default(),
            par_control: core::default::Default::default(),
            par_denominator: core::default::Default::default(),
            par_numerator: core::default::Default::default(),
            profile: core::default::Default::default(),
            quality_level: core::default::Default::default(),
            qvbr_quality_level: core::default::Default::default(),
            rate_control_mode: core::default::Default::default(),
            scan_type: core::default::Default::default(),
            scene_change_detect: core::default::Default::default(),
            slices: core::default::Default::default(),
            softness: core::default::Default::default(),
            spatial_aq: core::default::Default::default(),
            subgop_length: core::default::Default::default(),
            syntax: core::default::Default::default(),
            temporal_aq: core::default::Default::default(),
            timecode_insertion: core::default::Default::default(),
            filter_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElRef {
        MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `adaptive_quantization` after provisioning.\n"]
    pub fn adaptive_quantization(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.adaptive_quantization", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `afd_signaling` after provisioning.\n"]
    pub fn afd_signaling(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.afd_signaling", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bitrate` after provisioning.\n"]
    pub fn bitrate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.bitrate", self.base))
    }
    #[doc = "Get a reference to the value of field `buf_fill_pct` after provisioning.\n"]
    pub fn buf_fill_pct(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.buf_fill_pct", self.base))
    }
    #[doc = "Get a reference to the value of field `buf_size` after provisioning.\n"]
    pub fn buf_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.buf_size", self.base))
    }
    #[doc = "Get a reference to the value of field `color_metadata` after provisioning.\n"]
    pub fn color_metadata(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.color_metadata", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `entropy_encoding` after provisioning.\n"]
    pub fn entropy_encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entropy_encoding", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fixed_afd` after provisioning.\n"]
    pub fn fixed_afd(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fixed_afd", self.base))
    }
    #[doc = "Get a reference to the value of field `flicker_aq` after provisioning.\n"]
    pub fn flicker_aq(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.flicker_aq", self.base))
    }
    #[doc = "Get a reference to the value of field `force_field_pictures` after provisioning.\n"]
    pub fn force_field_pictures(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_field_pictures", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `framerate_control` after provisioning.\n"]
    pub fn framerate_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.framerate_control", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `framerate_denominator` after provisioning.\n"]
    pub fn framerate_denominator(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.framerate_denominator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `framerate_numerator` after provisioning.\n"]
    pub fn framerate_numerator(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.framerate_numerator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gop_b_reference` after provisioning.\n"]
    pub fn gop_b_reference(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gop_b_reference", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gop_closed_cadence` after provisioning.\n"]
    pub fn gop_closed_cadence(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gop_closed_cadence", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gop_num_b_frames` after provisioning.\n"]
    pub fn gop_num_b_frames(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gop_num_b_frames", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gop_size` after provisioning.\n"]
    pub fn gop_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.gop_size", self.base))
    }
    #[doc = "Get a reference to the value of field `gop_size_units` after provisioning.\n"]
    pub fn gop_size_units(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gop_size_units", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `level` after provisioning.\n"]
    pub fn level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.level", self.base))
    }
    #[doc = "Get a reference to the value of field `look_ahead_rate_control` after provisioning.\n"]
    pub fn look_ahead_rate_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.look_ahead_rate_control", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_bitrate` after provisioning.\n"]
    pub fn max_bitrate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_bitrate", self.base))
    }
    #[doc = "Get a reference to the value of field `min_i_interval` after provisioning.\n"]
    pub fn min_i_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_i_interval", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `num_ref_frames` after provisioning.\n"]
    pub fn num_ref_frames(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.num_ref_frames", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `par_control` after provisioning.\n"]
    pub fn par_control(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.par_control", self.base))
    }
    #[doc = "Get a reference to the value of field `par_denominator` after provisioning.\n"]
    pub fn par_denominator(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.par_denominator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `par_numerator` after provisioning.\n"]
    pub fn par_numerator(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.par_numerator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `profile` after provisioning.\n"]
    pub fn profile(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.profile", self.base))
    }
    #[doc = "Get a reference to the value of field `quality_level` after provisioning.\n"]
    pub fn quality_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quality_level", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `qvbr_quality_level` after provisioning.\n"]
    pub fn qvbr_quality_level(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.qvbr_quality_level", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rate_control_mode` after provisioning.\n"]
    pub fn rate_control_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rate_control_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scan_type` after provisioning.\n"]
    pub fn scan_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scan_type", self.base))
    }
    #[doc = "Get a reference to the value of field `scene_change_detect` after provisioning.\n"]
    pub fn scene_change_detect(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scene_change_detect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `slices` after provisioning.\n"]
    pub fn slices(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.slices", self.base))
    }
    #[doc = "Get a reference to the value of field `softness` after provisioning.\n"]
    pub fn softness(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.softness", self.base))
    }
    #[doc = "Get a reference to the value of field `spatial_aq` after provisioning.\n"]
    pub fn spatial_aq(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.spatial_aq", self.base))
    }
    #[doc = "Get a reference to the value of field `subgop_length` after provisioning.\n"]
    pub fn subgop_length(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subgop_length", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `syntax` after provisioning.\n"]
    pub fn syntax(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.syntax", self.base))
    }
    #[doc = "Get a reference to the value of field `temporal_aq` after provisioning.\n"]
    pub fn temporal_aq(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.temporal_aq", self.base))
    }
    #[doc = "Get a reference to the value of field `timecode_insertion` after provisioning.\n"]
    pub fn timecode_insertion(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timecode_insertion", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `filter_settings` after provisioning.\n"]    pub fn filter_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElColorSpacePassthroughSettingsEl
{}
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElColorSpacePassthroughSettingsEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElColorSpacePassthroughSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElColorSpacePassthroughSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElColorSpacePassthroughSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElColorSpacePassthroughSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElColorSpacePassthroughSettingsEl { MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElColorSpacePassthroughSettingsEl { } } }
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElColorSpacePassthroughSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElColorSpacePassthroughSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElColorSpacePassthroughSettingsElRef { MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElColorSpacePassthroughSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElColorSpacePassthroughSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElDolbyVision81SettingsEl
{}
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElDolbyVision81SettingsEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElDolbyVision81SettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElDolbyVision81SettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElDolbyVision81SettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElDolbyVision81SettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElDolbyVision81SettingsEl { MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElDolbyVision81SettingsEl { } } }
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElDolbyVision81SettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElDolbyVision81SettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElDolbyVision81SettingsElRef { MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElDolbyVision81SettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElDolbyVision81SettingsElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElHdr10SettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max_cll: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_fall: Option<PrimField<f64>>,
}
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElHdr10SettingsEl { # [doc = "Set the field `max_cll`.\n"] pub fn set_max_cll (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . max_cll = Some (v . into ()) ; self } # [doc = "Set the field `max_fall`.\n"] pub fn set_max_fall (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . max_fall = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElHdr10SettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElHdr10SettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElHdr10SettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElHdr10SettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElHdr10SettingsEl { MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElHdr10SettingsEl { max_cll : core :: default :: Default :: default () , max_fall : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElHdr10SettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElHdr10SettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElHdr10SettingsElRef { MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElHdr10SettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElHdr10SettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `max_cll` after provisioning.\n"] pub fn max_cll (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max_cll" , self . base)) } # [doc = "Get a reference to the value of field `max_fall` after provisioning.\n"] pub fn max_fall (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max_fall" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec601SettingsEl
{}
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec601SettingsEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec601SettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec601SettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec601SettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec601SettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec601SettingsEl { MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec601SettingsEl { } } }
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec601SettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec601SettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec601SettingsElRef { MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec601SettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec601SettingsElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec709SettingsEl
{}
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec709SettingsEl { }
impl ToListMappable for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec709SettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec709SettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec709SettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec709SettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec709SettingsEl { MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec709SettingsEl { } } }
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec709SettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec709SettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec709SettingsElRef { MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec709SettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec709SettingsElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElDynamic { color_space_passthrough_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElColorSpacePassthroughSettingsEl >> , dolby_vision81_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElDolbyVision81SettingsEl >> , hdr10_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElHdr10SettingsEl >> , rec601_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec601SettingsEl >> , rec709_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec709SettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] color_space_passthrough_settings : Option < Vec < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElColorSpacePassthroughSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] dolby_vision81_settings : Option < Vec < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElDolbyVision81SettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] hdr10_settings : Option < Vec < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElHdr10SettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] rec601_settings : Option < Vec < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec601SettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] rec709_settings : Option < Vec < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec709SettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsEl { # [doc = "Set the field `color_space_passthrough_settings`.\n"] pub fn set_color_space_passthrough_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElColorSpacePassthroughSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . color_space_passthrough_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . color_space_passthrough_settings = Some (d) ; } } self } # [doc = "Set the field `dolby_vision81_settings`.\n"] pub fn set_dolby_vision81_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElDolbyVision81SettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . dolby_vision81_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . dolby_vision81_settings = Some (d) ; } } self } # [doc = "Set the field `hdr10_settings`.\n"] pub fn set_hdr10_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElHdr10SettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . hdr10_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . hdr10_settings = Some (d) ; } } self } # [doc = "Set the field `rec601_settings`.\n"] pub fn set_rec601_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec601SettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . rec601_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . rec601_settings = Some (d) ; } } self } # [doc = "Set the field `rec709_settings`.\n"] pub fn set_rec709_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec709SettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . rec709_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . rec709_settings = Some (d) ; } } self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsEl { MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsEl { color_space_passthrough_settings : core :: default :: Default :: default () , dolby_vision81_settings : core :: default :: Default :: default () , hdr10_settings : core :: default :: Default :: default () , rec601_settings : core :: default :: Default :: default () , rec709_settings : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRef { MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `color_space_passthrough_settings` after provisioning.\n"] pub fn color_space_passthrough_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElColorSpacePassthroughSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.color_space_passthrough_settings" , self . base)) } # [doc = "Get a reference to the value of field `dolby_vision81_settings` after provisioning.\n"] pub fn dolby_vision81_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElDolbyVision81SettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.dolby_vision81_settings" , self . base)) } # [doc = "Get a reference to the value of field `hdr10_settings` after provisioning.\n"] pub fn hdr10_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElHdr10SettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.hdr10_settings" , self . base)) } # [doc = "Get a reference to the value of field `rec601_settings` after provisioning.\n"] pub fn rec601_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec601SettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.rec601_settings" , self . base)) } # [doc = "Get a reference to the value of field `rec709_settings` after provisioning.\n"] pub fn rec709_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRec709SettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.rec709_settings" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsElTemporalFilterSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    post_filter_sharpening: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strength: Option<PrimField<String>>,
}
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsElTemporalFilterSettingsEl { # [doc = "Set the field `post_filter_sharpening`.\n"] pub fn set_post_filter_sharpening (mut self , v : impl Into < PrimField < String > >) -> Self { self . post_filter_sharpening = Some (v . into ()) ; self } # [doc = "Set the field `strength`.\n"] pub fn set_strength (mut self , v : impl Into < PrimField < String > >) -> Self { self . strength = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsElTemporalFilterSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsElTemporalFilterSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsElTemporalFilterSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsElTemporalFilterSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsElTemporalFilterSettingsEl { MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsElTemporalFilterSettingsEl { post_filter_sharpening : core :: default :: Default :: default () , strength : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsElTemporalFilterSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsElTemporalFilterSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsElTemporalFilterSettingsElRef { MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsElTemporalFilterSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsElTemporalFilterSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `post_filter_sharpening` after provisioning.\n"] pub fn post_filter_sharpening (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.post_filter_sharpening" , self . base)) } # [doc = "Get a reference to the value of field `strength` after provisioning.\n"] pub fn strength (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.strength" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsElDynamic { temporal_filter_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsElTemporalFilterSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] temporal_filter_settings : Option < Vec < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsElTemporalFilterSettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsEl { # [doc = "Set the field `temporal_filter_settings`.\n"] pub fn set_temporal_filter_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsElTemporalFilterSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . temporal_filter_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . temporal_filter_settings = Some (d) ; } } self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsEl { MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsEl { temporal_filter_settings : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsElRef { MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `temporal_filter_settings` after provisioning.\n"] pub fn temporal_filter_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsElTemporalFilterSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.temporal_filter_settings" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElTimecodeBurninSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timecode_burnin_font_size: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timecode_burnin_position: Option<PrimField<String>>,
}
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElTimecodeBurninSettingsEl { # [doc = "Set the field `prefix`.\n"] pub fn set_prefix (mut self , v : impl Into < PrimField < String > >) -> Self { self . prefix = Some (v . into ()) ; self } # [doc = "Set the field `timecode_burnin_font_size`.\n"] pub fn set_timecode_burnin_font_size (mut self , v : impl Into < PrimField < String > >) -> Self { self . timecode_burnin_font_size = Some (v . into ()) ; self } # [doc = "Set the field `timecode_burnin_position`.\n"] pub fn set_timecode_burnin_position (mut self , v : impl Into < PrimField < String > >) -> Self { self . timecode_burnin_position = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElTimecodeBurninSettingsEl { type O = BlockAssignable < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElTimecodeBurninSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElTimecodeBurninSettingsEl
{}
impl BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElTimecodeBurninSettingsEl { pub fn build (self) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElTimecodeBurninSettingsEl { MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElTimecodeBurninSettingsEl { prefix : core :: default :: Default :: default () , timecode_burnin_font_size : core :: default :: Default :: default () , timecode_burnin_position : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElTimecodeBurninSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElTimecodeBurninSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElTimecodeBurninSettingsElRef { MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElTimecodeBurninSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElTimecodeBurninSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `prefix` after provisioning.\n"] pub fn prefix (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.prefix" , self . base)) } # [doc = "Get a reference to the value of field `timecode_burnin_font_size` after provisioning.\n"] pub fn timecode_burnin_font_size (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.timecode_burnin_font_size" , self . base)) } # [doc = "Get a reference to the value of field `timecode_burnin_position` after provisioning.\n"] pub fn timecode_burnin_position (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.timecode_burnin_position" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElDynamic { color_space_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsEl >> , filter_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsEl >> , timecode_burnin_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElTimecodeBurninSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsEl { # [serde (skip_serializing_if = "Option::is_none")] adaptive_quantization : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] afd_signaling : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] alternative_transfer_function : Option < PrimField < String > > , bitrate : PrimField < f64 > , # [serde (skip_serializing_if = "Option::is_none")] buf_size : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] color_metadata : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] fixed_afd : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] flicker_aq : Option < PrimField < String > > , framerate_denominator : PrimField < f64 > , framerate_numerator : PrimField < f64 > , # [serde (skip_serializing_if = "Option::is_none")] gop_closed_cadence : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] gop_size : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] gop_size_units : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] level : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] look_ahead_rate_control : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] max_bitrate : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] min_i_interval : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] min_qp : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] mv_over_picture_boundaries : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] mv_temporal_predictor : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] par_denominator : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] par_numerator : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] profile : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] qvbr_quality_level : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] rate_control_mode : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] scan_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] scene_change_detect : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] slices : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] tier : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] tile_height : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] tile_padding : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] tile_width : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] timecode_insertion : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] treeblock_size : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] color_space_settings : Option < Vec < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] filter_settings : Option < Vec < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] timecode_burnin_settings : Option < Vec < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElTimecodeBurninSettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsEl {
    #[doc = "Set the field `adaptive_quantization`.\n"]
    pub fn set_adaptive_quantization(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.adaptive_quantization = Some(v.into());
        self
    }
    #[doc = "Set the field `afd_signaling`.\n"]
    pub fn set_afd_signaling(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.afd_signaling = Some(v.into());
        self
    }
    #[doc = "Set the field `alternative_transfer_function`.\n"]
    pub fn set_alternative_transfer_function(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.alternative_transfer_function = Some(v.into());
        self
    }
    #[doc = "Set the field `buf_size`.\n"]
    pub fn set_buf_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.buf_size = Some(v.into());
        self
    }
    #[doc = "Set the field `color_metadata`.\n"]
    pub fn set_color_metadata(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.color_metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `fixed_afd`.\n"]
    pub fn set_fixed_afd(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fixed_afd = Some(v.into());
        self
    }
    #[doc = "Set the field `flicker_aq`.\n"]
    pub fn set_flicker_aq(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.flicker_aq = Some(v.into());
        self
    }
    #[doc = "Set the field `gop_closed_cadence`.\n"]
    pub fn set_gop_closed_cadence(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.gop_closed_cadence = Some(v.into());
        self
    }
    #[doc = "Set the field `gop_size`.\n"]
    pub fn set_gop_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.gop_size = Some(v.into());
        self
    }
    #[doc = "Set the field `gop_size_units`.\n"]
    pub fn set_gop_size_units(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gop_size_units = Some(v.into());
        self
    }
    #[doc = "Set the field `level`.\n"]
    pub fn set_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.level = Some(v.into());
        self
    }
    #[doc = "Set the field `look_ahead_rate_control`.\n"]
    pub fn set_look_ahead_rate_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.look_ahead_rate_control = Some(v.into());
        self
    }
    #[doc = "Set the field `max_bitrate`.\n"]
    pub fn set_max_bitrate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_bitrate = Some(v.into());
        self
    }
    #[doc = "Set the field `min_i_interval`.\n"]
    pub fn set_min_i_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_i_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `min_qp`.\n"]
    pub fn set_min_qp(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_qp = Some(v.into());
        self
    }
    #[doc = "Set the field `mv_over_picture_boundaries`.\n"]
    pub fn set_mv_over_picture_boundaries(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mv_over_picture_boundaries = Some(v.into());
        self
    }
    #[doc = "Set the field `mv_temporal_predictor`.\n"]
    pub fn set_mv_temporal_predictor(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mv_temporal_predictor = Some(v.into());
        self
    }
    #[doc = "Set the field `par_denominator`.\n"]
    pub fn set_par_denominator(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.par_denominator = Some(v.into());
        self
    }
    #[doc = "Set the field `par_numerator`.\n"]
    pub fn set_par_numerator(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.par_numerator = Some(v.into());
        self
    }
    #[doc = "Set the field `profile`.\n"]
    pub fn set_profile(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.profile = Some(v.into());
        self
    }
    #[doc = "Set the field `qvbr_quality_level`.\n"]
    pub fn set_qvbr_quality_level(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.qvbr_quality_level = Some(v.into());
        self
    }
    #[doc = "Set the field `rate_control_mode`.\n"]
    pub fn set_rate_control_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rate_control_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `scan_type`.\n"]
    pub fn set_scan_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scan_type = Some(v.into());
        self
    }
    #[doc = "Set the field `scene_change_detect`.\n"]
    pub fn set_scene_change_detect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scene_change_detect = Some(v.into());
        self
    }
    #[doc = "Set the field `slices`.\n"]
    pub fn set_slices(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.slices = Some(v.into());
        self
    }
    #[doc = "Set the field `tier`.\n"]
    pub fn set_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tier = Some(v.into());
        self
    }
    #[doc = "Set the field `tile_height`.\n"]
    pub fn set_tile_height(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.tile_height = Some(v.into());
        self
    }
    #[doc = "Set the field `tile_padding`.\n"]
    pub fn set_tile_padding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tile_padding = Some(v.into());
        self
    }
    #[doc = "Set the field `tile_width`.\n"]
    pub fn set_tile_width(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.tile_width = Some(v.into());
        self
    }
    #[doc = "Set the field `timecode_insertion`.\n"]
    pub fn set_timecode_insertion(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timecode_insertion = Some(v.into());
        self
    }
    #[doc = "Set the field `treeblock_size`.\n"]
    pub fn set_treeblock_size(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.treeblock_size = Some(v.into());
        self
    }
    #[doc = "Set the field `color_space_settings`.\n"]
    pub fn set_color_space_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.color_space_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.color_space_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `filter_settings`.\n"]
    pub fn set_filter_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timecode_burnin_settings`.\n"]
    pub fn set_timecode_burnin_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElTimecodeBurninSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.timecode_burnin_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.timecode_burnin_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsEl
{
    type O = BlockAssignable<
        MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsEl {
    #[doc = ""]
    pub bitrate: PrimField<f64>,
    #[doc = ""]
    pub framerate_denominator: PrimField<f64>,
    #[doc = ""]
    pub framerate_numerator: PrimField<f64>,
}
impl BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsEl {
        MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsEl {
            adaptive_quantization: core::default::Default::default(),
            afd_signaling: core::default::Default::default(),
            alternative_transfer_function: core::default::Default::default(),
            bitrate: self.bitrate,
            buf_size: core::default::Default::default(),
            color_metadata: core::default::Default::default(),
            fixed_afd: core::default::Default::default(),
            flicker_aq: core::default::Default::default(),
            framerate_denominator: self.framerate_denominator,
            framerate_numerator: self.framerate_numerator,
            gop_closed_cadence: core::default::Default::default(),
            gop_size: core::default::Default::default(),
            gop_size_units: core::default::Default::default(),
            level: core::default::Default::default(),
            look_ahead_rate_control: core::default::Default::default(),
            max_bitrate: core::default::Default::default(),
            min_i_interval: core::default::Default::default(),
            min_qp: core::default::Default::default(),
            mv_over_picture_boundaries: core::default::Default::default(),
            mv_temporal_predictor: core::default::Default::default(),
            par_denominator: core::default::Default::default(),
            par_numerator: core::default::Default::default(),
            profile: core::default::Default::default(),
            qvbr_quality_level: core::default::Default::default(),
            rate_control_mode: core::default::Default::default(),
            scan_type: core::default::Default::default(),
            scene_change_detect: core::default::Default::default(),
            slices: core::default::Default::default(),
            tier: core::default::Default::default(),
            tile_height: core::default::Default::default(),
            tile_padding: core::default::Default::default(),
            tile_width: core::default::Default::default(),
            timecode_insertion: core::default::Default::default(),
            treeblock_size: core::default::Default::default(),
            color_space_settings: core::default::Default::default(),
            filter_settings: core::default::Default::default(),
            timecode_burnin_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElRef {
        MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `adaptive_quantization` after provisioning.\n"]
    pub fn adaptive_quantization(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.adaptive_quantization", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `afd_signaling` after provisioning.\n"]
    pub fn afd_signaling(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.afd_signaling", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `alternative_transfer_function` after provisioning.\n"]
    pub fn alternative_transfer_function(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alternative_transfer_function", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bitrate` after provisioning.\n"]
    pub fn bitrate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.bitrate", self.base))
    }
    #[doc = "Get a reference to the value of field `buf_size` after provisioning.\n"]
    pub fn buf_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.buf_size", self.base))
    }
    #[doc = "Get a reference to the value of field `color_metadata` after provisioning.\n"]
    pub fn color_metadata(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.color_metadata", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fixed_afd` after provisioning.\n"]
    pub fn fixed_afd(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fixed_afd", self.base))
    }
    #[doc = "Get a reference to the value of field `flicker_aq` after provisioning.\n"]
    pub fn flicker_aq(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.flicker_aq", self.base))
    }
    #[doc = "Get a reference to the value of field `framerate_denominator` after provisioning.\n"]
    pub fn framerate_denominator(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.framerate_denominator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `framerate_numerator` after provisioning.\n"]
    pub fn framerate_numerator(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.framerate_numerator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gop_closed_cadence` after provisioning.\n"]
    pub fn gop_closed_cadence(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gop_closed_cadence", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gop_size` after provisioning.\n"]
    pub fn gop_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.gop_size", self.base))
    }
    #[doc = "Get a reference to the value of field `gop_size_units` after provisioning.\n"]
    pub fn gop_size_units(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gop_size_units", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `level` after provisioning.\n"]
    pub fn level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.level", self.base))
    }
    #[doc = "Get a reference to the value of field `look_ahead_rate_control` after provisioning.\n"]
    pub fn look_ahead_rate_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.look_ahead_rate_control", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_bitrate` after provisioning.\n"]
    pub fn max_bitrate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_bitrate", self.base))
    }
    #[doc = "Get a reference to the value of field `min_i_interval` after provisioning.\n"]
    pub fn min_i_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_i_interval", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_qp` after provisioning.\n"]
    pub fn min_qp(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_qp", self.base))
    }
    #[doc = "Get a reference to the value of field `mv_over_picture_boundaries` after provisioning.\n"]
    pub fn mv_over_picture_boundaries(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mv_over_picture_boundaries", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mv_temporal_predictor` after provisioning.\n"]
    pub fn mv_temporal_predictor(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mv_temporal_predictor", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `par_denominator` after provisioning.\n"]
    pub fn par_denominator(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.par_denominator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `par_numerator` after provisioning.\n"]
    pub fn par_numerator(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.par_numerator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `profile` after provisioning.\n"]
    pub fn profile(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.profile", self.base))
    }
    #[doc = "Get a reference to the value of field `qvbr_quality_level` after provisioning.\n"]
    pub fn qvbr_quality_level(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.qvbr_quality_level", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rate_control_mode` after provisioning.\n"]
    pub fn rate_control_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rate_control_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scan_type` after provisioning.\n"]
    pub fn scan_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scan_type", self.base))
    }
    #[doc = "Get a reference to the value of field `scene_change_detect` after provisioning.\n"]
    pub fn scene_change_detect(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scene_change_detect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `slices` after provisioning.\n"]
    pub fn slices(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.slices", self.base))
    }
    #[doc = "Get a reference to the value of field `tier` after provisioning.\n"]
    pub fn tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tier", self.base))
    }
    #[doc = "Get a reference to the value of field `tile_height` after provisioning.\n"]
    pub fn tile_height(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tile_height", self.base))
    }
    #[doc = "Get a reference to the value of field `tile_padding` after provisioning.\n"]
    pub fn tile_padding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tile_padding", self.base))
    }
    #[doc = "Get a reference to the value of field `tile_width` after provisioning.\n"]
    pub fn tile_width(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tile_width", self.base))
    }
    #[doc = "Get a reference to the value of field `timecode_insertion` after provisioning.\n"]
    pub fn timecode_insertion(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timecode_insertion", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `treeblock_size` after provisioning.\n"]
    pub fn treeblock_size(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.treeblock_size", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `color_space_settings` after provisioning.\n"]    pub fn color_space_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElColorSpaceSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.color_space_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `filter_settings` after provisioning.\n"]    pub fn filter_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElFilterSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timecode_burnin_settings` after provisioning.\n"]    pub fn timecode_burnin_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElTimecodeBurninSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.timecode_burnin_settings", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElDynamic { frame_capture_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsEl >> , h264_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsEl >> , h265_settings : Option < DynamicBlock < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] frame_capture_settings : Option < Vec < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] h264_settings : Option < Vec < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] h265_settings : Option < Vec < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsEl > > , dynamic : MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElDynamic , }
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsEl {
    #[doc = "Set the field `frame_capture_settings`.\n"]
    pub fn set_frame_capture_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.frame_capture_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.frame_capture_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `h264_settings`.\n"]
    pub fn set_h264_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.h264_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.h264_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `h265_settings`.\n"]
    pub fn set_h265_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.h265_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.h265_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsEl {
    type O = BlockAssignable<MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsEl {}
impl BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsEl {
        MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsEl {
            frame_capture_settings: core::default::Default::default(),
            h264_settings: core::default::Default::default(),
            h265_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElRef {
        MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `frame_capture_settings` after provisioning.\n"]    pub fn frame_capture_settings (& self) -> ListRef < MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.frame_capture_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `h264_settings` after provisioning.\n"]
    pub fn h264_settings(
        &self,
    ) -> ListRef<MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.h264_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `h265_settings` after provisioning.\n"]
    pub fn h265_settings(
        &self,
    ) -> ListRef<MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH265SettingsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.h265_settings", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElVideoDescriptionsElDynamic {
    codec_settings:
        Option<DynamicBlock<MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsEl>>,
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<PrimField<f64>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    respond_to_afd: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scaling_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sharpness: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    codec_settings:
        Option<Vec<MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsEl>>,
    dynamic: MedialiveChannelEncoderSettingsElVideoDescriptionsElDynamic,
}
impl MedialiveChannelEncoderSettingsElVideoDescriptionsEl {
    #[doc = "Set the field `height`.\n"]
    pub fn set_height(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.height = Some(v.into());
        self
    }
    #[doc = "Set the field `respond_to_afd`.\n"]
    pub fn set_respond_to_afd(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.respond_to_afd = Some(v.into());
        self
    }
    #[doc = "Set the field `scaling_behavior`.\n"]
    pub fn set_scaling_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scaling_behavior = Some(v.into());
        self
    }
    #[doc = "Set the field `sharpness`.\n"]
    pub fn set_sharpness(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.sharpness = Some(v.into());
        self
    }
    #[doc = "Set the field `width`.\n"]
    pub fn set_width(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.width = Some(v.into());
        self
    }
    #[doc = "Set the field `codec_settings`.\n"]
    pub fn set_codec_settings(
        mut self,
        v: impl Into<
            BlockAssignable<MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.codec_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.codec_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveChannelEncoderSettingsElVideoDescriptionsEl {
    type O = BlockAssignable<MedialiveChannelEncoderSettingsElVideoDescriptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsElVideoDescriptionsEl {
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildMedialiveChannelEncoderSettingsElVideoDescriptionsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElVideoDescriptionsEl {
        MedialiveChannelEncoderSettingsElVideoDescriptionsEl {
            height: core::default::Default::default(),
            name: self.name,
            respond_to_afd: core::default::Default::default(),
            scaling_behavior: core::default::Default::default(),
            sharpness: core::default::Default::default(),
            width: core::default::Default::default(),
            codec_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElVideoDescriptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElRef {
        MedialiveChannelEncoderSettingsElVideoDescriptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElVideoDescriptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `height` after provisioning.\n"]
    pub fn height(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.height", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `respond_to_afd` after provisioning.\n"]
    pub fn respond_to_afd(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.respond_to_afd", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scaling_behavior` after provisioning.\n"]
    pub fn scaling_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scaling_behavior", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sharpness` after provisioning.\n"]
    pub fn sharpness(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.sharpness", self.base))
    }
    #[doc = "Get a reference to the value of field `width` after provisioning.\n"]
    pub fn width(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.width", self.base))
    }
    #[doc = "Get a reference to the value of field `codec_settings` after provisioning.\n"]
    pub fn codec_settings(
        &self,
    ) -> ListRef<MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.codec_settings", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElDynamic {
    audio_descriptions: Option<DynamicBlock<MedialiveChannelEncoderSettingsElAudioDescriptionsEl>>,
    avail_blanking: Option<DynamicBlock<MedialiveChannelEncoderSettingsElAvailBlankingEl>>,
    caption_descriptions:
        Option<DynamicBlock<MedialiveChannelEncoderSettingsElCaptionDescriptionsEl>>,
    global_configuration:
        Option<DynamicBlock<MedialiveChannelEncoderSettingsElGlobalConfigurationEl>>,
    motion_graphics_configuration:
        Option<DynamicBlock<MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationEl>>,
    nielsen_configuration:
        Option<DynamicBlock<MedialiveChannelEncoderSettingsElNielsenConfigurationEl>>,
    output_groups: Option<DynamicBlock<MedialiveChannelEncoderSettingsElOutputGroupsEl>>,
    timecode_config: Option<DynamicBlock<MedialiveChannelEncoderSettingsElTimecodeConfigEl>>,
    video_descriptions: Option<DynamicBlock<MedialiveChannelEncoderSettingsElVideoDescriptionsEl>>,
}
#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_descriptions: Option<Vec<MedialiveChannelEncoderSettingsElAudioDescriptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avail_blanking: Option<Vec<MedialiveChannelEncoderSettingsElAvailBlankingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_descriptions: Option<Vec<MedialiveChannelEncoderSettingsElCaptionDescriptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    global_configuration: Option<Vec<MedialiveChannelEncoderSettingsElGlobalConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    motion_graphics_configuration:
        Option<Vec<MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nielsen_configuration: Option<Vec<MedialiveChannelEncoderSettingsElNielsenConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_groups: Option<Vec<MedialiveChannelEncoderSettingsElOutputGroupsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timecode_config: Option<Vec<MedialiveChannelEncoderSettingsElTimecodeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_descriptions: Option<Vec<MedialiveChannelEncoderSettingsElVideoDescriptionsEl>>,
    dynamic: MedialiveChannelEncoderSettingsElDynamic,
}
impl MedialiveChannelEncoderSettingsEl {
    #[doc = "Set the field `audio_descriptions`.\n"]
    pub fn set_audio_descriptions(
        mut self,
        v: impl Into<BlockAssignable<MedialiveChannelEncoderSettingsElAudioDescriptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.audio_descriptions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_descriptions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `avail_blanking`.\n"]
    pub fn set_avail_blanking(
        mut self,
        v: impl Into<BlockAssignable<MedialiveChannelEncoderSettingsElAvailBlankingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.avail_blanking = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.avail_blanking = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `caption_descriptions`.\n"]
    pub fn set_caption_descriptions(
        mut self,
        v: impl Into<BlockAssignable<MedialiveChannelEncoderSettingsElCaptionDescriptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.caption_descriptions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.caption_descriptions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `global_configuration`.\n"]
    pub fn set_global_configuration(
        mut self,
        v: impl Into<BlockAssignable<MedialiveChannelEncoderSettingsElGlobalConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.global_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.global_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `motion_graphics_configuration`.\n"]
    pub fn set_motion_graphics_configuration(
        mut self,
        v: impl Into<BlockAssignable<MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.motion_graphics_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.motion_graphics_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `nielsen_configuration`.\n"]
    pub fn set_nielsen_configuration(
        mut self,
        v: impl Into<BlockAssignable<MedialiveChannelEncoderSettingsElNielsenConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.nielsen_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.nielsen_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `output_groups`.\n"]
    pub fn set_output_groups(
        mut self,
        v: impl Into<BlockAssignable<MedialiveChannelEncoderSettingsElOutputGroupsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.output_groups = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.output_groups = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timecode_config`.\n"]
    pub fn set_timecode_config(
        mut self,
        v: impl Into<BlockAssignable<MedialiveChannelEncoderSettingsElTimecodeConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.timecode_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.timecode_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `video_descriptions`.\n"]
    pub fn set_video_descriptions(
        mut self,
        v: impl Into<BlockAssignable<MedialiveChannelEncoderSettingsElVideoDescriptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.video_descriptions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.video_descriptions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveChannelEncoderSettingsEl {
    type O = BlockAssignable<MedialiveChannelEncoderSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelEncoderSettingsEl {}
impl BuildMedialiveChannelEncoderSettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsEl {
        MedialiveChannelEncoderSettingsEl {
            audio_descriptions: core::default::Default::default(),
            avail_blanking: core::default::Default::default(),
            caption_descriptions: core::default::Default::default(),
            global_configuration: core::default::Default::default(),
            motion_graphics_configuration: core::default::Default::default(),
            nielsen_configuration: core::default::Default::default(),
            output_groups: core::default::Default::default(),
            timecode_config: core::default::Default::default(),
            video_descriptions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelEncoderSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelEncoderSettingsElRef {
    fn new(shared: StackShared, base: String) -> MedialiveChannelEncoderSettingsElRef {
        MedialiveChannelEncoderSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelEncoderSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `avail_blanking` after provisioning.\n"]
    pub fn avail_blanking(&self) -> ListRef<MedialiveChannelEncoderSettingsElAvailBlankingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.avail_blanking", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `caption_descriptions` after provisioning.\n"]
    pub fn caption_descriptions(
        &self,
    ) -> ListRef<MedialiveChannelEncoderSettingsElCaptionDescriptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.caption_descriptions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `global_configuration` after provisioning.\n"]
    pub fn global_configuration(
        &self,
    ) -> ListRef<MedialiveChannelEncoderSettingsElGlobalConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.global_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `motion_graphics_configuration` after provisioning.\n"]
    pub fn motion_graphics_configuration(
        &self,
    ) -> ListRef<MedialiveChannelEncoderSettingsElMotionGraphicsConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.motion_graphics_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nielsen_configuration` after provisioning.\n"]
    pub fn nielsen_configuration(
        &self,
    ) -> ListRef<MedialiveChannelEncoderSettingsElNielsenConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.nielsen_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `output_groups` after provisioning.\n"]
    pub fn output_groups(&self) -> ListRef<MedialiveChannelEncoderSettingsElOutputGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_groups", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timecode_config` after provisioning.\n"]
    pub fn timecode_config(&self) -> ListRef<MedialiveChannelEncoderSettingsElTimecodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.timecode_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `video_descriptions` after provisioning.\n"]
    pub fn video_descriptions(
        &self,
    ) -> ListRef<MedialiveChannelEncoderSettingsElVideoDescriptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.video_descriptions", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsEl
{
    audio_selector_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_silence_threshold_msec: Option<PrimField<f64>>,
}
impl MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsEl { # [doc = "Set the field `audio_silence_threshold_msec`.\n"] pub fn set_audio_silence_threshold_msec (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . audio_silence_threshold_msec = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsEl { type O = BlockAssignable < MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsEl
{
    #[doc = ""]
    pub audio_selector_name: PrimField<String>,
}
impl BuildMedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsEl { pub fn build (self) -> MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsEl { MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsEl { audio_selector_name : self . audio_selector_name , audio_silence_threshold_msec : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsElRef { MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `audio_selector_name` after provisioning.\n"] pub fn audio_selector_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.audio_selector_name" , self . base)) } # [doc = "Get a reference to the value of field `audio_silence_threshold_msec` after provisioning.\n"] pub fn audio_silence_threshold_msec (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.audio_silence_threshold_msec" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    input_loss_threshold_msec: Option<PrimField<f64>>,
}
impl MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsEl { # [doc = "Set the field `input_loss_threshold_msec`.\n"] pub fn set_input_loss_threshold_msec (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . input_loss_threshold_msec = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsEl { type O = BlockAssignable < MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsEl
{}
impl BuildMedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsEl { pub fn build (self) -> MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsEl { MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsEl { input_loss_threshold_msec : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsElRef { MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `input_loss_threshold_msec` after provisioning.\n"] pub fn input_loss_threshold_msec (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.input_loss_threshold_msec" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    black_detect_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_black_threshold_msec: Option<PrimField<f64>>,
}
impl MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsEl { # [doc = "Set the field `black_detect_threshold`.\n"] pub fn set_black_detect_threshold (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . black_detect_threshold = Some (v . into ()) ; self } # [doc = "Set the field `video_black_threshold_msec`.\n"] pub fn set_video_black_threshold_msec (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . video_black_threshold_msec = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsEl { type O = BlockAssignable < MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsEl
{}
impl BuildMedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsEl { pub fn build (self) -> MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsEl { MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsEl { black_detect_threshold : core :: default :: Default :: default () , video_black_threshold_msec : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsElRef { MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `black_detect_threshold` after provisioning.\n"] pub fn black_detect_threshold (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.black_detect_threshold" , self . base)) } # [doc = "Get a reference to the value of field `video_black_threshold_msec` after provisioning.\n"] pub fn video_black_threshold_msec (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.video_black_threshold_msec" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElDynamic { audio_silence_settings : Option < DynamicBlock < MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsEl >> , input_loss_settings : Option < DynamicBlock < MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsEl >> , video_black_settings : Option < DynamicBlock < MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] audio_silence_settings : Option < Vec < MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] input_loss_settings : Option < Vec < MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] video_black_settings : Option < Vec < MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsEl > > , dynamic : MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElDynamic , }
impl MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsEl { # [doc = "Set the field `audio_silence_settings`.\n"] pub fn set_audio_silence_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . audio_silence_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . audio_silence_settings = Some (d) ; } } self } # [doc = "Set the field `input_loss_settings`.\n"] pub fn set_input_loss_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . input_loss_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . input_loss_settings = Some (d) ; } } self } # [doc = "Set the field `video_black_settings`.\n"] pub fn set_video_black_settings (mut self , v : impl Into < BlockAssignable < MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . video_black_settings = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . video_black_settings = Some (d) ; } } self } }
impl ToListMappable for MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsEl { type O = BlockAssignable < MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsEl
{}
impl BuildMedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsEl { pub fn build (self) -> MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsEl { MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsEl { audio_silence_settings : core :: default :: Default :: default () , input_loss_settings : core :: default :: Default :: default () , video_black_settings : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElRef { MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `audio_silence_settings` after provisioning.\n"] pub fn audio_silence_settings (& self) -> ListRef < MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.audio_silence_settings" , self . base)) } # [doc = "Get a reference to the value of field `input_loss_settings` after provisioning.\n"] pub fn input_loss_settings (& self) -> ListRef < MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.input_loss_settings" , self . base)) } # [doc = "Get a reference to the value of field `video_black_settings` after provisioning.\n"] pub fn video_black_settings (& self) -> ListRef < MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.video_black_settings" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElDynamic { failover_condition_settings : Option < DynamicBlock < MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionEl { # [serde (skip_serializing_if = "Option::is_none")] failover_condition_settings : Option < Vec < MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsEl > > , dynamic : MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElDynamic , }
impl MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionEl {
    #[doc = "Set the field `failover_condition_settings`.\n"]
    pub fn set_failover_condition_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.failover_condition_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.failover_condition_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionEl
{
    type O = BlockAssignable<
        MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionEl
{}
impl BuildMedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionEl {
    pub fn build(
        self,
    ) -> MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionEl {
        MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionEl {
            failover_condition_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElRef
    {
        MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `failover_condition_settings` after provisioning.\n"]    pub fn failover_condition_settings (& self) -> ListRef < MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.failover_condition_settings", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElDynamic {
    failover_condition: Option<
        DynamicBlock<
            MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    error_clear_time_msec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_preference: Option<PrimField<String>>,
    secondary_input_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failover_condition: Option<
        Vec<MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionEl>,
    >,
    dynamic: MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElDynamic,
}
impl MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsEl {
    #[doc = "Set the field `error_clear_time_msec`.\n"]
    pub fn set_error_clear_time_msec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.error_clear_time_msec = Some(v.into());
        self
    }
    #[doc = "Set the field `input_preference`.\n"]
    pub fn set_input_preference(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.input_preference = Some(v.into());
        self
    }
    #[doc = "Set the field `failover_condition`.\n"]
    pub fn set_failover_condition(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.failover_condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.failover_condition = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsEl {
    type O = BlockAssignable<MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsEl {
    #[doc = ""]
    pub secondary_input_id: PrimField<String>,
}
impl BuildMedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsEl {
    pub fn build(self) -> MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsEl {
        MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsEl {
            error_clear_time_msec: core::default::Default::default(),
            input_preference: core::default::Default::default(),
            secondary_input_id: self.secondary_input_id,
            failover_condition: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElRef {
        MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `error_clear_time_msec` after provisioning.\n"]
    pub fn error_clear_time_msec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.error_clear_time_msec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `input_preference` after provisioning.\n"]
    pub fn input_preference(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_preference", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_input_id` after provisioning.\n"]
    pub fn secondary_input_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secondary_input_id", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionEl
{
    group_id: PrimField<String>,
    name: PrimField<String>,
}
impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionEl { }
impl ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionEl { type O = BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionEl
{
    #[doc = ""]
    pub group_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionEl { pub fn build (self) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionEl { MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionEl { group_id : self . group_id , name : self . name , } } }
pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionElRef { MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `group_id` after provisioning.\n"] pub fn group_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.group_id" , self . base)) } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionEl
{
    language_code: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_selection_policy: Option<PrimField<String>>,
}
impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionEl { # [doc = "Set the field `language_selection_policy`.\n"] pub fn set_language_selection_policy (mut self , v : impl Into < PrimField < String > >) -> Self { self . language_selection_policy = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionEl { type O = BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionEl
{
    #[doc = ""]
    pub language_code: PrimField<String>,
}
impl BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionEl { pub fn build (self) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionEl { MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionEl { language_code : self . language_code , language_selection_policy : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionElRef { MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `language_code` after provisioning.\n"] pub fn language_code (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.language_code" , self . base)) } # [doc = "Get a reference to the value of field `language_selection_policy` after provisioning.\n"] pub fn language_selection_policy (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.language_selection_policy" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionEl
{
    pid: PrimField<f64>,
}
impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionEl { }
impl ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionEl { type O = BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionEl
{
    #[doc = ""]
    pub pid: PrimField<f64>,
}
impl BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionEl { pub fn build (self) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionEl { MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionEl { pid : self . pid , } } }
pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionElRef { MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `pid` after provisioning.\n"] pub fn pid (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pid" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElDolbyEDecodeEl
{
    program_selection: PrimField<String>,
}
impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElDolbyEDecodeEl { }
impl ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElDolbyEDecodeEl { type O = BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElDolbyEDecodeEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElDolbyEDecodeEl
{
    #[doc = ""]
    pub program_selection: PrimField<String>,
}
impl BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElDolbyEDecodeEl { pub fn build (self) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElDolbyEDecodeEl { MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElDolbyEDecodeEl { program_selection : self . program_selection , } } }
pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElDolbyEDecodeElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElDolbyEDecodeElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElDolbyEDecodeElRef { MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElDolbyEDecodeElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElDolbyEDecodeElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `program_selection` after provisioning.\n"] pub fn program_selection (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.program_selection" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTracksEl
{
    track: PrimField<f64>,
}
impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTracksEl { }
impl ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTracksEl { type O = BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTracksEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTracksEl
{
    #[doc = ""]
    pub track: PrimField<f64>,
}
impl BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTracksEl { pub fn build (self) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTracksEl { MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTracksEl { track : self . track , } } }
pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTracksElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTracksElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTracksElRef { MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTracksElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTracksElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `track` after provisioning.\n"] pub fn track (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.track" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElDynamic { dolby_e_decode : Option < DynamicBlock < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElDolbyEDecodeEl >> , tracks : Option < DynamicBlock < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTracksEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionEl { # [serde (skip_serializing_if = "Option::is_none")] dolby_e_decode : Option < Vec < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElDolbyEDecodeEl > > , # [serde (skip_serializing_if = "Option::is_none")] tracks : Option < Vec < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTracksEl > > , dynamic : MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElDynamic , }
impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionEl { # [doc = "Set the field `dolby_e_decode`.\n"] pub fn set_dolby_e_decode (mut self , v : impl Into < BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElDolbyEDecodeEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . dolby_e_decode = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . dolby_e_decode = Some (d) ; } } self } # [doc = "Set the field `tracks`.\n"] pub fn set_tracks (mut self , v : impl Into < BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTracksEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . tracks = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . tracks = Some (d) ; } } self } }
impl ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionEl { type O = BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionEl
{}
impl BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionEl { pub fn build (self) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionEl { MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionEl { dolby_e_decode : core :: default :: Default :: default () , tracks : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElRef { MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `dolby_e_decode` after provisioning.\n"] pub fn dolby_e_decode (& self) -> ListRef < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElDolbyEDecodeElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.dolby_e_decode" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElDynamic { audio_hls_rendition_selection : Option < DynamicBlock < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionEl >> , audio_language_selection : Option < DynamicBlock < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionEl >> , audio_pid_selection : Option < DynamicBlock < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionEl >> , audio_track_selection : Option < DynamicBlock < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] audio_hls_rendition_selection : Option < Vec < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionEl > > , # [serde (skip_serializing_if = "Option::is_none")] audio_language_selection : Option < Vec < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionEl > > , # [serde (skip_serializing_if = "Option::is_none")] audio_pid_selection : Option < Vec < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionEl > > , # [serde (skip_serializing_if = "Option::is_none")] audio_track_selection : Option < Vec < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionEl > > , dynamic : MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElDynamic , }
impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsEl {
    #[doc = "Set the field `audio_hls_rendition_selection`.\n"]
    pub fn set_audio_hls_rendition_selection(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.audio_hls_rendition_selection = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_hls_rendition_selection = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `audio_language_selection`.\n"]
    pub fn set_audio_language_selection(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.audio_language_selection = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_language_selection = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `audio_pid_selection`.\n"]
    pub fn set_audio_pid_selection(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.audio_pid_selection = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_pid_selection = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `audio_track_selection`.\n"]
    pub fn set_audio_track_selection(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.audio_track_selection = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_track_selection = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsEl
{
    type O = BlockAssignable<
        MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsEl
{}
impl BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsEl {
        MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsEl {
            audio_hls_rendition_selection: core::default::Default::default(),
            audio_language_selection: core::default::Default::default(),
            audio_pid_selection: core::default::Default::default(),
            audio_track_selection: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `audio_hls_rendition_selection` after provisioning.\n"]    pub fn audio_hls_rendition_selection (& self) -> ListRef < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.audio_hls_rendition_selection", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `audio_language_selection` after provisioning.\n"]    pub fn audio_language_selection (& self) -> ListRef < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.audio_language_selection", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `audio_pid_selection` after provisioning.\n"]    pub fn audio_pid_selection (& self) -> ListRef < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.audio_pid_selection", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `audio_track_selection` after provisioning.\n"]    pub fn audio_track_selection (& self) -> ListRef < MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.audio_track_selection", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElDynamic {
    selector_settings: Option<
        DynamicBlock<
            MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selector_settings: Option<
        Vec<MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsEl>,
    >,
    dynamic: MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElDynamic,
}
impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorEl {
    #[doc = "Set the field `selector_settings`.\n"]
    pub fn set_selector_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.selector_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.selector_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorEl {
    type O = BlockAssignable<MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorEl {
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorEl {
    pub fn build(self) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorEl {
        MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorEl {
            name: self.name,
            selector_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `selector_settings` after provisioning.\n"]
    pub fn selector_settings(
        &self,
    ) -> ListRef<
        MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.selector_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    source_ancillary_channel_number: Option<PrimField<f64>>,
}
impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsEl { # [doc = "Set the field `source_ancillary_channel_number`.\n"] pub fn set_source_ancillary_channel_number (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . source_ancillary_channel_number = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsEl { type O = BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsEl
{}
impl BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsEl { pub fn build (self) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsEl { MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsEl { source_ancillary_channel_number : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsElRef { MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `source_ancillary_channel_number` after provisioning.\n"] pub fn source_ancillary_channel_number (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.source_ancillary_channel_number" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAribSourceSettingsEl
{}
impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAribSourceSettingsEl { }
impl ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAribSourceSettingsEl { type O = BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAribSourceSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAribSourceSettingsEl
{}
impl BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAribSourceSettingsEl { pub fn build (self) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAribSourceSettingsEl { MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAribSourceSettingsEl { } } }
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAribSourceSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAribSourceSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAribSourceSettingsElRef { MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAribSourceSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAribSourceSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbSubSourceSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    ocr_language: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pid: Option<PrimField<f64>>,
}
impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbSubSourceSettingsEl { # [doc = "Set the field `ocr_language`.\n"] pub fn set_ocr_language (mut self , v : impl Into < PrimField < String > >) -> Self { self . ocr_language = Some (v . into ()) ; self } # [doc = "Set the field `pid`.\n"] pub fn set_pid (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . pid = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbSubSourceSettingsEl { type O = BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbSubSourceSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbSubSourceSettingsEl
{}
impl BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbSubSourceSettingsEl { pub fn build (self) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbSubSourceSettingsEl { MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbSubSourceSettingsEl { ocr_language : core :: default :: Default :: default () , pid : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbSubSourceSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbSubSourceSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbSubSourceSettingsElRef { MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbSubSourceSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbSubSourceSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `ocr_language` after provisioning.\n"] pub fn ocr_language (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ocr_language" , self . base)) } # [doc = "Get a reference to the value of field `pid` after provisioning.\n"] pub fn pid (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pid" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    convert_608_to_708: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scte20_detection: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_608_channel_number: Option<PrimField<f64>>,
}
impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsEl { # [doc = "Set the field `convert_608_to_708`.\n"] pub fn set_convert_608_to_708 (mut self , v : impl Into < PrimField < String > >) -> Self { self . convert_608_to_708 = Some (v . into ()) ; self } # [doc = "Set the field `scte20_detection`.\n"] pub fn set_scte20_detection (mut self , v : impl Into < PrimField < String > >) -> Self { self . scte20_detection = Some (v . into ()) ; self } # [doc = "Set the field `source_608_channel_number`.\n"] pub fn set_source_608_channel_number (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . source_608_channel_number = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsEl { type O = BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsEl
{}
impl BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsEl { pub fn build (self) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsEl { MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsEl { convert_608_to_708 : core :: default :: Default :: default () , scte20_detection : core :: default :: Default :: default () , source_608_channel_number : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsElRef { MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `convert_608_to_708` after provisioning.\n"] pub fn convert_608_to_708 (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.convert_608_to_708" , self . base)) } # [doc = "Get a reference to the value of field `scte20_detection` after provisioning.\n"] pub fn scte20_detection (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.scte20_detection" , self . base)) } # [doc = "Get a reference to the value of field `source_608_channel_number` after provisioning.\n"] pub fn source_608_channel_number (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.source_608_channel_number" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    convert_608_to_708: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_608_channel_number: Option<PrimField<f64>>,
}
impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsEl { # [doc = "Set the field `convert_608_to_708`.\n"] pub fn set_convert_608_to_708 (mut self , v : impl Into < PrimField < String > >) -> Self { self . convert_608_to_708 = Some (v . into ()) ; self } # [doc = "Set the field `source_608_channel_number`.\n"] pub fn set_source_608_channel_number (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . source_608_channel_number = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsEl { type O = BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsEl
{}
impl BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsEl { pub fn build (self) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsEl { MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsEl { convert_608_to_708 : core :: default :: Default :: default () , source_608_channel_number : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsElRef { MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `convert_608_to_708` after provisioning.\n"] pub fn convert_608_to_708 (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.convert_608_to_708" , self . base)) } # [doc = "Get a reference to the value of field `source_608_channel_number` after provisioning.\n"] pub fn source_608_channel_number (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.source_608_channel_number" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    ocr_language: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pid: Option<PrimField<f64>>,
}
impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsEl { # [doc = "Set the field `ocr_language`.\n"] pub fn set_ocr_language (mut self , v : impl Into < PrimField < String > >) -> Self { self . ocr_language = Some (v . into ()) ; self } # [doc = "Set the field `pid`.\n"] pub fn set_pid (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . pid = Some (v . into ()) ; self } }
impl ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsEl { type O = BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsEl
{}
impl BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsEl { pub fn build (self) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsEl { MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsEl { ocr_language : core :: default :: Default :: default () , pid : core :: default :: Default :: default () , } } }
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsElRef { MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `ocr_language` after provisioning.\n"] pub fn ocr_language (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ocr_language" , self . base)) } # [doc = "Get a reference to the value of field `pid` after provisioning.\n"] pub fn pid (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pid" , self . base)) } }
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleEl
{
    height: PrimField<f64>,
    left_offset: PrimField<f64>,
    top_offset: PrimField<f64>,
    width: PrimField<f64>,
}
impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleEl { }
impl ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleEl { type O = BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleEl
{
    #[doc = ""]
    pub height: PrimField<f64>,
    #[doc = ""]
    pub left_offset: PrimField<f64>,
    #[doc = ""]
    pub top_offset: PrimField<f64>,
    #[doc = ""]
    pub width: PrimField<f64>,
}
impl BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleEl { pub fn build (self) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleEl { MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleEl { height : self . height , left_offset : self . left_offset , top_offset : self . top_offset , width : self . width , } } }
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleElRef { MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `height` after provisioning.\n"] pub fn height (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.height" , self . base)) } # [doc = "Get a reference to the value of field `left_offset` after provisioning.\n"] pub fn left_offset (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.left_offset" , self . base)) } # [doc = "Get a reference to the value of field `top_offset` after provisioning.\n"] pub fn top_offset (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.top_offset" , self . base)) } # [doc = "Get a reference to the value of field `width` after provisioning.\n"] pub fn width (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.width" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElDynamic { output_rectangle : Option < DynamicBlock < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] page_number : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] output_rectangle : Option < Vec < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleEl > > , dynamic : MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElDynamic , }
impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsEl { # [doc = "Set the field `page_number`.\n"] pub fn set_page_number (mut self , v : impl Into < PrimField < String > >) -> Self { self . page_number = Some (v . into ()) ; self } # [doc = "Set the field `output_rectangle`.\n"] pub fn set_output_rectangle (mut self , v : impl Into < BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . output_rectangle = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . output_rectangle = Some (d) ; } } self } }
impl ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsEl { type O = BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsEl
{}
impl BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsEl { pub fn build (self) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsEl { MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsEl { page_number : core :: default :: Default :: default () , output_rectangle : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElRef { fn new (shared : StackShared , base : String) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElRef { MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElRef { shared : shared , base : base . to_string () , } } }
impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `page_number` after provisioning.\n"] pub fn page_number (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.page_number" , self . base)) } # [doc = "Get a reference to the value of field `output_rectangle` after provisioning.\n"] pub fn output_rectangle (& self) -> ListRef < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.output_rectangle" , self . base)) } }
#[derive(Serialize, Default)]
struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDynamic { ancillary_source_settings : Option < DynamicBlock < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsEl >> , arib_source_settings : Option < DynamicBlock < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAribSourceSettingsEl >> , dvb_sub_source_settings : Option < DynamicBlock < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbSubSourceSettingsEl >> , embedded_source_settings : Option < DynamicBlock < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsEl >> , scte20_source_settings : Option < DynamicBlock < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsEl >> , scte27_source_settings : Option < DynamicBlock < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsEl >> , teletext_source_settings : Option < DynamicBlock < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] ancillary_source_settings : Option < Vec < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] arib_source_settings : Option < Vec < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAribSourceSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] dvb_sub_source_settings : Option < Vec < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbSubSourceSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] embedded_source_settings : Option < Vec < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] scte20_source_settings : Option < Vec < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] scte27_source_settings : Option < Vec < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] teletext_source_settings : Option < Vec < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsEl > > , dynamic : MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDynamic , }
impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsEl {
    #[doc = "Set the field `ancillary_source_settings`.\n"]
    pub fn set_ancillary_source_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ancillary_source_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ancillary_source_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `arib_source_settings`.\n"]
    pub fn set_arib_source_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAribSourceSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.arib_source_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.arib_source_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dvb_sub_source_settings`.\n"]
    pub fn set_dvb_sub_source_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbSubSourceSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dvb_sub_source_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dvb_sub_source_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `embedded_source_settings`.\n"]
    pub fn set_embedded_source_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.embedded_source_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.embedded_source_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scte20_source_settings`.\n"]
    pub fn set_scte20_source_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.scte20_source_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.scte20_source_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scte27_source_settings`.\n"]
    pub fn set_scte27_source_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.scte27_source_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.scte27_source_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `teletext_source_settings`.\n"]
    pub fn set_teletext_source_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.teletext_source_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.teletext_source_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsEl
{
    type O = BlockAssignable<
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsEl
{}
impl BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsEl {
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsEl {
            ancillary_source_settings: core::default::Default::default(),
            arib_source_settings: core::default::Default::default(),
            dvb_sub_source_settings: core::default::Default::default(),
            embedded_source_settings: core::default::Default::default(),
            scte20_source_settings: core::default::Default::default(),
            scte27_source_settings: core::default::Default::default(),
            teletext_source_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElRef
    {
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ancillary_source_settings` after provisioning.\n"]    pub fn ancillary_source_settings (& self) -> ListRef < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.ancillary_source_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `arib_source_settings` after provisioning.\n"]    pub fn arib_source_settings (& self) -> ListRef < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAribSourceSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.arib_source_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dvb_sub_source_settings` after provisioning.\n"]    pub fn dvb_sub_source_settings (& self) -> ListRef < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbSubSourceSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.dvb_sub_source_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `embedded_source_settings` after provisioning.\n"]    pub fn embedded_source_settings (& self) -> ListRef < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.embedded_source_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scte20_source_settings` after provisioning.\n"]    pub fn scte20_source_settings (& self) -> ListRef < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.scte20_source_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scte27_source_settings` after provisioning.\n"]    pub fn scte27_source_settings (& self) -> ListRef < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.scte27_source_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `teletext_source_settings` after provisioning.\n"]    pub fn teletext_source_settings (& self) -> ListRef < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.teletext_source_settings", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElDynamic {
    selector_settings: Option<
        DynamicBlock<
            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selector_settings: Option<
        Vec<MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsEl>,
    >,
    dynamic: MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElDynamic,
}
impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorEl {
    #[doc = "Set the field `language_code`.\n"]
    pub fn set_language_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.language_code = Some(v.into());
        self
    }
    #[doc = "Set the field `selector_settings`.\n"]
    pub fn set_selector_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.selector_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.selector_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorEl {
    type O = BlockAssignable<MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorEl {
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorEl {
    pub fn build(self) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorEl {
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorEl {
            language_code: core::default::Default::default(),
            name: self.name,
            selector_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `selector_settings` after provisioning.\n"]
    pub fn selector_settings(
        &self,
    ) -> ListRef<
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.selector_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    bandwidth: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buffer_segments: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scte35_source: Option<PrimField<String>>,
}
impl MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsEl {
    #[doc = "Set the field `bandwidth`.\n"]
    pub fn set_bandwidth(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.bandwidth = Some(v.into());
        self
    }
    #[doc = "Set the field `buffer_segments`.\n"]
    pub fn set_buffer_segments(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.buffer_segments = Some(v.into());
        self
    }
    #[doc = "Set the field `retries`.\n"]
    pub fn set_retries(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.retries = Some(v.into());
        self
    }
    #[doc = "Set the field `retry_interval`.\n"]
    pub fn set_retry_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.retry_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `scte35_source`.\n"]
    pub fn set_scte35_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scte35_source = Some(v.into());
        self
    }
}
impl ToListMappable
    for MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsEl
{
    type O = BlockAssignable<
        MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsEl
{}
impl
    BuildMedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsEl
{
    pub fn build(
        self,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsEl
    {
        MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsEl {
            bandwidth: core::default::Default::default(),
            buffer_segments: core::default::Default::default(),
            retries: core::default::Default::default(),
            retry_interval: core::default::Default::default(),
            scte35_source: core::default::Default::default(),
        }
    }
}
pub struct MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsElRef
    {
        MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsElRef { shared : shared , base : base . to_string () , }
    }
}
impl MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bandwidth` after provisioning.\n"]
    pub fn bandwidth(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.bandwidth", self.base))
    }
    #[doc = "Get a reference to the value of field `buffer_segments` after provisioning.\n"]
    pub fn buffer_segments(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.buffer_segments", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `retries` after provisioning.\n"]
    pub fn retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retries", self.base))
    }
    #[doc = "Get a reference to the value of field `retry_interval` after provisioning.\n"]
    pub fn retry_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retry_interval", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scte35_source` after provisioning.\n"]
    pub fn scte35_source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scte35_source", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElDynamic { hls_input_settings : Option < DynamicBlock < MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsEl >> , }
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] server_validation : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] hls_input_settings : Option < Vec < MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsEl > > , dynamic : MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElDynamic , }
impl MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsEl {
    #[doc = "Set the field `server_validation`.\n"]
    pub fn set_server_validation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.server_validation = Some(v.into());
        self
    }
    #[doc = "Set the field `hls_input_settings`.\n"]
    pub fn set_hls_input_settings(
        mut self,
        v : impl Into < BlockAssignable < MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hls_input_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hls_input_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsEl {
    type O =
        BlockAssignable<MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsEl {}
impl BuildMedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsEl {
    pub fn build(self) -> MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsEl {
        MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsEl {
            server_validation: core::default::Default::default(),
            hls_input_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `server_validation` after provisioning.\n"]
    pub fn server_validation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_validation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hls_input_settings` after provisioning.\n"]    pub fn hls_input_settings (& self) -> ListRef < MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.hls_input_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    color_space: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color_space_usage: Option<PrimField<String>>,
}
impl MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorEl {
    #[doc = "Set the field `color_space`.\n"]
    pub fn set_color_space(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.color_space = Some(v.into());
        self
    }
    #[doc = "Set the field `color_space_usage`.\n"]
    pub fn set_color_space_usage(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.color_space_usage = Some(v.into());
        self
    }
}
impl ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorEl {
    type O = BlockAssignable<MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorEl {}
impl BuildMedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorEl {
    pub fn build(self) -> MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorEl {
        MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorEl {
            color_space: core::default::Default::default(),
            color_space_usage: core::default::Default::default(),
        }
    }
}
pub struct MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `color_space` after provisioning.\n"]
    pub fn color_space(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.color_space", self.base))
    }
    #[doc = "Get a reference to the value of field `color_space_usage` after provisioning.\n"]
    pub fn color_space_usage(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.color_space_usage", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MedialiveChannelInputAttachmentsElInputSettingsElDynamic {
    audio_selector:
        Option<DynamicBlock<MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorEl>>,
    caption_selector:
        Option<DynamicBlock<MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorEl>>,
    network_input_settings: Option<
        DynamicBlock<MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsEl>,
    >,
    video_selector:
        Option<DynamicBlock<MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorEl>>,
}
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    deblock_filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    denoise_filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_strength: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scte35_pid: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    smpte2038_data_preference: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_end_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_selector: Option<Vec<MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_selector:
        Option<Vec<MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_input_settings:
        Option<Vec<MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_selector: Option<Vec<MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorEl>>,
    dynamic: MedialiveChannelInputAttachmentsElInputSettingsElDynamic,
}
impl MedialiveChannelInputAttachmentsElInputSettingsEl {
    #[doc = "Set the field `deblock_filter`.\n"]
    pub fn set_deblock_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deblock_filter = Some(v.into());
        self
    }
    #[doc = "Set the field `denoise_filter`.\n"]
    pub fn set_denoise_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.denoise_filter = Some(v.into());
        self
    }
    #[doc = "Set the field `filter_strength`.\n"]
    pub fn set_filter_strength(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.filter_strength = Some(v.into());
        self
    }
    #[doc = "Set the field `input_filter`.\n"]
    pub fn set_input_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.input_filter = Some(v.into());
        self
    }
    #[doc = "Set the field `scte35_pid`.\n"]
    pub fn set_scte35_pid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scte35_pid = Some(v.into());
        self
    }
    #[doc = "Set the field `smpte2038_data_preference`.\n"]
    pub fn set_smpte2038_data_preference(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.smpte2038_data_preference = Some(v.into());
        self
    }
    #[doc = "Set the field `source_end_behavior`.\n"]
    pub fn set_source_end_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_end_behavior = Some(v.into());
        self
    }
    #[doc = "Set the field `audio_selector`.\n"]
    pub fn set_audio_selector(
        mut self,
        v: impl Into<BlockAssignable<MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.audio_selector = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_selector = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `caption_selector`.\n"]
    pub fn set_caption_selector(
        mut self,
        v: impl Into<
            BlockAssignable<MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.caption_selector = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.caption_selector = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_input_settings`.\n"]
    pub fn set_network_input_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_input_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_input_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `video_selector`.\n"]
    pub fn set_video_selector(
        mut self,
        v: impl Into<BlockAssignable<MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.video_selector = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.video_selector = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsEl {
    type O = BlockAssignable<MedialiveChannelInputAttachmentsElInputSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsEl {}
impl BuildMedialiveChannelInputAttachmentsElInputSettingsEl {
    pub fn build(self) -> MedialiveChannelInputAttachmentsElInputSettingsEl {
        MedialiveChannelInputAttachmentsElInputSettingsEl {
            deblock_filter: core::default::Default::default(),
            denoise_filter: core::default::Default::default(),
            filter_strength: core::default::Default::default(),
            input_filter: core::default::Default::default(),
            scte35_pid: core::default::Default::default(),
            smpte2038_data_preference: core::default::Default::default(),
            source_end_behavior: core::default::Default::default(),
            audio_selector: core::default::Default::default(),
            caption_selector: core::default::Default::default(),
            network_input_settings: core::default::Default::default(),
            video_selector: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelInputAttachmentsElInputSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElInputSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelInputAttachmentsElInputSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deblock_filter` after provisioning.\n"]
    pub fn deblock_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deblock_filter", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `denoise_filter` after provisioning.\n"]
    pub fn denoise_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.denoise_filter", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `filter_strength` after provisioning.\n"]
    pub fn filter_strength(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter_strength", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `input_filter` after provisioning.\n"]
    pub fn input_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.input_filter", self.base))
    }
    #[doc = "Get a reference to the value of field `scte35_pid` after provisioning.\n"]
    pub fn scte35_pid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scte35_pid", self.base))
    }
    #[doc = "Get a reference to the value of field `smpte2038_data_preference` after provisioning.\n"]
    pub fn smpte2038_data_preference(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.smpte2038_data_preference", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_end_behavior` after provisioning.\n"]
    pub fn source_end_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_end_behavior", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `audio_selector` after provisioning.\n"]
    pub fn audio_selector(
        &self,
    ) -> ListRef<MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.audio_selector", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `caption_selector` after provisioning.\n"]
    pub fn caption_selector(
        &self,
    ) -> ListRef<MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.caption_selector", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_input_settings` after provisioning.\n"]
    pub fn network_input_settings(
        &self,
    ) -> ListRef<MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_input_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `video_selector` after provisioning.\n"]
    pub fn video_selector(
        &self,
    ) -> ListRef<MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.video_selector", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MedialiveChannelInputAttachmentsElDynamic {
    automatic_input_failover_settings:
        Option<DynamicBlock<MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsEl>>,
    input_settings: Option<DynamicBlock<MedialiveChannelInputAttachmentsElInputSettingsEl>>,
}
#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsEl {
    input_attachment_name: PrimField<String>,
    input_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_input_failover_settings:
        Option<Vec<MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_settings: Option<Vec<MedialiveChannelInputAttachmentsElInputSettingsEl>>,
    dynamic: MedialiveChannelInputAttachmentsElDynamic,
}
impl MedialiveChannelInputAttachmentsEl {
    #[doc = "Set the field `automatic_input_failover_settings`.\n"]
    pub fn set_automatic_input_failover_settings(
        mut self,
        v: impl Into<
            BlockAssignable<MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.automatic_input_failover_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.automatic_input_failover_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `input_settings`.\n"]
    pub fn set_input_settings(
        mut self,
        v: impl Into<BlockAssignable<MedialiveChannelInputAttachmentsElInputSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.input_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.input_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveChannelInputAttachmentsEl {
    type O = BlockAssignable<MedialiveChannelInputAttachmentsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelInputAttachmentsEl {
    #[doc = ""]
    pub input_attachment_name: PrimField<String>,
    #[doc = ""]
    pub input_id: PrimField<String>,
}
impl BuildMedialiveChannelInputAttachmentsEl {
    pub fn build(self) -> MedialiveChannelInputAttachmentsEl {
        MedialiveChannelInputAttachmentsEl {
            input_attachment_name: self.input_attachment_name,
            input_id: self.input_id,
            automatic_input_failover_settings: core::default::Default::default(),
            input_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveChannelInputAttachmentsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputAttachmentsElRef {
    fn new(shared: StackShared, base: String) -> MedialiveChannelInputAttachmentsElRef {
        MedialiveChannelInputAttachmentsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelInputAttachmentsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `input_attachment_name` after provisioning.\n"]
    pub fn input_attachment_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_attachment_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `input_id` after provisioning.\n"]
    pub fn input_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.input_id", self.base))
    }
    #[doc = "Get a reference to the value of field `automatic_input_failover_settings` after provisioning.\n"]
    pub fn automatic_input_failover_settings(
        &self,
    ) -> ListRef<MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.automatic_input_failover_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `input_settings` after provisioning.\n"]
    pub fn input_settings(&self) -> ListRef<MedialiveChannelInputAttachmentsElInputSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelInputSpecificationEl {
    codec: PrimField<String>,
    input_resolution: PrimField<String>,
    maximum_bitrate: PrimField<String>,
}
impl MedialiveChannelInputSpecificationEl {}
impl ToListMappable for MedialiveChannelInputSpecificationEl {
    type O = BlockAssignable<MedialiveChannelInputSpecificationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelInputSpecificationEl {
    #[doc = ""]
    pub codec: PrimField<String>,
    #[doc = ""]
    pub input_resolution: PrimField<String>,
    #[doc = ""]
    pub maximum_bitrate: PrimField<String>,
}
impl BuildMedialiveChannelInputSpecificationEl {
    pub fn build(self) -> MedialiveChannelInputSpecificationEl {
        MedialiveChannelInputSpecificationEl {
            codec: self.codec,
            input_resolution: self.input_resolution,
            maximum_bitrate: self.maximum_bitrate,
        }
    }
}
pub struct MedialiveChannelInputSpecificationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelInputSpecificationElRef {
    fn new(shared: StackShared, base: String) -> MedialiveChannelInputSpecificationElRef {
        MedialiveChannelInputSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelInputSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `codec` after provisioning.\n"]
    pub fn codec(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.codec", self.base))
    }
    #[doc = "Get a reference to the value of field `input_resolution` after provisioning.\n"]
    pub fn input_resolution(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_resolution", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maximum_bitrate` after provisioning.\n"]
    pub fn maximum_bitrate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_bitrate", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelMaintenanceEl {
    maintenance_day: PrimField<String>,
    maintenance_start_time: PrimField<String>,
}
impl MedialiveChannelMaintenanceEl {}
impl ToListMappable for MedialiveChannelMaintenanceEl {
    type O = BlockAssignable<MedialiveChannelMaintenanceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelMaintenanceEl {
    #[doc = ""]
    pub maintenance_day: PrimField<String>,
    #[doc = ""]
    pub maintenance_start_time: PrimField<String>,
}
impl BuildMedialiveChannelMaintenanceEl {
    pub fn build(self) -> MedialiveChannelMaintenanceEl {
        MedialiveChannelMaintenanceEl {
            maintenance_day: self.maintenance_day,
            maintenance_start_time: self.maintenance_start_time,
        }
    }
}
pub struct MedialiveChannelMaintenanceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelMaintenanceElRef {
    fn new(shared: StackShared, base: String) -> MedialiveChannelMaintenanceElRef {
        MedialiveChannelMaintenanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelMaintenanceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `maintenance_day` after provisioning.\n"]
    pub fn maintenance_day(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_day", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_start_time` after provisioning.\n"]
    pub fn maintenance_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_start_time", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveChannelTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl MedialiveChannelTimeoutsEl {
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
impl ToListMappable for MedialiveChannelTimeoutsEl {
    type O = BlockAssignable<MedialiveChannelTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelTimeoutsEl {}
impl BuildMedialiveChannelTimeoutsEl {
    pub fn build(self) -> MedialiveChannelTimeoutsEl {
        MedialiveChannelTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct MedialiveChannelTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> MedialiveChannelTimeoutsElRef {
        MedialiveChannelTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelTimeoutsElRef {
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
#[derive(Serialize)]
pub struct MedialiveChannelVpcEl {
    public_address_allocation_ids: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    subnet_ids: SetField<PrimField<String>>,
}
impl MedialiveChannelVpcEl {
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_group_ids = Some(v.into());
        self
    }
}
impl ToListMappable for MedialiveChannelVpcEl {
    type O = BlockAssignable<MedialiveChannelVpcEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveChannelVpcEl {
    #[doc = ""]
    pub public_address_allocation_ids: ListField<PrimField<String>>,
    #[doc = ""]
    pub subnet_ids: SetField<PrimField<String>>,
}
impl BuildMedialiveChannelVpcEl {
    pub fn build(self) -> MedialiveChannelVpcEl {
        MedialiveChannelVpcEl {
            public_address_allocation_ids: self.public_address_allocation_ids,
            security_group_ids: core::default::Default::default(),
            subnet_ids: self.subnet_ids,
        }
    }
}
pub struct MedialiveChannelVpcElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveChannelVpcElRef {
    fn new(shared: StackShared, base: String) -> MedialiveChannelVpcElRef {
        MedialiveChannelVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveChannelVpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.availability_zones", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_interface_ids` after provisioning.\n"]
    pub fn network_interface_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.network_interface_ids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `public_address_allocation_ids` after provisioning.\n"]
    pub fn public_address_allocation_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.public_address_allocation_ids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}
#[derive(Serialize, Default)]
struct MedialiveChannelDynamic {
    cdi_input_specification: Option<DynamicBlock<MedialiveChannelCdiInputSpecificationEl>>,
    destinations: Option<DynamicBlock<MedialiveChannelDestinationsEl>>,
    encoder_settings: Option<DynamicBlock<MedialiveChannelEncoderSettingsEl>>,
    input_attachments: Option<DynamicBlock<MedialiveChannelInputAttachmentsEl>>,
    input_specification: Option<DynamicBlock<MedialiveChannelInputSpecificationEl>>,
    maintenance: Option<DynamicBlock<MedialiveChannelMaintenanceEl>>,
    vpc: Option<DynamicBlock<MedialiveChannelVpcEl>>,
}
