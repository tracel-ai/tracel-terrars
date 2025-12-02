use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct MedialiveInputData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_security_groups: Option<ListField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destinations: Option<Vec<MedialiveInputDestinationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_devices: Option<Vec<MedialiveInputInputDevicesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    media_connect_flows: Option<Vec<MedialiveInputMediaConnectFlowsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sources: Option<Vec<MedialiveInputSourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<MedialiveInputTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc: Option<Vec<MedialiveInputVpcEl>>,
    dynamic: MedialiveInputDynamic,
}
struct MedialiveInput_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<MedialiveInputData>,
}
#[derive(Clone)]
pub struct MedialiveInput(Rc<MedialiveInput_>);
impl MedialiveInput {
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
    #[doc = "Set the field `input_security_groups`.\n"]
    pub fn set_input_security_groups(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().input_security_groups = Some(v.into());
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
    #[doc = "Set the field `destinations`.\n"]
    pub fn set_destinations(
        self,
        v: impl Into<BlockAssignable<MedialiveInputDestinationsEl>>,
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
    #[doc = "Set the field `input_devices`.\n"]
    pub fn set_input_devices(
        self,
        v: impl Into<BlockAssignable<MedialiveInputInputDevicesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().input_devices = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.input_devices = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `media_connect_flows`.\n"]
    pub fn set_media_connect_flows(
        self,
        v: impl Into<BlockAssignable<MedialiveInputMediaConnectFlowsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().media_connect_flows = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.media_connect_flows = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sources`.\n"]
    pub fn set_sources(self, v: impl Into<BlockAssignable<MedialiveInputSourcesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sources = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<MedialiveInputTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(self, v: impl Into<BlockAssignable<MedialiveInputVpcEl>>) -> Self {
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
    #[doc = "Get a reference to the value of field `attached_channels` after provisioning.\n"]
    pub fn attached_channels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attached_channels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `input_class` after provisioning.\n"]
    pub fn input_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_class", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `input_partner_ids` after provisioning.\n"]
    pub fn input_partner_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_partner_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `input_security_groups` after provisioning.\n"]
    pub fn input_security_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_security_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `input_source_type` after provisioning.\n"]
    pub fn input_source_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_source_type", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MedialiveInputTimeoutsElRef {
        MedialiveInputTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> ListRef<MedialiveInputVpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc", self.extract_ref()))
    }
}
impl Referable for MedialiveInput {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for MedialiveInput {}
impl ToListMappable for MedialiveInput {
    type O = ListRef<MedialiveInputRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for MedialiveInput_ {
    fn extract_resource_type(&self) -> String {
        "aws_medialive_input".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildMedialiveInput {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildMedialiveInput {
    pub fn build(self, stack: &mut Stack) -> MedialiveInput {
        let out = MedialiveInput(Rc::new(MedialiveInput_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(MedialiveInputData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                input_security_groups: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                role_arn: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: self.type_,
                destinations: core::default::Default::default(),
                input_devices: core::default::Default::default(),
                media_connect_flows: core::default::Default::default(),
                sources: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct MedialiveInputRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveInputRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl MedialiveInputRef {
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
    #[doc = "Get a reference to the value of field `attached_channels` after provisioning.\n"]
    pub fn attached_channels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attached_channels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `input_class` after provisioning.\n"]
    pub fn input_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_class", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `input_partner_ids` after provisioning.\n"]
    pub fn input_partner_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_partner_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `input_security_groups` after provisioning.\n"]
    pub fn input_security_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_security_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `input_source_type` after provisioning.\n"]
    pub fn input_source_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_source_type", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MedialiveInputTimeoutsElRef {
        MedialiveInputTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> ListRef<MedialiveInputVpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc", self.extract_ref()))
    }
}
#[derive(Serialize)]
pub struct MedialiveInputDestinationsEl {
    stream_name: PrimField<String>,
}
impl MedialiveInputDestinationsEl {}
impl ToListMappable for MedialiveInputDestinationsEl {
    type O = BlockAssignable<MedialiveInputDestinationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveInputDestinationsEl {
    #[doc = ""]
    pub stream_name: PrimField<String>,
}
impl BuildMedialiveInputDestinationsEl {
    pub fn build(self) -> MedialiveInputDestinationsEl {
        MedialiveInputDestinationsEl {
            stream_name: self.stream_name,
        }
    }
}
pub struct MedialiveInputDestinationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveInputDestinationsElRef {
    fn new(shared: StackShared, base: String) -> MedialiveInputDestinationsElRef {
        MedialiveInputDestinationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveInputDestinationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `stream_name` after provisioning.\n"]
    pub fn stream_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_name", self.base))
    }
}
#[derive(Serialize)]
pub struct MedialiveInputInputDevicesEl {
    id: PrimField<String>,
}
impl MedialiveInputInputDevicesEl {}
impl ToListMappable for MedialiveInputInputDevicesEl {
    type O = BlockAssignable<MedialiveInputInputDevicesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveInputInputDevicesEl {
    #[doc = ""]
    pub id: PrimField<String>,
}
impl BuildMedialiveInputInputDevicesEl {
    pub fn build(self) -> MedialiveInputInputDevicesEl {
        MedialiveInputInputDevicesEl { id: self.id }
    }
}
pub struct MedialiveInputInputDevicesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveInputInputDevicesElRef {
    fn new(shared: StackShared, base: String) -> MedialiveInputInputDevicesElRef {
        MedialiveInputInputDevicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveInputInputDevicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}
#[derive(Serialize)]
pub struct MedialiveInputMediaConnectFlowsEl {
    flow_arn: PrimField<String>,
}
impl MedialiveInputMediaConnectFlowsEl {}
impl ToListMappable for MedialiveInputMediaConnectFlowsEl {
    type O = BlockAssignable<MedialiveInputMediaConnectFlowsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveInputMediaConnectFlowsEl {
    #[doc = ""]
    pub flow_arn: PrimField<String>,
}
impl BuildMedialiveInputMediaConnectFlowsEl {
    pub fn build(self) -> MedialiveInputMediaConnectFlowsEl {
        MedialiveInputMediaConnectFlowsEl {
            flow_arn: self.flow_arn,
        }
    }
}
pub struct MedialiveInputMediaConnectFlowsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveInputMediaConnectFlowsElRef {
    fn new(shared: StackShared, base: String) -> MedialiveInputMediaConnectFlowsElRef {
        MedialiveInputMediaConnectFlowsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveInputMediaConnectFlowsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `flow_arn` after provisioning.\n"]
    pub fn flow_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.flow_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct MedialiveInputSourcesEl {
    password_param: PrimField<String>,
    url: PrimField<String>,
    username: PrimField<String>,
}
impl MedialiveInputSourcesEl {}
impl ToListMappable for MedialiveInputSourcesEl {
    type O = BlockAssignable<MedialiveInputSourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveInputSourcesEl {
    #[doc = ""]
    pub password_param: PrimField<String>,
    #[doc = ""]
    pub url: PrimField<String>,
    #[doc = ""]
    pub username: PrimField<String>,
}
impl BuildMedialiveInputSourcesEl {
    pub fn build(self) -> MedialiveInputSourcesEl {
        MedialiveInputSourcesEl {
            password_param: self.password_param,
            url: self.url,
            username: self.username,
        }
    }
}
pub struct MedialiveInputSourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveInputSourcesElRef {
    fn new(shared: StackShared, base: String) -> MedialiveInputSourcesElRef {
        MedialiveInputSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveInputSourcesElRef {
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
    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct MedialiveInputTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl MedialiveInputTimeoutsEl {
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
impl ToListMappable for MedialiveInputTimeoutsEl {
    type O = BlockAssignable<MedialiveInputTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveInputTimeoutsEl {}
impl BuildMedialiveInputTimeoutsEl {
    pub fn build(self) -> MedialiveInputTimeoutsEl {
        MedialiveInputTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct MedialiveInputTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveInputTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> MedialiveInputTimeoutsElRef {
        MedialiveInputTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveInputTimeoutsElRef {
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
pub struct MedialiveInputVpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<ListField<PrimField<String>>>,
    subnet_ids: ListField<PrimField<String>>,
}
impl MedialiveInputVpcEl {
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.security_group_ids = Some(v.into());
        self
    }
}
impl ToListMappable for MedialiveInputVpcEl {
    type O = BlockAssignable<MedialiveInputVpcEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMedialiveInputVpcEl {
    #[doc = ""]
    pub subnet_ids: ListField<PrimField<String>>,
}
impl BuildMedialiveInputVpcEl {
    pub fn build(self) -> MedialiveInputVpcEl {
        MedialiveInputVpcEl {
            security_group_ids: core::default::Default::default(),
            subnet_ids: self.subnet_ids,
        }
    }
}
pub struct MedialiveInputVpcElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MedialiveInputVpcElRef {
    fn new(shared: StackShared, base: String) -> MedialiveInputVpcElRef {
        MedialiveInputVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MedialiveInputVpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}
#[derive(Serialize, Default)]
struct MedialiveInputDynamic {
    destinations: Option<DynamicBlock<MedialiveInputDestinationsEl>>,
    input_devices: Option<DynamicBlock<MedialiveInputInputDevicesEl>>,
    media_connect_flows: Option<DynamicBlock<MedialiveInputMediaConnectFlowsEl>>,
    sources: Option<DynamicBlock<MedialiveInputSourcesEl>>,
    vpc: Option<DynamicBlock<MedialiveInputVpcEl>>,
}
