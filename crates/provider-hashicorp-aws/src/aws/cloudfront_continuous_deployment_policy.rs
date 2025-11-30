use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct CloudfrontContinuousDeploymentPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    staging_distribution_dns_names:
        Option<Vec<CloudfrontContinuousDeploymentPolicyStagingDistributionDnsNamesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    traffic_config: Option<Vec<CloudfrontContinuousDeploymentPolicyTrafficConfigEl>>,
    dynamic: CloudfrontContinuousDeploymentPolicyDynamic,
}

struct CloudfrontContinuousDeploymentPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudfrontContinuousDeploymentPolicyData>,
}

#[derive(Clone)]
pub struct CloudfrontContinuousDeploymentPolicy(Rc<CloudfrontContinuousDeploymentPolicy_>);

impl CloudfrontContinuousDeploymentPolicy {
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

    #[doc = "Set the field `staging_distribution_dns_names`.\n"]
    pub fn set_staging_distribution_dns_names(
        self,
        v: impl Into<BlockAssignable<CloudfrontContinuousDeploymentPolicyStagingDistributionDnsNamesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().staging_distribution_dns_names = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .staging_distribution_dns_names = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `traffic_config`.\n"]
    pub fn set_traffic_config(
        self,
        v: impl Into<BlockAssignable<CloudfrontContinuousDeploymentPolicyTrafficConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().traffic_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.traffic_config = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified_time` after provisioning.\n"]
    pub fn last_modified_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `staging_distribution_dns_names` after provisioning.\n"]
    pub fn staging_distribution_dns_names(
        &self,
    ) -> ListRef<CloudfrontContinuousDeploymentPolicyStagingDistributionDnsNamesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.staging_distribution_dns_names", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `traffic_config` after provisioning.\n"]
    pub fn traffic_config(
        &self,
    ) -> ListRef<CloudfrontContinuousDeploymentPolicyTrafficConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.traffic_config", self.extract_ref()),
        )
    }
}

impl Referable for CloudfrontContinuousDeploymentPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for CloudfrontContinuousDeploymentPolicy {}

impl ToListMappable for CloudfrontContinuousDeploymentPolicy {
    type O = ListRef<CloudfrontContinuousDeploymentPolicyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CloudfrontContinuousDeploymentPolicy_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudfront_continuous_deployment_policy".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudfrontContinuousDeploymentPolicy {
    pub tf_id: String,
    #[doc = ""]
    pub enabled: PrimField<bool>,
}

impl BuildCloudfrontContinuousDeploymentPolicy {
    pub fn build(self, stack: &mut Stack) -> CloudfrontContinuousDeploymentPolicy {
        let out =
            CloudfrontContinuousDeploymentPolicy(Rc::new(CloudfrontContinuousDeploymentPolicy_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(CloudfrontContinuousDeploymentPolicyData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    enabled: self.enabled,
                    staging_distribution_dns_names: core::default::Default::default(),
                    traffic_config: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudfrontContinuousDeploymentPolicyRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontContinuousDeploymentPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl CloudfrontContinuousDeploymentPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified_time` after provisioning.\n"]
    pub fn last_modified_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `staging_distribution_dns_names` after provisioning.\n"]
    pub fn staging_distribution_dns_names(
        &self,
    ) -> ListRef<CloudfrontContinuousDeploymentPolicyStagingDistributionDnsNamesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.staging_distribution_dns_names", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `traffic_config` after provisioning.\n"]
    pub fn traffic_config(
        &self,
    ) -> ListRef<CloudfrontContinuousDeploymentPolicyTrafficConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.traffic_config", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct CloudfrontContinuousDeploymentPolicyStagingDistributionDnsNamesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<SetField<PrimField<String>>>,
    quantity: PrimField<f64>,
}

impl CloudfrontContinuousDeploymentPolicyStagingDistributionDnsNamesEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.items = Some(v.into());
        self
    }
}

impl ToListMappable for CloudfrontContinuousDeploymentPolicyStagingDistributionDnsNamesEl {
    type O = BlockAssignable<CloudfrontContinuousDeploymentPolicyStagingDistributionDnsNamesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontContinuousDeploymentPolicyStagingDistributionDnsNamesEl {
    #[doc = ""]
    pub quantity: PrimField<f64>,
}

impl BuildCloudfrontContinuousDeploymentPolicyStagingDistributionDnsNamesEl {
    pub fn build(self) -> CloudfrontContinuousDeploymentPolicyStagingDistributionDnsNamesEl {
        CloudfrontContinuousDeploymentPolicyStagingDistributionDnsNamesEl {
            items: core::default::Default::default(),
            quantity: self.quantity,
        }
    }
}

pub struct CloudfrontContinuousDeploymentPolicyStagingDistributionDnsNamesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontContinuousDeploymentPolicyStagingDistributionDnsNamesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontContinuousDeploymentPolicyStagingDistributionDnsNamesElRef {
        CloudfrontContinuousDeploymentPolicyStagingDistributionDnsNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontContinuousDeploymentPolicyStagingDistributionDnsNamesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }

    #[doc = "Get a reference to the value of field `quantity` after provisioning.\n"]
    pub fn quantity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.quantity", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleHeaderConfigEl {
    header: PrimField<String>,
    value: PrimField<String>,
}

impl CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleHeaderConfigEl {}

