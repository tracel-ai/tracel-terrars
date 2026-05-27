use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct FirebaseAppHostingDefaultDomainData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    backend: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    domain_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FirebaseAppHostingDefaultDomainTimeoutsEl>,
}
struct FirebaseAppHostingDefaultDomain_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FirebaseAppHostingDefaultDomainData>,
}
#[derive(Clone)]
pub struct FirebaseAppHostingDefaultDomain(Rc<FirebaseAppHostingDefaultDomain_>);
impl FirebaseAppHostingDefaultDomain {
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
    #[doc = "Set the field `disabled`.\nWhether the domain is disabled. Defaults to false."]
    pub fn set_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = Some(v.into());
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
    pub fn set_timeouts(self, v: impl Into<FirebaseAppHostingDefaultDomainTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `backend` after provisioning.\nThe ID of the Backend that this Domain is associated with"]
    pub fn backend(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backend", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime at which the domain was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the domain is disabled. Defaults to false."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\nId of the domain. For default domain, it should be {{backend}}--{{project_id}}.{{location}}.hosted.app"]
    pub fn domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nServer-computed checksum based on other values; may be sent\non update or delete to ensure operation is done on expected resource."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the Backend that this Domain is associated with"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the domain, e.g.\n'projects/{project}/locations/{locationId}/backends/{backendId}/domains/{domainId}'"]
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
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem-assigned, unique identifier."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime at which the domain was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaseAppHostingDefaultDomainTimeoutsElRef {
        FirebaseAppHostingDefaultDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for FirebaseAppHostingDefaultDomain {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for FirebaseAppHostingDefaultDomain {}
impl ToListMappable for FirebaseAppHostingDefaultDomain {
    type O = ListRef<FirebaseAppHostingDefaultDomainRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for FirebaseAppHostingDefaultDomain_ {
    fn extract_resource_type(&self) -> String {
        "google_firebase_app_hosting_default_domain".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildFirebaseAppHostingDefaultDomain {
    pub tf_id: String,
    #[doc = "The ID of the Backend that this Domain is associated with"]
    pub backend: PrimField<String>,
    #[doc = "Id of the domain. For default domain, it should be {{backend}}--{{project_id}}.{{location}}.hosted.app"]
    pub domain_id: PrimField<String>,
    #[doc = "The location of the Backend that this Domain is associated with"]
    pub location: PrimField<String>,
}
impl BuildFirebaseAppHostingDefaultDomain {
    pub fn build(self, stack: &mut Stack) -> FirebaseAppHostingDefaultDomain {
        let out = FirebaseAppHostingDefaultDomain(Rc::new(FirebaseAppHostingDefaultDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FirebaseAppHostingDefaultDomainData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                backend: self.backend,
                disabled: core::default::Default::default(),
                domain_id: self.domain_id,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct FirebaseAppHostingDefaultDomainRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingDefaultDomainRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl FirebaseAppHostingDefaultDomainRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backend` after provisioning.\nThe ID of the Backend that this Domain is associated with"]
    pub fn backend(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backend", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime at which the domain was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the domain is disabled. Defaults to false."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\nId of the domain. For default domain, it should be {{backend}}--{{project_id}}.{{location}}.hosted.app"]
    pub fn domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nServer-computed checksum based on other values; may be sent\non update or delete to ensure operation is done on expected resource."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the Backend that this Domain is associated with"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the domain, e.g.\n'projects/{project}/locations/{locationId}/backends/{backendId}/domains/{domainId}'"]
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
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem-assigned, unique identifier."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime at which the domain was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaseAppHostingDefaultDomainTimeoutsElRef {
        FirebaseAppHostingDefaultDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingDefaultDomainTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl FirebaseAppHostingDefaultDomainTimeoutsEl {
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
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for FirebaseAppHostingDefaultDomainTimeoutsEl {
    type O = BlockAssignable<FirebaseAppHostingDefaultDomainTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingDefaultDomainTimeoutsEl {}
impl BuildFirebaseAppHostingDefaultDomainTimeoutsEl {
    pub fn build(self) -> FirebaseAppHostingDefaultDomainTimeoutsEl {
        FirebaseAppHostingDefaultDomainTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct FirebaseAppHostingDefaultDomainTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingDefaultDomainTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FirebaseAppHostingDefaultDomainTimeoutsElRef {
        FirebaseAppHostingDefaultDomainTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingDefaultDomainTimeoutsElRef {
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
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
