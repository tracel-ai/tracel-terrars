use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct MedialiveMultiplexData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    availability_zones: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_multiplex: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multiplex_settings: Option<Vec<MedialiveMultiplexMultiplexSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<MedialiveMultiplexTimeoutsEl>,
    dynamic: MedialiveMultiplexDynamic,
}
struct MedialiveMultiplex_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<MedialiveMultiplexData>,
}
#[derive(Clone)]
pub struct MedialiveMultiplex(Rc<MedialiveMultiplex_>);
impl MedialiveMultiplex {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `start_multiplex`.\n"]
    pub fn set_start_multiplex(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().start_multiplex = Some(v.into());
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
    #[doc = "Set the field `multiplex_settings`.\n"]
    pub fn set_multiplex_settings(
        self,
        v: impl Into<BlockAssignable<MedialiveMultiplexMultiplexSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().multiplex_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.multiplex_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<MedialiveMultiplexTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.availability_zones", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `start_multiplex` after provisioning.\n"]
    pub fn start_multiplex(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_multiplex", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `multiplex_settings` after provisioning.\n"]
    pub fn multiplex_settings(&self) -> ListRef<MedialiveMultiplexMultiplexSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multiplex_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MedialiveMultiplexTimeoutsElRef {
        MedialiveMultiplexTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for MedialiveMultiplex {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for MedialiveMultiplex {}
impl ToListMappable for MedialiveMultiplex {
    type O = ListRef<MedialiveMultiplexRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for MedialiveMultiplex_ {
    fn extract_resource_type(&self) -> String {
        "aws_medialive_multiplex".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildMedialiveMultiplex {
    pub tf_id: String,
    #[doc = ""]
    pub availability_zones: ListField<PrimField<String>>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildMedialiveMultiplex {
    pub fn build(self, stack: &mut Stack) -> MedialiveMultiplex {
        let out = MedialiveMultiplex(Rc::new(MedialiveMultiplex_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(MedialiveMultiplexData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                availability_zones: self.availability_zones,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                start_multiplex: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                multiplex_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct MedialiveMultiplexRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveMultiplexRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl MedialiveMultiplexRef {
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
    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.availability_zones", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `start_multiplex` after provisioning.\n"]
    pub fn start_multiplex(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_multiplex", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `multiplex_settings` after provisioning.\n"]
    pub fn multiplex_settings(&self) -> ListRef<MedialiveMultiplexMultiplexSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multiplex_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MedialiveMultiplexTimeoutsElRef {
        MedialiveMultiplexTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveMultiplexMultiplexSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_video_buffer_delay_milliseconds: Option<PrimField<f64>>,
    transport_stream_bitrate: PrimField<f64>,
    transport_stream_id: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transport_stream_reserved_bitrate: Option<PrimField<f64>>,
}
impl MedialiveMultiplexMultiplexSettingsEl {
    #[doc = "Set the field `maximum_video_buffer_delay_milliseconds`.\n"]
    pub fn set_maximum_video_buffer_delay_milliseconds(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.maximum_video_buffer_delay_milliseconds = Some(v.into());
        self
    }
    #[doc = "Set the field `transport_stream_reserved_bitrate`.\n"]
    pub fn set_transport_stream_reserved_bitrate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.transport_stream_reserved_bitrate = Some(v.into());
        self
    }
}
impl ToListMappable for MedialiveMultiplexMultiplexSettingsEl {
    type O = BlockAssignable<MedialiveMultiplexMultiplexSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveMultiplexMultiplexSettingsEl {
    #[doc = ""]
    pub transport_stream_bitrate: PrimField<f64>,
    #[doc = ""]
    pub transport_stream_id: PrimField<f64>,
}
impl BuildMedialiveMultiplexMultiplexSettingsEl {
    pub fn build(self) -> MedialiveMultiplexMultiplexSettingsEl {
        MedialiveMultiplexMultiplexSettingsEl {
            maximum_video_buffer_delay_milliseconds: core::default::Default::default(),
            transport_stream_bitrate: self.transport_stream_bitrate,
            transport_stream_id: self.transport_stream_id,
            transport_stream_reserved_bitrate: core::default::Default::default(),
        }
    }
}
pub struct MedialiveMultiplexMultiplexSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveMultiplexMultiplexSettingsElRef {
    fn new(shared: StackShared, base: String) -> MedialiveMultiplexMultiplexSettingsElRef {
        MedialiveMultiplexMultiplexSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveMultiplexMultiplexSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `maximum_video_buffer_delay_milliseconds` after provisioning.\n"]
    pub fn maximum_video_buffer_delay_milliseconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_video_buffer_delay_milliseconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `transport_stream_bitrate` after provisioning.\n"]
    pub fn transport_stream_bitrate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transport_stream_bitrate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `transport_stream_id` after provisioning.\n"]
    pub fn transport_stream_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transport_stream_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `transport_stream_reserved_bitrate` after provisioning.\n"]
    pub fn transport_stream_reserved_bitrate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transport_stream_reserved_bitrate", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MedialiveMultiplexTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl MedialiveMultiplexTimeoutsEl {
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
impl ToListMappable for MedialiveMultiplexTimeoutsEl {
    type O = BlockAssignable<MedialiveMultiplexTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveMultiplexTimeoutsEl {}
impl BuildMedialiveMultiplexTimeoutsEl {
    pub fn build(self) -> MedialiveMultiplexTimeoutsEl {
        MedialiveMultiplexTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct MedialiveMultiplexTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveMultiplexTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> MedialiveMultiplexTimeoutsElRef {
        MedialiveMultiplexTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveMultiplexTimeoutsElRef {
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
struct MedialiveMultiplexDynamic {
    multiplex_settings: Option<DynamicBlock<MedialiveMultiplexMultiplexSettingsEl>>,
}
