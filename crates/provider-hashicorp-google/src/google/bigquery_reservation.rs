use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BigqueryReservationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    concurrency: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_idle_slots: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reservation_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_location: Option<PrimField<String>>,
    slot_capacity: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscale: Option<Vec<BigqueryReservationAutoscaleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BigqueryReservationTimeoutsEl>,
    dynamic: BigqueryReservationDynamic,
}
struct BigqueryReservation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BigqueryReservationData>,
}
#[derive(Clone)]
pub struct BigqueryReservation(Rc<BigqueryReservation_>);
impl BigqueryReservation {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `concurrency`.\nMaximum number of queries that are allowed to run concurrently in this reservation. This is a soft limit due to asynchronous nature of the system and various optimizations for small queries. Default value is 0 which means that concurrency will be automatically set based on the reservation size."]
    pub fn set_concurrency(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().concurrency = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `edition`.\nThe edition type. Valid values are STANDARD, ENTERPRISE, ENTERPRISE_PLUS"]
    pub fn set_edition(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().edition = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_idle_slots`.\nIf false, any query using this reservation will use idle slots from other reservations within\nthe same admin project. If true, a query using this reservation will execute with the slot\ncapacity specified above at most."]
    pub fn set_ignore_idle_slots(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ignore_idle_slots = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe geographic location where the transfer config should reside.\nExamples: US, EU, asia-northeast1. The default value is US."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `reservation_group`.\nThe reservation group that this reservation belongs to."]
    pub fn set_reservation_group(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().reservation_group = Some(v.into());
        self
    }
    #[doc = "Set the field `secondary_location`.\nThe current location of the reservation's secondary replica. This field is only set for\nreservations using the managed disaster recovery feature. Users can set this in create\nreservation calls to create a failover reservation or in update reservation calls to convert\na non-failover reservation to a failover reservation(or vice versa)."]
    pub fn set_secondary_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().secondary_location = Some(v.into());
        self
    }
    #[doc = "Set the field `autoscale`.\n"]
    pub fn set_autoscale(
        self,
        v: impl Into<BlockAssignable<BigqueryReservationAutoscaleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().autoscale = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.autoscale = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BigqueryReservationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `concurrency` after provisioning.\nMaximum number of queries that are allowed to run concurrently in this reservation. This is a soft limit due to asynchronous nature of the system and various optimizations for small queries. Default value is 0 which means that concurrency will be automatically set based on the reservation size."]
    pub fn concurrency(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.concurrency", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `edition` after provisioning.\nThe edition type. Valid values are STANDARD, ENTERPRISE, ENTERPRISE_PLUS"]
    pub fn edition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.edition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ignore_idle_slots` after provisioning.\nIf false, any query using this reservation will use idle slots from other reservations within\nthe same admin project. If true, a query using this reservation will execute with the slot\ncapacity specified above at most."]
    pub fn ignore_idle_slots(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_idle_slots", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the transfer config should reside.\nExamples: US, EU, asia-northeast1. The default value is US."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the reservation. This field must only contain alphanumeric characters or dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `original_primary_location` after provisioning.\nThe location where the reservation was originally created. This is set only during the\nfailover reservation's creation. All billing charges for the failover reservation will be\napplied to this location."]
    pub fn original_primary_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.original_primary_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `primary_location` after provisioning.\nThe current location of the reservation's primary replica. This field is only set for\nreservations using the managed disaster recovery feature."]
    pub fn primary_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replication_status` after provisioning.\nThe Disaster Recovery(DR) replication status of the reservation. This is only available for\nthe primary replicas of DR/failover reservations and provides information about the both the\nstaleness of the secondary and the last error encountered while trying to replicate changes\nfrom the primary to the secondary. If this field is blank, it means that the reservation is\neither not a DR reservation or the reservation is a DR secondary or that any replication\noperations on the reservation have succeeded."]
    pub fn replication_status(&self) -> ListRef<BigqueryReservationReplicationStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replication_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reservation_group` after provisioning.\nThe reservation group that this reservation belongs to."]
    pub fn reservation_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reservation_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_location` after provisioning.\nThe current location of the reservation's secondary replica. This field is only set for\nreservations using the managed disaster recovery feature. Users can set this in create\nreservation calls to create a failover reservation or in update reservation calls to convert\na non-failover reservation to a failover reservation(or vice versa)."]
    pub fn secondary_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secondary_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `slot_capacity` after provisioning.\nMinimum slots available to this reservation. A slot is a unit of computational power in BigQuery, and serves as the\nunit of parallelism. Queries using this reservation might use more slots during runtime if ignoreIdleSlots is set to false."]
    pub fn slot_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.slot_capacity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `autoscale` after provisioning.\n"]
    pub fn autoscale(&self) -> ListRef<BigqueryReservationAutoscaleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscale", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryReservationTimeoutsElRef {
        BigqueryReservationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BigqueryReservation {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BigqueryReservation {}
impl ToListMappable for BigqueryReservation {
    type O = ListRef<BigqueryReservationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BigqueryReservation_ {
    fn extract_resource_type(&self) -> String {
        "google_bigquery_reservation".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBigqueryReservation {
    pub tf_id: String,
    #[doc = "The name of the reservation. This field must only contain alphanumeric characters or dash."]
    pub name: PrimField<String>,
    #[doc = "Minimum slots available to this reservation. A slot is a unit of computational power in BigQuery, and serves as the\nunit of parallelism. Queries using this reservation might use more slots during runtime if ignoreIdleSlots is set to false."]
    pub slot_capacity: PrimField<f64>,
}
impl BuildBigqueryReservation {
    pub fn build(self, stack: &mut Stack) -> BigqueryReservation {
        let out = BigqueryReservation(Rc::new(BigqueryReservation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BigqueryReservationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                concurrency: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                edition: core::default::Default::default(),
                id: core::default::Default::default(),
                ignore_idle_slots: core::default::Default::default(),
                location: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                reservation_group: core::default::Default::default(),
                secondary_location: core::default::Default::default(),
                slot_capacity: self.slot_capacity,
                autoscale: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BigqueryReservationRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryReservationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BigqueryReservationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `concurrency` after provisioning.\nMaximum number of queries that are allowed to run concurrently in this reservation. This is a soft limit due to asynchronous nature of the system and various optimizations for small queries. Default value is 0 which means that concurrency will be automatically set based on the reservation size."]
    pub fn concurrency(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.concurrency", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `edition` after provisioning.\nThe edition type. Valid values are STANDARD, ENTERPRISE, ENTERPRISE_PLUS"]
    pub fn edition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.edition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ignore_idle_slots` after provisioning.\nIf false, any query using this reservation will use idle slots from other reservations within\nthe same admin project. If true, a query using this reservation will execute with the slot\ncapacity specified above at most."]
    pub fn ignore_idle_slots(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_idle_slots", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the transfer config should reside.\nExamples: US, EU, asia-northeast1. The default value is US."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the reservation. This field must only contain alphanumeric characters or dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `original_primary_location` after provisioning.\nThe location where the reservation was originally created. This is set only during the\nfailover reservation's creation. All billing charges for the failover reservation will be\napplied to this location."]
    pub fn original_primary_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.original_primary_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `primary_location` after provisioning.\nThe current location of the reservation's primary replica. This field is only set for\nreservations using the managed disaster recovery feature."]
    pub fn primary_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replication_status` after provisioning.\nThe Disaster Recovery(DR) replication status of the reservation. This is only available for\nthe primary replicas of DR/failover reservations and provides information about the both the\nstaleness of the secondary and the last error encountered while trying to replicate changes\nfrom the primary to the secondary. If this field is blank, it means that the reservation is\neither not a DR reservation or the reservation is a DR secondary or that any replication\noperations on the reservation have succeeded."]
    pub fn replication_status(&self) -> ListRef<BigqueryReservationReplicationStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replication_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reservation_group` after provisioning.\nThe reservation group that this reservation belongs to."]
    pub fn reservation_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reservation_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_location` after provisioning.\nThe current location of the reservation's secondary replica. This field is only set for\nreservations using the managed disaster recovery feature. Users can set this in create\nreservation calls to create a failover reservation or in update reservation calls to convert\na non-failover reservation to a failover reservation(or vice versa)."]
    pub fn secondary_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secondary_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `slot_capacity` after provisioning.\nMinimum slots available to this reservation. A slot is a unit of computational power in BigQuery, and serves as the\nunit of parallelism. Queries using this reservation might use more slots during runtime if ignoreIdleSlots is set to false."]
    pub fn slot_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.slot_capacity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `autoscale` after provisioning.\n"]
    pub fn autoscale(&self) -> ListRef<BigqueryReservationAutoscaleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscale", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryReservationTimeoutsElRef {
        BigqueryReservationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryReservationReplicationStatusElErrorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl BigqueryReservationReplicationStatusElErrorEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryReservationReplicationStatusElErrorEl {
    type O = BlockAssignable<BigqueryReservationReplicationStatusElErrorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryReservationReplicationStatusElErrorEl {}
impl BuildBigqueryReservationReplicationStatusElErrorEl {
    pub fn build(self) -> BigqueryReservationReplicationStatusElErrorEl {
        BigqueryReservationReplicationStatusElErrorEl {
            code: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct BigqueryReservationReplicationStatusElErrorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryReservationReplicationStatusElErrorElRef {
    fn new(shared: StackShared, base: String) -> BigqueryReservationReplicationStatusElErrorElRef {
        BigqueryReservationReplicationStatusElErrorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryReservationReplicationStatusElErrorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryReservationReplicationStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<ListField<BigqueryReservationReplicationStatusElErrorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_error_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_replication_time: Option<PrimField<String>>,
}
impl BigqueryReservationReplicationStatusEl {
    #[doc = "Set the field `error`.\n"]
    pub fn set_error(
        mut self,
        v: impl Into<ListField<BigqueryReservationReplicationStatusElErrorEl>>,
    ) -> Self {
        self.error = Some(v.into());
        self
    }
    #[doc = "Set the field `last_error_time`.\n"]
    pub fn set_last_error_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_error_time = Some(v.into());
        self
    }
    #[doc = "Set the field `last_replication_time`.\n"]
    pub fn set_last_replication_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_replication_time = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryReservationReplicationStatusEl {
    type O = BlockAssignable<BigqueryReservationReplicationStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryReservationReplicationStatusEl {}
impl BuildBigqueryReservationReplicationStatusEl {
    pub fn build(self) -> BigqueryReservationReplicationStatusEl {
        BigqueryReservationReplicationStatusEl {
            error: core::default::Default::default(),
            last_error_time: core::default::Default::default(),
            last_replication_time: core::default::Default::default(),
        }
    }
}
pub struct BigqueryReservationReplicationStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryReservationReplicationStatusElRef {
    fn new(shared: StackShared, base: String) -> BigqueryReservationReplicationStatusElRef {
        BigqueryReservationReplicationStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryReservationReplicationStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(&self) -> ListRef<BigqueryReservationReplicationStatusElErrorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.error", self.base))
    }
    #[doc = "Get a reference to the value of field `last_error_time` after provisioning.\n"]
    pub fn last_error_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_error_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `last_replication_time` after provisioning.\n"]
    pub fn last_replication_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_replication_time", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryReservationAutoscaleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_slots: Option<PrimField<f64>>,
}
impl BigqueryReservationAutoscaleEl {
    #[doc = "Set the field `max_slots`.\nNumber of slots to be scaled when needed."]
    pub fn set_max_slots(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_slots = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryReservationAutoscaleEl {
    type O = BlockAssignable<BigqueryReservationAutoscaleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryReservationAutoscaleEl {}
impl BuildBigqueryReservationAutoscaleEl {
    pub fn build(self) -> BigqueryReservationAutoscaleEl {
        BigqueryReservationAutoscaleEl {
            max_slots: core::default::Default::default(),
        }
    }
}
pub struct BigqueryReservationAutoscaleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryReservationAutoscaleElRef {
    fn new(shared: StackShared, base: String) -> BigqueryReservationAutoscaleElRef {
        BigqueryReservationAutoscaleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryReservationAutoscaleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `current_slots` after provisioning.\nThe slot capacity added to this reservation when autoscale happens. Will be between [0, max_slots]."]
    pub fn current_slots(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.current_slots", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_slots` after provisioning.\nNumber of slots to be scaled when needed."]
    pub fn max_slots(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_slots", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryReservationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BigqueryReservationTimeoutsEl {
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
impl ToListMappable for BigqueryReservationTimeoutsEl {
    type O = BlockAssignable<BigqueryReservationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryReservationTimeoutsEl {}
impl BuildBigqueryReservationTimeoutsEl {
    pub fn build(self) -> BigqueryReservationTimeoutsEl {
        BigqueryReservationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BigqueryReservationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryReservationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryReservationTimeoutsElRef {
        BigqueryReservationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryReservationTimeoutsElRef {
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
#[derive(Serialize, Default)]
struct BigqueryReservationDynamic {
    autoscale: Option<DynamicBlock<BigqueryReservationAutoscaleEl>>,
}
