use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataStorageBucketObjectsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_glob: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
}
struct DataStorageBucketObjects_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataStorageBucketObjectsData>,
}
#[derive(Clone)]
pub struct DataStorageBucketObjects(Rc<DataStorageBucketObjects_>);
impl DataStorageBucketObjects {
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
    #[doc = "Set the field `match_glob`.\n"]
    pub fn set_match_glob(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().match_glob = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().prefix = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bucket_objects` after provisioning.\n"]
    pub fn bucket_objects(&self) -> ListRef<DataStorageBucketObjectsBucketObjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bucket_objects", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `match_glob` after provisioning.\n"]
    pub fn match_glob(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.match_glob", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prefix", self.extract_ref()),
        )
    }
}
impl Referable for DataStorageBucketObjects {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataStorageBucketObjects {}
impl ToListMappable for DataStorageBucketObjects {
    type O = ListRef<DataStorageBucketObjectsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataStorageBucketObjects_ {
    fn extract_datasource_type(&self) -> String {
        "google_storage_bucket_objects".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataStorageBucketObjects {
    pub tf_id: String,
    #[doc = ""]
    pub bucket: PrimField<String>,
}
impl BuildDataStorageBucketObjects {
    pub fn build(self, stack: &mut Stack) -> DataStorageBucketObjects {
        let out = DataStorageBucketObjects(Rc::new(DataStorageBucketObjects_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataStorageBucketObjectsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                bucket: self.bucket,
                id: core::default::Default::default(),
                match_glob: core::default::Default::default(),
                prefix: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataStorageBucketObjectsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketObjectsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataStorageBucketObjectsRef {
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
    #[doc = "Get a reference to the value of field `bucket_objects` after provisioning.\n"]
    pub fn bucket_objects(&self) -> ListRef<DataStorageBucketObjectsBucketObjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bucket_objects", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `match_glob` after provisioning.\n"]
    pub fn match_glob(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.match_glob", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prefix", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageBucketObjectsBucketObjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    media_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_class: Option<PrimField<String>>,
}
impl DataStorageBucketObjectsBucketObjectsEl {
    #[doc = "Set the field `content_type`.\n"]
    pub fn set_content_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_type = Some(v.into());
        self
    }
    #[doc = "Set the field `media_link`.\n"]
    pub fn set_media_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.media_link = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `self_link`.\n"]
    pub fn set_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.self_link = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_class`.\n"]
    pub fn set_storage_class(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_class = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageBucketObjectsBucketObjectsEl {
    type O = BlockAssignable<DataStorageBucketObjectsBucketObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageBucketObjectsBucketObjectsEl {}
impl BuildDataStorageBucketObjectsBucketObjectsEl {
    pub fn build(self) -> DataStorageBucketObjectsBucketObjectsEl {
        DataStorageBucketObjectsBucketObjectsEl {
            content_type: core::default::Default::default(),
            media_link: core::default::Default::default(),
            name: core::default::Default::default(),
            self_link: core::default::Default::default(),
            storage_class: core::default::Default::default(),
        }
    }
}
pub struct DataStorageBucketObjectsBucketObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketObjectsBucketObjectsElRef {
    fn new(shared: StackShared, base: String) -> DataStorageBucketObjectsBucketObjectsElRef {
        DataStorageBucketObjectsBucketObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageBucketObjectsBucketObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }
    #[doc = "Get a reference to the value of field `media_link` after provisioning.\n"]
    pub fn media_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.media_link", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_class", self.base),
        )
    }
}
