use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataTagsTagKeyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    parent: PrimField<String>,
    short_name: PrimField<String>,
}
struct DataTagsTagKey_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataTagsTagKeyData>,
}
#[derive(Clone)]
pub struct DataTagsTagKey(Rc<DataTagsTagKey_>);
impl DataTagsTagKey {
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
    #[doc = "Get a reference to the value of field `allowed_values_regex` after provisioning.\n"]
    pub fn allowed_values_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allowed_values_regex", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `namespaced_name` after provisioning.\n"]
    pub fn namespaced_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespaced_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\n"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `short_name` after provisioning.\n"]
    pub fn short_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.short_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
}
impl Referable for DataTagsTagKey {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataTagsTagKey {}
impl ToListMappable for DataTagsTagKey {
    type O = ListRef<DataTagsTagKeyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataTagsTagKey_ {
    fn extract_datasource_type(&self) -> String {
        "google_tags_tag_key".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataTagsTagKey {
    pub tf_id: String,
    #[doc = ""]
    pub parent: PrimField<String>,
    #[doc = ""]
    pub short_name: PrimField<String>,
}
impl BuildDataTagsTagKey {
    pub fn build(self, stack: &mut Stack) -> DataTagsTagKey {
        let out = DataTagsTagKey(Rc::new(DataTagsTagKey_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataTagsTagKeyData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                parent: self.parent,
                short_name: self.short_name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataTagsTagKeyRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataTagsTagKeyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataTagsTagKeyRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `allowed_values_regex` after provisioning.\n"]
    pub fn allowed_values_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allowed_values_regex", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `namespaced_name` after provisioning.\n"]
    pub fn namespaced_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespaced_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\n"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `short_name` after provisioning.\n"]
    pub fn short_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.short_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
}
