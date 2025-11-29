use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ImagebuilderImageRecipeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ami_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    parent_image: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_data_base64: Option<PrimField<String>>,
    version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    working_directory: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_device_mapping: Option<Vec<ImagebuilderImageRecipeBlockDeviceMappingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    component: Option<Vec<ImagebuilderImageRecipeComponentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    systems_manager_agent: Option<Vec<ImagebuilderImageRecipeSystemsManagerAgentEl>>,
    dynamic: ImagebuilderImageRecipeDynamic,
}

struct ImagebuilderImageRecipe_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ImagebuilderImageRecipeData>,
}

#[derive(Clone)]
pub struct ImagebuilderImageRecipe(Rc<ImagebuilderImageRecipe_>);

impl ImagebuilderImageRecipe {
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

    #[doc = "Set the field `ami_tags`.\n"]
    pub fn set_ami_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().ami_tags = Some(v.into());
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

    #[doc = "Set the field `user_data_base64`.\n"]
    pub fn set_user_data_base64(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user_data_base64 = Some(v.into());
        self
    }

    #[doc = "Set the field `working_directory`.\n"]
    pub fn set_working_directory(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().working_directory = Some(v.into());
        self
    }

    #[doc = "Set the field `block_device_mapping`.\n"]
    pub fn set_block_device_mapping(
        self,
        v: impl Into<BlockAssignable<ImagebuilderImageRecipeBlockDeviceMappingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().block_device_mapping = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.block_device_mapping = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `component`.\n"]
    pub fn set_component(self, v: impl Into<BlockAssignable<ImagebuilderImageRecipeComponentEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().component = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.component = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `systems_manager_agent`.\n"]
    pub fn set_systems_manager_agent(
        self,
        v: impl Into<BlockAssignable<ImagebuilderImageRecipeSystemsManagerAgentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().systems_manager_agent = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.systems_manager_agent = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `ami_tags` after provisioning.\n"]
    pub fn ami_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.ami_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_image` after provisioning.\n"]
    pub fn parent_image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent_image", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_data_base64` after provisioning.\n"]
    pub fn user_data_base64(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_data_base64", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `working_directory` after provisioning.\n"]
    pub fn working_directory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.working_directory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> ListRef<ImagebuilderImageRecipeComponentElRef> {
        ListRef::new(self.shared().clone(), format!("{}.component", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `systems_manager_agent` after provisioning.\n"]
    pub fn systems_manager_agent(&self) -> ListRef<ImagebuilderImageRecipeSystemsManagerAgentElRef> {
        ListRef::new(self.shared().clone(), format!("{}.systems_manager_agent", self.extract_ref()))
    }
}

impl Referable for ImagebuilderImageRecipe {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ImagebuilderImageRecipe { }

impl ToListMappable for ImagebuilderImageRecipe {
    type O = ListRef<ImagebuilderImageRecipeRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ImagebuilderImageRecipe_ {
    fn extract_resource_type(&self) -> String {
        "aws_imagebuilder_image_recipe".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildImagebuilderImageRecipe {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub parent_image: PrimField<String>,
    #[doc = ""]
    pub version: PrimField<String>,
}

impl BuildImagebuilderImageRecipe {
    pub fn build(self, stack: &mut Stack) -> ImagebuilderImageRecipe {
        let out = ImagebuilderImageRecipe(Rc::new(ImagebuilderImageRecipe_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ImagebuilderImageRecipeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                ami_tags: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                parent_image: self.parent_image,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                user_data_base64: core::default::Default::default(),
                version: self.version,
                working_directory: core::default::Default::default(),
                block_device_mapping: core::default::Default::default(),
                component: core::default::Default::default(),
                systems_manager_agent: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ImagebuilderImageRecipeRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderImageRecipeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl ImagebuilderImageRecipeRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ami_tags` after provisioning.\n"]
    pub fn ami_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.ami_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_image` after provisioning.\n"]
    pub fn parent_image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent_image", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_data_base64` after provisioning.\n"]
    pub fn user_data_base64(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_data_base64", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `working_directory` after provisioning.\n"]
    pub fn working_directory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.working_directory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> ListRef<ImagebuilderImageRecipeComponentElRef> {
        ListRef::new(self.shared().clone(), format!("{}.component", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `systems_manager_agent` after provisioning.\n"]
    pub fn systems_manager_agent(&self) -> ListRef<ImagebuilderImageRecipeSystemsManagerAgentElRef> {
        ListRef::new(self.shared().clone(), format!("{}.systems_manager_agent", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderImageRecipeBlockDeviceMappingElEbsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_on_termination: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encrypted: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_type: Option<PrimField<String>>,
}

impl ImagebuilderImageRecipeBlockDeviceMappingElEbsEl {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete_on_termination = Some(v.into());
        self
    }

    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encrypted = Some(v.into());
        self
    }

    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iops = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_id`.\n"]
    pub fn set_snapshot_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.snapshot_id = Some(v.into());
        self
    }

    #[doc = "Set the field `throughput`.\n"]
    pub fn set_throughput(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.throughput = Some(v.into());
        self
    }

    #[doc = "Set the field `volume_size`.\n"]
    pub fn set_volume_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volume_size = Some(v.into());
        self
    }

    #[doc = "Set the field `volume_type`.\n"]
    pub fn set_volume_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.volume_type = Some(v.into());
        self
    }
}

impl ToListMappable for ImagebuilderImageRecipeBlockDeviceMappingElEbsEl {
    type O = BlockAssignable<ImagebuilderImageRecipeBlockDeviceMappingElEbsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImageRecipeBlockDeviceMappingElEbsEl {}

impl BuildImagebuilderImageRecipeBlockDeviceMappingElEbsEl {
    pub fn build(self) -> ImagebuilderImageRecipeBlockDeviceMappingElEbsEl {
        ImagebuilderImageRecipeBlockDeviceMappingElEbsEl {
            delete_on_termination: core::default::Default::default(),
            encrypted: core::default::Default::default(),
            iops: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            snapshot_id: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderImageRecipeBlockDeviceMappingElEbsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderImageRecipeBlockDeviceMappingElEbsElRef {
    fn new(shared: StackShared, base: String) -> ImagebuilderImageRecipeBlockDeviceMappingElEbsElRef {
        ImagebuilderImageRecipeBlockDeviceMappingElEbsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImageRecipeBlockDeviceMappingElEbsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_on_termination", self.base))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.base))
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_id` after provisioning.\n"]
    pub fn snapshot_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_id", self.base))
    }

    #[doc = "Get a reference to the value of field `throughput` after provisioning.\n"]
    pub fn throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_size", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderImageRecipeBlockDeviceMappingElDynamic {
    ebs: Option<DynamicBlock<ImagebuilderImageRecipeBlockDeviceMappingElEbsEl>>,
}

#[derive(Serialize)]
pub struct ImagebuilderImageRecipeBlockDeviceMappingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    device_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_device: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs: Option<Vec<ImagebuilderImageRecipeBlockDeviceMappingElEbsEl>>,
    dynamic: ImagebuilderImageRecipeBlockDeviceMappingElDynamic,
}

impl ImagebuilderImageRecipeBlockDeviceMappingEl {
    #[doc = "Set the field `device_name`.\n"]
    pub fn set_device_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_name = Some(v.into());
        self
    }

    #[doc = "Set the field `no_device`.\n"]
    pub fn set_no_device(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.no_device = Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_name`.\n"]
    pub fn set_virtual_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.virtual_name = Some(v.into());
        self
    }

    #[doc = "Set the field `ebs`.\n"]
    pub fn set_ebs(mut self, v: impl Into<BlockAssignable<ImagebuilderImageRecipeBlockDeviceMappingElEbsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ebs = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ebs = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ImagebuilderImageRecipeBlockDeviceMappingEl {
    type O = BlockAssignable<ImagebuilderImageRecipeBlockDeviceMappingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImageRecipeBlockDeviceMappingEl {}

impl BuildImagebuilderImageRecipeBlockDeviceMappingEl {
    pub fn build(self) -> ImagebuilderImageRecipeBlockDeviceMappingEl {
        ImagebuilderImageRecipeBlockDeviceMappingEl {
            device_name: core::default::Default::default(),
            no_device: core::default::Default::default(),
            virtual_name: core::default::Default::default(),
            ebs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ImagebuilderImageRecipeBlockDeviceMappingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderImageRecipeBlockDeviceMappingElRef {
    fn new(shared: StackShared, base: String) -> ImagebuilderImageRecipeBlockDeviceMappingElRef {
        ImagebuilderImageRecipeBlockDeviceMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImageRecipeBlockDeviceMappingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc = "Get a reference to the value of field `no_device` after provisioning.\n"]
    pub fn no_device(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.no_device", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_name` after provisioning.\n"]
    pub fn virtual_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ebs` after provisioning.\n"]
    pub fn ebs(&self) -> ListRef<ImagebuilderImageRecipeBlockDeviceMappingElEbsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ebs", self.base))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderImageRecipeComponentElParameterEl {
    name: PrimField<String>,
    value: PrimField<String>,
}

impl ImagebuilderImageRecipeComponentElParameterEl { }

impl ToListMappable for ImagebuilderImageRecipeComponentElParameterEl {
    type O = BlockAssignable<ImagebuilderImageRecipeComponentElParameterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImageRecipeComponentElParameterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildImagebuilderImageRecipeComponentElParameterEl {
    pub fn build(self) -> ImagebuilderImageRecipeComponentElParameterEl {
        ImagebuilderImageRecipeComponentElParameterEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct ImagebuilderImageRecipeComponentElParameterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderImageRecipeComponentElParameterElRef {
    fn new(shared: StackShared, base: String) -> ImagebuilderImageRecipeComponentElParameterElRef {
        ImagebuilderImageRecipeComponentElParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImageRecipeComponentElParameterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderImageRecipeComponentElDynamic {
    parameter: Option<DynamicBlock<ImagebuilderImageRecipeComponentElParameterEl>>,
}

#[derive(Serialize)]
pub struct ImagebuilderImageRecipeComponentEl {
    component_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<Vec<ImagebuilderImageRecipeComponentElParameterEl>>,
    dynamic: ImagebuilderImageRecipeComponentElDynamic,
}

impl ImagebuilderImageRecipeComponentEl {
    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        mut self,
        v: impl Into<BlockAssignable<ImagebuilderImageRecipeComponentElParameterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parameter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parameter = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ImagebuilderImageRecipeComponentEl {
    type O = BlockAssignable<ImagebuilderImageRecipeComponentEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImageRecipeComponentEl {
    #[doc = ""]
    pub component_arn: PrimField<String>,
}

impl BuildImagebuilderImageRecipeComponentEl {
    pub fn build(self) -> ImagebuilderImageRecipeComponentEl {
        ImagebuilderImageRecipeComponentEl {
            component_arn: self.component_arn,
            parameter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ImagebuilderImageRecipeComponentElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderImageRecipeComponentElRef {
    fn new(shared: StackShared, base: String) -> ImagebuilderImageRecipeComponentElRef {
        ImagebuilderImageRecipeComponentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImageRecipeComponentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `component_arn` after provisioning.\n"]
    pub fn component_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.component_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderImageRecipeSystemsManagerAgentEl {
    uninstall_after_build: PrimField<bool>,
}

impl ImagebuilderImageRecipeSystemsManagerAgentEl { }

impl ToListMappable for ImagebuilderImageRecipeSystemsManagerAgentEl {
    type O = BlockAssignable<ImagebuilderImageRecipeSystemsManagerAgentEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImageRecipeSystemsManagerAgentEl {
    #[doc = ""]
    pub uninstall_after_build: PrimField<bool>,
}

impl BuildImagebuilderImageRecipeSystemsManagerAgentEl {
    pub fn build(self) -> ImagebuilderImageRecipeSystemsManagerAgentEl {
        ImagebuilderImageRecipeSystemsManagerAgentEl { uninstall_after_build: self.uninstall_after_build }
    }
}

pub struct ImagebuilderImageRecipeSystemsManagerAgentElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderImageRecipeSystemsManagerAgentElRef {
    fn new(shared: StackShared, base: String) -> ImagebuilderImageRecipeSystemsManagerAgentElRef {
        ImagebuilderImageRecipeSystemsManagerAgentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImageRecipeSystemsManagerAgentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `uninstall_after_build` after provisioning.\n"]
    pub fn uninstall_after_build(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.uninstall_after_build", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderImageRecipeDynamic {
    block_device_mapping: Option<DynamicBlock<ImagebuilderImageRecipeBlockDeviceMappingEl>>,
    component: Option<DynamicBlock<ImagebuilderImageRecipeComponentEl>>,
    systems_manager_agent: Option<DynamicBlock<ImagebuilderImageRecipeSystemsManagerAgentEl>>,
}
