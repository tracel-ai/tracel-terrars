use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSsmParametersByPathData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recursive: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    with_decryption: Option<PrimField<bool>>,
}

struct DataSsmParametersByPath_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSsmParametersByPathData>,
}

#[derive(Clone)]
pub struct DataSsmParametersByPath(Rc<DataSsmParametersByPath_>);

impl DataSsmParametersByPath {
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

    #[doc = "Set the field `recursive`.\n"]
    pub fn set_recursive(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().recursive = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `with_decryption`.\n"]
    pub fn set_with_decryption(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().with_decryption = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arns` after provisioning.\n"]
    pub fn arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `names` after provisioning.\n"]
    pub fn names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.names", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recursive` after provisioning.\n"]
    pub fn recursive(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.recursive", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `types` after provisioning.\n"]
    pub fn types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `with_decryption` after provisioning.\n"]
    pub fn with_decryption(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.with_decryption", self.extract_ref()))
    }
}

impl Referable for DataSsmParametersByPath {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataSsmParametersByPath { }

impl ToListMappable for DataSsmParametersByPath {
    type O = ListRef<DataSsmParametersByPathRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSsmParametersByPath_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ssm_parameters_by_path".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSsmParametersByPath {
    pub tf_id: String,
    #[doc = ""]
    pub path: PrimField<String>,
}

impl BuildDataSsmParametersByPath {
    pub fn build(self, stack: &mut Stack) -> DataSsmParametersByPath {
        let out = DataSsmParametersByPath(Rc::new(DataSsmParametersByPath_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSsmParametersByPathData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                path: self.path,
                recursive: core::default::Default::default(),
                region: core::default::Default::default(),
                with_decryption: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSsmParametersByPathRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmParametersByPathRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataSsmParametersByPathRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arns` after provisioning.\n"]
    pub fn arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `names` after provisioning.\n"]
    pub fn names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.names", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recursive` after provisioning.\n"]
    pub fn recursive(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.recursive", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `types` after provisioning.\n"]
    pub fn types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `with_decryption` after provisioning.\n"]
    pub fn with_decryption(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.with_decryption", self.extract_ref()))
    }
}
