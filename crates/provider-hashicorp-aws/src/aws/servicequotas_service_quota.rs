use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ServicequotasServiceQuotaData {
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
    quota_code: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    service_code: PrimField<String>,
    value: PrimField<f64>,
}

struct ServicequotasServiceQuota_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ServicequotasServiceQuotaData>,
}

#[derive(Clone)]
pub struct ServicequotasServiceQuota(Rc<ServicequotasServiceQuota_>);

impl ServicequotasServiceQuota {
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

    #[doc = "Get a reference to the value of field `adjustable` after provisioning.\n"]
    pub fn adjustable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.adjustable", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_value", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_code` after provisioning.\n"]
    pub fn quota_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quota_code", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `quota_name` after provisioning.\n"]
    pub fn quota_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quota_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `request_id` after provisioning.\n"]
    pub fn request_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `request_status` after provisioning.\n"]
    pub fn request_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_code` after provisioning.\n"]
    pub fn service_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_code", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `usage_metric` after provisioning.\n"]
    pub fn usage_metric(&self) -> ListRef<ServicequotasServiceQuotaUsageMetricElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.usage_metric", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.value", self.extract_ref()),
        )
    }
}

impl Referable for ServicequotasServiceQuota {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for ServicequotasServiceQuota {}

impl ToListMappable for ServicequotasServiceQuota {
    type O = ListRef<ServicequotasServiceQuotaRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ServicequotasServiceQuota_ {
    fn extract_resource_type(&self) -> String {
        "aws_servicequotas_service_quota".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildServicequotasServiceQuota {
    pub tf_id: String,
    #[doc = ""]
    pub quota_code: PrimField<String>,
    #[doc = ""]
    pub service_code: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildServicequotasServiceQuota {
    pub fn build(self, stack: &mut Stack) -> ServicequotasServiceQuota {
        let out = ServicequotasServiceQuota(Rc::new(ServicequotasServiceQuota_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ServicequotasServiceQuotaData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                quota_code: self.quota_code,
                region: core::default::Default::default(),
                service_code: self.service_code,
                value: self.value,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ServicequotasServiceQuotaRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServicequotasServiceQuotaRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl ServicequotasServiceQuotaRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `adjustable` after provisioning.\n"]
    pub fn adjustable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.adjustable", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_value", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_code` after provisioning.\n"]
    pub fn quota_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quota_code", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `quota_name` after provisioning.\n"]
    pub fn quota_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quota_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `request_id` after provisioning.\n"]
    pub fn request_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `request_status` after provisioning.\n"]
    pub fn request_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_code` after provisioning.\n"]
    pub fn service_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_code", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `usage_metric` after provisioning.\n"]
    pub fn usage_metric(&self) -> ListRef<ServicequotasServiceQuotaUsageMetricElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.usage_metric", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.value", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ServicequotasServiceQuotaUsageMetricElMetricDimensionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    class: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl ServicequotasServiceQuotaUsageMetricElMetricDimensionsEl {
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

impl ToListMappable for ServicequotasServiceQuotaUsageMetricElMetricDimensionsEl {
    type O = BlockAssignable<ServicequotasServiceQuotaUsageMetricElMetricDimensionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServicequotasServiceQuotaUsageMetricElMetricDimensionsEl {}

impl BuildServicequotasServiceQuotaUsageMetricElMetricDimensionsEl {
    pub fn build(self) -> ServicequotasServiceQuotaUsageMetricElMetricDimensionsEl {
        ServicequotasServiceQuotaUsageMetricElMetricDimensionsEl {
            class: core::default::Default::default(),
            resource: core::default::Default::default(),
            service: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct ServicequotasServiceQuotaUsageMetricElMetricDimensionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServicequotasServiceQuotaUsageMetricElMetricDimensionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ServicequotasServiceQuotaUsageMetricElMetricDimensionsElRef {
        ServicequotasServiceQuotaUsageMetricElMetricDimensionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServicequotasServiceQuotaUsageMetricElMetricDimensionsElRef {
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
pub struct ServicequotasServiceQuotaUsageMetricEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_dimensions: Option<ListField<ServicequotasServiceQuotaUsageMetricElMetricDimensionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_statistic_recommendation: Option<PrimField<String>>,
}

impl ServicequotasServiceQuotaUsageMetricEl {
    #[doc = "Set the field `metric_dimensions`.\n"]
    pub fn set_metric_dimensions(
        mut self,
        v: impl Into<ListField<ServicequotasServiceQuotaUsageMetricElMetricDimensionsEl>>,
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

impl ToListMappable for ServicequotasServiceQuotaUsageMetricEl {
    type O = BlockAssignable<ServicequotasServiceQuotaUsageMetricEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServicequotasServiceQuotaUsageMetricEl {}

impl BuildServicequotasServiceQuotaUsageMetricEl {
    pub fn build(self) -> ServicequotasServiceQuotaUsageMetricEl {
        ServicequotasServiceQuotaUsageMetricEl {
            metric_dimensions: core::default::Default::default(),
            metric_name: core::default::Default::default(),
            metric_namespace: core::default::Default::default(),
            metric_statistic_recommendation: core::default::Default::default(),
        }
    }
}

pub struct ServicequotasServiceQuotaUsageMetricElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServicequotasServiceQuotaUsageMetricElRef {
    fn new(shared: StackShared, base: String) -> ServicequotasServiceQuotaUsageMetricElRef {
        ServicequotasServiceQuotaUsageMetricElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServicequotasServiceQuotaUsageMetricElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_dimensions` after provisioning.\n"]
    pub fn metric_dimensions(
        &self,
    ) -> ListRef<ServicequotasServiceQuotaUsageMetricElMetricDimensionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metric_dimensions", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_namespace` after provisioning.\n"]
    pub fn metric_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric_namespace", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `metric_statistic_recommendation` after provisioning.\n"]
    pub fn metric_statistic_recommendation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric_statistic_recommendation", self.base),
        )
    }
}
