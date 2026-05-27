use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct FirebaseAppHostingDomainData {
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
    deletion_policy: Option<PrimField<String>>,
    domain_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serve: Option<Vec<FirebaseAppHostingDomainServeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FirebaseAppHostingDomainTimeoutsEl>,
    dynamic: FirebaseAppHostingDomainDynamic,
}
struct FirebaseAppHostingDomain_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FirebaseAppHostingDomainData>,
}
#[derive(Clone)]
pub struct FirebaseAppHostingDomain(Rc<FirebaseAppHostingDomain_>);
impl FirebaseAppHostingDomain {
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
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `serve`.\n"]
    pub fn set_serve(self, v: impl Into<BlockAssignable<FirebaseAppHostingDomainServeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().serve = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.serve = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FirebaseAppHostingDomainTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `custom_domain_status` after provisioning.\nThe status of a custom domain's linkage to the Backend."]
    pub fn custom_domain_status(&self) -> ListRef<FirebaseAppHostingDomainCustomDomainStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_domain_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nTime at which the domain was deleted."]
    pub fn delete_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\nId of the domain to create.\nMust be a valid domain name, such as \"foo.com\""]
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
    #[doc = "Get a reference to the value of field `purge_time` after provisioning.\nTime at which a soft-deleted domain will be purged, rendering in\npermanently deleted."]
    pub fn purge_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.purge_time", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `serve` after provisioning.\n"]
    pub fn serve(&self) -> ListRef<FirebaseAppHostingDomainServeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.serve", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaseAppHostingDomainTimeoutsElRef {
        FirebaseAppHostingDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for FirebaseAppHostingDomain {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for FirebaseAppHostingDomain {}
impl ToListMappable for FirebaseAppHostingDomain {
    type O = ListRef<FirebaseAppHostingDomainRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for FirebaseAppHostingDomain_ {
    fn extract_resource_type(&self) -> String {
        "google_firebase_app_hosting_domain".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildFirebaseAppHostingDomain {
    pub tf_id: String,
    #[doc = "The ID of the Backend that this Domain is associated with"]
    pub backend: PrimField<String>,
    #[doc = "Id of the domain to create.\nMust be a valid domain name, such as \"foo.com\""]
    pub domain_id: PrimField<String>,
    #[doc = "The location of the Backend that this Domain is associated with"]
    pub location: PrimField<String>,
}
impl BuildFirebaseAppHostingDomain {
    pub fn build(self, stack: &mut Stack) -> FirebaseAppHostingDomain {
        let out = FirebaseAppHostingDomain(Rc::new(FirebaseAppHostingDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FirebaseAppHostingDomainData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                backend: self.backend,
                deletion_policy: core::default::Default::default(),
                domain_id: self.domain_id,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                serve: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct FirebaseAppHostingDomainRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingDomainRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl FirebaseAppHostingDomainRef {
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
    #[doc = "Get a reference to the value of field `custom_domain_status` after provisioning.\nThe status of a custom domain's linkage to the Backend."]
    pub fn custom_domain_status(&self) -> ListRef<FirebaseAppHostingDomainCustomDomainStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_domain_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nTime at which the domain was deleted."]
    pub fn delete_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\nId of the domain to create.\nMust be a valid domain name, such as \"foo.com\""]
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
    #[doc = "Get a reference to the value of field `purge_time` after provisioning.\nTime at which a soft-deleted domain will be purged, rendering in\npermanently deleted."]
    pub fn purge_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.purge_time", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `serve` after provisioning.\n"]
    pub fn serve(&self) -> ListRef<FirebaseAppHostingDomainServeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.serve", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaseAppHostingDomainTimeoutsElRef {
        FirebaseAppHostingDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingDomainCustomDomainStatusElIssuesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl FirebaseAppHostingDomainCustomDomainStatusElIssuesEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.details = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable for FirebaseAppHostingDomainCustomDomainStatusElIssuesEl {
    type O = BlockAssignable<FirebaseAppHostingDomainCustomDomainStatusElIssuesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingDomainCustomDomainStatusElIssuesEl {}
impl BuildFirebaseAppHostingDomainCustomDomainStatusElIssuesEl {
    pub fn build(self) -> FirebaseAppHostingDomainCustomDomainStatusElIssuesEl {
        FirebaseAppHostingDomainCustomDomainStatusElIssuesEl {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct FirebaseAppHostingDomainCustomDomainStatusElIssuesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingDomainCustomDomainStatusElIssuesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FirebaseAppHostingDomainCustomDomainStatusElIssuesElRef {
        FirebaseAppHostingDomainCustomDomainStatusElIssuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingDomainCustomDomainStatusElIssuesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.details", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElCheckErrorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElCheckErrorEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.details = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable
    for FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElCheckErrorEl
{
    type O = BlockAssignable<
        FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElCheckErrorEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElCheckErrorEl
{}
impl BuildFirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElCheckErrorEl {
    pub fn build(
        self,
    ) -> FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElCheckErrorEl {
        FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElCheckErrorEl {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElCheckErrorElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElCheckErrorElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElCheckErrorElRef
    {
        FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElCheckErrorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElCheckErrorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.details", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElRecordsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rdata: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relevant_state: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_action: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElRecordsEl {
    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain_name = Some(v.into());
        self
    }
    #[doc = "Set the field `rdata`.\n"]
    pub fn set_rdata(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rdata = Some(v.into());
        self
    }
    #[doc = "Set the field `relevant_state`.\n"]
    pub fn set_relevant_state(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.relevant_state = Some(v.into());
        self
    }
    #[doc = "Set the field `required_action`.\n"]
    pub fn set_required_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.required_action = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable
    for FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElRecordsEl
{
    type O = BlockAssignable<
        FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElRecordsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElRecordsEl
{}
impl BuildFirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElRecordsEl {
    pub fn build(
        self,
    ) -> FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElRecordsEl {
        FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElRecordsEl {
            domain_name: core::default::Default::default(),
            rdata: core::default::Default::default(),
            relevant_state: core::default::Default::default(),
            required_action: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElRecordsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElRecordsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElRecordsElRef {
        FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElRecordsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElRecordsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }
    #[doc = "Get a reference to the value of field `rdata` after provisioning.\n"]
    pub fn rdata(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rdata", self.base))
    }
    #[doc = "Get a reference to the value of field `relevant_state` after provisioning.\n"]
    pub fn relevant_state(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.relevant_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `required_action` after provisioning.\n"]
    pub fn required_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.required_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    check_error: Option<
        ListField<
            FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElCheckErrorEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    records: Option<
        ListField<
            FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElRecordsEl,
        >,
    >,
}
impl FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredEl {
    #[doc = "Set the field `check_error`.\n"]
    pub fn set_check_error(
        mut self,
        v : impl Into < ListField < FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElCheckErrorEl > >,
    ) -> Self {
        self.check_error = Some(v.into());
        self
    }
    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain_name = Some(v.into());
        self
    }
    #[doc = "Set the field `records`.\n"]
    pub fn set_records(
        mut self,
        v: impl Into<
            ListField<
                FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElRecordsEl,
            >,
        >,
    ) -> Self {
        self.records = Some(v.into());
        self
    }
}
impl ToListMappable for FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredEl {
    type O =
        BlockAssignable<FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredEl {}
impl BuildFirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredEl {
    pub fn build(
        self,
    ) -> FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredEl {
        FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredEl {
            check_error: core::default::Default::default(),
            domain_name: core::default::Default::default(),
            records: core::default::Default::default(),
        }
    }
}
pub struct FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElRef {
        FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `check_error` after provisioning.\n"]
    pub fn check_error(
        &self,
    ) -> ListRef<
        FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElCheckErrorElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.check_error", self.base))
    }
    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }
    #[doc = "Get a reference to the value of field `records` after provisioning.\n"]
    pub fn records(
        &self,
    ) -> ListRef<
        FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElRecordsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.records", self.base))
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElCheckErrorEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElCheckErrorEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.details = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable
    for FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElCheckErrorEl
{
    type O = BlockAssignable<
        FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElCheckErrorEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElCheckErrorEl
{}
impl BuildFirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElCheckErrorEl {
    pub fn build(
        self,
    ) -> FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElCheckErrorEl
    {
        FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElCheckErrorEl {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElCheckErrorElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElCheckErrorElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElCheckErrorElRef
    {
        FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElCheckErrorElRef { shared : shared , base : base . to_string () , }
    }
}
impl FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElCheckErrorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.details", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElRecordsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rdata: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relevant_state: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_action: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElRecordsEl {
    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain_name = Some(v.into());
        self
    }
    #[doc = "Set the field `rdata`.\n"]
    pub fn set_rdata(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rdata = Some(v.into());
        self
    }
    #[doc = "Set the field `relevant_state`.\n"]
    pub fn set_relevant_state(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.relevant_state = Some(v.into());
        self
    }
    #[doc = "Set the field `required_action`.\n"]
    pub fn set_required_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.required_action = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable
    for FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElRecordsEl
{
    type O = BlockAssignable<
        FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElRecordsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElRecordsEl
{}
impl BuildFirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElRecordsEl {
    pub fn build(
        self,
    ) -> FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElRecordsEl {
        FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElRecordsEl {
            domain_name: core::default::Default::default(),
            rdata: core::default::Default::default(),
            relevant_state: core::default::Default::default(),
            required_action: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElRecordsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElRecordsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElRecordsElRef
    {
        FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElRecordsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElRecordsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }
    #[doc = "Get a reference to the value of field `rdata` after provisioning.\n"]
    pub fn rdata(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rdata", self.base))
    }
    #[doc = "Get a reference to the value of field `relevant_state` after provisioning.\n"]
    pub fn relevant_state(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.relevant_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `required_action` after provisioning.\n"]
    pub fn required_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.required_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredEl { # [serde (skip_serializing_if = "Option::is_none")] check_error : Option < ListField < FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElCheckErrorEl > > , # [serde (skip_serializing_if = "Option::is_none")] domain_name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] records : Option < ListField < FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElRecordsEl > > , }
impl FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredEl {
    #[doc = "Set the field `check_error`.\n"]
    pub fn set_check_error(
        mut self,
        v : impl Into < ListField < FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElCheckErrorEl > >,
    ) -> Self {
        self.check_error = Some(v.into());
        self
    }
    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain_name = Some(v.into());
        self
    }
    #[doc = "Set the field `records`.\n"]
    pub fn set_records(
        mut self,
        v : impl Into < ListField < FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElRecordsEl > >,
    ) -> Self {
        self.records = Some(v.into());
        self
    }
}
impl ToListMappable
    for FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredEl
{
    type O = BlockAssignable<
        FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredEl {}
impl BuildFirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredEl {
    pub fn build(
        self,
    ) -> FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredEl {
        FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredEl {
            check_error: core::default::Default::default(),
            domain_name: core::default::Default::default(),
            records: core::default::Default::default(),
        }
    }
}
pub struct FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElRef {
        FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `check_error` after provisioning.\n"]
    pub fn check_error(
        &self,
    ) -> ListRef<
        FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElCheckErrorElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.check_error", self.base))
    }
    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }
    #[doc = "Get a reference to the value of field `records` after provisioning.\n"]
    pub fn records(
        &self,
    ) -> ListRef<
        FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElRecordsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.records", self.base))
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    check_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desired: Option<
        ListField<FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    discovered: Option<
        ListField<FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name: Option<PrimField<String>>,
}
impl FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesEl {
    #[doc = "Set the field `check_time`.\n"]
    pub fn set_check_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.check_time = Some(v.into());
        self
    }
    #[doc = "Set the field `desired`.\n"]
    pub fn set_desired(
        mut self,
        v: impl Into<
            ListField<FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredEl>,
        >,
    ) -> Self {
        self.desired = Some(v.into());
        self
    }
    #[doc = "Set the field `discovered`.\n"]
    pub fn set_discovered(
        mut self,
        v: impl Into<
            ListField<FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredEl>,
        >,
    ) -> Self {
        self.discovered = Some(v.into());
        self
    }
    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain_name = Some(v.into());
        self
    }
}
impl ToListMappable for FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesEl {
    type O = BlockAssignable<FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesEl {}
impl BuildFirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesEl {
    pub fn build(self) -> FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesEl {
        FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesEl {
            check_time: core::default::Default::default(),
            desired: core::default::Default::default(),
            discovered: core::default::Default::default(),
            domain_name: core::default::Default::default(),
        }
    }
}
pub struct FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElRef {
        FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `check_time` after provisioning.\n"]
    pub fn check_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.check_time", self.base))
    }
    #[doc = "Get a reference to the value of field `desired` after provisioning.\n"]
    pub fn desired(
        &self,
    ) -> ListRef<FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDesiredElRef> {
        ListRef::new(self.shared().clone(), format!("{}.desired", self.base))
    }
    #[doc = "Get a reference to the value of field `discovered` after provisioning.\n"]
    pub fn discovered(
        &self,
    ) -> ListRef<FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElDiscoveredElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.discovered", self.base))
    }
    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingDomainCustomDomainStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cert_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issues: Option<ListField<FirebaseAppHostingDomainCustomDomainStatusElIssuesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ownership_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_dns_updates:
        Option<ListField<FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesEl>>,
}
impl FirebaseAppHostingDomainCustomDomainStatusEl {
    #[doc = "Set the field `cert_state`.\n"]
    pub fn set_cert_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cert_state = Some(v.into());
        self
    }
    #[doc = "Set the field `host_state`.\n"]
    pub fn set_host_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_state = Some(v.into());
        self
    }
    #[doc = "Set the field `issues`.\n"]
    pub fn set_issues(
        mut self,
        v: impl Into<ListField<FirebaseAppHostingDomainCustomDomainStatusElIssuesEl>>,
    ) -> Self {
        self.issues = Some(v.into());
        self
    }
    #[doc = "Set the field `ownership_state`.\n"]
    pub fn set_ownership_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ownership_state = Some(v.into());
        self
    }
    #[doc = "Set the field `required_dns_updates`.\n"]
    pub fn set_required_dns_updates(
        mut self,
        v: impl Into<ListField<FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesEl>>,
    ) -> Self {
        self.required_dns_updates = Some(v.into());
        self
    }
}
impl ToListMappable for FirebaseAppHostingDomainCustomDomainStatusEl {
    type O = BlockAssignable<FirebaseAppHostingDomainCustomDomainStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingDomainCustomDomainStatusEl {}
impl BuildFirebaseAppHostingDomainCustomDomainStatusEl {
    pub fn build(self) -> FirebaseAppHostingDomainCustomDomainStatusEl {
        FirebaseAppHostingDomainCustomDomainStatusEl {
            cert_state: core::default::Default::default(),
            host_state: core::default::Default::default(),
            issues: core::default::Default::default(),
            ownership_state: core::default::Default::default(),
            required_dns_updates: core::default::Default::default(),
        }
    }
}
pub struct FirebaseAppHostingDomainCustomDomainStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingDomainCustomDomainStatusElRef {
    fn new(shared: StackShared, base: String) -> FirebaseAppHostingDomainCustomDomainStatusElRef {
        FirebaseAppHostingDomainCustomDomainStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingDomainCustomDomainStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cert_state` after provisioning.\n"]
    pub fn cert_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cert_state", self.base))
    }
    #[doc = "Get a reference to the value of field `host_state` after provisioning.\n"]
    pub fn host_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_state", self.base))
    }
    #[doc = "Get a reference to the value of field `issues` after provisioning.\n"]
    pub fn issues(&self) -> ListRef<FirebaseAppHostingDomainCustomDomainStatusElIssuesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.issues", self.base))
    }
    #[doc = "Get a reference to the value of field `ownership_state` after provisioning.\n"]
    pub fn ownership_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ownership_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `required_dns_updates` after provisioning.\n"]
    pub fn required_dns_updates(
        &self,
    ) -> ListRef<FirebaseAppHostingDomainCustomDomainStatusElRequiredDnsUpdatesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.required_dns_updates", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingDomainServeElRedirectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    uri: PrimField<String>,
}
impl FirebaseAppHostingDomainServeElRedirectEl {
    #[doc = "Set the field `status`.\nThe status code to use in a redirect response. Must be a valid HTTP 3XX\nstatus code. Defaults to 302 if not present."]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}
impl ToListMappable for FirebaseAppHostingDomainServeElRedirectEl {
    type O = BlockAssignable<FirebaseAppHostingDomainServeElRedirectEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingDomainServeElRedirectEl {
    #[doc = "The URI of the redirect's intended destination. This URI will be\nprepended to the original request path. URI without a scheme are\nassumed to be HTTPS."]
    pub uri: PrimField<String>,
}
impl BuildFirebaseAppHostingDomainServeElRedirectEl {
    pub fn build(self) -> FirebaseAppHostingDomainServeElRedirectEl {
        FirebaseAppHostingDomainServeElRedirectEl {
            status: core::default::Default::default(),
            uri: self.uri,
        }
    }
}
pub struct FirebaseAppHostingDomainServeElRedirectElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingDomainServeElRedirectElRef {
    fn new(shared: StackShared, base: String) -> FirebaseAppHostingDomainServeElRedirectElRef {
        FirebaseAppHostingDomainServeElRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingDomainServeElRedirectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status code to use in a redirect response. Must be a valid HTTP 3XX\nstatus code. Defaults to 302 if not present."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe URI of the redirect's intended destination. This URI will be\nprepended to the original request path. URI without a scheme are\nassumed to be HTTPS."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct FirebaseAppHostingDomainServeElDynamic {
    redirect: Option<DynamicBlock<FirebaseAppHostingDomainServeElRedirectEl>>,
}
#[derive(Serialize)]
pub struct FirebaseAppHostingDomainServeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect: Option<Vec<FirebaseAppHostingDomainServeElRedirectEl>>,
    dynamic: FirebaseAppHostingDomainServeElDynamic,
}
impl FirebaseAppHostingDomainServeEl {
    #[doc = "Set the field `redirect`.\n"]
    pub fn set_redirect(
        mut self,
        v: impl Into<BlockAssignable<FirebaseAppHostingDomainServeElRedirectEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.redirect = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.redirect = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FirebaseAppHostingDomainServeEl {
    type O = BlockAssignable<FirebaseAppHostingDomainServeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingDomainServeEl {}
impl BuildFirebaseAppHostingDomainServeEl {
    pub fn build(self) -> FirebaseAppHostingDomainServeEl {
        FirebaseAppHostingDomainServeEl {
            redirect: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FirebaseAppHostingDomainServeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingDomainServeElRef {
    fn new(shared: StackShared, base: String) -> FirebaseAppHostingDomainServeElRef {
        FirebaseAppHostingDomainServeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingDomainServeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> ListRef<FirebaseAppHostingDomainServeElRedirectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingDomainTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl FirebaseAppHostingDomainTimeoutsEl {
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
impl ToListMappable for FirebaseAppHostingDomainTimeoutsEl {
    type O = BlockAssignable<FirebaseAppHostingDomainTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingDomainTimeoutsEl {}
impl BuildFirebaseAppHostingDomainTimeoutsEl {
    pub fn build(self) -> FirebaseAppHostingDomainTimeoutsEl {
        FirebaseAppHostingDomainTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct FirebaseAppHostingDomainTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingDomainTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FirebaseAppHostingDomainTimeoutsElRef {
        FirebaseAppHostingDomainTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingDomainTimeoutsElRef {
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
struct FirebaseAppHostingDomainDynamic {
    serve: Option<DynamicBlock<FirebaseAppHostingDomainServeEl>>,
}
