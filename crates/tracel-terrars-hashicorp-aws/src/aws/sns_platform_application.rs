use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SnsPlatformApplicationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apple_platform_bundle_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apple_platform_team_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_delivery_failure_topic_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_endpoint_created_topic_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_endpoint_deleted_topic_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_endpoint_updated_topic_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_feedback_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    platform: PrimField<String>,
    platform_credential: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform_principal: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_feedback_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_feedback_sample_rate: Option<PrimField<String>>,
}

struct SnsPlatformApplication_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SnsPlatformApplicationData>,
}

#[derive(Clone)]
pub struct SnsPlatformApplication(Rc<SnsPlatformApplication_>);

impl SnsPlatformApplication {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `apple_platform_bundle_id`.\n"]
    pub fn set_apple_platform_bundle_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().apple_platform_bundle_id = Some(v.into());
        self
    }

    #[doc = "Set the field `apple_platform_team_id`.\n"]
    pub fn set_apple_platform_team_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().apple_platform_team_id = Some(v.into());
        self
    }

    #[doc = "Set the field `event_delivery_failure_topic_arn`.\n"]
    pub fn set_event_delivery_failure_topic_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().event_delivery_failure_topic_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `event_endpoint_created_topic_arn`.\n"]
    pub fn set_event_endpoint_created_topic_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().event_endpoint_created_topic_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `event_endpoint_deleted_topic_arn`.\n"]
    pub fn set_event_endpoint_deleted_topic_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().event_endpoint_deleted_topic_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `event_endpoint_updated_topic_arn`.\n"]
    pub fn set_event_endpoint_updated_topic_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().event_endpoint_updated_topic_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `failure_feedback_role_arn`.\n"]
    pub fn set_failure_feedback_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().failure_feedback_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `platform_principal`.\n"]
    pub fn set_platform_principal(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().platform_principal = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `success_feedback_role_arn`.\n"]
    pub fn set_success_feedback_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().success_feedback_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `success_feedback_sample_rate`.\n"]
    pub fn set_success_feedback_sample_rate(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().success_feedback_sample_rate = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `apple_platform_bundle_id` after provisioning.\n"]
    pub fn apple_platform_bundle_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.apple_platform_bundle_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `apple_platform_team_id` after provisioning.\n"]
    pub fn apple_platform_team_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.apple_platform_team_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_delivery_failure_topic_arn` after provisioning.\n"]
    pub fn event_delivery_failure_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_delivery_failure_topic_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_endpoint_created_topic_arn` after provisioning.\n"]
    pub fn event_endpoint_created_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_endpoint_created_topic_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_endpoint_deleted_topic_arn` after provisioning.\n"]
    pub fn event_endpoint_deleted_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_endpoint_deleted_topic_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_endpoint_updated_topic_arn` after provisioning.\n"]
    pub fn event_endpoint_updated_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_endpoint_updated_topic_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_feedback_role_arn` after provisioning.\n"]
    pub fn failure_feedback_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.failure_feedback_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_credential` after provisioning.\n"]
    pub fn platform_credential(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_credential", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_principal` after provisioning.\n"]
    pub fn platform_principal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_principal", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `success_feedback_role_arn` after provisioning.\n"]
    pub fn success_feedback_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.success_feedback_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `success_feedback_sample_rate` after provisioning.\n"]
    pub fn success_feedback_sample_rate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.success_feedback_sample_rate", self.extract_ref()))
    }
}

impl Referable for SnsPlatformApplication {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for SnsPlatformApplication { }

impl ToListMappable for SnsPlatformApplication {
    type O = ListRef<SnsPlatformApplicationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SnsPlatformApplication_ {
    fn extract_resource_type(&self) -> String {
        "aws_sns_platform_application".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSnsPlatformApplication {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub platform: PrimField<String>,
    #[doc = ""]
    pub platform_credential: PrimField<String>,
}

impl BuildSnsPlatformApplication {
    pub fn build(self, stack: &mut Stack) -> SnsPlatformApplication {
        let out = SnsPlatformApplication(Rc::new(SnsPlatformApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SnsPlatformApplicationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                apple_platform_bundle_id: core::default::Default::default(),
                apple_platform_team_id: core::default::Default::default(),
                event_delivery_failure_topic_arn: core::default::Default::default(),
                event_endpoint_created_topic_arn: core::default::Default::default(),
                event_endpoint_deleted_topic_arn: core::default::Default::default(),
                event_endpoint_updated_topic_arn: core::default::Default::default(),
                failure_feedback_role_arn: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                platform: self.platform,
                platform_credential: self.platform_credential,
                platform_principal: core::default::Default::default(),
                region: core::default::Default::default(),
                success_feedback_role_arn: core::default::Default::default(),
                success_feedback_sample_rate: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SnsPlatformApplicationRef {
    shared: StackShared,
    base: String,
}

impl Ref for SnsPlatformApplicationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl SnsPlatformApplicationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `apple_platform_bundle_id` after provisioning.\n"]
    pub fn apple_platform_bundle_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.apple_platform_bundle_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `apple_platform_team_id` after provisioning.\n"]
    pub fn apple_platform_team_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.apple_platform_team_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_delivery_failure_topic_arn` after provisioning.\n"]
    pub fn event_delivery_failure_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_delivery_failure_topic_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_endpoint_created_topic_arn` after provisioning.\n"]
    pub fn event_endpoint_created_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_endpoint_created_topic_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_endpoint_deleted_topic_arn` after provisioning.\n"]
    pub fn event_endpoint_deleted_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_endpoint_deleted_topic_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_endpoint_updated_topic_arn` after provisioning.\n"]
    pub fn event_endpoint_updated_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_endpoint_updated_topic_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_feedback_role_arn` after provisioning.\n"]
    pub fn failure_feedback_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.failure_feedback_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_credential` after provisioning.\n"]
    pub fn platform_credential(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_credential", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_principal` after provisioning.\n"]
    pub fn platform_principal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_principal", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `success_feedback_role_arn` after provisioning.\n"]
    pub fn success_feedback_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.success_feedback_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `success_feedback_sample_rate` after provisioning.\n"]
    pub fn success_feedback_sample_rate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.success_feedback_sample_rate", self.extract_ref()))
    }
}
