use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct IapBrandData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    application_title: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    support_email: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IapBrandTimeoutsEl>,
}
struct IapBrand_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IapBrandData>,
}
#[derive(Clone)]
pub struct IapBrand(Rc<IapBrand_>);
impl IapBrand {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<IapBrandTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `application_title` after provisioning.\nApplication name displayed on OAuth consent screen."]
    pub fn application_title(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_title", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nOutput only. Identifier of the brand, in the format 'projects/{project_number}/brands/{brand_id}'\nNOTE: The name can also be expressed as 'projects/{project_id}/brands/{brand_id}', e.g. when importing.\nNOTE: The brand identification corresponds to the project number as only one\nbrand can be created per project."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_internal_only` after provisioning.\nWhether the brand is only intended for usage inside the GSuite organization only."]
    pub fn org_internal_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_internal_only", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `support_email` after provisioning.\nSupport email displayed on the OAuth consent screen. Can be either a\nuser or group email. When a user email is specified, the caller must\nbe the user with the associated email address. When a group email is\nspecified, the caller can be either a user or a service account which\nis an owner of the specified group in Cloud Identity."]
    pub fn support_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.support_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IapBrandTimeoutsElRef {
        IapBrandTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for IapBrand {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for IapBrand {}
impl ToListMappable for IapBrand {
    type O = ListRef<IapBrandRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for IapBrand_ {
    fn extract_resource_type(&self) -> String {
        "google_iap_brand".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildIapBrand {
    pub tf_id: String,
    #[doc = "Application name displayed on OAuth consent screen."]
    pub application_title: PrimField<String>,
    #[doc = "Support email displayed on the OAuth consent screen. Can be either a\nuser or group email. When a user email is specified, the caller must\nbe the user with the associated email address. When a group email is\nspecified, the caller can be either a user or a service account which\nis an owner of the specified group in Cloud Identity."]
    pub support_email: PrimField<String>,
}
impl BuildIapBrand {
    pub fn build(self, stack: &mut Stack) -> IapBrand {
        let out = IapBrand(Rc::new(IapBrand_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IapBrandData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                application_title: self.application_title,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                support_email: self.support_email,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct IapBrandRef {
    shared: StackShared,
    base: String,
}
impl Ref for IapBrandRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl IapBrandRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `application_title` after provisioning.\nApplication name displayed on OAuth consent screen."]
    pub fn application_title(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_title", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nOutput only. Identifier of the brand, in the format 'projects/{project_number}/brands/{brand_id}'\nNOTE: The name can also be expressed as 'projects/{project_id}/brands/{brand_id}', e.g. when importing.\nNOTE: The brand identification corresponds to the project number as only one\nbrand can be created per project."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_internal_only` after provisioning.\nWhether the brand is only intended for usage inside the GSuite organization only."]
    pub fn org_internal_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_internal_only", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `support_email` after provisioning.\nSupport email displayed on the OAuth consent screen. Can be either a\nuser or group email. When a user email is specified, the caller must\nbe the user with the associated email address. When a group email is\nspecified, the caller can be either a user or a service account which\nis an owner of the specified group in Cloud Identity."]
    pub fn support_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.support_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IapBrandTimeoutsElRef {
        IapBrandTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct IapBrandTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl IapBrandTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}
impl ToListMappable for IapBrandTimeoutsEl {
    type O = BlockAssignable<IapBrandTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIapBrandTimeoutsEl {}
impl BuildIapBrandTimeoutsEl {
    pub fn build(self) -> IapBrandTimeoutsEl {
        IapBrandTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct IapBrandTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IapBrandTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> IapBrandTimeoutsElRef {
        IapBrandTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IapBrandTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}
