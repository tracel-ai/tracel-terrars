use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCloudQuotasQuotaInfoData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    parent: PrimField<String>,
    quota_id: PrimField<String>,
    service: PrimField<String>,
}
struct DataCloudQuotasQuotaInfo_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudQuotasQuotaInfoData>,
}
#[derive(Clone)]
pub struct DataCloudQuotasQuotaInfo(Rc<DataCloudQuotasQuotaInfo_>);
impl DataCloudQuotasQuotaInfo {
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
    #[doc = "Get a reference to the value of field `container_type` after provisioning.\n"]
    pub fn container_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dimensions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dimensions_infos` after provisioning.\n"]
    pub fn dimensions_infos(&self) -> ListRef<DataCloudQuotasQuotaInfoDimensionsInfosElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dimensions_infos", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `is_concurrent` after provisioning.\n"]
    pub fn is_concurrent(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_concurrent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `is_fixed` after provisioning.\n"]
    pub fn is_fixed(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_fixed", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `is_precise` after provisioning.\n"]
    pub fn is_precise(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_precise", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metric` after provisioning.\n"]
    pub fn metric(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metric_display_name` after provisioning.\n"]
    pub fn metric_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric_display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metric_unit` after provisioning.\n"]
    pub fn metric_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric_unit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\n"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quota_display_name` after provisioning.\n"]
    pub fn quota_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quota_display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quota_id` after provisioning.\n"]
    pub fn quota_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quota_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quota_increase_eligibility` after provisioning.\n"]
    pub fn quota_increase_eligibility(
        &self,
    ) -> ListRef<DataCloudQuotasQuotaInfoQuotaIncreaseEligibilityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.quota_increase_eligibility", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `refresh_interval` after provisioning.\n"]
    pub fn refresh_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.refresh_interval", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_request_quota_uri` after provisioning.\n"]
    pub fn service_request_quota_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_request_quota_uri", self.extract_ref()),
        )
    }
}
impl Referable for DataCloudQuotasQuotaInfo {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataCloudQuotasQuotaInfo {}
impl ToListMappable for DataCloudQuotasQuotaInfo {
    type O = ListRef<DataCloudQuotasQuotaInfoRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataCloudQuotasQuotaInfo_ {
    fn extract_datasource_type(&self) -> String {
        "google_cloud_quotas_quota_info".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCloudQuotasQuotaInfo {
    pub tf_id: String,
    #[doc = ""]
    pub parent: PrimField<String>,
    #[doc = ""]
    pub quota_id: PrimField<String>,
    #[doc = ""]
    pub service: PrimField<String>,
}
impl BuildDataCloudQuotasQuotaInfo {
    pub fn build(self, stack: &mut Stack) -> DataCloudQuotasQuotaInfo {
        let out = DataCloudQuotasQuotaInfo(Rc::new(DataCloudQuotasQuotaInfo_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCloudQuotasQuotaInfoData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                parent: self.parent,
                quota_id: self.quota_id,
                service: self.service,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataCloudQuotasQuotaInfoRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudQuotasQuotaInfoRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCloudQuotasQuotaInfoRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `container_type` after provisioning.\n"]
    pub fn container_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dimensions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dimensions_infos` after provisioning.\n"]
    pub fn dimensions_infos(&self) -> ListRef<DataCloudQuotasQuotaInfoDimensionsInfosElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dimensions_infos", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `is_concurrent` after provisioning.\n"]
    pub fn is_concurrent(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_concurrent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `is_fixed` after provisioning.\n"]
    pub fn is_fixed(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_fixed", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `is_precise` after provisioning.\n"]
    pub fn is_precise(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_precise", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metric` after provisioning.\n"]
    pub fn metric(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metric_display_name` after provisioning.\n"]
    pub fn metric_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric_display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metric_unit` after provisioning.\n"]
    pub fn metric_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric_unit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\n"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quota_display_name` after provisioning.\n"]
    pub fn quota_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quota_display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quota_id` after provisioning.\n"]
    pub fn quota_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quota_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quota_increase_eligibility` after provisioning.\n"]
    pub fn quota_increase_eligibility(
        &self,
    ) -> ListRef<DataCloudQuotasQuotaInfoQuotaIncreaseEligibilityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.quota_increase_eligibility", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `refresh_interval` after provisioning.\n"]
    pub fn refresh_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.refresh_interval", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_request_quota_uri` after provisioning.\n"]
    pub fn service_request_quota_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_request_quota_uri", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudQuotasQuotaInfoDimensionsInfosElDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataCloudQuotasQuotaInfoDimensionsInfosElDetailsEl {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudQuotasQuotaInfoDimensionsInfosElDetailsEl {
    type O = BlockAssignable<DataCloudQuotasQuotaInfoDimensionsInfosElDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudQuotasQuotaInfoDimensionsInfosElDetailsEl {}
impl BuildDataCloudQuotasQuotaInfoDimensionsInfosElDetailsEl {
    pub fn build(self) -> DataCloudQuotasQuotaInfoDimensionsInfosElDetailsEl {
        DataCloudQuotasQuotaInfoDimensionsInfosElDetailsEl {
            value: core::default::Default::default(),
        }
    }
}
pub struct DataCloudQuotasQuotaInfoDimensionsInfosElDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudQuotasQuotaInfoDimensionsInfosElDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudQuotasQuotaInfoDimensionsInfosElDetailsElRef {
        DataCloudQuotasQuotaInfoDimensionsInfosElDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudQuotasQuotaInfoDimensionsInfosElDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudQuotasQuotaInfoDimensionsInfosEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    applicable_locations: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<ListField<DataCloudQuotasQuotaInfoDimensionsInfosElDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions: Option<RecField<PrimField<String>>>,
}
impl DataCloudQuotasQuotaInfoDimensionsInfosEl {
    #[doc = "Set the field `applicable_locations`.\n"]
    pub fn set_applicable_locations(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.applicable_locations = Some(v.into());
        self
    }
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        mut self,
        v: impl Into<ListField<DataCloudQuotasQuotaInfoDimensionsInfosElDetailsEl>>,
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
impl ToListMappable for DataCloudQuotasQuotaInfoDimensionsInfosEl {
    type O = BlockAssignable<DataCloudQuotasQuotaInfoDimensionsInfosEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudQuotasQuotaInfoDimensionsInfosEl {}
impl BuildDataCloudQuotasQuotaInfoDimensionsInfosEl {
    pub fn build(self) -> DataCloudQuotasQuotaInfoDimensionsInfosEl {
        DataCloudQuotasQuotaInfoDimensionsInfosEl {
            applicable_locations: core::default::Default::default(),
            details: core::default::Default::default(),
            dimensions: core::default::Default::default(),
        }
    }
}
pub struct DataCloudQuotasQuotaInfoDimensionsInfosElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudQuotasQuotaInfoDimensionsInfosElRef {
    fn new(shared: StackShared, base: String) -> DataCloudQuotasQuotaInfoDimensionsInfosElRef {
        DataCloudQuotasQuotaInfoDimensionsInfosElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudQuotasQuotaInfoDimensionsInfosElRef {
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
    pub fn details(&self) -> ListRef<DataCloudQuotasQuotaInfoDimensionsInfosElDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.details", self.base))
    }
    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudQuotasQuotaInfoQuotaIncreaseEligibilityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ineligibility_reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_eligible: Option<PrimField<bool>>,
}
impl DataCloudQuotasQuotaInfoQuotaIncreaseEligibilityEl {
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
impl ToListMappable for DataCloudQuotasQuotaInfoQuotaIncreaseEligibilityEl {
    type O = BlockAssignable<DataCloudQuotasQuotaInfoQuotaIncreaseEligibilityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudQuotasQuotaInfoQuotaIncreaseEligibilityEl {}
impl BuildDataCloudQuotasQuotaInfoQuotaIncreaseEligibilityEl {
    pub fn build(self) -> DataCloudQuotasQuotaInfoQuotaIncreaseEligibilityEl {
        DataCloudQuotasQuotaInfoQuotaIncreaseEligibilityEl {
            ineligibility_reason: core::default::Default::default(),
            is_eligible: core::default::Default::default(),
        }
    }
}
pub struct DataCloudQuotasQuotaInfoQuotaIncreaseEligibilityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudQuotasQuotaInfoQuotaIncreaseEligibilityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudQuotasQuotaInfoQuotaIncreaseEligibilityElRef {
        DataCloudQuotasQuotaInfoQuotaIncreaseEligibilityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudQuotasQuotaInfoQuotaIncreaseEligibilityElRef {
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
