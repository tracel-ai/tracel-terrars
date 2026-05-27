use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SiteVerificationWebResourceData {
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
    verification_method: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    site: Option<Vec<SiteVerificationWebResourceSiteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SiteVerificationWebResourceTimeoutsEl>,
    dynamic: SiteVerificationWebResourceDynamic,
}
struct SiteVerificationWebResource_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SiteVerificationWebResourceData>,
}
#[derive(Clone)]
pub struct SiteVerificationWebResource(Rc<SiteVerificationWebResource_>);
impl SiteVerificationWebResource {
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
    #[doc = "Set the field `site`.\n"]
    pub fn set_site(
        self,
        v: impl Into<BlockAssignable<SiteVerificationWebResourceSiteEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().site = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.site = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SiteVerificationWebResourceTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `owners` after provisioning.\nThe email addresses of all direct, verified owners of this exact property. Indirect owners —\nfor example verified owners of the containing domain—are not included in this list."]
    pub fn owners(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.owners", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `verification_method` after provisioning.\nThe verification method for the Site Verification system to use to verify\nthis site or domain. Possible values: [\"ANALYTICS\", \"DNS_CNAME\", \"DNS_TXT\", \"FILE\", \"META\", \"TAG_MANAGER\"]"]
    pub fn verification_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.verification_method", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `web_resource_id` after provisioning.\nThe string used to identify this web resource."]
    pub fn web_resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.web_resource_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `site` after provisioning.\n"]
    pub fn site(&self) -> ListRef<SiteVerificationWebResourceSiteElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.site", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SiteVerificationWebResourceTimeoutsElRef {
        SiteVerificationWebResourceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for SiteVerificationWebResource {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SiteVerificationWebResource {}
impl ToListMappable for SiteVerificationWebResource {
    type O = ListRef<SiteVerificationWebResourceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SiteVerificationWebResource_ {
    fn extract_resource_type(&self) -> String {
        "google_site_verification_web_resource".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSiteVerificationWebResource {
    pub tf_id: String,
    #[doc = "The verification method for the Site Verification system to use to verify\nthis site or domain. Possible values: [\"ANALYTICS\", \"DNS_CNAME\", \"DNS_TXT\", \"FILE\", \"META\", \"TAG_MANAGER\"]"]
    pub verification_method: PrimField<String>,
}
impl BuildSiteVerificationWebResource {
    pub fn build(self, stack: &mut Stack) -> SiteVerificationWebResource {
        let out = SiteVerificationWebResource(Rc::new(SiteVerificationWebResource_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SiteVerificationWebResourceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                verification_method: self.verification_method,
                site: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SiteVerificationWebResourceRef {
    shared: StackShared,
    base: String,
}
impl Ref for SiteVerificationWebResourceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SiteVerificationWebResourceRef {
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `owners` after provisioning.\nThe email addresses of all direct, verified owners of this exact property. Indirect owners —\nfor example verified owners of the containing domain—are not included in this list."]
    pub fn owners(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.owners", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `verification_method` after provisioning.\nThe verification method for the Site Verification system to use to verify\nthis site or domain. Possible values: [\"ANALYTICS\", \"DNS_CNAME\", \"DNS_TXT\", \"FILE\", \"META\", \"TAG_MANAGER\"]"]
    pub fn verification_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.verification_method", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `web_resource_id` after provisioning.\nThe string used to identify this web resource."]
    pub fn web_resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.web_resource_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `site` after provisioning.\n"]
    pub fn site(&self) -> ListRef<SiteVerificationWebResourceSiteElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.site", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SiteVerificationWebResourceTimeoutsElRef {
        SiteVerificationWebResourceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SiteVerificationWebResourceSiteEl {
    identifier: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl SiteVerificationWebResourceSiteEl {}
impl ToListMappable for SiteVerificationWebResourceSiteEl {
    type O = BlockAssignable<SiteVerificationWebResourceSiteEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSiteVerificationWebResourceSiteEl {
    #[doc = "The site identifier. If the type is set to SITE, the identifier is a URL. If the type is\nset to INET_DOMAIN, the identifier is a domain name."]
    pub identifier: PrimField<String>,
    #[doc = "The type of resource to be verified. Possible values: [\"INET_DOMAIN\", \"SITE\"]"]
    pub type_: PrimField<String>,
}
impl BuildSiteVerificationWebResourceSiteEl {
    pub fn build(self) -> SiteVerificationWebResourceSiteEl {
        SiteVerificationWebResourceSiteEl {
            identifier: self.identifier,
            type_: self.type_,
        }
    }
}
pub struct SiteVerificationWebResourceSiteElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SiteVerificationWebResourceSiteElRef {
    fn new(shared: StackShared, base: String) -> SiteVerificationWebResourceSiteElRef {
        SiteVerificationWebResourceSiteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SiteVerificationWebResourceSiteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `identifier` after provisioning.\nThe site identifier. If the type is set to SITE, the identifier is a URL. If the type is\nset to INET_DOMAIN, the identifier is a domain name."]
    pub fn identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of resource to be verified. Possible values: [\"INET_DOMAIN\", \"SITE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct SiteVerificationWebResourceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl SiteVerificationWebResourceTimeoutsEl {
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
impl ToListMappable for SiteVerificationWebResourceTimeoutsEl {
    type O = BlockAssignable<SiteVerificationWebResourceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSiteVerificationWebResourceTimeoutsEl {}
impl BuildSiteVerificationWebResourceTimeoutsEl {
    pub fn build(self) -> SiteVerificationWebResourceTimeoutsEl {
        SiteVerificationWebResourceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct SiteVerificationWebResourceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SiteVerificationWebResourceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SiteVerificationWebResourceTimeoutsElRef {
        SiteVerificationWebResourceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SiteVerificationWebResourceTimeoutsElRef {
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
struct SiteVerificationWebResourceDynamic {
    site: Option<DynamicBlock<SiteVerificationWebResourceSiteEl>>,
}
