use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataStorageObjectSignedUrlData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_md5: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credentials: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extension_headers: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    path: PrimField<String>,
}
struct DataStorageObjectSignedUrl_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataStorageObjectSignedUrlData>,
}
#[derive(Clone)]
pub struct DataStorageObjectSignedUrl(Rc<DataStorageObjectSignedUrl_>);
impl DataStorageObjectSignedUrl {
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
    #[doc = "Set the field `content_md5`.\n"]
    pub fn set_content_md5(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().content_md5 = Some(v.into());
        self
    }
    #[doc = "Set the field `content_type`.\n"]
    pub fn set_content_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().content_type = Some(v.into());
        self
    }
    #[doc = "Set the field `credentials`.\n"]
    pub fn set_credentials(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().credentials = Some(v.into());
        self
    }
    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().duration = Some(v.into());
        self
    }
    #[doc = "Set the field `extension_headers`.\n"]
    pub fn set_extension_headers(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().extension_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `http_method`.\n"]
    pub fn set_http_method(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().http_method = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_md5` after provisioning.\n"]
    pub fn content_md5(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_md5", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `credentials` after provisioning.\n"]
    pub fn credentials(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credentials", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `extension_headers` after provisioning.\n"]
    pub fn extension_headers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.extension_headers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `http_method` after provisioning.\n"]
    pub fn http_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_method", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `signed_url` after provisioning.\n"]
    pub fn signed_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.signed_url", self.extract_ref()),
        )
    }
}
impl Referable for DataStorageObjectSignedUrl {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataStorageObjectSignedUrl {}
impl ToListMappable for DataStorageObjectSignedUrl {
    type O = ListRef<DataStorageObjectSignedUrlRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataStorageObjectSignedUrl_ {
    fn extract_datasource_type(&self) -> String {
        "google_storage_object_signed_url".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataStorageObjectSignedUrl {
    pub tf_id: String,
    #[doc = ""]
    pub bucket: PrimField<String>,
    #[doc = ""]
    pub path: PrimField<String>,
}
impl BuildDataStorageObjectSignedUrl {
    pub fn build(self, stack: &mut Stack) -> DataStorageObjectSignedUrl {
        let out = DataStorageObjectSignedUrl(Rc::new(DataStorageObjectSignedUrl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataStorageObjectSignedUrlData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                bucket: self.bucket,
                content_md5: core::default::Default::default(),
                content_type: core::default::Default::default(),
                credentials: core::default::Default::default(),
                duration: core::default::Default::default(),
                extension_headers: core::default::Default::default(),
                http_method: core::default::Default::default(),
                id: core::default::Default::default(),
                path: self.path,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataStorageObjectSignedUrlRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageObjectSignedUrlRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataStorageObjectSignedUrlRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_md5` after provisioning.\n"]
    pub fn content_md5(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_md5", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `credentials` after provisioning.\n"]
    pub fn credentials(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credentials", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `extension_headers` after provisioning.\n"]
    pub fn extension_headers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.extension_headers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `http_method` after provisioning.\n"]
    pub fn http_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_method", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `signed_url` after provisioning.\n"]
    pub fn signed_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.signed_url", self.extract_ref()),
        )
    }
}
