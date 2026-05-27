use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DiscoveryEngineTargetSiteData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    data_store_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exact_match: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    provided_uri_pattern: PrimField<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DiscoveryEngineTargetSiteTimeoutsEl>,
}
struct DiscoveryEngineTargetSite_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DiscoveryEngineTargetSiteData>,
}
#[derive(Clone)]
pub struct DiscoveryEngineTargetSite(Rc<DiscoveryEngineTargetSite_>);
impl DiscoveryEngineTargetSite {
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
    #[doc = "Set the field `exact_match`.\nIf set to false, a uri_pattern is generated to include all pages whose\naddress contains the provided_uri_pattern. If set to true, an uri_pattern\nis generated to try to be an exact match of the provided_uri_pattern or\njust the specific page if the provided_uri_pattern is a specific one.\nprovided_uri_pattern is always normalized to generate the URI pattern to\nbe used by the search engine."]
    pub fn set_exact_match(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().exact_match = Some(v.into());
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
    #[doc = "Set the field `type_`.\nThe possible target site types. Possible values: [\"INCLUDE\", \"EXCLUDE\"]"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DiscoveryEngineTargetSiteTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `data_store_id` after provisioning.\nThe unique id of the data store."]
    pub fn data_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_store_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `exact_match` after provisioning.\nIf set to false, a uri_pattern is generated to include all pages whose\naddress contains the provided_uri_pattern. If set to true, an uri_pattern\nis generated to try to be an exact match of the provided_uri_pattern or\njust the specific page if the provided_uri_pattern is a specific one.\nprovided_uri_pattern is always normalized to generate the URI pattern to\nbe used by the search engine."]
    pub fn exact_match(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exact_match", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `failure_reason` after provisioning.\nSite search indexing failure reasons."]
    pub fn failure_reason(&self) -> ListRef<DiscoveryEngineTargetSiteFailureReasonElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.failure_reason", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generated_uri_pattern` after provisioning.\nThis is system-generated based on the 'provided_uri_pattern'."]
    pub fn generated_uri_pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generated_uri_pattern", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `indexing_status` after provisioning.\nThe indexing status."]
    pub fn indexing_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.indexing_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the target site. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}/siteSearchEngine/targetSites/{target_site_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024\ncharacters."]
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
    #[doc = "Get a reference to the value of field `provided_uri_pattern` after provisioning.\nThe user provided URI pattern from which the 'generated_uri_pattern' is\ngenerated."]
    pub fn provided_uri_pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provided_uri_pattern", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `root_domain_uri` after provisioning.\nRoot domain of the 'provided_uri_pattern'."]
    pub fn root_domain_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_domain_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `site_verification_info` after provisioning.\nSite ownership and validity verification status."]
    pub fn site_verification_info(
        &self,
    ) -> ListRef<DiscoveryEngineTargetSiteSiteVerificationInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.site_verification_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_site_id` after provisioning.\nThe unique id of the target site."]
    pub fn target_site_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_site_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe possible target site types. Possible values: [\"INCLUDE\", \"EXCLUDE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe target site's last updated time."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineTargetSiteTimeoutsElRef {
        DiscoveryEngineTargetSiteTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DiscoveryEngineTargetSite {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DiscoveryEngineTargetSite {}
impl ToListMappable for DiscoveryEngineTargetSite {
    type O = ListRef<DiscoveryEngineTargetSiteRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DiscoveryEngineTargetSite_ {
    fn extract_resource_type(&self) -> String {
        "google_discovery_engine_target_site".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDiscoveryEngineTargetSite {
    pub tf_id: String,
    #[doc = "The unique id of the data store."]
    pub data_store_id: PrimField<String>,
    #[doc = "The geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub location: PrimField<String>,
    #[doc = "The user provided URI pattern from which the 'generated_uri_pattern' is\ngenerated."]
    pub provided_uri_pattern: PrimField<String>,
}
impl BuildDiscoveryEngineTargetSite {
    pub fn build(self, stack: &mut Stack) -> DiscoveryEngineTargetSite {
        let out = DiscoveryEngineTargetSite(Rc::new(DiscoveryEngineTargetSite_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DiscoveryEngineTargetSiteData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                data_store_id: self.data_store_id,
                deletion_policy: core::default::Default::default(),
                exact_match: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                provided_uri_pattern: self.provided_uri_pattern,
                type_: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DiscoveryEngineTargetSiteRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineTargetSiteRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DiscoveryEngineTargetSiteRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_store_id` after provisioning.\nThe unique id of the data store."]
    pub fn data_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_store_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `exact_match` after provisioning.\nIf set to false, a uri_pattern is generated to include all pages whose\naddress contains the provided_uri_pattern. If set to true, an uri_pattern\nis generated to try to be an exact match of the provided_uri_pattern or\njust the specific page if the provided_uri_pattern is a specific one.\nprovided_uri_pattern is always normalized to generate the URI pattern to\nbe used by the search engine."]
    pub fn exact_match(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exact_match", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `failure_reason` after provisioning.\nSite search indexing failure reasons."]
    pub fn failure_reason(&self) -> ListRef<DiscoveryEngineTargetSiteFailureReasonElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.failure_reason", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generated_uri_pattern` after provisioning.\nThis is system-generated based on the 'provided_uri_pattern'."]
    pub fn generated_uri_pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generated_uri_pattern", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `indexing_status` after provisioning.\nThe indexing status."]
    pub fn indexing_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.indexing_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the target site. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}/siteSearchEngine/targetSites/{target_site_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024\ncharacters."]
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
    #[doc = "Get a reference to the value of field `provided_uri_pattern` after provisioning.\nThe user provided URI pattern from which the 'generated_uri_pattern' is\ngenerated."]
    pub fn provided_uri_pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provided_uri_pattern", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `root_domain_uri` after provisioning.\nRoot domain of the 'provided_uri_pattern'."]
    pub fn root_domain_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_domain_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `site_verification_info` after provisioning.\nSite ownership and validity verification status."]
    pub fn site_verification_info(
        &self,
    ) -> ListRef<DiscoveryEngineTargetSiteSiteVerificationInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.site_verification_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_site_id` after provisioning.\nThe unique id of the target site."]
    pub fn target_site_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_site_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe possible target site types. Possible values: [\"INCLUDE\", \"EXCLUDE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe target site's last updated time."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineTargetSiteTimeoutsElRef {
        DiscoveryEngineTargetSiteTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineTargetSiteFailureReasonElQuotaFailureEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    total_required_quota: Option<PrimField<f64>>,
}
impl DiscoveryEngineTargetSiteFailureReasonElQuotaFailureEl {
    #[doc = "Set the field `total_required_quota`.\n"]
    pub fn set_total_required_quota(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.total_required_quota = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineTargetSiteFailureReasonElQuotaFailureEl {
    type O = BlockAssignable<DiscoveryEngineTargetSiteFailureReasonElQuotaFailureEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineTargetSiteFailureReasonElQuotaFailureEl {}
impl BuildDiscoveryEngineTargetSiteFailureReasonElQuotaFailureEl {
    pub fn build(self) -> DiscoveryEngineTargetSiteFailureReasonElQuotaFailureEl {
        DiscoveryEngineTargetSiteFailureReasonElQuotaFailureEl {
            total_required_quota: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineTargetSiteFailureReasonElQuotaFailureElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineTargetSiteFailureReasonElQuotaFailureElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineTargetSiteFailureReasonElQuotaFailureElRef {
        DiscoveryEngineTargetSiteFailureReasonElQuotaFailureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineTargetSiteFailureReasonElQuotaFailureElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `total_required_quota` after provisioning.\n"]
    pub fn total_required_quota(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_required_quota", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineTargetSiteFailureReasonEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    quota_failure: Option<ListField<DiscoveryEngineTargetSiteFailureReasonElQuotaFailureEl>>,
}
impl DiscoveryEngineTargetSiteFailureReasonEl {
    #[doc = "Set the field `quota_failure`.\n"]
    pub fn set_quota_failure(
        mut self,
        v: impl Into<ListField<DiscoveryEngineTargetSiteFailureReasonElQuotaFailureEl>>,
    ) -> Self {
        self.quota_failure = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineTargetSiteFailureReasonEl {
    type O = BlockAssignable<DiscoveryEngineTargetSiteFailureReasonEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineTargetSiteFailureReasonEl {}
impl BuildDiscoveryEngineTargetSiteFailureReasonEl {
    pub fn build(self) -> DiscoveryEngineTargetSiteFailureReasonEl {
        DiscoveryEngineTargetSiteFailureReasonEl {
            quota_failure: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineTargetSiteFailureReasonElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineTargetSiteFailureReasonElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineTargetSiteFailureReasonElRef {
        DiscoveryEngineTargetSiteFailureReasonElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineTargetSiteFailureReasonElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `quota_failure` after provisioning.\n"]
    pub fn quota_failure(
        &self,
    ) -> ListRef<DiscoveryEngineTargetSiteFailureReasonElQuotaFailureElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.quota_failure", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineTargetSiteSiteVerificationInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    site_verification_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verify_time: Option<PrimField<String>>,
}
impl DiscoveryEngineTargetSiteSiteVerificationInfoEl {
    #[doc = "Set the field `site_verification_state`.\n"]
    pub fn set_site_verification_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.site_verification_state = Some(v.into());
        self
    }
    #[doc = "Set the field `verify_time`.\n"]
    pub fn set_verify_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.verify_time = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineTargetSiteSiteVerificationInfoEl {
    type O = BlockAssignable<DiscoveryEngineTargetSiteSiteVerificationInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineTargetSiteSiteVerificationInfoEl {}
impl BuildDiscoveryEngineTargetSiteSiteVerificationInfoEl {
    pub fn build(self) -> DiscoveryEngineTargetSiteSiteVerificationInfoEl {
        DiscoveryEngineTargetSiteSiteVerificationInfoEl {
            site_verification_state: core::default::Default::default(),
            verify_time: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineTargetSiteSiteVerificationInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineTargetSiteSiteVerificationInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineTargetSiteSiteVerificationInfoElRef {
        DiscoveryEngineTargetSiteSiteVerificationInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineTargetSiteSiteVerificationInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `site_verification_state` after provisioning.\n"]
    pub fn site_verification_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.site_verification_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `verify_time` after provisioning.\n"]
    pub fn verify_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.verify_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineTargetSiteTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl DiscoveryEngineTargetSiteTimeoutsEl {
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
impl ToListMappable for DiscoveryEngineTargetSiteTimeoutsEl {
    type O = BlockAssignable<DiscoveryEngineTargetSiteTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineTargetSiteTimeoutsEl {}
impl BuildDiscoveryEngineTargetSiteTimeoutsEl {
    pub fn build(self) -> DiscoveryEngineTargetSiteTimeoutsEl {
        DiscoveryEngineTargetSiteTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineTargetSiteTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineTargetSiteTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineTargetSiteTimeoutsElRef {
        DiscoveryEngineTargetSiteTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineTargetSiteTimeoutsElRef {
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
