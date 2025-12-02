use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SesConfigurationSetData {
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
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reputation_metrics_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sending_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_options: Option<Vec<SesConfigurationSetDeliveryOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tracking_options: Option<Vec<SesConfigurationSetTrackingOptionsEl>>,
    dynamic: SesConfigurationSetDynamic,
}
struct SesConfigurationSet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SesConfigurationSetData>,
}
#[derive(Clone)]
pub struct SesConfigurationSet(Rc<SesConfigurationSet_>);
impl SesConfigurationSet {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `reputation_metrics_enabled`.\n"]
    pub fn set_reputation_metrics_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().reputation_metrics_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `sending_enabled`.\n"]
    pub fn set_sending_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().sending_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `delivery_options`.\n"]
    pub fn set_delivery_options(
        self,
        v: impl Into<BlockAssignable<SesConfigurationSetDeliveryOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().delivery_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.delivery_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tracking_options`.\n"]
    pub fn set_tracking_options(
        self,
        v: impl Into<BlockAssignable<SesConfigurationSetTrackingOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tracking_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tracking_options = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_fresh_start` after provisioning.\n"]
    pub fn last_fresh_start(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_fresh_start", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reputation_metrics_enabled` after provisioning.\n"]
    pub fn reputation_metrics_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reputation_metrics_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sending_enabled` after provisioning.\n"]
    pub fn sending_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sending_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delivery_options` after provisioning.\n"]
    pub fn delivery_options(&self) -> ListRef<SesConfigurationSetDeliveryOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.delivery_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tracking_options` after provisioning.\n"]
    pub fn tracking_options(&self) -> ListRef<SesConfigurationSetTrackingOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tracking_options", self.extract_ref()),
        )
    }
}
impl Referable for SesConfigurationSet {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SesConfigurationSet {}
impl ToListMappable for SesConfigurationSet {
    type O = ListRef<SesConfigurationSetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SesConfigurationSet_ {
    fn extract_resource_type(&self) -> String {
        "aws_ses_configuration_set".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSesConfigurationSet {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildSesConfigurationSet {
    pub fn build(self, stack: &mut Stack) -> SesConfigurationSet {
        let out = SesConfigurationSet(Rc::new(SesConfigurationSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SesConfigurationSetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                reputation_metrics_enabled: core::default::Default::default(),
                sending_enabled: core::default::Default::default(),
                delivery_options: core::default::Default::default(),
                tracking_options: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SesConfigurationSetRef {
    shared: StackShared,
    base: String,
}
impl Ref for SesConfigurationSetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SesConfigurationSetRef {
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_fresh_start` after provisioning.\n"]
    pub fn last_fresh_start(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_fresh_start", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reputation_metrics_enabled` after provisioning.\n"]
    pub fn reputation_metrics_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reputation_metrics_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sending_enabled` after provisioning.\n"]
    pub fn sending_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sending_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delivery_options` after provisioning.\n"]
    pub fn delivery_options(&self) -> ListRef<SesConfigurationSetDeliveryOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.delivery_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tracking_options` after provisioning.\n"]
    pub fn tracking_options(&self) -> ListRef<SesConfigurationSetTrackingOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tracking_options", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SesConfigurationSetDeliveryOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_policy: Option<PrimField<String>>,
}
impl SesConfigurationSetDeliveryOptionsEl {
    #[doc = "Set the field `tls_policy`.\n"]
    pub fn set_tls_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tls_policy = Some(v.into());
        self
    }
}
impl ToListMappable for SesConfigurationSetDeliveryOptionsEl {
    type O = BlockAssignable<SesConfigurationSetDeliveryOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSesConfigurationSetDeliveryOptionsEl {}
impl BuildSesConfigurationSetDeliveryOptionsEl {
    pub fn build(self) -> SesConfigurationSetDeliveryOptionsEl {
        SesConfigurationSetDeliveryOptionsEl {
            tls_policy: core::default::Default::default(),
        }
    }
}
pub struct SesConfigurationSetDeliveryOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SesConfigurationSetDeliveryOptionsElRef {
    fn new(shared: StackShared, base: String) -> SesConfigurationSetDeliveryOptionsElRef {
        SesConfigurationSetDeliveryOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SesConfigurationSetDeliveryOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `tls_policy` after provisioning.\n"]
    pub fn tls_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tls_policy", self.base))
    }
}
#[derive(Serialize)]
pub struct SesConfigurationSetTrackingOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_redirect_domain: Option<PrimField<String>>,
}
impl SesConfigurationSetTrackingOptionsEl {
    #[doc = "Set the field `custom_redirect_domain`.\n"]
    pub fn set_custom_redirect_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.custom_redirect_domain = Some(v.into());
        self
    }
}
impl ToListMappable for SesConfigurationSetTrackingOptionsEl {
    type O = BlockAssignable<SesConfigurationSetTrackingOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSesConfigurationSetTrackingOptionsEl {}
impl BuildSesConfigurationSetTrackingOptionsEl {
    pub fn build(self) -> SesConfigurationSetTrackingOptionsEl {
        SesConfigurationSetTrackingOptionsEl {
            custom_redirect_domain: core::default::Default::default(),
        }
    }
}
pub struct SesConfigurationSetTrackingOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SesConfigurationSetTrackingOptionsElRef {
    fn new(shared: StackShared, base: String) -> SesConfigurationSetTrackingOptionsElRef {
        SesConfigurationSetTrackingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SesConfigurationSetTrackingOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_redirect_domain` after provisioning.\n"]
    pub fn custom_redirect_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_redirect_domain", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SesConfigurationSetDynamic {
    delivery_options: Option<DynamicBlock<SesConfigurationSetDeliveryOptionsEl>>,
    tracking_options: Option<DynamicBlock<SesConfigurationSetTrackingOptionsEl>>,
}
