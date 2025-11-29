use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Route53RecordData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_overwrite: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multivalue_answer_routing_policy: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    records: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    set_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<f64>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    zone_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<Vec<Route53RecordAliasEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_routing_policy: Option<Vec<Route53RecordCidrRoutingPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failover_routing_policy: Option<Vec<Route53RecordFailoverRoutingPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    geolocation_routing_policy: Option<Vec<Route53RecordGeolocationRoutingPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    geoproximity_routing_policy: Option<Vec<Route53RecordGeoproximityRoutingPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    latency_routing_policy: Option<Vec<Route53RecordLatencyRoutingPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<Route53RecordTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_routing_policy: Option<Vec<Route53RecordWeightedRoutingPolicyEl>>,
    dynamic: Route53RecordDynamic,
}

struct Route53Record_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Route53RecordData>,
}

#[derive(Clone)]
pub struct Route53Record(Rc<Route53Record_>);

impl Route53Record {
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

    #[doc = "Set the field `allow_overwrite`.\n"]
    pub fn set_allow_overwrite(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_overwrite = Some(v.into());
        self
    }

    #[doc = "Set the field `health_check_id`.\n"]
    pub fn set_health_check_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().health_check_id = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `multivalue_answer_routing_policy`.\n"]
    pub fn set_multivalue_answer_routing_policy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().multivalue_answer_routing_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `records`.\n"]
    pub fn set_records(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().records = Some(v.into());
        self
    }

