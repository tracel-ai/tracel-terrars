use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataRoute53RecordsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_regex: Option<PrimField<String>>,
    zone_id: PrimField<String>,
}
struct DataRoute53Records_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRoute53RecordsData>,
}
#[derive(Clone)]
pub struct DataRoute53Records(Rc<DataRoute53Records_>);
impl DataRoute53Records {
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
    #[doc = "Set the field `name_regex`.\n"]
    pub fn set_name_regex(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_regex = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `name_regex` after provisioning.\n"]
    pub fn name_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_regex", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_record_sets` after provisioning.\n"]
    pub fn resource_record_sets(&self) -> ListRef<DataRoute53RecordsResourceRecordSetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_record_sets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone_id", self.extract_ref()),
        )
    }
}
impl Referable for DataRoute53Records {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataRoute53Records {}
impl ToListMappable for DataRoute53Records {
    type O = ListRef<DataRoute53RecordsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataRoute53Records_ {
    fn extract_datasource_type(&self) -> String {
        "aws_route53_records".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataRoute53Records {
    pub tf_id: String,
    #[doc = ""]
    pub zone_id: PrimField<String>,
}
impl BuildDataRoute53Records {
    pub fn build(self, stack: &mut Stack) -> DataRoute53Records {
        let out = DataRoute53Records(Rc::new(DataRoute53Records_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRoute53RecordsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                name_regex: core::default::Default::default(),
                zone_id: self.zone_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataRoute53RecordsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRoute53RecordsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataRoute53RecordsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `name_regex` after provisioning.\n"]
    pub fn name_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_regex", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_record_sets` after provisioning.\n"]
    pub fn resource_record_sets(&self) -> ListRef<DataRoute53RecordsResourceRecordSetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_record_sets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone_id", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataRoute53RecordsResourceRecordSetsElAliasTarget {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluate_target_health: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hosted_zone_id: Option<PrimField<String>>,
}
impl DataRoute53RecordsResourceRecordSetsElAliasTarget {
    #[doc = "Set the field `dns_name`.\n"]
    pub fn set_dns_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dns_name = Some(v.into());
        self
    }
    #[doc = "Set the field `evaluate_target_health`.\n"]
    pub fn set_evaluate_target_health(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.evaluate_target_health = Some(v.into());
        self
    }
    #[doc = "Set the field `hosted_zone_id`.\n"]
    pub fn set_hosted_zone_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hosted_zone_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataRoute53RecordsResourceRecordSetsElAliasTarget {
    type O = BlockAssignable<DataRoute53RecordsResourceRecordSetsElAliasTarget>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRoute53RecordsResourceRecordSetsElAliasTarget {}
impl BuildDataRoute53RecordsResourceRecordSetsElAliasTarget {
    pub fn build(self) -> DataRoute53RecordsResourceRecordSetsElAliasTarget {
        DataRoute53RecordsResourceRecordSetsElAliasTarget {
            dns_name: core::default::Default::default(),
            evaluate_target_health: core::default::Default::default(),
            hosted_zone_id: core::default::Default::default(),
        }
    }
}
pub struct DataRoute53RecordsResourceRecordSetsElAliasTargetRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRoute53RecordsResourceRecordSetsElAliasTargetRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRoute53RecordsResourceRecordSetsElAliasTargetRef {
        DataRoute53RecordsResourceRecordSetsElAliasTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRoute53RecordsResourceRecordSetsElAliasTargetRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }
    #[doc = "Get a reference to the value of field `evaluate_target_health` after provisioning.\n"]
    pub fn evaluate_target_health(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.evaluate_target_health", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hosted_zone_id", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataRoute53RecordsResourceRecordSetsElCidrRoutingConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    collection_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location_name: Option<PrimField<String>>,
}
impl DataRoute53RecordsResourceRecordSetsElCidrRoutingConfig {
    #[doc = "Set the field `collection_id`.\n"]
    pub fn set_collection_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.collection_id = Some(v.into());
        self
    }
    #[doc = "Set the field `location_name`.\n"]
    pub fn set_location_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataRoute53RecordsResourceRecordSetsElCidrRoutingConfig {
    type O = BlockAssignable<DataRoute53RecordsResourceRecordSetsElCidrRoutingConfig>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRoute53RecordsResourceRecordSetsElCidrRoutingConfig {}
impl BuildDataRoute53RecordsResourceRecordSetsElCidrRoutingConfig {
    pub fn build(self) -> DataRoute53RecordsResourceRecordSetsElCidrRoutingConfig {
        DataRoute53RecordsResourceRecordSetsElCidrRoutingConfig {
            collection_id: core::default::Default::default(),
            location_name: core::default::Default::default(),
        }
    }
}
pub struct DataRoute53RecordsResourceRecordSetsElCidrRoutingConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRoute53RecordsResourceRecordSetsElCidrRoutingConfigRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRoute53RecordsResourceRecordSetsElCidrRoutingConfigRef {
        DataRoute53RecordsResourceRecordSetsElCidrRoutingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRoute53RecordsResourceRecordSetsElCidrRoutingConfigRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `collection_id` after provisioning.\n"]
    pub fn collection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collection_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `location_name` after provisioning.\n"]
    pub fn location_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataRoute53RecordsResourceRecordSetsElGeolocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    continent_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subdivision_code: Option<PrimField<String>>,
}
impl DataRoute53RecordsResourceRecordSetsElGeolocation {
    #[doc = "Set the field `continent_code`.\n"]
    pub fn set_continent_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.continent_code = Some(v.into());
        self
    }
    #[doc = "Set the field `country_code`.\n"]
    pub fn set_country_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.country_code = Some(v.into());
        self
    }
    #[doc = "Set the field `subdivision_code`.\n"]
    pub fn set_subdivision_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subdivision_code = Some(v.into());
        self
    }
}
impl ToListMappable for DataRoute53RecordsResourceRecordSetsElGeolocation {
    type O = BlockAssignable<DataRoute53RecordsResourceRecordSetsElGeolocation>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRoute53RecordsResourceRecordSetsElGeolocation {}
impl BuildDataRoute53RecordsResourceRecordSetsElGeolocation {
    pub fn build(self) -> DataRoute53RecordsResourceRecordSetsElGeolocation {
        DataRoute53RecordsResourceRecordSetsElGeolocation {
            continent_code: core::default::Default::default(),
            country_code: core::default::Default::default(),
            subdivision_code: core::default::Default::default(),
        }
    }
}
pub struct DataRoute53RecordsResourceRecordSetsElGeolocationRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRoute53RecordsResourceRecordSetsElGeolocationRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRoute53RecordsResourceRecordSetsElGeolocationRef {
        DataRoute53RecordsResourceRecordSetsElGeolocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRoute53RecordsResourceRecordSetsElGeolocationRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `continent_code` after provisioning.\n"]
    pub fn continent_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.continent_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `country_code` after provisioning.\n"]
    pub fn country_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.base))
    }
    #[doc = "Get a reference to the value of field `subdivision_code` after provisioning.\n"]
    pub fn subdivision_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subdivision_code", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataRoute53RecordsResourceRecordSetsElGeoproximityLocationCoordinates {
    #[serde(skip_serializing_if = "Option::is_none")]
    latitude: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    longitude: Option<PrimField<String>>,
}
impl DataRoute53RecordsResourceRecordSetsElGeoproximityLocationCoordinates {
    #[doc = "Set the field `latitude`.\n"]
    pub fn set_latitude(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.latitude = Some(v.into());
        self
    }
    #[doc = "Set the field `longitude`.\n"]
    pub fn set_longitude(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.longitude = Some(v.into());
        self
    }
}
impl ToListMappable for DataRoute53RecordsResourceRecordSetsElGeoproximityLocationCoordinates {
    type O = BlockAssignable<DataRoute53RecordsResourceRecordSetsElGeoproximityLocationCoordinates>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRoute53RecordsResourceRecordSetsElGeoproximityLocationCoordinates {}
impl BuildDataRoute53RecordsResourceRecordSetsElGeoproximityLocationCoordinates {
    pub fn build(self) -> DataRoute53RecordsResourceRecordSetsElGeoproximityLocationCoordinates {
        DataRoute53RecordsResourceRecordSetsElGeoproximityLocationCoordinates {
            latitude: core::default::Default::default(),
            longitude: core::default::Default::default(),
        }
    }
}
pub struct DataRoute53RecordsResourceRecordSetsElGeoproximityLocationCoordinatesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRoute53RecordsResourceRecordSetsElGeoproximityLocationCoordinatesRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRoute53RecordsResourceRecordSetsElGeoproximityLocationCoordinatesRef {
        DataRoute53RecordsResourceRecordSetsElGeoproximityLocationCoordinatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRoute53RecordsResourceRecordSetsElGeoproximityLocationCoordinatesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `latitude` after provisioning.\n"]
    pub fn latitude(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.latitude", self.base))
    }
    #[doc = "Get a reference to the value of field `longitude` after provisioning.\n"]
    pub fn longitude(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.longitude", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRoute53RecordsResourceRecordSetsElGeoproximityLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bias: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coordinates: Option<DataRoute53RecordsResourceRecordSetsElGeoproximityLocationCoordinates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_zone_group: Option<PrimField<String>>,
}
impl DataRoute53RecordsResourceRecordSetsElGeoproximityLocation {
    #[doc = "Set the field `aws_region`.\n"]
    pub fn set_aws_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.aws_region = Some(v.into());
        self
    }
    #[doc = "Set the field `bias`.\n"]
    pub fn set_bias(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.bias = Some(v.into());
        self
    }
    #[doc = "Set the field `coordinates`.\n"]
    pub fn set_coordinates(
        mut self,
        v: impl Into<DataRoute53RecordsResourceRecordSetsElGeoproximityLocationCoordinates>,
    ) -> Self {
        self.coordinates = Some(v.into());
        self
    }
    #[doc = "Set the field `local_zone_group`.\n"]
    pub fn set_local_zone_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_zone_group = Some(v.into());
        self
    }
}
impl ToListMappable for DataRoute53RecordsResourceRecordSetsElGeoproximityLocation {
    type O = BlockAssignable<DataRoute53RecordsResourceRecordSetsElGeoproximityLocation>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRoute53RecordsResourceRecordSetsElGeoproximityLocation {}
impl BuildDataRoute53RecordsResourceRecordSetsElGeoproximityLocation {
    pub fn build(self) -> DataRoute53RecordsResourceRecordSetsElGeoproximityLocation {
        DataRoute53RecordsResourceRecordSetsElGeoproximityLocation {
            aws_region: core::default::Default::default(),
            bias: core::default::Default::default(),
            coordinates: core::default::Default::default(),
            local_zone_group: core::default::Default::default(),
        }
    }
}
pub struct DataRoute53RecordsResourceRecordSetsElGeoproximityLocationRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRoute53RecordsResourceRecordSetsElGeoproximityLocationRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRoute53RecordsResourceRecordSetsElGeoproximityLocationRef {
        DataRoute53RecordsResourceRecordSetsElGeoproximityLocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRoute53RecordsResourceRecordSetsElGeoproximityLocationRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aws_region` after provisioning.\n"]
    pub fn aws_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_region", self.base))
    }
    #[doc = "Get a reference to the value of field `bias` after provisioning.\n"]
    pub fn bias(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.bias", self.base))
    }
    #[doc = "Get a reference to the value of field `coordinates` after provisioning.\n"]
    pub fn coordinates(
        &self,
    ) -> DataRoute53RecordsResourceRecordSetsElGeoproximityLocationCoordinatesRef {
        DataRoute53RecordsResourceRecordSetsElGeoproximityLocationCoordinatesRef::new(
            self.shared().clone(),
            format!("{}.coordinates", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_zone_group` after provisioning.\n"]
    pub fn local_zone_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_zone_group", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataRoute53RecordsResourceRecordSetsElResourceRecordsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataRoute53RecordsResourceRecordSetsElResourceRecordsEl {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DataRoute53RecordsResourceRecordSetsElResourceRecordsEl {
    type O = BlockAssignable<DataRoute53RecordsResourceRecordSetsElResourceRecordsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRoute53RecordsResourceRecordSetsElResourceRecordsEl {}
impl BuildDataRoute53RecordsResourceRecordSetsElResourceRecordsEl {
    pub fn build(self) -> DataRoute53RecordsResourceRecordSetsElResourceRecordsEl {
        DataRoute53RecordsResourceRecordSetsElResourceRecordsEl {
            value: core::default::Default::default(),
        }
    }
}
pub struct DataRoute53RecordsResourceRecordSetsElResourceRecordsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRoute53RecordsResourceRecordSetsElResourceRecordsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRoute53RecordsResourceRecordSetsElResourceRecordsElRef {
        DataRoute53RecordsResourceRecordSetsElResourceRecordsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRoute53RecordsResourceRecordSetsElResourceRecordsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRoute53RecordsResourceRecordSetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    alias_target: Option<DataRoute53RecordsResourceRecordSetsElAliasTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_routing_config: Option<DataRoute53RecordsResourceRecordSetsElCidrRoutingConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failover: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    geolocation: Option<DataRoute53RecordsResourceRecordSetsElGeolocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    geoproximity_location: Option<DataRoute53RecordsResourceRecordSetsElGeoproximityLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_value_answer: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_records: Option<ListField<DataRoute53RecordsResourceRecordSetsElResourceRecordsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    set_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    traffic_policy_instance_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}
impl DataRoute53RecordsResourceRecordSetsEl {
    #[doc = "Set the field `alias_target`.\n"]
    pub fn set_alias_target(
        mut self,
        v: impl Into<DataRoute53RecordsResourceRecordSetsElAliasTarget>,
    ) -> Self {
        self.alias_target = Some(v.into());
        self
    }
    #[doc = "Set the field `cidr_routing_config`.\n"]
    pub fn set_cidr_routing_config(
        mut self,
        v: impl Into<DataRoute53RecordsResourceRecordSetsElCidrRoutingConfig>,
    ) -> Self {
        self.cidr_routing_config = Some(v.into());
        self
    }
    #[doc = "Set the field `failover`.\n"]
    pub fn set_failover(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.failover = Some(v.into());
        self
    }
    #[doc = "Set the field `geolocation`.\n"]
    pub fn set_geolocation(
        mut self,
        v: impl Into<DataRoute53RecordsResourceRecordSetsElGeolocation>,
    ) -> Self {
        self.geolocation = Some(v.into());
        self
    }
    #[doc = "Set the field `geoproximity_location`.\n"]
    pub fn set_geoproximity_location(
        mut self,
        v: impl Into<DataRoute53RecordsResourceRecordSetsElGeoproximityLocation>,
    ) -> Self {
        self.geoproximity_location = Some(v.into());
        self
    }
    #[doc = "Set the field `health_check_id`.\n"]
    pub fn set_health_check_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.health_check_id = Some(v.into());
        self
    }
    #[doc = "Set the field `multi_value_answer`.\n"]
    pub fn set_multi_value_answer(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.multi_value_answer = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_records`.\n"]
    pub fn set_resource_records(
        mut self,
        v: impl Into<ListField<DataRoute53RecordsResourceRecordSetsElResourceRecordsEl>>,
    ) -> Self {
        self.resource_records = Some(v.into());
        self
    }
    #[doc = "Set the field `set_identifier`.\n"]
    pub fn set_set_identifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.set_identifier = Some(v.into());
        self
    }
    #[doc = "Set the field `traffic_policy_instance_id`.\n"]
    pub fn set_traffic_policy_instance_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.traffic_policy_instance_id = Some(v.into());
        self
    }
    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}
impl ToListMappable for DataRoute53RecordsResourceRecordSetsEl {
    type O = BlockAssignable<DataRoute53RecordsResourceRecordSetsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRoute53RecordsResourceRecordSetsEl {}
impl BuildDataRoute53RecordsResourceRecordSetsEl {
    pub fn build(self) -> DataRoute53RecordsResourceRecordSetsEl {
        DataRoute53RecordsResourceRecordSetsEl {
            alias_target: core::default::Default::default(),
            cidr_routing_config: core::default::Default::default(),
            failover: core::default::Default::default(),
            geolocation: core::default::Default::default(),
            geoproximity_location: core::default::Default::default(),
            health_check_id: core::default::Default::default(),
            multi_value_answer: core::default::Default::default(),
            name: core::default::Default::default(),
            region: core::default::Default::default(),
            resource_records: core::default::Default::default(),
            set_identifier: core::default::Default::default(),
            traffic_policy_instance_id: core::default::Default::default(),
            ttl: core::default::Default::default(),
            type_: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}
pub struct DataRoute53RecordsResourceRecordSetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRoute53RecordsResourceRecordSetsElRef {
    fn new(shared: StackShared, base: String) -> DataRoute53RecordsResourceRecordSetsElRef {
        DataRoute53RecordsResourceRecordSetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRoute53RecordsResourceRecordSetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `alias_target` after provisioning.\n"]
    pub fn alias_target(&self) -> DataRoute53RecordsResourceRecordSetsElAliasTargetRef {
        DataRoute53RecordsResourceRecordSetsElAliasTargetRef::new(
            self.shared().clone(),
            format!("{}.alias_target", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cidr_routing_config` after provisioning.\n"]
    pub fn cidr_routing_config(
        &self,
    ) -> DataRoute53RecordsResourceRecordSetsElCidrRoutingConfigRef {
        DataRoute53RecordsResourceRecordSetsElCidrRoutingConfigRef::new(
            self.shared().clone(),
            format!("{}.cidr_routing_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `failover` after provisioning.\n"]
    pub fn failover(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.failover", self.base))
    }
    #[doc = "Get a reference to the value of field `geolocation` after provisioning.\n"]
    pub fn geolocation(&self) -> DataRoute53RecordsResourceRecordSetsElGeolocationRef {
        DataRoute53RecordsResourceRecordSetsElGeolocationRef::new(
            self.shared().clone(),
            format!("{}.geolocation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `geoproximity_location` after provisioning.\n"]
    pub fn geoproximity_location(
        &self,
    ) -> DataRoute53RecordsResourceRecordSetsElGeoproximityLocationRef {
        DataRoute53RecordsResourceRecordSetsElGeoproximityLocationRef::new(
            self.shared().clone(),
            format!("{}.geoproximity_location", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `health_check_id` after provisioning.\n"]
    pub fn health_check_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_check_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `multi_value_answer` after provisioning.\n"]
    pub fn multi_value_answer(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multi_value_answer", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_records` after provisioning.\n"]
    pub fn resource_records(
        &self,
    ) -> ListRef<DataRoute53RecordsResourceRecordSetsElResourceRecordsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_records", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `set_identifier` after provisioning.\n"]
    pub fn set_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.set_identifier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `traffic_policy_instance_id` after provisioning.\n"]
    pub fn traffic_policy_instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.traffic_policy_instance_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}
