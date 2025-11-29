use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSagemakerPrebuiltEcrImageData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_suffix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_tag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    repository_name: PrimField<String>,
}

struct DataSagemakerPrebuiltEcrImage_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSagemakerPrebuiltEcrImageData>,
}

#[derive(Clone)]
pub struct DataSagemakerPrebuiltEcrImage(Rc<DataSagemakerPrebuiltEcrImage_>);

impl DataSagemakerPrebuiltEcrImage {
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

    #[doc = "Set the field `dns_suffix`.\n"]
    pub fn set_dns_suffix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().dns_suffix = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `image_tag`.\n"]
    pub fn set_image_tag(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().image_tag = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `dns_suffix` after provisioning.\n"]
    pub fn dns_suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_suffix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tag` after provisioning.\n"]
    pub fn image_tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_tag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registry_path` after provisioning.\n"]
    pub fn registry_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\n"]
    pub fn repository_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_name", self.extract_ref()))
    }
}

impl Referable for DataSagemakerPrebuiltEcrImage {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataSagemakerPrebuiltEcrImage { }

impl ToListMappable for DataSagemakerPrebuiltEcrImage {
    type O = ListRef<DataSagemakerPrebuiltEcrImageRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSagemakerPrebuiltEcrImage_ {
    fn extract_datasource_type(&self) -> String {
        "aws_sagemaker_prebuilt_ecr_image".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSagemakerPrebuiltEcrImage {
    pub tf_id: String,
    #[doc = ""]
    pub repository_name: PrimField<String>,
}

impl BuildDataSagemakerPrebuiltEcrImage {
    pub fn build(self, stack: &mut Stack) -> DataSagemakerPrebuiltEcrImage {
        let out = DataSagemakerPrebuiltEcrImage(Rc::new(DataSagemakerPrebuiltEcrImage_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSagemakerPrebuiltEcrImageData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                dns_suffix: core::default::Default::default(),
                id: core::default::Default::default(),
                image_tag: core::default::Default::default(),
                region: core::default::Default::default(),
                repository_name: self.repository_name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSagemakerPrebuiltEcrImageRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSagemakerPrebuiltEcrImageRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataSagemakerPrebuiltEcrImageRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `dns_suffix` after provisioning.\n"]
    pub fn dns_suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_suffix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tag` after provisioning.\n"]
    pub fn image_tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_tag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registry_path` after provisioning.\n"]
    pub fn registry_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\n"]
    pub fn repository_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_name", self.extract_ref()))
    }
}
