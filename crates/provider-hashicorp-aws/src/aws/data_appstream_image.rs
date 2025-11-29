use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAppstreamImageData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    most_recent: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

struct DataAppstreamImage_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAppstreamImageData>,
}

#[derive(Clone)]
pub struct DataAppstreamImage(Rc<DataAppstreamImage_>);

impl DataAppstreamImage {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().arn = Some(v.into());
        self
    }

    #[doc = "Set the field `most_recent`.\n"]
    pub fn set_most_recent(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().most_recent = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc = "Set the field `name_regex`.\n"]
    pub fn set_name_regex(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_regex = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `applications` after provisioning.\n"]
    pub fn applications(&self) -> ListRef<DataAppstreamImageApplicationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.applications", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `appstream_agent_version` after provisioning.\n"]
    pub fn appstream_agent_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.appstream_agent_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `base_image_arn` after provisioning.\n"]
    pub fn base_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base_image_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_builder_name` after provisioning.\n"]
    pub fn image_builder_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_builder_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_builder_supported` after provisioning.\n"]
    pub fn image_builder_supported(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_builder_supported", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_permissions` after provisioning.\n"]
    pub fn image_permissions(&self) -> ListRef<DataAppstreamImageImagePermissionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image_permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `most_recent` after provisioning.\n"]
    pub fn most_recent(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.most_recent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_regex` after provisioning.\n"]
    pub fn name_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_regex", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_base_image_released_date` after provisioning.\n"]
    pub fn public_base_image_released_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_base_image_released_date", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `state_change_reason` after provisioning.\n"]
    pub fn state_change_reason(&self) -> ListRef<DataAppstreamImageStateChangeReasonElRef> {
        ListRef::new(self.shared().clone(), format!("{}.state_change_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl Referable for DataAppstreamImage {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataAppstreamImage { }

impl ToListMappable for DataAppstreamImage {
    type O = ListRef<DataAppstreamImageRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataAppstreamImage_ {
    fn extract_datasource_type(&self) -> String {
        "aws_appstream_image".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAppstreamImage {
    pub tf_id: String,
}

impl BuildDataAppstreamImage {
    pub fn build(self, stack: &mut Stack) -> DataAppstreamImage {
        let out = DataAppstreamImage(Rc::new(DataAppstreamImage_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAppstreamImageData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                arn: core::default::Default::default(),
                most_recent: core::default::Default::default(),
                name: core::default::Default::default(),
                name_regex: core::default::Default::default(),
                region: core::default::Default::default(),
                type_: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAppstreamImageRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppstreamImageRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataAppstreamImageRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `applications` after provisioning.\n"]
    pub fn applications(&self) -> ListRef<DataAppstreamImageApplicationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.applications", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `appstream_agent_version` after provisioning.\n"]
    pub fn appstream_agent_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.appstream_agent_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `base_image_arn` after provisioning.\n"]
    pub fn base_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base_image_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_builder_name` after provisioning.\n"]
    pub fn image_builder_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_builder_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_builder_supported` after provisioning.\n"]
    pub fn image_builder_supported(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_builder_supported", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_permissions` after provisioning.\n"]
    pub fn image_permissions(&self) -> ListRef<DataAppstreamImageImagePermissionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image_permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `most_recent` after provisioning.\n"]
    pub fn most_recent(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.most_recent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_regex` after provisioning.\n"]
    pub fn name_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_regex", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_base_image_released_date` after provisioning.\n"]
    pub fn public_base_image_released_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_base_image_released_date", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `state_change_reason` after provisioning.\n"]
    pub fn state_change_reason(&self) -> ListRef<DataAppstreamImageStateChangeReasonElRef> {
        ListRef::new(self.shared().clone(), format!("{}.state_change_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAppstreamImageApplicationsElIconS3LocationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_key: Option<PrimField<String>>,
}

impl DataAppstreamImageApplicationsElIconS3LocationEl {
    #[doc = "Set the field `s3_bucket`.\n"]
    pub fn set_s3_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_bucket = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_key`.\n"]
    pub fn set_s3_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_key = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppstreamImageApplicationsElIconS3LocationEl {
    type O = BlockAssignable<DataAppstreamImageApplicationsElIconS3LocationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppstreamImageApplicationsElIconS3LocationEl {}

impl BuildDataAppstreamImageApplicationsElIconS3LocationEl {
    pub fn build(self) -> DataAppstreamImageApplicationsElIconS3LocationEl {
        DataAppstreamImageApplicationsElIconS3LocationEl {
            s3_bucket: core::default::Default::default(),
            s3_key: core::default::Default::default(),
        }
    }
}

pub struct DataAppstreamImageApplicationsElIconS3LocationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppstreamImageApplicationsElIconS3LocationElRef {
    fn new(shared: StackShared, base: String) -> DataAppstreamImageApplicationsElIconS3LocationElRef {
        DataAppstreamImageApplicationsElIconS3LocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppstreamImageApplicationsElIconS3LocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_key` after provisioning.\n"]
    pub fn s3_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppstreamImageApplicationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    app_block_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_s3_location: Option<ListField<DataAppstreamImageApplicationsElIconS3LocationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_families: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_parameters: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platforms: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    working_directory: Option<PrimField<String>>,
}

impl DataAppstreamImageApplicationsEl {
    #[doc = "Set the field `app_block_arn`.\n"]
    pub fn set_app_block_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.app_block_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `created_time`.\n"]
    pub fn set_created_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_time = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `icon_s3_location`.\n"]
    pub fn set_icon_s3_location(
        mut self,
        v: impl Into<ListField<DataAppstreamImageApplicationsElIconS3LocationEl>>,
    ) -> Self {
        self.icon_s3_location = Some(v.into());
        self
    }

    #[doc = "Set the field `icon_url`.\n"]
    pub fn set_icon_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.icon_url = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_families`.\n"]
    pub fn set_instance_families(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.instance_families = Some(v.into());
        self
    }

    #[doc = "Set the field `launch_parameters`.\n"]
    pub fn set_launch_parameters(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.launch_parameters = Some(v.into());
        self
    }

    #[doc = "Set the field `launch_path`.\n"]
    pub fn set_launch_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.launch_path = Some(v.into());
        self
    }

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.metadata = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `platforms`.\n"]
    pub fn set_platforms(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.platforms = Some(v.into());
        self
    }

    #[doc = "Set the field `working_directory`.\n"]
    pub fn set_working_directory(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.working_directory = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppstreamImageApplicationsEl {
    type O = BlockAssignable<DataAppstreamImageApplicationsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppstreamImageApplicationsEl {}

impl BuildDataAppstreamImageApplicationsEl {
    pub fn build(self) -> DataAppstreamImageApplicationsEl {
        DataAppstreamImageApplicationsEl {
            app_block_arn: core::default::Default::default(),
            arn: core::default::Default::default(),
            created_time: core::default::Default::default(),
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            enabled: core::default::Default::default(),
            icon_s3_location: core::default::Default::default(),
            icon_url: core::default::Default::default(),
            instance_families: core::default::Default::default(),
            launch_parameters: core::default::Default::default(),
            launch_path: core::default::Default::default(),
            metadata: core::default::Default::default(),
            name: core::default::Default::default(),
            platforms: core::default::Default::default(),
            working_directory: core::default::Default::default(),
        }
    }
}

pub struct DataAppstreamImageApplicationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppstreamImageApplicationsElRef {
    fn new(shared: StackShared, base: String) -> DataAppstreamImageApplicationsElRef {
        DataAppstreamImageApplicationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppstreamImageApplicationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_block_arn` after provisioning.\n"]
    pub fn app_block_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_block_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `icon_s3_location` after provisioning.\n"]
    pub fn icon_s3_location(&self) -> ListRef<DataAppstreamImageApplicationsElIconS3LocationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.icon_s3_location", self.base))
    }

    #[doc = "Get a reference to the value of field `icon_url` after provisioning.\n"]
    pub fn icon_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.icon_url", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_families` after provisioning.\n"]
    pub fn instance_families(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.instance_families", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_parameters` after provisioning.\n"]
    pub fn launch_parameters(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.launch_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_path` after provisioning.\n"]
    pub fn launch_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.launch_path", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `platforms` after provisioning.\n"]
    pub fn platforms(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.platforms", self.base))
    }

    #[doc = "Get a reference to the value of field `working_directory` after provisioning.\n"]
    pub fn working_directory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.working_directory", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppstreamImageImagePermissionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_fleet: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_image_builder: Option<PrimField<bool>>,
}

impl DataAppstreamImageImagePermissionsEl {
    #[doc = "Set the field `allow_fleet`.\n"]
    pub fn set_allow_fleet(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_fleet = Some(v.into());
        self
    }

    #[doc = "Set the field `allow_image_builder`.\n"]
    pub fn set_allow_image_builder(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_image_builder = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppstreamImageImagePermissionsEl {
    type O = BlockAssignable<DataAppstreamImageImagePermissionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppstreamImageImagePermissionsEl {}

impl BuildDataAppstreamImageImagePermissionsEl {
    pub fn build(self) -> DataAppstreamImageImagePermissionsEl {
        DataAppstreamImageImagePermissionsEl {
            allow_fleet: core::default::Default::default(),
            allow_image_builder: core::default::Default::default(),
        }
    }
}

pub struct DataAppstreamImageImagePermissionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppstreamImageImagePermissionsElRef {
    fn new(shared: StackShared, base: String) -> DataAppstreamImageImagePermissionsElRef {
        DataAppstreamImageImagePermissionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppstreamImageImagePermissionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_fleet` after provisioning.\n"]
    pub fn allow_fleet(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_fleet", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_image_builder` after provisioning.\n"]
    pub fn allow_image_builder(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_image_builder", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppstreamImageStateChangeReasonEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}

impl DataAppstreamImageStateChangeReasonEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.code = Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppstreamImageStateChangeReasonEl {
    type O = BlockAssignable<DataAppstreamImageStateChangeReasonEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppstreamImageStateChangeReasonEl {}

impl BuildDataAppstreamImageStateChangeReasonEl {
    pub fn build(self) -> DataAppstreamImageStateChangeReasonEl {
        DataAppstreamImageStateChangeReasonEl {
            code: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

pub struct DataAppstreamImageStateChangeReasonElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppstreamImageStateChangeReasonElRef {
    fn new(shared: StackShared, base: String) -> DataAppstreamImageStateChangeReasonElRef {
        DataAppstreamImageStateChangeReasonElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppstreamImageStateChangeReasonElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
