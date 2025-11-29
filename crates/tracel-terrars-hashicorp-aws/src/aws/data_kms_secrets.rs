use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataKmsSecretsData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<Vec<DataKmsSecretsSecretEl>>,
    dynamic: DataKmsSecretsDynamic,
}

struct DataKmsSecrets_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataKmsSecretsData>,
}

#[derive(Clone)]
pub struct DataKmsSecrets(Rc<DataKmsSecrets_>);

impl DataKmsSecrets {
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

    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(self, v: impl Into<BlockAssignable<DataKmsSecretsSecretEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().secret = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.secret = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `plaintext` after provisioning.\n"]
    pub fn plaintext(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.plaintext", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl Referable for DataKmsSecrets {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataKmsSecrets { }

impl ToListMappable for DataKmsSecrets {
    type O = ListRef<DataKmsSecretsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataKmsSecrets_ {
    fn extract_datasource_type(&self) -> String {
        "aws_kms_secrets".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataKmsSecrets {
    pub tf_id: String,
}

impl BuildDataKmsSecrets {
    pub fn build(self, stack: &mut Stack) -> DataKmsSecrets {
        let out = DataKmsSecrets(Rc::new(DataKmsSecrets_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataKmsSecretsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                secret: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataKmsSecretsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKmsSecretsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataKmsSecretsRef {
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

    #[doc = "Get a reference to the value of field `plaintext` after provisioning.\n"]
    pub fn plaintext(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.plaintext", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataKmsSecretsSecretEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grant_tokens: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_id: Option<PrimField<String>>,
    name: PrimField<String>,
    payload: PrimField<String>,
}

impl DataKmsSecretsSecretEl {
    #[doc = "Set the field `context`.\n"]
    pub fn set_context(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.context = Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_algorithm`.\n"]
    pub fn set_encryption_algorithm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_algorithm = Some(v.into());
        self
    }

    #[doc = "Set the field `grant_tokens`.\n"]
    pub fn set_grant_tokens(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.grant_tokens = Some(v.into());
        self
    }

    #[doc = "Set the field `key_id`.\n"]
    pub fn set_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataKmsSecretsSecretEl {
    type O = BlockAssignable<DataKmsSecretsSecretEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKmsSecretsSecretEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub payload: PrimField<String>,
}

impl BuildDataKmsSecretsSecretEl {
    pub fn build(self) -> DataKmsSecretsSecretEl {
        DataKmsSecretsSecretEl {
            context: core::default::Default::default(),
            encryption_algorithm: core::default::Default::default(),
            grant_tokens: core::default::Default::default(),
            key_id: core::default::Default::default(),
            name: self.name,
            payload: self.payload,
        }
    }
}

pub struct DataKmsSecretsSecretElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKmsSecretsSecretElRef {
    fn new(shared: StackShared, base: String) -> DataKmsSecretsSecretElRef {
        DataKmsSecretsSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKmsSecretsSecretElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `context` after provisioning.\n"]
    pub fn context(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.context", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_algorithm` after provisioning.\n"]
    pub fn encryption_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_algorithm", self.base))
    }

    #[doc = "Get a reference to the value of field `grant_tokens` after provisioning.\n"]
    pub fn grant_tokens(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.grant_tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `payload` after provisioning.\n"]
    pub fn payload(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.payload", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataKmsSecretsDynamic {
    secret: Option<DynamicBlock<DataKmsSecretsSecretEl>>,
}
