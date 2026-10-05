use super::provider::ProviderGrafana;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct FolderData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    org_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_folder_uid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prevent_destroy_if_not_empty: Option<PrimField<bool>>,
    title: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<PrimField<String>>,
}
struct Folder_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FolderData>,
}
#[derive(Clone)]
pub struct Folder(Rc<Folder_>);
impl Folder {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGrafana) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }
    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }
    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }
    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => true,
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    }
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }
    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(r.extract_ref());
        self
    }
    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(attr.to_string());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `org_id`.\nThe Organization ID. If not set, the Org ID defined in the provider block will be used."]
    pub fn set_org_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().org_id = Some(v.into());
        self
    }
    #[doc = "Set the field `parent_folder_uid`.\nThe uid of the parent folder. If set, the folder will be nested. If not set, the folder will be created in the root folder. Note: This requires the nestedFolders feature flag to be enabled on your Grafana instance."]
    pub fn set_parent_folder_uid(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent_folder_uid = Some(v.into());
        self
    }
    #[doc = "Set the field `prevent_destroy_if_not_empty`.\nPrevent deletion of the folder if it is not empty (contains dashboards or alert rules). This feature requires Grafana 10.2 or later. Defaults to `false`."]
    pub fn set_prevent_destroy_if_not_empty(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().prevent_destroy_if_not_empty = Some(v.into());
        self
    }
    #[doc = "Set the field `uid`.\nUnique identifier."]
    pub fn set_uid(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().uid = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Organization ID. If not set, the Org ID defined in the provider block will be used."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent_folder_uid` after provisioning.\nThe uid of the parent folder. If set, the folder will be nested. If not set, the folder will be created in the root folder. Note: This requires the nestedFolders feature flag to be enabled on your Grafana instance."]
    pub fn parent_folder_uid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_folder_uid", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prevent_destroy_if_not_empty` after provisioning.\nPrevent deletion of the folder if it is not empty (contains dashboards or alert rules). This feature requires Grafana 10.2 or later. Defaults to `false`."]
    pub fn prevent_destroy_if_not_empty(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prevent_destroy_if_not_empty", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nThe title of the folder."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.title", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nUnique identifier."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe full URL of the folder."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}
impl Referable for Folder {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for Folder {}
impl ToListMappable for Folder {
    type O = ListRef<FolderRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for Folder_ {
    fn extract_resource_type(&self) -> String {
        "grafana_folder".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildFolder {
    pub tf_id: String,
    #[doc = "The title of the folder."]
    pub title: PrimField<String>,
}
impl BuildFolder {
    pub fn build(self, stack: &mut Stack) -> Folder {
        let out = Folder(Rc::new(Folder_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FolderData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                org_id: core::default::Default::default(),
                parent_folder_uid: core::default::Default::default(),
                prevent_destroy_if_not_empty: core::default::Default::default(),
                title: self.title,
                uid: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct FolderRef {
    shared: StackShared,
    base: String,
}
impl Ref for FolderRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl FolderRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Organization ID. If not set, the Org ID defined in the provider block will be used."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent_folder_uid` after provisioning.\nThe uid of the parent folder. If set, the folder will be nested. If not set, the folder will be created in the root folder. Note: This requires the nestedFolders feature flag to be enabled on your Grafana instance."]
    pub fn parent_folder_uid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_folder_uid", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prevent_destroy_if_not_empty` after provisioning.\nPrevent deletion of the folder if it is not empty (contains dashboards or alert rules). This feature requires Grafana 10.2 or later. Defaults to `false`."]
    pub fn prevent_destroy_if_not_empty(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prevent_destroy_if_not_empty", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nThe title of the folder."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.title", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nUnique identifier."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe full URL of the folder."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}
