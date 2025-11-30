use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct CloudfrontMonitoringSubscriptionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    distribution_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitoring_subscription: Option<Vec<CloudfrontMonitoringSubscriptionMonitoringSubscriptionEl>>,
    dynamic: CloudfrontMonitoringSubscriptionDynamic,
}

struct CloudfrontMonitoringSubscription_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudfrontMonitoringSubscriptionData>,
}

#[derive(Clone)]
pub struct CloudfrontMonitoringSubscription(Rc<CloudfrontMonitoringSubscription_>);

impl CloudfrontMonitoringSubscription {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `monitoring_subscription`.\n"]
    pub fn set_monitoring_subscription(
        self,
        v: impl Into<BlockAssignable<CloudfrontMonitoringSubscriptionMonitoringSubscriptionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().monitoring_subscription = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.monitoring_subscription = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `distribution_id` after provisioning.\n"]
    pub fn distribution_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.distribution_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_subscription` after provisioning.\n"]
    pub fn monitoring_subscription(
        &self,
    ) -> ListRef<CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.monitoring_subscription", self.extract_ref()),
        )
    }
}

impl Referable for CloudfrontMonitoringSubscription {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for CloudfrontMonitoringSubscription {}

impl ToListMappable for CloudfrontMonitoringSubscription {
    type O = ListRef<CloudfrontMonitoringSubscriptionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CloudfrontMonitoringSubscription_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudfront_monitoring_subscription".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudfrontMonitoringSubscription {
    pub tf_id: String,
    #[doc = ""]
    pub distribution_id: PrimField<String>,
}

impl BuildCloudfrontMonitoringSubscription {
    pub fn build(self, stack: &mut Stack) -> CloudfrontMonitoringSubscription {
        let out = CloudfrontMonitoringSubscription(Rc::new(CloudfrontMonitoringSubscription_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudfrontMonitoringSubscriptionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                distribution_id: self.distribution_id,
                id: core::default::Default::default(),
                monitoring_subscription: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudfrontMonitoringSubscriptionRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontMonitoringSubscriptionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl CloudfrontMonitoringSubscriptionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `distribution_id` after provisioning.\n"]
    pub fn distribution_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.distribution_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_subscription` after provisioning.\n"]
    pub fn monitoring_subscription(
        &self,
    ) -> ListRef<CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.monitoring_subscription", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigEl
{
    realtime_metrics_subscription_status: PrimField<String>,
}

impl CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigEl {}

impl ToListMappable
    for CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigEl
{
    type O = BlockAssignable<
        CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigEl
{
    #[doc = ""]
    pub realtime_metrics_subscription_status: PrimField<String>,
}

impl
    BuildCloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigEl
{
    pub fn build(
        self,
    ) -> CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigEl
    {
        CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigEl {
            realtime_metrics_subscription_status: self.realtime_metrics_subscription_status,
        }
    }
}

pub struct CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigElRef {
        CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl
    CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `realtime_metrics_subscription_status` after provisioning.\n"]
    pub fn realtime_metrics_subscription_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.realtime_metrics_subscription_status", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct CloudfrontMonitoringSubscriptionMonitoringSubscriptionElDynamic {
    realtime_metrics_subscription_config: Option<
        DynamicBlock<CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct CloudfrontMonitoringSubscriptionMonitoringSubscriptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    realtime_metrics_subscription_config: Option<
        Vec<CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigEl>,
    >,
    dynamic: CloudfrontMonitoringSubscriptionMonitoringSubscriptionElDynamic,
}

impl CloudfrontMonitoringSubscriptionMonitoringSubscriptionEl {
    #[doc = "Set the field `realtime_metrics_subscription_config`.\n"]
    pub fn set_realtime_metrics_subscription_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.realtime_metrics_subscription_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.realtime_metrics_subscription_config = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CloudfrontMonitoringSubscriptionMonitoringSubscriptionEl {
    type O = BlockAssignable<CloudfrontMonitoringSubscriptionMonitoringSubscriptionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMonitoringSubscriptionMonitoringSubscriptionEl {}

impl BuildCloudfrontMonitoringSubscriptionMonitoringSubscriptionEl {
    pub fn build(self) -> CloudfrontMonitoringSubscriptionMonitoringSubscriptionEl {
        CloudfrontMonitoringSubscriptionMonitoringSubscriptionEl {
            realtime_metrics_subscription_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRef {
        CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `realtime_metrics_subscription_config` after provisioning.\n"]
    pub fn realtime_metrics_subscription_config(
        &self,
    ) -> ListRef<CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.realtime_metrics_subscription_config", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct CloudfrontMonitoringSubscriptionDynamic {
    monitoring_subscription:
        Option<DynamicBlock<CloudfrontMonitoringSubscriptionMonitoringSubscriptionEl>>,
}
