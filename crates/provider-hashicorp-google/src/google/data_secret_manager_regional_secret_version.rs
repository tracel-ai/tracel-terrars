use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataSecretManagerRegionalSecretVersionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_secret_data_base64: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    secret: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
struct DataSecretManagerRegionalSecretVersion_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSecretManagerRegionalSecretVersionData>,
}
#[derive(Clone)]
pub struct DataSecretManagerRegionalSecretVersion(Rc<DataSecretManagerRegionalSecretVersion_>);
impl DataSecretManagerRegionalSecretVersion {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `is_secret_data_base64`.\n"]
    pub fn set_is_secret_data_base64(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_secret_data_base64 = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_managed_encryption` after provisioning.\n"]
    pub fn customer_managed_encryption(
        &self,
    ) -> ListRef<DataSecretManagerRegionalSecretVersionCustomerManagedEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.customer_managed_encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destroy_time` after provisioning.\n"]
    pub fn destroy_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destroy_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `is_secret_data_base64` after provisioning.\n"]
    pub fn is_secret_data_base64(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_secret_data_base64", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secret_data` after provisioning.\n"]
    pub fn secret_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
}
impl Referable for DataSecretManagerRegionalSecretVersion {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataSecretManagerRegionalSecretVersion {}
impl ToListMappable for DataSecretManagerRegionalSecretVersion {
    type O = ListRef<DataSecretManagerRegionalSecretVersionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataSecretManagerRegionalSecretVersion_ {
    fn extract_datasource_type(&self) -> String {
        "google_secret_manager_regional_secret_version".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataSecretManagerRegionalSecretVersion {
    pub tf_id: String,
    #[doc = ""]
    pub secret: PrimField<String>,
}
impl BuildDataSecretManagerRegionalSecretVersion {
    pub fn build(self, stack: &mut Stack) -> DataSecretManagerRegionalSecretVersion {
        let out = DataSecretManagerRegionalSecretVersion(Rc::new(
            DataSecretManagerRegionalSecretVersion_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataSecretManagerRegionalSecretVersionData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    id: core::default::Default::default(),
                    is_secret_data_base64: core::default::Default::default(),
                    location: core::default::Default::default(),
                    project: core::default::Default::default(),
                    secret: self.secret,
                    version: core::default::Default::default(),
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataSecretManagerRegionalSecretVersionRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretManagerRegionalSecretVersionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataSecretManagerRegionalSecretVersionRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_managed_encryption` after provisioning.\n"]
    pub fn customer_managed_encryption(
        &self,
    ) -> ListRef<DataSecretManagerRegionalSecretVersionCustomerManagedEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.customer_managed_encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destroy_time` after provisioning.\n"]
    pub fn destroy_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destroy_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `is_secret_data_base64` after provisioning.\n"]
    pub fn is_secret_data_base64(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_secret_data_base64", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secret_data` after provisioning.\n"]
    pub fn secret_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataSecretManagerRegionalSecretVersionCustomerManagedEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_version_name: Option<PrimField<String>>,
}
impl DataSecretManagerRegionalSecretVersionCustomerManagedEncryptionEl {
    #[doc = "Set the field `kms_key_version_name`.\n"]
    pub fn set_kms_key_version_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_version_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataSecretManagerRegionalSecretVersionCustomerManagedEncryptionEl {
    type O = BlockAssignable<DataSecretManagerRegionalSecretVersionCustomerManagedEncryptionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSecretManagerRegionalSecretVersionCustomerManagedEncryptionEl {}
impl BuildDataSecretManagerRegionalSecretVersionCustomerManagedEncryptionEl {
    pub fn build(self) -> DataSecretManagerRegionalSecretVersionCustomerManagedEncryptionEl {
        DataSecretManagerRegionalSecretVersionCustomerManagedEncryptionEl {
            kms_key_version_name: core::default::Default::default(),
        }
    }
}
pub struct DataSecretManagerRegionalSecretVersionCustomerManagedEncryptionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretManagerRegionalSecretVersionCustomerManagedEncryptionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSecretManagerRegionalSecretVersionCustomerManagedEncryptionElRef {
        DataSecretManagerRegionalSecretVersionCustomerManagedEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSecretManagerRegionalSecretVersionCustomerManagedEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_version_name` after provisioning.\n"]
    pub fn kms_key_version_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_version_name", self.base),
        )
    }
}
