use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct RolesanywhereTrustAnchorData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_settings: Option<Vec<RolesanywhereTrustAnchorNotificationSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Vec<RolesanywhereTrustAnchorSourceEl>>,
    dynamic: RolesanywhereTrustAnchorDynamic,
}

struct RolesanywhereTrustAnchor_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RolesanywhereTrustAnchorData>,
}

#[derive(Clone)]
pub struct RolesanywhereTrustAnchor(Rc<RolesanywhereTrustAnchor_>);

impl RolesanywhereTrustAnchor {
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

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
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

    #[doc = "Set the field `notification_settings`.\n"]
    pub fn set_notification_settings(
        self,
        v: impl Into<BlockAssignable<RolesanywhereTrustAnchorNotificationSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(self, v: impl Into<BlockAssignable<RolesanywhereTrustAnchorSourceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<RolesanywhereTrustAnchorSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

impl Referable for RolesanywhereTrustAnchor {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for RolesanywhereTrustAnchor { }

impl ToListMappable for RolesanywhereTrustAnchor {
    type O = ListRef<RolesanywhereTrustAnchorRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for RolesanywhereTrustAnchor_ {
    fn extract_resource_type(&self) -> String {
        "aws_rolesanywhere_trust_anchor".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRolesanywhereTrustAnchor {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildRolesanywhereTrustAnchor {
    pub fn build(self, stack: &mut Stack) -> RolesanywhereTrustAnchor {
        let out = RolesanywhereTrustAnchor(Rc::new(RolesanywhereTrustAnchor_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RolesanywhereTrustAnchorData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                notification_settings: core::default::Default::default(),
                source: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RolesanywhereTrustAnchorRef {
    shared: StackShared,
    base: String,
}

impl Ref for RolesanywhereTrustAnchorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl RolesanywhereTrustAnchorRef {
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
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<RolesanywhereTrustAnchorSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RolesanywhereTrustAnchorNotificationSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threshold: Option<PrimField<f64>>,
}

impl RolesanywhereTrustAnchorNotificationSettingsEl {
    #[doc = "Set the field `channel`.\n"]
    pub fn set_channel(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.channel = Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `event`.\n"]
    pub fn set_event(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.event = Some(v.into());
        self
    }

    #[doc = "Set the field `threshold`.\n"]
    pub fn set_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.threshold = Some(v.into());
        self
    }
}

impl ToListMappable for RolesanywhereTrustAnchorNotificationSettingsEl {
    type O = BlockAssignable<RolesanywhereTrustAnchorNotificationSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRolesanywhereTrustAnchorNotificationSettingsEl {}

impl BuildRolesanywhereTrustAnchorNotificationSettingsEl {
    pub fn build(self) -> RolesanywhereTrustAnchorNotificationSettingsEl {
        RolesanywhereTrustAnchorNotificationSettingsEl {
            channel: core::default::Default::default(),
            enabled: core::default::Default::default(),
            event: core::default::Default::default(),
            threshold: core::default::Default::default(),
        }
    }
}

pub struct RolesanywhereTrustAnchorNotificationSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RolesanywhereTrustAnchorNotificationSettingsElRef {
    fn new(shared: StackShared, base: String) -> RolesanywhereTrustAnchorNotificationSettingsElRef {
        RolesanywhereTrustAnchorNotificationSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RolesanywhereTrustAnchorNotificationSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `channel` after provisioning.\n"]
    pub fn channel(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }

    #[doc = "Get a reference to the value of field `configured_by` after provisioning.\n"]
    pub fn configured_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configured_by", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `event` after provisioning.\n"]
    pub fn event(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event", self.base))
    }

    #[doc = "Get a reference to the value of field `threshold` after provisioning.\n"]
    pub fn threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct RolesanywhereTrustAnchorSourceElSourceDataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    acm_pca_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x509_certificate_data: Option<PrimField<String>>,
}

impl RolesanywhereTrustAnchorSourceElSourceDataEl {
    #[doc = "Set the field `acm_pca_arn`.\n"]
    pub fn set_acm_pca_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.acm_pca_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `x509_certificate_data`.\n"]
    pub fn set_x509_certificate_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.x509_certificate_data = Some(v.into());
        self
    }
}

impl ToListMappable for RolesanywhereTrustAnchorSourceElSourceDataEl {
    type O = BlockAssignable<RolesanywhereTrustAnchorSourceElSourceDataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRolesanywhereTrustAnchorSourceElSourceDataEl {}

impl BuildRolesanywhereTrustAnchorSourceElSourceDataEl {
    pub fn build(self) -> RolesanywhereTrustAnchorSourceElSourceDataEl {
        RolesanywhereTrustAnchorSourceElSourceDataEl {
            acm_pca_arn: core::default::Default::default(),
            x509_certificate_data: core::default::Default::default(),
        }
    }
}

pub struct RolesanywhereTrustAnchorSourceElSourceDataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RolesanywhereTrustAnchorSourceElSourceDataElRef {
    fn new(shared: StackShared, base: String) -> RolesanywhereTrustAnchorSourceElSourceDataElRef {
        RolesanywhereTrustAnchorSourceElSourceDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RolesanywhereTrustAnchorSourceElSourceDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acm_pca_arn` after provisioning.\n"]
    pub fn acm_pca_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.acm_pca_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `x509_certificate_data` after provisioning.\n"]
    pub fn x509_certificate_data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.x509_certificate_data", self.base))
    }
}

#[derive(Serialize, Default)]
struct RolesanywhereTrustAnchorSourceElDynamic {
    source_data: Option<DynamicBlock<RolesanywhereTrustAnchorSourceElSourceDataEl>>,
}

#[derive(Serialize)]
pub struct RolesanywhereTrustAnchorSourceEl {
    source_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_data: Option<Vec<RolesanywhereTrustAnchorSourceElSourceDataEl>>,
    dynamic: RolesanywhereTrustAnchorSourceElDynamic,
}

impl RolesanywhereTrustAnchorSourceEl {
    #[doc = "Set the field `source_data`.\n"]
    pub fn set_source_data(
        mut self,
        v: impl Into<BlockAssignable<RolesanywhereTrustAnchorSourceElSourceDataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_data = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_data = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for RolesanywhereTrustAnchorSourceEl {
    type O = BlockAssignable<RolesanywhereTrustAnchorSourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRolesanywhereTrustAnchorSourceEl {
    #[doc = ""]
    pub source_type: PrimField<String>,
}

impl BuildRolesanywhereTrustAnchorSourceEl {
    pub fn build(self) -> RolesanywhereTrustAnchorSourceEl {
        RolesanywhereTrustAnchorSourceEl {
            source_type: self.source_type,
            source_data: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct RolesanywhereTrustAnchorSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RolesanywhereTrustAnchorSourceElRef {
    fn new(shared: StackShared, base: String) -> RolesanywhereTrustAnchorSourceElRef {
        RolesanywhereTrustAnchorSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RolesanywhereTrustAnchorSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_type` after provisioning.\n"]
    pub fn source_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_type", self.base))
    }

    #[doc = "Get a reference to the value of field `source_data` after provisioning.\n"]
    pub fn source_data(&self) -> ListRef<RolesanywhereTrustAnchorSourceElSourceDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_data", self.base))
    }
}

#[derive(Serialize, Default)]
struct RolesanywhereTrustAnchorDynamic {
    notification_settings: Option<DynamicBlock<RolesanywhereTrustAnchorNotificationSettingsEl>>,
    source: Option<DynamicBlock<RolesanywhereTrustAnchorSourceEl>>,
}
