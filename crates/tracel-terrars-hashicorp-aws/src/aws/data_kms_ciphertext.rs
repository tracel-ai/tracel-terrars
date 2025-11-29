use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataKmsCiphertextData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    key_id: PrimField<String>,
    plaintext: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataKmsCiphertext_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataKmsCiphertextData>,
}

#[derive(Clone)]
pub struct DataKmsCiphertext(Rc<DataKmsCiphertext_>);

impl DataKmsCiphertext {
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

    #[doc = "Set the field `context`.\n"]
    pub fn set_context(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().context = Some(v.into());
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

    #[doc = "Get a reference to the value of field `ciphertext_blob` after provisioning.\n"]
    pub fn ciphertext_blob(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ciphertext_blob", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `context` after provisioning.\n"]
    pub fn context(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `plaintext` after provisioning.\n"]
    pub fn plaintext(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.plaintext", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl Referable for DataKmsCiphertext {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataKmsCiphertext { }

impl ToListMappable for DataKmsCiphertext {
    type O = ListRef<DataKmsCiphertextRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataKmsCiphertext_ {
    fn extract_datasource_type(&self) -> String {
        "aws_kms_ciphertext".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataKmsCiphertext {
    pub tf_id: String,
    #[doc = ""]
    pub key_id: PrimField<String>,
    #[doc = ""]
    pub plaintext: PrimField<String>,
}

impl BuildDataKmsCiphertext {
    pub fn build(self, stack: &mut Stack) -> DataKmsCiphertext {
        let out = DataKmsCiphertext(Rc::new(DataKmsCiphertext_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataKmsCiphertextData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                context: core::default::Default::default(),
                id: core::default::Default::default(),
                key_id: self.key_id,
                plaintext: self.plaintext,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataKmsCiphertextRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKmsCiphertextRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataKmsCiphertextRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `ciphertext_blob` after provisioning.\n"]
    pub fn ciphertext_blob(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ciphertext_blob", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `context` after provisioning.\n"]
    pub fn context(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `plaintext` after provisioning.\n"]
    pub fn plaintext(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.plaintext", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}
