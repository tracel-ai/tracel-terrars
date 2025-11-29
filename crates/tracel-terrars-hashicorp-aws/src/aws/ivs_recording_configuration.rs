use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct IvsRecordingConfigurationData {
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
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recording_reconnect_window_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_configuration: Option<Vec<IvsRecordingConfigurationDestinationConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_configuration: Option<Vec<IvsRecordingConfigurationThumbnailConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IvsRecordingConfigurationTimeoutsEl>,
    dynamic: IvsRecordingConfigurationDynamic,
}

struct IvsRecordingConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IvsRecordingConfigurationData>,
}

#[derive(Clone)]
pub struct IvsRecordingConfiguration(Rc<IvsRecordingConfiguration_>);

impl IvsRecordingConfiguration {
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

    #[doc = "Set the field `recording_reconnect_window_seconds`.\n"]
    pub fn set_recording_reconnect_window_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().recording_reconnect_window_seconds = Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_configuration`.\n"]
    pub fn set_destination_configuration(
        self,
        v: impl Into<BlockAssignable<IvsRecordingConfigurationDestinationConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `thumbnail_configuration`.\n"]
    pub fn set_thumbnail_configuration(
        self,
        v: impl Into<BlockAssignable<IvsRecordingConfigurationThumbnailConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().thumbnail_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.thumbnail_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<IvsRecordingConfigurationTimeoutsEl>) -> Self {
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
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recording_reconnect_window_seconds` after provisioning.\n"]
    pub fn recording_reconnect_window_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.recording_reconnect_window_seconds", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_configuration` after provisioning.\n"]
    pub fn destination_configuration(&self) -> ListRef<IvsRecordingConfigurationDestinationConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `thumbnail_configuration` after provisioning.\n"]
    pub fn thumbnail_configuration(&self) -> ListRef<IvsRecordingConfigurationThumbnailConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.thumbnail_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IvsRecordingConfigurationTimeoutsElRef {
        IvsRecordingConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for IvsRecordingConfiguration {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for IvsRecordingConfiguration { }

impl ToListMappable for IvsRecordingConfiguration {
    type O = ListRef<IvsRecordingConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for IvsRecordingConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_ivs_recording_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIvsRecordingConfiguration {
    pub tf_id: String,
}

impl BuildIvsRecordingConfiguration {
    pub fn build(self, stack: &mut Stack) -> IvsRecordingConfiguration {
        let out = IvsRecordingConfiguration(Rc::new(IvsRecordingConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IvsRecordingConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                recording_reconnect_window_seconds: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                destination_configuration: core::default::Default::default(),
                thumbnail_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IvsRecordingConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for IvsRecordingConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl IvsRecordingConfigurationRef {
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
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recording_reconnect_window_seconds` after provisioning.\n"]
    pub fn recording_reconnect_window_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.recording_reconnect_window_seconds", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_configuration` after provisioning.\n"]
    pub fn destination_configuration(&self) -> ListRef<IvsRecordingConfigurationDestinationConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `thumbnail_configuration` after provisioning.\n"]
    pub fn thumbnail_configuration(&self) -> ListRef<IvsRecordingConfigurationThumbnailConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.thumbnail_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IvsRecordingConfigurationTimeoutsElRef {
        IvsRecordingConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct IvsRecordingConfigurationDestinationConfigurationElS3El {
    bucket_name: PrimField<String>,
}

impl IvsRecordingConfigurationDestinationConfigurationElS3El { }

impl ToListMappable for IvsRecordingConfigurationDestinationConfigurationElS3El {
    type O = BlockAssignable<IvsRecordingConfigurationDestinationConfigurationElS3El>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIvsRecordingConfigurationDestinationConfigurationElS3El {
    #[doc = ""]
    pub bucket_name: PrimField<String>,
}

impl BuildIvsRecordingConfigurationDestinationConfigurationElS3El {
    pub fn build(self) -> IvsRecordingConfigurationDestinationConfigurationElS3El {
        IvsRecordingConfigurationDestinationConfigurationElS3El { bucket_name: self.bucket_name }
    }
}

pub struct IvsRecordingConfigurationDestinationConfigurationElS3ElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IvsRecordingConfigurationDestinationConfigurationElS3ElRef {
    fn new(shared: StackShared, base: String) -> IvsRecordingConfigurationDestinationConfigurationElS3ElRef {
        IvsRecordingConfigurationDestinationConfigurationElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IvsRecordingConfigurationDestinationConfigurationElS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct IvsRecordingConfigurationDestinationConfigurationElDynamic {
    s3: Option<DynamicBlock<IvsRecordingConfigurationDestinationConfigurationElS3El>>,
}

#[derive(Serialize)]
pub struct IvsRecordingConfigurationDestinationConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<Vec<IvsRecordingConfigurationDestinationConfigurationElS3El>>,
    dynamic: IvsRecordingConfigurationDestinationConfigurationElDynamic,
}

impl IvsRecordingConfigurationDestinationConfigurationEl {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl Into<BlockAssignable<IvsRecordingConfigurationDestinationConfigurationElS3El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3 = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for IvsRecordingConfigurationDestinationConfigurationEl {
    type O = BlockAssignable<IvsRecordingConfigurationDestinationConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIvsRecordingConfigurationDestinationConfigurationEl {}

impl BuildIvsRecordingConfigurationDestinationConfigurationEl {
    pub fn build(self) -> IvsRecordingConfigurationDestinationConfigurationEl {
        IvsRecordingConfigurationDestinationConfigurationEl {
            s3: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct IvsRecordingConfigurationDestinationConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IvsRecordingConfigurationDestinationConfigurationElRef {
    fn new(shared: StackShared, base: String) -> IvsRecordingConfigurationDestinationConfigurationElRef {
        IvsRecordingConfigurationDestinationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IvsRecordingConfigurationDestinationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> ListRef<IvsRecordingConfigurationDestinationConfigurationElS3ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize)]
pub struct IvsRecordingConfigurationThumbnailConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    recording_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_interval_seconds: Option<PrimField<f64>>,
}

impl IvsRecordingConfigurationThumbnailConfigurationEl {
    #[doc = "Set the field `recording_mode`.\n"]
    pub fn set_recording_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.recording_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `target_interval_seconds`.\n"]
    pub fn set_target_interval_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target_interval_seconds = Some(v.into());
        self
    }
}

impl ToListMappable for IvsRecordingConfigurationThumbnailConfigurationEl {
    type O = BlockAssignable<IvsRecordingConfigurationThumbnailConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIvsRecordingConfigurationThumbnailConfigurationEl {}

impl BuildIvsRecordingConfigurationThumbnailConfigurationEl {
    pub fn build(self) -> IvsRecordingConfigurationThumbnailConfigurationEl {
        IvsRecordingConfigurationThumbnailConfigurationEl {
            recording_mode: core::default::Default::default(),
            target_interval_seconds: core::default::Default::default(),
        }
    }
}

pub struct IvsRecordingConfigurationThumbnailConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IvsRecordingConfigurationThumbnailConfigurationElRef {
    fn new(shared: StackShared, base: String) -> IvsRecordingConfigurationThumbnailConfigurationElRef {
        IvsRecordingConfigurationThumbnailConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IvsRecordingConfigurationThumbnailConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `recording_mode` after provisioning.\n"]
    pub fn recording_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.recording_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `target_interval_seconds` after provisioning.\n"]
    pub fn target_interval_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_interval_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct IvsRecordingConfigurationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl IvsRecordingConfigurationTimeoutsEl {
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
}

impl ToListMappable for IvsRecordingConfigurationTimeoutsEl {
    type O = BlockAssignable<IvsRecordingConfigurationTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIvsRecordingConfigurationTimeoutsEl {}

impl BuildIvsRecordingConfigurationTimeoutsEl {
    pub fn build(self) -> IvsRecordingConfigurationTimeoutsEl {
        IvsRecordingConfigurationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct IvsRecordingConfigurationTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IvsRecordingConfigurationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> IvsRecordingConfigurationTimeoutsElRef {
        IvsRecordingConfigurationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IvsRecordingConfigurationTimeoutsElRef {
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
}

#[derive(Serialize, Default)]
struct IvsRecordingConfigurationDynamic {
    destination_configuration: Option<DynamicBlock<IvsRecordingConfigurationDestinationConfigurationEl>>,
    thumbnail_configuration: Option<DynamicBlock<IvsRecordingConfigurationThumbnailConfigurationEl>>,
}
