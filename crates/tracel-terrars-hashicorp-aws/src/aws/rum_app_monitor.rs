use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct RumAppMonitorData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cw_log_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_list: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_monitor_configuration: Option<Vec<RumAppMonitorAppMonitorConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_events: Option<Vec<RumAppMonitorCustomEventsEl>>,
    dynamic: RumAppMonitorDynamic,
}

struct RumAppMonitor_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RumAppMonitorData>,
}

#[derive(Clone)]
pub struct RumAppMonitor(Rc<RumAppMonitor_>);

impl RumAppMonitor {
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

    #[doc = "Set the field `cw_log_enabled`.\n"]
    pub fn set_cw_log_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().cw_log_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().domain = Some(v.into());
        self
    }

    #[doc = "Set the field `domain_list`.\n"]
    pub fn set_domain_list(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().domain_list = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
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

    #[doc = "Set the field `app_monitor_configuration`.\n"]
    pub fn set_app_monitor_configuration(
        self,
        v: impl Into<BlockAssignable<RumAppMonitorAppMonitorConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().app_monitor_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.app_monitor_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_events`.\n"]
    pub fn set_custom_events(self, v: impl Into<BlockAssignable<RumAppMonitorCustomEventsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_events = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_events = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `app_monitor_id` after provisioning.\n"]
    pub fn app_monitor_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_monitor_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cw_log_enabled` after provisioning.\n"]
    pub fn cw_log_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cw_log_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cw_log_group` after provisioning.\n"]
    pub fn cw_log_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cw_log_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_list` after provisioning.\n"]
    pub fn domain_list(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.domain_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_monitor_configuration` after provisioning.\n"]
    pub fn app_monitor_configuration(&self) -> ListRef<RumAppMonitorAppMonitorConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.app_monitor_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_events` after provisioning.\n"]
    pub fn custom_events(&self) -> ListRef<RumAppMonitorCustomEventsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_events", self.extract_ref()))
    }
}

impl Referable for RumAppMonitor {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for RumAppMonitor { }

impl ToListMappable for RumAppMonitor {
    type O = ListRef<RumAppMonitorRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for RumAppMonitor_ {
    fn extract_resource_type(&self) -> String {
        "aws_rum_app_monitor".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRumAppMonitor {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildRumAppMonitor {
    pub fn build(self, stack: &mut Stack) -> RumAppMonitor {
        let out = RumAppMonitor(Rc::new(RumAppMonitor_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RumAppMonitorData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cw_log_enabled: core::default::Default::default(),
                domain: core::default::Default::default(),
                domain_list: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                app_monitor_configuration: core::default::Default::default(),
                custom_events: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RumAppMonitorRef {
    shared: StackShared,
    base: String,
}

impl Ref for RumAppMonitorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl RumAppMonitorRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_monitor_id` after provisioning.\n"]
    pub fn app_monitor_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_monitor_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cw_log_enabled` after provisioning.\n"]
    pub fn cw_log_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cw_log_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cw_log_group` after provisioning.\n"]
    pub fn cw_log_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cw_log_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_list` after provisioning.\n"]
    pub fn domain_list(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.domain_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_monitor_configuration` after provisioning.\n"]
    pub fn app_monitor_configuration(&self) -> ListRef<RumAppMonitorAppMonitorConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.app_monitor_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_events` after provisioning.\n"]
    pub fn custom_events(&self) -> ListRef<RumAppMonitorCustomEventsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_events", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RumAppMonitorAppMonitorConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_cookies: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_xray: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_pages: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    favorite_pages: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_pool_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    included_pages: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_sample_rate: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    telemetries: Option<SetField<PrimField<String>>>,
}

impl RumAppMonitorAppMonitorConfigurationEl {
    #[doc = "Set the field `allow_cookies`.\n"]
    pub fn set_allow_cookies(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_cookies = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_xray`.\n"]
    pub fn set_enable_xray(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_xray = Some(v.into());
        self
    }

    #[doc = "Set the field `excluded_pages`.\n"]
    pub fn set_excluded_pages(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.excluded_pages = Some(v.into());
        self
    }

    #[doc = "Set the field `favorite_pages`.\n"]
    pub fn set_favorite_pages(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.favorite_pages = Some(v.into());
        self
    }

    #[doc = "Set the field `guest_role_arn`.\n"]
    pub fn set_guest_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.guest_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `identity_pool_id`.\n"]
    pub fn set_identity_pool_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_pool_id = Some(v.into());
        self
    }

    #[doc = "Set the field `included_pages`.\n"]
    pub fn set_included_pages(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.included_pages = Some(v.into());
        self
    }

    #[doc = "Set the field `session_sample_rate`.\n"]
    pub fn set_session_sample_rate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.session_sample_rate = Some(v.into());
        self
    }

    #[doc = "Set the field `telemetries`.\n"]
    pub fn set_telemetries(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.telemetries = Some(v.into());
        self
    }
}

impl ToListMappable for RumAppMonitorAppMonitorConfigurationEl {
    type O = BlockAssignable<RumAppMonitorAppMonitorConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRumAppMonitorAppMonitorConfigurationEl {}

impl BuildRumAppMonitorAppMonitorConfigurationEl {
    pub fn build(self) -> RumAppMonitorAppMonitorConfigurationEl {
        RumAppMonitorAppMonitorConfigurationEl {
            allow_cookies: core::default::Default::default(),
            enable_xray: core::default::Default::default(),
            excluded_pages: core::default::Default::default(),
            favorite_pages: core::default::Default::default(),
            guest_role_arn: core::default::Default::default(),
            identity_pool_id: core::default::Default::default(),
            included_pages: core::default::Default::default(),
            session_sample_rate: core::default::Default::default(),
            telemetries: core::default::Default::default(),
        }
    }
}

pub struct RumAppMonitorAppMonitorConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RumAppMonitorAppMonitorConfigurationElRef {
    fn new(shared: StackShared, base: String) -> RumAppMonitorAppMonitorConfigurationElRef {
        RumAppMonitorAppMonitorConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RumAppMonitorAppMonitorConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_cookies` after provisioning.\n"]
    pub fn allow_cookies(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_cookies", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_xray` after provisioning.\n"]
    pub fn enable_xray(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_xray", self.base))
    }

    #[doc = "Get a reference to the value of field `excluded_pages` after provisioning.\n"]
    pub fn excluded_pages(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.excluded_pages", self.base))
    }

    #[doc = "Get a reference to the value of field `favorite_pages` after provisioning.\n"]
    pub fn favorite_pages(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.favorite_pages", self.base))
    }

    #[doc = "Get a reference to the value of field `guest_role_arn` after provisioning.\n"]
    pub fn guest_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.guest_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_pool_id` after provisioning.\n"]
    pub fn identity_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_pool_id", self.base))
    }

    #[doc = "Get a reference to the value of field `included_pages` after provisioning.\n"]
    pub fn included_pages(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.included_pages", self.base))
    }

    #[doc = "Get a reference to the value of field `session_sample_rate` after provisioning.\n"]
    pub fn session_sample_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_sample_rate", self.base))
    }

    #[doc = "Get a reference to the value of field `telemetries` after provisioning.\n"]
    pub fn telemetries(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.telemetries", self.base))
    }
}

#[derive(Serialize)]
pub struct RumAppMonitorCustomEventsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl RumAppMonitorCustomEventsEl {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for RumAppMonitorCustomEventsEl {
    type O = BlockAssignable<RumAppMonitorCustomEventsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRumAppMonitorCustomEventsEl {}

impl BuildRumAppMonitorCustomEventsEl {
    pub fn build(self) -> RumAppMonitorCustomEventsEl {
        RumAppMonitorCustomEventsEl { status: core::default::Default::default() }
    }
}

pub struct RumAppMonitorCustomEventsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RumAppMonitorCustomEventsElRef {
    fn new(shared: StackShared, base: String) -> RumAppMonitorCustomEventsElRef {
        RumAppMonitorCustomEventsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RumAppMonitorCustomEventsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize, Default)]
struct RumAppMonitorDynamic {
    app_monitor_configuration: Option<DynamicBlock<RumAppMonitorAppMonitorConfigurationEl>>,
    custom_events: Option<DynamicBlock<RumAppMonitorCustomEventsEl>>,
}
