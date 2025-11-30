use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct AutoscalingGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zones: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_rebalance: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_cooldown: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_instance_warmup: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_capacity_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled_metrics: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_delete: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_delete_warm_pool: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check_grace_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_failed_scaling_activities: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_configuration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_instance_lifetime: Option<PrimField<f64>>,
    max_size: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metrics_granularity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_elb_capacity: Option<PrimField<f64>>,
    min_size: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_from_scale_in: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_linked_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suspended_processes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_group_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    termination_policies: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_zone_identifier: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for_capacity_timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for_elb_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone_distribution: Option<Vec<AutoscalingGroupAvailabilityZoneDistributionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_reservation_specification:
        Option<Vec<AutoscalingGroupCapacityReservationSpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_lifecycle_hook: Option<Vec<AutoscalingGroupInitialLifecycleHookEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_maintenance_policy: Option<Vec<AutoscalingGroupInstanceMaintenancePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_refresh: Option<Vec<AutoscalingGroupInstanceRefreshEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template: Option<Vec<AutoscalingGroupLaunchTemplateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mixed_instances_policy: Option<Vec<AutoscalingGroupMixedInstancesPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<Vec<AutoscalingGroupTagEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AutoscalingGroupTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    traffic_source: Option<Vec<AutoscalingGroupTrafficSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warm_pool: Option<Vec<AutoscalingGroupWarmPoolEl>>,
    dynamic: AutoscalingGroupDynamic,
}

struct AutoscalingGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AutoscalingGroupData>,
}

#[derive(Clone)]
pub struct AutoscalingGroup(Rc<AutoscalingGroup_>);

impl AutoscalingGroup {
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

