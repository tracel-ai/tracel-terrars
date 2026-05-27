use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataFoldersData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    parent_id: PrimField<String>,
}
struct DataFolders_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataFoldersData>,
}
#[derive(Clone)]
pub struct DataFolders(Rc<DataFolders_>);
impl DataFolders {
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
    #[doc = "Get a reference to the value of field `folders` after provisioning.\n"]
    pub fn folders(&self) -> ListRef<DataFoldersFoldersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.folders", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `parent_id` after provisioning.\n"]
    pub fn parent_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_id", self.extract_ref()),
        )
    }
}
impl Referable for DataFolders {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataFolders {}
impl ToListMappable for DataFolders {
    type O = ListRef<DataFoldersRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataFolders_ {
    fn extract_datasource_type(&self) -> String {
        "google_folders".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataFolders {
    pub tf_id: String,
    #[doc = ""]
    pub parent_id: PrimField<String>,
}
impl BuildDataFolders {
    pub fn build(self, stack: &mut Stack) -> DataFolders {
        let out = DataFolders(Rc::new(DataFolders_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataFoldersData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                parent_id: self.parent_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataFoldersRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataFoldersRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataFoldersRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `folders` after provisioning.\n"]
    pub fn folders(&self) -> ListRef<DataFoldersFoldersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.folders", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `parent_id` after provisioning.\n"]
    pub fn parent_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_id", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataFoldersFoldersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    etag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl DataFoldersFoldersEl {
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `delete_time`.\n"]
    pub fn set_delete_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete_time = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `etag`.\n"]
    pub fn set_etag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.etag = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `parent`.\n"]
    pub fn set_parent(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.parent = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataFoldersFoldersEl {
    type O = BlockAssignable<DataFoldersFoldersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataFoldersFoldersEl {}
impl BuildDataFoldersFoldersEl {
    pub fn build(self) -> DataFoldersFoldersEl {
        DataFoldersFoldersEl {
            create_time: core::default::Default::default(),
            delete_time: core::default::Default::default(),
            display_name: core::default::Default::default(),
            etag: core::default::Default::default(),
            name: core::default::Default::default(),
            parent: core::default::Default::default(),
            state: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct DataFoldersFoldersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataFoldersFoldersElRef {
    fn new(shared: StackShared, base: String) -> DataFoldersFoldersElRef {
        DataFoldersFoldersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataFoldersFoldersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\n"]
    pub fn delete_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\n"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
