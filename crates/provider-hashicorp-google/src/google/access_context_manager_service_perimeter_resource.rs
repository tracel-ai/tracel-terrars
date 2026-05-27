use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AccessContextManagerServicePerimeterResourceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    perimeter_name: PrimField<String>,
    resource: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AccessContextManagerServicePerimeterResourceTimeoutsEl>,
}
struct AccessContextManagerServicePerimeterResource_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AccessContextManagerServicePerimeterResourceData>,
}
#[derive(Clone)]
pub struct AccessContextManagerServicePerimeterResource(
    Rc<AccessContextManagerServicePerimeterResource_>,
);
impl AccessContextManagerServicePerimeterResource {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<AccessContextManagerServicePerimeterResourceTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `access_policy_id` after provisioning.\nThe name of the Access Policy this resource belongs to."]
    pub fn access_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe perimeter etag is internally used to prevent overwriting the list of perimeter resources on PATCH calls. It is retrieved from the same GET perimeter API call that's used to get the current list of resources. The resource to add or remove is merged into that list and then this etag is sent with the PATCH call along with the updated resource list."]
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
    #[doc = "Get a reference to the value of field `perimeter_name` after provisioning.\nThe name of the Service Perimeter to add this resource to."]
    pub fn perimeter_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.perimeter_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource` after provisioning.\nA GCP resource that is inside of the service perimeter.\nCurrently only projects are allowed.\nFormat: projects/{project_number}"]
    pub fn resource(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerServicePerimeterResourceTimeoutsElRef {
        AccessContextManagerServicePerimeterResourceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for AccessContextManagerServicePerimeterResource {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AccessContextManagerServicePerimeterResource {}
impl ToListMappable for AccessContextManagerServicePerimeterResource {
    type O = ListRef<AccessContextManagerServicePerimeterResourceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AccessContextManagerServicePerimeterResource_ {
    fn extract_resource_type(&self) -> String {
        "google_access_context_manager_service_perimeter_resource".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAccessContextManagerServicePerimeterResource {
    pub tf_id: String,
    #[doc = "The name of the Service Perimeter to add this resource to."]
    pub perimeter_name: PrimField<String>,
    #[doc = "A GCP resource that is inside of the service perimeter.\nCurrently only projects are allowed.\nFormat: projects/{project_number}"]
    pub resource: PrimField<String>,
}
impl BuildAccessContextManagerServicePerimeterResource {
    pub fn build(self, stack: &mut Stack) -> AccessContextManagerServicePerimeterResource {
        let out = AccessContextManagerServicePerimeterResource(Rc::new(
            AccessContextManagerServicePerimeterResource_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(AccessContextManagerServicePerimeterResourceData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    deletion_policy: core::default::Default::default(),
                    id: core::default::Default::default(),
                    perimeter_name: self.perimeter_name,
                    resource: self.resource,
                    timeouts: core::default::Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AccessContextManagerServicePerimeterResourceRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterResourceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AccessContextManagerServicePerimeterResourceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_policy_id` after provisioning.\nThe name of the Access Policy this resource belongs to."]
    pub fn access_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe perimeter etag is internally used to prevent overwriting the list of perimeter resources on PATCH calls. It is retrieved from the same GET perimeter API call that's used to get the current list of resources. The resource to add or remove is merged into that list and then this etag is sent with the PATCH call along with the updated resource list."]
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
    #[doc = "Get a reference to the value of field `perimeter_name` after provisioning.\nThe name of the Service Perimeter to add this resource to."]
    pub fn perimeter_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.perimeter_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource` after provisioning.\nA GCP resource that is inside of the service perimeter.\nCurrently only projects are allowed.\nFormat: projects/{project_number}"]
    pub fn resource(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerServicePerimeterResourceTimeoutsElRef {
        AccessContextManagerServicePerimeterResourceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterResourceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl AccessContextManagerServicePerimeterResourceTimeoutsEl {
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
impl ToListMappable for AccessContextManagerServicePerimeterResourceTimeoutsEl {
    type O = BlockAssignable<AccessContextManagerServicePerimeterResourceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterResourceTimeoutsEl {}
impl BuildAccessContextManagerServicePerimeterResourceTimeoutsEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterResourceTimeoutsEl {
        AccessContextManagerServicePerimeterResourceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterResourceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterResourceTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterResourceTimeoutsElRef {
        AccessContextManagerServicePerimeterResourceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterResourceTimeoutsElRef {
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
