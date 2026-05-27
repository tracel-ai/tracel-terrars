use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct IamWorkforcePoolData {
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
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    parent: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_duration: Option<PrimField<String>>,
    workforce_pool_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_restrictions: Option<Vec<IamWorkforcePoolAccessRestrictionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IamWorkforcePoolTimeoutsEl>,
    dynamic: IamWorkforcePoolDynamic,
}
struct IamWorkforcePool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IamWorkforcePoolData>,
}
#[derive(Clone)]
pub struct IamWorkforcePool(Rc<IamWorkforcePool_>);
impl IamWorkforcePool {
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
    #[doc = "Set the field `description`.\nA user-specified description of the pool. Cannot exceed 256 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nWhether the pool is disabled. You cannot use a disabled pool to exchange tokens,\nor use existing tokens to access resources. If the pool is re-enabled, existing tokens grant access again."]
    pub fn set_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nA user-specified display name of the pool in Google Cloud Console. Cannot exceed 32 characters."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `session_duration`.\nDuration that the Google Cloud access tokens, console sign-in sessions,\nand 'gcloud' sign-in sessions from this pool are valid.\nMust be greater than 15 minutes (900s) and less than 12 hours (43200s).\nIf 'sessionDuration' is not configured, minted credentials have a default duration of one hour (3600s).\nA duration in seconds with up to nine fractional digits, ending with ''s''. Example: \"'3.5s'\"."]
    pub fn set_session_duration(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().session_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `access_restrictions`.\n"]
    pub fn set_access_restrictions(
        self,
        v: impl Into<BlockAssignable<IamWorkforcePoolAccessRestrictionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_restrictions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_restrictions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<IamWorkforcePoolTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA user-specified description of the pool. Cannot exceed 256 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the pool is disabled. You cannot use a disabled pool to exchange tokens,\nor use existing tokens to access resources. If the pool is re-enabled, existing tokens grant access again."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA user-specified display name of the pool in Google Cloud Console. Cannot exceed 32 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nOutput only. The resource name of the pool.\nFormat: 'locations/{location}/workforcePools/{workforcePoolId}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nImmutable. The resource name of the parent. Format: 'organizations/{org-id}'."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `session_duration` after provisioning.\nDuration that the Google Cloud access tokens, console sign-in sessions,\nand 'gcloud' sign-in sessions from this pool are valid.\nMust be greater than 15 minutes (900s) and less than 12 hours (43200s).\nIf 'sessionDuration' is not configured, minted credentials have a default duration of one hour (3600s).\nA duration in seconds with up to nine fractional digits, ending with ''s''. Example: \"'3.5s'\"."]
    pub fn session_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The state of the pool.\n * STATE_UNSPECIFIED: State unspecified.\n * ACTIVE: The pool is active, and may be used in Google Cloud policies.\n * DELETED: The pool is soft-deleted. Soft-deleted pools are permanently deleted\n   after approximately 30 days. You can restore a soft-deleted pool using\n   [workforcePools.undelete](https://cloud.google.com/iam/docs/reference/rest/v1/locations.workforcePools/undelete#google.iam.admin.v1.WorkforcePools.UndeleteWorkforcePool).\n   You cannot reuse the ID of a soft-deleted pool until it is permanently deleted.\n   While a pool is deleted, you cannot use it to exchange tokens, or use\n   existing tokens to access resources. If the pool is undeleted, existing\n   tokens grant access again."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workforce_pool_id` after provisioning.\nThe name of the pool. The ID must be a globally unique string of 6 to 63 lowercase letters,\ndigits, or hyphens. It must start with a letter, and cannot have a trailing hyphen.\nThe prefix 'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workforce_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workforce_pool_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `access_restrictions` after provisioning.\n"]
    pub fn access_restrictions(&self) -> ListRef<IamWorkforcePoolAccessRestrictionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_restrictions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkforcePoolTimeoutsElRef {
        IamWorkforcePoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for IamWorkforcePool {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for IamWorkforcePool {}
impl ToListMappable for IamWorkforcePool {
    type O = ListRef<IamWorkforcePoolRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for IamWorkforcePool_ {
    fn extract_resource_type(&self) -> String {
        "google_iam_workforce_pool".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildIamWorkforcePool {
    pub tf_id: String,
    #[doc = "The location for the resource."]
    pub location: PrimField<String>,
    #[doc = "Immutable. The resource name of the parent. Format: 'organizations/{org-id}'."]
    pub parent: PrimField<String>,
    #[doc = "The name of the pool. The ID must be a globally unique string of 6 to 63 lowercase letters,\ndigits, or hyphens. It must start with a letter, and cannot have a trailing hyphen.\nThe prefix 'gcp-' is reserved for use by Google, and may not be specified."]
    pub workforce_pool_id: PrimField<String>,
}
impl BuildIamWorkforcePool {
    pub fn build(self, stack: &mut Stack) -> IamWorkforcePool {
        let out = IamWorkforcePool(Rc::new(IamWorkforcePool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IamWorkforcePoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                disabled: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                parent: self.parent,
                session_duration: core::default::Default::default(),
                workforce_pool_id: self.workforce_pool_id,
                access_restrictions: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct IamWorkforcePoolRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkforcePoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl IamWorkforcePoolRef {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA user-specified description of the pool. Cannot exceed 256 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the pool is disabled. You cannot use a disabled pool to exchange tokens,\nor use existing tokens to access resources. If the pool is re-enabled, existing tokens grant access again."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA user-specified display name of the pool in Google Cloud Console. Cannot exceed 32 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nOutput only. The resource name of the pool.\nFormat: 'locations/{location}/workforcePools/{workforcePoolId}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nImmutable. The resource name of the parent. Format: 'organizations/{org-id}'."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `session_duration` after provisioning.\nDuration that the Google Cloud access tokens, console sign-in sessions,\nand 'gcloud' sign-in sessions from this pool are valid.\nMust be greater than 15 minutes (900s) and less than 12 hours (43200s).\nIf 'sessionDuration' is not configured, minted credentials have a default duration of one hour (3600s).\nA duration in seconds with up to nine fractional digits, ending with ''s''. Example: \"'3.5s'\"."]
    pub fn session_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The state of the pool.\n * STATE_UNSPECIFIED: State unspecified.\n * ACTIVE: The pool is active, and may be used in Google Cloud policies.\n * DELETED: The pool is soft-deleted. Soft-deleted pools are permanently deleted\n   after approximately 30 days. You can restore a soft-deleted pool using\n   [workforcePools.undelete](https://cloud.google.com/iam/docs/reference/rest/v1/locations.workforcePools/undelete#google.iam.admin.v1.WorkforcePools.UndeleteWorkforcePool).\n   You cannot reuse the ID of a soft-deleted pool until it is permanently deleted.\n   While a pool is deleted, you cannot use it to exchange tokens, or use\n   existing tokens to access resources. If the pool is undeleted, existing\n   tokens grant access again."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workforce_pool_id` after provisioning.\nThe name of the pool. The ID must be a globally unique string of 6 to 63 lowercase letters,\ndigits, or hyphens. It must start with a letter, and cannot have a trailing hyphen.\nThe prefix 'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workforce_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workforce_pool_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `access_restrictions` after provisioning.\n"]
    pub fn access_restrictions(&self) -> ListRef<IamWorkforcePoolAccessRestrictionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_restrictions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkforcePoolTimeoutsElRef {
        IamWorkforcePoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct IamWorkforcePoolAccessRestrictionsElAllowedServicesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<PrimField<String>>,
}
impl IamWorkforcePoolAccessRestrictionsElAllowedServicesEl {
    #[doc = "Set the field `domain`.\nDomain name of the service.\nExample: console.cloud.google"]
    pub fn set_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain = Some(v.into());
        self
    }
}
impl ToListMappable for IamWorkforcePoolAccessRestrictionsElAllowedServicesEl {
    type O = BlockAssignable<IamWorkforcePoolAccessRestrictionsElAllowedServicesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamWorkforcePoolAccessRestrictionsElAllowedServicesEl {}
impl BuildIamWorkforcePoolAccessRestrictionsElAllowedServicesEl {
    pub fn build(self) -> IamWorkforcePoolAccessRestrictionsElAllowedServicesEl {
        IamWorkforcePoolAccessRestrictionsElAllowedServicesEl {
            domain: core::default::Default::default(),
        }
    }
}
pub struct IamWorkforcePoolAccessRestrictionsElAllowedServicesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkforcePoolAccessRestrictionsElAllowedServicesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IamWorkforcePoolAccessRestrictionsElAllowedServicesElRef {
        IamWorkforcePoolAccessRestrictionsElAllowedServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamWorkforcePoolAccessRestrictionsElAllowedServicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\nDomain name of the service.\nExample: console.cloud.google"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
}
#[derive(Serialize, Default)]
struct IamWorkforcePoolAccessRestrictionsElDynamic {
    allowed_services: Option<DynamicBlock<IamWorkforcePoolAccessRestrictionsElAllowedServicesEl>>,
}
#[derive(Serialize)]
pub struct IamWorkforcePoolAccessRestrictionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_programmatic_signin: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_services: Option<Vec<IamWorkforcePoolAccessRestrictionsElAllowedServicesEl>>,
    dynamic: IamWorkforcePoolAccessRestrictionsElDynamic,
}
impl IamWorkforcePoolAccessRestrictionsEl {
    #[doc = "Set the field `disable_programmatic_signin`.\nDisable programmatic sign-in by disabling token issue via the Security Token API endpoint.\nSee [Security Token Service API](https://cloud.google.com/iam/docs/reference/sts/rest)."]
    pub fn set_disable_programmatic_signin(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_programmatic_signin = Some(v.into());
        self
    }
    #[doc = "Set the field `allowed_services`.\n"]
    pub fn set_allowed_services(
        mut self,
        v: impl Into<BlockAssignable<IamWorkforcePoolAccessRestrictionsElAllowedServicesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.allowed_services = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_services = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IamWorkforcePoolAccessRestrictionsEl {
    type O = BlockAssignable<IamWorkforcePoolAccessRestrictionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamWorkforcePoolAccessRestrictionsEl {}
impl BuildIamWorkforcePoolAccessRestrictionsEl {
    pub fn build(self) -> IamWorkforcePoolAccessRestrictionsEl {
        IamWorkforcePoolAccessRestrictionsEl {
            disable_programmatic_signin: core::default::Default::default(),
            allowed_services: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IamWorkforcePoolAccessRestrictionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkforcePoolAccessRestrictionsElRef {
    fn new(shared: StackShared, base: String) -> IamWorkforcePoolAccessRestrictionsElRef {
        IamWorkforcePoolAccessRestrictionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamWorkforcePoolAccessRestrictionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_programmatic_signin` after provisioning.\nDisable programmatic sign-in by disabling token issue via the Security Token API endpoint.\nSee [Security Token Service API](https://cloud.google.com/iam/docs/reference/sts/rest)."]
    pub fn disable_programmatic_signin(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_programmatic_signin", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allowed_services` after provisioning.\n"]
    pub fn allowed_services(
        &self,
    ) -> ListRef<IamWorkforcePoolAccessRestrictionsElAllowedServicesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_services", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IamWorkforcePoolTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl IamWorkforcePoolTimeoutsEl {
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
impl ToListMappable for IamWorkforcePoolTimeoutsEl {
    type O = BlockAssignable<IamWorkforcePoolTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamWorkforcePoolTimeoutsEl {}
impl BuildIamWorkforcePoolTimeoutsEl {
    pub fn build(self) -> IamWorkforcePoolTimeoutsEl {
        IamWorkforcePoolTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct IamWorkforcePoolTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkforcePoolTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> IamWorkforcePoolTimeoutsElRef {
        IamWorkforcePoolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamWorkforcePoolTimeoutsElRef {
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
struct IamWorkforcePoolDynamic {
    access_restrictions: Option<DynamicBlock<IamWorkforcePoolAccessRestrictionsEl>>,
}
