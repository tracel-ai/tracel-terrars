use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct PinpointAppData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    campaign_hook: Option<Vec<PinpointAppCampaignHookEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limits: Option<Vec<PinpointAppLimitsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quiet_time: Option<Vec<PinpointAppQuietTimeEl>>,
    dynamic: PinpointAppDynamic,
}

struct PinpointApp_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PinpointAppData>,
}

#[derive(Clone)]
pub struct PinpointApp(Rc<PinpointApp_>);

impl PinpointApp {
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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = Some(v.into());
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

    #[doc = "Set the field `campaign_hook`.\n"]
    pub fn set_campaign_hook(
        self,
        v: impl Into<BlockAssignable<PinpointAppCampaignHookEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().campaign_hook = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.campaign_hook = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(self, v: impl Into<BlockAssignable<PinpointAppLimitsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().limits = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.limits = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `quiet_time`.\n"]
    pub fn set_quiet_time(self, v: impl Into<BlockAssignable<PinpointAppQuietTimeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().quiet_time = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.quiet_time = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `application_id` after provisioning.\n"]
    pub fn application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `campaign_hook` after provisioning.\n"]
    pub fn campaign_hook(&self) -> ListRef<PinpointAppCampaignHookElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.campaign_hook", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> ListRef<PinpointAppLimitsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.limits", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `quiet_time` after provisioning.\n"]
    pub fn quiet_time(&self) -> ListRef<PinpointAppQuietTimeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.quiet_time", self.extract_ref()),
        )
    }
}

impl Referable for PinpointApp {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for PinpointApp {}

impl ToListMappable for PinpointApp {
    type O = ListRef<PinpointAppRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for PinpointApp_ {
    fn extract_resource_type(&self) -> String {
        "aws_pinpoint_app".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPinpointApp {
    pub tf_id: String,
}

impl BuildPinpointApp {
    pub fn build(self, stack: &mut Stack) -> PinpointApp {
        let out = PinpointApp(Rc::new(PinpointApp_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(PinpointAppData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                campaign_hook: core::default::Default::default(),
                limits: core::default::Default::default(),
                quiet_time: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PinpointAppRef {
    shared: StackShared,
    base: String,
}

impl Ref for PinpointAppRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl PinpointAppRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_id` after provisioning.\n"]
    pub fn application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `campaign_hook` after provisioning.\n"]
    pub fn campaign_hook(&self) -> ListRef<PinpointAppCampaignHookElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.campaign_hook", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> ListRef<PinpointAppLimitsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.limits", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `quiet_time` after provisioning.\n"]
    pub fn quiet_time(&self) -> ListRef<PinpointAppQuietTimeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.quiet_time", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct PinpointAppCampaignHookEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_function_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_url: Option<PrimField<String>>,
}

impl PinpointAppCampaignHookEl {
    #[doc = "Set the field `lambda_function_name`.\n"]
    pub fn set_lambda_function_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lambda_function_name = Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }

    #[doc = "Set the field `web_url`.\n"]
    pub fn set_web_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.web_url = Some(v.into());
        self
    }
}

impl ToListMappable for PinpointAppCampaignHookEl {
    type O = BlockAssignable<PinpointAppCampaignHookEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPinpointAppCampaignHookEl {}

impl BuildPinpointAppCampaignHookEl {
    pub fn build(self) -> PinpointAppCampaignHookEl {
        PinpointAppCampaignHookEl {
            lambda_function_name: core::default::Default::default(),
            mode: core::default::Default::default(),
            web_url: core::default::Default::default(),
        }
    }
}

pub struct PinpointAppCampaignHookElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PinpointAppCampaignHookElRef {
    fn new(shared: StackShared, base: String) -> PinpointAppCampaignHookElRef {
        PinpointAppCampaignHookElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PinpointAppCampaignHookElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda_function_name` after provisioning.\n"]
    pub fn lambda_function_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lambda_function_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `web_url` after provisioning.\n"]
    pub fn web_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_url", self.base))
    }
}

#[derive(Serialize)]
pub struct PinpointAppLimitsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    daily: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_duration: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    messages_per_second: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total: Option<PrimField<f64>>,
}

impl PinpointAppLimitsEl {
    #[doc = "Set the field `daily`.\n"]
    pub fn set_daily(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.daily = Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_duration`.\n"]
    pub fn set_maximum_duration(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_duration = Some(v.into());
        self
    }

    #[doc = "Set the field `messages_per_second`.\n"]
    pub fn set_messages_per_second(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.messages_per_second = Some(v.into());
        self
    }

    #[doc = "Set the field `total`.\n"]
    pub fn set_total(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.total = Some(v.into());
        self
    }
}

impl ToListMappable for PinpointAppLimitsEl {
    type O = BlockAssignable<PinpointAppLimitsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPinpointAppLimitsEl {}

impl BuildPinpointAppLimitsEl {
    pub fn build(self) -> PinpointAppLimitsEl {
        PinpointAppLimitsEl {
            daily: core::default::Default::default(),
            maximum_duration: core::default::Default::default(),
            messages_per_second: core::default::Default::default(),
            total: core::default::Default::default(),
        }
    }
}

pub struct PinpointAppLimitsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PinpointAppLimitsElRef {
    fn new(shared: StackShared, base: String) -> PinpointAppLimitsElRef {
        PinpointAppLimitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PinpointAppLimitsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `daily` after provisioning.\n"]
    pub fn daily(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.daily", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_duration` after provisioning.\n"]
    pub fn maximum_duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_duration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `messages_per_second` after provisioning.\n"]
    pub fn messages_per_second(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.messages_per_second", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `total` after provisioning.\n"]
    pub fn total(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.total", self.base))
    }
}

#[derive(Serialize)]
pub struct PinpointAppQuietTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<PrimField<String>>,
}

impl PinpointAppQuietTimeEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end = Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start = Some(v.into());
        self
    }
}

impl ToListMappable for PinpointAppQuietTimeEl {
    type O = BlockAssignable<PinpointAppQuietTimeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPinpointAppQuietTimeEl {}

impl BuildPinpointAppQuietTimeEl {
    pub fn build(self) -> PinpointAppQuietTimeEl {
        PinpointAppQuietTimeEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
        }
    }
}

pub struct PinpointAppQuietTimeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PinpointAppQuietTimeElRef {
    fn new(shared: StackShared, base: String) -> PinpointAppQuietTimeElRef {
        PinpointAppQuietTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PinpointAppQuietTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
}

#[derive(Serialize, Default)]
struct PinpointAppDynamic {
    campaign_hook: Option<DynamicBlock<PinpointAppCampaignHookEl>>,
    limits: Option<DynamicBlock<PinpointAppLimitsEl>>,
    quiet_time: Option<DynamicBlock<PinpointAppQuietTimeEl>>,
}
