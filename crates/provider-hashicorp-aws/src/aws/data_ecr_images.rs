use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataEcrImagesData {
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
}
struct DataEcrImages_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEcrImagesData>,
}
#[derive(Clone)]
pub struct DataEcrImages(Rc<DataEcrImages_>);
impl DataEcrImages {
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
    #[doc = "Set the field `registry_id`.\nID of the registry (AWS account ID)"]
    pub fn set_registry_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().registry_id = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `image_ids` after provisioning.\n"]
    pub fn image_ids(&self) -> ListRef<DataEcrImagesImageIdsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\nID of the registry (AWS account ID)"]
    pub fn registry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.registry_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\nName of the repository"]
    pub fn repository_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_name", self.extract_ref()),
        )
    }
}
impl Referable for DataEcrImages {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataEcrImages {}
impl ToListMappable for DataEcrImages {
    type O = ListRef<DataEcrImagesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataEcrImages_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ecr_images".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataEcrImages {
    pub tf_id: String,
    #[doc = "Name of the repository"]
    pub repository_name: PrimField<String>,
}
impl BuildDataEcrImages {
    pub fn build(self, stack: &mut Stack) -> DataEcrImages {
        let out = DataEcrImages(Rc::new(DataEcrImages_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEcrImagesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                region: core::default::Default::default(),
                registry_id: core::default::Default::default(),
                repository_name: self.repository_name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataEcrImagesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEcrImagesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataEcrImagesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `image_ids` after provisioning.\n"]
    pub fn image_ids(&self) -> ListRef<DataEcrImagesImageIdsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\nID of the registry (AWS account ID)"]
    pub fn registry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.registry_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\nName of the repository"]
    pub fn repository_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_name", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataEcrImagesImageIdsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    image_digest: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_tag: Option<PrimField<String>>,
}
impl DataEcrImagesImageIdsEl {
    #[doc = "Set the field `image_digest`.\n"]
    pub fn set_image_digest(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_digest = Some(v.into());
        self
    }
    #[doc = "Set the field `image_tag`.\n"]
    pub fn set_image_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_tag = Some(v.into());
        self
    }
}
impl ToListMappable for DataEcrImagesImageIdsEl {
    type O = BlockAssignable<DataEcrImagesImageIdsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataEcrImagesImageIdsEl {}
impl BuildDataEcrImagesImageIdsEl {
    pub fn build(self) -> DataEcrImagesImageIdsEl {
        DataEcrImagesImageIdsEl {
            image_digest: core::default::Default::default(),
            image_tag: core::default::Default::default(),
        }
    }
}
pub struct DataEcrImagesImageIdsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEcrImagesImageIdsElRef {
    fn new(shared: StackShared, base: String) -> DataEcrImagesImageIdsElRef {
        DataEcrImagesImageIdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataEcrImagesImageIdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `image_digest` after provisioning.\n"]
    pub fn image_digest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_digest", self.base))
    }
    #[doc = "Get a reference to the value of field `image_tag` after provisioning.\n"]
    pub fn image_tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_tag", self.base))
    }
}
