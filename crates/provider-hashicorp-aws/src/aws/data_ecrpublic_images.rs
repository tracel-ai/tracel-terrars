use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataEcrpublicImagesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registry_id: Option<PrimField<String>>,
    repository_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_ids: Option<Vec<DataEcrpublicImagesImageIdsEl>>,
    dynamic: DataEcrpublicImagesDynamic,
}
struct DataEcrpublicImages_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEcrpublicImagesData>,
}
#[derive(Clone)]
pub struct DataEcrpublicImages(Rc<DataEcrpublicImages_>);
impl DataEcrpublicImages {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `registry_id`.\nAWS account ID associated with the public registry that contains the repository. If not specified, the default public registry is assumed."]
    pub fn set_registry_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().registry_id = Some(v.into());
        self
    }
    #[doc = "Set the field `image_ids`.\n"]
    pub fn set_image_ids(
        self,
        v: impl Into<BlockAssignable<DataEcrpublicImagesImageIdsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().image_ids = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.image_ids = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `images` after provisioning.\n"]
    pub fn images(&self) -> ListRef<DataEcrpublicImagesImagesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.images", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\nAWS account ID associated with the public registry that contains the repository. If not specified, the default public registry is assumed."]
    pub fn registry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.registry_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\nName of the public repository."]
    pub fn repository_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `image_ids` after provisioning.\n"]
    pub fn image_ids(&self) -> ListRef<DataEcrpublicImagesImageIdsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_ids", self.extract_ref()),
        )
    }
}
impl Referable for DataEcrpublicImages {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataEcrpublicImages {}
impl ToListMappable for DataEcrpublicImages {
    type O = ListRef<DataEcrpublicImagesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataEcrpublicImages_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ecrpublic_images".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataEcrpublicImages {
    pub tf_id: String,
    #[doc = "Name of the public repository."]
    pub repository_name: PrimField<String>,
}
impl BuildDataEcrpublicImages {
    pub fn build(self, stack: &mut Stack) -> DataEcrpublicImages {
        let out = DataEcrpublicImages(Rc::new(DataEcrpublicImages_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEcrpublicImagesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                region: core::default::Default::default(),
                registry_id: core::default::Default::default(),
                repository_name: self.repository_name,
                image_ids: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataEcrpublicImagesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEcrpublicImagesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataEcrpublicImagesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `images` after provisioning.\n"]
    pub fn images(&self) -> ListRef<DataEcrpublicImagesImagesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.images", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\nAWS account ID associated with the public registry that contains the repository. If not specified, the default public registry is assumed."]
    pub fn registry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.registry_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\nName of the public repository."]
    pub fn repository_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `image_ids` after provisioning.\n"]
    pub fn image_ids(&self) -> ListRef<DataEcrpublicImagesImageIdsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_ids", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataEcrpublicImagesImagesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    artifact_media_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_digest: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_manifest_media_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_pushed_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_size_in_bytes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registry_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_name: Option<PrimField<String>>,
}
impl DataEcrpublicImagesImagesEl {
    #[doc = "Set the field `artifact_media_type`.\n"]
    pub fn set_artifact_media_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.artifact_media_type = Some(v.into());
        self
    }
    #[doc = "Set the field `image_digest`.\n"]
    pub fn set_image_digest(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_digest = Some(v.into());
        self
    }
    #[doc = "Set the field `image_manifest_media_type`.\n"]
    pub fn set_image_manifest_media_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_manifest_media_type = Some(v.into());
        self
    }
    #[doc = "Set the field `image_pushed_at`.\n"]
    pub fn set_image_pushed_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_pushed_at = Some(v.into());
        self
    }
    #[doc = "Set the field `image_size_in_bytes`.\n"]
    pub fn set_image_size_in_bytes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.image_size_in_bytes = Some(v.into());
        self
    }
    #[doc = "Set the field `image_tags`.\n"]
    pub fn set_image_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.image_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `registry_id`.\n"]
    pub fn set_registry_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.registry_id = Some(v.into());
        self
    }
    #[doc = "Set the field `repository_name`.\n"]
    pub fn set_repository_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataEcrpublicImagesImagesEl {
    type O = BlockAssignable<DataEcrpublicImagesImagesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataEcrpublicImagesImagesEl {}
impl BuildDataEcrpublicImagesImagesEl {
    pub fn build(self) -> DataEcrpublicImagesImagesEl {
        DataEcrpublicImagesImagesEl {
            artifact_media_type: core::default::Default::default(),
            image_digest: core::default::Default::default(),
            image_manifest_media_type: core::default::Default::default(),
            image_pushed_at: core::default::Default::default(),
            image_size_in_bytes: core::default::Default::default(),
            image_tags: core::default::Default::default(),
            registry_id: core::default::Default::default(),
            repository_name: core::default::Default::default(),
        }
    }
}
pub struct DataEcrpublicImagesImagesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEcrpublicImagesImagesElRef {
    fn new(shared: StackShared, base: String) -> DataEcrpublicImagesImagesElRef {
        DataEcrpublicImagesImagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataEcrpublicImagesImagesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `artifact_media_type` after provisioning.\n"]
    pub fn artifact_media_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.artifact_media_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_digest` after provisioning.\n"]
    pub fn image_digest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_digest", self.base))
    }
    #[doc = "Get a reference to the value of field `image_manifest_media_type` after provisioning.\n"]
    pub fn image_manifest_media_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_manifest_media_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_pushed_at` after provisioning.\n"]
    pub fn image_pushed_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_pushed_at", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_size_in_bytes` after provisioning.\n"]
    pub fn image_size_in_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_size_in_bytes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_tags` after provisioning.\n"]
    pub fn image_tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.image_tags", self.base))
    }
    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.base))
    }
    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\n"]
    pub fn repository_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataEcrpublicImagesImageIdsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    image_digest: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_tag: Option<PrimField<String>>,
}
impl DataEcrpublicImagesImageIdsEl {
    #[doc = "Set the field `image_digest`.\nImage digest."]
    pub fn set_image_digest(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_digest = Some(v.into());
        self
    }
    #[doc = "Set the field `image_tag`.\nImage tag."]
    pub fn set_image_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_tag = Some(v.into());
        self
    }
}
impl ToListMappable for DataEcrpublicImagesImageIdsEl {
    type O = BlockAssignable<DataEcrpublicImagesImageIdsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataEcrpublicImagesImageIdsEl {}
impl BuildDataEcrpublicImagesImageIdsEl {
    pub fn build(self) -> DataEcrpublicImagesImageIdsEl {
        DataEcrpublicImagesImageIdsEl {
            image_digest: core::default::Default::default(),
            image_tag: core::default::Default::default(),
        }
    }
}
pub struct DataEcrpublicImagesImageIdsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEcrpublicImagesImageIdsElRef {
    fn new(shared: StackShared, base: String) -> DataEcrpublicImagesImageIdsElRef {
        DataEcrpublicImagesImageIdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataEcrpublicImagesImageIdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `image_digest` after provisioning.\nImage digest."]
    pub fn image_digest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_digest", self.base))
    }
    #[doc = "Get a reference to the value of field `image_tag` after provisioning.\nImage tag."]
    pub fn image_tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_tag", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataEcrpublicImagesDynamic {
    image_ids: Option<DynamicBlock<DataEcrpublicImagesImageIdsEl>>,
}
