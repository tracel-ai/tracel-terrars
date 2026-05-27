use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct IamPrincipalAccessBoundaryPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    organization: PrimField<String>,
    principal_access_boundary_policy_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<Vec<IamPrincipalAccessBoundaryPolicyDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IamPrincipalAccessBoundaryPolicyTimeoutsEl>,
    dynamic: IamPrincipalAccessBoundaryPolicyDynamic,
}
struct IamPrincipalAccessBoundaryPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IamPrincipalAccessBoundaryPolicyData>,
}
#[derive(Clone)]
pub struct IamPrincipalAccessBoundaryPolicy(Rc<IamPrincipalAccessBoundaryPolicy_>);
impl IamPrincipalAccessBoundaryPolicy {
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
    #[doc = "Set the field `annotations`.\nUser defined annotations. See https://google.aip.dev/148#annotations\nfor more details such as format and size limitations\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nThe description of the principal access boundary policy. Must be less than or equal to 63 characters."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        self,
        v: impl Into<BlockAssignable<IamPrincipalAccessBoundaryPolicyDetailsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().details = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.details = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<IamPrincipalAccessBoundaryPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUser defined annotations. See https://google.aip.dev/148#annotations\nfor more details such as format and size limitations\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the principal access boundary policy was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe description of the principal access boundary policy. Must be less than or equal to 63 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe etag for the principal access boundary. If this is provided on update, it must match the server's etag."]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location the principal access boundary policy is in."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the principal access boundary policy.  The following format is supported:\n 'organizations/{organization_id}/locations/{location}/principalAccessBoundaryPolicies/{policy_id}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization` after provisioning.\nThe parent organization of the principal access boundary policy."]
    pub fn organization(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `principal_access_boundary_policy_id` after provisioning.\nThe ID to use to create the principal access boundary policy.\nThis value must start with a lowercase letter followed by up to 62 lowercase letters, numbers, hyphens, or dots. Pattern, /a-z{2,62}/."]
    pub fn principal_access_boundary_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.principal_access_boundary_policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. The globally unique ID of the principal access boundary policy."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the principal access boundary policy was most recently updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> ListRef<IamPrincipalAccessBoundaryPolicyDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamPrincipalAccessBoundaryPolicyTimeoutsElRef {
        IamPrincipalAccessBoundaryPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for IamPrincipalAccessBoundaryPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for IamPrincipalAccessBoundaryPolicy {}
impl ToListMappable for IamPrincipalAccessBoundaryPolicy {
    type O = ListRef<IamPrincipalAccessBoundaryPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for IamPrincipalAccessBoundaryPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_iam_principal_access_boundary_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildIamPrincipalAccessBoundaryPolicy {
    pub tf_id: String,
    #[doc = "The location the principal access boundary policy is in."]
    pub location: PrimField<String>,
    #[doc = "The parent organization of the principal access boundary policy."]
    pub organization: PrimField<String>,
    #[doc = "The ID to use to create the principal access boundary policy.\nThis value must start with a lowercase letter followed by up to 62 lowercase letters, numbers, hyphens, or dots. Pattern, /a-z{2,62}/."]
    pub principal_access_boundary_policy_id: PrimField<String>,
}
impl BuildIamPrincipalAccessBoundaryPolicy {
    pub fn build(self, stack: &mut Stack) -> IamPrincipalAccessBoundaryPolicy {
        let out = IamPrincipalAccessBoundaryPolicy(Rc::new(IamPrincipalAccessBoundaryPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IamPrincipalAccessBoundaryPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                organization: self.organization,
                principal_access_boundary_policy_id: self.principal_access_boundary_policy_id,
                details: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct IamPrincipalAccessBoundaryPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamPrincipalAccessBoundaryPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl IamPrincipalAccessBoundaryPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUser defined annotations. See https://google.aip.dev/148#annotations\nfor more details such as format and size limitations\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the principal access boundary policy was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe description of the principal access boundary policy. Must be less than or equal to 63 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe etag for the principal access boundary. If this is provided on update, it must match the server's etag."]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location the principal access boundary policy is in."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the principal access boundary policy.  The following format is supported:\n 'organizations/{organization_id}/locations/{location}/principalAccessBoundaryPolicies/{policy_id}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization` after provisioning.\nThe parent organization of the principal access boundary policy."]
    pub fn organization(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `principal_access_boundary_policy_id` after provisioning.\nThe ID to use to create the principal access boundary policy.\nThis value must start with a lowercase letter followed by up to 62 lowercase letters, numbers, hyphens, or dots. Pattern, /a-z{2,62}/."]
    pub fn principal_access_boundary_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.principal_access_boundary_policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. The globally unique ID of the principal access boundary policy."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the principal access boundary policy was most recently updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> ListRef<IamPrincipalAccessBoundaryPolicyDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamPrincipalAccessBoundaryPolicyTimeoutsElRef {
        IamPrincipalAccessBoundaryPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct IamPrincipalAccessBoundaryPolicyDetailsElRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    effect: PrimField<String>,
    resources: ListField<PrimField<String>>,
}
impl IamPrincipalAccessBoundaryPolicyDetailsElRulesEl {
    #[doc = "Set the field `description`.\nThe description of the principal access boundary policy rule. Must be less than or equal to 256 characters."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
}
impl ToListMappable for IamPrincipalAccessBoundaryPolicyDetailsElRulesEl {
    type O = BlockAssignable<IamPrincipalAccessBoundaryPolicyDetailsElRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamPrincipalAccessBoundaryPolicyDetailsElRulesEl {
    #[doc = "The access relationship of principals to the resources in this rule.\nPossible values: ALLOW"]
    pub effect: PrimField<String>,
    #[doc = "A list of Cloud Resource Manager resources. The resource\nand all the descendants are included. The number of resources in a policy\nis limited to 500 across all rules.\nThe following resource types are supported:\n* Organizations, such as '//cloudresourcemanager.googleapis.com/organizations/123'.\n* Folders, such as '//cloudresourcemanager.googleapis.com/folders/123'.\n* Projects, such as '//cloudresourcemanager.googleapis.com/projects/123'\nor '//cloudresourcemanager.googleapis.com/projects/my-project-id'."]
    pub resources: ListField<PrimField<String>>,
}
impl BuildIamPrincipalAccessBoundaryPolicyDetailsElRulesEl {
    pub fn build(self) -> IamPrincipalAccessBoundaryPolicyDetailsElRulesEl {
        IamPrincipalAccessBoundaryPolicyDetailsElRulesEl {
            description: core::default::Default::default(),
            effect: self.effect,
            resources: self.resources,
        }
    }
}
pub struct IamPrincipalAccessBoundaryPolicyDetailsElRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamPrincipalAccessBoundaryPolicyDetailsElRulesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IamPrincipalAccessBoundaryPolicyDetailsElRulesElRef {
        IamPrincipalAccessBoundaryPolicyDetailsElRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamPrincipalAccessBoundaryPolicyDetailsElRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the principal access boundary policy rule. Must be less than or equal to 256 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `effect` after provisioning.\nThe access relationship of principals to the resources in this rule.\nPossible values: ALLOW"]
    pub fn effect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\nA list of Cloud Resource Manager resources. The resource\nand all the descendants are included. The number of resources in a policy\nis limited to 500 across all rules.\nThe following resource types are supported:\n* Organizations, such as '//cloudresourcemanager.googleapis.com/organizations/123'.\n* Folders, such as '//cloudresourcemanager.googleapis.com/folders/123'.\n* Projects, such as '//cloudresourcemanager.googleapis.com/projects/123'\nor '//cloudresourcemanager.googleapis.com/projects/my-project-id'."]
    pub fn resources(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
}
#[derive(Serialize, Default)]
struct IamPrincipalAccessBoundaryPolicyDetailsElDynamic {
    rules: Option<DynamicBlock<IamPrincipalAccessBoundaryPolicyDetailsElRulesEl>>,
}
#[derive(Serialize)]
pub struct IamPrincipalAccessBoundaryPolicyDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enforcement_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<Vec<IamPrincipalAccessBoundaryPolicyDetailsElRulesEl>>,
    dynamic: IamPrincipalAccessBoundaryPolicyDetailsElDynamic,
}
impl IamPrincipalAccessBoundaryPolicyDetailsEl {
    #[doc = "Set the field `enforcement_version`.\nThe version number that indicates which Google Cloud services\nare included in the enforcement (e.g. \\\"latest\\\", \\\"1\\\", ...). If empty, the\nPAB policy version will be set to the current latest version, and this version\nwon't get updated when new versions are released."]
    pub fn set_enforcement_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enforcement_version = Some(v.into());
        self
    }
    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(
        mut self,
        v: impl Into<BlockAssignable<IamPrincipalAccessBoundaryPolicyDetailsElRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rules = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IamPrincipalAccessBoundaryPolicyDetailsEl {
    type O = BlockAssignable<IamPrincipalAccessBoundaryPolicyDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamPrincipalAccessBoundaryPolicyDetailsEl {}
impl BuildIamPrincipalAccessBoundaryPolicyDetailsEl {
    pub fn build(self) -> IamPrincipalAccessBoundaryPolicyDetailsEl {
        IamPrincipalAccessBoundaryPolicyDetailsEl {
            enforcement_version: core::default::Default::default(),
            rules: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IamPrincipalAccessBoundaryPolicyDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamPrincipalAccessBoundaryPolicyDetailsElRef {
    fn new(shared: StackShared, base: String) -> IamPrincipalAccessBoundaryPolicyDetailsElRef {
        IamPrincipalAccessBoundaryPolicyDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamPrincipalAccessBoundaryPolicyDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enforcement_version` after provisioning.\nThe version number that indicates which Google Cloud services\nare included in the enforcement (e.g. \\\"latest\\\", \\\"1\\\", ...). If empty, the\nPAB policy version will be set to the current latest version, and this version\nwon't get updated when new versions are released."]
    pub fn enforcement_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforcement_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<IamPrincipalAccessBoundaryPolicyDetailsElRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }
}
#[derive(Serialize)]
pub struct IamPrincipalAccessBoundaryPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl IamPrincipalAccessBoundaryPolicyTimeoutsEl {
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
impl ToListMappable for IamPrincipalAccessBoundaryPolicyTimeoutsEl {
    type O = BlockAssignable<IamPrincipalAccessBoundaryPolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamPrincipalAccessBoundaryPolicyTimeoutsEl {}
impl BuildIamPrincipalAccessBoundaryPolicyTimeoutsEl {
    pub fn build(self) -> IamPrincipalAccessBoundaryPolicyTimeoutsEl {
        IamPrincipalAccessBoundaryPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct IamPrincipalAccessBoundaryPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamPrincipalAccessBoundaryPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> IamPrincipalAccessBoundaryPolicyTimeoutsElRef {
        IamPrincipalAccessBoundaryPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamPrincipalAccessBoundaryPolicyTimeoutsElRef {
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
struct IamPrincipalAccessBoundaryPolicyDynamic {
    details: Option<DynamicBlock<IamPrincipalAccessBoundaryPolicyDetailsEl>>,
}
