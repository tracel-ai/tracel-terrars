use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataImagebuilderContainerRecipeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataImagebuilderContainerRecipe_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataImagebuilderContainerRecipeData>,
}

#[derive(Clone)]
pub struct DataImagebuilderContainerRecipe(Rc<DataImagebuilderContainerRecipe_>);

impl DataImagebuilderContainerRecipe {
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> ListRef<DataImagebuilderContainerRecipeComponentElRef> {
        ListRef::new(self.shared().clone(), format!("{}.component", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_type` after provisioning.\n"]
    pub fn container_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dockerfile_template_data` after provisioning.\n"]
    pub fn dockerfile_template_data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dockerfile_template_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_configuration` after provisioning.\n"]
    pub fn instance_configuration(&self) -> ListRef<DataImagebuilderContainerRecipeInstanceConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.instance_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `target_repository` after provisioning.\n"]
    pub fn target_repository(&self) -> ListRef<DataImagebuilderContainerRecipeTargetRepositoryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target_repository", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `working_directory` after provisioning.\n"]
    pub fn working_directory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.working_directory", self.extract_ref()))
    }
}

impl Referable for DataImagebuilderContainerRecipe {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataImagebuilderContainerRecipe { }

impl ToListMappable for DataImagebuilderContainerRecipe {
    type O = ListRef<DataImagebuilderContainerRecipeRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataImagebuilderContainerRecipe_ {
    fn extract_datasource_type(&self) -> String {
        "aws_imagebuilder_container_recipe".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataImagebuilderContainerRecipe {
    pub tf_id: String,
    #[doc = ""]
    pub arn: PrimField<String>,
}

impl BuildDataImagebuilderContainerRecipe {
    pub fn build(self, stack: &mut Stack) -> DataImagebuilderContainerRecipe {
        let out = DataImagebuilderContainerRecipe(Rc::new(DataImagebuilderContainerRecipe_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataImagebuilderContainerRecipeData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                arn: self.arn,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataImagebuilderContainerRecipeRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderContainerRecipeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataImagebuilderContainerRecipeRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> ListRef<DataImagebuilderContainerRecipeComponentElRef> {
        ListRef::new(self.shared().clone(), format!("{}.component", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_type` after provisioning.\n"]
    pub fn container_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dockerfile_template_data` after provisioning.\n"]
    pub fn dockerfile_template_data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dockerfile_template_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_configuration` after provisioning.\n"]
    pub fn instance_configuration(&self) -> ListRef<DataImagebuilderContainerRecipeInstanceConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.instance_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `target_repository` after provisioning.\n"]
    pub fn target_repository(&self) -> ListRef<DataImagebuilderContainerRecipeTargetRepositoryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target_repository", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `working_directory` after provisioning.\n"]
    pub fn working_directory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.working_directory", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderContainerRecipeComponentElParameterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataImagebuilderContainerRecipeComponentElParameterEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataImagebuilderContainerRecipeComponentElParameterEl {
    type O = BlockAssignable<DataImagebuilderContainerRecipeComponentElParameterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderContainerRecipeComponentElParameterEl {}

impl BuildDataImagebuilderContainerRecipeComponentElParameterEl {
    pub fn build(self) -> DataImagebuilderContainerRecipeComponentElParameterEl {
        DataImagebuilderContainerRecipeComponentElParameterEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderContainerRecipeComponentElParameterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderContainerRecipeComponentElParameterElRef {
    fn new(shared: StackShared, base: String) -> DataImagebuilderContainerRecipeComponentElParameterElRef {
        DataImagebuilderContainerRecipeComponentElParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderContainerRecipeComponentElParameterElRef {
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

#[derive(Serialize)]
pub struct DataImagebuilderContainerRecipeComponentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    component_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<SetField<DataImagebuilderContainerRecipeComponentElParameterEl>>,
}

impl DataImagebuilderContainerRecipeComponentEl {
    #[doc = "Set the field `component_arn`.\n"]
    pub fn set_component_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.component_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        mut self,
        v: impl Into<SetField<DataImagebuilderContainerRecipeComponentElParameterEl>>,
    ) -> Self {
        self.parameter = Some(v.into());
        self
    }
}

impl ToListMappable for DataImagebuilderContainerRecipeComponentEl {
    type O = BlockAssignable<DataImagebuilderContainerRecipeComponentEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderContainerRecipeComponentEl {}

impl BuildDataImagebuilderContainerRecipeComponentEl {
    pub fn build(self) -> DataImagebuilderContainerRecipeComponentEl {
        DataImagebuilderContainerRecipeComponentEl {
            component_arn: core::default::Default::default(),
            parameter: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderContainerRecipeComponentElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderContainerRecipeComponentElRef {
    fn new(shared: StackShared, base: String) -> DataImagebuilderContainerRecipeComponentElRef {
        DataImagebuilderContainerRecipeComponentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderContainerRecipeComponentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `component_arn` after provisioning.\n"]
    pub fn component_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.component_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `parameter` after provisioning.\n"]
    pub fn parameter(&self) -> SetRef<DataImagebuilderContainerRecipeComponentElParameterElRef> {
        SetRef::new(self.shared().clone(), format!("{}.parameter", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_on_termination: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encrypted: Option<PrimField<bool>>,
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

impl DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.delete_on_termination = Some(v.into());
        self
    }

    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(mut self, v: impl Into<PrimField<bool>>) -> Self {
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

impl ToListMappable for DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {
    type O = BlockAssignable<DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {}

impl BuildDataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {
    pub fn build(self) -> DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {
        DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {
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

pub struct DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsElRef {
        DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_on_termination", self.base))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> PrimExpr<bool> {
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

#[derive(Serialize)]
pub struct DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    device_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs: Option<ListField<DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_device: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_name: Option<PrimField<String>>,
}

impl DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {
    #[doc = "Set the field `device_name`.\n"]
    pub fn set_device_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_name = Some(v.into());
        self
    }

    #[doc = "Set the field `ebs`.\n"]
    pub fn set_ebs(
        mut self,
        v: impl Into<ListField<DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl>>,
    ) -> Self {
        self.ebs = Some(v.into());
        self
    }

    #[doc = "Set the field `no_device`.\n"]
    pub fn set_no_device(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.no_device = Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_name`.\n"]
    pub fn set_virtual_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.virtual_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {
    type O = BlockAssignable<DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {}

impl BuildDataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {
    pub fn build(self) -> DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {
        DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {
            device_name: core::default::Default::default(),
            ebs: core::default::Default::default(),
            no_device: core::default::Default::default(),
            virtual_name: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElRef {
        DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ebs` after provisioning.\n"]
    pub fn ebs(&self) -> ListRef<DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ebs", self.base))
    }

    #[doc = "Get a reference to the value of field `no_device` after provisioning.\n"]
    pub fn no_device(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.no_device", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_name` after provisioning.\n"]
    pub fn virtual_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderContainerRecipeInstanceConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    block_device_mapping: Option<SetField<DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<PrimField<String>>,
}

impl DataImagebuilderContainerRecipeInstanceConfigurationEl {
    #[doc = "Set the field `block_device_mapping`.\n"]
    pub fn set_block_device_mapping(
        mut self,
        v: impl Into<SetField<DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl>>,
    ) -> Self {
        self.block_device_mapping = Some(v.into());
        self
    }

    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image = Some(v.into());
        self
    }
}

impl ToListMappable for DataImagebuilderContainerRecipeInstanceConfigurationEl {
    type O = BlockAssignable<DataImagebuilderContainerRecipeInstanceConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderContainerRecipeInstanceConfigurationEl {}

impl BuildDataImagebuilderContainerRecipeInstanceConfigurationEl {
    pub fn build(self) -> DataImagebuilderContainerRecipeInstanceConfigurationEl {
        DataImagebuilderContainerRecipeInstanceConfigurationEl {
            block_device_mapping: core::default::Default::default(),
            image: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderContainerRecipeInstanceConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderContainerRecipeInstanceConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataImagebuilderContainerRecipeInstanceConfigurationElRef {
        DataImagebuilderContainerRecipeInstanceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderContainerRecipeInstanceConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `block_device_mapping` after provisioning.\n"]
    pub fn block_device_mapping(
        &self,
    ) -> SetRef<DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElRef> {
        SetRef::new(self.shared().clone(), format!("{}.block_device_mapping", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderContainerRecipeTargetRepositoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}

impl DataImagebuilderContainerRecipeTargetRepositoryEl {
    #[doc = "Set the field `repository_name`.\n"]
    pub fn set_repository_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository_name = Some(v.into());
        self
    }

    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
}

impl ToListMappable for DataImagebuilderContainerRecipeTargetRepositoryEl {
    type O = BlockAssignable<DataImagebuilderContainerRecipeTargetRepositoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderContainerRecipeTargetRepositoryEl {}

impl BuildDataImagebuilderContainerRecipeTargetRepositoryEl {
    pub fn build(self) -> DataImagebuilderContainerRecipeTargetRepositoryEl {
        DataImagebuilderContainerRecipeTargetRepositoryEl {
            repository_name: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderContainerRecipeTargetRepositoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderContainerRecipeTargetRepositoryElRef {
    fn new(shared: StackShared, base: String) -> DataImagebuilderContainerRecipeTargetRepositoryElRef {
        DataImagebuilderContainerRecipeTargetRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderContainerRecipeTargetRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\n"]
    pub fn repository_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_name", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
