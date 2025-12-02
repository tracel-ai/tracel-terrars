use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataSesv2ConfigurationSetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    configuration_set_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}
struct DataSesv2ConfigurationSet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSesv2ConfigurationSetData>,
}
#[derive(Clone)]
pub struct DataSesv2ConfigurationSet(Rc<DataSesv2ConfigurationSet_>);
impl DataSesv2ConfigurationSet {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
    #[doc = "Get a reference to the value of field `delivery_options` after provisioning.\n"]
    pub fn delivery_options(&self) -> ListRef<DataSesv2ConfigurationSetDeliveryOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.delivery_options", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `reputation_options` after provisioning.\n"]
    pub fn reputation_options(&self) -> ListRef<DataSesv2ConfigurationSetReputationOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reputation_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sending_options` after provisioning.\n"]
    pub fn sending_options(&self) -> ListRef<DataSesv2ConfigurationSetSendingOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sending_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `suppression_options` after provisioning.\n"]
    pub fn suppression_options(&self) -> ListRef<DataSesv2ConfigurationSetSuppressionOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.suppression_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tracking_options` after provisioning.\n"]
    pub fn tracking_options(&self) -> ListRef<DataSesv2ConfigurationSetTrackingOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tracking_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vdm_options` after provisioning.\n"]
    pub fn vdm_options(&self) -> ListRef<DataSesv2ConfigurationSetVdmOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vdm_options", self.extract_ref()),
        )
    }
}
impl Referable for DataSesv2ConfigurationSet {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataSesv2ConfigurationSet {}
impl ToListMappable for DataSesv2ConfigurationSet {
    type O = ListRef<DataSesv2ConfigurationSetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataSesv2ConfigurationSet_ {
    fn extract_datasource_type(&self) -> String {
        "aws_sesv2_configuration_set".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataSesv2ConfigurationSet {
    pub tf_id: String,
    #[doc = ""]
    pub configuration_set_name: PrimField<String>,
}
impl BuildDataSesv2ConfigurationSet {
    pub fn build(self, stack: &mut Stack) -> DataSesv2ConfigurationSet {
        let out = DataSesv2ConfigurationSet(Rc::new(DataSesv2ConfigurationSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSesv2ConfigurationSetData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                configuration_set_name: self.configuration_set_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataSesv2ConfigurationSetRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSesv2ConfigurationSetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataSesv2ConfigurationSetRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
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
    #[doc = "Get a reference to the value of field `delivery_options` after provisioning.\n"]
    pub fn delivery_options(&self) -> ListRef<DataSesv2ConfigurationSetDeliveryOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.delivery_options", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `reputation_options` after provisioning.\n"]
    pub fn reputation_options(&self) -> ListRef<DataSesv2ConfigurationSetReputationOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reputation_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sending_options` after provisioning.\n"]
    pub fn sending_options(&self) -> ListRef<DataSesv2ConfigurationSetSendingOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sending_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `suppression_options` after provisioning.\n"]
    pub fn suppression_options(&self) -> ListRef<DataSesv2ConfigurationSetSuppressionOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.suppression_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tracking_options` after provisioning.\n"]
    pub fn tracking_options(&self) -> ListRef<DataSesv2ConfigurationSetTrackingOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tracking_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vdm_options` after provisioning.\n"]
    pub fn vdm_options(&self) -> ListRef<DataSesv2ConfigurationSetVdmOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vdm_options", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataSesv2ConfigurationSetDeliveryOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_delivery_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sending_pool_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_policy: Option<PrimField<String>>,
}
impl DataSesv2ConfigurationSetDeliveryOptionsEl {
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
impl ToListMappable for DataSesv2ConfigurationSetDeliveryOptionsEl {
    type O = BlockAssignable<DataSesv2ConfigurationSetDeliveryOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSesv2ConfigurationSetDeliveryOptionsEl {}
impl BuildDataSesv2ConfigurationSetDeliveryOptionsEl {
    pub fn build(self) -> DataSesv2ConfigurationSetDeliveryOptionsEl {
        DataSesv2ConfigurationSetDeliveryOptionsEl {
            max_delivery_seconds: core::default::Default::default(),
            sending_pool_name: core::default::Default::default(),
            tls_policy: core::default::Default::default(),
        }
    }
}
pub struct DataSesv2ConfigurationSetDeliveryOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSesv2ConfigurationSetDeliveryOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataSesv2ConfigurationSetDeliveryOptionsElRef {
        DataSesv2ConfigurationSetDeliveryOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSesv2ConfigurationSetDeliveryOptionsElRef {
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
pub struct DataSesv2ConfigurationSetReputationOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    last_fresh_start: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reputation_metrics_enabled: Option<PrimField<bool>>,
}
impl DataSesv2ConfigurationSetReputationOptionsEl {
    #[doc = "Set the field `last_fresh_start`.\n"]
    pub fn set_last_fresh_start(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_fresh_start = Some(v.into());
        self
    }
    #[doc = "Set the field `reputation_metrics_enabled`.\n"]
    pub fn set_reputation_metrics_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.reputation_metrics_enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataSesv2ConfigurationSetReputationOptionsEl {
    type O = BlockAssignable<DataSesv2ConfigurationSetReputationOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSesv2ConfigurationSetReputationOptionsEl {}
impl BuildDataSesv2ConfigurationSetReputationOptionsEl {
    pub fn build(self) -> DataSesv2ConfigurationSetReputationOptionsEl {
        DataSesv2ConfigurationSetReputationOptionsEl {
            last_fresh_start: core::default::Default::default(),
            reputation_metrics_enabled: core::default::Default::default(),
        }
    }
}
pub struct DataSesv2ConfigurationSetReputationOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSesv2ConfigurationSetReputationOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataSesv2ConfigurationSetReputationOptionsElRef {
        DataSesv2ConfigurationSetReputationOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSesv2ConfigurationSetReputationOptionsElRef {
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
pub struct DataSesv2ConfigurationSetSendingOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    sending_enabled: Option<PrimField<bool>>,
}
impl DataSesv2ConfigurationSetSendingOptionsEl {
    #[doc = "Set the field `sending_enabled`.\n"]
    pub fn set_sending_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.sending_enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataSesv2ConfigurationSetSendingOptionsEl {
    type O = BlockAssignable<DataSesv2ConfigurationSetSendingOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSesv2ConfigurationSetSendingOptionsEl {}
impl BuildDataSesv2ConfigurationSetSendingOptionsEl {
    pub fn build(self) -> DataSesv2ConfigurationSetSendingOptionsEl {
        DataSesv2ConfigurationSetSendingOptionsEl {
            sending_enabled: core::default::Default::default(),
        }
    }
}
pub struct DataSesv2ConfigurationSetSendingOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSesv2ConfigurationSetSendingOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataSesv2ConfigurationSetSendingOptionsElRef {
        DataSesv2ConfigurationSetSendingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSesv2ConfigurationSetSendingOptionsElRef {
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
pub struct DataSesv2ConfigurationSetSuppressionOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    suppressed_reasons: Option<ListField<PrimField<String>>>,
}
impl DataSesv2ConfigurationSetSuppressionOptionsEl {
    #[doc = "Set the field `suppressed_reasons`.\n"]
    pub fn set_suppressed_reasons(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.suppressed_reasons = Some(v.into());
        self
    }
}
impl ToListMappable for DataSesv2ConfigurationSetSuppressionOptionsEl {
    type O = BlockAssignable<DataSesv2ConfigurationSetSuppressionOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSesv2ConfigurationSetSuppressionOptionsEl {}
impl BuildDataSesv2ConfigurationSetSuppressionOptionsEl {
    pub fn build(self) -> DataSesv2ConfigurationSetSuppressionOptionsEl {
        DataSesv2ConfigurationSetSuppressionOptionsEl {
            suppressed_reasons: core::default::Default::default(),
        }
    }
}
pub struct DataSesv2ConfigurationSetSuppressionOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSesv2ConfigurationSetSuppressionOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataSesv2ConfigurationSetSuppressionOptionsElRef {
        DataSesv2ConfigurationSetSuppressionOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSesv2ConfigurationSetSuppressionOptionsElRef {
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
pub struct DataSesv2ConfigurationSetTrackingOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_redirect_domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    https_policy: Option<PrimField<String>>,
}
impl DataSesv2ConfigurationSetTrackingOptionsEl {
    #[doc = "Set the field `custom_redirect_domain`.\n"]
    pub fn set_custom_redirect_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.custom_redirect_domain = Some(v.into());
        self
    }
    #[doc = "Set the field `https_policy`.\n"]
    pub fn set_https_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.https_policy = Some(v.into());
        self
    }
}
impl ToListMappable for DataSesv2ConfigurationSetTrackingOptionsEl {
    type O = BlockAssignable<DataSesv2ConfigurationSetTrackingOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSesv2ConfigurationSetTrackingOptionsEl {}
impl BuildDataSesv2ConfigurationSetTrackingOptionsEl {
    pub fn build(self) -> DataSesv2ConfigurationSetTrackingOptionsEl {
        DataSesv2ConfigurationSetTrackingOptionsEl {
            custom_redirect_domain: core::default::Default::default(),
            https_policy: core::default::Default::default(),
        }
    }
}
pub struct DataSesv2ConfigurationSetTrackingOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSesv2ConfigurationSetTrackingOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataSesv2ConfigurationSetTrackingOptionsElRef {
        DataSesv2ConfigurationSetTrackingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSesv2ConfigurationSetTrackingOptionsElRef {
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
pub struct DataSesv2ConfigurationSetVdmOptionsElDashboardOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    engagement_metrics: Option<PrimField<String>>,
}
impl DataSesv2ConfigurationSetVdmOptionsElDashboardOptionsEl {
    #[doc = "Set the field `engagement_metrics`.\n"]
    pub fn set_engagement_metrics(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.engagement_metrics = Some(v.into());
        self
    }
}
impl ToListMappable for DataSesv2ConfigurationSetVdmOptionsElDashboardOptionsEl {
    type O = BlockAssignable<DataSesv2ConfigurationSetVdmOptionsElDashboardOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSesv2ConfigurationSetVdmOptionsElDashboardOptionsEl {}
impl BuildDataSesv2ConfigurationSetVdmOptionsElDashboardOptionsEl {
    pub fn build(self) -> DataSesv2ConfigurationSetVdmOptionsElDashboardOptionsEl {
        DataSesv2ConfigurationSetVdmOptionsElDashboardOptionsEl {
            engagement_metrics: core::default::Default::default(),
        }
    }
}
pub struct DataSesv2ConfigurationSetVdmOptionsElDashboardOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSesv2ConfigurationSetVdmOptionsElDashboardOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSesv2ConfigurationSetVdmOptionsElDashboardOptionsElRef {
        DataSesv2ConfigurationSetVdmOptionsElDashboardOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSesv2ConfigurationSetVdmOptionsElDashboardOptionsElRef {
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
pub struct DataSesv2ConfigurationSetVdmOptionsElGuardianOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    optimized_shared_delivery: Option<PrimField<String>>,
}
impl DataSesv2ConfigurationSetVdmOptionsElGuardianOptionsEl {
    #[doc = "Set the field `optimized_shared_delivery`.\n"]
    pub fn set_optimized_shared_delivery(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.optimized_shared_delivery = Some(v.into());
        self
    }
}
impl ToListMappable for DataSesv2ConfigurationSetVdmOptionsElGuardianOptionsEl {
    type O = BlockAssignable<DataSesv2ConfigurationSetVdmOptionsElGuardianOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSesv2ConfigurationSetVdmOptionsElGuardianOptionsEl {}
impl BuildDataSesv2ConfigurationSetVdmOptionsElGuardianOptionsEl {
    pub fn build(self) -> DataSesv2ConfigurationSetVdmOptionsElGuardianOptionsEl {
        DataSesv2ConfigurationSetVdmOptionsElGuardianOptionsEl {
            optimized_shared_delivery: core::default::Default::default(),
        }
    }
}
pub struct DataSesv2ConfigurationSetVdmOptionsElGuardianOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSesv2ConfigurationSetVdmOptionsElGuardianOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSesv2ConfigurationSetVdmOptionsElGuardianOptionsElRef {
        DataSesv2ConfigurationSetVdmOptionsElGuardianOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSesv2ConfigurationSetVdmOptionsElGuardianOptionsElRef {
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
#[derive(Serialize)]
pub struct DataSesv2ConfigurationSetVdmOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dashboard_options: Option<ListField<DataSesv2ConfigurationSetVdmOptionsElDashboardOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guardian_options: Option<ListField<DataSesv2ConfigurationSetVdmOptionsElGuardianOptionsEl>>,
}
impl DataSesv2ConfigurationSetVdmOptionsEl {
    #[doc = "Set the field `dashboard_options`.\n"]
    pub fn set_dashboard_options(
        mut self,
        v: impl Into<ListField<DataSesv2ConfigurationSetVdmOptionsElDashboardOptionsEl>>,
    ) -> Self {
        self.dashboard_options = Some(v.into());
        self
    }
    #[doc = "Set the field `guardian_options`.\n"]
    pub fn set_guardian_options(
        mut self,
        v: impl Into<ListField<DataSesv2ConfigurationSetVdmOptionsElGuardianOptionsEl>>,
    ) -> Self {
        self.guardian_options = Some(v.into());
        self
    }
}
impl ToListMappable for DataSesv2ConfigurationSetVdmOptionsEl {
    type O = BlockAssignable<DataSesv2ConfigurationSetVdmOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSesv2ConfigurationSetVdmOptionsEl {}
impl BuildDataSesv2ConfigurationSetVdmOptionsEl {
    pub fn build(self) -> DataSesv2ConfigurationSetVdmOptionsEl {
        DataSesv2ConfigurationSetVdmOptionsEl {
            dashboard_options: core::default::Default::default(),
            guardian_options: core::default::Default::default(),
        }
    }
}
pub struct DataSesv2ConfigurationSetVdmOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSesv2ConfigurationSetVdmOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataSesv2ConfigurationSetVdmOptionsElRef {
        DataSesv2ConfigurationSetVdmOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSesv2ConfigurationSetVdmOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dashboard_options` after provisioning.\n"]
    pub fn dashboard_options(
        &self,
    ) -> ListRef<DataSesv2ConfigurationSetVdmOptionsElDashboardOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dashboard_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `guardian_options` after provisioning.\n"]
    pub fn guardian_options(
        &self,
    ) -> ListRef<DataSesv2ConfigurationSetVdmOptionsElGuardianOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guardian_options", self.base),
        )
    }
}
