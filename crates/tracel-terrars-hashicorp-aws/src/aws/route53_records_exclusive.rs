use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Route53RecordsExclusiveData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    zone_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_record_set: Option<Vec<Route53RecordsExclusiveResourceRecordSetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<Route53RecordsExclusiveTimeoutsEl>,
    dynamic: Route53RecordsExclusiveDynamic,
}

struct Route53RecordsExclusive_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Route53RecordsExclusiveData>,
}

#[derive(Clone)]
pub struct Route53RecordsExclusive(Rc<Route53RecordsExclusive_>);

impl Route53RecordsExclusive {
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

    #[doc = "Set the field `resource_record_set`.\n"]
    pub fn set_resource_record_set(
        self,
        v: impl Into<BlockAssignable<Route53RecordsExclusiveResourceRecordSetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_record_set = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_record_set = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<Route53RecordsExclusiveTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Route53RecordsExclusiveTimeoutsElRef {
        Route53RecordsExclusiveTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for Route53RecordsExclusive {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Route53RecordsExclusive { }

impl ToListMappable for Route53RecordsExclusive {
    type O = ListRef<Route53RecordsExclusiveRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Route53RecordsExclusive_ {
    fn extract_resource_type(&self) -> String {
        "aws_route53_records_exclusive".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRoute53RecordsExclusive {
    pub tf_id: String,
    #[doc = ""]
    pub zone_id: PrimField<String>,
}

impl BuildRoute53RecordsExclusive {
    pub fn build(self, stack: &mut Stack) -> Route53RecordsExclusive {
        let out = Route53RecordsExclusive(Rc::new(Route53RecordsExclusive_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Route53RecordsExclusiveData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                zone_id: self.zone_id,
                resource_record_set: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Route53RecordsExclusiveRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53RecordsExclusiveRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl Route53RecordsExclusiveRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Route53RecordsExclusiveTimeoutsElRef {
        Route53RecordsExclusiveTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Route53RecordsExclusiveResourceRecordSetElAliasTargetEl {
    dns_name: PrimField<String>,
    evaluate_target_health: PrimField<bool>,
    hosted_zone_id: PrimField<String>,
}

impl Route53RecordsExclusiveResourceRecordSetElAliasTargetEl { }

impl ToListMappable for Route53RecordsExclusiveResourceRecordSetElAliasTargetEl {
    type O = BlockAssignable<Route53RecordsExclusiveResourceRecordSetElAliasTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordsExclusiveResourceRecordSetElAliasTargetEl {
    #[doc = ""]
    pub dns_name: PrimField<String>,
    #[doc = ""]
    pub evaluate_target_health: PrimField<bool>,
    #[doc = ""]
    pub hosted_zone_id: PrimField<String>,
}

impl BuildRoute53RecordsExclusiveResourceRecordSetElAliasTargetEl {
    pub fn build(self) -> Route53RecordsExclusiveResourceRecordSetElAliasTargetEl {
        Route53RecordsExclusiveResourceRecordSetElAliasTargetEl {
            dns_name: self.dns_name,
            evaluate_target_health: self.evaluate_target_health,
            hosted_zone_id: self.hosted_zone_id,
        }
    }
}

pub struct Route53RecordsExclusiveResourceRecordSetElAliasTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53RecordsExclusiveResourceRecordSetElAliasTargetElRef {
    fn new(shared: StackShared, base: String) -> Route53RecordsExclusiveResourceRecordSetElAliasTargetElRef {
        Route53RecordsExclusiveResourceRecordSetElAliasTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordsExclusiveResourceRecordSetElAliasTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluate_target_health` after provisioning.\n"]
    pub fn evaluate_target_health(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.evaluate_target_health", self.base))
    }

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53RecordsExclusiveResourceRecordSetElCidrRoutingConfigEl {
    collection_id: PrimField<String>,
    location_name: PrimField<String>,
}

impl Route53RecordsExclusiveResourceRecordSetElCidrRoutingConfigEl { }

impl ToListMappable for Route53RecordsExclusiveResourceRecordSetElCidrRoutingConfigEl {
    type O = BlockAssignable<Route53RecordsExclusiveResourceRecordSetElCidrRoutingConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordsExclusiveResourceRecordSetElCidrRoutingConfigEl {
    #[doc = ""]
    pub collection_id: PrimField<String>,
    #[doc = ""]
    pub location_name: PrimField<String>,
}

impl BuildRoute53RecordsExclusiveResourceRecordSetElCidrRoutingConfigEl {
    pub fn build(self) -> Route53RecordsExclusiveResourceRecordSetElCidrRoutingConfigEl {
        Route53RecordsExclusiveResourceRecordSetElCidrRoutingConfigEl {
            collection_id: self.collection_id,
            location_name: self.location_name,
        }
    }
}

pub struct Route53RecordsExclusiveResourceRecordSetElCidrRoutingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53RecordsExclusiveResourceRecordSetElCidrRoutingConfigElRef {
    fn new(shared: StackShared, base: String) -> Route53RecordsExclusiveResourceRecordSetElCidrRoutingConfigElRef {
        Route53RecordsExclusiveResourceRecordSetElCidrRoutingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordsExclusiveResourceRecordSetElCidrRoutingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `collection_id` after provisioning.\n"]
    pub fn collection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.collection_id", self.base))
    }

    #[doc = "Get a reference to the value of field `location_name` after provisioning.\n"]
    pub fn location_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location_name", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53RecordsExclusiveResourceRecordSetElGeolocationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    continent_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subdivision_code: Option<PrimField<String>>,
}

impl Route53RecordsExclusiveResourceRecordSetElGeolocationEl {
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

impl ToListMappable for Route53RecordsExclusiveResourceRecordSetElGeolocationEl {
    type O = BlockAssignable<Route53RecordsExclusiveResourceRecordSetElGeolocationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordsExclusiveResourceRecordSetElGeolocationEl {}

impl BuildRoute53RecordsExclusiveResourceRecordSetElGeolocationEl {
    pub fn build(self) -> Route53RecordsExclusiveResourceRecordSetElGeolocationEl {
        Route53RecordsExclusiveResourceRecordSetElGeolocationEl {
            continent_code: core::default::Default::default(),
            country_code: core::default::Default::default(),
            subdivision_code: core::default::Default::default(),
        }
    }
}

pub struct Route53RecordsExclusiveResourceRecordSetElGeolocationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53RecordsExclusiveResourceRecordSetElGeolocationElRef {
    fn new(shared: StackShared, base: String) -> Route53RecordsExclusiveResourceRecordSetElGeolocationElRef {
        Route53RecordsExclusiveResourceRecordSetElGeolocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordsExclusiveResourceRecordSetElGeolocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `continent_code` after provisioning.\n"]
    pub fn continent_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.continent_code", self.base))
    }

    #[doc = "Get a reference to the value of field `country_code` after provisioning.\n"]
    pub fn country_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.base))
    }

    #[doc = "Get a reference to the value of field `subdivision_code` after provisioning.\n"]
    pub fn subdivision_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subdivision_code", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationElCoordinatesEl {
    latitude: PrimField<String>,
    longitude: PrimField<String>,
}

impl Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationElCoordinatesEl { }

impl ToListMappable for Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationElCoordinatesEl {
    type O = BlockAssignable<Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationElCoordinatesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordsExclusiveResourceRecordSetElGeoproximityLocationElCoordinatesEl {
    #[doc = ""]
    pub latitude: PrimField<String>,
    #[doc = ""]
    pub longitude: PrimField<String>,
}

impl BuildRoute53RecordsExclusiveResourceRecordSetElGeoproximityLocationElCoordinatesEl {
    pub fn build(self) -> Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationElCoordinatesEl {
        Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationElCoordinatesEl {
            latitude: self.latitude,
            longitude: self.longitude,
        }
    }
}

pub struct Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationElCoordinatesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationElCoordinatesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationElCoordinatesElRef {
        Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationElCoordinatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationElCoordinatesElRef {
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

#[derive(Serialize, Default)]
struct Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationElDynamic {
    coordinates: Option<DynamicBlock<Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationElCoordinatesEl>>,
}

#[derive(Serialize)]
pub struct Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bias: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_zone_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coordinates: Option<Vec<Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationElCoordinatesEl>>,
    dynamic: Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationElDynamic,
}

impl Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationEl {
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

    #[doc = "Set the field `local_zone_group`.\n"]
    pub fn set_local_zone_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_zone_group = Some(v.into());
        self
    }

    #[doc = "Set the field `coordinates`.\n"]
    pub fn set_coordinates(
        mut self,
        v: impl Into<BlockAssignable<Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationElCoordinatesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.coordinates = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.coordinates = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationEl {
    type O = BlockAssignable<Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordsExclusiveResourceRecordSetElGeoproximityLocationEl {}

impl BuildRoute53RecordsExclusiveResourceRecordSetElGeoproximityLocationEl {
    pub fn build(self) -> Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationEl {
        Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationEl {
            aws_region: core::default::Default::default(),
            bias: core::default::Default::default(),
            local_zone_group: core::default::Default::default(),
            coordinates: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationElRef {
    fn new(shared: StackShared, base: String) -> Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationElRef {
        Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationElRef {
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

    #[doc = "Get a reference to the value of field `local_zone_group` after provisioning.\n"]
    pub fn local_zone_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_zone_group", self.base))
    }

    #[doc = "Get a reference to the value of field `coordinates` after provisioning.\n"]
    pub fn coordinates(
        &self,
    ) -> ListRef<Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationElCoordinatesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.coordinates", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53RecordsExclusiveResourceRecordSetElResourceRecordsEl {
    value: PrimField<String>,
}

impl Route53RecordsExclusiveResourceRecordSetElResourceRecordsEl { }

impl ToListMappable for Route53RecordsExclusiveResourceRecordSetElResourceRecordsEl {
    type O = BlockAssignable<Route53RecordsExclusiveResourceRecordSetElResourceRecordsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordsExclusiveResourceRecordSetElResourceRecordsEl {
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildRoute53RecordsExclusiveResourceRecordSetElResourceRecordsEl {
    pub fn build(self) -> Route53RecordsExclusiveResourceRecordSetElResourceRecordsEl {
        Route53RecordsExclusiveResourceRecordSetElResourceRecordsEl { value: self.value }
    }
}

pub struct Route53RecordsExclusiveResourceRecordSetElResourceRecordsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53RecordsExclusiveResourceRecordSetElResourceRecordsElRef {
    fn new(shared: StackShared, base: String) -> Route53RecordsExclusiveResourceRecordSetElResourceRecordsElRef {
        Route53RecordsExclusiveResourceRecordSetElResourceRecordsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordsExclusiveResourceRecordSetElResourceRecordsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct Route53RecordsExclusiveResourceRecordSetElDynamic {
    alias_target: Option<DynamicBlock<Route53RecordsExclusiveResourceRecordSetElAliasTargetEl>>,
    cidr_routing_config: Option<DynamicBlock<Route53RecordsExclusiveResourceRecordSetElCidrRoutingConfigEl>>,
    geolocation: Option<DynamicBlock<Route53RecordsExclusiveResourceRecordSetElGeolocationEl>>,
    geoproximity_location: Option<DynamicBlock<Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationEl>>,
    resource_records: Option<DynamicBlock<Route53RecordsExclusiveResourceRecordSetElResourceRecordsEl>>,
}

#[derive(Serialize)]
pub struct Route53RecordsExclusiveResourceRecordSetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    failover: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_value_answer: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    alias_target: Option<Vec<Route53RecordsExclusiveResourceRecordSetElAliasTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_routing_config: Option<Vec<Route53RecordsExclusiveResourceRecordSetElCidrRoutingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    geolocation: Option<Vec<Route53RecordsExclusiveResourceRecordSetElGeolocationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    geoproximity_location: Option<Vec<Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_records: Option<Vec<Route53RecordsExclusiveResourceRecordSetElResourceRecordsEl>>,
    dynamic: Route53RecordsExclusiveResourceRecordSetElDynamic,
}

impl Route53RecordsExclusiveResourceRecordSetEl {
    #[doc = "Set the field `failover`.\n"]
    pub fn set_failover(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.failover = Some(v.into());
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

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
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

    #[doc = "Set the field `alias_target`.\n"]
    pub fn set_alias_target(
        mut self,
        v: impl Into<BlockAssignable<Route53RecordsExclusiveResourceRecordSetElAliasTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.alias_target = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.alias_target = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cidr_routing_config`.\n"]
    pub fn set_cidr_routing_config(
        mut self,
        v: impl Into<BlockAssignable<Route53RecordsExclusiveResourceRecordSetElCidrRoutingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cidr_routing_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cidr_routing_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `geolocation`.\n"]
    pub fn set_geolocation(
        mut self,
        v: impl Into<BlockAssignable<Route53RecordsExclusiveResourceRecordSetElGeolocationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.geolocation = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.geolocation = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `geoproximity_location`.\n"]
    pub fn set_geoproximity_location(
        mut self,
        v: impl Into<BlockAssignable<Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.geoproximity_location = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.geoproximity_location = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_records`.\n"]
    pub fn set_resource_records(
        mut self,
        v: impl Into<BlockAssignable<Route53RecordsExclusiveResourceRecordSetElResourceRecordsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_records = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_records = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for Route53RecordsExclusiveResourceRecordSetEl {
    type O = BlockAssignable<Route53RecordsExclusiveResourceRecordSetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordsExclusiveResourceRecordSetEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildRoute53RecordsExclusiveResourceRecordSetEl {
    pub fn build(self) -> Route53RecordsExclusiveResourceRecordSetEl {
        Route53RecordsExclusiveResourceRecordSetEl {
            failover: core::default::Default::default(),
            health_check_id: core::default::Default::default(),
            multi_value_answer: core::default::Default::default(),
            name: self.name,
            region: core::default::Default::default(),
            set_identifier: core::default::Default::default(),
            traffic_policy_instance_id: core::default::Default::default(),
            ttl: core::default::Default::default(),
            type_: core::default::Default::default(),
            weight: core::default::Default::default(),
            alias_target: core::default::Default::default(),
            cidr_routing_config: core::default::Default::default(),
            geolocation: core::default::Default::default(),
            geoproximity_location: core::default::Default::default(),
            resource_records: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Route53RecordsExclusiveResourceRecordSetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53RecordsExclusiveResourceRecordSetElRef {
    fn new(shared: StackShared, base: String) -> Route53RecordsExclusiveResourceRecordSetElRef {
        Route53RecordsExclusiveResourceRecordSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordsExclusiveResourceRecordSetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failover` after provisioning.\n"]
    pub fn failover(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.failover", self.base))
    }

    #[doc = "Get a reference to the value of field `health_check_id` after provisioning.\n"]
    pub fn health_check_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_check_id", self.base))
    }

    #[doc = "Get a reference to the value of field `multi_value_answer` after provisioning.\n"]
    pub fn multi_value_answer(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.multi_value_answer", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `set_identifier` after provisioning.\n"]
    pub fn set_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.set_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `traffic_policy_instance_id` after provisioning.\n"]
    pub fn traffic_policy_instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.traffic_policy_instance_id", self.base))
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

    #[doc = "Get a reference to the value of field `alias_target` after provisioning.\n"]
    pub fn alias_target(&self) -> ListRef<Route53RecordsExclusiveResourceRecordSetElAliasTargetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.alias_target", self.base))
    }

    #[doc = "Get a reference to the value of field `cidr_routing_config` after provisioning.\n"]
    pub fn cidr_routing_config(&self) -> ListRef<Route53RecordsExclusiveResourceRecordSetElCidrRoutingConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cidr_routing_config", self.base))
    }

    #[doc = "Get a reference to the value of field `geolocation` after provisioning.\n"]
    pub fn geolocation(&self) -> ListRef<Route53RecordsExclusiveResourceRecordSetElGeolocationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.geolocation", self.base))
    }

    #[doc = "Get a reference to the value of field `geoproximity_location` after provisioning.\n"]
    pub fn geoproximity_location(&self) -> ListRef<Route53RecordsExclusiveResourceRecordSetElGeoproximityLocationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.geoproximity_location", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_records` after provisioning.\n"]
    pub fn resource_records(&self) -> ListRef<Route53RecordsExclusiveResourceRecordSetElResourceRecordsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.resource_records", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53RecordsExclusiveTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl Route53RecordsExclusiveTimeoutsEl {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for Route53RecordsExclusiveTimeoutsEl {
    type O = BlockAssignable<Route53RecordsExclusiveTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordsExclusiveTimeoutsEl {}

impl BuildRoute53RecordsExclusiveTimeoutsEl {
    pub fn build(self) -> Route53RecordsExclusiveTimeoutsEl {
        Route53RecordsExclusiveTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct Route53RecordsExclusiveTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53RecordsExclusiveTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> Route53RecordsExclusiveTimeoutsElRef {
        Route53RecordsExclusiveTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordsExclusiveTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct Route53RecordsExclusiveDynamic {
    resource_record_set: Option<DynamicBlock<Route53RecordsExclusiveResourceRecordSetEl>>,
}
