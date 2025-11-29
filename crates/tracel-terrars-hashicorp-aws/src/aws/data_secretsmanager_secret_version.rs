use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSecretsmanagerSecretVersionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    secret_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_stage: Option<PrimField<String>>,
}

struct DataSecretsmanagerSecretVersion_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSecretsmanagerSecretVersionData>,
}

#[derive(Clone)]
pub struct DataSecretsmanagerSecretVersion(Rc<DataSecretsmanagerSecretVersion_>);

impl DataSecretsmanagerSecretVersion {
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

    #[doc = "Set the field `version_id`.\n"]
    pub fn set_version_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version_id = Some(v.into());
        self
    }

    #[doc = "Set the field `version_stage`.\n"]
    pub fn set_version_stage(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version_stage = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_binary` after provisioning.\n"]
    pub fn secret_binary(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_binary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_id` after provisioning.\n"]
    pub fn secret_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_string` after provisioning.\n"]
    pub fn secret_string(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_string", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_stage` after provisioning.\n"]
    pub fn version_stage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_stage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_stages` after provisioning.\n"]
    pub fn version_stages(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.version_stages", self.extract_ref()))
    }
}

impl Referable for DataSecretsmanagerSecretVersion {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataSecretsmanagerSecretVersion { }

impl ToListMappable for DataSecretsmanagerSecretVersion {
    type O = ListRef<DataSecretsmanagerSecretVersionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSecretsmanagerSecretVersion_ {
    fn extract_datasource_type(&self) -> String {
        "aws_secretsmanager_secret_version".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSecretsmanagerSecretVersion {
    pub tf_id: String,
    #[doc = ""]
    pub secret_id: PrimField<String>,
}

impl BuildDataSecretsmanagerSecretVersion {
    pub fn build(self, stack: &mut Stack) -> DataSecretsmanagerSecretVersion {
        let out = DataSecretsmanagerSecretVersion(Rc::new(DataSecretsmanagerSecretVersion_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSecretsmanagerSecretVersionData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                secret_id: self.secret_id,
                version_id: core::default::Default::default(),
                version_stage: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSecretsmanagerSecretVersionRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSecretsmanagerSecretVersionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataSecretsmanagerSecretVersionRef {
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

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_binary` after provisioning.\n"]
    pub fn secret_binary(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_binary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_id` after provisioning.\n"]
    pub fn secret_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_string` after provisioning.\n"]
    pub fn secret_string(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_string", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_stage` after provisioning.\n"]
    pub fn version_stage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_stage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_stages` after provisioning.\n"]
    pub fn version_stages(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.version_stages", self.extract_ref()))
    }
}
