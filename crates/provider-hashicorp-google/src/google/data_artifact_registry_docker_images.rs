use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataArtifactRegistryDockerImagesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    repository_id: PrimField<String>,
}
struct DataArtifactRegistryDockerImages_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataArtifactRegistryDockerImagesData>,
}
#[derive(Clone)]
pub struct DataArtifactRegistryDockerImages(Rc<DataArtifactRegistryDockerImages_>);
impl DataArtifactRegistryDockerImages {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `docker_images` after provisioning.\n"]
    pub fn docker_images(&self) -> ListRef<DataArtifactRegistryDockerImagesDockerImagesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.docker_images", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `repository_id` after provisioning.\n"]
    pub fn repository_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_id", self.extract_ref()),
        )
    }
}
impl Referable for DataArtifactRegistryDockerImages {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataArtifactRegistryDockerImages {}
impl ToListMappable for DataArtifactRegistryDockerImages {
    type O = ListRef<DataArtifactRegistryDockerImagesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataArtifactRegistryDockerImages_ {
    fn extract_datasource_type(&self) -> String {
        "google_artifact_registry_docker_images".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataArtifactRegistryDockerImages {
    pub tf_id: String,
    #[doc = ""]
    pub location: PrimField<String>,
    #[doc = ""]
    pub repository_id: PrimField<String>,
}
impl BuildDataArtifactRegistryDockerImages {
    pub fn build(self, stack: &mut Stack) -> DataArtifactRegistryDockerImages {
        let out = DataArtifactRegistryDockerImages(Rc::new(DataArtifactRegistryDockerImages_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataArtifactRegistryDockerImagesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                repository_id: self.repository_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataArtifactRegistryDockerImagesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataArtifactRegistryDockerImagesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataArtifactRegistryDockerImagesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `docker_images` after provisioning.\n"]
    pub fn docker_images(&self) -> ListRef<DataArtifactRegistryDockerImagesDockerImagesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.docker_images", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `repository_id` after provisioning.\n"]
    pub fn repository_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_id", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataArtifactRegistryDockerImagesDockerImagesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    build_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_size_bytes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    media_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upload_time: Option<PrimField<String>>,
}
impl DataArtifactRegistryDockerImagesDockerImagesEl {
    #[doc = "Set the field `build_time`.\n"]
    pub fn set_build_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.build_time = Some(v.into());
        self
    }
    #[doc = "Set the field `image_name`.\n"]
    pub fn set_image_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_name = Some(v.into());
        self
    }
    #[doc = "Set the field `image_size_bytes`.\n"]
    pub fn set_image_size_bytes(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_size_bytes = Some(v.into());
        self
    }
    #[doc = "Set the field `media_type`.\n"]
    pub fn set_media_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.media_type = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `self_link`.\n"]
    pub fn set_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.self_link = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
    #[doc = "Set the field `upload_time`.\n"]
    pub fn set_upload_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.upload_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataArtifactRegistryDockerImagesDockerImagesEl {
    type O = BlockAssignable<DataArtifactRegistryDockerImagesDockerImagesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataArtifactRegistryDockerImagesDockerImagesEl {}
impl BuildDataArtifactRegistryDockerImagesDockerImagesEl {
    pub fn build(self) -> DataArtifactRegistryDockerImagesDockerImagesEl {
        DataArtifactRegistryDockerImagesDockerImagesEl {
            build_time: core::default::Default::default(),
            image_name: core::default::Default::default(),
            image_size_bytes: core::default::Default::default(),
            media_type: core::default::Default::default(),
            name: core::default::Default::default(),
            self_link: core::default::Default::default(),
            tags: core::default::Default::default(),
            update_time: core::default::Default::default(),
            upload_time: core::default::Default::default(),
        }
    }
}
pub struct DataArtifactRegistryDockerImagesDockerImagesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataArtifactRegistryDockerImagesDockerImagesElRef {
    fn new(shared: StackShared, base: String) -> DataArtifactRegistryDockerImagesDockerImagesElRef {
        DataArtifactRegistryDockerImagesDockerImagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataArtifactRegistryDockerImagesDockerImagesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `build_time` after provisioning.\n"]
    pub fn build_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_time", self.base))
    }
    #[doc = "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_name", self.base))
    }
    #[doc = "Get a reference to the value of field `image_size_bytes` after provisioning.\n"]
    pub fn image_size_bytes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_size_bytes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `media_type` after provisioning.\n"]
    pub fn media_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.media_type", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
    #[doc = "Get a reference to the value of field `upload_time` after provisioning.\n"]
    pub fn upload_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.upload_time", self.base))
    }
}
