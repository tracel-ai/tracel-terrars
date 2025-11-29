use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataServicequotasServiceQuotaData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quota_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quota_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    service_code: PrimField<String>,
}

struct DataServicequotasServiceQuota_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataServicequotasServiceQuotaData>,
}

#[derive(Clone)]
pub struct DataServicequotasServiceQuota(Rc<DataServicequotasServiceQuota_>);

impl DataServicequotasServiceQuota {
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

    #[doc = "Set the field `quota_code`.\n"]
    pub fn set_quota_code(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().quota_code = Some(v.into());
        self
    }

    #[doc = "Set the field `quota_name`.\n"]
    pub fn set_quota_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().quota_name = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `adjustable` after provisioning.\n"]
    pub fn adjustable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.adjustable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `global_quota` after provisioning.\n"]
    pub fn global_quota(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.global_quota", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_code` after provisioning.\n"]
    pub fn quota_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.quota_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_name` after provisioning.\n"]
    pub fn quota_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.quota_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_code` after provisioning.\n"]
    pub fn service_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `usage_metric` after provisioning.\n"]
    pub fn usage_metric(&self) -> ListRef<DataServicequotasServiceQuotaUsageMetricElRef> {
        ListRef::new(self.shared().clone(), format!("{}.usage_metric", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }
}

impl Referable for DataServicequotasServiceQuota {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataServicequotasServiceQuota { }

impl ToListMappable for DataServicequotasServiceQuota {
    type O = ListRef<DataServicequotasServiceQuotaRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataServicequotasServiceQuota_ {
    fn extract_datasource_type(&self) -> String {
        "aws_servicequotas_service_quota".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataServicequotasServiceQuota {
    pub tf_id: String,
    #[doc = ""]
    pub service_code: PrimField<String>,
}

impl BuildDataServicequotasServiceQuota {
    pub fn build(self, stack: &mut Stack) -> DataServicequotasServiceQuota {
        let out = DataServicequotasServiceQuota(Rc::new(DataServicequotasServiceQuota_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataServicequotasServiceQuotaData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                quota_code: core::default::Default::default(),
                quota_name: core::default::Default::default(),
                region: core::default::Default::default(),
                service_code: self.service_code,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataServicequotasServiceQuotaRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataServicequotasServiceQuotaRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataServicequotasServiceQuotaRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `adjustable` after provisioning.\n"]
    pub fn adjustable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.adjustable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `global_quota` after provisioning.\n"]
    pub fn global_quota(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.global_quota", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_code` after provisioning.\n"]
    pub fn quota_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.quota_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_name` after provisioning.\n"]
    pub fn quota_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.quota_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_code` after provisioning.\n"]
    pub fn service_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `usage_metric` after provisioning.\n"]
    pub fn usage_metric(&self) -> ListRef<DataServicequotasServiceQuotaUsageMetricElRef> {
        ListRef::new(self.shared().clone(), format!("{}.usage_metric", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataServicequotasServiceQuotaUsageMetricElMetricDimensionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    class: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataServicequotasServiceQuotaUsageMetricElMetricDimensionsEl {
    #[doc = "Set the field `class`.\n"]
    pub fn set_class(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.class = Some(v.into());
        self
    }

    #[doc = "Set the field `resource`.\n"]
    pub fn set_resource(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource = Some(v.into());
        self
    }

    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataServicequotasServiceQuotaUsageMetricElMetricDimensionsEl {
    type O = BlockAssignable<DataServicequotasServiceQuotaUsageMetricElMetricDimensionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataServicequotasServiceQuotaUsageMetricElMetricDimensionsEl {}

impl BuildDataServicequotasServiceQuotaUsageMetricElMetricDimensionsEl {
    pub fn build(self) -> DataServicequotasServiceQuotaUsageMetricElMetricDimensionsEl {
        DataServicequotasServiceQuotaUsageMetricElMetricDimensionsEl {
            class: core::default::Default::default(),
            resource: core::default::Default::default(),
            service: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataServicequotasServiceQuotaUsageMetricElMetricDimensionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataServicequotasServiceQuotaUsageMetricElMetricDimensionsElRef {
    fn new(shared: StackShared, base: String) -> DataServicequotasServiceQuotaUsageMetricElMetricDimensionsElRef {
        DataServicequotasServiceQuotaUsageMetricElMetricDimensionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataServicequotasServiceQuotaUsageMetricElMetricDimensionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `class` after provisioning.\n"]
    pub fn class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.class", self.base))
    }

    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataServicequotasServiceQuotaUsageMetricEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_dimensions: Option<ListField<DataServicequotasServiceQuotaUsageMetricElMetricDimensionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_statistic_recommendation: Option<PrimField<String>>,
}

impl DataServicequotasServiceQuotaUsageMetricEl {
    #[doc = "Set the field `metric_dimensions`.\n"]
    pub fn set_metric_dimensions(
        mut self,
        v: impl Into<ListField<DataServicequotasServiceQuotaUsageMetricElMetricDimensionsEl>>,
    ) -> Self {
        self.metric_dimensions = Some(v.into());
        self
    }

    #[doc = "Set the field `metric_name`.\n"]
    pub fn set_metric_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metric_name = Some(v.into());
        self
    }

    #[doc = "Set the field `metric_namespace`.\n"]
    pub fn set_metric_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metric_namespace = Some(v.into());
        self
    }

    #[doc = "Set the field `metric_statistic_recommendation`.\n"]
    pub fn set_metric_statistic_recommendation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metric_statistic_recommendation = Some(v.into());
        self
    }
}

impl ToListMappable for DataServicequotasServiceQuotaUsageMetricEl {
    type O = BlockAssignable<DataServicequotasServiceQuotaUsageMetricEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataServicequotasServiceQuotaUsageMetricEl {}

impl BuildDataServicequotasServiceQuotaUsageMetricEl {
    pub fn build(self) -> DataServicequotasServiceQuotaUsageMetricEl {
        DataServicequotasServiceQuotaUsageMetricEl {
            metric_dimensions: core::default::Default::default(),
            metric_name: core::default::Default::default(),
            metric_namespace: core::default::Default::default(),
            metric_statistic_recommendation: core::default::Default::default(),
        }
    }
}

pub struct DataServicequotasServiceQuotaUsageMetricElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataServicequotasServiceQuotaUsageMetricElRef {
    fn new(shared: StackShared, base: String) -> DataServicequotasServiceQuotaUsageMetricElRef {
        DataServicequotasServiceQuotaUsageMetricElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataServicequotasServiceQuotaUsageMetricElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_dimensions` after provisioning.\n"]
    pub fn metric_dimensions(&self) -> ListRef<DataServicequotasServiceQuotaUsageMetricElMetricDimensionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.metric_dimensions", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_namespace` after provisioning.\n"]
    pub fn metric_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_statistic_recommendation` after provisioning.\n"]
    pub fn metric_statistic_recommendation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_statistic_recommendation", self.base))
    }
}