    #[doc = "Set the field `availability_zones`.\n"]
    pub fn set_availability_zones(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().availability_zones = Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_rebalance`.\n"]
    pub fn set_capacity_rebalance(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().capacity_rebalance = Some(v.into());
        self
    }

    #[doc = "Set the field `context`.\n"]
    pub fn set_context(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().context = Some(v.into());
        self
    }

    #[doc = "Set the field `default_cooldown`.\n"]
    pub fn set_default_cooldown(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().default_cooldown = Some(v.into());
        self
    }

    #[doc = "Set the field `default_instance_warmup`.\n"]
    pub fn set_default_instance_warmup(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().default_instance_warmup = Some(v.into());
        self
    }

    #[doc = "Set the field `desired_capacity`.\n"]
    pub fn set_desired_capacity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().desired_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `desired_capacity_type`.\n"]
    pub fn set_desired_capacity_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().desired_capacity_type = Some(v.into());
        self
    }

    #[doc = "Set the field `enabled_metrics`.\n"]
    pub fn set_enabled_metrics(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().enabled_metrics = Some(v.into());
        self
    }

    #[doc = "Set the field `force_delete`.\n"]
    pub fn set_force_delete(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_delete = Some(v.into());
        self
    }

    #[doc = "Set the field `force_delete_warm_pool`.\n"]
    pub fn set_force_delete_warm_pool(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_delete_warm_pool = Some(v.into());
        self
    }

    #[doc = "Set the field `health_check_grace_period`.\n"]
    pub fn set_health_check_grace_period(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().health_check_grace_period = Some(v.into());
        self
    }

    #[doc = "Set the field `health_check_type`.\n"]
    pub fn set_health_check_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().health_check_type = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_failed_scaling_activities`.\n"]
    pub fn set_ignore_failed_scaling_activities(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ignore_failed_scaling_activities = Some(v.into());
        self
    }

    #[doc = "Set the field `launch_configuration`.\n"]
    pub fn set_launch_configuration(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().launch_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `load_balancers`.\n"]
    pub fn set_load_balancers(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().load_balancers = Some(v.into());
        self
    }

    #[doc = "Set the field `max_instance_lifetime`.\n"]
    pub fn set_max_instance_lifetime(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_instance_lifetime = Some(v.into());
        self
    }

    #[doc = "Set the field `metrics_granularity`.\n"]
    pub fn set_metrics_granularity(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().metrics_granularity = Some(v.into());
        self
    }

    #[doc = "Set the field `min_elb_capacity`.\n"]
    pub fn set_min_elb_capacity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_elb_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `placement_group`.\n"]
    pub fn set_placement_group(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().placement_group = Some(v.into());
        self
    }

    #[doc = "Set the field `protect_from_scale_in`.\n"]
    pub fn set_protect_from_scale_in(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().protect_from_scale_in = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `service_linked_role_arn`.\n"]
    pub fn set_service_linked_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_linked_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `suspended_processes`.\n"]
    pub fn set_suspended_processes(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().suspended_processes = Some(v.into());
        self
    }

    #[doc = "Set the field `target_group_arns`.\n"]
    pub fn set_target_group_arns(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().target_group_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `termination_policies`.\n"]
    pub fn set_termination_policies(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().termination_policies = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_zone_identifier`.\n"]
    pub fn set_vpc_zone_identifier(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().vpc_zone_identifier = Some(v.into());
        self
    }

    #[doc = "Set the field `wait_for_capacity_timeout`.\n"]
    pub fn set_wait_for_capacity_timeout(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().wait_for_capacity_timeout = Some(v.into());
        self
    }

    #[doc = "Set the field `wait_for_elb_capacity`.\n"]
    pub fn set_wait_for_elb_capacity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().wait_for_elb_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `availability_zone_distribution`.\n"]
    pub fn set_availability_zone_distribution(
        self,
        v: impl Into<BlockAssignable<AutoscalingGroupAvailabilityZoneDistributionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().availability_zone_distribution = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .availability_zone_distribution = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `capacity_reservation_specification`.\n"]
    pub fn set_capacity_reservation_specification(
        self,
        v: impl Into<BlockAssignable<AutoscalingGroupCapacityReservationSpecificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capacity_reservation_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .capacity_reservation_specification = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `initial_lifecycle_hook`.\n"]
    pub fn set_initial_lifecycle_hook(
        self,
        v: impl Into<BlockAssignable<AutoscalingGroupInitialLifecycleHookEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().initial_lifecycle_hook = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.initial_lifecycle_hook = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `instance_maintenance_policy`.\n"]
    pub fn set_instance_maintenance_policy(
        self,
        v: impl Into<BlockAssignable<AutoscalingGroupInstanceMaintenancePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_maintenance_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_maintenance_policy = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `instance_refresh`.\n"]
    pub fn set_instance_refresh(
        self,
        v: impl Into<BlockAssignable<AutoscalingGroupInstanceRefreshEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_refresh = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_refresh = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `launch_template`.\n"]
    pub fn set_launch_template(
        self,
        v: impl Into<BlockAssignable<AutoscalingGroupLaunchTemplateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().launch_template = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.launch_template = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `mixed_instances_policy`.\n"]
    pub fn set_mixed_instances_policy(
        self,
        v: impl Into<BlockAssignable<AutoscalingGroupMixedInstancesPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mixed_instances_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mixed_instances_policy = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(self, v: impl Into<BlockAssignable<AutoscalingGroupTagEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tag = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tag = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<AutoscalingGroupTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Set the field `traffic_source`.\n"]
    pub fn set_traffic_source(
        self,
        v: impl Into<BlockAssignable<AutoscalingGroupTrafficSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().traffic_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.traffic_source = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `warm_pool`.\n"]
    pub fn set_warm_pool(self, v: impl Into<BlockAssignable<AutoscalingGroupWarmPoolEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().warm_pool = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.warm_pool = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.availability_zones", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capacity_rebalance` after provisioning.\n"]
    pub fn capacity_rebalance(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_rebalance", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `context` after provisioning.\n"]
    pub fn context(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.context", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_cooldown` after provisioning.\n"]
    pub fn default_cooldown(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_cooldown", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_instance_warmup` after provisioning.\n"]
    pub fn default_instance_warmup(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_instance_warmup", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `desired_capacity` after provisioning.\n"]
    pub fn desired_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `desired_capacity_type` after provisioning.\n"]
    pub fn desired_capacity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_capacity_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enabled_metrics` after provisioning.\n"]
    pub fn enabled_metrics(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.enabled_metrics", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `force_delete` after provisioning.\n"]
    pub fn force_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_delete", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `force_delete_warm_pool` after provisioning.\n"]
    pub fn force_delete_warm_pool(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_delete_warm_pool", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `health_check_grace_period` after provisioning.\n"]
    pub fn health_check_grace_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_check_grace_period", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `health_check_type` after provisioning.\n"]
    pub fn health_check_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_check_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ignore_failed_scaling_activities` after provisioning.\n"]
    pub fn ignore_failed_scaling_activities(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_failed_scaling_activities", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `launch_configuration` after provisioning.\n"]
    pub fn launch_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.launch_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `load_balancers` after provisioning.\n"]
    pub fn load_balancers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.load_balancers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `max_instance_lifetime` after provisioning.\n"]
    pub fn max_instance_lifetime(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_instance_lifetime", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `max_size` after provisioning.\n"]
    pub fn max_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `metrics_granularity` after provisioning.\n"]
    pub fn metrics_granularity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metrics_granularity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `min_elb_capacity` after provisioning.\n"]
    pub fn min_elb_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_elb_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `min_size` after provisioning.\n"]
    pub fn min_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `placement_group` after provisioning.\n"]
    pub fn placement_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.placement_group", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `predicted_capacity` after provisioning.\n"]
    pub fn predicted_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.predicted_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `protect_from_scale_in` after provisioning.\n"]
    pub fn protect_from_scale_in(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protect_from_scale_in", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_linked_role_arn` after provisioning.\n"]
    pub fn service_linked_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_linked_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `suspended_processes` after provisioning.\n"]
    pub fn suspended_processes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.suspended_processes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `target_group_arns` after provisioning.\n"]
    pub fn target_group_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.target_group_arns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `termination_policies` after provisioning.\n"]
    pub fn termination_policies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.termination_policies", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_zone_identifier` after provisioning.\n"]
    pub fn vpc_zone_identifier(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.vpc_zone_identifier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `wait_for_capacity_timeout` after provisioning.\n"]
    pub fn wait_for_capacity_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_capacity_timeout", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `wait_for_elb_capacity` after provisioning.\n"]
    pub fn wait_for_elb_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_elb_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `warm_pool_size` after provisioning.\n"]
    pub fn warm_pool_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.warm_pool_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `availability_zone_distribution` after provisioning.\n"]
    pub fn availability_zone_distribution(
        &self,
    ) -> ListRef<AutoscalingGroupAvailabilityZoneDistributionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.availability_zone_distribution", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_specification` after provisioning.\n"]
    pub fn capacity_reservation_specification(
        &self,
    ) -> ListRef<AutoscalingGroupCapacityReservationSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_specification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_maintenance_policy` after provisioning.\n"]
    pub fn instance_maintenance_policy(
        &self,
    ) -> ListRef<AutoscalingGroupInstanceMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_maintenance_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_refresh` after provisioning.\n"]
    pub fn instance_refresh(&self) -> ListRef<AutoscalingGroupInstanceRefreshElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_refresh", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> ListRef<AutoscalingGroupLaunchTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.launch_template", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `mixed_instances_policy` after provisioning.\n"]
    pub fn mixed_instances_policy(&self) -> ListRef<AutoscalingGroupMixedInstancesPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mixed_instances_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AutoscalingGroupTimeoutsElRef {
        AutoscalingGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `warm_pool` after provisioning.\n"]
    pub fn warm_pool(&self) -> ListRef<AutoscalingGroupWarmPoolElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.warm_pool", self.extract_ref()),
        )
    }
}

impl Referable for AutoscalingGroup {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for AutoscalingGroup {}

impl ToListMappable for AutoscalingGroup {
    type O = ListRef<AutoscalingGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AutoscalingGroup_ {
    fn extract_resource_type(&self) -> String {
        "aws_autoscaling_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAutoscalingGroup {
    pub tf_id: String,
    #[doc = ""]
    pub max_size: PrimField<f64>,
    #[doc = ""]
    pub min_size: PrimField<f64>,
}

impl BuildAutoscalingGroup {
    pub fn build(self, stack: &mut Stack) -> AutoscalingGroup {
        let out = AutoscalingGroup(Rc::new(AutoscalingGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AutoscalingGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                availability_zones: core::default::Default::default(),
                capacity_rebalance: core::default::Default::default(),
                context: core::default::Default::default(),
                default_cooldown: core::default::Default::default(),
                default_instance_warmup: core::default::Default::default(),
                desired_capacity: core::default::Default::default(),
                desired_capacity_type: core::default::Default::default(),
                enabled_metrics: core::default::Default::default(),
                force_delete: core::default::Default::default(),
                force_delete_warm_pool: core::default::Default::default(),
                health_check_grace_period: core::default::Default::default(),
                health_check_type: core::default::Default::default(),
                id: core::default::Default::default(),
                ignore_failed_scaling_activities: core::default::Default::default(),
                launch_configuration: core::default::Default::default(),
                load_balancers: core::default::Default::default(),
                max_instance_lifetime: core::default::Default::default(),
                max_size: self.max_size,
                metrics_granularity: core::default::Default::default(),
                min_elb_capacity: core::default::Default::default(),
                min_size: self.min_size,
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                placement_group: core::default::Default::default(),
                protect_from_scale_in: core::default::Default::default(),
                region: core::default::Default::default(),
                service_linked_role_arn: core::default::Default::default(),
                suspended_processes: core::default::Default::default(),
                target_group_arns: core::default::Default::default(),
                termination_policies: core::default::Default::default(),
                vpc_zone_identifier: core::default::Default::default(),
                wait_for_capacity_timeout: core::default::Default::default(),
                wait_for_elb_capacity: core::default::Default::default(),
                availability_zone_distribution: core::default::Default::default(),
                capacity_reservation_specification: core::default::Default::default(),
                initial_lifecycle_hook: core::default::Default::default(),
                instance_maintenance_policy: core::default::Default::default(),
                instance_refresh: core::default::Default::default(),
                launch_template: core::default::Default::default(),
                mixed_instances_policy: core::default::Default::default(),
                tag: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                traffic_source: core::default::Default::default(),
                warm_pool: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AutoscalingGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl AutoscalingGroupRef {
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

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.availability_zones", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capacity_rebalance` after provisioning.\n"]
    pub fn capacity_rebalance(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_rebalance", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `context` after provisioning.\n"]
    pub fn context(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.context", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_cooldown` after provisioning.\n"]
    pub fn default_cooldown(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_cooldown", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_instance_warmup` after provisioning.\n"]
    pub fn default_instance_warmup(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_instance_warmup", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `desired_capacity` after provisioning.\n"]
    pub fn desired_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `desired_capacity_type` after provisioning.\n"]
    pub fn desired_capacity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_capacity_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enabled_metrics` after provisioning.\n"]
    pub fn enabled_metrics(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.enabled_metrics", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `force_delete` after provisioning.\n"]
    pub fn force_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_delete", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `force_delete_warm_pool` after provisioning.\n"]
    pub fn force_delete_warm_pool(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_delete_warm_pool", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `health_check_grace_period` after provisioning.\n"]
    pub fn health_check_grace_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_check_grace_period", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `health_check_type` after provisioning.\n"]
    pub fn health_check_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_check_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ignore_failed_scaling_activities` after provisioning.\n"]
    pub fn ignore_failed_scaling_activities(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_failed_scaling_activities", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `launch_configuration` after provisioning.\n"]
    pub fn launch_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.launch_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `load_balancers` after provisioning.\n"]
    pub fn load_balancers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.load_balancers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `max_instance_lifetime` after provisioning.\n"]
    pub fn max_instance_lifetime(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_instance_lifetime", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `max_size` after provisioning.\n"]
    pub fn max_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `metrics_granularity` after provisioning.\n"]
    pub fn metrics_granularity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metrics_granularity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `min_elb_capacity` after provisioning.\n"]
    pub fn min_elb_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_elb_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `min_size` after provisioning.\n"]
    pub fn min_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `placement_group` after provisioning.\n"]
    pub fn placement_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.placement_group", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `predicted_capacity` after provisioning.\n"]
    pub fn predicted_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.predicted_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `protect_from_scale_in` after provisioning.\n"]
    pub fn protect_from_scale_in(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protect_from_scale_in", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_linked_role_arn` after provisioning.\n"]
    pub fn service_linked_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_linked_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `suspended_processes` after provisioning.\n"]
    pub fn suspended_processes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.suspended_processes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `target_group_arns` after provisioning.\n"]
    pub fn target_group_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.target_group_arns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `termination_policies` after provisioning.\n"]
    pub fn termination_policies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.termination_policies", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_zone_identifier` after provisioning.\n"]
    pub fn vpc_zone_identifier(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.vpc_zone_identifier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `wait_for_capacity_timeout` after provisioning.\n"]
    pub fn wait_for_capacity_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_capacity_timeout", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `wait_for_elb_capacity` after provisioning.\n"]
    pub fn wait_for_elb_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_elb_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `warm_pool_size` after provisioning.\n"]
    pub fn warm_pool_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.warm_pool_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `availability_zone_distribution` after provisioning.\n"]
    pub fn availability_zone_distribution(
        &self,
    ) -> ListRef<AutoscalingGroupAvailabilityZoneDistributionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.availability_zone_distribution", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_specification` after provisioning.\n"]
    pub fn capacity_reservation_specification(
        &self,
    ) -> ListRef<AutoscalingGroupCapacityReservationSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_specification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_maintenance_policy` after provisioning.\n"]
    pub fn instance_maintenance_policy(
        &self,
    ) -> ListRef<AutoscalingGroupInstanceMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_maintenance_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_refresh` after provisioning.\n"]
    pub fn instance_refresh(&self) -> ListRef<AutoscalingGroupInstanceRefreshElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_refresh", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> ListRef<AutoscalingGroupLaunchTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.launch_template", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `mixed_instances_policy` after provisioning.\n"]
    pub fn mixed_instances_policy(&self) -> ListRef<AutoscalingGroupMixedInstancesPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mixed_instances_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AutoscalingGroupTimeoutsElRef {
        AutoscalingGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `warm_pool` after provisioning.\n"]
    pub fn warm_pool(&self) -> ListRef<AutoscalingGroupWarmPoolElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.warm_pool", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupAvailabilityZoneDistributionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_distribution_strategy: Option<PrimField<String>>,
}

impl AutoscalingGroupAvailabilityZoneDistributionEl {
    #[doc = "Set the field `capacity_distribution_strategy`.\n"]
    pub fn set_capacity_distribution_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.capacity_distribution_strategy = Some(v.into());
        self
    }
}

impl ToListMappable for AutoscalingGroupAvailabilityZoneDistributionEl {
    type O = BlockAssignable<AutoscalingGroupAvailabilityZoneDistributionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupAvailabilityZoneDistributionEl {}

impl BuildAutoscalingGroupAvailabilityZoneDistributionEl {
    pub fn build(self) -> AutoscalingGroupAvailabilityZoneDistributionEl {
        AutoscalingGroupAvailabilityZoneDistributionEl {
            capacity_distribution_strategy: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupAvailabilityZoneDistributionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupAvailabilityZoneDistributionElRef {
    fn new(shared: StackShared, base: String) -> AutoscalingGroupAvailabilityZoneDistributionElRef {
        AutoscalingGroupAvailabilityZoneDistributionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupAvailabilityZoneDistributionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_distribution_strategy` after provisioning.\n"]
    pub fn capacity_distribution_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_distribution_strategy", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupCapacityReservationSpecificationElCapacityReservationTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_reservation_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_reservation_resource_group_arns: Option<ListField<PrimField<String>>>,
}

impl AutoscalingGroupCapacityReservationSpecificationElCapacityReservationTargetEl {
    #[doc = "Set the field `capacity_reservation_ids`.\n"]
    pub fn set_capacity_reservation_ids(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.capacity_reservation_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_reservation_resource_group_arns`.\n"]
    pub fn set_capacity_reservation_resource_group_arns(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.capacity_reservation_resource_group_arns = Some(v.into());
        self
    }
}

impl ToListMappable
    for AutoscalingGroupCapacityReservationSpecificationElCapacityReservationTargetEl
{
    type O = BlockAssignable<
        AutoscalingGroupCapacityReservationSpecificationElCapacityReservationTargetEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupCapacityReservationSpecificationElCapacityReservationTargetEl {}

impl BuildAutoscalingGroupCapacityReservationSpecificationElCapacityReservationTargetEl {
    pub fn build(
        self,
    ) -> AutoscalingGroupCapacityReservationSpecificationElCapacityReservationTargetEl {
        AutoscalingGroupCapacityReservationSpecificationElCapacityReservationTargetEl {
            capacity_reservation_ids: core::default::Default::default(),
            capacity_reservation_resource_group_arns: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupCapacityReservationSpecificationElCapacityReservationTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupCapacityReservationSpecificationElCapacityReservationTargetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingGroupCapacityReservationSpecificationElCapacityReservationTargetElRef {
        AutoscalingGroupCapacityReservationSpecificationElCapacityReservationTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupCapacityReservationSpecificationElCapacityReservationTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_ids` after provisioning.\n"]
    pub fn capacity_reservation_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_ids", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_resource_group_arns` after provisioning.\n"]
    pub fn capacity_reservation_resource_group_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_resource_group_arns", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct AutoscalingGroupCapacityReservationSpecificationElDynamic {
    capacity_reservation_target: Option<
        DynamicBlock<AutoscalingGroupCapacityReservationSpecificationElCapacityReservationTargetEl>,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingGroupCapacityReservationSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_reservation_preference: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_reservation_target:
        Option<Vec<AutoscalingGroupCapacityReservationSpecificationElCapacityReservationTargetEl>>,
    dynamic: AutoscalingGroupCapacityReservationSpecificationElDynamic,
}

impl AutoscalingGroupCapacityReservationSpecificationEl {
    #[doc = "Set the field `capacity_reservation_preference`.\n"]
    pub fn set_capacity_reservation_preference(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.capacity_reservation_preference = Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_reservation_target`.\n"]
    pub fn set_capacity_reservation_target(
        mut self,
        v: impl Into<
            BlockAssignable<
                AutoscalingGroupCapacityReservationSpecificationElCapacityReservationTargetEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.capacity_reservation_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.capacity_reservation_target = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AutoscalingGroupCapacityReservationSpecificationEl {
    type O = BlockAssignable<AutoscalingGroupCapacityReservationSpecificationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupCapacityReservationSpecificationEl {}

impl BuildAutoscalingGroupCapacityReservationSpecificationEl {
    pub fn build(self) -> AutoscalingGroupCapacityReservationSpecificationEl {
        AutoscalingGroupCapacityReservationSpecificationEl {
            capacity_reservation_preference: core::default::Default::default(),
            capacity_reservation_target: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AutoscalingGroupCapacityReservationSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupCapacityReservationSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingGroupCapacityReservationSpecificationElRef {
        AutoscalingGroupCapacityReservationSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupCapacityReservationSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_preference` after provisioning.\n"]
    pub fn capacity_reservation_preference(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_preference", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_target` after provisioning.\n"]
    pub fn capacity_reservation_target(
        &self,
    ) -> ListRef<AutoscalingGroupCapacityReservationSpecificationElCapacityReservationTargetElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_target", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupInitialLifecycleHookEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_result: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    heartbeat_timeout: Option<PrimField<f64>>,
    lifecycle_transition: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_metadata: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_target_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
}

impl AutoscalingGroupInitialLifecycleHookEl {
    #[doc = "Set the field `default_result`.\n"]
    pub fn set_default_result(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_result = Some(v.into());
        self
    }

    #[doc = "Set the field `heartbeat_timeout`.\n"]
    pub fn set_heartbeat_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.heartbeat_timeout = Some(v.into());
        self
    }

    #[doc = "Set the field `notification_metadata`.\n"]
    pub fn set_notification_metadata(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.notification_metadata = Some(v.into());
        self
    }

    #[doc = "Set the field `notification_target_arn`.\n"]
    pub fn set_notification_target_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.notification_target_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_arn = Some(v.into());
        self
    }
}

impl ToListMappable for AutoscalingGroupInitialLifecycleHookEl {
    type O = BlockAssignable<AutoscalingGroupInitialLifecycleHookEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupInitialLifecycleHookEl {
    #[doc = ""]
    pub lifecycle_transition: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildAutoscalingGroupInitialLifecycleHookEl {
    pub fn build(self) -> AutoscalingGroupInitialLifecycleHookEl {
        AutoscalingGroupInitialLifecycleHookEl {
            default_result: core::default::Default::default(),
            heartbeat_timeout: core::default::Default::default(),
            lifecycle_transition: self.lifecycle_transition,
            name: self.name,
            notification_metadata: core::default::Default::default(),
            notification_target_arn: core::default::Default::default(),
            role_arn: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupInitialLifecycleHookElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupInitialLifecycleHookElRef {
    fn new(shared: StackShared, base: String) -> AutoscalingGroupInitialLifecycleHookElRef {
        AutoscalingGroupInitialLifecycleHookElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupInitialLifecycleHookElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_result` after provisioning.\n"]
    pub fn default_result(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_result", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `heartbeat_timeout` after provisioning.\n"]
    pub fn heartbeat_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.heartbeat_timeout", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `lifecycle_transition` after provisioning.\n"]
    pub fn lifecycle_transition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifecycle_transition", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `notification_metadata` after provisioning.\n"]
    pub fn notification_metadata(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notification_metadata", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `notification_target_arn` after provisioning.\n"]
    pub fn notification_target_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notification_target_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupInstanceMaintenancePolicyEl {
    max_healthy_percentage: PrimField<f64>,
    min_healthy_percentage: PrimField<f64>,
}

impl AutoscalingGroupInstanceMaintenancePolicyEl {}

impl ToListMappable for AutoscalingGroupInstanceMaintenancePolicyEl {
    type O = BlockAssignable<AutoscalingGroupInstanceMaintenancePolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupInstanceMaintenancePolicyEl {
    #[doc = ""]
    pub max_healthy_percentage: PrimField<f64>,
    #[doc = ""]
    pub min_healthy_percentage: PrimField<f64>,
}

impl BuildAutoscalingGroupInstanceMaintenancePolicyEl {
    pub fn build(self) -> AutoscalingGroupInstanceMaintenancePolicyEl {
        AutoscalingGroupInstanceMaintenancePolicyEl {
            max_healthy_percentage: self.max_healthy_percentage,
            min_healthy_percentage: self.min_healthy_percentage,
        }
    }
}

pub struct AutoscalingGroupInstanceMaintenancePolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupInstanceMaintenancePolicyElRef {
    fn new(shared: StackShared, base: String) -> AutoscalingGroupInstanceMaintenancePolicyElRef {
        AutoscalingGroupInstanceMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupInstanceMaintenancePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_healthy_percentage` after provisioning.\n"]
    pub fn max_healthy_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_healthy_percentage", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `min_healthy_percentage` after provisioning.\n"]
    pub fn min_healthy_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_healthy_percentage", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupInstanceRefreshElPreferencesElAlarmSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    alarms: Option<ListField<PrimField<String>>>,
}

impl AutoscalingGroupInstanceRefreshElPreferencesElAlarmSpecificationEl {
    #[doc = "Set the field `alarms`.\n"]
    pub fn set_alarms(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.alarms = Some(v.into());
        self
    }
}

impl ToListMappable for AutoscalingGroupInstanceRefreshElPreferencesElAlarmSpecificationEl {
    type O = BlockAssignable<AutoscalingGroupInstanceRefreshElPreferencesElAlarmSpecificationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupInstanceRefreshElPreferencesElAlarmSpecificationEl {}

impl BuildAutoscalingGroupInstanceRefreshElPreferencesElAlarmSpecificationEl {
    pub fn build(self) -> AutoscalingGroupInstanceRefreshElPreferencesElAlarmSpecificationEl {
        AutoscalingGroupInstanceRefreshElPreferencesElAlarmSpecificationEl {
            alarms: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupInstanceRefreshElPreferencesElAlarmSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupInstanceRefreshElPreferencesElAlarmSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingGroupInstanceRefreshElPreferencesElAlarmSpecificationElRef {
        AutoscalingGroupInstanceRefreshElPreferencesElAlarmSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupInstanceRefreshElPreferencesElAlarmSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alarms` after provisioning.\n"]
    pub fn alarms(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.alarms", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingGroupInstanceRefreshElPreferencesElDynamic {
    alarm_specification:
        Option<DynamicBlock<AutoscalingGroupInstanceRefreshElPreferencesElAlarmSpecificationEl>>,
}

#[derive(Serialize)]
pub struct AutoscalingGroupInstanceRefreshElPreferencesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_rollback: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    checkpoint_delay: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    checkpoint_percentages: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_warmup: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_healthy_percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_healthy_percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_in_protected_instances: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_matching: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    standby_instances: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alarm_specification:
        Option<Vec<AutoscalingGroupInstanceRefreshElPreferencesElAlarmSpecificationEl>>,
    dynamic: AutoscalingGroupInstanceRefreshElPreferencesElDynamic,
}

impl AutoscalingGroupInstanceRefreshElPreferencesEl {
    #[doc = "Set the field `auto_rollback`.\n"]
    pub fn set_auto_rollback(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_rollback = Some(v.into());
        self
    }

    #[doc = "Set the field `checkpoint_delay`.\n"]
    pub fn set_checkpoint_delay(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.checkpoint_delay = Some(v.into());
        self
    }

    #[doc = "Set the field `checkpoint_percentages`.\n"]
    pub fn set_checkpoint_percentages(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.checkpoint_percentages = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_warmup`.\n"]
    pub fn set_instance_warmup(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_warmup = Some(v.into());
        self
    }

    #[doc = "Set the field `max_healthy_percentage`.\n"]
    pub fn set_max_healthy_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_healthy_percentage = Some(v.into());
        self
    }

    #[doc = "Set the field `min_healthy_percentage`.\n"]
    pub fn set_min_healthy_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_healthy_percentage = Some(v.into());
        self
    }

    #[doc = "Set the field `scale_in_protected_instances`.\n"]
    pub fn set_scale_in_protected_instances(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scale_in_protected_instances = Some(v.into());
        self
    }

    #[doc = "Set the field `skip_matching`.\n"]
    pub fn set_skip_matching(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.skip_matching = Some(v.into());
        self
    }

    #[doc = "Set the field `standby_instances`.\n"]
    pub fn set_standby_instances(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.standby_instances = Some(v.into());
        self
    }

    #[doc = "Set the field `alarm_specification`.\n"]
    pub fn set_alarm_specification(
        mut self,
        v: impl Into<
            BlockAssignable<AutoscalingGroupInstanceRefreshElPreferencesElAlarmSpecificationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.alarm_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.alarm_specification = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AutoscalingGroupInstanceRefreshElPreferencesEl {
    type O = BlockAssignable<AutoscalingGroupInstanceRefreshElPreferencesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupInstanceRefreshElPreferencesEl {}

impl BuildAutoscalingGroupInstanceRefreshElPreferencesEl {
    pub fn build(self) -> AutoscalingGroupInstanceRefreshElPreferencesEl {
        AutoscalingGroupInstanceRefreshElPreferencesEl {
            auto_rollback: core::default::Default::default(),
            checkpoint_delay: core::default::Default::default(),
            checkpoint_percentages: core::default::Default::default(),
            instance_warmup: core::default::Default::default(),
            max_healthy_percentage: core::default::Default::default(),
            min_healthy_percentage: core::default::Default::default(),
            scale_in_protected_instances: core::default::Default::default(),
            skip_matching: core::default::Default::default(),
            standby_instances: core::default::Default::default(),
            alarm_specification: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AutoscalingGroupInstanceRefreshElPreferencesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupInstanceRefreshElPreferencesElRef {
    fn new(shared: StackShared, base: String) -> AutoscalingGroupInstanceRefreshElPreferencesElRef {
        AutoscalingGroupInstanceRefreshElPreferencesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupInstanceRefreshElPreferencesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_rollback` after provisioning.\n"]
    pub fn auto_rollback(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_rollback", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `checkpoint_delay` after provisioning.\n"]
    pub fn checkpoint_delay(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.checkpoint_delay", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `checkpoint_percentages` after provisioning.\n"]
    pub fn checkpoint_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.checkpoint_percentages", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `instance_warmup` after provisioning.\n"]
    pub fn instance_warmup(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_warmup", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `max_healthy_percentage` after provisioning.\n"]
    pub fn max_healthy_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_healthy_percentage", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `min_healthy_percentage` after provisioning.\n"]
    pub fn min_healthy_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_healthy_percentage", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `scale_in_protected_instances` after provisioning.\n"]
    pub fn scale_in_protected_instances(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scale_in_protected_instances", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `skip_matching` after provisioning.\n"]
    pub fn skip_matching(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_matching", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `standby_instances` after provisioning.\n"]
    pub fn standby_instances(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.standby_instances", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `alarm_specification` after provisioning.\n"]
    pub fn alarm_specification(
        &self,
    ) -> ListRef<AutoscalingGroupInstanceRefreshElPreferencesElAlarmSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.alarm_specification", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct AutoscalingGroupInstanceRefreshElDynamic {
    preferences: Option<DynamicBlock<AutoscalingGroupInstanceRefreshElPreferencesEl>>,
}

#[derive(Serialize)]
pub struct AutoscalingGroupInstanceRefreshEl {
    strategy: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    triggers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferences: Option<Vec<AutoscalingGroupInstanceRefreshElPreferencesEl>>,
    dynamic: AutoscalingGroupInstanceRefreshElDynamic,
}

impl AutoscalingGroupInstanceRefreshEl {
    #[doc = "Set the field `triggers`.\n"]
    pub fn set_triggers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.triggers = Some(v.into());
        self
    }

    #[doc = "Set the field `preferences`.\n"]
    pub fn set_preferences(
        mut self,
        v: impl Into<BlockAssignable<AutoscalingGroupInstanceRefreshElPreferencesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.preferences = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.preferences = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AutoscalingGroupInstanceRefreshEl {
    type O = BlockAssignable<AutoscalingGroupInstanceRefreshEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupInstanceRefreshEl {
    #[doc = ""]
    pub strategy: PrimField<String>,
}

impl BuildAutoscalingGroupInstanceRefreshEl {
    pub fn build(self) -> AutoscalingGroupInstanceRefreshEl {
        AutoscalingGroupInstanceRefreshEl {
            strategy: self.strategy,
            triggers: core::default::Default::default(),
            preferences: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AutoscalingGroupInstanceRefreshElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupInstanceRefreshElRef {
    fn new(shared: StackShared, base: String) -> AutoscalingGroupInstanceRefreshElRef {
        AutoscalingGroupInstanceRefreshElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupInstanceRefreshElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `strategy` after provisioning.\n"]
    pub fn strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `triggers` after provisioning.\n"]
    pub fn triggers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.triggers", self.base))
    }

    #[doc = "Get a reference to the value of field `preferences` after provisioning.\n"]
    pub fn preferences(&self) -> ListRef<AutoscalingGroupInstanceRefreshElPreferencesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.preferences", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupLaunchTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl AutoscalingGroupLaunchTemplateEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable for AutoscalingGroupLaunchTemplateEl {
    type O = BlockAssignable<AutoscalingGroupLaunchTemplateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupLaunchTemplateEl {}

impl BuildAutoscalingGroupLaunchTemplateEl {
    pub fn build(self) -> AutoscalingGroupLaunchTemplateEl {
        AutoscalingGroupLaunchTemplateEl {
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupLaunchTemplateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupLaunchTemplateElRef {
    fn new(shared: StackShared, base: String) -> AutoscalingGroupLaunchTemplateElRef {
        AutoscalingGroupLaunchTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupLaunchTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    on_demand_allocation_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_demand_base_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_demand_percentage_above_base_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot_allocation_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot_instance_pools: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot_max_price: Option<PrimField<String>>,
}

impl AutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl {
    #[doc = "Set the field `on_demand_allocation_strategy`.\n"]
    pub fn set_on_demand_allocation_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_demand_allocation_strategy = Some(v.into());
        self
    }

    #[doc = "Set the field `on_demand_base_capacity`.\n"]
    pub fn set_on_demand_base_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.on_demand_base_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `on_demand_percentage_above_base_capacity`.\n"]
    pub fn set_on_demand_percentage_above_base_capacity(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.on_demand_percentage_above_base_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `spot_allocation_strategy`.\n"]
    pub fn set_spot_allocation_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.spot_allocation_strategy = Some(v.into());
        self
    }

    #[doc = "Set the field `spot_instance_pools`.\n"]
    pub fn set_spot_instance_pools(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.spot_instance_pools = Some(v.into());
        self
    }

    #[doc = "Set the field `spot_max_price`.\n"]
    pub fn set_spot_max_price(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.spot_max_price = Some(v.into());
        self
    }
}

impl ToListMappable for AutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl {
    type O = BlockAssignable<AutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl {}

impl BuildAutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl {
    pub fn build(self) -> AutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl {
        AutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl {
            on_demand_allocation_strategy: core::default::Default::default(),
            on_demand_base_capacity: core::default::Default::default(),
            on_demand_percentage_above_base_capacity: core::default::Default::default(),
            spot_allocation_strategy: core::default::Default::default(),
            spot_instance_pools: core::default::Default::default(),
            spot_max_price: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElInstancesDistributionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupMixedInstancesPolicyElInstancesDistributionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingGroupMixedInstancesPolicyElInstancesDistributionElRef {
        AutoscalingGroupMixedInstancesPolicyElInstancesDistributionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElInstancesDistributionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `on_demand_allocation_strategy` after provisioning.\n"]
    pub fn on_demand_allocation_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_demand_allocation_strategy", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `on_demand_base_capacity` after provisioning.\n"]
    pub fn on_demand_base_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_demand_base_capacity", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `on_demand_percentage_above_base_capacity` after provisioning.\n"]
    pub fn on_demand_percentage_above_base_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_demand_percentage_above_base_capacity", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `spot_allocation_strategy` after provisioning.\n"]
    pub fn spot_allocation_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.spot_allocation_strategy", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `spot_instance_pools` after provisioning.\n"]
    pub fn spot_instance_pools(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.spot_instance_pools", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `spot_max_price` after provisioning.\n"]
    pub fn spot_max_price(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.spot_max_price", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl {
    #[doc = "Set the field `launch_template_id`.\n"]
    pub fn set_launch_template_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.launch_template_id = Some(v.into());
        self
    }

    #[doc = "Set the field `launch_template_name`.\n"]
    pub fn set_launch_template_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.launch_template_name = Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable
    for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl
{
    type O = BlockAssignable<
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl
{}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl {
    pub fn build(
        self,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl {
            launch_template_id: core::default::Default::default(),
            launch_template_name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref
    for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationElRef
    {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `launch_template_id` after provisioning.\n"]
    pub fn launch_template_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.launch_template_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `launch_template_name` after provisioning.\n"]
    pub fn launch_template_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.launch_template_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min = Some(v.into());
        self
    }
}

impl ToListMappable for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl {
    type O =
        BlockAssignable<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl
{}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl {
    pub fn build(
        self,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountElRef {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min = Some(v.into());
        self
    }
}

impl ToListMappable for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl {
    type O =
        BlockAssignable<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl
{}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl {
    pub fn build(
        self,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min = Some(v.into());
        self
    }
}

impl ToListMappable for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
    type O =
        BlockAssignable<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl
{}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
    pub fn build(
        self,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min = Some(v.into());
        self
    }
}

impl ToListMappable for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl {
    type O =
        BlockAssignable<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl
{}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl {
    pub fn build(
        self,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuElRef {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min = Some(v.into());
        self
    }
}

impl ToListMappable for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl {
    type O =
        BlockAssignable<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl
{}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl {
    pub fn build(
        self,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibElRef {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min = Some(v.into());
        self
    }
}

impl ToListMappable for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl {
    type O =
        BlockAssignable<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl
{}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl {
    pub fn build(
        self,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsElRef {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min = Some(v.into());
        self
    }
}

impl ToListMappable for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl {
    type O =
        BlockAssignable<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl
{}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl {
    pub fn build(
        self,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountElRef {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min = Some(v.into());
        self
    }
}

impl ToListMappable for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl {
    type O =
        BlockAssignable<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl
{}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl {
    pub fn build(
        self,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbElRef {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min = Some(v.into());
        self
    }
}

impl ToListMappable for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl {
    type O =
        BlockAssignable<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl
{}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl {
    pub fn build(
        self,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountElRef {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElDynamic {
    accelerator_count: Option<
        DynamicBlock<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl,
        >,
    >,
    accelerator_total_memory_mib: Option<
        DynamicBlock<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl,
        >,
    >,
    baseline_ebs_bandwidth_mbps: Option<
        DynamicBlock<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl,
        >,
    >,
    memory_gib_per_vcpu: Option<
        DynamicBlock<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl,
        >,
    >,
    memory_mib: Option<
        DynamicBlock<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl,
        >,
    >,
    network_bandwidth_gbps: Option<
        DynamicBlock<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl,
        >,
    >,
    network_interface_count: Option<
        DynamicBlock<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl,
        >,
    >,
    total_local_storage_gb: Option<
        DynamicBlock<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl,
        >,
    >,
    vcpu_count: Option<
        DynamicBlock<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_manufacturers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_names: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_instance_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bare_metal: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    burstable_performance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_manufacturers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_instance_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_generations: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_storage: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_storage_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_spot_price_as_percentage_of_optimal_on_demand_price: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_demand_max_price_percentage_over_lowest_price: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_hibernate_support: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot_max_price_percentage_over_lowest_price: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_count: Option<
        Vec<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_total_memory_mib: Option<
        Vec<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    baseline_ebs_bandwidth_mbps: Option<
        Vec<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_gib_per_vcpu: Option<
        Vec<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_mib: Option<
        Vec<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_bandwidth_gbps: Option<
        Vec<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface_count: Option<
        Vec<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_local_storage_gb: Option<
        Vec<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcpu_count: Option<
        Vec<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl>,
    >,
    dynamic: AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElDynamic,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl {
    #[doc = "Set the field `accelerator_manufacturers`.\n"]
    pub fn set_accelerator_manufacturers(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.accelerator_manufacturers = Some(v.into());
        self
    }

    #[doc = "Set the field `accelerator_names`.\n"]
    pub fn set_accelerator_names(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.accelerator_names = Some(v.into());
        self
    }

    #[doc = "Set the field `accelerator_types`.\n"]
    pub fn set_accelerator_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.accelerator_types = Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_instance_types`.\n"]
    pub fn set_allowed_instance_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allowed_instance_types = Some(v.into());
        self
    }

    #[doc = "Set the field `bare_metal`.\n"]
    pub fn set_bare_metal(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bare_metal = Some(v.into());
        self
    }

    #[doc = "Set the field `burstable_performance`.\n"]
    pub fn set_burstable_performance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.burstable_performance = Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_manufacturers`.\n"]
    pub fn set_cpu_manufacturers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.cpu_manufacturers = Some(v.into());
        self
    }

    #[doc = "Set the field `excluded_instance_types`.\n"]
    pub fn set_excluded_instance_types(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.excluded_instance_types = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_generations`.\n"]
    pub fn set_instance_generations(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.instance_generations = Some(v.into());
        self
    }

    #[doc = "Set the field `local_storage`.\n"]
    pub fn set_local_storage(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_storage = Some(v.into());
        self
    }

    #[doc = "Set the field `local_storage_types`.\n"]
    pub fn set_local_storage_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.local_storage_types = Some(v.into());
        self
    }

    #[doc = "Set the field `max_spot_price_as_percentage_of_optimal_on_demand_price`.\n"]
    pub fn set_max_spot_price_as_percentage_of_optimal_on_demand_price(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.max_spot_price_as_percentage_of_optimal_on_demand_price = Some(v.into());
        self
    }

    #[doc = "Set the field `on_demand_max_price_percentage_over_lowest_price`.\n"]
    pub fn set_on_demand_max_price_percentage_over_lowest_price(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.on_demand_max_price_percentage_over_lowest_price = Some(v.into());
        self
    }

    #[doc = "Set the field `require_hibernate_support`.\n"]
    pub fn set_require_hibernate_support(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_hibernate_support = Some(v.into());
        self
    }

    #[doc = "Set the field `spot_max_price_percentage_over_lowest_price`.\n"]
    pub fn set_spot_max_price_percentage_over_lowest_price(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.spot_max_price_percentage_over_lowest_price = Some(v.into());
        self
    }

    #[doc = "Set the field `accelerator_count`.\n"]
    pub fn set_accelerator_count(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.accelerator_count = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerator_count = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `accelerator_total_memory_mib`.\n"]
    pub fn set_accelerator_total_memory_mib(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.accelerator_total_memory_mib = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerator_total_memory_mib = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `baseline_ebs_bandwidth_mbps`.\n"]
    pub fn set_baseline_ebs_bandwidth_mbps(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.baseline_ebs_bandwidth_mbps = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.baseline_ebs_bandwidth_mbps = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `memory_gib_per_vcpu`.\n"]
    pub fn set_memory_gib_per_vcpu(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.memory_gib_per_vcpu = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.memory_gib_per_vcpu = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `memory_mib`.\n"]
    pub fn set_memory_mib(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.memory_mib = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.memory_mib = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `network_bandwidth_gbps`.\n"]
    pub fn set_network_bandwidth_gbps(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_bandwidth_gbps = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_bandwidth_gbps = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `network_interface_count`.\n"]
    pub fn set_network_interface_count(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_interface_count = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_interface_count = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `total_local_storage_gb`.\n"]
    pub fn set_total_local_storage_gb(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.total_local_storage_gb = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.total_local_storage_gb = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `vcpu_count`.\n"]
    pub fn set_vcpu_count(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vcpu_count = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vcpu_count = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl
{
    type O = BlockAssignable<
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl
{}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl {
    pub fn build(
        self,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl
    {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl {
            accelerator_manufacturers: core::default::Default::default(),
            accelerator_names: core::default::Default::default(),
            accelerator_types: core::default::Default::default(),
            allowed_instance_types: core::default::Default::default(),
            bare_metal: core::default::Default::default(),
            burstable_performance: core::default::Default::default(),
            cpu_manufacturers: core::default::Default::default(),
            excluded_instance_types: core::default::Default::default(),
            instance_generations: core::default::Default::default(),
            local_storage: core::default::Default::default(),
            local_storage_types: core::default::Default::default(),
            max_spot_price_as_percentage_of_optimal_on_demand_price:
                core::default::Default::default(),
            on_demand_max_price_percentage_over_lowest_price: core::default::Default::default(),
            require_hibernate_support: core::default::Default::default(),
            spot_max_price_percentage_over_lowest_price: core::default::Default::default(),
            accelerator_count: core::default::Default::default(),
            accelerator_total_memory_mib: core::default::Default::default(),
            baseline_ebs_bandwidth_mbps: core::default::Default::default(),
            memory_gib_per_vcpu: core::default::Default::default(),
            memory_mib: core::default::Default::default(),
            network_bandwidth_gbps: core::default::Default::default(),
            network_interface_count: core::default::Default::default(),
            total_local_storage_gb: core::default::Default::default(),
            vcpu_count: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElRef
{
    shared: StackShared,
    base: String,
}

impl Ref
    for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElRef
    {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accelerator_manufacturers` after provisioning.\n"]
    pub fn accelerator_manufacturers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.accelerator_manufacturers", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `accelerator_names` after provisioning.\n"]
    pub fn accelerator_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.accelerator_names", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `accelerator_types` after provisioning.\n"]
    pub fn accelerator_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.accelerator_types", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `allowed_instance_types` after provisioning.\n"]
    pub fn allowed_instance_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.allowed_instance_types", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `bare_metal` after provisioning.\n"]
    pub fn bare_metal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bare_metal", self.base))
    }

    #[doc = "Get a reference to the value of field `burstable_performance` after provisioning.\n"]
    pub fn burstable_performance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.burstable_performance", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cpu_manufacturers` after provisioning.\n"]
    pub fn cpu_manufacturers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.cpu_manufacturers", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `excluded_instance_types` after provisioning.\n"]
    pub fn excluded_instance_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.excluded_instance_types", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `instance_generations` after provisioning.\n"]
    pub fn instance_generations(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.instance_generations", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `local_storage` after provisioning.\n"]
    pub fn local_storage(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_storage", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `local_storage_types` after provisioning.\n"]
    pub fn local_storage_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.local_storage_types", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `max_spot_price_as_percentage_of_optimal_on_demand_price` after provisioning.\n"]
    pub fn max_spot_price_as_percentage_of_optimal_on_demand_price(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.max_spot_price_as_percentage_of_optimal_on_demand_price",
                self.base
            ),
        )
    }

    #[doc = "Get a reference to the value of field `on_demand_max_price_percentage_over_lowest_price` after provisioning.\n"]
    pub fn on_demand_max_price_percentage_over_lowest_price(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.on_demand_max_price_percentage_over_lowest_price",
                self.base
            ),
        )
    }

    #[doc = "Get a reference to the value of field `require_hibernate_support` after provisioning.\n"]
    pub fn require_hibernate_support(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_hibernate_support", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `spot_max_price_percentage_over_lowest_price` after provisioning.\n"]
    pub fn spot_max_price_percentage_over_lowest_price(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.spot_max_price_percentage_over_lowest_price", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `accelerator_count` after provisioning.\n"]
    pub fn accelerator_count(
        &self,
    ) -> ListRef<
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.accelerator_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `accelerator_total_memory_mib` after provisioning.\n"]
    pub fn accelerator_total_memory_mib(
        &self,
    ) -> ListRef<
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.accelerator_total_memory_mib", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `baseline_ebs_bandwidth_mbps` after provisioning.\n"]
    pub fn baseline_ebs_bandwidth_mbps(
        &self,
    ) -> ListRef<
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.baseline_ebs_bandwidth_mbps", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `memory_gib_per_vcpu` after provisioning.\n"]
    pub fn memory_gib_per_vcpu(
        &self,
    ) -> ListRef<
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.memory_gib_per_vcpu", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `memory_mib` after provisioning.\n"]
    pub fn memory_mib(
        &self,
    ) -> ListRef<
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibElRef,
    >{
        ListRef::new(self.shared().clone(), format!("{}.memory_mib", self.base))
    }

    #[doc = "Get a reference to the value of field `network_bandwidth_gbps` after provisioning.\n"]
    pub fn network_bandwidth_gbps(
        &self,
    ) -> ListRef<
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_bandwidth_gbps", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `network_interface_count` after provisioning.\n"]
    pub fn network_interface_count(
        &self,
    ) -> ListRef<
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interface_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `total_local_storage_gb` after provisioning.\n"]
    pub fn total_local_storage_gb(
        &self,
    ) -> ListRef<
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.total_local_storage_gb", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `vcpu_count` after provisioning.\n"]
    pub fn vcpu_count(
        &self,
    ) -> ListRef<
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountElRef,
    >{
        ListRef::new(self.shared().clone(), format!("{}.vcpu_count", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl {
    #[doc = "Set the field `launch_template_id`.\n"]
    pub fn set_launch_template_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.launch_template_id = Some(v.into());
        self
    }

    #[doc = "Set the field `launch_template_name`.\n"]
    pub fn set_launch_template_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.launch_template_name = Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl {
    type O =
        BlockAssignable<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl
{}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl {
    pub fn build(
        self,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl {
            launch_template_id: core::default::Default::default(),
            launch_template_name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationElRef {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl
    AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `launch_template_id` after provisioning.\n"]
    pub fn launch_template_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.launch_template_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `launch_template_name` after provisioning.\n"]
    pub fn launch_template_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.launch_template_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElDynamic {
    instance_requirements: Option<
        DynamicBlock<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl>,
    >,
    launch_template_specification: Option<
        DynamicBlock<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl>,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_capacity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_requirements: Option<
        Vec<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_specification: Option<
        Vec<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl>,
    >,
    dynamic: AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElDynamic,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `weighted_capacity`.\n"]
    pub fn set_weighted_capacity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.weighted_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_requirements`.\n"]
    pub fn set_instance_requirements(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.instance_requirements = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_requirements = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `launch_template_specification`.\n"]
    pub fn set_launch_template_specification(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.launch_template_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.launch_template_specification = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl {
    type O = BlockAssignable<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl {}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl {
    pub fn build(self) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl {
            instance_type: core::default::Default::default(),
            weighted_capacity: core::default::Default::default(),
            instance_requirements: core::default::Default::default(),
            launch_template_specification: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElRef {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `weighted_capacity` after provisioning.\n"]
    pub fn weighted_capacity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.weighted_capacity", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `instance_requirements` after provisioning.\n"]
    pub fn instance_requirements(
        &self,
    ) -> ListRef<
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_requirements", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `launch_template_specification` after provisioning.\n"]
    pub fn launch_template_specification(
        &self,
    ) -> ListRef<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.launch_template_specification", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElDynamic {
    launch_template_specification: Option<
        DynamicBlock<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl,
        >,
    >,
    override_:
        Option<DynamicBlock<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl>>,
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_specification: Option<
        Vec<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl>,
    >,
    #[serde(rename = "override", skip_serializing_if = "Option::is_none")]
    override_: Option<Vec<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl>>,
    dynamic: AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElDynamic,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl {
    #[doc = "Set the field `launch_template_specification`.\n"]
    pub fn set_launch_template_specification(
        mut self,
        v: impl Into<
            BlockAssignable<
                AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.launch_template_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.launch_template_specification = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `override_`.\n"]
    pub fn set_override(
        mut self,
        v: impl Into<BlockAssignable<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.override_ = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.override_ = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl {
    type O = BlockAssignable<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl {}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl {
    pub fn build(self) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl {
            launch_template_specification: core::default::Default::default(),
            override_: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElRef {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `launch_template_specification` after provisioning.\n"]
    pub fn launch_template_specification(
        &self,
    ) -> ListRef<
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.launch_template_specification", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(
        &self,
    ) -> ListRef<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElRef> {
        ListRef::new(self.shared().clone(), format!("{}.override", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingGroupMixedInstancesPolicyElDynamic {
    instances_distribution:
        Option<DynamicBlock<AutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl>>,
    launch_template: Option<DynamicBlock<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl>>,
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instances_distribution:
        Option<Vec<AutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template: Option<Vec<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl>>,
    dynamic: AutoscalingGroupMixedInstancesPolicyElDynamic,
}

impl AutoscalingGroupMixedInstancesPolicyEl {
    #[doc = "Set the field `instances_distribution`.\n"]
    pub fn set_instances_distribution(
        mut self,
        v: impl Into<BlockAssignable<AutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.instances_distribution = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.instances_distribution = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `launch_template`.\n"]
    pub fn set_launch_template(
        mut self,
        v: impl Into<BlockAssignable<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.launch_template = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.launch_template = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AutoscalingGroupMixedInstancesPolicyEl {
    type O = BlockAssignable<AutoscalingGroupMixedInstancesPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyEl {}

impl BuildAutoscalingGroupMixedInstancesPolicyEl {
    pub fn build(self) -> AutoscalingGroupMixedInstancesPolicyEl {
        AutoscalingGroupMixedInstancesPolicyEl {
            instances_distribution: core::default::Default::default(),
            launch_template: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupMixedInstancesPolicyElRef {
    fn new(shared: StackShared, base: String) -> AutoscalingGroupMixedInstancesPolicyElRef {
        AutoscalingGroupMixedInstancesPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instances_distribution` after provisioning.\n"]
    pub fn instances_distribution(
        &self,
    ) -> ListRef<AutoscalingGroupMixedInstancesPolicyElInstancesDistributionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instances_distribution", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(
        &self,
    ) -> ListRef<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.launch_template", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupTagEl {
    key: PrimField<String>,
    propagate_at_launch: PrimField<bool>,
    value: PrimField<String>,
}

impl AutoscalingGroupTagEl {}

impl ToListMappable for AutoscalingGroupTagEl {
    type O = BlockAssignable<AutoscalingGroupTagEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupTagEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub propagate_at_launch: PrimField<bool>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildAutoscalingGroupTagEl {
    pub fn build(self) -> AutoscalingGroupTagEl {
        AutoscalingGroupTagEl {
            key: self.key,
            propagate_at_launch: self.propagate_at_launch,
            value: self.value,
        }
    }
}

pub struct AutoscalingGroupTagElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupTagElRef {
    fn new(shared: StackShared, base: String) -> AutoscalingGroupTagElRef {
        AutoscalingGroupTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupTagElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `propagate_at_launch` after provisioning.\n"]
    pub fn propagate_at_launch(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.propagate_at_launch", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl AutoscalingGroupTimeoutsEl {
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

impl ToListMappable for AutoscalingGroupTimeoutsEl {
    type O = BlockAssignable<AutoscalingGroupTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupTimeoutsEl {}

impl BuildAutoscalingGroupTimeoutsEl {
    pub fn build(self) -> AutoscalingGroupTimeoutsEl {
        AutoscalingGroupTimeoutsEl {
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> AutoscalingGroupTimeoutsElRef {
        AutoscalingGroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
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
pub struct AutoscalingGroupTrafficSourceEl {
    identifier: PrimField<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl AutoscalingGroupTrafficSourceEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for AutoscalingGroupTrafficSourceEl {
    type O = BlockAssignable<AutoscalingGroupTrafficSourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupTrafficSourceEl {
    #[doc = ""]
    pub identifier: PrimField<String>,
}

impl BuildAutoscalingGroupTrafficSourceEl {
    pub fn build(self) -> AutoscalingGroupTrafficSourceEl {
        AutoscalingGroupTrafficSourceEl {
            identifier: self.identifier,
            type_: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupTrafficSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupTrafficSourceElRef {
    fn new(shared: StackShared, base: String) -> AutoscalingGroupTrafficSourceElRef {
        AutoscalingGroupTrafficSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupTrafficSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupWarmPoolElInstanceReusePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    reuse_on_scale_in: Option<PrimField<bool>>,
}

impl AutoscalingGroupWarmPoolElInstanceReusePolicyEl {
    #[doc = "Set the field `reuse_on_scale_in`.\n"]
    pub fn set_reuse_on_scale_in(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.reuse_on_scale_in = Some(v.into());
        self
    }
}

impl ToListMappable for AutoscalingGroupWarmPoolElInstanceReusePolicyEl {
    type O = BlockAssignable<AutoscalingGroupWarmPoolElInstanceReusePolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupWarmPoolElInstanceReusePolicyEl {}

impl BuildAutoscalingGroupWarmPoolElInstanceReusePolicyEl {
    pub fn build(self) -> AutoscalingGroupWarmPoolElInstanceReusePolicyEl {
        AutoscalingGroupWarmPoolElInstanceReusePolicyEl {
            reuse_on_scale_in: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupWarmPoolElInstanceReusePolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupWarmPoolElInstanceReusePolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingGroupWarmPoolElInstanceReusePolicyElRef {
        AutoscalingGroupWarmPoolElInstanceReusePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupWarmPoolElInstanceReusePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `reuse_on_scale_in` after provisioning.\n"]
    pub fn reuse_on_scale_in(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reuse_on_scale_in", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct AutoscalingGroupWarmPoolElDynamic {
    instance_reuse_policy: Option<DynamicBlock<AutoscalingGroupWarmPoolElInstanceReusePolicyEl>>,
}

#[derive(Serialize)]
pub struct AutoscalingGroupWarmPoolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_group_prepared_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pool_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_reuse_policy: Option<Vec<AutoscalingGroupWarmPoolElInstanceReusePolicyEl>>,
    dynamic: AutoscalingGroupWarmPoolElDynamic,
}

impl AutoscalingGroupWarmPoolEl {
    #[doc = "Set the field `max_group_prepared_capacity`.\n"]
    pub fn set_max_group_prepared_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_group_prepared_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `min_size`.\n"]
    pub fn set_min_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_size = Some(v.into());
        self
    }

    #[doc = "Set the field `pool_state`.\n"]
    pub fn set_pool_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pool_state = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_reuse_policy`.\n"]
    pub fn set_instance_reuse_policy(
        mut self,
        v: impl Into<BlockAssignable<AutoscalingGroupWarmPoolElInstanceReusePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.instance_reuse_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_reuse_policy = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AutoscalingGroupWarmPoolEl {
    type O = BlockAssignable<AutoscalingGroupWarmPoolEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupWarmPoolEl {}

impl BuildAutoscalingGroupWarmPoolEl {
    pub fn build(self) -> AutoscalingGroupWarmPoolEl {
        AutoscalingGroupWarmPoolEl {
            max_group_prepared_capacity: core::default::Default::default(),
            min_size: core::default::Default::default(),
            pool_state: core::default::Default::default(),
            instance_reuse_policy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AutoscalingGroupWarmPoolElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingGroupWarmPoolElRef {
    fn new(shared: StackShared, base: String) -> AutoscalingGroupWarmPoolElRef {
        AutoscalingGroupWarmPoolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupWarmPoolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_group_prepared_capacity` after provisioning.\n"]
    pub fn max_group_prepared_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_group_prepared_capacity", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `min_size` after provisioning.\n"]
    pub fn min_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_size", self.base))
    }

    #[doc = "Get a reference to the value of field `pool_state` after provisioning.\n"]
    pub fn pool_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pool_state", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_reuse_policy` after provisioning.\n"]
    pub fn instance_reuse_policy(
        &self,
    ) -> ListRef<AutoscalingGroupWarmPoolElInstanceReusePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_reuse_policy", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct AutoscalingGroupDynamic {
    availability_zone_distribution:
        Option<DynamicBlock<AutoscalingGroupAvailabilityZoneDistributionEl>>,
    capacity_reservation_specification:
        Option<DynamicBlock<AutoscalingGroupCapacityReservationSpecificationEl>>,
    initial_lifecycle_hook: Option<DynamicBlock<AutoscalingGroupInitialLifecycleHookEl>>,
    instance_maintenance_policy: Option<DynamicBlock<AutoscalingGroupInstanceMaintenancePolicyEl>>,
    instance_refresh: Option<DynamicBlock<AutoscalingGroupInstanceRefreshEl>>,
    launch_template: Option<DynamicBlock<AutoscalingGroupLaunchTemplateEl>>,
    mixed_instances_policy: Option<DynamicBlock<AutoscalingGroupMixedInstancesPolicyEl>>,
    tag: Option<DynamicBlock<AutoscalingGroupTagEl>>,
    traffic_source: Option<DynamicBlock<AutoscalingGroupTrafficSourceEl>>,
    warm_pool: Option<DynamicBlock<AutoscalingGroupWarmPoolEl>>,
}
