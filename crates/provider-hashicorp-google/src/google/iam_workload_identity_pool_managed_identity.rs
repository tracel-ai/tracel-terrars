use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct IamWorkloadIdentityPoolManagedIdentityData {
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
    workload_identity_pool_managed_identity_id: PrimField<String>,
    workload_identity_pool_namespace_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attestation_rules: Option<Vec<IamWorkloadIdentityPoolManagedIdentityAttestationRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IamWorkloadIdentityPoolManagedIdentityTimeoutsEl>,
    dynamic: IamWorkloadIdentityPoolManagedIdentityDynamic,
}
struct IamWorkloadIdentityPoolManagedIdentity_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IamWorkloadIdentityPoolManagedIdentityData>,
}
#[derive(Clone)]
pub struct IamWorkloadIdentityPoolManagedIdentity(Rc<IamWorkloadIdentityPoolManagedIdentity_>);
impl IamWorkloadIdentityPoolManagedIdentity {
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
    #[doc = "Set the field `description`.\nA description of the managed identity. Cannot exceed 256 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nWhether the managed identity is disabled. If disabled, credentials may no longer be issued for\nthe identity, however existing credentials will still be accepted until they expire."]
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
    #[doc = "Set the field `attestation_rules`.\n"]
    pub fn set_attestation_rules(
        self,
        v: impl Into<BlockAssignable<IamWorkloadIdentityPoolManagedIdentityAttestationRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attestation_rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attestation_rules = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<IamWorkloadIdentityPoolManagedIdentityTimeoutsEl>,
    ) -> Self {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the managed identity. Cannot exceed 256 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the managed identity is disabled. If disabled, credentials may no longer be issued for\nthe identity, however existing credentials will still be accepted until they expire."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the managed identity as\n'projects/{project_number}/locations/global/workloadIdentityPools/{workload_identity_pool_id}/namespaces/{workload_identity_pool_namespace_id}/managedIdentities/{workload_identity_pool_managed_identity_id}'."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the managed identity.\n* 'ACTIVE': The managed identity is active.\n* 'DELETED': The managed identity is soft-deleted. Soft-deleted managed identities are\npermanently deleted after approximately 30 days. You can restore a soft-deleted managed\nidentity using UndeleteWorkloadIdentityPoolManagedIdentity. You cannot reuse the ID of a\nsoft-deleted managed identity until it is permanently deleted."]
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
    #[doc = "Get a reference to the value of field `workload_identity_pool_managed_identity_id` after provisioning.\nThe ID to use for the managed identity. This value must:\n* contain at most 63 characters\n* contain only lowercase alphanumeric characters or '-'\n* start with an alphanumeric character\n* end with an alphanumeric character\n\n\nThe prefix 'gcp-' will be reserved for future uses."]
    pub fn workload_identity_pool_managed_identity_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.workload_identity_pool_managed_identity_id",
                self.extract_ref()
            ),
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
    pub fn timeouts(&self) -> IamWorkloadIdentityPoolManagedIdentityTimeoutsElRef {
        IamWorkloadIdentityPoolManagedIdentityTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for IamWorkloadIdentityPoolManagedIdentity {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for IamWorkloadIdentityPoolManagedIdentity {}
impl ToListMappable for IamWorkloadIdentityPoolManagedIdentity {
    type O = ListRef<IamWorkloadIdentityPoolManagedIdentityRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for IamWorkloadIdentityPoolManagedIdentity_ {
    fn extract_resource_type(&self) -> String {
        "google_iam_workload_identity_pool_managed_identity".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildIamWorkloadIdentityPoolManagedIdentity {
    pub tf_id: String,
    #[doc = "The ID to use for the pool, which becomes the final component of the resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub workload_identity_pool_id: PrimField<String>,
    #[doc = "The ID to use for the managed identity. This value must:\n* contain at most 63 characters\n* contain only lowercase alphanumeric characters or '-'\n* start with an alphanumeric character\n* end with an alphanumeric character\n\n\nThe prefix 'gcp-' will be reserved for future uses."]
    pub workload_identity_pool_managed_identity_id: PrimField<String>,
    #[doc = "The ID to use for the namespace. This value must:\n* contain at most 63 characters\n* contain only lowercase alphanumeric characters or '-'\n* start with an alphanumeric character\n* end with an alphanumeric character\n\n\nThe prefix 'gcp-' will be reserved for future uses."]
    pub workload_identity_pool_namespace_id: PrimField<String>,
}
impl BuildIamWorkloadIdentityPoolManagedIdentity {
    pub fn build(self, stack: &mut Stack) -> IamWorkloadIdentityPoolManagedIdentity {
        let out = IamWorkloadIdentityPoolManagedIdentity(Rc::new(
            IamWorkloadIdentityPoolManagedIdentity_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(IamWorkloadIdentityPoolManagedIdentityData {
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
                    workload_identity_pool_managed_identity_id: self
                        .workload_identity_pool_managed_identity_id,
                    workload_identity_pool_namespace_id: self.workload_identity_pool_namespace_id,
                    attestation_rules: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct IamWorkloadIdentityPoolManagedIdentityRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkloadIdentityPoolManagedIdentityRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl IamWorkloadIdentityPoolManagedIdentityRef {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the managed identity. Cannot exceed 256 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the managed identity is disabled. If disabled, credentials may no longer be issued for\nthe identity, however existing credentials will still be accepted until they expire."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the managed identity as\n'projects/{project_number}/locations/global/workloadIdentityPools/{workload_identity_pool_id}/namespaces/{workload_identity_pool_namespace_id}/managedIdentities/{workload_identity_pool_managed_identity_id}'."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the managed identity.\n* 'ACTIVE': The managed identity is active.\n* 'DELETED': The managed identity is soft-deleted. Soft-deleted managed identities are\npermanently deleted after approximately 30 days. You can restore a soft-deleted managed\nidentity using UndeleteWorkloadIdentityPoolManagedIdentity. You cannot reuse the ID of a\nsoft-deleted managed identity until it is permanently deleted."]
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
    #[doc = "Get a reference to the value of field `workload_identity_pool_managed_identity_id` after provisioning.\nThe ID to use for the managed identity. This value must:\n* contain at most 63 characters\n* contain only lowercase alphanumeric characters or '-'\n* start with an alphanumeric character\n* end with an alphanumeric character\n\n\nThe prefix 'gcp-' will be reserved for future uses."]
    pub fn workload_identity_pool_managed_identity_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.workload_identity_pool_managed_identity_id",
                self.extract_ref()
            ),
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
    pub fn timeouts(&self) -> IamWorkloadIdentityPoolManagedIdentityTimeoutsElRef {
        IamWorkloadIdentityPoolManagedIdentityTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolManagedIdentityAttestationRulesEl {
    google_cloud_resource: PrimField<String>,
}
impl IamWorkloadIdentityPoolManagedIdentityAttestationRulesEl {}
impl ToListMappable for IamWorkloadIdentityPoolManagedIdentityAttestationRulesEl {
    type O = BlockAssignable<IamWorkloadIdentityPoolManagedIdentityAttestationRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamWorkloadIdentityPoolManagedIdentityAttestationRulesEl {
    #[doc = "A single workload operating on Google Cloud. For example:\n'//compute.googleapis.com/projects/123/uid/zones/us-central1-a/instances/12345678'."]
    pub google_cloud_resource: PrimField<String>,
}
impl BuildIamWorkloadIdentityPoolManagedIdentityAttestationRulesEl {
    pub fn build(self) -> IamWorkloadIdentityPoolManagedIdentityAttestationRulesEl {
        IamWorkloadIdentityPoolManagedIdentityAttestationRulesEl {
            google_cloud_resource: self.google_cloud_resource,
        }
    }
}
pub struct IamWorkloadIdentityPoolManagedIdentityAttestationRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkloadIdentityPoolManagedIdentityAttestationRulesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IamWorkloadIdentityPoolManagedIdentityAttestationRulesElRef {
        IamWorkloadIdentityPoolManagedIdentityAttestationRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamWorkloadIdentityPoolManagedIdentityAttestationRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `google_cloud_resource` after provisioning.\nA single workload operating on Google Cloud. For example:\n'//compute.googleapis.com/projects/123/uid/zones/us-central1-a/instances/12345678'."]
    pub fn google_cloud_resource(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.google_cloud_resource", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolManagedIdentityTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl IamWorkloadIdentityPoolManagedIdentityTimeoutsEl {
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
impl ToListMappable for IamWorkloadIdentityPoolManagedIdentityTimeoutsEl {
    type O = BlockAssignable<IamWorkloadIdentityPoolManagedIdentityTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamWorkloadIdentityPoolManagedIdentityTimeoutsEl {}
impl BuildIamWorkloadIdentityPoolManagedIdentityTimeoutsEl {
    pub fn build(self) -> IamWorkloadIdentityPoolManagedIdentityTimeoutsEl {
        IamWorkloadIdentityPoolManagedIdentityTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct IamWorkloadIdentityPoolManagedIdentityTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkloadIdentityPoolManagedIdentityTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IamWorkloadIdentityPoolManagedIdentityTimeoutsElRef {
        IamWorkloadIdentityPoolManagedIdentityTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamWorkloadIdentityPoolManagedIdentityTimeoutsElRef {
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
#[derive(Serialize, Default)]
struct IamWorkloadIdentityPoolManagedIdentityDynamic {
    attestation_rules:
        Option<DynamicBlock<IamWorkloadIdentityPoolManagedIdentityAttestationRulesEl>>,
}
