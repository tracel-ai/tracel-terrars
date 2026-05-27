use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct IamWorkloadIdentityPoolNamespaceData {
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
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    workload_identity_pool_id: PrimField<String>,
    workload_identity_pool_namespace_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IamWorkloadIdentityPoolNamespaceTimeoutsEl>,
}
struct IamWorkloadIdentityPoolNamespace_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IamWorkloadIdentityPoolNamespaceData>,
}
#[derive(Clone)]
pub struct IamWorkloadIdentityPoolNamespace(Rc<IamWorkloadIdentityPoolNamespace_>);
impl IamWorkloadIdentityPoolNamespace {
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
    #[doc = "Set the field `description`.\nA description of the namespace. Cannot exceed 256 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nWhether the namespace is disabled. If disabled, credentials may no longer be issued for\nidentities within this namespace, however existing credentials will still be accepted until\nthey expire."]
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
    pub fn set_timeouts(self, v: impl Into<IamWorkloadIdentityPoolNamespaceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the namespace. Cannot exceed 256 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the namespace is disabled. If disabled, credentials may no longer be issued for\nidentities within this namespace, however existing credentials will still be accepted until\nthey expire."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the namespace as\n'projects/{project_number}/locations/global/workloadIdentityPools/{workload_identity_pool_id}/namespaces/{workload_identity_pool_namespace_id}'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owner_service` after provisioning.\nDefines the owner that is allowed to mutate this resource. If present, this resource can only\nbe mutated by the owner."]
    pub fn owner_service(&self) -> ListRef<IamWorkloadIdentityPoolNamespaceOwnerServiceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.owner_service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the namespace.\n* 'ACTIVE': The namespace is active.\n* 'DELETED': The namespace is soft-deleted. Soft-deleted namespaces are permanently deleted\nafter approximately 30 days. You can restore a soft-deleted namespace using\nUndeleteWorkloadIdentityPoolNamespace. You cannot reuse the ID of a soft-deleted namespace\nuntil it is permanently deleted."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_identity_pool_id` after provisioning.\nThe ID to use for the pool, which becomes the final component of the resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workload_identity_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_identity_pool_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_identity_pool_namespace_id` after provisioning.\nThe ID to use for the namespace. This value must:\n* contain at most 63 characters\n* contain only lowercase alphanumeric characters or '-'\n* start with an alphanumeric character\n* end with an alphanumeric character\n\n\nThe prefix 'gcp-' will be reserved for future uses."]
    pub fn workload_identity_pool_namespace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_identity_pool_namespace_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkloadIdentityPoolNamespaceTimeoutsElRef {
        IamWorkloadIdentityPoolNamespaceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for IamWorkloadIdentityPoolNamespace {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for IamWorkloadIdentityPoolNamespace {}
impl ToListMappable for IamWorkloadIdentityPoolNamespace {
    type O = ListRef<IamWorkloadIdentityPoolNamespaceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for IamWorkloadIdentityPoolNamespace_ {
    fn extract_resource_type(&self) -> String {
        "google_iam_workload_identity_pool_namespace".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildIamWorkloadIdentityPoolNamespace {
    pub tf_id: String,
    #[doc = "The ID to use for the pool, which becomes the final component of the resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub workload_identity_pool_id: PrimField<String>,
    #[doc = "The ID to use for the namespace. This value must:\n* contain at most 63 characters\n* contain only lowercase alphanumeric characters or '-'\n* start with an alphanumeric character\n* end with an alphanumeric character\n\n\nThe prefix 'gcp-' will be reserved for future uses."]
    pub workload_identity_pool_namespace_id: PrimField<String>,
}
impl BuildIamWorkloadIdentityPoolNamespace {
    pub fn build(self, stack: &mut Stack) -> IamWorkloadIdentityPoolNamespace {
        let out = IamWorkloadIdentityPoolNamespace(Rc::new(IamWorkloadIdentityPoolNamespace_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IamWorkloadIdentityPoolNamespaceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                disabled: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                workload_identity_pool_id: self.workload_identity_pool_id,
                workload_identity_pool_namespace_id: self.workload_identity_pool_namespace_id,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct IamWorkloadIdentityPoolNamespaceRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkloadIdentityPoolNamespaceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl IamWorkloadIdentityPoolNamespaceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the namespace. Cannot exceed 256 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the namespace is disabled. If disabled, credentials may no longer be issued for\nidentities within this namespace, however existing credentials will still be accepted until\nthey expire."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the namespace as\n'projects/{project_number}/locations/global/workloadIdentityPools/{workload_identity_pool_id}/namespaces/{workload_identity_pool_namespace_id}'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owner_service` after provisioning.\nDefines the owner that is allowed to mutate this resource. If present, this resource can only\nbe mutated by the owner."]
    pub fn owner_service(&self) -> ListRef<IamWorkloadIdentityPoolNamespaceOwnerServiceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.owner_service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the namespace.\n* 'ACTIVE': The namespace is active.\n* 'DELETED': The namespace is soft-deleted. Soft-deleted namespaces are permanently deleted\nafter approximately 30 days. You can restore a soft-deleted namespace using\nUndeleteWorkloadIdentityPoolNamespace. You cannot reuse the ID of a soft-deleted namespace\nuntil it is permanently deleted."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_identity_pool_id` after provisioning.\nThe ID to use for the pool, which becomes the final component of the resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workload_identity_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_identity_pool_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_identity_pool_namespace_id` after provisioning.\nThe ID to use for the namespace. This value must:\n* contain at most 63 characters\n* contain only lowercase alphanumeric characters or '-'\n* start with an alphanumeric character\n* end with an alphanumeric character\n\n\nThe prefix 'gcp-' will be reserved for future uses."]
    pub fn workload_identity_pool_namespace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_identity_pool_namespace_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkloadIdentityPoolNamespaceTimeoutsElRef {
        IamWorkloadIdentityPoolNamespaceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolNamespaceOwnerServiceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    principal_subject: Option<PrimField<String>>,
}
impl IamWorkloadIdentityPoolNamespaceOwnerServiceEl {
    #[doc = "Set the field `principal_subject`.\n"]
    pub fn set_principal_subject(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.principal_subject = Some(v.into());
        self
    }
}
impl ToListMappable for IamWorkloadIdentityPoolNamespaceOwnerServiceEl {
    type O = BlockAssignable<IamWorkloadIdentityPoolNamespaceOwnerServiceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamWorkloadIdentityPoolNamespaceOwnerServiceEl {}
impl BuildIamWorkloadIdentityPoolNamespaceOwnerServiceEl {
    pub fn build(self) -> IamWorkloadIdentityPoolNamespaceOwnerServiceEl {
        IamWorkloadIdentityPoolNamespaceOwnerServiceEl {
            principal_subject: core::default::Default::default(),
        }
    }
}
pub struct IamWorkloadIdentityPoolNamespaceOwnerServiceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkloadIdentityPoolNamespaceOwnerServiceElRef {
    fn new(shared: StackShared, base: String) -> IamWorkloadIdentityPoolNamespaceOwnerServiceElRef {
        IamWorkloadIdentityPoolNamespaceOwnerServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamWorkloadIdentityPoolNamespaceOwnerServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `principal_subject` after provisioning.\n"]
    pub fn principal_subject(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.principal_subject", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolNamespaceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl IamWorkloadIdentityPoolNamespaceTimeoutsEl {
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
impl ToListMappable for IamWorkloadIdentityPoolNamespaceTimeoutsEl {
    type O = BlockAssignable<IamWorkloadIdentityPoolNamespaceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamWorkloadIdentityPoolNamespaceTimeoutsEl {}
impl BuildIamWorkloadIdentityPoolNamespaceTimeoutsEl {
    pub fn build(self) -> IamWorkloadIdentityPoolNamespaceTimeoutsEl {
        IamWorkloadIdentityPoolNamespaceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct IamWorkloadIdentityPoolNamespaceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkloadIdentityPoolNamespaceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> IamWorkloadIdentityPoolNamespaceTimeoutsElRef {
        IamWorkloadIdentityPoolNamespaceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamWorkloadIdentityPoolNamespaceTimeoutsElRef {
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
