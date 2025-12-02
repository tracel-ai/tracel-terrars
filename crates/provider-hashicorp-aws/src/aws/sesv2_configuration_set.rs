use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct Sesv2ConfigurationSetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    configuration_set_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_options: Option<Vec<Sesv2ConfigurationSetDeliveryOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reputation_options: Option<Vec<Sesv2ConfigurationSetReputationOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sending_options: Option<Vec<Sesv2ConfigurationSetSendingOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suppression_options: Option<Vec<Sesv2ConfigurationSetSuppressionOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tracking_options: Option<Vec<Sesv2ConfigurationSetTrackingOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vdm_options: Option<Vec<Sesv2ConfigurationSetVdmOptionsEl>>,
    dynamic: Sesv2ConfigurationSetDynamic,
}
struct Sesv2ConfigurationSet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Sesv2ConfigurationSetData>,
}
#[derive(Clone)]
pub struct Sesv2ConfigurationSet(Rc<Sesv2ConfigurationSet_>);
impl Sesv2ConfigurationSet {
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
    #[doc = "Set the field `delivery_options`.\n"]
    pub fn set_delivery_options(
        self,
        v: impl Into<BlockAssignable<Sesv2ConfigurationSetDeliveryOptionsEl>>,
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
    #[doc = "Set the field `reputation_options`.\n"]
    pub fn set_reputation_options(
        self,
        v: impl Into<BlockAssignable<Sesv2ConfigurationSetReputationOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().reputation_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.reputation_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sending_options`.\n"]
    pub fn set_sending_options(
        self,
        v: impl Into<BlockAssignable<Sesv2ConfigurationSetSendingOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sending_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sending_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `suppression_options`.\n"]
    pub fn set_suppression_options(
        self,
        v: impl Into<BlockAssignable<Sesv2ConfigurationSetSuppressionOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().suppression_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.suppression_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tracking_options`.\n"]
    pub fn set_tracking_options(
        self,
        v: impl Into<BlockAssignable<Sesv2ConfigurationSetTrackingOptionsEl>>,
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
    #[doc = "Set the field `vdm_options`.\n"]
    pub fn set_vdm_options(
        self,
        v: impl Into<BlockAssignable<Sesv2ConfigurationSetVdmOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vdm_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vdm_options = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `configuration_set_name` after provisioning.\n"]
    pub fn configuration_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration_set_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
    #[doc = "Get a reference to the value of field `delivery_options` after provisioning.\n"]
    pub fn delivery_options(&self) -> ListRef<Sesv2ConfigurationSetDeliveryOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.delivery_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reputation_options` after provisioning.\n"]
    pub fn reputation_options(&self) -> ListRef<Sesv2ConfigurationSetReputationOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reputation_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sending_options` after provisioning.\n"]
    pub fn sending_options(&self) -> ListRef<Sesv2ConfigurationSetSendingOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sending_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `suppression_options` after provisioning.\n"]
    pub fn suppression_options(&self) -> ListRef<Sesv2ConfigurationSetSuppressionOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.suppression_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tracking_options` after provisioning.\n"]
    pub fn tracking_options(&self) -> ListRef<Sesv2ConfigurationSetTrackingOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tracking_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vdm_options` after provisioning.\n"]
    pub fn vdm_options(&self) -> ListRef<Sesv2ConfigurationSetVdmOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vdm_options", self.extract_ref()),
        )
    }
}
impl Referable for Sesv2ConfigurationSet {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for Sesv2ConfigurationSet {}
impl ToListMappable for Sesv2ConfigurationSet {
    type O = ListRef<Sesv2ConfigurationSetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for Sesv2ConfigurationSet_ {
    fn extract_resource_type(&self) -> String {
        "aws_sesv2_configuration_set".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSesv2ConfigurationSet {
    pub tf_id: String,
    #[doc = ""]
    pub configuration_set_name: PrimField<String>,
}
impl BuildSesv2ConfigurationSet {
    pub fn build(self, stack: &mut Stack) -> Sesv2ConfigurationSet {
        let out = Sesv2ConfigurationSet(Rc::new(Sesv2ConfigurationSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Sesv2ConfigurationSetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                configuration_set_name: self.configuration_set_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                delivery_options: core::default::Default::default(),
                reputation_options: core::default::Default::default(),
                sending_options: core::default::Default::default(),
                suppression_options: core::default::Default::default(),
                tracking_options: core::default::Default::default(),
                vdm_options: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct Sesv2ConfigurationSetRef {
    shared: StackShared,
    base: String,
}
impl Ref for Sesv2ConfigurationSetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl Sesv2ConfigurationSetRef {
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
    #[doc = "Get a reference to the value of field `configuration_set_name` after provisioning.\n"]
    pub fn configuration_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration_set_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
    #[doc = "Get a reference to the value of field `delivery_options` after provisioning.\n"]
    pub fn delivery_options(&self) -> ListRef<Sesv2ConfigurationSetDeliveryOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.delivery_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reputation_options` after provisioning.\n"]
    pub fn reputation_options(&self) -> ListRef<Sesv2ConfigurationSetReputationOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reputation_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sending_options` after provisioning.\n"]
    pub fn sending_options(&self) -> ListRef<Sesv2ConfigurationSetSendingOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sending_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `suppression_options` after provisioning.\n"]
    pub fn suppression_options(&self) -> ListRef<Sesv2ConfigurationSetSuppressionOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.suppression_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tracking_options` after provisioning.\n"]
    pub fn tracking_options(&self) -> ListRef<Sesv2ConfigurationSetTrackingOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tracking_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vdm_options` after provisioning.\n"]
    pub fn vdm_options(&self) -> ListRef<Sesv2ConfigurationSetVdmOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vdm_options", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct Sesv2ConfigurationSetDeliveryOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_delivery_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sending_pool_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_policy: Option<PrimField<String>>,
}
impl Sesv2ConfigurationSetDeliveryOptionsEl {
    #[doc = "Set the field `max_delivery_seconds`.\n"]
    pub fn set_max_delivery_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_delivery_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `sending_pool_name`.\n"]
    pub fn set_sending_pool_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sending_pool_name = Some(v.into());
        self
    }
    #[doc = "Set the field `tls_policy`.\n"]
    pub fn set_tls_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tls_policy = Some(v.into());
        self
    }
}
impl ToListMappable for Sesv2ConfigurationSetDeliveryOptionsEl {
    type O = BlockAssignable<Sesv2ConfigurationSetDeliveryOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSesv2ConfigurationSetDeliveryOptionsEl {}
impl BuildSesv2ConfigurationSetDeliveryOptionsEl {
    pub fn build(self) -> Sesv2ConfigurationSetDeliveryOptionsEl {
        Sesv2ConfigurationSetDeliveryOptionsEl {
            max_delivery_seconds: core::default::Default::default(),
            sending_pool_name: core::default::Default::default(),
            tls_policy: core::default::Default::default(),
        }
    }
}
pub struct Sesv2ConfigurationSetDeliveryOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Sesv2ConfigurationSetDeliveryOptionsElRef {
    fn new(shared: StackShared, base: String) -> Sesv2ConfigurationSetDeliveryOptionsElRef {
        Sesv2ConfigurationSetDeliveryOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Sesv2ConfigurationSetDeliveryOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_delivery_seconds` after provisioning.\n"]
    pub fn max_delivery_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_delivery_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sending_pool_name` after provisioning.\n"]
    pub fn sending_pool_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sending_pool_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tls_policy` after provisioning.\n"]
    pub fn tls_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tls_policy", self.base))
    }
}
#[derive(Serialize)]
pub struct Sesv2ConfigurationSetReputationOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    reputation_metrics_enabled: Option<PrimField<bool>>,
}
impl Sesv2ConfigurationSetReputationOptionsEl {
    #[doc = "Set the field `reputation_metrics_enabled`.\n"]
    pub fn set_reputation_metrics_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.reputation_metrics_enabled = Some(v.into());
        self
    }
}
impl ToListMappable for Sesv2ConfigurationSetReputationOptionsEl {
    type O = BlockAssignable<Sesv2ConfigurationSetReputationOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSesv2ConfigurationSetReputationOptionsEl {}
impl BuildSesv2ConfigurationSetReputationOptionsEl {
    pub fn build(self) -> Sesv2ConfigurationSetReputationOptionsEl {
        Sesv2ConfigurationSetReputationOptionsEl {
            reputation_metrics_enabled: core::default::Default::default(),
        }
    }
}
pub struct Sesv2ConfigurationSetReputationOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Sesv2ConfigurationSetReputationOptionsElRef {
    fn new(shared: StackShared, base: String) -> Sesv2ConfigurationSetReputationOptionsElRef {
        Sesv2ConfigurationSetReputationOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Sesv2ConfigurationSetReputationOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `last_fresh_start` after provisioning.\n"]
    pub fn last_fresh_start(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_fresh_start", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `reputation_metrics_enabled` after provisioning.\n"]
    pub fn reputation_metrics_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reputation_metrics_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Sesv2ConfigurationSetSendingOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    sending_enabled: Option<PrimField<bool>>,
}
impl Sesv2ConfigurationSetSendingOptionsEl {
    #[doc = "Set the field `sending_enabled`.\n"]
    pub fn set_sending_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.sending_enabled = Some(v.into());
        self
    }
}
impl ToListMappable for Sesv2ConfigurationSetSendingOptionsEl {
    type O = BlockAssignable<Sesv2ConfigurationSetSendingOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSesv2ConfigurationSetSendingOptionsEl {}
impl BuildSesv2ConfigurationSetSendingOptionsEl {
    pub fn build(self) -> Sesv2ConfigurationSetSendingOptionsEl {
        Sesv2ConfigurationSetSendingOptionsEl {
            sending_enabled: core::default::Default::default(),
        }
    }
}
pub struct Sesv2ConfigurationSetSendingOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Sesv2ConfigurationSetSendingOptionsElRef {
    fn new(shared: StackShared, base: String) -> Sesv2ConfigurationSetSendingOptionsElRef {
        Sesv2ConfigurationSetSendingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Sesv2ConfigurationSetSendingOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `sending_enabled` after provisioning.\n"]
    pub fn sending_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sending_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Sesv2ConfigurationSetSuppressionOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    suppressed_reasons: Option<ListField<PrimField<String>>>,
}
impl Sesv2ConfigurationSetSuppressionOptionsEl {
    #[doc = "Set the field `suppressed_reasons`.\n"]
    pub fn set_suppressed_reasons(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.suppressed_reasons = Some(v.into());
        self
    }
}
impl ToListMappable for Sesv2ConfigurationSetSuppressionOptionsEl {
    type O = BlockAssignable<Sesv2ConfigurationSetSuppressionOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSesv2ConfigurationSetSuppressionOptionsEl {}
impl BuildSesv2ConfigurationSetSuppressionOptionsEl {
    pub fn build(self) -> Sesv2ConfigurationSetSuppressionOptionsEl {
        Sesv2ConfigurationSetSuppressionOptionsEl {
            suppressed_reasons: core::default::Default::default(),
        }
    }
}
pub struct Sesv2ConfigurationSetSuppressionOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Sesv2ConfigurationSetSuppressionOptionsElRef {
    fn new(shared: StackShared, base: String) -> Sesv2ConfigurationSetSuppressionOptionsElRef {
        Sesv2ConfigurationSetSuppressionOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Sesv2ConfigurationSetSuppressionOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `suppressed_reasons` after provisioning.\n"]
    pub fn suppressed_reasons(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.suppressed_reasons", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Sesv2ConfigurationSetTrackingOptionsEl {
    custom_redirect_domain: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    https_policy: Option<PrimField<String>>,
}
impl Sesv2ConfigurationSetTrackingOptionsEl {
    #[doc = "Set the field `https_policy`.\n"]
    pub fn set_https_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.https_policy = Some(v.into());
        self
    }
}
impl ToListMappable for Sesv2ConfigurationSetTrackingOptionsEl {
    type O = BlockAssignable<Sesv2ConfigurationSetTrackingOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSesv2ConfigurationSetTrackingOptionsEl {
    #[doc = ""]
    pub custom_redirect_domain: PrimField<String>,
}
impl BuildSesv2ConfigurationSetTrackingOptionsEl {
    pub fn build(self) -> Sesv2ConfigurationSetTrackingOptionsEl {
        Sesv2ConfigurationSetTrackingOptionsEl {
            custom_redirect_domain: self.custom_redirect_domain,
            https_policy: core::default::Default::default(),
        }
    }
}
pub struct Sesv2ConfigurationSetTrackingOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Sesv2ConfigurationSetTrackingOptionsElRef {
    fn new(shared: StackShared, base: String) -> Sesv2ConfigurationSetTrackingOptionsElRef {
        Sesv2ConfigurationSetTrackingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Sesv2ConfigurationSetTrackingOptionsElRef {
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
    #[doc = "Get a reference to the value of field `https_policy` after provisioning.\n"]
    pub fn https_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.https_policy", self.base))
    }
}
#[derive(Serialize)]
pub struct Sesv2ConfigurationSetVdmOptionsElDashboardOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    engagement_metrics: Option<PrimField<String>>,
}
impl Sesv2ConfigurationSetVdmOptionsElDashboardOptionsEl {
    #[doc = "Set the field `engagement_metrics`.\n"]
    pub fn set_engagement_metrics(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.engagement_metrics = Some(v.into());
        self
    }
}
impl ToListMappable for Sesv2ConfigurationSetVdmOptionsElDashboardOptionsEl {
    type O = BlockAssignable<Sesv2ConfigurationSetVdmOptionsElDashboardOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSesv2ConfigurationSetVdmOptionsElDashboardOptionsEl {}
impl BuildSesv2ConfigurationSetVdmOptionsElDashboardOptionsEl {
    pub fn build(self) -> Sesv2ConfigurationSetVdmOptionsElDashboardOptionsEl {
        Sesv2ConfigurationSetVdmOptionsElDashboardOptionsEl {
            engagement_metrics: core::default::Default::default(),
        }
    }
}
pub struct Sesv2ConfigurationSetVdmOptionsElDashboardOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Sesv2ConfigurationSetVdmOptionsElDashboardOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Sesv2ConfigurationSetVdmOptionsElDashboardOptionsElRef {
        Sesv2ConfigurationSetVdmOptionsElDashboardOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Sesv2ConfigurationSetVdmOptionsElDashboardOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `engagement_metrics` after provisioning.\n"]
    pub fn engagement_metrics(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engagement_metrics", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Sesv2ConfigurationSetVdmOptionsElGuardianOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    optimized_shared_delivery: Option<PrimField<String>>,
}
impl Sesv2ConfigurationSetVdmOptionsElGuardianOptionsEl {
    #[doc = "Set the field `optimized_shared_delivery`.\n"]
    pub fn set_optimized_shared_delivery(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.optimized_shared_delivery = Some(v.into());
        self
    }
}
impl ToListMappable for Sesv2ConfigurationSetVdmOptionsElGuardianOptionsEl {
    type O = BlockAssignable<Sesv2ConfigurationSetVdmOptionsElGuardianOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSesv2ConfigurationSetVdmOptionsElGuardianOptionsEl {}
impl BuildSesv2ConfigurationSetVdmOptionsElGuardianOptionsEl {
    pub fn build(self) -> Sesv2ConfigurationSetVdmOptionsElGuardianOptionsEl {
        Sesv2ConfigurationSetVdmOptionsElGuardianOptionsEl {
            optimized_shared_delivery: core::default::Default::default(),
        }
    }
}
pub struct Sesv2ConfigurationSetVdmOptionsElGuardianOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Sesv2ConfigurationSetVdmOptionsElGuardianOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Sesv2ConfigurationSetVdmOptionsElGuardianOptionsElRef {
        Sesv2ConfigurationSetVdmOptionsElGuardianOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Sesv2ConfigurationSetVdmOptionsElGuardianOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `optimized_shared_delivery` after provisioning.\n"]
    pub fn optimized_shared_delivery(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.optimized_shared_delivery", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Sesv2ConfigurationSetVdmOptionsElDynamic {
    dashboard_options: Option<DynamicBlock<Sesv2ConfigurationSetVdmOptionsElDashboardOptionsEl>>,
    guardian_options: Option<DynamicBlock<Sesv2ConfigurationSetVdmOptionsElGuardianOptionsEl>>,
}
#[derive(Serialize)]
pub struct Sesv2ConfigurationSetVdmOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dashboard_options: Option<Vec<Sesv2ConfigurationSetVdmOptionsElDashboardOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guardian_options: Option<Vec<Sesv2ConfigurationSetVdmOptionsElGuardianOptionsEl>>,
    dynamic: Sesv2ConfigurationSetVdmOptionsElDynamic,
}
impl Sesv2ConfigurationSetVdmOptionsEl {
    #[doc = "Set the field `dashboard_options`.\n"]
    pub fn set_dashboard_options(
        mut self,
        v: impl Into<BlockAssignable<Sesv2ConfigurationSetVdmOptionsElDashboardOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dashboard_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dashboard_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `guardian_options`.\n"]
    pub fn set_guardian_options(
        mut self,
        v: impl Into<BlockAssignable<Sesv2ConfigurationSetVdmOptionsElGuardianOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.guardian_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.guardian_options = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Sesv2ConfigurationSetVdmOptionsEl {
    type O = BlockAssignable<Sesv2ConfigurationSetVdmOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSesv2ConfigurationSetVdmOptionsEl {}
impl BuildSesv2ConfigurationSetVdmOptionsEl {
    pub fn build(self) -> Sesv2ConfigurationSetVdmOptionsEl {
        Sesv2ConfigurationSetVdmOptionsEl {
            dashboard_options: core::default::Default::default(),
            guardian_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Sesv2ConfigurationSetVdmOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Sesv2ConfigurationSetVdmOptionsElRef {
    fn new(shared: StackShared, base: String) -> Sesv2ConfigurationSetVdmOptionsElRef {
        Sesv2ConfigurationSetVdmOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Sesv2ConfigurationSetVdmOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dashboard_options` after provisioning.\n"]
    pub fn dashboard_options(
        &self,
    ) -> ListRef<Sesv2ConfigurationSetVdmOptionsElDashboardOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dashboard_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `guardian_options` after provisioning.\n"]
    pub fn guardian_options(
        &self,
    ) -> ListRef<Sesv2ConfigurationSetVdmOptionsElGuardianOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guardian_options", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Sesv2ConfigurationSetDynamic {
    delivery_options: Option<DynamicBlock<Sesv2ConfigurationSetDeliveryOptionsEl>>,
    reputation_options: Option<DynamicBlock<Sesv2ConfigurationSetReputationOptionsEl>>,
    sending_options: Option<DynamicBlock<Sesv2ConfigurationSetSendingOptionsEl>>,
    suppression_options: Option<DynamicBlock<Sesv2ConfigurationSetSuppressionOptionsEl>>,
    tracking_options: Option<DynamicBlock<Sesv2ConfigurationSetTrackingOptionsEl>>,
    vdm_options: Option<DynamicBlock<Sesv2ConfigurationSetVdmOptionsEl>>,
}
