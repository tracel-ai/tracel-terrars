use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCloudcontrolapiResourceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    type_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    type_version_id: Option<PrimField<String>>,
}

struct DataCloudcontrolapiResource_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudcontrolapiResourceData>,
}

#[derive(Clone)]
pub struct DataCloudcontrolapiResource(Rc<DataCloudcontrolapiResource_>);

impl DataCloudcontrolapiResource {
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

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `type_version_id`.\n"]
    pub fn set_type_version_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_version_id = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_name` after provisioning.\n"]
    pub fn type_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_version_id` after provisioning.\n"]
    pub fn type_version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type_version_id", self.extract_ref()))
    }
}

impl Referable for DataCloudcontrolapiResource {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataCloudcontrolapiResource { }

impl ToListMappable for DataCloudcontrolapiResource {
    type O = ListRef<DataCloudcontrolapiResourceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataCloudcontrolapiResource_ {
    fn extract_datasource_type(&self) -> String {
        "aws_cloudcontrolapi_resource".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudcontrolapiResource {
    pub tf_id: String,
    #[doc = ""]
    pub identifier: PrimField<String>,
    #[doc = ""]
    pub type_name: PrimField<String>,
}

impl BuildDataCloudcontrolapiResource {
    pub fn build(self, stack: &mut Stack) -> DataCloudcontrolapiResource {
        let out = DataCloudcontrolapiResource(Rc::new(DataCloudcontrolapiResource_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCloudcontrolapiResourceData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                identifier: self.identifier,
                region: core::default::Default::default(),
                role_arn: core::default::Default::default(),
                type_name: self.type_name,
                type_version_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudcontrolapiResourceRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCloudcontrolapiResourceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataCloudcontrolapiResourceRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_name` after provisioning.\n"]
    pub fn type_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_version_id` after provisioning.\n"]
    pub fn type_version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type_version_id", self.extract_ref()))
    }
}
