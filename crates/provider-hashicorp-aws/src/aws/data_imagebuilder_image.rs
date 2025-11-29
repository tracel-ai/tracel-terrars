use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataImagebuilderImageData {
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

struct DataImagebuilderImage_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataImagebuilderImageData>,
}

#[derive(Clone)]
pub struct DataImagebuilderImage(Rc<DataImagebuilderImage_>);

impl DataImagebuilderImage {
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

    #[doc = "Get a reference to the value of field `build_version_arn` after provisioning.\n"]
    pub fn build_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_version_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_recipe_arn` after provisioning.\n"]
    pub fn container_recipe_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_recipe_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `distribution_configuration_arn` after provisioning.\n"]
    pub fn distribution_configuration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.distribution_configuration_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enhanced_image_metadata_enabled` after provisioning.\n"]
    pub fn enhanced_image_metadata_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enhanced_image_metadata_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_recipe_arn` after provisioning.\n"]
    pub fn image_recipe_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_recipe_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_scanning_configuration` after provisioning.\n"]
    pub fn image_scanning_configuration(&self) -> ListRef<DataImagebuilderImageImageScanningConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image_scanning_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tests_configuration` after provisioning.\n"]
    pub fn image_tests_configuration(&self) -> ListRef<DataImagebuilderImageImageTestsConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image_tests_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `infrastructure_configuration_arn` after provisioning.\n"]
    pub fn infrastructure_configuration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.infrastructure_configuration_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `os_version` after provisioning.\n"]
    pub fn os_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.os_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_resources` after provisioning.\n"]
    pub fn output_resources(&self) -> ListRef<DataImagebuilderImageOutputResourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.output_resources", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

impl Referable for DataImagebuilderImage {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataImagebuilderImage { }

impl ToListMappable for DataImagebuilderImage {
    type O = ListRef<DataImagebuilderImageRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataImagebuilderImage_ {
    fn extract_datasource_type(&self) -> String {
        "aws_imagebuilder_image".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataImagebuilderImage {
    pub tf_id: String,
    #[doc = ""]
    pub arn: PrimField<String>,
}

impl BuildDataImagebuilderImage {
    pub fn build(self, stack: &mut Stack) -> DataImagebuilderImage {
        let out = DataImagebuilderImage(Rc::new(DataImagebuilderImage_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataImagebuilderImageData {
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

pub struct DataImagebuilderImageRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderImageRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataImagebuilderImageRef {
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

    #[doc = "Get a reference to the value of field `build_version_arn` after provisioning.\n"]
    pub fn build_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_version_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_recipe_arn` after provisioning.\n"]
    pub fn container_recipe_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_recipe_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `distribution_configuration_arn` after provisioning.\n"]
    pub fn distribution_configuration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.distribution_configuration_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enhanced_image_metadata_enabled` after provisioning.\n"]
    pub fn enhanced_image_metadata_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enhanced_image_metadata_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_recipe_arn` after provisioning.\n"]
    pub fn image_recipe_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_recipe_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_scanning_configuration` after provisioning.\n"]
    pub fn image_scanning_configuration(&self) -> ListRef<DataImagebuilderImageImageScanningConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image_scanning_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tests_configuration` after provisioning.\n"]
    pub fn image_tests_configuration(&self) -> ListRef<DataImagebuilderImageImageTestsConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image_tests_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `infrastructure_configuration_arn` after provisioning.\n"]
    pub fn infrastructure_configuration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.infrastructure_configuration_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `os_version` after provisioning.\n"]
    pub fn os_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.os_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_resources` after provisioning.\n"]
    pub fn output_resources(&self) -> ListRef<DataImagebuilderImageOutputResourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.output_resources", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderImageImageScanningConfigurationElEcrConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_tags: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_name: Option<PrimField<String>>,
}

impl DataImagebuilderImageImageScanningConfigurationElEcrConfigurationEl {
    #[doc = "Set the field `container_tags`.\n"]
    pub fn set_container_tags(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.container_tags = Some(v.into());
        self
    }

    #[doc = "Set the field `repository_name`.\n"]
    pub fn set_repository_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataImagebuilderImageImageScanningConfigurationElEcrConfigurationEl {
    type O = BlockAssignable<DataImagebuilderImageImageScanningConfigurationElEcrConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderImageImageScanningConfigurationElEcrConfigurationEl {}

impl BuildDataImagebuilderImageImageScanningConfigurationElEcrConfigurationEl {
    pub fn build(self) -> DataImagebuilderImageImageScanningConfigurationElEcrConfigurationEl {
        DataImagebuilderImageImageScanningConfigurationElEcrConfigurationEl {
            container_tags: core::default::Default::default(),
            repository_name: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderImageImageScanningConfigurationElEcrConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderImageImageScanningConfigurationElEcrConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataImagebuilderImageImageScanningConfigurationElEcrConfigurationElRef {
        DataImagebuilderImageImageScanningConfigurationElEcrConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderImageImageScanningConfigurationElEcrConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_tags` after provisioning.\n"]
    pub fn container_tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.container_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\n"]
    pub fn repository_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderImageImageScanningConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ecr_configuration: Option<ListField<DataImagebuilderImageImageScanningConfigurationElEcrConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_scanning_enabled: Option<PrimField<bool>>,
}

impl DataImagebuilderImageImageScanningConfigurationEl {
    #[doc = "Set the field `ecr_configuration`.\n"]
    pub fn set_ecr_configuration(
        mut self,
        v: impl Into<ListField<DataImagebuilderImageImageScanningConfigurationElEcrConfigurationEl>>,
    ) -> Self {
        self.ecr_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `image_scanning_enabled`.\n"]
    pub fn set_image_scanning_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.image_scanning_enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataImagebuilderImageImageScanningConfigurationEl {
    type O = BlockAssignable<DataImagebuilderImageImageScanningConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderImageImageScanningConfigurationEl {}

impl BuildDataImagebuilderImageImageScanningConfigurationEl {
    pub fn build(self) -> DataImagebuilderImageImageScanningConfigurationEl {
        DataImagebuilderImageImageScanningConfigurationEl {
            ecr_configuration: core::default::Default::default(),
            image_scanning_enabled: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderImageImageScanningConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderImageImageScanningConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataImagebuilderImageImageScanningConfigurationElRef {
        DataImagebuilderImageImageScanningConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderImageImageScanningConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ecr_configuration` after provisioning.\n"]
    pub fn ecr_configuration(&self) -> ListRef<DataImagebuilderImageImageScanningConfigurationElEcrConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ecr_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `image_scanning_enabled` after provisioning.\n"]
    pub fn image_scanning_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_scanning_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderImageImageTestsConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    image_tests_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_minutes: Option<PrimField<f64>>,
}

impl DataImagebuilderImageImageTestsConfigurationEl {
    #[doc = "Set the field `image_tests_enabled`.\n"]
    pub fn set_image_tests_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.image_tests_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_minutes`.\n"]
    pub fn set_timeout_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_minutes = Some(v.into());
        self
    }
}

impl ToListMappable for DataImagebuilderImageImageTestsConfigurationEl {
    type O = BlockAssignable<DataImagebuilderImageImageTestsConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderImageImageTestsConfigurationEl {}

impl BuildDataImagebuilderImageImageTestsConfigurationEl {
    pub fn build(self) -> DataImagebuilderImageImageTestsConfigurationEl {
        DataImagebuilderImageImageTestsConfigurationEl {
            image_tests_enabled: core::default::Default::default(),
            timeout_minutes: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderImageImageTestsConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderImageImageTestsConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataImagebuilderImageImageTestsConfigurationElRef {
        DataImagebuilderImageImageTestsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderImageImageTestsConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image_tests_enabled` after provisioning.\n"]
    pub fn image_tests_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_tests_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_minutes` after provisioning.\n"]
    pub fn timeout_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout_minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderImageOutputResourcesElAmisEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

impl DataImagebuilderImageOutputResourcesElAmisEl {
    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.account_id = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
}

impl ToListMappable for DataImagebuilderImageOutputResourcesElAmisEl {
    type O = BlockAssignable<DataImagebuilderImageOutputResourcesElAmisEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderImageOutputResourcesElAmisEl {}

impl BuildDataImagebuilderImageOutputResourcesElAmisEl {
    pub fn build(self) -> DataImagebuilderImageOutputResourcesElAmisEl {
        DataImagebuilderImageOutputResourcesElAmisEl {
            account_id: core::default::Default::default(),
            description: core::default::Default::default(),
            image: core::default::Default::default(),
            name: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderImageOutputResourcesElAmisElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderImageOutputResourcesElAmisElRef {
    fn new(shared: StackShared, base: String) -> DataImagebuilderImageOutputResourcesElAmisElRef {
        DataImagebuilderImageOutputResourcesElAmisElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderImageOutputResourcesElAmisElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderImageOutputResourcesElContainersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    image_uris: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

impl DataImagebuilderImageOutputResourcesElContainersEl {
    #[doc = "Set the field `image_uris`.\n"]
    pub fn set_image_uris(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.image_uris = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
}

impl ToListMappable for DataImagebuilderImageOutputResourcesElContainersEl {
    type O = BlockAssignable<DataImagebuilderImageOutputResourcesElContainersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderImageOutputResourcesElContainersEl {}

impl BuildDataImagebuilderImageOutputResourcesElContainersEl {
    pub fn build(self) -> DataImagebuilderImageOutputResourcesElContainersEl {
        DataImagebuilderImageOutputResourcesElContainersEl {
            image_uris: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderImageOutputResourcesElContainersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderImageOutputResourcesElContainersElRef {
    fn new(shared: StackShared, base: String) -> DataImagebuilderImageOutputResourcesElContainersElRef {
        DataImagebuilderImageOutputResourcesElContainersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderImageOutputResourcesElContainersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image_uris` after provisioning.\n"]
    pub fn image_uris(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.image_uris", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderImageOutputResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    amis: Option<SetField<DataImagebuilderImageOutputResourcesElAmisEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    containers: Option<SetField<DataImagebuilderImageOutputResourcesElContainersEl>>,
}

impl DataImagebuilderImageOutputResourcesEl {
    #[doc = "Set the field `amis`.\n"]
    pub fn set_amis(mut self, v: impl Into<SetField<DataImagebuilderImageOutputResourcesElAmisEl>>) -> Self {
        self.amis = Some(v.into());
        self
    }

    #[doc = "Set the field `containers`.\n"]
    pub fn set_containers(mut self, v: impl Into<SetField<DataImagebuilderImageOutputResourcesElContainersEl>>) -> Self {
        self.containers = Some(v.into());
        self
    }
}

impl ToListMappable for DataImagebuilderImageOutputResourcesEl {
    type O = BlockAssignable<DataImagebuilderImageOutputResourcesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderImageOutputResourcesEl {}

impl BuildDataImagebuilderImageOutputResourcesEl {
    pub fn build(self) -> DataImagebuilderImageOutputResourcesEl {
        DataImagebuilderImageOutputResourcesEl {
            amis: core::default::Default::default(),
            containers: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderImageOutputResourcesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderImageOutputResourcesElRef {
    fn new(shared: StackShared, base: String) -> DataImagebuilderImageOutputResourcesElRef {
        DataImagebuilderImageOutputResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderImageOutputResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amis` after provisioning.\n"]
    pub fn amis(&self) -> SetRef<DataImagebuilderImageOutputResourcesElAmisElRef> {
        SetRef::new(self.shared().clone(), format!("{}.amis", self.base))
    }

    #[doc = "Get a reference to the value of field `containers` after provisioning.\n"]
    pub fn containers(&self) -> SetRef<DataImagebuilderImageOutputResourcesElContainersElRef> {
        SetRef::new(self.shared().clone(), format!("{}.containers", self.base))
    }
}
