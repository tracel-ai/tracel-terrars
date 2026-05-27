use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AccessContextManagerServicePerimeterIngressPolicyData {
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
    perimeter: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress_from: Option<Vec<AccessContextManagerServicePerimeterIngressPolicyIngressFromEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress_to: Option<Vec<AccessContextManagerServicePerimeterIngressPolicyIngressToEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AccessContextManagerServicePerimeterIngressPolicyTimeoutsEl>,
    dynamic: AccessContextManagerServicePerimeterIngressPolicyDynamic,
}
struct AccessContextManagerServicePerimeterIngressPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AccessContextManagerServicePerimeterIngressPolicyData>,
}
#[derive(Clone)]
pub struct AccessContextManagerServicePerimeterIngressPolicy(
    Rc<AccessContextManagerServicePerimeterIngressPolicy_>,
);
impl AccessContextManagerServicePerimeterIngressPolicy {
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
    #[doc = "Set the field `title`.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn set_title(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().title = Some(v.into());
        self
    }
    #[doc = "Set the field `ingress_from`.\n"]
    pub fn set_ingress_from(
        self,
        v: impl Into<BlockAssignable<AccessContextManagerServicePerimeterIngressPolicyIngressFromEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ingress_from = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ingress_from = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ingress_to`.\n"]
    pub fn set_ingress_to(
        self,
        v: impl Into<BlockAssignable<AccessContextManagerServicePerimeterIngressPolicyIngressToEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ingress_to = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ingress_to = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<AccessContextManagerServicePerimeterIngressPolicyTimeoutsEl>,
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe perimeter etag is internally used to prevent overwriting the list of policies on PATCH calls. It is retrieved from the same GET perimeter API call that's used to get the current list of policies. The policy defined in this resource is added or removed from that list, and then this etag is sent with the PATCH call along with the updated policies."]
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
    #[doc = "Get a reference to the value of field `perimeter` after provisioning.\nThe name of the Service Perimeter to add this resource to."]
    pub fn perimeter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.perimeter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.title", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ingress_from` after provisioning.\n"]
    pub fn ingress_from(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimeterIngressPolicyIngressFromElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ingress_from", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ingress_to` after provisioning.\n"]
    pub fn ingress_to(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimeterIngressPolicyIngressToElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ingress_to", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerServicePerimeterIngressPolicyTimeoutsElRef {
        AccessContextManagerServicePerimeterIngressPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for AccessContextManagerServicePerimeterIngressPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AccessContextManagerServicePerimeterIngressPolicy {}
impl ToListMappable for AccessContextManagerServicePerimeterIngressPolicy {
    type O = ListRef<AccessContextManagerServicePerimeterIngressPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AccessContextManagerServicePerimeterIngressPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_access_context_manager_service_perimeter_ingress_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAccessContextManagerServicePerimeterIngressPolicy {
    pub tf_id: String,
    #[doc = "The name of the Service Perimeter to add this resource to."]
    pub perimeter: PrimField<String>,
}
impl BuildAccessContextManagerServicePerimeterIngressPolicy {
    pub fn build(self, stack: &mut Stack) -> AccessContextManagerServicePerimeterIngressPolicy {
        let out = AccessContextManagerServicePerimeterIngressPolicy(Rc::new(
            AccessContextManagerServicePerimeterIngressPolicy_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(AccessContextManagerServicePerimeterIngressPolicyData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    deletion_policy: core::default::Default::default(),
                    id: core::default::Default::default(),
                    perimeter: self.perimeter,
                    title: core::default::Default::default(),
                    ingress_from: core::default::Default::default(),
                    ingress_to: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AccessContextManagerServicePerimeterIngressPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterIngressPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AccessContextManagerServicePerimeterIngressPolicyRef {
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe perimeter etag is internally used to prevent overwriting the list of policies on PATCH calls. It is retrieved from the same GET perimeter API call that's used to get the current list of policies. The policy defined in this resource is added or removed from that list, and then this etag is sent with the PATCH call along with the updated policies."]
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
    #[doc = "Get a reference to the value of field `perimeter` after provisioning.\nThe name of the Service Perimeter to add this resource to."]
    pub fn perimeter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.perimeter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.title", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ingress_from` after provisioning.\n"]
    pub fn ingress_from(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimeterIngressPolicyIngressFromElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ingress_from", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ingress_to` after provisioning.\n"]
    pub fn ingress_to(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimeterIngressPolicyIngressToElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ingress_to", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerServicePerimeterIngressPolicyTimeoutsElRef {
        AccessContextManagerServicePerimeterIngressPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<PrimField<String>>,
}
impl AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesEl {
    #[doc = "Set the field `access_level`.\nAn 'AccessLevel' resource name that allow resources within the\n'ServicePerimeters' to be accessed from the internet. 'AccessLevels' listed\nmust be in the same policy as this 'ServicePerimeter'. Referencing a nonexistent\n'AccessLevel' will cause an error. If no 'AccessLevel' names are listed,\nresources within the perimeter can only be accessed via Google Cloud calls\nwith request origins within the perimeter.\nExample 'accessPolicies/MY_POLICY/accessLevels/MY_LEVEL.'\nIf * is specified, then all IngressSources will be allowed."]
    pub fn set_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_level = Some(v.into());
        self
    }
    #[doc = "Set the field `resource`.\nA Google Cloud resource that is allowed to ingress the perimeter.\nRequests from these resources will be allowed to access perimeter data.\nCurrently only projects and VPCs are allowed.\nProject format: 'projects/{projectNumber}'\nVPC network format:\n'//compute.googleapis.com/projects/{PROJECT_ID}/global/networks/{NAME}'.\nThe project may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the case\nof allowing all Google Cloud resources only is not supported."]
    pub fn set_resource(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource = Some(v.into());
        self
    }
}
impl ToListMappable for AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesEl {
    type O =
        BlockAssignable<AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesEl {}
impl BuildAccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesEl {
        AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesEl {
            access_level: core::default::Default::default(),
            resource: core::default::Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesElRef {
        AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_level` after provisioning.\nAn 'AccessLevel' resource name that allow resources within the\n'ServicePerimeters' to be accessed from the internet. 'AccessLevels' listed\nmust be in the same policy as this 'ServicePerimeter'. Referencing a nonexistent\n'AccessLevel' will cause an error. If no 'AccessLevel' names are listed,\nresources within the perimeter can only be accessed via Google Cloud calls\nwith request origins within the perimeter.\nExample 'accessPolicies/MY_POLICY/accessLevels/MY_LEVEL.'\nIf * is specified, then all IngressSources will be allowed."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.base))
    }
    #[doc = "Get a reference to the value of field `resource` after provisioning.\nA Google Cloud resource that is allowed to ingress the perimeter.\nRequests from these resources will be allowed to access perimeter data.\nCurrently only projects and VPCs are allowed.\nProject format: 'projects/{projectNumber}'\nVPC network format:\n'//compute.googleapis.com/projects/{PROJECT_ID}/global/networks/{NAME}'.\nThe project may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the case\nof allowing all Google Cloud resources only is not supported."]
    pub fn resource(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterIngressPolicyIngressFromElDynamic {
    sources: Option<
        DynamicBlock<AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesEl>,
    >,
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterIngressPolicyIngressFromEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identities: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sources: Option<Vec<AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesEl>>,
    dynamic: AccessContextManagerServicePerimeterIngressPolicyIngressFromElDynamic,
}
impl AccessContextManagerServicePerimeterIngressPolicyIngressFromEl {
    #[doc = "Set the field `identities`.\nIdentities can be an individual user, service account, Google group,\nor third-party identity. For third-party identity, only single identities\nare supported and other identity types are not supported.The v1 identities\nthat have the prefix user, group and serviceAccount in\nhttps://cloud.google.com/iam/docs/principal-identifiers#v1 are supported."]
    pub fn set_identities(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.identities = Some(v.into());
        self
    }
    #[doc = "Set the field `identity_type`.\nSpecifies the type of identities that are allowed access from outside the\nperimeter. If left unspecified, then members of 'identities' field will be\nallowed access. Possible values: [\"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn set_identity_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_type = Some(v.into());
        self
    }
    #[doc = "Set the field `sources`.\n"]
    pub fn set_sources(
        mut self,
        v: impl Into<
            BlockAssignable<
                AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sources = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerServicePerimeterIngressPolicyIngressFromEl {
    type O = BlockAssignable<AccessContextManagerServicePerimeterIngressPolicyIngressFromEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterIngressPolicyIngressFromEl {}
impl BuildAccessContextManagerServicePerimeterIngressPolicyIngressFromEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterIngressPolicyIngressFromEl {
        AccessContextManagerServicePerimeterIngressPolicyIngressFromEl {
            identities: core::default::Default::default(),
            identity_type: core::default::Default::default(),
            sources: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterIngressPolicyIngressFromElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterIngressPolicyIngressFromElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterIngressPolicyIngressFromElRef {
        AccessContextManagerServicePerimeterIngressPolicyIngressFromElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterIngressPolicyIngressFromElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `identities` after provisioning.\nIdentities can be an individual user, service account, Google group,\nor third-party identity. For third-party identity, only single identities\nare supported and other identity types are not supported.The v1 identities\nthat have the prefix user, group and serviceAccount in\nhttps://cloud.google.com/iam/docs/principal-identifiers#v1 are supported."]
    pub fn identities(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.identities", self.base))
    }
    #[doc = "Get a reference to the value of field `identity_type` after provisioning.\nSpecifies the type of identities that are allowed access from outside the\nperimeter. If left unspecified, then members of 'identities' field will be\nallowed access. Possible values: [\"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn identity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission: Option<PrimField<String>>,
}
impl AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsEl {
    #[doc = "Set the field `method`.\nValue for method should be a valid method name for the corresponding\nserviceName in 'ApiOperation'. If '*' used as value for 'method', then\nALL methods and permissions are allowed."]
    pub fn set_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.method = Some(v.into());
        self
    }
    #[doc = "Set the field `permission`.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."]
    pub fn set_permission(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.permission = Some(v.into());
        self
    }
}
impl ToListMappable
    for AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsEl
{
    type O = BlockAssignable<
        AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsEl
{}
impl
    BuildAccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsEl
{
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsEl
    {
        AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsEl {
            method: core::default::Default::default(),
            permission: core::default::Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsElRef
    {
        AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsElRef { shared : shared , base : base . to_string () , }
    }
}
impl AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `method` after provisioning.\nValue for method should be a valid method name for the corresponding\nserviceName in 'ApiOperation'. If '*' used as value for 'method', then\nALL methods and permissions are allowed."]
    pub fn method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }
    #[doc = "Get a reference to the value of field `permission` after provisioning.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."]
    pub fn permission(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElDynamic { method_selectors : Option < DynamicBlock < AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsEl >> , }
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsEl { # [serde (skip_serializing_if = "Option::is_none")] service_name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] method_selectors : Option < Vec < AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsEl > > , dynamic : AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElDynamic , }
impl AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsEl {
    #[doc = "Set the field `service_name`.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with 'serviceName'\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn set_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_name = Some(v.into());
        self
    }
    #[doc = "Set the field `method_selectors`.\n"]
    pub fn set_method_selectors(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.method_selectors = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.method_selectors = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsEl {
    type O =
        BlockAssignable<AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsEl {}
impl BuildAccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsEl {
        AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsEl {
            service_name: core::default::Default::default(),
            method_selectors: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElRef {
        AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service_name` after provisioning.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with 'serviceName'\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
    #[doc = "Get a reference to the value of field `method_selectors` after provisioning.\n"]    pub fn method_selectors (& self) -> ListRef < AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.method_selectors", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterIngressPolicyIngressToElDynamic {
    operations: Option<
        DynamicBlock<AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsEl>,
    >,
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterIngressPolicyIngressToEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operations:
        Option<Vec<AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsEl>>,
    dynamic: AccessContextManagerServicePerimeterIngressPolicyIngressToElDynamic,
}
impl AccessContextManagerServicePerimeterIngressPolicyIngressToEl {
    #[doc = "Set the field `resources`.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', protected by this 'ServicePerimeter'\nthat are allowed to be accessed by sources defined in the\ncorresponding 'IngressFrom'. A request matches if it contains\na resource in this list. If '*' is specified for resources,\nthen this 'IngressTo' rule will authorize access to all\nresources inside the perimeter, provided that the request\nalso matches the 'operations' field."]
    pub fn set_resources(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.resources = Some(v.into());
        self
    }
    #[doc = "Set the field `roles`.\nA list of IAM roles that represent the set of operations that the sources\nspecified in the corresponding 'IngressFrom'\nare allowed to perform."]
    pub fn set_roles(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.roles = Some(v.into());
        self
    }
    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v: impl Into<
            BlockAssignable<
                AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.operations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerServicePerimeterIngressPolicyIngressToEl {
    type O = BlockAssignable<AccessContextManagerServicePerimeterIngressPolicyIngressToEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterIngressPolicyIngressToEl {}
impl BuildAccessContextManagerServicePerimeterIngressPolicyIngressToEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterIngressPolicyIngressToEl {
        AccessContextManagerServicePerimeterIngressPolicyIngressToEl {
            resources: core::default::Default::default(),
            roles: core::default::Default::default(),
            operations: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterIngressPolicyIngressToElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterIngressPolicyIngressToElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterIngressPolicyIngressToElRef {
        AccessContextManagerServicePerimeterIngressPolicyIngressToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterIngressPolicyIngressToElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', protected by this 'ServicePerimeter'\nthat are allowed to be accessed by sources defined in the\ncorresponding 'IngressFrom'. A request matches if it contains\na resource in this list. If '*' is specified for resources,\nthen this 'IngressTo' rule will authorize access to all\nresources inside the perimeter, provided that the request\nalso matches the 'operations' field."]
    pub fn resources(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
    #[doc = "Get a reference to the value of field `roles` after provisioning.\nA list of IAM roles that represent the set of operations that the sources\nspecified in the corresponding 'IngressFrom'\nare allowed to perform."]
    pub fn roles(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.roles", self.base))
    }
    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterIngressPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl AccessContextManagerServicePerimeterIngressPolicyTimeoutsEl {
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
impl ToListMappable for AccessContextManagerServicePerimeterIngressPolicyTimeoutsEl {
    type O = BlockAssignable<AccessContextManagerServicePerimeterIngressPolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterIngressPolicyTimeoutsEl {}
impl BuildAccessContextManagerServicePerimeterIngressPolicyTimeoutsEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterIngressPolicyTimeoutsEl {
        AccessContextManagerServicePerimeterIngressPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterIngressPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterIngressPolicyTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterIngressPolicyTimeoutsElRef {
        AccessContextManagerServicePerimeterIngressPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterIngressPolicyTimeoutsElRef {
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
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterIngressPolicyDynamic {
    ingress_from:
        Option<DynamicBlock<AccessContextManagerServicePerimeterIngressPolicyIngressFromEl>>,
    ingress_to: Option<DynamicBlock<AccessContextManagerServicePerimeterIngressPolicyIngressToEl>>,
}
