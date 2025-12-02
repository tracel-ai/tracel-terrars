use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataSecretsmanagerSecretVersionsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_deprecated: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    secret_id: PrimField<String>,
}
struct DataSecretsmanagerSecretVersions_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSecretsmanagerSecretVersionsData>,
}
#[derive(Clone)]
pub struct DataSecretsmanagerSecretVersions(Rc<DataSecretsmanagerSecretVersions_>);
impl DataSecretsmanagerSecretVersions {
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
    #[doc = "Set the field `include_deprecated`.\n"]
    pub fn set_include_deprecated(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_deprecated = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `include_deprecated` after provisioning.\n"]
    pub fn include_deprecated(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_deprecated", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secret_id` after provisioning.\n"]
    pub fn secret_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `versions` after provisioning.\n"]
    pub fn versions(&self) -> ListRef<DataSecretsmanagerSecretVersionsVersionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.versions", self.extract_ref()),
        )
    }
}
impl Referable for DataSecretsmanagerSecretVersions {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataSecretsmanagerSecretVersions {}
impl ToListMappable for DataSecretsmanagerSecretVersions {
    type O = ListRef<DataSecretsmanagerSecretVersionsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataSecretsmanagerSecretVersions_ {
    fn extract_datasource_type(&self) -> String {
        "aws_secretsmanager_secret_versions".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataSecretsmanagerSecretVersions {
    pub tf_id: String,
    #[doc = ""]
    pub secret_id: PrimField<String>,
}
impl BuildDataSecretsmanagerSecretVersions {
    pub fn build(self, stack: &mut Stack) -> DataSecretsmanagerSecretVersions {
        let out = DataSecretsmanagerSecretVersions(Rc::new(DataSecretsmanagerSecretVersions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSecretsmanagerSecretVersionsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                include_deprecated: core::default::Default::default(),
                region: core::default::Default::default(),
                secret_id: self.secret_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataSecretsmanagerSecretVersionsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretsmanagerSecretVersionsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataSecretsmanagerSecretVersionsRef {
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
    #[doc = "Get a reference to the value of field `include_deprecated` after provisioning.\n"]
    pub fn include_deprecated(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_deprecated", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secret_id` after provisioning.\n"]
    pub fn secret_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `versions` after provisioning.\n"]
    pub fn versions(&self) -> ListRef<DataSecretsmanagerSecretVersionsVersionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.versions", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataSecretsmanagerSecretVersionsVersionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    created_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_accessed_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_stages: Option<ListField<PrimField<String>>>,
}
impl DataSecretsmanagerSecretVersionsVersionsEl {
    #[doc = "Set the field `created_time`.\n"]
    pub fn set_created_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_time = Some(v.into());
        self
    }
    #[doc = "Set the field `last_accessed_date`.\n"]
    pub fn set_last_accessed_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_accessed_date = Some(v.into());
        self
    }
    #[doc = "Set the field `version_id`.\n"]
    pub fn set_version_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version_id = Some(v.into());
        self
    }
    #[doc = "Set the field `version_stages`.\n"]
    pub fn set_version_stages(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.version_stages = Some(v.into());
        self
    }
}
impl ToListMappable for DataSecretsmanagerSecretVersionsVersionsEl {
    type O = BlockAssignable<DataSecretsmanagerSecretVersionsVersionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSecretsmanagerSecretVersionsVersionsEl {}
impl BuildDataSecretsmanagerSecretVersionsVersionsEl {
    pub fn build(self) -> DataSecretsmanagerSecretVersionsVersionsEl {
        DataSecretsmanagerSecretVersionsVersionsEl {
            created_time: core::default::Default::default(),
            last_accessed_date: core::default::Default::default(),
            version_id: core::default::Default::default(),
            version_stages: core::default::Default::default(),
        }
    }
}
pub struct DataSecretsmanagerSecretVersionsVersionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretsmanagerSecretVersionsVersionsElRef {
    fn new(shared: StackShared, base: String) -> DataSecretsmanagerSecretVersionsVersionsElRef {
        DataSecretsmanagerSecretVersionsVersionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSecretsmanagerSecretVersionsVersionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.base))
    }
    #[doc = "Get a reference to the value of field `last_accessed_date` after provisioning.\n"]
    pub fn last_accessed_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_accessed_date", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.base))
    }
    #[doc = "Get a reference to the value of field `version_stages` after provisioning.\n"]
    pub fn version_stages(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.version_stages", self.base),
        )
    }
}
