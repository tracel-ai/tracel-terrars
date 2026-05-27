use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataProjectIamCustomRolesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_deleted: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    view: Option<PrimField<String>>,
}
struct DataProjectIamCustomRoles_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataProjectIamCustomRolesData>,
}
#[derive(Clone)]
pub struct DataProjectIamCustomRoles(Rc<DataProjectIamCustomRoles_>);
impl DataProjectIamCustomRoles {
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
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `show_deleted`.\n"]
    pub fn set_show_deleted(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().show_deleted = Some(v.into());
        self
    }
    #[doc = "Set the field `view`.\n"]
    pub fn set_view(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().view = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `roles` after provisioning.\n"]
    pub fn roles(&self) -> ListRef<DataProjectIamCustomRolesRolesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.roles", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `show_deleted` after provisioning.\n"]
    pub fn show_deleted(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.show_deleted", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `view` after provisioning.\n"]
    pub fn view(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.view", self.extract_ref()),
        )
    }
}
impl Referable for DataProjectIamCustomRoles {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataProjectIamCustomRoles {}
impl ToListMappable for DataProjectIamCustomRoles {
    type O = ListRef<DataProjectIamCustomRolesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataProjectIamCustomRoles_ {
    fn extract_datasource_type(&self) -> String {
        "google_project_iam_custom_roles".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataProjectIamCustomRoles {
    pub tf_id: String,
}
impl BuildDataProjectIamCustomRoles {
    pub fn build(self, stack: &mut Stack) -> DataProjectIamCustomRoles {
        let out = DataProjectIamCustomRoles(Rc::new(DataProjectIamCustomRoles_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataProjectIamCustomRolesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                show_deleted: core::default::Default::default(),
                view: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataProjectIamCustomRolesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataProjectIamCustomRolesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataProjectIamCustomRolesRef {
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
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `roles` after provisioning.\n"]
    pub fn roles(&self) -> ListRef<DataProjectIamCustomRolesRolesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.roles", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `show_deleted` after provisioning.\n"]
    pub fn show_deleted(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.show_deleted", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `view` after provisioning.\n"]
    pub fn view(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.view", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataProjectIamCustomRolesRolesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    deleted: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stage: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl DataProjectIamCustomRolesRolesEl {
    #[doc = "Set the field `deleted`.\n"]
    pub fn set_deleted(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deleted = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `permissions`.\n"]
    pub fn set_permissions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.permissions = Some(v.into());
        self
    }
    #[doc = "Set the field `role_id`.\n"]
    pub fn set_role_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_id = Some(v.into());
        self
    }
    #[doc = "Set the field `stage`.\n"]
    pub fn set_stage(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stage = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\n"]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}
impl ToListMappable for DataProjectIamCustomRolesRolesEl {
    type O = BlockAssignable<DataProjectIamCustomRolesRolesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataProjectIamCustomRolesRolesEl {}
impl BuildDataProjectIamCustomRolesRolesEl {
    pub fn build(self) -> DataProjectIamCustomRolesRolesEl {
        DataProjectIamCustomRolesRolesEl {
            deleted: core::default::Default::default(),
            description: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            permissions: core::default::Default::default(),
            role_id: core::default::Default::default(),
            stage: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}
pub struct DataProjectIamCustomRolesRolesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataProjectIamCustomRolesRolesElRef {
    fn new(shared: StackShared, base: String) -> DataProjectIamCustomRolesRolesElRef {
        DataProjectIamCustomRolesRolesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataProjectIamCustomRolesRolesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deleted` after provisioning.\n"]
    pub fn deleted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deleted", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.permissions", self.base))
    }
    #[doc = "Get a reference to the value of field `role_id` after provisioning.\n"]
    pub fn role_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_id", self.base))
    }
    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stage", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}
