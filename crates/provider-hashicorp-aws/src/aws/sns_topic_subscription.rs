use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct SnsTopicSubscriptionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confirmation_timeout_in_minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_policy: Option<PrimField<String>>,
    endpoint: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_auto_confirms: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_policy_scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    protocol: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_message_delivery: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redrive_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replay_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_role_arn: Option<PrimField<String>>,
    topic_arn: PrimField<String>,
}

struct SnsTopicSubscription_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SnsTopicSubscriptionData>,
}

#[derive(Clone)]
pub struct SnsTopicSubscription(Rc<SnsTopicSubscription_>);

impl SnsTopicSubscription {
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

    #[doc = "Set the field `confirmation_timeout_in_minutes`.\n"]
    pub fn set_confirmation_timeout_in_minutes(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().confirmation_timeout_in_minutes = Some(v.into());
        self
    }

    #[doc = "Set the field `delivery_policy`.\n"]
    pub fn set_delivery_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().delivery_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_auto_confirms`.\n"]
    pub fn set_endpoint_auto_confirms(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().endpoint_auto_confirms = Some(v.into());
        self
    }

    #[doc = "Set the field `filter_policy`.\n"]
    pub fn set_filter_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `filter_policy_scope`.\n"]
    pub fn set_filter_policy_scope(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter_policy_scope = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `raw_message_delivery`.\n"]
    pub fn set_raw_message_delivery(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().raw_message_delivery = Some(v.into());
        self
    }

    #[doc = "Set the field `redrive_policy`.\n"]
    pub fn set_redrive_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().redrive_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `replay_policy`.\n"]
    pub fn set_replay_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().replay_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `subscription_role_arn`.\n"]
    pub fn set_subscription_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().subscription_role_arn = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `confirmation_timeout_in_minutes` after provisioning.\n"]
    pub fn confirmation_timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.confirmation_timeout_in_minutes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `confirmation_was_authenticated` after provisioning.\n"]
    pub fn confirmation_was_authenticated(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.confirmation_was_authenticated", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `delivery_policy` after provisioning.\n"]
    pub fn delivery_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint_auto_confirms` after provisioning.\n"]
    pub fn endpoint_auto_confirms(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_auto_confirms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `filter_policy` after provisioning.\n"]
    pub fn filter_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `filter_policy_scope` after provisioning.\n"]
    pub fn filter_policy_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter_policy_scope", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `pending_confirmation` after provisioning.\n"]
    pub fn pending_confirmation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pending_confirmation", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `raw_message_delivery` after provisioning.\n"]
    pub fn raw_message_delivery(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.raw_message_delivery", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `redrive_policy` after provisioning.\n"]
    pub fn redrive_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redrive_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `replay_policy` after provisioning.\n"]
    pub fn replay_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replay_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subscription_role_arn` after provisioning.\n"]
    pub fn subscription_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscription_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.topic_arn", self.extract_ref()),
        )
    }
}

impl Referable for SnsTopicSubscription {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for SnsTopicSubscription {}

impl ToListMappable for SnsTopicSubscription {
    type O = ListRef<SnsTopicSubscriptionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SnsTopicSubscription_ {
    fn extract_resource_type(&self) -> String {
        "aws_sns_topic_subscription".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSnsTopicSubscription {
    pub tf_id: String,
    #[doc = ""]
    pub endpoint: PrimField<String>,
    #[doc = ""]
    pub protocol: PrimField<String>,
    #[doc = ""]
    pub topic_arn: PrimField<String>,
}

impl BuildSnsTopicSubscription {
    pub fn build(self, stack: &mut Stack) -> SnsTopicSubscription {
        let out = SnsTopicSubscription(Rc::new(SnsTopicSubscription_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SnsTopicSubscriptionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                confirmation_timeout_in_minutes: core::default::Default::default(),
                delivery_policy: core::default::Default::default(),
                endpoint: self.endpoint,
                endpoint_auto_confirms: core::default::Default::default(),
                filter_policy: core::default::Default::default(),
                filter_policy_scope: core::default::Default::default(),
                id: core::default::Default::default(),
                protocol: self.protocol,
                raw_message_delivery: core::default::Default::default(),
                redrive_policy: core::default::Default::default(),
                region: core::default::Default::default(),
                replay_policy: core::default::Default::default(),
                subscription_role_arn: core::default::Default::default(),
                topic_arn: self.topic_arn,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SnsTopicSubscriptionRef {
    shared: StackShared,
    base: String,
}

impl Ref for SnsTopicSubscriptionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl SnsTopicSubscriptionRef {
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

    #[doc = "Get a reference to the value of field `confirmation_timeout_in_minutes` after provisioning.\n"]
    pub fn confirmation_timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.confirmation_timeout_in_minutes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `confirmation_was_authenticated` after provisioning.\n"]
    pub fn confirmation_was_authenticated(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.confirmation_was_authenticated", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `delivery_policy` after provisioning.\n"]
    pub fn delivery_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint_auto_confirms` after provisioning.\n"]
    pub fn endpoint_auto_confirms(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_auto_confirms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `filter_policy` after provisioning.\n"]
    pub fn filter_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `filter_policy_scope` after provisioning.\n"]
    pub fn filter_policy_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter_policy_scope", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `pending_confirmation` after provisioning.\n"]
    pub fn pending_confirmation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pending_confirmation", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `raw_message_delivery` after provisioning.\n"]
    pub fn raw_message_delivery(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.raw_message_delivery", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `redrive_policy` after provisioning.\n"]
    pub fn redrive_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redrive_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `replay_policy` after provisioning.\n"]
    pub fn replay_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replay_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subscription_role_arn` after provisioning.\n"]
    pub fn subscription_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscription_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.topic_arn", self.extract_ref()),
        )
    }
}
