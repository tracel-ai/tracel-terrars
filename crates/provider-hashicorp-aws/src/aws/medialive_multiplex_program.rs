use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct MedialiveMultiplexProgramData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    multiplex_id: PrimField<String>,
    program_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multiplex_program_settings: Option<Vec<MedialiveMultiplexProgramMultiplexProgramSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<MedialiveMultiplexProgramTimeoutsEl>,
    dynamic: MedialiveMultiplexProgramDynamic,
}
struct MedialiveMultiplexProgram_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<MedialiveMultiplexProgramData>,
}
#[derive(Clone)]
pub struct MedialiveMultiplexProgram(Rc<MedialiveMultiplexProgram_>);
impl MedialiveMultiplexProgram {
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
    #[doc = "Set the field `multiplex_program_settings`.\n"]
    pub fn set_multiplex_program_settings(
        self,
        v: impl Into<BlockAssignable<MedialiveMultiplexProgramMultiplexProgramSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().multiplex_program_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.multiplex_program_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<MedialiveMultiplexProgramTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `multiplex_id` after provisioning.\n"]
    pub fn multiplex_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multiplex_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `program_name` after provisioning.\n"]
    pub fn program_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.program_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multiplex_program_settings` after provisioning.\n"]
    pub fn multiplex_program_settings(
        &self,
    ) -> ListRef<MedialiveMultiplexProgramMultiplexProgramSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multiplex_program_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MedialiveMultiplexProgramTimeoutsElRef {
        MedialiveMultiplexProgramTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for MedialiveMultiplexProgram {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for MedialiveMultiplexProgram {}
impl ToListMappable for MedialiveMultiplexProgram {
    type O = ListRef<MedialiveMultiplexProgramRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for MedialiveMultiplexProgram_ {
    fn extract_resource_type(&self) -> String {
        "aws_medialive_multiplex_program".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildMedialiveMultiplexProgram {
    pub tf_id: String,
    #[doc = ""]
    pub multiplex_id: PrimField<String>,
    #[doc = ""]
    pub program_name: PrimField<String>,
}
impl BuildMedialiveMultiplexProgram {
    pub fn build(self, stack: &mut Stack) -> MedialiveMultiplexProgram {
        let out = MedialiveMultiplexProgram(Rc::new(MedialiveMultiplexProgram_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(MedialiveMultiplexProgramData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                multiplex_id: self.multiplex_id,
                program_name: self.program_name,
                region: core::default::Default::default(),
                multiplex_program_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct MedialiveMultiplexProgramRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveMultiplexProgramRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl MedialiveMultiplexProgramRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `multiplex_id` after provisioning.\n"]
    pub fn multiplex_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multiplex_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `program_name` after provisioning.\n"]
    pub fn program_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.program_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multiplex_program_settings` after provisioning.\n"]
    pub fn multiplex_program_settings(
        &self,
    ) -> ListRef<MedialiveMultiplexProgramMultiplexProgramSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multiplex_program_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MedialiveMultiplexProgramTimeoutsElRef {
        MedialiveMultiplexProgramTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorEl {
    provider_name: PrimField<String>,
    service_name: PrimField<String>,
}
impl MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorEl {}
impl ToListMappable for MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorEl {
    type O =
        BlockAssignable<MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorEl {
    #[doc = ""]
    pub provider_name: PrimField<String>,
    #[doc = ""]
    pub service_name: PrimField<String>,
}
impl BuildMedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorEl {
    pub fn build(self) -> MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorEl {
        MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorEl {
            provider_name: self.provider_name,
            service_name: self.service_name,
        }
    }
}
pub struct MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorElRef {
        MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `provider_name` after provisioning.\n"]
    pub fn provider_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provider_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
}
#[derive(Serialize)]
pub struct MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_bitrate: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_bitrate: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
}
impl MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsEl {
    #[doc = "Set the field `maximum_bitrate`.\n"]
    pub fn set_maximum_bitrate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_bitrate = Some(v.into());
        self
    }
    #[doc = "Set the field `minimum_bitrate`.\n"]
    pub fn set_minimum_bitrate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minimum_bitrate = Some(v.into());
        self
    }
    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }
}
impl ToListMappable
    for MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsEl
{
    type O = BlockAssignable<
        MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsEl
{}
impl BuildMedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsEl {
        MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsEl {
            maximum_bitrate: core::default::Default::default(),
            minimum_bitrate: core::default::Default::default(),
            priority: core::default::Default::default(),
        }
    }
}
pub struct MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsElRef
    {
        MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `maximum_bitrate` after provisioning.\n"]
    pub fn maximum_bitrate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_bitrate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `minimum_bitrate` after provisioning.\n"]
    pub fn minimum_bitrate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minimum_bitrate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
}
#[derive(Serialize, Default)]
struct MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElDynamic {
    statmux_settings: Option<
        DynamicBlock<
            MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    constant_bitrate: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statmux_settings: Option<
        Vec<MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsEl>,
    >,
    dynamic: MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElDynamic,
}
impl MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsEl {
    #[doc = "Set the field `constant_bitrate`.\n"]
    pub fn set_constant_bitrate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.constant_bitrate = Some(v.into());
        self
    }
    #[doc = "Set the field `statmux_settings`.\n"]
    pub fn set_statmux_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.statmux_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.statmux_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsEl {
    type O = BlockAssignable<MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsEl {}
impl BuildMedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsEl {
    pub fn build(self) -> MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsEl {
        MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsEl {
            constant_bitrate: core::default::Default::default(),
            statmux_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElRef {
        MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `constant_bitrate` after provisioning.\n"]
    pub fn constant_bitrate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.constant_bitrate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `statmux_settings` after provisioning.\n"]
    pub fn statmux_settings(
        &self,
    ) -> ListRef<
        MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.statmux_settings", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MedialiveMultiplexProgramMultiplexProgramSettingsElDynamic {
    service_descriptor: Option<
        DynamicBlock<MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorEl>,
    >,
    video_settings:
        Option<DynamicBlock<MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsEl>>,
}
#[derive(Serialize)]
pub struct MedialiveMultiplexProgramMultiplexProgramSettingsEl {
    preferred_channel_pipeline: PrimField<String>,
    program_number: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_descriptor:
        Option<Vec<MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_settings: Option<Vec<MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsEl>>,
    dynamic: MedialiveMultiplexProgramMultiplexProgramSettingsElDynamic,
}
impl MedialiveMultiplexProgramMultiplexProgramSettingsEl {
    #[doc = "Set the field `service_descriptor`.\n"]
    pub fn set_service_descriptor(
        mut self,
        v: impl Into<
            BlockAssignable<MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_descriptor = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_descriptor = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `video_settings`.\n"]
    pub fn set_video_settings(
        mut self,
        v: impl Into<
            BlockAssignable<MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.video_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.video_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MedialiveMultiplexProgramMultiplexProgramSettingsEl {
    type O = BlockAssignable<MedialiveMultiplexProgramMultiplexProgramSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveMultiplexProgramMultiplexProgramSettingsEl {
    #[doc = ""]
    pub preferred_channel_pipeline: PrimField<String>,
    #[doc = ""]
    pub program_number: PrimField<f64>,
}
impl BuildMedialiveMultiplexProgramMultiplexProgramSettingsEl {
    pub fn build(self) -> MedialiveMultiplexProgramMultiplexProgramSettingsEl {
        MedialiveMultiplexProgramMultiplexProgramSettingsEl {
            preferred_channel_pipeline: self.preferred_channel_pipeline,
            program_number: self.program_number,
            service_descriptor: core::default::Default::default(),
            video_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MedialiveMultiplexProgramMultiplexProgramSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveMultiplexProgramMultiplexProgramSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MedialiveMultiplexProgramMultiplexProgramSettingsElRef {
        MedialiveMultiplexProgramMultiplexProgramSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveMultiplexProgramMultiplexProgramSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `preferred_channel_pipeline` after provisioning.\n"]
    pub fn preferred_channel_pipeline(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preferred_channel_pipeline", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `program_number` after provisioning.\n"]
    pub fn program_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.program_number", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_descriptor` after provisioning.\n"]
    pub fn service_descriptor(
        &self,
    ) -> ListRef<MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_descriptor", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `video_settings` after provisioning.\n"]
    pub fn video_settings(
        &self,
    ) -> ListRef<MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.video_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveMultiplexProgramTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
}
impl MedialiveMultiplexProgramTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
}
impl ToListMappable for MedialiveMultiplexProgramTimeoutsEl {
    type O = BlockAssignable<MedialiveMultiplexProgramTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveMultiplexProgramTimeoutsEl {}
impl BuildMedialiveMultiplexProgramTimeoutsEl {
    pub fn build(self) -> MedialiveMultiplexProgramTimeoutsEl {
        MedialiveMultiplexProgramTimeoutsEl {
            create: core::default::Default::default(),
        }
    }
}
pub struct MedialiveMultiplexProgramTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveMultiplexProgramTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> MedialiveMultiplexProgramTimeoutsElRef {
        MedialiveMultiplexProgramTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveMultiplexProgramTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}
#[derive(Serialize, Default)]
struct MedialiveMultiplexProgramDynamic {
    multiplex_program_settings:
        Option<DynamicBlock<MedialiveMultiplexProgramMultiplexProgramSettingsEl>>,
}
