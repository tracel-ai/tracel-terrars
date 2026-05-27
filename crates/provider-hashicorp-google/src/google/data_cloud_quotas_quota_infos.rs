use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCloudQuotasQuotaInfosData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    parent: PrimField<String>,
    service: PrimField<String>,
}
struct DataCloudQuotasQuotaInfos_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudQuotasQuotaInfosData>,
}
#[derive(Clone)]
pub struct DataCloudQuotasQuotaInfos(Rc<DataCloudQuotasQuotaInfos_>);
impl DataCloudQuotasQuotaInfos {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\n"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quota_infos` after provisioning.\n"]
    pub fn quota_infos(&self) -> ListRef<DataCloudQuotasQuotaInfosQuotaInfosElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.quota_infos", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service", self.extract_ref()),
        )
    }
}
impl Referable for DataCloudQuotasQuotaInfos {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataCloudQuotasQuotaInfos {}
impl ToListMappable for DataCloudQuotasQuotaInfos {
    type O = ListRef<DataCloudQuotasQuotaInfosRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataCloudQuotasQuotaInfos_ {
    fn extract_datasource_type(&self) -> String {
        "google_cloud_quotas_quota_infos".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCloudQuotasQuotaInfos {
    pub tf_id: String,
    #[doc = ""]
    pub parent: PrimField<String>,
    #[doc = ""]
    pub service: PrimField<String>,
}
impl BuildDataCloudQuotasQuotaInfos {
    pub fn build(self, stack: &mut Stack) -> DataCloudQuotasQuotaInfos {
        let out = DataCloudQuotasQuotaInfos(Rc::new(DataCloudQuotasQuotaInfos_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCloudQuotasQuotaInfosData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                parent: self.parent,
                service: self.service,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataCloudQuotasQuotaInfosRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudQuotasQuotaInfosRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCloudQuotasQuotaInfosRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\n"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quota_infos` after provisioning.\n"]
    pub fn quota_infos(&self) -> ListRef<DataCloudQuotasQuotaInfosQuotaInfosElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.quota_infos", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosElDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosElDetailsEl {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosElDetailsEl {
    type O = BlockAssignable<DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosElDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosElDetailsEl {}
impl BuildDataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosElDetailsEl {
    pub fn build(self) -> DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosElDetailsEl {
        DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosElDetailsEl {
            value: core::default::Default::default(),
        }
    }
}
pub struct DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosElDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosElDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosElDetailsElRef {
        DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosElDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosElDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    applicable_locations: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<ListField<DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosElDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions: Option<RecField<PrimField<String>>>,
}
impl DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosEl {
    #[doc = "Set the field `applicable_locations`.\n"]
    pub fn set_applicable_locations(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.applicable_locations = Some(v.into());
        self
    }
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        mut self,
        v: impl Into<ListField<DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosElDetailsEl>>,
    ) -> Self {
        self.details = Some(v.into());
        self
    }
    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.dimensions = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosEl {
    type O = BlockAssignable<DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosEl {}
impl BuildDataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosEl {
    pub fn build(self) -> DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosEl {
        DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosEl {
            applicable_locations: core::default::Default::default(),
            details: core::default::Default::default(),
            dimensions: core::default::Default::default(),
        }
    }
}
pub struct DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosElRef {
        DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `applicable_locations` after provisioning.\n"]
    pub fn applicable_locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.applicable_locations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(
        &self,
    ) -> ListRef<DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosElDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.details", self.base))
    }
    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudQuotasQuotaInfosQuotaInfosElQuotaIncreaseEligibilityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ineligibility_reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_eligible: Option<PrimField<bool>>,
}
impl DataCloudQuotasQuotaInfosQuotaInfosElQuotaIncreaseEligibilityEl {
    #[doc = "Set the field `ineligibility_reason`.\n"]
    pub fn set_ineligibility_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ineligibility_reason = Some(v.into());
        self
    }
    #[doc = "Set the field `is_eligible`.\n"]
    pub fn set_is_eligible(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_eligible = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudQuotasQuotaInfosQuotaInfosElQuotaIncreaseEligibilityEl {
    type O = BlockAssignable<DataCloudQuotasQuotaInfosQuotaInfosElQuotaIncreaseEligibilityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudQuotasQuotaInfosQuotaInfosElQuotaIncreaseEligibilityEl {}
impl BuildDataCloudQuotasQuotaInfosQuotaInfosElQuotaIncreaseEligibilityEl {
    pub fn build(self) -> DataCloudQuotasQuotaInfosQuotaInfosElQuotaIncreaseEligibilityEl {
        DataCloudQuotasQuotaInfosQuotaInfosElQuotaIncreaseEligibilityEl {
            ineligibility_reason: core::default::Default::default(),
            is_eligible: core::default::Default::default(),
        }
    }
}
pub struct DataCloudQuotasQuotaInfosQuotaInfosElQuotaIncreaseEligibilityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudQuotasQuotaInfosQuotaInfosElQuotaIncreaseEligibilityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudQuotasQuotaInfosQuotaInfosElQuotaIncreaseEligibilityElRef {
        DataCloudQuotasQuotaInfosQuotaInfosElQuotaIncreaseEligibilityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudQuotasQuotaInfosQuotaInfosElQuotaIncreaseEligibilityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ineligibility_reason` after provisioning.\n"]
    pub fn ineligibility_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ineligibility_reason", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `is_eligible` after provisioning.\n"]
    pub fn is_eligible(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_eligible", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudQuotasQuotaInfosQuotaInfosEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions_infos: Option<ListField<DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_concurrent: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_fixed: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_precise: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quota_display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quota_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quota_increase_eligibility:
        Option<ListField<DataCloudQuotasQuotaInfosQuotaInfosElQuotaIncreaseEligibilityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_interval: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_request_quota_uri: Option<PrimField<String>>,
}
impl DataCloudQuotasQuotaInfosQuotaInfosEl {
    #[doc = "Set the field `container_type`.\n"]
    pub fn set_container_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container_type = Some(v.into());
        self
    }
    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dimensions = Some(v.into());
        self
    }
    #[doc = "Set the field `dimensions_infos`.\n"]
    pub fn set_dimensions_infos(
        mut self,
        v: impl Into<ListField<DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosEl>>,
    ) -> Self {
        self.dimensions_infos = Some(v.into());
        self
    }
    #[doc = "Set the field `is_concurrent`.\n"]
    pub fn set_is_concurrent(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_concurrent = Some(v.into());
        self
    }
    #[doc = "Set the field `is_fixed`.\n"]
    pub fn set_is_fixed(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_fixed = Some(v.into());
        self
    }
    #[doc = "Set the field `is_precise`.\n"]
    pub fn set_is_precise(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_precise = Some(v.into());
        self
    }
    #[doc = "Set the field `metric`.\n"]
    pub fn set_metric(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metric = Some(v.into());
        self
    }
    #[doc = "Set the field `metric_display_name`.\n"]
    pub fn set_metric_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metric_display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `metric_unit`.\n"]
    pub fn set_metric_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metric_unit = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `quota_display_name`.\n"]
    pub fn set_quota_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.quota_display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `quota_id`.\n"]
    pub fn set_quota_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.quota_id = Some(v.into());
        self
    }
    #[doc = "Set the field `quota_increase_eligibility`.\n"]
    pub fn set_quota_increase_eligibility(
        mut self,
        v: impl Into<ListField<DataCloudQuotasQuotaInfosQuotaInfosElQuotaIncreaseEligibilityEl>>,
    ) -> Self {
        self.quota_increase_eligibility = Some(v.into());
        self
    }
    #[doc = "Set the field `refresh_interval`.\n"]
    pub fn set_refresh_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.refresh_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
    #[doc = "Set the field `service_request_quota_uri`.\n"]
    pub fn set_service_request_quota_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_request_quota_uri = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudQuotasQuotaInfosQuotaInfosEl {
    type O = BlockAssignable<DataCloudQuotasQuotaInfosQuotaInfosEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudQuotasQuotaInfosQuotaInfosEl {}
impl BuildDataCloudQuotasQuotaInfosQuotaInfosEl {
    pub fn build(self) -> DataCloudQuotasQuotaInfosQuotaInfosEl {
        DataCloudQuotasQuotaInfosQuotaInfosEl {
            container_type: core::default::Default::default(),
            dimensions: core::default::Default::default(),
            dimensions_infos: core::default::Default::default(),
            is_concurrent: core::default::Default::default(),
            is_fixed: core::default::Default::default(),
            is_precise: core::default::Default::default(),
            metric: core::default::Default::default(),
            metric_display_name: core::default::Default::default(),
            metric_unit: core::default::Default::default(),
            name: core::default::Default::default(),
            quota_display_name: core::default::Default::default(),
            quota_id: core::default::Default::default(),
            quota_increase_eligibility: core::default::Default::default(),
            refresh_interval: core::default::Default::default(),
            service: core::default::Default::default(),
            service_request_quota_uri: core::default::Default::default(),
        }
    }
}
pub struct DataCloudQuotasQuotaInfosQuotaInfosElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudQuotasQuotaInfosQuotaInfosElRef {
    fn new(shared: StackShared, base: String) -> DataCloudQuotasQuotaInfosQuotaInfosElRef {
        DataCloudQuotasQuotaInfosQuotaInfosElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudQuotasQuotaInfosQuotaInfosElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container_type` after provisioning.\n"]
    pub fn container_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }
    #[doc = "Get a reference to the value of field `dimensions_infos` after provisioning.\n"]
    pub fn dimensions_infos(
        &self,
    ) -> ListRef<DataCloudQuotasQuotaInfosQuotaInfosElDimensionsInfosElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dimensions_infos", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `is_concurrent` after provisioning.\n"]
    pub fn is_concurrent(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_concurrent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `is_fixed` after provisioning.\n"]
    pub fn is_fixed(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_fixed", self.base))
    }
    #[doc = "Get a reference to the value of field `is_precise` after provisioning.\n"]
    pub fn is_precise(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_precise", self.base))
    }
    #[doc = "Get a reference to the value of field `metric` after provisioning.\n"]
    pub fn metric(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric", self.base))
    }
    #[doc = "Get a reference to the value of field `metric_display_name` after provisioning.\n"]
    pub fn metric_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric_display_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metric_unit` after provisioning.\n"]
    pub fn metric_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_unit", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `quota_display_name` after provisioning.\n"]
    pub fn quota_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quota_display_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `quota_id` after provisioning.\n"]
    pub fn quota_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.quota_id", self.base))
    }
    #[doc = "Get a reference to the value of field `quota_increase_eligibility` after provisioning.\n"]
    pub fn quota_increase_eligibility(
        &self,
    ) -> ListRef<DataCloudQuotasQuotaInfosQuotaInfosElQuotaIncreaseEligibilityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.quota_increase_eligibility", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `refresh_interval` after provisioning.\n"]
    pub fn refresh_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.refresh_interval", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
    #[doc = "Get a reference to the value of field `service_request_quota_uri` after provisioning.\n"]
    pub fn service_request_quota_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_request_quota_uri", self.base),
        )
    }
}
