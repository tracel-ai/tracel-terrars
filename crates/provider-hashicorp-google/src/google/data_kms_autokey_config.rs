use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataKmsAutokeyConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    folder: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}
struct DataKmsAutokeyConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataKmsAutokeyConfigData>,
}
#[derive(Clone)]
pub struct DataKmsAutokeyConfig(Rc<DataKmsAutokeyConfig_>);
impl DataKmsAutokeyConfig {
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
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe etag of the AutokeyConfig for optimistic concurrency control."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `folder` after provisioning.\nThe folder for which to retrieve config."]
    pub fn folder(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.folder", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `key_project` after provisioning.\nThe target key project for a given folder where KMS Autokey will provision a\nCryptoKey for any new KeyHandle the Developer creates. Should have the form\n'projects/<project_id_or_number>'."]
    pub fn key_project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `key_project_resolution_mode` after provisioning.\nHow Autokey determines which project to use when provisioning CMEK keys. Possible values: [\"DEDICATED_KEY_PROJECT\", \"RESOURCE_PROJECT\", \"DISABLED\"]"]
    pub fn key_project_resolution_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_project_resolution_mode", self.extract_ref()),
        )
    }
}
impl Referable for DataKmsAutokeyConfig {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataKmsAutokeyConfig {}
impl ToListMappable for DataKmsAutokeyConfig {
    type O = ListRef<DataKmsAutokeyConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataKmsAutokeyConfig_ {
    fn extract_datasource_type(&self) -> String {
        "google_kms_autokey_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataKmsAutokeyConfig {
    pub tf_id: String,
    #[doc = "The folder for which to retrieve config."]
    pub folder: PrimField<String>,
}
impl BuildDataKmsAutokeyConfig {
    pub fn build(self, stack: &mut Stack) -> DataKmsAutokeyConfig {
        let out = DataKmsAutokeyConfig(Rc::new(DataKmsAutokeyConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataKmsAutokeyConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                folder: self.folder,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataKmsAutokeyConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataKmsAutokeyConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataKmsAutokeyConfigRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe etag of the AutokeyConfig for optimistic concurrency control."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `folder` after provisioning.\nThe folder for which to retrieve config."]
    pub fn folder(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.folder", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `key_project` after provisioning.\nThe target key project for a given folder where KMS Autokey will provision a\nCryptoKey for any new KeyHandle the Developer creates. Should have the form\n'projects/<project_id_or_number>'."]
    pub fn key_project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `key_project_resolution_mode` after provisioning.\nHow Autokey determines which project to use when provisioning CMEK keys. Possible values: [\"DEDICATED_KEY_PROJECT\", \"RESOURCE_PROJECT\", \"DISABLED\"]"]
    pub fn key_project_resolution_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_project_resolution_mode", self.extract_ref()),
        )
    }
}
