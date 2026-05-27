use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct IamWorkforcePoolProviderScimTenantData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    claim_mapping: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hard_delete: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    provider_id: PrimField<String>,
    scim_tenant_id: PrimField<String>,
    workforce_pool_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IamWorkforcePoolProviderScimTenantTimeoutsEl>,
}
struct IamWorkforcePoolProviderScimTenant_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IamWorkforcePoolProviderScimTenantData>,
}
#[derive(Clone)]
pub struct IamWorkforcePoolProviderScimTenant(Rc<IamWorkforcePoolProviderScimTenant_>);
impl IamWorkforcePoolProviderScimTenant {
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
    #[doc = "Set the field `claim_mapping`.\nMaps BYOID claims to SCIM claims. This is a required field for new SCIM Tenants being created."]
    pub fn set_claim_mapping(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().claim_mapping = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA user-specified description of the provider. Cannot exceed 256 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nA user-specified display name for the scim tenant. Cannot exceed 32 characters."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `hard_delete`.\nDeletes the SCIM tenant immediately. This operation cannot be undone."]
    pub fn set_hard_delete(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().hard_delete = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<IamWorkforcePoolProviderScimTenantTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `base_uri` after provisioning.\nRepresents the base URI as defined in [RFC 7644, Section\n1.3](https://datatracker.ietf.org/doc/html/rfc7644#section-1.3). Clients\nmust use this as the root address for managing resources under the tenant.\nFormat:\nhttps://iamscim.googleapis.com/{version}/{tenant_id}/"]
    pub fn base_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `claim_mapping` after provisioning.\nMaps BYOID claims to SCIM claims. This is a required field for new SCIM Tenants being created."]
    pub fn claim_mapping(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.claim_mapping", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA user-specified description of the provider. Cannot exceed 256 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA user-specified display name for the scim tenant. Cannot exceed 32 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hard_delete` after provisioning.\nDeletes the SCIM tenant immediately. This operation cannot be undone."]
    pub fn hard_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hard_delete", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the scim tenant.\nFormat: 'locations/{location}/workforcePools/{workforce_pool}/providers/{workforce_pool_provider}/scimTenants/{scim_tenant_id}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `provider_id` after provisioning.\nThe ID of the provider."]
    pub fn provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provider_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `purge_time` after provisioning.\nThe timestamp that represents the time when the SCIM tenant is purged."]
    pub fn purge_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.purge_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scim_tenant_id` after provisioning.\nThe ID to use for the SCIM tenant, which becomes the final component of the resource name. This value must be 4-32 characters, and may contain the characters [a-z0-9-]."]
    pub fn scim_tenant_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scim_tenant_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_agent` after provisioning.\nService Agent created by SCIM Tenant API. SCIM tokens created under\nthis tenant will be attached to this service agent."]
    pub fn service_agent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_agent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the scim tenant.\n* ACTIVE: The scim tenant is active and may be used to validate authentication credentials.\n* DELETED: The scim tenant is soft-deleted. Soft-deleted scim tenants are permanently\n  deleted after approximately 30 days."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workforce_pool_id` after provisioning.\nThe ID of the workforce pool."]
    pub fn workforce_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workforce_pool_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkforcePoolProviderScimTenantTimeoutsElRef {
        IamWorkforcePoolProviderScimTenantTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for IamWorkforcePoolProviderScimTenant {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for IamWorkforcePoolProviderScimTenant {}
impl ToListMappable for IamWorkforcePoolProviderScimTenant {
    type O = ListRef<IamWorkforcePoolProviderScimTenantRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for IamWorkforcePoolProviderScimTenant_ {
    fn extract_resource_type(&self) -> String {
        "google_iam_workforce_pool_provider_scim_tenant".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildIamWorkforcePoolProviderScimTenant {
    pub tf_id: String,
    #[doc = "The location for the resource."]
    pub location: PrimField<String>,
    #[doc = "The ID of the provider."]
    pub provider_id: PrimField<String>,
    #[doc = "The ID to use for the SCIM tenant, which becomes the final component of the resource name. This value must be 4-32 characters, and may contain the characters [a-z0-9-]."]
    pub scim_tenant_id: PrimField<String>,
    #[doc = "The ID of the workforce pool."]
    pub workforce_pool_id: PrimField<String>,
}
impl BuildIamWorkforcePoolProviderScimTenant {
    pub fn build(self, stack: &mut Stack) -> IamWorkforcePoolProviderScimTenant {
        let out =
            IamWorkforcePoolProviderScimTenant(Rc::new(IamWorkforcePoolProviderScimTenant_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(IamWorkforcePoolProviderScimTenantData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    claim_mapping: core::default::Default::default(),
                    deletion_policy: core::default::Default::default(),
                    description: core::default::Default::default(),
                    display_name: core::default::Default::default(),
                    hard_delete: core::default::Default::default(),
                    id: core::default::Default::default(),
                    location: self.location,
                    provider_id: self.provider_id,
                    scim_tenant_id: self.scim_tenant_id,
                    workforce_pool_id: self.workforce_pool_id,
                    timeouts: core::default::Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct IamWorkforcePoolProviderScimTenantRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkforcePoolProviderScimTenantRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl IamWorkforcePoolProviderScimTenantRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `base_uri` after provisioning.\nRepresents the base URI as defined in [RFC 7644, Section\n1.3](https://datatracker.ietf.org/doc/html/rfc7644#section-1.3). Clients\nmust use this as the root address for managing resources under the tenant.\nFormat:\nhttps://iamscim.googleapis.com/{version}/{tenant_id}/"]
    pub fn base_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `claim_mapping` after provisioning.\nMaps BYOID claims to SCIM claims. This is a required field for new SCIM Tenants being created."]
    pub fn claim_mapping(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.claim_mapping", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA user-specified description of the provider. Cannot exceed 256 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA user-specified display name for the scim tenant. Cannot exceed 32 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hard_delete` after provisioning.\nDeletes the SCIM tenant immediately. This operation cannot be undone."]
    pub fn hard_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hard_delete", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the scim tenant.\nFormat: 'locations/{location}/workforcePools/{workforce_pool}/providers/{workforce_pool_provider}/scimTenants/{scim_tenant_id}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `provider_id` after provisioning.\nThe ID of the provider."]
    pub fn provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provider_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `purge_time` after provisioning.\nThe timestamp that represents the time when the SCIM tenant is purged."]
    pub fn purge_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.purge_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scim_tenant_id` after provisioning.\nThe ID to use for the SCIM tenant, which becomes the final component of the resource name. This value must be 4-32 characters, and may contain the characters [a-z0-9-]."]
    pub fn scim_tenant_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scim_tenant_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_agent` after provisioning.\nService Agent created by SCIM Tenant API. SCIM tokens created under\nthis tenant will be attached to this service agent."]
    pub fn service_agent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_agent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the scim tenant.\n* ACTIVE: The scim tenant is active and may be used to validate authentication credentials.\n* DELETED: The scim tenant is soft-deleted. Soft-deleted scim tenants are permanently\n  deleted after approximately 30 days."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workforce_pool_id` after provisioning.\nThe ID of the workforce pool."]
    pub fn workforce_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workforce_pool_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkforcePoolProviderScimTenantTimeoutsElRef {
        IamWorkforcePoolProviderScimTenantTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct IamWorkforcePoolProviderScimTenantTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl IamWorkforcePoolProviderScimTenantTimeoutsEl {
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
impl ToListMappable for IamWorkforcePoolProviderScimTenantTimeoutsEl {
    type O = BlockAssignable<IamWorkforcePoolProviderScimTenantTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamWorkforcePoolProviderScimTenantTimeoutsEl {}
impl BuildIamWorkforcePoolProviderScimTenantTimeoutsEl {
    pub fn build(self) -> IamWorkforcePoolProviderScimTenantTimeoutsEl {
        IamWorkforcePoolProviderScimTenantTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct IamWorkforcePoolProviderScimTenantTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkforcePoolProviderScimTenantTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> IamWorkforcePoolProviderScimTenantTimeoutsElRef {
        IamWorkforcePoolProviderScimTenantTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamWorkforcePoolProviderScimTenantTimeoutsElRef {
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
