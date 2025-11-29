use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSecretsmanagerRandomPasswordData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_characters: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_lowercase: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_numbers: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_punctuation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_uppercase: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_space: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_each_included_type: Option<PrimField<bool>>,
}

struct DataSecretsmanagerRandomPassword_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSecretsmanagerRandomPasswordData>,
}

#[derive(Clone)]
pub struct DataSecretsmanagerRandomPassword(Rc<DataSecretsmanagerRandomPassword_>);

impl DataSecretsmanagerRandomPassword {
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

    #[doc = "Set the field `exclude_characters`.\n"]
    pub fn set_exclude_characters(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().exclude_characters = Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_lowercase`.\n"]
    pub fn set_exclude_lowercase(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().exclude_lowercase = Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_numbers`.\n"]
    pub fn set_exclude_numbers(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().exclude_numbers = Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_punctuation`.\n"]
    pub fn set_exclude_punctuation(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().exclude_punctuation = Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_uppercase`.\n"]
    pub fn set_exclude_uppercase(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().exclude_uppercase = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `include_space`.\n"]
    pub fn set_include_space(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_space = Some(v.into());
        self
    }

    #[doc = "Set the field `password_length`.\n"]
    pub fn set_password_length(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().password_length = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `require_each_included_type`.\n"]
    pub fn set_require_each_included_type(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_each_included_type = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `exclude_characters` after provisioning.\n"]
    pub fn exclude_characters(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exclude_characters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_lowercase` after provisioning.\n"]
    pub fn exclude_lowercase(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.exclude_lowercase", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_numbers` after provisioning.\n"]
    pub fn exclude_numbers(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.exclude_numbers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_punctuation` after provisioning.\n"]
    pub fn exclude_punctuation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.exclude_punctuation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_uppercase` after provisioning.\n"]
    pub fn exclude_uppercase(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.exclude_uppercase", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_space` after provisioning.\n"]
    pub fn include_space(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_space", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_length` after provisioning.\n"]
    pub fn password_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `random_password` after provisioning.\n"]
    pub fn random_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.random_password", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_each_included_type` after provisioning.\n"]
    pub fn require_each_included_type(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_each_included_type", self.extract_ref()))
    }
}

impl Referable for DataSecretsmanagerRandomPassword {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataSecretsmanagerRandomPassword { }

impl ToListMappable for DataSecretsmanagerRandomPassword {
    type O = ListRef<DataSecretsmanagerRandomPasswordRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSecretsmanagerRandomPassword_ {
    fn extract_datasource_type(&self) -> String {
        "aws_secretsmanager_random_password".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSecretsmanagerRandomPassword {
    pub tf_id: String,
}

impl BuildDataSecretsmanagerRandomPassword {
    pub fn build(self, stack: &mut Stack) -> DataSecretsmanagerRandomPassword {
        let out = DataSecretsmanagerRandomPassword(Rc::new(DataSecretsmanagerRandomPassword_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSecretsmanagerRandomPasswordData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                exclude_characters: core::default::Default::default(),
                exclude_lowercase: core::default::Default::default(),
                exclude_numbers: core::default::Default::default(),
                exclude_punctuation: core::default::Default::default(),
                exclude_uppercase: core::default::Default::default(),
                id: core::default::Default::default(),
                include_space: core::default::Default::default(),
                password_length: core::default::Default::default(),
                region: core::default::Default::default(),
                require_each_included_type: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSecretsmanagerRandomPasswordRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSecretsmanagerRandomPasswordRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataSecretsmanagerRandomPasswordRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `exclude_characters` after provisioning.\n"]
    pub fn exclude_characters(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exclude_characters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_lowercase` after provisioning.\n"]
    pub fn exclude_lowercase(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.exclude_lowercase", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_numbers` after provisioning.\n"]
    pub fn exclude_numbers(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.exclude_numbers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_punctuation` after provisioning.\n"]
    pub fn exclude_punctuation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.exclude_punctuation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_uppercase` after provisioning.\n"]
    pub fn exclude_uppercase(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.exclude_uppercase", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_space` after provisioning.\n"]
    pub fn include_space(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_space", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_length` after provisioning.\n"]
    pub fn password_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `random_password` after provisioning.\n"]
    pub fn random_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.random_password", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_each_included_type` after provisioning.\n"]
    pub fn require_each_included_type(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_each_included_type", self.extract_ref()))
    }
}