    #[doc = "Set the field `set_identifier`.\n"]
    pub fn set_set_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().set_identifier = Some(v.into());
        self
    }

    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ttl = Some(v.into());
        self
    }

    #[doc = "Set the field `alias`.\n"]
    pub fn set_alias(self, v: impl Into<BlockAssignable<Route53RecordAliasEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().alias = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.alias = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cidr_routing_policy`.\n"]
    pub fn set_cidr_routing_policy(self, v: impl Into<BlockAssignable<Route53RecordCidrRoutingPolicyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cidr_routing_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cidr_routing_policy = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `failover_routing_policy`.\n"]
    pub fn set_failover_routing_policy(
        self,
        v: impl Into<BlockAssignable<Route53RecordFailoverRoutingPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().failover_routing_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.failover_routing_policy = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `geolocation_routing_policy`.\n"]
    pub fn set_geolocation_routing_policy(
        self,
        v: impl Into<BlockAssignable<Route53RecordGeolocationRoutingPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().geolocation_routing_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.geolocation_routing_policy = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `geoproximity_routing_policy`.\n"]
    pub fn set_geoproximity_routing_policy(
        self,
        v: impl Into<BlockAssignable<Route53RecordGeoproximityRoutingPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().geoproximity_routing_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.geoproximity_routing_policy = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `latency_routing_policy`.\n"]
    pub fn set_latency_routing_policy(self, v: impl Into<BlockAssignable<Route53RecordLatencyRoutingPolicyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().latency_routing_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.latency_routing_policy = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<Route53RecordTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Set the field `weighted_routing_policy`.\n"]
    pub fn set_weighted_routing_policy(
        self,
        v: impl Into<BlockAssignable<Route53RecordWeightedRoutingPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().weighted_routing_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.weighted_routing_policy = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `allow_overwrite` after provisioning.\n"]
    pub fn allow_overwrite(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_overwrite", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fqdn` after provisioning.\n"]
    pub fn fqdn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fqdn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_id` after provisioning.\n"]
    pub fn health_check_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_check_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multivalue_answer_routing_policy` after provisioning.\n"]
    pub fn multivalue_answer_routing_policy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.multivalue_answer_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `records` after provisioning.\n"]
    pub fn records(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.records", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `set_identifier` after provisioning.\n"]
    pub fn set_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.set_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> ListRef<Route53RecordAliasElRef> {
        ListRef::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cidr_routing_policy` after provisioning.\n"]
    pub fn cidr_routing_policy(&self) -> ListRef<Route53RecordCidrRoutingPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cidr_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failover_routing_policy` after provisioning.\n"]
    pub fn failover_routing_policy(&self) -> ListRef<Route53RecordFailoverRoutingPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.failover_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `geolocation_routing_policy` after provisioning.\n"]
    pub fn geolocation_routing_policy(&self) -> ListRef<Route53RecordGeolocationRoutingPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.geolocation_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `geoproximity_routing_policy` after provisioning.\n"]
    pub fn geoproximity_routing_policy(&self) -> ListRef<Route53RecordGeoproximityRoutingPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.geoproximity_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `latency_routing_policy` after provisioning.\n"]
    pub fn latency_routing_policy(&self) -> ListRef<Route53RecordLatencyRoutingPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.latency_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Route53RecordTimeoutsElRef {
        Route53RecordTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `weighted_routing_policy` after provisioning.\n"]
    pub fn weighted_routing_policy(&self) -> ListRef<Route53RecordWeightedRoutingPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.weighted_routing_policy", self.extract_ref()))
    }
}

impl Referable for Route53Record {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Route53Record { }

impl ToListMappable for Route53Record {
    type O = ListRef<Route53RecordRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Route53Record_ {
    fn extract_resource_type(&self) -> String {
        "aws_route53_record".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRoute53Record {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
    #[doc = ""]
    pub zone_id: PrimField<String>,
}

impl BuildRoute53Record {
    pub fn build(self, stack: &mut Stack) -> Route53Record {
        let out = Route53Record(Rc::new(Route53Record_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Route53RecordData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allow_overwrite: core::default::Default::default(),
                health_check_id: core::default::Default::default(),
                id: core::default::Default::default(),
                multivalue_answer_routing_policy: core::default::Default::default(),
                name: self.name,
                records: core::default::Default::default(),
                set_identifier: core::default::Default::default(),
                ttl: core::default::Default::default(),
                type_: self.type_,
                zone_id: self.zone_id,
                alias: core::default::Default::default(),
                cidr_routing_policy: core::default::Default::default(),
                failover_routing_policy: core::default::Default::default(),
                geolocation_routing_policy: core::default::Default::default(),
                geoproximity_routing_policy: core::default::Default::default(),
                latency_routing_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                weighted_routing_policy: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Route53RecordRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53RecordRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl Route53RecordRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_overwrite` after provisioning.\n"]
    pub fn allow_overwrite(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_overwrite", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fqdn` after provisioning.\n"]
    pub fn fqdn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fqdn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_id` after provisioning.\n"]
    pub fn health_check_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_check_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multivalue_answer_routing_policy` after provisioning.\n"]
    pub fn multivalue_answer_routing_policy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.multivalue_answer_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `records` after provisioning.\n"]
    pub fn records(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.records", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `set_identifier` after provisioning.\n"]
    pub fn set_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.set_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> ListRef<Route53RecordAliasElRef> {
        ListRef::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cidr_routing_policy` after provisioning.\n"]
    pub fn cidr_routing_policy(&self) -> ListRef<Route53RecordCidrRoutingPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cidr_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failover_routing_policy` after provisioning.\n"]
    pub fn failover_routing_policy(&self) -> ListRef<Route53RecordFailoverRoutingPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.failover_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `geolocation_routing_policy` after provisioning.\n"]
    pub fn geolocation_routing_policy(&self) -> ListRef<Route53RecordGeolocationRoutingPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.geolocation_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `geoproximity_routing_policy` after provisioning.\n"]
    pub fn geoproximity_routing_policy(&self) -> ListRef<Route53RecordGeoproximityRoutingPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.geoproximity_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `latency_routing_policy` after provisioning.\n"]
    pub fn latency_routing_policy(&self) -> ListRef<Route53RecordLatencyRoutingPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.latency_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Route53RecordTimeoutsElRef {
        Route53RecordTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `weighted_routing_policy` after provisioning.\n"]
    pub fn weighted_routing_policy(&self) -> ListRef<Route53RecordWeightedRoutingPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.weighted_routing_policy", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Route53RecordAliasEl {
    evaluate_target_health: PrimField<bool>,
    name: PrimField<String>,
    zone_id: PrimField<String>,
}

impl Route53RecordAliasEl { }

impl ToListMappable for Route53RecordAliasEl {
    type O = BlockAssignable<Route53RecordAliasEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordAliasEl {
    #[doc = ""]
    pub evaluate_target_health: PrimField<bool>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub zone_id: PrimField<String>,
}

impl BuildRoute53RecordAliasEl {
    pub fn build(self) -> Route53RecordAliasEl {
        Route53RecordAliasEl {
            evaluate_target_health: self.evaluate_target_health,
            name: self.name,
            zone_id: self.zone_id,
        }
    }
}

pub struct Route53RecordAliasElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53RecordAliasElRef {
    fn new(shared: StackShared, base: String) -> Route53RecordAliasElRef {
        Route53RecordAliasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordAliasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `evaluate_target_health` after provisioning.\n"]
    pub fn evaluate_target_health(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.evaluate_target_health", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53RecordCidrRoutingPolicyEl {
    collection_id: PrimField<String>,
    location_name: PrimField<String>,
}

impl Route53RecordCidrRoutingPolicyEl { }

impl ToListMappable for Route53RecordCidrRoutingPolicyEl {
    type O = BlockAssignable<Route53RecordCidrRoutingPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordCidrRoutingPolicyEl {
    #[doc = ""]
    pub collection_id: PrimField<String>,
    #[doc = ""]
    pub location_name: PrimField<String>,
}

impl BuildRoute53RecordCidrRoutingPolicyEl {
    pub fn build(self) -> Route53RecordCidrRoutingPolicyEl {
        Route53RecordCidrRoutingPolicyEl {
            collection_id: self.collection_id,
            location_name: self.location_name,
        }
    }
}

pub struct Route53RecordCidrRoutingPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53RecordCidrRoutingPolicyElRef {
    fn new(shared: StackShared, base: String) -> Route53RecordCidrRoutingPolicyElRef {
        Route53RecordCidrRoutingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordCidrRoutingPolicyElRef {
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
pub struct Route53RecordFailoverRoutingPolicyEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl Route53RecordFailoverRoutingPolicyEl { }

impl ToListMappable for Route53RecordFailoverRoutingPolicyEl {
    type O = BlockAssignable<Route53RecordFailoverRoutingPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordFailoverRoutingPolicyEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildRoute53RecordFailoverRoutingPolicyEl {
    pub fn build(self) -> Route53RecordFailoverRoutingPolicyEl {
        Route53RecordFailoverRoutingPolicyEl { type_: self.type_ }
    }
}

pub struct Route53RecordFailoverRoutingPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53RecordFailoverRoutingPolicyElRef {
    fn new(shared: StackShared, base: String) -> Route53RecordFailoverRoutingPolicyElRef {
        Route53RecordFailoverRoutingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordFailoverRoutingPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53RecordGeolocationRoutingPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    continent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subdivision: Option<PrimField<String>>,
}

impl Route53RecordGeolocationRoutingPolicyEl {
    #[doc = "Set the field `continent`.\n"]
    pub fn set_continent(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.continent = Some(v.into());
        self
    }

    #[doc = "Set the field `country`.\n"]
    pub fn set_country(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.country = Some(v.into());
        self
    }

    #[doc = "Set the field `subdivision`.\n"]
    pub fn set_subdivision(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subdivision = Some(v.into());
        self
    }
}

impl ToListMappable for Route53RecordGeolocationRoutingPolicyEl {
    type O = BlockAssignable<Route53RecordGeolocationRoutingPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordGeolocationRoutingPolicyEl {}

impl BuildRoute53RecordGeolocationRoutingPolicyEl {
    pub fn build(self) -> Route53RecordGeolocationRoutingPolicyEl {
        Route53RecordGeolocationRoutingPolicyEl {
            continent: core::default::Default::default(),
            country: core::default::Default::default(),
            subdivision: core::default::Default::default(),
        }
    }
}

pub struct Route53RecordGeolocationRoutingPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53RecordGeolocationRoutingPolicyElRef {
    fn new(shared: StackShared, base: String) -> Route53RecordGeolocationRoutingPolicyElRef {
        Route53RecordGeolocationRoutingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordGeolocationRoutingPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `continent` after provisioning.\n"]
    pub fn continent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.continent", self.base))
    }

    #[doc = "Get a reference to the value of field `country` after provisioning.\n"]
    pub fn country(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.country", self.base))
    }

    #[doc = "Get a reference to the value of field `subdivision` after provisioning.\n"]
    pub fn subdivision(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subdivision", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53RecordGeoproximityRoutingPolicyElCoordinatesEl {
    latitude: PrimField<String>,
    longitude: PrimField<String>,
}

impl Route53RecordGeoproximityRoutingPolicyElCoordinatesEl { }

impl ToListMappable for Route53RecordGeoproximityRoutingPolicyElCoordinatesEl {
    type O = BlockAssignable<Route53RecordGeoproximityRoutingPolicyElCoordinatesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordGeoproximityRoutingPolicyElCoordinatesEl {
    #[doc = ""]
    pub latitude: PrimField<String>,
    #[doc = ""]
    pub longitude: PrimField<String>,
}

impl BuildRoute53RecordGeoproximityRoutingPolicyElCoordinatesEl {
    pub fn build(self) -> Route53RecordGeoproximityRoutingPolicyElCoordinatesEl {
        Route53RecordGeoproximityRoutingPolicyElCoordinatesEl {
            latitude: self.latitude,
            longitude: self.longitude,
        }
    }
}

pub struct Route53RecordGeoproximityRoutingPolicyElCoordinatesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53RecordGeoproximityRoutingPolicyElCoordinatesElRef {
    fn new(shared: StackShared, base: String) -> Route53RecordGeoproximityRoutingPolicyElCoordinatesElRef {
        Route53RecordGeoproximityRoutingPolicyElCoordinatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordGeoproximityRoutingPolicyElCoordinatesElRef {
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
struct Route53RecordGeoproximityRoutingPolicyElDynamic {
    coordinates: Option<DynamicBlock<Route53RecordGeoproximityRoutingPolicyElCoordinatesEl>>,
}

#[derive(Serialize)]
pub struct Route53RecordGeoproximityRoutingPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bias: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_zone_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coordinates: Option<Vec<Route53RecordGeoproximityRoutingPolicyElCoordinatesEl>>,
    dynamic: Route53RecordGeoproximityRoutingPolicyElDynamic,
}

impl Route53RecordGeoproximityRoutingPolicyEl {
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
        v: impl Into<BlockAssignable<Route53RecordGeoproximityRoutingPolicyElCoordinatesEl>>,
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

impl ToListMappable for Route53RecordGeoproximityRoutingPolicyEl {
    type O = BlockAssignable<Route53RecordGeoproximityRoutingPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordGeoproximityRoutingPolicyEl {}

impl BuildRoute53RecordGeoproximityRoutingPolicyEl {
    pub fn build(self) -> Route53RecordGeoproximityRoutingPolicyEl {
        Route53RecordGeoproximityRoutingPolicyEl {
            aws_region: core::default::Default::default(),
            bias: core::default::Default::default(),
            local_zone_group: core::default::Default::default(),
            coordinates: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Route53RecordGeoproximityRoutingPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53RecordGeoproximityRoutingPolicyElRef {
    fn new(shared: StackShared, base: String) -> Route53RecordGeoproximityRoutingPolicyElRef {
        Route53RecordGeoproximityRoutingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordGeoproximityRoutingPolicyElRef {
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
}

#[derive(Serialize)]
pub struct Route53RecordLatencyRoutingPolicyEl {
    region: PrimField<String>,
}

impl Route53RecordLatencyRoutingPolicyEl { }

impl ToListMappable for Route53RecordLatencyRoutingPolicyEl {
    type O = BlockAssignable<Route53RecordLatencyRoutingPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordLatencyRoutingPolicyEl {
    #[doc = ""]
    pub region: PrimField<String>,
}

impl BuildRoute53RecordLatencyRoutingPolicyEl {
    pub fn build(self) -> Route53RecordLatencyRoutingPolicyEl {
        Route53RecordLatencyRoutingPolicyEl { region: self.region }
    }
}

pub struct Route53RecordLatencyRoutingPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53RecordLatencyRoutingPolicyElRef {
    fn new(shared: StackShared, base: String) -> Route53RecordLatencyRoutingPolicyElRef {
        Route53RecordLatencyRoutingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordLatencyRoutingPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53RecordTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl Route53RecordTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for Route53RecordTimeoutsEl {
    type O = BlockAssignable<Route53RecordTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordTimeoutsEl {}

impl BuildRoute53RecordTimeoutsEl {
    pub fn build(self) -> Route53RecordTimeoutsEl {
        Route53RecordTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct Route53RecordTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53RecordTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> Route53RecordTimeoutsElRef {
        Route53RecordTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53RecordWeightedRoutingPolicyEl {
    weight: PrimField<f64>,
}

impl Route53RecordWeightedRoutingPolicyEl { }

impl ToListMappable for Route53RecordWeightedRoutingPolicyEl {
    type O = BlockAssignable<Route53RecordWeightedRoutingPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordWeightedRoutingPolicyEl {
    #[doc = ""]
    pub weight: PrimField<f64>,
}

impl BuildRoute53RecordWeightedRoutingPolicyEl {
    pub fn build(self) -> Route53RecordWeightedRoutingPolicyEl {
        Route53RecordWeightedRoutingPolicyEl { weight: self.weight }
    }
}

pub struct Route53RecordWeightedRoutingPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53RecordWeightedRoutingPolicyElRef {
    fn new(shared: StackShared, base: String) -> Route53RecordWeightedRoutingPolicyElRef {
        Route53RecordWeightedRoutingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordWeightedRoutingPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize, Default)]
struct Route53RecordDynamic {
    alias: Option<DynamicBlock<Route53RecordAliasEl>>,
    cidr_routing_policy: Option<DynamicBlock<Route53RecordCidrRoutingPolicyEl>>,
    failover_routing_policy: Option<DynamicBlock<Route53RecordFailoverRoutingPolicyEl>>,
    geolocation_routing_policy: Option<DynamicBlock<Route53RecordGeolocationRoutingPolicyEl>>,
    geoproximity_routing_policy: Option<DynamicBlock<Route53RecordGeoproximityRoutingPolicyEl>>,
    latency_routing_policy: Option<DynamicBlock<Route53RecordLatencyRoutingPolicyEl>>,
    weighted_routing_policy: Option<DynamicBlock<Route53RecordWeightedRoutingPolicyEl>>,
}
