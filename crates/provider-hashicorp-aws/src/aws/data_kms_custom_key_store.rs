use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataKmsCustomKeyStoreData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_key_store_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_key_store_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataKmsCustomKeyStore_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataKmsCustomKeyStoreData>,
}

#[derive(Clone)]
pub struct DataKmsCustomKeyStore(Rc<DataKmsCustomKeyStore_>);

impl DataKmsCustomKeyStore {
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

    #[doc = "Set the field `custom_key_store_id`.\n"]
    pub fn set_custom_key_store_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().custom_key_store_id = Some(v.into());
        self
    }

    #[doc = "Set the field `custom_key_store_name`.\n"]
    pub fn set_custom_key_store_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().custom_key_store_name = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `cloud_hsm_cluster_id` after provisioning.\n"]
    pub fn cloud_hsm_cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_hsm_cluster_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `connection_state` after provisioning.\n"]
    pub fn connection_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_state", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_date", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `custom_key_store_id` after provisioning.\n"]
    pub fn custom_key_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_key_store_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `custom_key_store_name` after provisioning.\n"]
    pub fn custom_key_store_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_key_store_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `trust_anchor_certificate` after provisioning.\n"]
    pub fn trust_anchor_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trust_anchor_certificate", self.extract_ref()),
        )
    }
}

impl Referable for DataKmsCustomKeyStore {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataKmsCustomKeyStore {}

impl ToListMappable for DataKmsCustomKeyStore {
    type O = ListRef<DataKmsCustomKeyStoreRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataKmsCustomKeyStore_ {
    fn extract_datasource_type(&self) -> String {
        "aws_kms_custom_key_store".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataKmsCustomKeyStore {
    pub tf_id: String,
}

impl BuildDataKmsCustomKeyStore {
    pub fn build(self, stack: &mut Stack) -> DataKmsCustomKeyStore {
        let out = DataKmsCustomKeyStore(Rc::new(DataKmsCustomKeyStore_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataKmsCustomKeyStoreData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                custom_key_store_id: core::default::Default::default(),
                custom_key_store_name: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataKmsCustomKeyStoreRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKmsCustomKeyStoreRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataKmsCustomKeyStoreRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `cloud_hsm_cluster_id` after provisioning.\n"]
    pub fn cloud_hsm_cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_hsm_cluster_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `connection_state` after provisioning.\n"]
    pub fn connection_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_state", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_date", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `custom_key_store_id` after provisioning.\n"]
    pub fn custom_key_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_key_store_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `custom_key_store_name` after provisioning.\n"]
    pub fn custom_key_store_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_key_store_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `trust_anchor_certificate` after provisioning.\n"]
    pub fn trust_anchor_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trust_anchor_certificate", self.extract_ref()),
        )
    }
}