impl ToListMappable for CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleHeaderConfigEl {
    type O =
        BlockAssignable<CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleHeaderConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontContinuousDeploymentPolicyTrafficConfigElSingleHeaderConfigEl {
    #[doc = ""]
    pub header: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildCloudfrontContinuousDeploymentPolicyTrafficConfigElSingleHeaderConfigEl {
    pub fn build(self) -> CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleHeaderConfigEl {
        CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleHeaderConfigEl {
            header: self.header,
            value: self.value,
        }
    }
}

pub struct CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleHeaderConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleHeaderConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleHeaderConfigElRef {
        CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleHeaderConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleHeaderConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigElSessionStickinessConfigEl
{
    idle_ttl: PrimField<f64>,
    maximum_ttl: PrimField<f64>,
}

impl
    CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigElSessionStickinessConfigEl
{
}

impl ToListMappable for CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigElSessionStickinessConfigEl {
    type O =
        BlockAssignable<
            CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigElSessionStickinessConfigEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigElSessionStickinessConfigEl
{
    #[doc = ""]
    pub idle_ttl: PrimField<f64>,
    #[doc = ""]
    pub maximum_ttl: PrimField<f64>,
}

impl BuildCloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigElSessionStickinessConfigEl {
    pub fn build(
        self,
    ) -> CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigElSessionStickinessConfigEl {
        CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigElSessionStickinessConfigEl {
            idle_ttl: self.idle_ttl,
            maximum_ttl: self.maximum_ttl,
        }
    }
}

pub struct CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigElSessionStickinessConfigElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigElSessionStickinessConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigElSessionStickinessConfigElRef {
        CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigElSessionStickinessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigElSessionStickinessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle_ttl` after provisioning.\n"]
    pub fn idle_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.idle_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_ttl` after provisioning.\n"]
    pub fn maximum_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_ttl", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigElDynamic {
    session_stickiness_config: Option<
        DynamicBlock<CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigElSessionStickinessConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigEl {
    weight: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_stickiness_config: Option<
        Vec<CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigElSessionStickinessConfigEl>,
    >,
    dynamic: CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigElDynamic,
}

impl CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigEl {
    #[doc = "Set the field `session_stickiness_config`.\n"]
    pub fn set_session_stickiness_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigElSessionStickinessConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.session_stickiness_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.session_stickiness_config = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigEl {
    type O =
        BlockAssignable<CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigEl {
    #[doc = ""]
    pub weight: PrimField<f64>,
}

impl BuildCloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigEl {
    pub fn build(self) -> CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigEl {
        CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigEl {
            weight: self.weight,
            session_stickiness_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigElRef {
        CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }

    #[doc = "Get a reference to the value of field `session_stickiness_config` after provisioning.\n"]
    pub fn session_stickiness_config(
        &self,
    ) -> ListRef<CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigElSessionStickinessConfigElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.session_stickiness_config", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct CloudfrontContinuousDeploymentPolicyTrafficConfigElDynamic {
    single_header_config: Option<
        DynamicBlock<CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleHeaderConfigEl>,
    >,
    single_weight_config: Option<
        DynamicBlock<CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct CloudfrontContinuousDeploymentPolicyTrafficConfigEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    single_header_config:
        Option<Vec<CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleHeaderConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    single_weight_config:
        Option<Vec<CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigEl>>,
    dynamic: CloudfrontContinuousDeploymentPolicyTrafficConfigElDynamic,
}

impl CloudfrontContinuousDeploymentPolicyTrafficConfigEl {
    #[doc = "Set the field `single_header_config`.\n"]
    pub fn set_single_header_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleHeaderConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.single_header_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.single_header_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `single_weight_config`.\n"]
    pub fn set_single_weight_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.single_weight_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.single_weight_config = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CloudfrontContinuousDeploymentPolicyTrafficConfigEl {
    type O = BlockAssignable<CloudfrontContinuousDeploymentPolicyTrafficConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontContinuousDeploymentPolicyTrafficConfigEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildCloudfrontContinuousDeploymentPolicyTrafficConfigEl {
    pub fn build(self) -> CloudfrontContinuousDeploymentPolicyTrafficConfigEl {
        CloudfrontContinuousDeploymentPolicyTrafficConfigEl {
            type_: self.type_,
            single_header_config: core::default::Default::default(),
            single_weight_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudfrontContinuousDeploymentPolicyTrafficConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontContinuousDeploymentPolicyTrafficConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontContinuousDeploymentPolicyTrafficConfigElRef {
        CloudfrontContinuousDeploymentPolicyTrafficConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontContinuousDeploymentPolicyTrafficConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `single_header_config` after provisioning.\n"]
    pub fn single_header_config(
        &self,
    ) -> ListRef<CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleHeaderConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.single_header_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `single_weight_config` after provisioning.\n"]
    pub fn single_weight_config(
        &self,
    ) -> ListRef<CloudfrontContinuousDeploymentPolicyTrafficConfigElSingleWeightConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.single_weight_config", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct CloudfrontContinuousDeploymentPolicyDynamic {
    staging_distribution_dns_names:
        Option<DynamicBlock<CloudfrontContinuousDeploymentPolicyStagingDistributionDnsNamesEl>>,
    traffic_config: Option<DynamicBlock<CloudfrontContinuousDeploymentPolicyTrafficConfigEl>>,
}
