use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudfrontDistributionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aliases: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    anycast_ip_list_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    continuous_deployment_policy_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_root_object: Option<PrimField<String>>,
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_ipv6_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_class: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retain_on_delete: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    staging: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for_deployment: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_acl_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_error_response: Option<Vec<CloudfrontDistributionCustomErrorResponseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_cache_behavior: Option<Vec<CloudfrontDistributionDefaultCacheBehaviorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_config: Option<Vec<CloudfrontDistributionLoggingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ordered_cache_behavior: Option<Vec<CloudfrontDistributionOrderedCacheBehaviorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<Vec<CloudfrontDistributionOriginEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_group: Option<Vec<CloudfrontDistributionOriginGroupEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restrictions: Option<Vec<CloudfrontDistributionRestrictionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    viewer_certificate: Option<Vec<CloudfrontDistributionViewerCertificateEl>>,
    dynamic: CloudfrontDistributionDynamic,
}
struct CloudfrontDistribution_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudfrontDistributionData>,
}
#[derive(Clone)]
pub struct CloudfrontDistribution(Rc<CloudfrontDistribution_>);
impl CloudfrontDistribution {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
    #[doc = "Set the field `aliases`.\n"]
    pub fn set_aliases(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().aliases = Some(v.into());
        self
    }
    #[doc = "Set the field `anycast_ip_list_id`.\n"]
    pub fn set_anycast_ip_list_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().anycast_ip_list_id = Some(v.into());
        self
    }
    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().comment = Some(v.into());
        self
    }
    #[doc = "Set the field `continuous_deployment_policy_id`.\n"]
    pub fn set_continuous_deployment_policy_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().continuous_deployment_policy_id = Some(v.into());
        self
    }
    #[doc = "Set the field `default_root_object`.\n"]
    pub fn set_default_root_object(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_root_object = Some(v.into());
        self
    }
    #[doc = "Set the field `http_version`.\n"]
    pub fn set_http_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().http_version = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `is_ipv6_enabled`.\n"]
    pub fn set_is_ipv6_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_ipv6_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `price_class`.\n"]
    pub fn set_price_class(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().price_class = Some(v.into());
        self
    }
    #[doc = "Set the field `retain_on_delete`.\n"]
    pub fn set_retain_on_delete(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().retain_on_delete = Some(v.into());
        self
    }
    #[doc = "Set the field `staging`.\n"]
    pub fn set_staging(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().staging = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }
    #[doc = "Set the field `wait_for_deployment`.\n"]
    pub fn set_wait_for_deployment(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_deployment = Some(v.into());
        self
    }
    #[doc = "Set the field `web_acl_id`.\n"]
    pub fn set_web_acl_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().web_acl_id = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_error_response`.\n"]
    pub fn set_custom_error_response(
        self,
        v: impl Into<BlockAssignable<CloudfrontDistributionCustomErrorResponseEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_error_response = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_error_response = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `default_cache_behavior`.\n"]
    pub fn set_default_cache_behavior(
        self,
        v: impl Into<BlockAssignable<CloudfrontDistributionDefaultCacheBehaviorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_cache_behavior = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_cache_behavior = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        self,
        v: impl Into<BlockAssignable<CloudfrontDistributionLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ordered_cache_behavior`.\n"]
    pub fn set_ordered_cache_behavior(
        self,
        v: impl Into<BlockAssignable<CloudfrontDistributionOrderedCacheBehaviorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ordered_cache_behavior = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ordered_cache_behavior = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `origin`.\n"]
    pub fn set_origin(self, v: impl Into<BlockAssignable<CloudfrontDistributionOriginEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().origin = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.origin = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `origin_group`.\n"]
    pub fn set_origin_group(
        self,
        v: impl Into<BlockAssignable<CloudfrontDistributionOriginGroupEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().origin_group = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.origin_group = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `restrictions`.\n"]
    pub fn set_restrictions(
        self,
        v: impl Into<BlockAssignable<CloudfrontDistributionRestrictionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().restrictions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.restrictions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `viewer_certificate`.\n"]
    pub fn set_viewer_certificate(
        self,
        v: impl Into<BlockAssignable<CloudfrontDistributionViewerCertificateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().viewer_certificate = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.viewer_certificate = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `aliases` after provisioning.\n"]
    pub fn aliases(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.aliases", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `anycast_ip_list_id` after provisioning.\n"]
    pub fn anycast_ip_list_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.anycast_ip_list_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `caller_reference` after provisioning.\n"]
    pub fn caller_reference(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.caller_reference", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.comment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `continuous_deployment_policy_id` after provisioning.\n"]
    pub fn continuous_deployment_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.continuous_deployment_policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_root_object` after provisioning.\n"]
    pub fn default_root_object(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_root_object", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hosted_zone_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `http_version` after provisioning.\n"]
    pub fn http_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `in_progress_validation_batches` after provisioning.\n"]
    pub fn in_progress_validation_batches(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.in_progress_validation_batches", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `is_ipv6_enabled` after provisioning.\n"]
    pub fn is_ipv6_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_ipv6_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_modified_time` after provisioning.\n"]
    pub fn last_modified_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_v1_enabled` after provisioning.\n"]
    pub fn logging_v1_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logging_v1_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `price_class` after provisioning.\n"]
    pub fn price_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.price_class", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retain_on_delete` after provisioning.\n"]
    pub fn retain_on_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retain_on_delete", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `staging` after provisioning.\n"]
    pub fn staging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.staging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trusted_key_groups` after provisioning.\n"]
    pub fn trusted_key_groups(&self) -> ListRef<CloudfrontDistributionTrustedKeyGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trusted_key_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trusted_signers` after provisioning.\n"]
    pub fn trusted_signers(&self) -> ListRef<CloudfrontDistributionTrustedSignersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trusted_signers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `wait_for_deployment` after provisioning.\n"]
    pub fn wait_for_deployment(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_deployment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `web_acl_id` after provisioning.\n"]
    pub fn web_acl_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.web_acl_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_cache_behavior` after provisioning.\n"]
    pub fn default_cache_behavior(
        &self,
    ) -> ListRef<CloudfrontDistributionDefaultCacheBehaviorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_cache_behavior", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> ListRef<CloudfrontDistributionLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ordered_cache_behavior` after provisioning.\n"]
    pub fn ordered_cache_behavior(
        &self,
    ) -> ListRef<CloudfrontDistributionOrderedCacheBehaviorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ordered_cache_behavior", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restrictions` after provisioning.\n"]
    pub fn restrictions(&self) -> ListRef<CloudfrontDistributionRestrictionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restrictions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `viewer_certificate` after provisioning.\n"]
    pub fn viewer_certificate(&self) -> ListRef<CloudfrontDistributionViewerCertificateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.viewer_certificate", self.extract_ref()),
        )
    }
}
impl Referable for CloudfrontDistribution {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudfrontDistribution {}
impl ToListMappable for CloudfrontDistribution {
    type O = ListRef<CloudfrontDistributionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudfrontDistribution_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudfront_distribution".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudfrontDistribution {
    pub tf_id: String,
    #[doc = ""]
    pub enabled: PrimField<bool>,
}
impl BuildCloudfrontDistribution {
    pub fn build(self, stack: &mut Stack) -> CloudfrontDistribution {
        let out = CloudfrontDistribution(Rc::new(CloudfrontDistribution_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudfrontDistributionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                aliases: core::default::Default::default(),
                anycast_ip_list_id: core::default::Default::default(),
                comment: core::default::Default::default(),
                continuous_deployment_policy_id: core::default::Default::default(),
                default_root_object: core::default::Default::default(),
                enabled: self.enabled,
                http_version: core::default::Default::default(),
                id: core::default::Default::default(),
                is_ipv6_enabled: core::default::Default::default(),
                price_class: core::default::Default::default(),
                retain_on_delete: core::default::Default::default(),
                staging: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                wait_for_deployment: core::default::Default::default(),
                web_acl_id: core::default::Default::default(),
                custom_error_response: core::default::Default::default(),
                default_cache_behavior: core::default::Default::default(),
                logging_config: core::default::Default::default(),
                ordered_cache_behavior: core::default::Default::default(),
                origin: core::default::Default::default(),
                origin_group: core::default::Default::default(),
                restrictions: core::default::Default::default(),
                viewer_certificate: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudfrontDistributionRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudfrontDistributionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aliases` after provisioning.\n"]
    pub fn aliases(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.aliases", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `anycast_ip_list_id` after provisioning.\n"]
    pub fn anycast_ip_list_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.anycast_ip_list_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `caller_reference` after provisioning.\n"]
    pub fn caller_reference(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.caller_reference", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.comment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `continuous_deployment_policy_id` after provisioning.\n"]
    pub fn continuous_deployment_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.continuous_deployment_policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_root_object` after provisioning.\n"]
    pub fn default_root_object(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_root_object", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hosted_zone_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `http_version` after provisioning.\n"]
    pub fn http_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `in_progress_validation_batches` after provisioning.\n"]
    pub fn in_progress_validation_batches(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.in_progress_validation_batches", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `is_ipv6_enabled` after provisioning.\n"]
    pub fn is_ipv6_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_ipv6_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_modified_time` after provisioning.\n"]
    pub fn last_modified_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_v1_enabled` after provisioning.\n"]
    pub fn logging_v1_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logging_v1_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `price_class` after provisioning.\n"]
    pub fn price_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.price_class", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retain_on_delete` after provisioning.\n"]
    pub fn retain_on_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retain_on_delete", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `staging` after provisioning.\n"]
    pub fn staging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.staging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trusted_key_groups` after provisioning.\n"]
    pub fn trusted_key_groups(&self) -> ListRef<CloudfrontDistributionTrustedKeyGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trusted_key_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trusted_signers` after provisioning.\n"]
    pub fn trusted_signers(&self) -> ListRef<CloudfrontDistributionTrustedSignersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trusted_signers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `wait_for_deployment` after provisioning.\n"]
    pub fn wait_for_deployment(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_deployment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `web_acl_id` after provisioning.\n"]
    pub fn web_acl_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.web_acl_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_cache_behavior` after provisioning.\n"]
    pub fn default_cache_behavior(
        &self,
    ) -> ListRef<CloudfrontDistributionDefaultCacheBehaviorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_cache_behavior", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> ListRef<CloudfrontDistributionLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ordered_cache_behavior` after provisioning.\n"]
    pub fn ordered_cache_behavior(
        &self,
    ) -> ListRef<CloudfrontDistributionOrderedCacheBehaviorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ordered_cache_behavior", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restrictions` after provisioning.\n"]
    pub fn restrictions(&self) -> ListRef<CloudfrontDistributionRestrictionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restrictions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `viewer_certificate` after provisioning.\n"]
    pub fn viewer_certificate(&self) -> ListRef<CloudfrontDistributionViewerCertificateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.viewer_certificate", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudfrontDistributionTrustedKeyGroupsElItemsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key_group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_pair_ids: Option<SetField<PrimField<String>>>,
}
impl CloudfrontDistributionTrustedKeyGroupsElItemsEl {
    #[doc = "Set the field `key_group_id`.\n"]
    pub fn set_key_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_group_id = Some(v.into());
        self
    }
    #[doc = "Set the field `key_pair_ids`.\n"]
    pub fn set_key_pair_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.key_pair_ids = Some(v.into());
        self
    }
}
impl ToListMappable for CloudfrontDistributionTrustedKeyGroupsElItemsEl {
    type O = BlockAssignable<CloudfrontDistributionTrustedKeyGroupsElItemsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionTrustedKeyGroupsElItemsEl {}
impl BuildCloudfrontDistributionTrustedKeyGroupsElItemsEl {
    pub fn build(self) -> CloudfrontDistributionTrustedKeyGroupsElItemsEl {
        CloudfrontDistributionTrustedKeyGroupsElItemsEl {
            key_group_id: core::default::Default::default(),
            key_pair_ids: core::default::Default::default(),
        }
    }
}
pub struct CloudfrontDistributionTrustedKeyGroupsElItemsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionTrustedKeyGroupsElItemsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontDistributionTrustedKeyGroupsElItemsElRef {
        CloudfrontDistributionTrustedKeyGroupsElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionTrustedKeyGroupsElItemsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key_group_id` after provisioning.\n"]
    pub fn key_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_group_id", self.base))
    }
    #[doc = "Get a reference to the value of field `key_pair_ids` after provisioning.\n"]
    pub fn key_pair_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.key_pair_ids", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudfrontDistributionTrustedKeyGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<ListField<CloudfrontDistributionTrustedKeyGroupsElItemsEl>>,
}
impl CloudfrontDistributionTrustedKeyGroupsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl Into<ListField<CloudfrontDistributionTrustedKeyGroupsElItemsEl>>,
    ) -> Self {
        self.items = Some(v.into());
        self
    }
}
impl ToListMappable for CloudfrontDistributionTrustedKeyGroupsEl {
    type O = BlockAssignable<CloudfrontDistributionTrustedKeyGroupsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionTrustedKeyGroupsEl {}
impl BuildCloudfrontDistributionTrustedKeyGroupsEl {
    pub fn build(self) -> CloudfrontDistributionTrustedKeyGroupsEl {
        CloudfrontDistributionTrustedKeyGroupsEl {
            enabled: core::default::Default::default(),
            items: core::default::Default::default(),
        }
    }
}
pub struct CloudfrontDistributionTrustedKeyGroupsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionTrustedKeyGroupsElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontDistributionTrustedKeyGroupsElRef {
        CloudfrontDistributionTrustedKeyGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionTrustedKeyGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> ListRef<CloudfrontDistributionTrustedKeyGroupsElItemsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudfrontDistributionTrustedSignersElItemsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_account_number: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_pair_ids: Option<SetField<PrimField<String>>>,
}
impl CloudfrontDistributionTrustedSignersElItemsEl {
    #[doc = "Set the field `aws_account_number`.\n"]
    pub fn set_aws_account_number(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.aws_account_number = Some(v.into());
        self
    }
    #[doc = "Set the field `key_pair_ids`.\n"]
    pub fn set_key_pair_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.key_pair_ids = Some(v.into());
        self
    }
}
impl ToListMappable for CloudfrontDistributionTrustedSignersElItemsEl {
    type O = BlockAssignable<CloudfrontDistributionTrustedSignersElItemsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionTrustedSignersElItemsEl {}
impl BuildCloudfrontDistributionTrustedSignersElItemsEl {
    pub fn build(self) -> CloudfrontDistributionTrustedSignersElItemsEl {
        CloudfrontDistributionTrustedSignersElItemsEl {
            aws_account_number: core::default::Default::default(),
            key_pair_ids: core::default::Default::default(),
        }
    }
}
pub struct CloudfrontDistributionTrustedSignersElItemsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionTrustedSignersElItemsElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontDistributionTrustedSignersElItemsElRef {
        CloudfrontDistributionTrustedSignersElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionTrustedSignersElItemsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aws_account_number` after provisioning.\n"]
    pub fn aws_account_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_account_number", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key_pair_ids` after provisioning.\n"]
    pub fn key_pair_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.key_pair_ids", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudfrontDistributionTrustedSignersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<ListField<CloudfrontDistributionTrustedSignersElItemsEl>>,
}
impl CloudfrontDistributionTrustedSignersEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl Into<ListField<CloudfrontDistributionTrustedSignersElItemsEl>>,
    ) -> Self {
        self.items = Some(v.into());
        self
    }
}
impl ToListMappable for CloudfrontDistributionTrustedSignersEl {
    type O = BlockAssignable<CloudfrontDistributionTrustedSignersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionTrustedSignersEl {}
impl BuildCloudfrontDistributionTrustedSignersEl {
    pub fn build(self) -> CloudfrontDistributionTrustedSignersEl {
        CloudfrontDistributionTrustedSignersEl {
            enabled: core::default::Default::default(),
            items: core::default::Default::default(),
        }
    }
}
pub struct CloudfrontDistributionTrustedSignersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionTrustedSignersElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontDistributionTrustedSignersElRef {
        CloudfrontDistributionTrustedSignersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionTrustedSignersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> ListRef<CloudfrontDistributionTrustedSignersElItemsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudfrontDistributionCustomErrorResponseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    error_caching_min_ttl: Option<PrimField<f64>>,
    error_code: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_page_path: Option<PrimField<String>>,
}
impl CloudfrontDistributionCustomErrorResponseEl {
    #[doc = "Set the field `error_caching_min_ttl`.\n"]
    pub fn set_error_caching_min_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.error_caching_min_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `response_code`.\n"]
    pub fn set_response_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.response_code = Some(v.into());
        self
    }
    #[doc = "Set the field `response_page_path`.\n"]
    pub fn set_response_page_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response_page_path = Some(v.into());
        self
    }
}
impl ToListMappable for CloudfrontDistributionCustomErrorResponseEl {
    type O = BlockAssignable<CloudfrontDistributionCustomErrorResponseEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionCustomErrorResponseEl {
    #[doc = ""]
    pub error_code: PrimField<f64>,
}
impl BuildCloudfrontDistributionCustomErrorResponseEl {
    pub fn build(self) -> CloudfrontDistributionCustomErrorResponseEl {
        CloudfrontDistributionCustomErrorResponseEl {
            error_caching_min_ttl: core::default::Default::default(),
            error_code: self.error_code,
            response_code: core::default::Default::default(),
            response_page_path: core::default::Default::default(),
        }
    }
}
pub struct CloudfrontDistributionCustomErrorResponseElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionCustomErrorResponseElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontDistributionCustomErrorResponseElRef {
        CloudfrontDistributionCustomErrorResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionCustomErrorResponseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `error_caching_min_ttl` after provisioning.\n"]
    pub fn error_caching_min_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.error_caching_min_ttl", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `error_code` after provisioning.\n"]
    pub fn error_code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.error_code", self.base))
    }
    #[doc = "Get a reference to the value of field `response_code` after provisioning.\n"]
    pub fn response_code(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `response_page_path` after provisioning.\n"]
    pub fn response_page_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_page_path", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesEl {
    forward: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    whitelisted_names: Option<SetField<PrimField<String>>>,
}
impl CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesEl {
    #[doc = "Set the field `whitelisted_names`.\n"]
    pub fn set_whitelisted_names(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.whitelisted_names = Some(v.into());
        self
    }
}
impl ToListMappable for CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesEl {
    type O =
        BlockAssignable<CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesEl {
    #[doc = ""]
    pub forward: PrimField<String>,
}
impl BuildCloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesEl {
    pub fn build(self) -> CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesEl {
        CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesEl {
            forward: self.forward,
            whitelisted_names: core::default::Default::default(),
        }
    }
}
pub struct CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesElRef {
        CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `forward` after provisioning.\n"]
    pub fn forward(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.forward", self.base))
    }
    #[doc = "Get a reference to the value of field `whitelisted_names` after provisioning.\n"]
    pub fn whitelisted_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.whitelisted_names", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElDynamic {
    cookies: Option<
        DynamicBlock<CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesEl>,
    >,
}
#[derive(Serialize)]
pub struct CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<SetField<PrimField<String>>>,
    query_string: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_string_cache_keys: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cookies: Option<Vec<CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesEl>>,
    dynamic: CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElDynamic,
}
impl CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesEl {
    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.headers = Some(v.into());
        self
    }
    #[doc = "Set the field `query_string_cache_keys`.\n"]
    pub fn set_query_string_cache_keys(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.query_string_cache_keys = Some(v.into());
        self
    }
    #[doc = "Set the field `cookies`.\n"]
    pub fn set_cookies(
        mut self,
        v: impl Into<
            BlockAssignable<CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cookies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cookies = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesEl {
    type O = BlockAssignable<CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionDefaultCacheBehaviorElForwardedValuesEl {
    #[doc = ""]
    pub query_string: PrimField<bool>,
}
impl BuildCloudfrontDistributionDefaultCacheBehaviorElForwardedValuesEl {
    pub fn build(self) -> CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesEl {
        CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesEl {
            headers: core::default::Default::default(),
            query_string: self.query_string,
            query_string_cache_keys: core::default::Default::default(),
            cookies: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElRef {
        CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }
    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.query_string", self.base))
    }
    #[doc = "Get a reference to the value of field `query_string_cache_keys` after provisioning.\n"]
    pub fn query_string_cache_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_string_cache_keys", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cookies` after provisioning.\n"]
    pub fn cookies(
        &self,
    ) -> ListRef<CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cookies", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationEl {
    event_type: PrimField<String>,
    function_arn: PrimField<String>,
}
impl CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationEl {}
impl ToListMappable for CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationEl {
    type O = BlockAssignable<CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationEl {
    #[doc = ""]
    pub event_type: PrimField<String>,
    #[doc = ""]
    pub function_arn: PrimField<String>,
}
impl BuildCloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationEl {
    pub fn build(self) -> CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationEl {
        CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationEl {
            event_type: self.event_type,
            function_arn: self.function_arn,
        }
    }
}
pub struct CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationElRef {
        CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `event_type` after provisioning.\n"]
    pub fn event_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_type", self.base))
    }
    #[doc = "Get a reference to the value of field `function_arn` after provisioning.\n"]
    pub fn function_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudfrontDistributionDefaultCacheBehaviorElGrpcConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl CloudfrontDistributionDefaultCacheBehaviorElGrpcConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for CloudfrontDistributionDefaultCacheBehaviorElGrpcConfigEl {
    type O = BlockAssignable<CloudfrontDistributionDefaultCacheBehaviorElGrpcConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionDefaultCacheBehaviorElGrpcConfigEl {}
impl BuildCloudfrontDistributionDefaultCacheBehaviorElGrpcConfigEl {
    pub fn build(self) -> CloudfrontDistributionDefaultCacheBehaviorElGrpcConfigEl {
        CloudfrontDistributionDefaultCacheBehaviorElGrpcConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct CloudfrontDistributionDefaultCacheBehaviorElGrpcConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionDefaultCacheBehaviorElGrpcConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontDistributionDefaultCacheBehaviorElGrpcConfigElRef {
        CloudfrontDistributionDefaultCacheBehaviorElGrpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionDefaultCacheBehaviorElGrpcConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationEl {
    event_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_body: Option<PrimField<bool>>,
    lambda_arn: PrimField<String>,
}
impl CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationEl {
    #[doc = "Set the field `include_body`.\n"]
    pub fn set_include_body(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_body = Some(v.into());
        self
    }
}
impl ToListMappable for CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationEl {
    type O =
        BlockAssignable<CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationEl {
    #[doc = ""]
    pub event_type: PrimField<String>,
    #[doc = ""]
    pub lambda_arn: PrimField<String>,
}
impl BuildCloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationEl {
    pub fn build(self) -> CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationEl {
        CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationEl {
            event_type: self.event_type,
            include_body: core::default::Default::default(),
            lambda_arn: self.lambda_arn,
        }
    }
}
pub struct CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationElRef {
        CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `event_type` after provisioning.\n"]
    pub fn event_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_type", self.base))
    }
    #[doc = "Get a reference to the value of field `include_body` after provisioning.\n"]
    pub fn include_body(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_body", self.base))
    }
    #[doc = "Get a reference to the value of field `lambda_arn` after provisioning.\n"]
    pub fn lambda_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lambda_arn", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudfrontDistributionDefaultCacheBehaviorElDynamic {
    forwarded_values:
        Option<DynamicBlock<CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesEl>>,
    function_association:
        Option<DynamicBlock<CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationEl>>,
    grpc_config: Option<DynamicBlock<CloudfrontDistributionDefaultCacheBehaviorElGrpcConfigEl>>,
    lambda_function_association: Option<
        DynamicBlock<CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationEl>,
    >,
}
#[derive(Serialize)]
pub struct CloudfrontDistributionDefaultCacheBehaviorEl {
    allowed_methods: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_policy_id: Option<PrimField<String>>,
    cached_methods: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compress: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_level_encryption_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_request_policy_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    realtime_log_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_headers_policy_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    smooth_streaming: Option<PrimField<bool>>,
    target_origin_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trusted_key_groups: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trusted_signers: Option<ListField<PrimField<String>>>,
    viewer_protocol_policy: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forwarded_values: Option<Vec<CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    function_association:
        Option<Vec<CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc_config: Option<Vec<CloudfrontDistributionDefaultCacheBehaviorElGrpcConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_function_association:
        Option<Vec<CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationEl>>,
    dynamic: CloudfrontDistributionDefaultCacheBehaviorElDynamic,
}
impl CloudfrontDistributionDefaultCacheBehaviorEl {
    #[doc = "Set the field `cache_policy_id`.\n"]
    pub fn set_cache_policy_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cache_policy_id = Some(v.into());
        self
    }
    #[doc = "Set the field `compress`.\n"]
    pub fn set_compress(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.compress = Some(v.into());
        self
    }
    #[doc = "Set the field `default_ttl`.\n"]
    pub fn set_default_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `field_level_encryption_id`.\n"]
    pub fn set_field_level_encryption_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field_level_encryption_id = Some(v.into());
        self
    }
    #[doc = "Set the field `max_ttl`.\n"]
    pub fn set_max_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `min_ttl`.\n"]
    pub fn set_min_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `origin_request_policy_id`.\n"]
    pub fn set_origin_request_policy_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.origin_request_policy_id = Some(v.into());
        self
    }
    #[doc = "Set the field `realtime_log_config_arn`.\n"]
    pub fn set_realtime_log_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.realtime_log_config_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `response_headers_policy_id`.\n"]
    pub fn set_response_headers_policy_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response_headers_policy_id = Some(v.into());
        self
    }
    #[doc = "Set the field `smooth_streaming`.\n"]
    pub fn set_smooth_streaming(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.smooth_streaming = Some(v.into());
        self
    }
    #[doc = "Set the field `trusted_key_groups`.\n"]
    pub fn set_trusted_key_groups(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.trusted_key_groups = Some(v.into());
        self
    }
    #[doc = "Set the field `trusted_signers`.\n"]
    pub fn set_trusted_signers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.trusted_signers = Some(v.into());
        self
    }
    #[doc = "Set the field `forwarded_values`.\n"]
    pub fn set_forwarded_values(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.forwarded_values = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.forwarded_values = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `function_association`.\n"]
    pub fn set_function_association(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.function_association = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.function_association = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `grpc_config`.\n"]
    pub fn set_grpc_config(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontDistributionDefaultCacheBehaviorElGrpcConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.grpc_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `lambda_function_association`.\n"]
    pub fn set_lambda_function_association(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lambda_function_association = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_function_association = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudfrontDistributionDefaultCacheBehaviorEl {
    type O = BlockAssignable<CloudfrontDistributionDefaultCacheBehaviorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionDefaultCacheBehaviorEl {
    #[doc = ""]
    pub allowed_methods: SetField<PrimField<String>>,
    #[doc = ""]
    pub cached_methods: SetField<PrimField<String>>,
    #[doc = ""]
    pub target_origin_id: PrimField<String>,
    #[doc = ""]
    pub viewer_protocol_policy: PrimField<String>,
}
impl BuildCloudfrontDistributionDefaultCacheBehaviorEl {
    pub fn build(self) -> CloudfrontDistributionDefaultCacheBehaviorEl {
        CloudfrontDistributionDefaultCacheBehaviorEl {
            allowed_methods: self.allowed_methods,
            cache_policy_id: core::default::Default::default(),
            cached_methods: self.cached_methods,
            compress: core::default::Default::default(),
            default_ttl: core::default::Default::default(),
            field_level_encryption_id: core::default::Default::default(),
            max_ttl: core::default::Default::default(),
            min_ttl: core::default::Default::default(),
            origin_request_policy_id: core::default::Default::default(),
            realtime_log_config_arn: core::default::Default::default(),
            response_headers_policy_id: core::default::Default::default(),
            smooth_streaming: core::default::Default::default(),
            target_origin_id: self.target_origin_id,
            trusted_key_groups: core::default::Default::default(),
            trusted_signers: core::default::Default::default(),
            viewer_protocol_policy: self.viewer_protocol_policy,
            forwarded_values: core::default::Default::default(),
            function_association: core::default::Default::default(),
            grpc_config: core::default::Default::default(),
            lambda_function_association: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudfrontDistributionDefaultCacheBehaviorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionDefaultCacheBehaviorElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontDistributionDefaultCacheBehaviorElRef {
        CloudfrontDistributionDefaultCacheBehaviorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionDefaultCacheBehaviorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_methods` after provisioning.\n"]
    pub fn allowed_methods(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.allowed_methods", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cache_policy_id` after provisioning.\n"]
    pub fn cache_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cache_policy_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cached_methods` after provisioning.\n"]
    pub fn cached_methods(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.cached_methods", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `compress` after provisioning.\n"]
    pub fn compress(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.compress", self.base))
    }
    #[doc = "Get a reference to the value of field `default_ttl` after provisioning.\n"]
    pub fn default_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `field_level_encryption_id` after provisioning.\n"]
    pub fn field_level_encryption_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.field_level_encryption_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_ttl` after provisioning.\n"]
    pub fn max_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `min_ttl` after provisioning.\n"]
    pub fn min_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `origin_request_policy_id` after provisioning.\n"]
    pub fn origin_request_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.origin_request_policy_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `realtime_log_config_arn` after provisioning.\n"]
    pub fn realtime_log_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.realtime_log_config_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `response_headers_policy_id` after provisioning.\n"]
    pub fn response_headers_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_headers_policy_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `smooth_streaming` after provisioning.\n"]
    pub fn smooth_streaming(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.smooth_streaming", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_origin_id` after provisioning.\n"]
    pub fn target_origin_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_origin_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `trusted_key_groups` after provisioning.\n"]
    pub fn trusted_key_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trusted_key_groups", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `trusted_signers` after provisioning.\n"]
    pub fn trusted_signers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trusted_signers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `viewer_protocol_policy` after provisioning.\n"]
    pub fn viewer_protocol_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.viewer_protocol_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `forwarded_values` after provisioning.\n"]
    pub fn forwarded_values(
        &self,
    ) -> ListRef<CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.forwarded_values", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `grpc_config` after provisioning.\n"]
    pub fn grpc_config(
        &self,
    ) -> ListRef<CloudfrontDistributionDefaultCacheBehaviorElGrpcConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grpc_config", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudfrontDistributionLoggingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_cookies: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
}
impl CloudfrontDistributionLoggingConfigEl {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `include_cookies`.\n"]
    pub fn set_include_cookies(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_cookies = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
}
impl ToListMappable for CloudfrontDistributionLoggingConfigEl {
    type O = BlockAssignable<CloudfrontDistributionLoggingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionLoggingConfigEl {}
impl BuildCloudfrontDistributionLoggingConfigEl {
    pub fn build(self) -> CloudfrontDistributionLoggingConfigEl {
        CloudfrontDistributionLoggingConfigEl {
            bucket: core::default::Default::default(),
            include_cookies: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}
pub struct CloudfrontDistributionLoggingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionLoggingConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontDistributionLoggingConfigElRef {
        CloudfrontDistributionLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionLoggingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `include_cookies` after provisioning.\n"]
    pub fn include_cookies(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_cookies", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesEl {
    forward: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    whitelisted_names: Option<SetField<PrimField<String>>>,
}
impl CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesEl {
    #[doc = "Set the field `whitelisted_names`.\n"]
    pub fn set_whitelisted_names(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.whitelisted_names = Some(v.into());
        self
    }
}
impl ToListMappable for CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesEl {
    type O =
        BlockAssignable<CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesEl {
    #[doc = ""]
    pub forward: PrimField<String>,
}
impl BuildCloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesEl {
    pub fn build(self) -> CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesEl {
        CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesEl {
            forward: self.forward,
            whitelisted_names: core::default::Default::default(),
        }
    }
}
pub struct CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesElRef {
        CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `forward` after provisioning.\n"]
    pub fn forward(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.forward", self.base))
    }
    #[doc = "Get a reference to the value of field `whitelisted_names` after provisioning.\n"]
    pub fn whitelisted_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.whitelisted_names", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElDynamic {
    cookies: Option<
        DynamicBlock<CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesEl>,
    >,
}
#[derive(Serialize)]
pub struct CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<SetField<PrimField<String>>>,
    query_string: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_string_cache_keys: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cookies: Option<Vec<CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesEl>>,
    dynamic: CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElDynamic,
}
impl CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesEl {
    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.headers = Some(v.into());
        self
    }
    #[doc = "Set the field `query_string_cache_keys`.\n"]
    pub fn set_query_string_cache_keys(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.query_string_cache_keys = Some(v.into());
        self
    }
    #[doc = "Set the field `cookies`.\n"]
    pub fn set_cookies(
        mut self,
        v: impl Into<
            BlockAssignable<CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cookies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cookies = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesEl {
    type O = BlockAssignable<CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionOrderedCacheBehaviorElForwardedValuesEl {
    #[doc = ""]
    pub query_string: PrimField<bool>,
}
impl BuildCloudfrontDistributionOrderedCacheBehaviorElForwardedValuesEl {
    pub fn build(self) -> CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesEl {
        CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesEl {
            headers: core::default::Default::default(),
            query_string: self.query_string,
            query_string_cache_keys: core::default::Default::default(),
            cookies: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElRef {
        CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }
    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.query_string", self.base))
    }
    #[doc = "Get a reference to the value of field `query_string_cache_keys` after provisioning.\n"]
    pub fn query_string_cache_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_string_cache_keys", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cookies` after provisioning.\n"]
    pub fn cookies(
        &self,
    ) -> ListRef<CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cookies", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationEl {
    event_type: PrimField<String>,
    function_arn: PrimField<String>,
}
impl CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationEl {}
impl ToListMappable for CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationEl {
    type O = BlockAssignable<CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationEl {
    #[doc = ""]
    pub event_type: PrimField<String>,
    #[doc = ""]
    pub function_arn: PrimField<String>,
}
impl BuildCloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationEl {
    pub fn build(self) -> CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationEl {
        CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationEl {
            event_type: self.event_type,
            function_arn: self.function_arn,
        }
    }
}
pub struct CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationElRef {
        CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `event_type` after provisioning.\n"]
    pub fn event_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_type", self.base))
    }
    #[doc = "Get a reference to the value of field `function_arn` after provisioning.\n"]
    pub fn function_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudfrontDistributionOrderedCacheBehaviorElGrpcConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl CloudfrontDistributionOrderedCacheBehaviorElGrpcConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for CloudfrontDistributionOrderedCacheBehaviorElGrpcConfigEl {
    type O = BlockAssignable<CloudfrontDistributionOrderedCacheBehaviorElGrpcConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionOrderedCacheBehaviorElGrpcConfigEl {}
impl BuildCloudfrontDistributionOrderedCacheBehaviorElGrpcConfigEl {
    pub fn build(self) -> CloudfrontDistributionOrderedCacheBehaviorElGrpcConfigEl {
        CloudfrontDistributionOrderedCacheBehaviorElGrpcConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct CloudfrontDistributionOrderedCacheBehaviorElGrpcConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionOrderedCacheBehaviorElGrpcConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontDistributionOrderedCacheBehaviorElGrpcConfigElRef {
        CloudfrontDistributionOrderedCacheBehaviorElGrpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionOrderedCacheBehaviorElGrpcConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationEl {
    event_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_body: Option<PrimField<bool>>,
    lambda_arn: PrimField<String>,
}
impl CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationEl {
    #[doc = "Set the field `include_body`.\n"]
    pub fn set_include_body(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_body = Some(v.into());
        self
    }
}
impl ToListMappable for CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationEl {
    type O =
        BlockAssignable<CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationEl {
    #[doc = ""]
    pub event_type: PrimField<String>,
    #[doc = ""]
    pub lambda_arn: PrimField<String>,
}
impl BuildCloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationEl {
    pub fn build(self) -> CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationEl {
        CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationEl {
            event_type: self.event_type,
            include_body: core::default::Default::default(),
            lambda_arn: self.lambda_arn,
        }
    }
}
pub struct CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationElRef {
        CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `event_type` after provisioning.\n"]
    pub fn event_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_type", self.base))
    }
    #[doc = "Get a reference to the value of field `include_body` after provisioning.\n"]
    pub fn include_body(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_body", self.base))
    }
    #[doc = "Get a reference to the value of field `lambda_arn` after provisioning.\n"]
    pub fn lambda_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lambda_arn", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudfrontDistributionOrderedCacheBehaviorElDynamic {
    forwarded_values:
        Option<DynamicBlock<CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesEl>>,
    function_association:
        Option<DynamicBlock<CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationEl>>,
    grpc_config: Option<DynamicBlock<CloudfrontDistributionOrderedCacheBehaviorElGrpcConfigEl>>,
    lambda_function_association: Option<
        DynamicBlock<CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationEl>,
    >,
}
#[derive(Serialize)]
pub struct CloudfrontDistributionOrderedCacheBehaviorEl {
    allowed_methods: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_policy_id: Option<PrimField<String>>,
    cached_methods: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compress: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_level_encryption_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_request_policy_id: Option<PrimField<String>>,
    path_pattern: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    realtime_log_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_headers_policy_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    smooth_streaming: Option<PrimField<bool>>,
    target_origin_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trusted_key_groups: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trusted_signers: Option<ListField<PrimField<String>>>,
    viewer_protocol_policy: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forwarded_values: Option<Vec<CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    function_association:
        Option<Vec<CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc_config: Option<Vec<CloudfrontDistributionOrderedCacheBehaviorElGrpcConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_function_association:
        Option<Vec<CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationEl>>,
    dynamic: CloudfrontDistributionOrderedCacheBehaviorElDynamic,
}
impl CloudfrontDistributionOrderedCacheBehaviorEl {
    #[doc = "Set the field `cache_policy_id`.\n"]
    pub fn set_cache_policy_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cache_policy_id = Some(v.into());
        self
    }
    #[doc = "Set the field `compress`.\n"]
    pub fn set_compress(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.compress = Some(v.into());
        self
    }
    #[doc = "Set the field `default_ttl`.\n"]
    pub fn set_default_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `field_level_encryption_id`.\n"]
    pub fn set_field_level_encryption_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field_level_encryption_id = Some(v.into());
        self
    }
    #[doc = "Set the field `max_ttl`.\n"]
    pub fn set_max_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `min_ttl`.\n"]
    pub fn set_min_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `origin_request_policy_id`.\n"]
    pub fn set_origin_request_policy_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.origin_request_policy_id = Some(v.into());
        self
    }
    #[doc = "Set the field `realtime_log_config_arn`.\n"]
    pub fn set_realtime_log_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.realtime_log_config_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `response_headers_policy_id`.\n"]
    pub fn set_response_headers_policy_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response_headers_policy_id = Some(v.into());
        self
    }
    #[doc = "Set the field `smooth_streaming`.\n"]
    pub fn set_smooth_streaming(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.smooth_streaming = Some(v.into());
        self
    }
    #[doc = "Set the field `trusted_key_groups`.\n"]
    pub fn set_trusted_key_groups(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.trusted_key_groups = Some(v.into());
        self
    }
    #[doc = "Set the field `trusted_signers`.\n"]
    pub fn set_trusted_signers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.trusted_signers = Some(v.into());
        self
    }
    #[doc = "Set the field `forwarded_values`.\n"]
    pub fn set_forwarded_values(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.forwarded_values = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.forwarded_values = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `function_association`.\n"]
    pub fn set_function_association(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.function_association = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.function_association = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `grpc_config`.\n"]
    pub fn set_grpc_config(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontDistributionOrderedCacheBehaviorElGrpcConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.grpc_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `lambda_function_association`.\n"]
    pub fn set_lambda_function_association(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lambda_function_association = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_function_association = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudfrontDistributionOrderedCacheBehaviorEl {
    type O = BlockAssignable<CloudfrontDistributionOrderedCacheBehaviorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionOrderedCacheBehaviorEl {
    #[doc = ""]
    pub allowed_methods: SetField<PrimField<String>>,
    #[doc = ""]
    pub cached_methods: SetField<PrimField<String>>,
    #[doc = ""]
    pub path_pattern: PrimField<String>,
    #[doc = ""]
    pub target_origin_id: PrimField<String>,
    #[doc = ""]
    pub viewer_protocol_policy: PrimField<String>,
}
impl BuildCloudfrontDistributionOrderedCacheBehaviorEl {
    pub fn build(self) -> CloudfrontDistributionOrderedCacheBehaviorEl {
        CloudfrontDistributionOrderedCacheBehaviorEl {
            allowed_methods: self.allowed_methods,
            cache_policy_id: core::default::Default::default(),
            cached_methods: self.cached_methods,
            compress: core::default::Default::default(),
            default_ttl: core::default::Default::default(),
            field_level_encryption_id: core::default::Default::default(),
            max_ttl: core::default::Default::default(),
            min_ttl: core::default::Default::default(),
            origin_request_policy_id: core::default::Default::default(),
            path_pattern: self.path_pattern,
            realtime_log_config_arn: core::default::Default::default(),
            response_headers_policy_id: core::default::Default::default(),
            smooth_streaming: core::default::Default::default(),
            target_origin_id: self.target_origin_id,
            trusted_key_groups: core::default::Default::default(),
            trusted_signers: core::default::Default::default(),
            viewer_protocol_policy: self.viewer_protocol_policy,
            forwarded_values: core::default::Default::default(),
            function_association: core::default::Default::default(),
            grpc_config: core::default::Default::default(),
            lambda_function_association: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudfrontDistributionOrderedCacheBehaviorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionOrderedCacheBehaviorElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontDistributionOrderedCacheBehaviorElRef {
        CloudfrontDistributionOrderedCacheBehaviorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionOrderedCacheBehaviorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_methods` after provisioning.\n"]
    pub fn allowed_methods(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.allowed_methods", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cache_policy_id` after provisioning.\n"]
    pub fn cache_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cache_policy_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cached_methods` after provisioning.\n"]
    pub fn cached_methods(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.cached_methods", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `compress` after provisioning.\n"]
    pub fn compress(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.compress", self.base))
    }
    #[doc = "Get a reference to the value of field `default_ttl` after provisioning.\n"]
    pub fn default_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `field_level_encryption_id` after provisioning.\n"]
    pub fn field_level_encryption_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.field_level_encryption_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_ttl` after provisioning.\n"]
    pub fn max_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `min_ttl` after provisioning.\n"]
    pub fn min_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `origin_request_policy_id` after provisioning.\n"]
    pub fn origin_request_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.origin_request_policy_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `path_pattern` after provisioning.\n"]
    pub fn path_pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_pattern", self.base))
    }
    #[doc = "Get a reference to the value of field `realtime_log_config_arn` after provisioning.\n"]
    pub fn realtime_log_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.realtime_log_config_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `response_headers_policy_id` after provisioning.\n"]
    pub fn response_headers_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_headers_policy_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `smooth_streaming` after provisioning.\n"]
    pub fn smooth_streaming(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.smooth_streaming", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_origin_id` after provisioning.\n"]
    pub fn target_origin_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_origin_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `trusted_key_groups` after provisioning.\n"]
    pub fn trusted_key_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trusted_key_groups", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `trusted_signers` after provisioning.\n"]
    pub fn trusted_signers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trusted_signers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `viewer_protocol_policy` after provisioning.\n"]
    pub fn viewer_protocol_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.viewer_protocol_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `forwarded_values` after provisioning.\n"]
    pub fn forwarded_values(
        &self,
    ) -> ListRef<CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.forwarded_values", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `grpc_config` after provisioning.\n"]
    pub fn grpc_config(
        &self,
    ) -> ListRef<CloudfrontDistributionOrderedCacheBehaviorElGrpcConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grpc_config", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudfrontDistributionOriginElCustomHeaderEl {
    name: PrimField<String>,
    value: PrimField<String>,
}
impl CloudfrontDistributionOriginElCustomHeaderEl {}
impl ToListMappable for CloudfrontDistributionOriginElCustomHeaderEl {
    type O = BlockAssignable<CloudfrontDistributionOriginElCustomHeaderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionOriginElCustomHeaderEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildCloudfrontDistributionOriginElCustomHeaderEl {
    pub fn build(self) -> CloudfrontDistributionOriginElCustomHeaderEl {
        CloudfrontDistributionOriginElCustomHeaderEl {
            name: self.name,
            value: self.value,
        }
    }
}
pub struct CloudfrontDistributionOriginElCustomHeaderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionOriginElCustomHeaderElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontDistributionOriginElCustomHeaderElRef {
        CloudfrontDistributionOriginElCustomHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionOriginElCustomHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudfrontDistributionOriginElCustomOriginConfigEl {
    http_port: PrimField<f64>,
    https_port: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_keepalive_timeout: Option<PrimField<f64>>,
    origin_protocol_policy: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_read_timeout: Option<PrimField<f64>>,
    origin_ssl_protocols: SetField<PrimField<String>>,
}
impl CloudfrontDistributionOriginElCustomOriginConfigEl {
    #[doc = "Set the field `ip_address_type`.\n"]
    pub fn set_ip_address_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address_type = Some(v.into());
        self
    }
    #[doc = "Set the field `origin_keepalive_timeout`.\n"]
    pub fn set_origin_keepalive_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.origin_keepalive_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `origin_read_timeout`.\n"]
    pub fn set_origin_read_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.origin_read_timeout = Some(v.into());
        self
    }
}
impl ToListMappable for CloudfrontDistributionOriginElCustomOriginConfigEl {
    type O = BlockAssignable<CloudfrontDistributionOriginElCustomOriginConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionOriginElCustomOriginConfigEl {
    #[doc = ""]
    pub http_port: PrimField<f64>,
    #[doc = ""]
    pub https_port: PrimField<f64>,
    #[doc = ""]
    pub origin_protocol_policy: PrimField<String>,
    #[doc = ""]
    pub origin_ssl_protocols: SetField<PrimField<String>>,
}
impl BuildCloudfrontDistributionOriginElCustomOriginConfigEl {
    pub fn build(self) -> CloudfrontDistributionOriginElCustomOriginConfigEl {
        CloudfrontDistributionOriginElCustomOriginConfigEl {
            http_port: self.http_port,
            https_port: self.https_port,
            ip_address_type: core::default::Default::default(),
            origin_keepalive_timeout: core::default::Default::default(),
            origin_protocol_policy: self.origin_protocol_policy,
            origin_read_timeout: core::default::Default::default(),
            origin_ssl_protocols: self.origin_ssl_protocols,
        }
    }
}
pub struct CloudfrontDistributionOriginElCustomOriginConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionOriginElCustomOriginConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontDistributionOriginElCustomOriginConfigElRef {
        CloudfrontDistributionOriginElCustomOriginConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionOriginElCustomOriginConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `http_port` after provisioning.\n"]
    pub fn http_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_port", self.base))
    }
    #[doc = "Get a reference to the value of field `https_port` after provisioning.\n"]
    pub fn https_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.https_port", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `origin_keepalive_timeout` after provisioning.\n"]
    pub fn origin_keepalive_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.origin_keepalive_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `origin_protocol_policy` after provisioning.\n"]
    pub fn origin_protocol_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.origin_protocol_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `origin_read_timeout` after provisioning.\n"]
    pub fn origin_read_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.origin_read_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `origin_ssl_protocols` after provisioning.\n"]
    pub fn origin_ssl_protocols(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.origin_ssl_protocols", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudfrontDistributionOriginElOriginShieldEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_shield_region: Option<PrimField<String>>,
}
impl CloudfrontDistributionOriginElOriginShieldEl {
    #[doc = "Set the field `origin_shield_region`.\n"]
    pub fn set_origin_shield_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.origin_shield_region = Some(v.into());
        self
    }
}
impl ToListMappable for CloudfrontDistributionOriginElOriginShieldEl {
    type O = BlockAssignable<CloudfrontDistributionOriginElOriginShieldEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionOriginElOriginShieldEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}
impl BuildCloudfrontDistributionOriginElOriginShieldEl {
    pub fn build(self) -> CloudfrontDistributionOriginElOriginShieldEl {
        CloudfrontDistributionOriginElOriginShieldEl {
            enabled: self.enabled,
            origin_shield_region: core::default::Default::default(),
        }
    }
}
pub struct CloudfrontDistributionOriginElOriginShieldElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionOriginElOriginShieldElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontDistributionOriginElOriginShieldElRef {
        CloudfrontDistributionOriginElOriginShieldElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionOriginElOriginShieldElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `origin_shield_region` after provisioning.\n"]
    pub fn origin_shield_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.origin_shield_region", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudfrontDistributionOriginElS3OriginConfigEl {
    origin_access_identity: PrimField<String>,
}
impl CloudfrontDistributionOriginElS3OriginConfigEl {}
impl ToListMappable for CloudfrontDistributionOriginElS3OriginConfigEl {
    type O = BlockAssignable<CloudfrontDistributionOriginElS3OriginConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionOriginElS3OriginConfigEl {
    #[doc = ""]
    pub origin_access_identity: PrimField<String>,
}
impl BuildCloudfrontDistributionOriginElS3OriginConfigEl {
    pub fn build(self) -> CloudfrontDistributionOriginElS3OriginConfigEl {
        CloudfrontDistributionOriginElS3OriginConfigEl {
            origin_access_identity: self.origin_access_identity,
        }
    }
}
pub struct CloudfrontDistributionOriginElS3OriginConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionOriginElS3OriginConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontDistributionOriginElS3OriginConfigElRef {
        CloudfrontDistributionOriginElS3OriginConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionOriginElS3OriginConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `origin_access_identity` after provisioning.\n"]
    pub fn origin_access_identity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.origin_access_identity", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudfrontDistributionOriginElVpcOriginConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_keepalive_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_read_timeout: Option<PrimField<f64>>,
    vpc_origin_id: PrimField<String>,
}
impl CloudfrontDistributionOriginElVpcOriginConfigEl {
    #[doc = "Set the field `origin_keepalive_timeout`.\n"]
    pub fn set_origin_keepalive_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.origin_keepalive_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `origin_read_timeout`.\n"]
    pub fn set_origin_read_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.origin_read_timeout = Some(v.into());
        self
    }
}
impl ToListMappable for CloudfrontDistributionOriginElVpcOriginConfigEl {
    type O = BlockAssignable<CloudfrontDistributionOriginElVpcOriginConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionOriginElVpcOriginConfigEl {
    #[doc = ""]
    pub vpc_origin_id: PrimField<String>,
}
impl BuildCloudfrontDistributionOriginElVpcOriginConfigEl {
    pub fn build(self) -> CloudfrontDistributionOriginElVpcOriginConfigEl {
        CloudfrontDistributionOriginElVpcOriginConfigEl {
            origin_keepalive_timeout: core::default::Default::default(),
            origin_read_timeout: core::default::Default::default(),
            vpc_origin_id: self.vpc_origin_id,
        }
    }
}
pub struct CloudfrontDistributionOriginElVpcOriginConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionOriginElVpcOriginConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontDistributionOriginElVpcOriginConfigElRef {
        CloudfrontDistributionOriginElVpcOriginConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionOriginElVpcOriginConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `origin_keepalive_timeout` after provisioning.\n"]
    pub fn origin_keepalive_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.origin_keepalive_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `origin_read_timeout` after provisioning.\n"]
    pub fn origin_read_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.origin_read_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_origin_id` after provisioning.\n"]
    pub fn vpc_origin_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_origin_id", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CloudfrontDistributionOriginElDynamic {
    custom_header: Option<DynamicBlock<CloudfrontDistributionOriginElCustomHeaderEl>>,
    custom_origin_config: Option<DynamicBlock<CloudfrontDistributionOriginElCustomOriginConfigEl>>,
    origin_shield: Option<DynamicBlock<CloudfrontDistributionOriginElOriginShieldEl>>,
    s3_origin_config: Option<DynamicBlock<CloudfrontDistributionOriginElS3OriginConfigEl>>,
    vpc_origin_config: Option<DynamicBlock<CloudfrontDistributionOriginElVpcOriginConfigEl>>,
}
#[derive(Serialize)]
pub struct CloudfrontDistributionOriginEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_attempts: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_timeout: Option<PrimField<f64>>,
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_access_control_id: Option<PrimField<String>>,
    origin_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_completion_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_header: Option<Vec<CloudfrontDistributionOriginElCustomHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_origin_config: Option<Vec<CloudfrontDistributionOriginElCustomOriginConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_shield: Option<Vec<CloudfrontDistributionOriginElOriginShieldEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_origin_config: Option<Vec<CloudfrontDistributionOriginElS3OriginConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_origin_config: Option<Vec<CloudfrontDistributionOriginElVpcOriginConfigEl>>,
    dynamic: CloudfrontDistributionOriginElDynamic,
}
impl CloudfrontDistributionOriginEl {
    #[doc = "Set the field `connection_attempts`.\n"]
    pub fn set_connection_attempts(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.connection_attempts = Some(v.into());
        self
    }
    #[doc = "Set the field `connection_timeout`.\n"]
    pub fn set_connection_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.connection_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `origin_access_control_id`.\n"]
    pub fn set_origin_access_control_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.origin_access_control_id = Some(v.into());
        self
    }
    #[doc = "Set the field `origin_path`.\n"]
    pub fn set_origin_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.origin_path = Some(v.into());
        self
    }
    #[doc = "Set the field `response_completion_timeout`.\n"]
    pub fn set_response_completion_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.response_completion_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_header`.\n"]
    pub fn set_custom_header(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontDistributionOriginElCustomHeaderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_header = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_header = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `custom_origin_config`.\n"]
    pub fn set_custom_origin_config(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontDistributionOriginElCustomOriginConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_origin_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_origin_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `origin_shield`.\n"]
    pub fn set_origin_shield(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontDistributionOriginElOriginShieldEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.origin_shield = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.origin_shield = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `s3_origin_config`.\n"]
    pub fn set_s3_origin_config(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontDistributionOriginElS3OriginConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_origin_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_origin_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vpc_origin_config`.\n"]
    pub fn set_vpc_origin_config(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontDistributionOriginElVpcOriginConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vpc_origin_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_origin_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudfrontDistributionOriginEl {
    type O = BlockAssignable<CloudfrontDistributionOriginEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionOriginEl {
    #[doc = ""]
    pub domain_name: PrimField<String>,
    #[doc = ""]
    pub origin_id: PrimField<String>,
}
impl BuildCloudfrontDistributionOriginEl {
    pub fn build(self) -> CloudfrontDistributionOriginEl {
        CloudfrontDistributionOriginEl {
            connection_attempts: core::default::Default::default(),
            connection_timeout: core::default::Default::default(),
            domain_name: self.domain_name,
            origin_access_control_id: core::default::Default::default(),
            origin_id: self.origin_id,
            origin_path: core::default::Default::default(),
            response_completion_timeout: core::default::Default::default(),
            custom_header: core::default::Default::default(),
            custom_origin_config: core::default::Default::default(),
            origin_shield: core::default::Default::default(),
            s3_origin_config: core::default::Default::default(),
            vpc_origin_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudfrontDistributionOriginElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionOriginElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontDistributionOriginElRef {
        CloudfrontDistributionOriginElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionOriginElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_attempts` after provisioning.\n"]
    pub fn connection_attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_attempts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `connection_timeout` after provisioning.\n"]
    pub fn connection_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }
    #[doc = "Get a reference to the value of field `origin_access_control_id` after provisioning.\n"]
    pub fn origin_access_control_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.origin_access_control_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `origin_id` after provisioning.\n"]
    pub fn origin_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin_id", self.base))
    }
    #[doc = "Get a reference to the value of field `origin_path` after provisioning.\n"]
    pub fn origin_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin_path", self.base))
    }
    #[doc = "Get a reference to the value of field `response_completion_timeout` after provisioning.\n"]
    pub fn response_completion_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_completion_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_origin_config` after provisioning.\n"]
    pub fn custom_origin_config(
        &self,
    ) -> ListRef<CloudfrontDistributionOriginElCustomOriginConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_origin_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `origin_shield` after provisioning.\n"]
    pub fn origin_shield(&self) -> ListRef<CloudfrontDistributionOriginElOriginShieldElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.origin_shield", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_origin_config` after provisioning.\n"]
    pub fn s3_origin_config(&self) -> ListRef<CloudfrontDistributionOriginElS3OriginConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_origin_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_origin_config` after provisioning.\n"]
    pub fn vpc_origin_config(&self) -> ListRef<CloudfrontDistributionOriginElVpcOriginConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_origin_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudfrontDistributionOriginGroupElFailoverCriteriaEl {
    status_codes: SetField<PrimField<f64>>,
}
impl CloudfrontDistributionOriginGroupElFailoverCriteriaEl {}
impl ToListMappable for CloudfrontDistributionOriginGroupElFailoverCriteriaEl {
    type O = BlockAssignable<CloudfrontDistributionOriginGroupElFailoverCriteriaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionOriginGroupElFailoverCriteriaEl {
    #[doc = ""]
    pub status_codes: SetField<PrimField<f64>>,
}
impl BuildCloudfrontDistributionOriginGroupElFailoverCriteriaEl {
    pub fn build(self) -> CloudfrontDistributionOriginGroupElFailoverCriteriaEl {
        CloudfrontDistributionOriginGroupElFailoverCriteriaEl {
            status_codes: self.status_codes,
        }
    }
}
pub struct CloudfrontDistributionOriginGroupElFailoverCriteriaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionOriginGroupElFailoverCriteriaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontDistributionOriginGroupElFailoverCriteriaElRef {
        CloudfrontDistributionOriginGroupElFailoverCriteriaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionOriginGroupElFailoverCriteriaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `status_codes` after provisioning.\n"]
    pub fn status_codes(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.status_codes", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudfrontDistributionOriginGroupElMemberEl {
    origin_id: PrimField<String>,
}
impl CloudfrontDistributionOriginGroupElMemberEl {}
impl ToListMappable for CloudfrontDistributionOriginGroupElMemberEl {
    type O = BlockAssignable<CloudfrontDistributionOriginGroupElMemberEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionOriginGroupElMemberEl {
    #[doc = ""]
    pub origin_id: PrimField<String>,
}
impl BuildCloudfrontDistributionOriginGroupElMemberEl {
    pub fn build(self) -> CloudfrontDistributionOriginGroupElMemberEl {
        CloudfrontDistributionOriginGroupElMemberEl {
            origin_id: self.origin_id,
        }
    }
}
pub struct CloudfrontDistributionOriginGroupElMemberElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionOriginGroupElMemberElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontDistributionOriginGroupElMemberElRef {
        CloudfrontDistributionOriginGroupElMemberElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionOriginGroupElMemberElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `origin_id` after provisioning.\n"]
    pub fn origin_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin_id", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudfrontDistributionOriginGroupElDynamic {
    failover_criteria: Option<DynamicBlock<CloudfrontDistributionOriginGroupElFailoverCriteriaEl>>,
    member: Option<DynamicBlock<CloudfrontDistributionOriginGroupElMemberEl>>,
}
#[derive(Serialize)]
pub struct CloudfrontDistributionOriginGroupEl {
    origin_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failover_criteria: Option<Vec<CloudfrontDistributionOriginGroupElFailoverCriteriaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member: Option<Vec<CloudfrontDistributionOriginGroupElMemberEl>>,
    dynamic: CloudfrontDistributionOriginGroupElDynamic,
}
impl CloudfrontDistributionOriginGroupEl {
    #[doc = "Set the field `failover_criteria`.\n"]
    pub fn set_failover_criteria(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontDistributionOriginGroupElFailoverCriteriaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.failover_criteria = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.failover_criteria = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `member`.\n"]
    pub fn set_member(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontDistributionOriginGroupElMemberEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.member = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.member = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudfrontDistributionOriginGroupEl {
    type O = BlockAssignable<CloudfrontDistributionOriginGroupEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionOriginGroupEl {
    #[doc = ""]
    pub origin_id: PrimField<String>,
}
impl BuildCloudfrontDistributionOriginGroupEl {
    pub fn build(self) -> CloudfrontDistributionOriginGroupEl {
        CloudfrontDistributionOriginGroupEl {
            origin_id: self.origin_id,
            failover_criteria: core::default::Default::default(),
            member: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudfrontDistributionOriginGroupElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionOriginGroupElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontDistributionOriginGroupElRef {
        CloudfrontDistributionOriginGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionOriginGroupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `origin_id` after provisioning.\n"]
    pub fn origin_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin_id", self.base))
    }
    #[doc = "Get a reference to the value of field `failover_criteria` after provisioning.\n"]
    pub fn failover_criteria(
        &self,
    ) -> ListRef<CloudfrontDistributionOriginGroupElFailoverCriteriaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.failover_criteria", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `member` after provisioning.\n"]
    pub fn member(&self) -> ListRef<CloudfrontDistributionOriginGroupElMemberElRef> {
        ListRef::new(self.shared().clone(), format!("{}.member", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudfrontDistributionRestrictionsElGeoRestrictionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    locations: Option<SetField<PrimField<String>>>,
    restriction_type: PrimField<String>,
}
impl CloudfrontDistributionRestrictionsElGeoRestrictionEl {
    #[doc = "Set the field `locations`.\n"]
    pub fn set_locations(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.locations = Some(v.into());
        self
    }
}
impl ToListMappable for CloudfrontDistributionRestrictionsElGeoRestrictionEl {
    type O = BlockAssignable<CloudfrontDistributionRestrictionsElGeoRestrictionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionRestrictionsElGeoRestrictionEl {
    #[doc = ""]
    pub restriction_type: PrimField<String>,
}
impl BuildCloudfrontDistributionRestrictionsElGeoRestrictionEl {
    pub fn build(self) -> CloudfrontDistributionRestrictionsElGeoRestrictionEl {
        CloudfrontDistributionRestrictionsElGeoRestrictionEl {
            locations: core::default::Default::default(),
            restriction_type: self.restriction_type,
        }
    }
}
pub struct CloudfrontDistributionRestrictionsElGeoRestrictionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionRestrictionsElGeoRestrictionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontDistributionRestrictionsElGeoRestrictionElRef {
        CloudfrontDistributionRestrictionsElGeoRestrictionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionRestrictionsElGeoRestrictionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `locations` after provisioning.\n"]
    pub fn locations(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
    #[doc = "Get a reference to the value of field `restriction_type` after provisioning.\n"]
    pub fn restriction_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.restriction_type", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CloudfrontDistributionRestrictionsElDynamic {
    geo_restriction: Option<DynamicBlock<CloudfrontDistributionRestrictionsElGeoRestrictionEl>>,
}
#[derive(Serialize)]
pub struct CloudfrontDistributionRestrictionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    geo_restriction: Option<Vec<CloudfrontDistributionRestrictionsElGeoRestrictionEl>>,
    dynamic: CloudfrontDistributionRestrictionsElDynamic,
}
impl CloudfrontDistributionRestrictionsEl {
    #[doc = "Set the field `geo_restriction`.\n"]
    pub fn set_geo_restriction(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontDistributionRestrictionsElGeoRestrictionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.geo_restriction = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.geo_restriction = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudfrontDistributionRestrictionsEl {
    type O = BlockAssignable<CloudfrontDistributionRestrictionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionRestrictionsEl {}
impl BuildCloudfrontDistributionRestrictionsEl {
    pub fn build(self) -> CloudfrontDistributionRestrictionsEl {
        CloudfrontDistributionRestrictionsEl {
            geo_restriction: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudfrontDistributionRestrictionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionRestrictionsElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontDistributionRestrictionsElRef {
        CloudfrontDistributionRestrictionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionRestrictionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `geo_restriction` after provisioning.\n"]
    pub fn geo_restriction(
        &self,
    ) -> ListRef<CloudfrontDistributionRestrictionsElGeoRestrictionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.geo_restriction", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudfrontDistributionViewerCertificateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    acm_certificate_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudfront_default_certificate: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_certificate_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_protocol_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_support_method: Option<PrimField<String>>,
}
impl CloudfrontDistributionViewerCertificateEl {
    #[doc = "Set the field `acm_certificate_arn`.\n"]
    pub fn set_acm_certificate_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.acm_certificate_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `cloudfront_default_certificate`.\n"]
    pub fn set_cloudfront_default_certificate(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cloudfront_default_certificate = Some(v.into());
        self
    }
    #[doc = "Set the field `iam_certificate_id`.\n"]
    pub fn set_iam_certificate_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.iam_certificate_id = Some(v.into());
        self
    }
    #[doc = "Set the field `minimum_protocol_version`.\n"]
    pub fn set_minimum_protocol_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.minimum_protocol_version = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl_support_method`.\n"]
    pub fn set_ssl_support_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_support_method = Some(v.into());
        self
    }
}
impl ToListMappable for CloudfrontDistributionViewerCertificateEl {
    type O = BlockAssignable<CloudfrontDistributionViewerCertificateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontDistributionViewerCertificateEl {}
impl BuildCloudfrontDistributionViewerCertificateEl {
    pub fn build(self) -> CloudfrontDistributionViewerCertificateEl {
        CloudfrontDistributionViewerCertificateEl {
            acm_certificate_arn: core::default::Default::default(),
            cloudfront_default_certificate: core::default::Default::default(),
            iam_certificate_id: core::default::Default::default(),
            minimum_protocol_version: core::default::Default::default(),
            ssl_support_method: core::default::Default::default(),
        }
    }
}
pub struct CloudfrontDistributionViewerCertificateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontDistributionViewerCertificateElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontDistributionViewerCertificateElRef {
        CloudfrontDistributionViewerCertificateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontDistributionViewerCertificateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `acm_certificate_arn` after provisioning.\n"]
    pub fn acm_certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.acm_certificate_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloudfront_default_certificate` after provisioning.\n"]
    pub fn cloudfront_default_certificate(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloudfront_default_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `iam_certificate_id` after provisioning.\n"]
    pub fn iam_certificate_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_certificate_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `minimum_protocol_version` after provisioning.\n"]
    pub fn minimum_protocol_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minimum_protocol_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_support_method` after provisioning.\n"]
    pub fn ssl_support_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_support_method", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CloudfrontDistributionDynamic {
    custom_error_response: Option<DynamicBlock<CloudfrontDistributionCustomErrorResponseEl>>,
    default_cache_behavior: Option<DynamicBlock<CloudfrontDistributionDefaultCacheBehaviorEl>>,
    logging_config: Option<DynamicBlock<CloudfrontDistributionLoggingConfigEl>>,
    ordered_cache_behavior: Option<DynamicBlock<CloudfrontDistributionOrderedCacheBehaviorEl>>,
    origin: Option<DynamicBlock<CloudfrontDistributionOriginEl>>,
    origin_group: Option<DynamicBlock<CloudfrontDistributionOriginGroupEl>>,
    restrictions: Option<DynamicBlock<CloudfrontDistributionRestrictionsEl>>,
    viewer_certificate: Option<DynamicBlock<CloudfrontDistributionViewerCertificateEl>>,
}
