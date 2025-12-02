use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct LightsailDistributionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    bundle_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_enabled: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_behavior: Option<Vec<LightsailDistributionCacheBehaviorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_behavior_settings: Option<Vec<LightsailDistributionCacheBehaviorSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_cache_behavior: Option<Vec<LightsailDistributionDefaultCacheBehaviorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<Vec<LightsailDistributionOriginEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<LightsailDistributionTimeoutsEl>,
    dynamic: LightsailDistributionDynamic,
}
struct LightsailDistribution_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LightsailDistributionData>,
}
#[derive(Clone)]
pub struct LightsailDistribution(Rc<LightsailDistribution_>);
impl LightsailDistribution {
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
    #[doc = "Set the field `certificate_name`.\nThe name of the SSL/TLS certificate attached to the distribution, if any."]
    pub fn set_certificate_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().certificate_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_address_type`.\nThe IP address type of the distribution."]
    pub fn set_ip_address_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ip_address_type = Some(v.into());
        self
    }
    #[doc = "Set the field `is_enabled`.\nIndicates whether the distribution is enabled."]
    pub fn set_is_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
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
    #[doc = "Set the field `cache_behavior`.\n"]
    pub fn set_cache_behavior(
        self,
        v: impl Into<BlockAssignable<LightsailDistributionCacheBehaviorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cache_behavior = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cache_behavior = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cache_behavior_settings`.\n"]
    pub fn set_cache_behavior_settings(
        self,
        v: impl Into<BlockAssignable<LightsailDistributionCacheBehaviorSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cache_behavior_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cache_behavior_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `default_cache_behavior`.\n"]
    pub fn set_default_cache_behavior(
        self,
        v: impl Into<BlockAssignable<LightsailDistributionDefaultCacheBehaviorEl>>,
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
    #[doc = "Set the field `origin`.\n"]
    pub fn set_origin(self, v: impl Into<BlockAssignable<LightsailDistributionOriginEl>>) -> Self {
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
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<LightsailDistributionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `alternative_domain_names` after provisioning.\nThe alternate domain names of the distribution."]
    pub fn alternative_domain_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.alternative_domain_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\nThe Amazon Resource Name (ARN) of the distribution."]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `bundle_id` after provisioning.\nThe bundle ID to use for the distribution."]
    pub fn bundle_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bundle_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `certificate_name` after provisioning.\nThe name of the SSL/TLS certificate attached to the distribution, if any."]
    pub fn certificate_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nThe timestamp when the distribution was created."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\nThe domain name of the distribution."]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\nThe IP address type of the distribution."]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `is_enabled` after provisioning.\nIndicates whether the distribution is enabled."]
    pub fn is_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nAn object that describes the location of the distribution, such as the AWS Region and Availability Zone."]
    pub fn location(&self) -> ListRef<LightsailDistributionLocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the distribution."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `origin_public_dns` after provisioning.\nThe public DNS of the origin."]
    pub fn origin_public_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.origin_public_dns", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\nThe Lightsail resource type (e.g., Distribution)."]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the distribution."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `support_code` after provisioning.\nThe support code. Include this code in your email to support when you have questions about your Lightsail distribution. This code enables our support team to look up your Lightsail information more easily."]
    pub fn support_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.support_code", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `cache_behavior_settings` after provisioning.\n"]
    pub fn cache_behavior_settings(
        &self,
    ) -> ListRef<LightsailDistributionCacheBehaviorSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cache_behavior_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_cache_behavior` after provisioning.\n"]
    pub fn default_cache_behavior(
        &self,
    ) -> ListRef<LightsailDistributionDefaultCacheBehaviorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_cache_behavior", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> ListRef<LightsailDistributionOriginElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.origin", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LightsailDistributionTimeoutsElRef {
        LightsailDistributionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for LightsailDistribution {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for LightsailDistribution {}
impl ToListMappable for LightsailDistribution {
    type O = ListRef<LightsailDistributionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for LightsailDistribution_ {
    fn extract_resource_type(&self) -> String {
        "aws_lightsail_distribution".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildLightsailDistribution {
    pub tf_id: String,
    #[doc = "The bundle ID to use for the distribution."]
    pub bundle_id: PrimField<String>,
    #[doc = "The name of the distribution."]
    pub name: PrimField<String>,
}
impl BuildLightsailDistribution {
    pub fn build(self, stack: &mut Stack) -> LightsailDistribution {
        let out = LightsailDistribution(Rc::new(LightsailDistribution_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LightsailDistributionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bundle_id: self.bundle_id,
                certificate_name: core::default::Default::default(),
                id: core::default::Default::default(),
                ip_address_type: core::default::Default::default(),
                is_enabled: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                cache_behavior: core::default::Default::default(),
                cache_behavior_settings: core::default::Default::default(),
                default_cache_behavior: core::default::Default::default(),
                origin: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct LightsailDistributionRef {
    shared: StackShared,
    base: String,
}
impl Ref for LightsailDistributionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl LightsailDistributionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `alternative_domain_names` after provisioning.\nThe alternate domain names of the distribution."]
    pub fn alternative_domain_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.alternative_domain_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\nThe Amazon Resource Name (ARN) of the distribution."]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `bundle_id` after provisioning.\nThe bundle ID to use for the distribution."]
    pub fn bundle_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bundle_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `certificate_name` after provisioning.\nThe name of the SSL/TLS certificate attached to the distribution, if any."]
    pub fn certificate_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nThe timestamp when the distribution was created."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\nThe domain name of the distribution."]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\nThe IP address type of the distribution."]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `is_enabled` after provisioning.\nIndicates whether the distribution is enabled."]
    pub fn is_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nAn object that describes the location of the distribution, such as the AWS Region and Availability Zone."]
    pub fn location(&self) -> ListRef<LightsailDistributionLocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the distribution."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `origin_public_dns` after provisioning.\nThe public DNS of the origin."]
    pub fn origin_public_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.origin_public_dns", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\nThe Lightsail resource type (e.g., Distribution)."]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the distribution."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `support_code` after provisioning.\nThe support code. Include this code in your email to support when you have questions about your Lightsail distribution. This code enables our support team to look up your Lightsail information more easily."]
    pub fn support_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.support_code", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `cache_behavior_settings` after provisioning.\n"]
    pub fn cache_behavior_settings(
        &self,
    ) -> ListRef<LightsailDistributionCacheBehaviorSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cache_behavior_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_cache_behavior` after provisioning.\n"]
    pub fn default_cache_behavior(
        &self,
    ) -> ListRef<LightsailDistributionDefaultCacheBehaviorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_cache_behavior", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> ListRef<LightsailDistributionOriginElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.origin", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LightsailDistributionTimeoutsElRef {
        LightsailDistributionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct LightsailDistributionLocationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region_name: Option<PrimField<String>>,
}
impl LightsailDistributionLocationEl {
    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.availability_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `region_name`.\n"]
    pub fn set_region_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region_name = Some(v.into());
        self
    }
}
impl ToListMappable for LightsailDistributionLocationEl {
    type O = BlockAssignable<LightsailDistributionLocationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLightsailDistributionLocationEl {}
impl BuildLightsailDistributionLocationEl {
    pub fn build(self) -> LightsailDistributionLocationEl {
        LightsailDistributionLocationEl {
            availability_zone: core::default::Default::default(),
            region_name: core::default::Default::default(),
        }
    }
}
pub struct LightsailDistributionLocationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LightsailDistributionLocationElRef {
    fn new(shared: StackShared, base: String) -> LightsailDistributionLocationElRef {
        LightsailDistributionLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LightsailDistributionLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `region_name` after provisioning.\n"]
    pub fn region_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region_name", self.base))
    }
}
#[derive(Serialize)]
pub struct LightsailDistributionCacheBehaviorEl {
    behavior: PrimField<String>,
    path: PrimField<String>,
}
impl LightsailDistributionCacheBehaviorEl {}
impl ToListMappable for LightsailDistributionCacheBehaviorEl {
    type O = BlockAssignable<LightsailDistributionCacheBehaviorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLightsailDistributionCacheBehaviorEl {
    #[doc = "The cache behavior for the specified path."]
    pub behavior: PrimField<String>,
    #[doc = "The path to a directory or file to cached, or not cache. Use an asterisk symbol to specify wildcard directories (path/to/assets/*), and file types (*.html, *jpg, *js). Directories and file paths are case-sensitive."]
    pub path: PrimField<String>,
}
impl BuildLightsailDistributionCacheBehaviorEl {
    pub fn build(self) -> LightsailDistributionCacheBehaviorEl {
        LightsailDistributionCacheBehaviorEl {
            behavior: self.behavior,
            path: self.path,
        }
    }
}
pub struct LightsailDistributionCacheBehaviorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LightsailDistributionCacheBehaviorElRef {
    fn new(shared: StackShared, base: String) -> LightsailDistributionCacheBehaviorElRef {
        LightsailDistributionCacheBehaviorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LightsailDistributionCacheBehaviorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `behavior` after provisioning.\nThe cache behavior for the specified path."]
    pub fn behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.behavior", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nThe path to a directory or file to cached, or not cache. Use an asterisk symbol to specify wildcard directories (path/to/assets/*), and file types (*.html, *jpg, *js). Directories and file paths are case-sensitive."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}
#[derive(Serialize)]
pub struct LightsailDistributionCacheBehaviorSettingsElForwardedCookiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cookies_allow_list: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    option: Option<PrimField<String>>,
}
impl LightsailDistributionCacheBehaviorSettingsElForwardedCookiesEl {
    #[doc = "Set the field `cookies_allow_list`.\nThe specific cookies to forward to your distribution's origin."]
    pub fn set_cookies_allow_list(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.cookies_allow_list = Some(v.into());
        self
    }
    #[doc = "Set the field `option`.\nSpecifies which cookies to forward to the distribution's origin for a cache behavior: all, none, or allow-list to forward only the cookies specified in the cookiesAllowList parameter."]
    pub fn set_option(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.option = Some(v.into());
        self
    }
}
impl ToListMappable for LightsailDistributionCacheBehaviorSettingsElForwardedCookiesEl {
    type O = BlockAssignable<LightsailDistributionCacheBehaviorSettingsElForwardedCookiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLightsailDistributionCacheBehaviorSettingsElForwardedCookiesEl {}
impl BuildLightsailDistributionCacheBehaviorSettingsElForwardedCookiesEl {
    pub fn build(self) -> LightsailDistributionCacheBehaviorSettingsElForwardedCookiesEl {
        LightsailDistributionCacheBehaviorSettingsElForwardedCookiesEl {
            cookies_allow_list: core::default::Default::default(),
            option: core::default::Default::default(),
        }
    }
}
pub struct LightsailDistributionCacheBehaviorSettingsElForwardedCookiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LightsailDistributionCacheBehaviorSettingsElForwardedCookiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LightsailDistributionCacheBehaviorSettingsElForwardedCookiesElRef {
        LightsailDistributionCacheBehaviorSettingsElForwardedCookiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LightsailDistributionCacheBehaviorSettingsElForwardedCookiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cookies_allow_list` after provisioning.\nThe specific cookies to forward to your distribution's origin."]
    pub fn cookies_allow_list(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.cookies_allow_list", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `option` after provisioning.\nSpecifies which cookies to forward to the distribution's origin for a cache behavior: all, none, or allow-list to forward only the cookies specified in the cookiesAllowList parameter."]
    pub fn option(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.option", self.base))
    }
}
#[derive(Serialize)]
pub struct LightsailDistributionCacheBehaviorSettingsElForwardedHeadersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    headers_allow_list: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    option: Option<PrimField<String>>,
}
impl LightsailDistributionCacheBehaviorSettingsElForwardedHeadersEl {
    #[doc = "Set the field `headers_allow_list`.\nThe specific headers to forward to your distribution's origin."]
    pub fn set_headers_allow_list(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.headers_allow_list = Some(v.into());
        self
    }
    #[doc = "Set the field `option`.\nThe headers that you want your distribution to forward to your origin and base caching on."]
    pub fn set_option(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.option = Some(v.into());
        self
    }
}
impl ToListMappable for LightsailDistributionCacheBehaviorSettingsElForwardedHeadersEl {
    type O = BlockAssignable<LightsailDistributionCacheBehaviorSettingsElForwardedHeadersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLightsailDistributionCacheBehaviorSettingsElForwardedHeadersEl {}
impl BuildLightsailDistributionCacheBehaviorSettingsElForwardedHeadersEl {
    pub fn build(self) -> LightsailDistributionCacheBehaviorSettingsElForwardedHeadersEl {
        LightsailDistributionCacheBehaviorSettingsElForwardedHeadersEl {
            headers_allow_list: core::default::Default::default(),
            option: core::default::Default::default(),
        }
    }
}
pub struct LightsailDistributionCacheBehaviorSettingsElForwardedHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LightsailDistributionCacheBehaviorSettingsElForwardedHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LightsailDistributionCacheBehaviorSettingsElForwardedHeadersElRef {
        LightsailDistributionCacheBehaviorSettingsElForwardedHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LightsailDistributionCacheBehaviorSettingsElForwardedHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `headers_allow_list` after provisioning.\nThe specific headers to forward to your distribution's origin."]
    pub fn headers_allow_list(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.headers_allow_list", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `option` after provisioning.\nThe headers that you want your distribution to forward to your origin and base caching on."]
    pub fn option(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.option", self.base))
    }
}
#[derive(Serialize)]
pub struct LightsailDistributionCacheBehaviorSettingsElForwardedQueryStringsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    option: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_strings_allowed_list: Option<SetField<PrimField<String>>>,
}
impl LightsailDistributionCacheBehaviorSettingsElForwardedQueryStringsEl {
    #[doc = "Set the field `option`.\nIndicates whether the distribution forwards and caches based on query strings."]
    pub fn set_option(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.option = Some(v.into());
        self
    }
    #[doc = "Set the field `query_strings_allowed_list`.\nThe specific query strings that the distribution forwards to the origin."]
    pub fn set_query_strings_allowed_list(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.query_strings_allowed_list = Some(v.into());
        self
    }
}
impl ToListMappable for LightsailDistributionCacheBehaviorSettingsElForwardedQueryStringsEl {
    type O = BlockAssignable<LightsailDistributionCacheBehaviorSettingsElForwardedQueryStringsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLightsailDistributionCacheBehaviorSettingsElForwardedQueryStringsEl {}
impl BuildLightsailDistributionCacheBehaviorSettingsElForwardedQueryStringsEl {
    pub fn build(self) -> LightsailDistributionCacheBehaviorSettingsElForwardedQueryStringsEl {
        LightsailDistributionCacheBehaviorSettingsElForwardedQueryStringsEl {
            option: core::default::Default::default(),
            query_strings_allowed_list: core::default::Default::default(),
        }
    }
}
pub struct LightsailDistributionCacheBehaviorSettingsElForwardedQueryStringsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LightsailDistributionCacheBehaviorSettingsElForwardedQueryStringsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LightsailDistributionCacheBehaviorSettingsElForwardedQueryStringsElRef {
        LightsailDistributionCacheBehaviorSettingsElForwardedQueryStringsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LightsailDistributionCacheBehaviorSettingsElForwardedQueryStringsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `option` after provisioning.\nIndicates whether the distribution forwards and caches based on query strings."]
    pub fn option(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.option", self.base))
    }
    #[doc = "Get a reference to the value of field `query_strings_allowed_list` after provisioning.\nThe specific query strings that the distribution forwards to the origin."]
    pub fn query_strings_allowed_list(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.query_strings_allowed_list", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct LightsailDistributionCacheBehaviorSettingsElDynamic {
    forwarded_cookies:
        Option<DynamicBlock<LightsailDistributionCacheBehaviorSettingsElForwardedCookiesEl>>,
    forwarded_headers:
        Option<DynamicBlock<LightsailDistributionCacheBehaviorSettingsElForwardedHeadersEl>>,
    forwarded_query_strings:
        Option<DynamicBlock<LightsailDistributionCacheBehaviorSettingsElForwardedQueryStringsEl>>,
}
#[derive(Serialize)]
pub struct LightsailDistributionCacheBehaviorSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_http_methods: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cached_http_methods: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forwarded_cookies: Option<Vec<LightsailDistributionCacheBehaviorSettingsElForwardedCookiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forwarded_headers: Option<Vec<LightsailDistributionCacheBehaviorSettingsElForwardedHeadersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forwarded_query_strings:
        Option<Vec<LightsailDistributionCacheBehaviorSettingsElForwardedQueryStringsEl>>,
    dynamic: LightsailDistributionCacheBehaviorSettingsElDynamic,
}
impl LightsailDistributionCacheBehaviorSettingsEl {
    #[doc = "Set the field `allowed_http_methods`.\nThe HTTP methods that are processed and forwarded to the distribution's origin."]
    pub fn set_allowed_http_methods(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.allowed_http_methods = Some(v.into());
        self
    }
    #[doc = "Set the field `cached_http_methods`.\nThe HTTP method responses that are cached by your distribution."]
    pub fn set_cached_http_methods(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cached_http_methods = Some(v.into());
        self
    }
    #[doc = "Set the field `default_ttl`.\nThe default amount of time that objects stay in the distribution's cache before the distribution forwards another request to the origin to determine whether the content has been updated."]
    pub fn set_default_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `maximum_ttl`.\nThe maximum amount of time that objects stay in the distribution's cache before the distribution forwards another request to the origin to determine whether the object has been updated."]
    pub fn set_maximum_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `minimum_ttl`.\nThe minimum amount of time that objects stay in the distribution's cache before the distribution forwards another request to the origin to determine whether the object has been updated."]
    pub fn set_minimum_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minimum_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `forwarded_cookies`.\n"]
    pub fn set_forwarded_cookies(
        mut self,
        v: impl Into<BlockAssignable<LightsailDistributionCacheBehaviorSettingsElForwardedCookiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.forwarded_cookies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.forwarded_cookies = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `forwarded_headers`.\n"]
    pub fn set_forwarded_headers(
        mut self,
        v: impl Into<BlockAssignable<LightsailDistributionCacheBehaviorSettingsElForwardedHeadersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.forwarded_headers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.forwarded_headers = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `forwarded_query_strings`.\n"]
    pub fn set_forwarded_query_strings(
        mut self,
        v: impl Into<
            BlockAssignable<LightsailDistributionCacheBehaviorSettingsElForwardedQueryStringsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.forwarded_query_strings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.forwarded_query_strings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for LightsailDistributionCacheBehaviorSettingsEl {
    type O = BlockAssignable<LightsailDistributionCacheBehaviorSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLightsailDistributionCacheBehaviorSettingsEl {}
impl BuildLightsailDistributionCacheBehaviorSettingsEl {
    pub fn build(self) -> LightsailDistributionCacheBehaviorSettingsEl {
        LightsailDistributionCacheBehaviorSettingsEl {
            allowed_http_methods: core::default::Default::default(),
            cached_http_methods: core::default::Default::default(),
            default_ttl: core::default::Default::default(),
            maximum_ttl: core::default::Default::default(),
            minimum_ttl: core::default::Default::default(),
            forwarded_cookies: core::default::Default::default(),
            forwarded_headers: core::default::Default::default(),
            forwarded_query_strings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LightsailDistributionCacheBehaviorSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LightsailDistributionCacheBehaviorSettingsElRef {
    fn new(shared: StackShared, base: String) -> LightsailDistributionCacheBehaviorSettingsElRef {
        LightsailDistributionCacheBehaviorSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LightsailDistributionCacheBehaviorSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_http_methods` after provisioning.\nThe HTTP methods that are processed and forwarded to the distribution's origin."]
    pub fn allowed_http_methods(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allowed_http_methods", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cached_http_methods` after provisioning.\nThe HTTP method responses that are cached by your distribution."]
    pub fn cached_http_methods(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cached_http_methods", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_ttl` after provisioning.\nThe default amount of time that objects stay in the distribution's cache before the distribution forwards another request to the origin to determine whether the content has been updated."]
    pub fn default_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `maximum_ttl` after provisioning.\nThe maximum amount of time that objects stay in the distribution's cache before the distribution forwards another request to the origin to determine whether the object has been updated."]
    pub fn maximum_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `minimum_ttl` after provisioning.\nThe minimum amount of time that objects stay in the distribution's cache before the distribution forwards another request to the origin to determine whether the object has been updated."]
    pub fn minimum_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `forwarded_cookies` after provisioning.\n"]
    pub fn forwarded_cookies(
        &self,
    ) -> ListRef<LightsailDistributionCacheBehaviorSettingsElForwardedCookiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.forwarded_cookies", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `forwarded_headers` after provisioning.\n"]
    pub fn forwarded_headers(
        &self,
    ) -> ListRef<LightsailDistributionCacheBehaviorSettingsElForwardedHeadersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.forwarded_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `forwarded_query_strings` after provisioning.\n"]
    pub fn forwarded_query_strings(
        &self,
    ) -> ListRef<LightsailDistributionCacheBehaviorSettingsElForwardedQueryStringsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.forwarded_query_strings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct LightsailDistributionDefaultCacheBehaviorEl {
    behavior: PrimField<String>,
}
impl LightsailDistributionDefaultCacheBehaviorEl {}
impl ToListMappable for LightsailDistributionDefaultCacheBehaviorEl {
    type O = BlockAssignable<LightsailDistributionDefaultCacheBehaviorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLightsailDistributionDefaultCacheBehaviorEl {
    #[doc = "The cache behavior of the distribution."]
    pub behavior: PrimField<String>,
}
impl BuildLightsailDistributionDefaultCacheBehaviorEl {
    pub fn build(self) -> LightsailDistributionDefaultCacheBehaviorEl {
        LightsailDistributionDefaultCacheBehaviorEl {
            behavior: self.behavior,
        }
    }
}
pub struct LightsailDistributionDefaultCacheBehaviorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LightsailDistributionDefaultCacheBehaviorElRef {
    fn new(shared: StackShared, base: String) -> LightsailDistributionDefaultCacheBehaviorElRef {
        LightsailDistributionDefaultCacheBehaviorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LightsailDistributionDefaultCacheBehaviorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `behavior` after provisioning.\nThe cache behavior of the distribution."]
    pub fn behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.behavior", self.base))
    }
}
#[derive(Serialize)]
pub struct LightsailDistributionOriginEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol_policy: Option<PrimField<String>>,
    region_name: PrimField<String>,
}
impl LightsailDistributionOriginEl {
    #[doc = "Set the field `protocol_policy`.\nThe protocol that your Amazon Lightsail distribution uses when establishing a connection with your origin to pull content."]
    pub fn set_protocol_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol_policy = Some(v.into());
        self
    }
}
impl ToListMappable for LightsailDistributionOriginEl {
    type O = BlockAssignable<LightsailDistributionOriginEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLightsailDistributionOriginEl {
    #[doc = "The name of the origin resource."]
    pub name: PrimField<String>,
    #[doc = "The AWS Region name of the origin resource."]
    pub region_name: PrimField<String>,
}
impl BuildLightsailDistributionOriginEl {
    pub fn build(self) -> LightsailDistributionOriginEl {
        LightsailDistributionOriginEl {
            name: self.name,
            protocol_policy: core::default::Default::default(),
            region_name: self.region_name,
        }
    }
}
pub struct LightsailDistributionOriginElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LightsailDistributionOriginElRef {
    fn new(shared: StackShared, base: String) -> LightsailDistributionOriginElRef {
        LightsailDistributionOriginElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LightsailDistributionOriginElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the origin resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `protocol_policy` after provisioning.\nThe protocol that your Amazon Lightsail distribution uses when establishing a connection with your origin to pull content."]
    pub fn protocol_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `region_name` after provisioning.\nThe AWS Region name of the origin resource."]
    pub fn region_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region_name", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\nThe resource type of the origin resource (e.g., Instance)."]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct LightsailDistributionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl LightsailDistributionTimeoutsEl {
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
impl ToListMappable for LightsailDistributionTimeoutsEl {
    type O = BlockAssignable<LightsailDistributionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLightsailDistributionTimeoutsEl {}
impl BuildLightsailDistributionTimeoutsEl {
    pub fn build(self) -> LightsailDistributionTimeoutsEl {
        LightsailDistributionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct LightsailDistributionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LightsailDistributionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> LightsailDistributionTimeoutsElRef {
        LightsailDistributionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LightsailDistributionTimeoutsElRef {
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
struct LightsailDistributionDynamic {
    cache_behavior: Option<DynamicBlock<LightsailDistributionCacheBehaviorEl>>,
    cache_behavior_settings: Option<DynamicBlock<LightsailDistributionCacheBehaviorSettingsEl>>,
    default_cache_behavior: Option<DynamicBlock<LightsailDistributionDefaultCacheBehaviorEl>>,
    origin: Option<DynamicBlock<LightsailDistributionOriginEl>>,
}
