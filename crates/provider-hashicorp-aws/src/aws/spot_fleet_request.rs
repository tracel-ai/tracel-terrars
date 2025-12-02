use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SpotFleetRequestData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allocation_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excess_capacity_termination_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fleet_type: Option<PrimField<String>>,
    iam_fleet_role: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_interruption_behaviour: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_pools_to_use_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_demand_allocation_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_demand_max_total_price: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_demand_target_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace_unhealthy_instances: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot_price: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    target_capacity: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_capacity_unit_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_group_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terminate_instances_on_delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terminate_instances_with_expiration: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_from: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_until: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for_fulfillment: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_specification: Option<Vec<SpotFleetRequestLaunchSpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_config: Option<Vec<SpotFleetRequestLaunchTemplateConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot_maintenance_strategies: Option<Vec<SpotFleetRequestSpotMaintenanceStrategiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SpotFleetRequestTimeoutsEl>,
    dynamic: SpotFleetRequestDynamic,
}
struct SpotFleetRequest_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SpotFleetRequestData>,
}
#[derive(Clone)]
pub struct SpotFleetRequest(Rc<SpotFleetRequest_>);
impl SpotFleetRequest {
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
    #[doc = "Set the field `allocation_strategy`.\n"]
    pub fn set_allocation_strategy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().allocation_strategy = Some(v.into());
        self
    }
    #[doc = "Set the field `context`.\n"]
    pub fn set_context(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().context = Some(v.into());
        self
    }
    #[doc = "Set the field `excess_capacity_termination_policy`.\n"]
    pub fn set_excess_capacity_termination_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().excess_capacity_termination_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `fleet_type`.\n"]
    pub fn set_fleet_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().fleet_type = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_interruption_behaviour`.\n"]
    pub fn set_instance_interruption_behaviour(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().instance_interruption_behaviour = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_pools_to_use_count`.\n"]
    pub fn set_instance_pools_to_use_count(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().instance_pools_to_use_count = Some(v.into());
        self
    }
    #[doc = "Set the field `load_balancers`.\n"]
    pub fn set_load_balancers(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().load_balancers = Some(v.into());
        self
    }
    #[doc = "Set the field `on_demand_allocation_strategy`.\n"]
    pub fn set_on_demand_allocation_strategy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().on_demand_allocation_strategy = Some(v.into());
        self
    }
    #[doc = "Set the field `on_demand_max_total_price`.\n"]
    pub fn set_on_demand_max_total_price(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().on_demand_max_total_price = Some(v.into());
        self
    }
    #[doc = "Set the field `on_demand_target_capacity`.\n"]
    pub fn set_on_demand_target_capacity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().on_demand_target_capacity = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `replace_unhealthy_instances`.\n"]
    pub fn set_replace_unhealthy_instances(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().replace_unhealthy_instances = Some(v.into());
        self
    }
    #[doc = "Set the field `spot_price`.\n"]
    pub fn set_spot_price(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().spot_price = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }
    #[doc = "Set the field `target_capacity_unit_type`.\n"]
    pub fn set_target_capacity_unit_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().target_capacity_unit_type = Some(v.into());
        self
    }
    #[doc = "Set the field `target_group_arns`.\n"]
    pub fn set_target_group_arns(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().target_group_arns = Some(v.into());
        self
    }
    #[doc = "Set the field `terminate_instances_on_delete`.\n"]
    pub fn set_terminate_instances_on_delete(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().terminate_instances_on_delete = Some(v.into());
        self
    }
    #[doc = "Set the field `terminate_instances_with_expiration`.\n"]
    pub fn set_terminate_instances_with_expiration(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().terminate_instances_with_expiration = Some(v.into());
        self
    }
    #[doc = "Set the field `valid_from`.\n"]
    pub fn set_valid_from(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().valid_from = Some(v.into());
        self
    }
    #[doc = "Set the field `valid_until`.\n"]
    pub fn set_valid_until(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().valid_until = Some(v.into());
        self
    }
    #[doc = "Set the field `wait_for_fulfillment`.\n"]
    pub fn set_wait_for_fulfillment(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_fulfillment = Some(v.into());
        self
    }
    #[doc = "Set the field `launch_specification`.\n"]
    pub fn set_launch_specification(
        self,
        v: impl Into<BlockAssignable<SpotFleetRequestLaunchSpecificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().launch_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.launch_specification = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `launch_template_config`.\n"]
    pub fn set_launch_template_config(
        self,
        v: impl Into<BlockAssignable<SpotFleetRequestLaunchTemplateConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().launch_template_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.launch_template_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `spot_maintenance_strategies`.\n"]
    pub fn set_spot_maintenance_strategies(
        self,
        v: impl Into<BlockAssignable<SpotFleetRequestSpotMaintenanceStrategiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spot_maintenance_strategies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spot_maintenance_strategies = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SpotFleetRequestTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `allocation_strategy` after provisioning.\n"]
    pub fn allocation_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocation_strategy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_token` after provisioning.\n"]
    pub fn client_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_token", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `context` after provisioning.\n"]
    pub fn context(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.context", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `excess_capacity_termination_policy` after provisioning.\n"]
    pub fn excess_capacity_termination_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.excess_capacity_termination_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_type` after provisioning.\n"]
    pub fn fleet_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fleet_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iam_fleet_role` after provisioning.\n"]
    pub fn iam_fleet_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_fleet_role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_interruption_behaviour` after provisioning.\n"]
    pub fn instance_interruption_behaviour(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_interruption_behaviour", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_pools_to_use_count` after provisioning.\n"]
    pub fn instance_pools_to_use_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_pools_to_use_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancers` after provisioning.\n"]
    pub fn load_balancers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.load_balancers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `on_demand_allocation_strategy` after provisioning.\n"]
    pub fn on_demand_allocation_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_demand_allocation_strategy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `on_demand_max_total_price` after provisioning.\n"]
    pub fn on_demand_max_total_price(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_demand_max_total_price", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `on_demand_target_capacity` after provisioning.\n"]
    pub fn on_demand_target_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_demand_target_capacity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replace_unhealthy_instances` after provisioning.\n"]
    pub fn replace_unhealthy_instances(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replace_unhealthy_instances", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spot_price` after provisioning.\n"]
    pub fn spot_price(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.spot_price", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spot_request_state` after provisioning.\n"]
    pub fn spot_request_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.spot_request_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_capacity` after provisioning.\n"]
    pub fn target_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_capacity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_capacity_unit_type` after provisioning.\n"]
    pub fn target_capacity_unit_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_capacity_unit_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_group_arns` after provisioning.\n"]
    pub fn target_group_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.target_group_arns", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terminate_instances_on_delete` after provisioning.\n"]
    pub fn terminate_instances_on_delete(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.terminate_instances_on_delete", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terminate_instances_with_expiration` after provisioning.\n"]
    pub fn terminate_instances_with_expiration(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.terminate_instances_with_expiration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `valid_from` after provisioning.\n"]
    pub fn valid_from(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.valid_from", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `valid_until` after provisioning.\n"]
    pub fn valid_until(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.valid_until", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `wait_for_fulfillment` after provisioning.\n"]
    pub fn wait_for_fulfillment(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_fulfillment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spot_maintenance_strategies` after provisioning.\n"]
    pub fn spot_maintenance_strategies(
        &self,
    ) -> ListRef<SpotFleetRequestSpotMaintenanceStrategiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spot_maintenance_strategies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SpotFleetRequestTimeoutsElRef {
        SpotFleetRequestTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for SpotFleetRequest {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SpotFleetRequest {}
impl ToListMappable for SpotFleetRequest {
    type O = ListRef<SpotFleetRequestRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SpotFleetRequest_ {
    fn extract_resource_type(&self) -> String {
        "aws_spot_fleet_request".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSpotFleetRequest {
    pub tf_id: String,
    #[doc = ""]
    pub iam_fleet_role: PrimField<String>,
    #[doc = ""]
    pub target_capacity: PrimField<f64>,
}
impl BuildSpotFleetRequest {
    pub fn build(self, stack: &mut Stack) -> SpotFleetRequest {
        let out = SpotFleetRequest(Rc::new(SpotFleetRequest_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SpotFleetRequestData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allocation_strategy: core::default::Default::default(),
                context: core::default::Default::default(),
                excess_capacity_termination_policy: core::default::Default::default(),
                fleet_type: core::default::Default::default(),
                iam_fleet_role: self.iam_fleet_role,
                id: core::default::Default::default(),
                instance_interruption_behaviour: core::default::Default::default(),
                instance_pools_to_use_count: core::default::Default::default(),
                load_balancers: core::default::Default::default(),
                on_demand_allocation_strategy: core::default::Default::default(),
                on_demand_max_total_price: core::default::Default::default(),
                on_demand_target_capacity: core::default::Default::default(),
                region: core::default::Default::default(),
                replace_unhealthy_instances: core::default::Default::default(),
                spot_price: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                target_capacity: self.target_capacity,
                target_capacity_unit_type: core::default::Default::default(),
                target_group_arns: core::default::Default::default(),
                terminate_instances_on_delete: core::default::Default::default(),
                terminate_instances_with_expiration: core::default::Default::default(),
                valid_from: core::default::Default::default(),
                valid_until: core::default::Default::default(),
                wait_for_fulfillment: core::default::Default::default(),
                launch_specification: core::default::Default::default(),
                launch_template_config: core::default::Default::default(),
                spot_maintenance_strategies: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SpotFleetRequestRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpotFleetRequestRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SpotFleetRequestRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allocation_strategy` after provisioning.\n"]
    pub fn allocation_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocation_strategy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_token` after provisioning.\n"]
    pub fn client_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_token", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `context` after provisioning.\n"]
    pub fn context(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.context", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `excess_capacity_termination_policy` after provisioning.\n"]
    pub fn excess_capacity_termination_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.excess_capacity_termination_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_type` after provisioning.\n"]
    pub fn fleet_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fleet_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iam_fleet_role` after provisioning.\n"]
    pub fn iam_fleet_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_fleet_role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_interruption_behaviour` after provisioning.\n"]
    pub fn instance_interruption_behaviour(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_interruption_behaviour", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_pools_to_use_count` after provisioning.\n"]
    pub fn instance_pools_to_use_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_pools_to_use_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancers` after provisioning.\n"]
    pub fn load_balancers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.load_balancers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `on_demand_allocation_strategy` after provisioning.\n"]
    pub fn on_demand_allocation_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_demand_allocation_strategy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `on_demand_max_total_price` after provisioning.\n"]
    pub fn on_demand_max_total_price(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_demand_max_total_price", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `on_demand_target_capacity` after provisioning.\n"]
    pub fn on_demand_target_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_demand_target_capacity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replace_unhealthy_instances` after provisioning.\n"]
    pub fn replace_unhealthy_instances(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replace_unhealthy_instances", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spot_price` after provisioning.\n"]
    pub fn spot_price(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.spot_price", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spot_request_state` after provisioning.\n"]
    pub fn spot_request_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.spot_request_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_capacity` after provisioning.\n"]
    pub fn target_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_capacity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_capacity_unit_type` after provisioning.\n"]
    pub fn target_capacity_unit_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_capacity_unit_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_group_arns` after provisioning.\n"]
    pub fn target_group_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.target_group_arns", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terminate_instances_on_delete` after provisioning.\n"]
    pub fn terminate_instances_on_delete(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.terminate_instances_on_delete", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terminate_instances_with_expiration` after provisioning.\n"]
    pub fn terminate_instances_with_expiration(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.terminate_instances_with_expiration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `valid_from` after provisioning.\n"]
    pub fn valid_from(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.valid_from", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `valid_until` after provisioning.\n"]
    pub fn valid_until(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.valid_until", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `wait_for_fulfillment` after provisioning.\n"]
    pub fn wait_for_fulfillment(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_fulfillment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spot_maintenance_strategies` after provisioning.\n"]
    pub fn spot_maintenance_strategies(
        &self,
    ) -> ListRef<SpotFleetRequestSpotMaintenanceStrategiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spot_maintenance_strategies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SpotFleetRequestTimeoutsElRef {
        SpotFleetRequestTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SpotFleetRequestLaunchSpecificationElEbsBlockDeviceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_on_termination: Option<PrimField<bool>>,
    device_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encrypted: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_type: Option<PrimField<String>>,
}
impl SpotFleetRequestLaunchSpecificationElEbsBlockDeviceEl {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.delete_on_termination = Some(v.into());
        self
    }
    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.encrypted = Some(v.into());
        self
    }
    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iops = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }
    #[doc = "Set the field `snapshot_id`.\n"]
    pub fn set_snapshot_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.snapshot_id = Some(v.into());
        self
    }
    #[doc = "Set the field `throughput`.\n"]
    pub fn set_throughput(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.throughput = Some(v.into());
        self
    }
    #[doc = "Set the field `volume_size`.\n"]
    pub fn set_volume_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volume_size = Some(v.into());
        self
    }
    #[doc = "Set the field `volume_type`.\n"]
    pub fn set_volume_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.volume_type = Some(v.into());
        self
    }
}
impl ToListMappable for SpotFleetRequestLaunchSpecificationElEbsBlockDeviceEl {
    type O = BlockAssignable<SpotFleetRequestLaunchSpecificationElEbsBlockDeviceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpotFleetRequestLaunchSpecificationElEbsBlockDeviceEl {
    #[doc = ""]
    pub device_name: PrimField<String>,
}
impl BuildSpotFleetRequestLaunchSpecificationElEbsBlockDeviceEl {
    pub fn build(self) -> SpotFleetRequestLaunchSpecificationElEbsBlockDeviceEl {
        SpotFleetRequestLaunchSpecificationElEbsBlockDeviceEl {
            delete_on_termination: core::default::Default::default(),
            device_name: self.device_name,
            encrypted: core::default::Default::default(),
            iops: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            snapshot_id: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}
pub struct SpotFleetRequestLaunchSpecificationElEbsBlockDeviceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpotFleetRequestLaunchSpecificationElEbsBlockDeviceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SpotFleetRequestLaunchSpecificationElEbsBlockDeviceElRef {
        SpotFleetRequestLaunchSpecificationElEbsBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpotFleetRequestLaunchSpecificationElEbsBlockDeviceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_on_termination", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }
    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.base))
    }
    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }
    #[doc = "Get a reference to the value of field `snapshot_id` after provisioning.\n"]
    pub fn snapshot_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_id", self.base))
    }
    #[doc = "Get a reference to the value of field `throughput` after provisioning.\n"]
    pub fn throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }
    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_size", self.base))
    }
    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.base))
    }
}
#[derive(Serialize)]
pub struct SpotFleetRequestLaunchSpecificationElEphemeralBlockDeviceEl {
    device_name: PrimField<String>,
    virtual_name: PrimField<String>,
}
impl SpotFleetRequestLaunchSpecificationElEphemeralBlockDeviceEl {}
impl ToListMappable for SpotFleetRequestLaunchSpecificationElEphemeralBlockDeviceEl {
    type O = BlockAssignable<SpotFleetRequestLaunchSpecificationElEphemeralBlockDeviceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpotFleetRequestLaunchSpecificationElEphemeralBlockDeviceEl {
    #[doc = ""]
    pub device_name: PrimField<String>,
    #[doc = ""]
    pub virtual_name: PrimField<String>,
}
impl BuildSpotFleetRequestLaunchSpecificationElEphemeralBlockDeviceEl {
    pub fn build(self) -> SpotFleetRequestLaunchSpecificationElEphemeralBlockDeviceEl {
        SpotFleetRequestLaunchSpecificationElEphemeralBlockDeviceEl {
            device_name: self.device_name,
            virtual_name: self.virtual_name,
        }
    }
}
pub struct SpotFleetRequestLaunchSpecificationElEphemeralBlockDeviceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpotFleetRequestLaunchSpecificationElEphemeralBlockDeviceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SpotFleetRequestLaunchSpecificationElEphemeralBlockDeviceElRef {
        SpotFleetRequestLaunchSpecificationElEphemeralBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpotFleetRequestLaunchSpecificationElEphemeralBlockDeviceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }
    #[doc = "Get a reference to the value of field `virtual_name` after provisioning.\n"]
    pub fn virtual_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_name", self.base))
    }
}
#[derive(Serialize)]
pub struct SpotFleetRequestLaunchSpecificationElRootBlockDeviceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_on_termination: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encrypted: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_type: Option<PrimField<String>>,
}
impl SpotFleetRequestLaunchSpecificationElRootBlockDeviceEl {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.delete_on_termination = Some(v.into());
        self
    }
    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.encrypted = Some(v.into());
        self
    }
    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iops = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }
    #[doc = "Set the field `throughput`.\n"]
    pub fn set_throughput(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.throughput = Some(v.into());
        self
    }
    #[doc = "Set the field `volume_size`.\n"]
    pub fn set_volume_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volume_size = Some(v.into());
        self
    }
    #[doc = "Set the field `volume_type`.\n"]
    pub fn set_volume_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.volume_type = Some(v.into());
        self
    }
}
impl ToListMappable for SpotFleetRequestLaunchSpecificationElRootBlockDeviceEl {
    type O = BlockAssignable<SpotFleetRequestLaunchSpecificationElRootBlockDeviceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpotFleetRequestLaunchSpecificationElRootBlockDeviceEl {}
impl BuildSpotFleetRequestLaunchSpecificationElRootBlockDeviceEl {
    pub fn build(self) -> SpotFleetRequestLaunchSpecificationElRootBlockDeviceEl {
        SpotFleetRequestLaunchSpecificationElRootBlockDeviceEl {
            delete_on_termination: core::default::Default::default(),
            encrypted: core::default::Default::default(),
            iops: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}
pub struct SpotFleetRequestLaunchSpecificationElRootBlockDeviceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpotFleetRequestLaunchSpecificationElRootBlockDeviceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SpotFleetRequestLaunchSpecificationElRootBlockDeviceElRef {
        SpotFleetRequestLaunchSpecificationElRootBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpotFleetRequestLaunchSpecificationElRootBlockDeviceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_on_termination", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.base))
    }
    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }
    #[doc = "Get a reference to the value of field `throughput` after provisioning.\n"]
    pub fn throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }
    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_size", self.base))
    }
    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.base))
    }
}
#[derive(Serialize, Default)]
struct SpotFleetRequestLaunchSpecificationElDynamic {
    ebs_block_device: Option<DynamicBlock<SpotFleetRequestLaunchSpecificationElEbsBlockDeviceEl>>,
    ephemeral_block_device:
        Option<DynamicBlock<SpotFleetRequestLaunchSpecificationElEphemeralBlockDeviceEl>>,
    root_block_device: Option<DynamicBlock<SpotFleetRequestLaunchSpecificationElRootBlockDeviceEl>>,
}
#[derive(Serialize)]
pub struct SpotFleetRequestLaunchSpecificationEl {
    ami: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    associate_public_ip_address: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_optimized: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_instance_profile: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_instance_profile_arn: Option<PrimField<String>>,
    instance_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitoring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement_tenancy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot_price: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_data: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_capacity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_block_device: Option<Vec<SpotFleetRequestLaunchSpecificationElEbsBlockDeviceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ephemeral_block_device:
        Option<Vec<SpotFleetRequestLaunchSpecificationElEphemeralBlockDeviceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_block_device: Option<Vec<SpotFleetRequestLaunchSpecificationElRootBlockDeviceEl>>,
    dynamic: SpotFleetRequestLaunchSpecificationElDynamic,
}
impl SpotFleetRequestLaunchSpecificationEl {
    #[doc = "Set the field `associate_public_ip_address`.\n"]
    pub fn set_associate_public_ip_address(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.associate_public_ip_address = Some(v.into());
        self
    }
    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.availability_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `ebs_optimized`.\n"]
    pub fn set_ebs_optimized(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ebs_optimized = Some(v.into());
        self
    }
    #[doc = "Set the field `iam_instance_profile`.\n"]
    pub fn set_iam_instance_profile(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.iam_instance_profile = Some(v.into());
        self
    }
    #[doc = "Set the field `iam_instance_profile_arn`.\n"]
    pub fn set_iam_instance_profile_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.iam_instance_profile_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `key_name`.\n"]
    pub fn set_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `monitoring`.\n"]
    pub fn set_monitoring(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.monitoring = Some(v.into());
        self
    }
    #[doc = "Set the field `placement_group`.\n"]
    pub fn set_placement_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.placement_group = Some(v.into());
        self
    }
    #[doc = "Set the field `placement_tenancy`.\n"]
    pub fn set_placement_tenancy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.placement_tenancy = Some(v.into());
        self
    }
    #[doc = "Set the field `spot_price`.\n"]
    pub fn set_spot_price(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.spot_price = Some(v.into());
        self
    }
    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnet_id = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
    #[doc = "Set the field `user_data`.\n"]
    pub fn set_user_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_data = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_security_group_ids`.\n"]
    pub fn set_vpc_security_group_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.vpc_security_group_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `weighted_capacity`.\n"]
    pub fn set_weighted_capacity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.weighted_capacity = Some(v.into());
        self
    }
    #[doc = "Set the field `ebs_block_device`.\n"]
    pub fn set_ebs_block_device(
        mut self,
        v: impl Into<BlockAssignable<SpotFleetRequestLaunchSpecificationElEbsBlockDeviceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ebs_block_device = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ebs_block_device = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ephemeral_block_device`.\n"]
    pub fn set_ephemeral_block_device(
        mut self,
        v: impl Into<BlockAssignable<SpotFleetRequestLaunchSpecificationElEphemeralBlockDeviceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ephemeral_block_device = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ephemeral_block_device = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `root_block_device`.\n"]
    pub fn set_root_block_device(
        mut self,
        v: impl Into<BlockAssignable<SpotFleetRequestLaunchSpecificationElRootBlockDeviceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.root_block_device = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.root_block_device = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SpotFleetRequestLaunchSpecificationEl {
    type O = BlockAssignable<SpotFleetRequestLaunchSpecificationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpotFleetRequestLaunchSpecificationEl {
    #[doc = ""]
    pub ami: PrimField<String>,
    #[doc = ""]
    pub instance_type: PrimField<String>,
}
impl BuildSpotFleetRequestLaunchSpecificationEl {
    pub fn build(self) -> SpotFleetRequestLaunchSpecificationEl {
        SpotFleetRequestLaunchSpecificationEl {
            ami: self.ami,
            associate_public_ip_address: core::default::Default::default(),
            availability_zone: core::default::Default::default(),
            ebs_optimized: core::default::Default::default(),
            iam_instance_profile: core::default::Default::default(),
            iam_instance_profile_arn: core::default::Default::default(),
            instance_type: self.instance_type,
            key_name: core::default::Default::default(),
            monitoring: core::default::Default::default(),
            placement_group: core::default::Default::default(),
            placement_tenancy: core::default::Default::default(),
            spot_price: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
            tags: core::default::Default::default(),
            user_data: core::default::Default::default(),
            vpc_security_group_ids: core::default::Default::default(),
            weighted_capacity: core::default::Default::default(),
            ebs_block_device: core::default::Default::default(),
            ephemeral_block_device: core::default::Default::default(),
            root_block_device: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SpotFleetRequestLaunchSpecificationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpotFleetRequestLaunchSpecificationElRef {
    fn new(shared: StackShared, base: String) -> SpotFleetRequestLaunchSpecificationElRef {
        SpotFleetRequestLaunchSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpotFleetRequestLaunchSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ami` after provisioning.\n"]
    pub fn ami(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ami", self.base))
    }
    #[doc = "Get a reference to the value of field `associate_public_ip_address` after provisioning.\n"]
    pub fn associate_public_ip_address(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.associate_public_ip_address", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ebs_optimized` after provisioning.\n"]
    pub fn ebs_optimized(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ebs_optimized", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `iam_instance_profile` after provisioning.\n"]
    pub fn iam_instance_profile(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_instance_profile", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `iam_instance_profile_arn` after provisioning.\n"]
    pub fn iam_instance_profile_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_instance_profile_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.base))
    }
    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitoring", self.base))
    }
    #[doc = "Get a reference to the value of field `placement_group` after provisioning.\n"]
    pub fn placement_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.placement_group", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `placement_tenancy` after provisioning.\n"]
    pub fn placement_tenancy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.placement_tenancy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `spot_price` after provisioning.\n"]
    pub fn spot_price(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.spot_price", self.base))
    }
    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
    #[doc = "Get a reference to the value of field `user_data` after provisioning.\n"]
    pub fn user_data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_data", self.base))
    }
    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.vpc_security_group_ids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `weighted_capacity` after provisioning.\n"]
    pub fn weighted_capacity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.weighted_capacity", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SpotFleetRequestLaunchTemplateConfigElLaunchTemplateSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl SpotFleetRequestLaunchTemplateConfigElLaunchTemplateSpecificationEl {
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
impl ToListMappable for SpotFleetRequestLaunchTemplateConfigElLaunchTemplateSpecificationEl {
    type O = BlockAssignable<SpotFleetRequestLaunchTemplateConfigElLaunchTemplateSpecificationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpotFleetRequestLaunchTemplateConfigElLaunchTemplateSpecificationEl {}
impl BuildSpotFleetRequestLaunchTemplateConfigElLaunchTemplateSpecificationEl {
    pub fn build(self) -> SpotFleetRequestLaunchTemplateConfigElLaunchTemplateSpecificationEl {
        SpotFleetRequestLaunchTemplateConfigElLaunchTemplateSpecificationEl {
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct SpotFleetRequestLaunchTemplateConfigElLaunchTemplateSpecificationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpotFleetRequestLaunchTemplateConfigElLaunchTemplateSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SpotFleetRequestLaunchTemplateConfigElLaunchTemplateSpecificationElRef {
        SpotFleetRequestLaunchTemplateConfigElLaunchTemplateSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpotFleetRequestLaunchTemplateConfigElLaunchTemplateSpecificationElRef {
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
pub struct SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorCountEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorCountEl {
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
impl ToListMappable
    for SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorCountEl
{
    type O = BlockAssignable<
        SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorCountEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorCountEl
{}
impl
    BuildSpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorCountEl
{
    pub fn build(
        self,
    ) -> SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorCountEl
    {
        SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorCountEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}
pub struct SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorCountElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorCountElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorCountElRef
    {
        SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorCountElRef { shared : shared , base : base . to_string () , }
    }
}
impl SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorCountElRef {
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
pub struct SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorTotalMemoryMibEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorTotalMemoryMibEl { # [doc = "Set the field `max`.\n"] pub fn set_max (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . max = Some (v . into ()) ; self } # [doc = "Set the field `min`.\n"] pub fn set_min (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . min = Some (v . into ()) ; self } }
impl ToListMappable for SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorTotalMemoryMibEl { type O = BlockAssignable < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorTotalMemoryMibEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorTotalMemoryMibEl
{}
impl BuildSpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorTotalMemoryMibEl { pub fn build (self) -> SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorTotalMemoryMibEl { SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorTotalMemoryMibEl { max : core :: default :: Default :: default () , min : core :: default :: Default :: default () , } } }
pub struct SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorTotalMemoryMibElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorTotalMemoryMibElRef { fn new (shared : StackShared , base : String) -> SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorTotalMemoryMibElRef { SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorTotalMemoryMibElRef { shared : shared , base : base . to_string () , } } }
impl SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorTotalMemoryMibElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `max` after provisioning.\n"] pub fn max (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max" , self . base)) } # [doc = "Get a reference to the value of field `min` after provisioning.\n"] pub fn min (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.min" , self . base)) } }
#[derive(Serialize)]
pub struct SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElBaselineEbsBandwidthMbpsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElBaselineEbsBandwidthMbpsEl { # [doc = "Set the field `max`.\n"] pub fn set_max (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . max = Some (v . into ()) ; self } # [doc = "Set the field `min`.\n"] pub fn set_min (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . min = Some (v . into ()) ; self } }
impl ToListMappable for SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElBaselineEbsBandwidthMbpsEl { type O = BlockAssignable < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElBaselineEbsBandwidthMbpsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElBaselineEbsBandwidthMbpsEl
{}
impl BuildSpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElBaselineEbsBandwidthMbpsEl { pub fn build (self) -> SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElBaselineEbsBandwidthMbpsEl { SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElBaselineEbsBandwidthMbpsEl { max : core :: default :: Default :: default () , min : core :: default :: Default :: default () , } } }
pub struct SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef { fn new (shared : StackShared , base : String) -> SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef { SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef { shared : shared , base : base . to_string () , } } }
impl SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `max` after provisioning.\n"] pub fn max (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max" , self . base)) } # [doc = "Get a reference to the value of field `min` after provisioning.\n"] pub fn min (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.min" , self . base)) } }
#[derive(Serialize)]
pub struct SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryGibPerVcpuEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryGibPerVcpuEl {
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
impl ToListMappable
    for SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryGibPerVcpuEl
{
    type O = BlockAssignable<
        SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryGibPerVcpuEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryGibPerVcpuEl
{}
impl
    BuildSpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryGibPerVcpuEl
{
    pub fn build(
        self,
    ) -> SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryGibPerVcpuEl
    {
        SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryGibPerVcpuEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}
pub struct SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryGibPerVcpuElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryGibPerVcpuElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryGibPerVcpuElRef
    {
        SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryGibPerVcpuElRef { shared : shared , base : base . to_string () , }
    }
}
impl SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryGibPerVcpuElRef {
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
pub struct SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryMibEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryMibEl {
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
impl ToListMappable
    for SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryMibEl
{
    type O = BlockAssignable<
        SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryMibEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryMibEl
{}
impl BuildSpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryMibEl {
    pub fn build(
        self,
    ) -> SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryMibEl {
        SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryMibEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}
pub struct SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryMibElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryMibElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryMibElRef {
        SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryMibElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryMibElRef {
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
pub struct SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkBandwidthGbpsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkBandwidthGbpsEl {
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
impl ToListMappable for SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkBandwidthGbpsEl { type O = BlockAssignable < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkBandwidthGbpsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkBandwidthGbpsEl
{}
impl BuildSpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkBandwidthGbpsEl { pub fn build (self) -> SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkBandwidthGbpsEl { SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkBandwidthGbpsEl { max : core :: default :: Default :: default () , min : core :: default :: Default :: default () , } } }
pub struct SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkBandwidthGbpsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkBandwidthGbpsElRef { fn new (shared : StackShared , base : String) -> SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkBandwidthGbpsElRef { SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkBandwidthGbpsElRef { shared : shared , base : base . to_string () , } } }
impl
    SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkBandwidthGbpsElRef
{
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
pub struct SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkInterfaceCountEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl
    SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkInterfaceCountEl
{
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
impl ToListMappable for SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkInterfaceCountEl { type O = BlockAssignable < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkInterfaceCountEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkInterfaceCountEl
{}
impl BuildSpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkInterfaceCountEl { pub fn build (self) -> SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkInterfaceCountEl { SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkInterfaceCountEl { max : core :: default :: Default :: default () , min : core :: default :: Default :: default () , } } }
pub struct SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkInterfaceCountElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkInterfaceCountElRef { fn new (shared : StackShared , base : String) -> SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkInterfaceCountElRef { SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkInterfaceCountElRef { shared : shared , base : base . to_string () , } } }
impl SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkInterfaceCountElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `max` after provisioning.\n"] pub fn max (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max" , self . base)) } # [doc = "Get a reference to the value of field `min` after provisioning.\n"] pub fn min (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.min" , self . base)) } }
#[derive(Serialize)]
pub struct SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElTotalLocalStorageGbEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElTotalLocalStorageGbEl {
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
impl ToListMappable
    for SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElTotalLocalStorageGbEl
{
    type O = BlockAssignable < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElTotalLocalStorageGbEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElTotalLocalStorageGbEl
{}
impl BuildSpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElTotalLocalStorageGbEl { pub fn build (self) -> SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElTotalLocalStorageGbEl { SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElTotalLocalStorageGbEl { max : core :: default :: Default :: default () , min : core :: default :: Default :: default () , } } }
pub struct SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElTotalLocalStorageGbElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElTotalLocalStorageGbElRef { fn new (shared : StackShared , base : String) -> SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElTotalLocalStorageGbElRef { SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElTotalLocalStorageGbElRef { shared : shared , base : base . to_string () , } } }
impl
    SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElTotalLocalStorageGbElRef
{
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
pub struct SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElVcpuCountEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElVcpuCountEl {
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
impl ToListMappable
    for SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElVcpuCountEl
{
    type O = BlockAssignable<
        SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElVcpuCountEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElVcpuCountEl
{}
impl BuildSpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElVcpuCountEl {
    pub fn build(
        self,
    ) -> SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElVcpuCountEl {
        SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElVcpuCountEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}
pub struct SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElVcpuCountElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElVcpuCountElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElVcpuCountElRef {
        SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElVcpuCountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElVcpuCountElRef {
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
struct SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElDynamic { accelerator_count : Option < DynamicBlock < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorCountEl >> , accelerator_total_memory_mib : Option < DynamicBlock < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorTotalMemoryMibEl >> , baseline_ebs_bandwidth_mbps : Option < DynamicBlock < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElBaselineEbsBandwidthMbpsEl >> , memory_gib_per_vcpu : Option < DynamicBlock < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryGibPerVcpuEl >> , memory_mib : Option < DynamicBlock < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryMibEl >> , network_bandwidth_gbps : Option < DynamicBlock < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkBandwidthGbpsEl >> , network_interface_count : Option < DynamicBlock < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkInterfaceCountEl >> , total_local_storage_gb : Option < DynamicBlock < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElTotalLocalStorageGbEl >> , vcpu_count : Option < DynamicBlock < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElVcpuCountEl >> , }
#[derive(Serialize)]
pub struct SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsEl { # [serde (skip_serializing_if = "Option::is_none")] accelerator_manufacturers : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] accelerator_names : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] accelerator_types : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] allowed_instance_types : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] bare_metal : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] burstable_performance : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] cpu_manufacturers : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] excluded_instance_types : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] instance_generations : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] local_storage : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] local_storage_types : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] on_demand_max_price_percentage_over_lowest_price : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] require_hibernate_support : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] spot_max_price_percentage_over_lowest_price : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] accelerator_count : Option < Vec < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorCountEl > > , # [serde (skip_serializing_if = "Option::is_none")] accelerator_total_memory_mib : Option < Vec < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorTotalMemoryMibEl > > , # [serde (skip_serializing_if = "Option::is_none")] baseline_ebs_bandwidth_mbps : Option < Vec < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElBaselineEbsBandwidthMbpsEl > > , # [serde (skip_serializing_if = "Option::is_none")] memory_gib_per_vcpu : Option < Vec < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryGibPerVcpuEl > > , # [serde (skip_serializing_if = "Option::is_none")] memory_mib : Option < Vec < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryMibEl > > , # [serde (skip_serializing_if = "Option::is_none")] network_bandwidth_gbps : Option < Vec < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkBandwidthGbpsEl > > , # [serde (skip_serializing_if = "Option::is_none")] network_interface_count : Option < Vec < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkInterfaceCountEl > > , # [serde (skip_serializing_if = "Option::is_none")] total_local_storage_gb : Option < Vec < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElTotalLocalStorageGbEl > > , # [serde (skip_serializing_if = "Option::is_none")] vcpu_count : Option < Vec < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElVcpuCountEl > > , dynamic : SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElDynamic , }
impl SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsEl {
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
        v : impl Into < BlockAssignable < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorCountEl >>,
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
        v : impl Into < BlockAssignable < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorTotalMemoryMibEl >>,
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
        v : impl Into < BlockAssignable < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElBaselineEbsBandwidthMbpsEl >>,
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
        v : impl Into < BlockAssignable < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryGibPerVcpuEl >>,
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
        v: impl Into<
            BlockAssignable<
                SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryMibEl,
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
        v : impl Into < BlockAssignable < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkBandwidthGbpsEl >>,
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
        v : impl Into < BlockAssignable < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkInterfaceCountEl >>,
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
        v : impl Into < BlockAssignable < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElTotalLocalStorageGbEl >>,
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
        v: impl Into<
            BlockAssignable<
                SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElVcpuCountEl,
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
impl ToListMappable for SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsEl {
    type O =
        BlockAssignable<SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsEl {}
impl BuildSpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsEl {
    pub fn build(self) -> SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsEl {
        SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsEl {
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
pub struct SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElRef {
        SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElRef {
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
    #[doc = "Get a reference to the value of field `accelerator_count` after provisioning.\n"]    pub fn accelerator_count (& self) -> ListRef < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorCountElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.accelerator_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `accelerator_total_memory_mib` after provisioning.\n"]    pub fn accelerator_total_memory_mib (& self) -> ListRef < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElAcceleratorTotalMemoryMibElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.accelerator_total_memory_mib", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `baseline_ebs_bandwidth_mbps` after provisioning.\n"]    pub fn baseline_ebs_bandwidth_mbps (& self) -> ListRef < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.baseline_ebs_bandwidth_mbps", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_gib_per_vcpu` after provisioning.\n"]    pub fn memory_gib_per_vcpu (& self) -> ListRef < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryGibPerVcpuElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.memory_gib_per_vcpu", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_mib` after provisioning.\n"]
    pub fn memory_mib(
        &self,
    ) -> ListRef<
        SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElMemoryMibElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.memory_mib", self.base))
    }
    #[doc = "Get a reference to the value of field `network_bandwidth_gbps` after provisioning.\n"]    pub fn network_bandwidth_gbps (& self) -> ListRef < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkBandwidthGbpsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_bandwidth_gbps", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_interface_count` after provisioning.\n"]    pub fn network_interface_count (& self) -> ListRef < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElNetworkInterfaceCountElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interface_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_local_storage_gb` after provisioning.\n"]    pub fn total_local_storage_gb (& self) -> ListRef < SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElTotalLocalStorageGbElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.total_local_storage_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vcpu_count` after provisioning.\n"]
    pub fn vcpu_count(
        &self,
    ) -> ListRef<
        SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElVcpuCountElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.vcpu_count", self.base))
    }
}
#[derive(Serialize, Default)]
struct SpotFleetRequestLaunchTemplateConfigElOverridesElDynamic {
    instance_requirements: Option<
        DynamicBlock<SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsEl>,
    >,
}
#[derive(Serialize)]
pub struct SpotFleetRequestLaunchTemplateConfigElOverridesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot_price: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_requirements:
        Option<Vec<SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsEl>>,
    dynamic: SpotFleetRequestLaunchTemplateConfigElOverridesElDynamic,
}
impl SpotFleetRequestLaunchTemplateConfigElOverridesEl {
    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.availability_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }
    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }
    #[doc = "Set the field `spot_price`.\n"]
    pub fn set_spot_price(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.spot_price = Some(v.into());
        self
    }
    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnet_id = Some(v.into());
        self
    }
    #[doc = "Set the field `weighted_capacity`.\n"]
    pub fn set_weighted_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weighted_capacity = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_requirements`.\n"]
    pub fn set_instance_requirements(
        mut self,
        v: impl Into<
            BlockAssignable<
                SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsEl,
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
}
impl ToListMappable for SpotFleetRequestLaunchTemplateConfigElOverridesEl {
    type O = BlockAssignable<SpotFleetRequestLaunchTemplateConfigElOverridesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpotFleetRequestLaunchTemplateConfigElOverridesEl {}
impl BuildSpotFleetRequestLaunchTemplateConfigElOverridesEl {
    pub fn build(self) -> SpotFleetRequestLaunchTemplateConfigElOverridesEl {
        SpotFleetRequestLaunchTemplateConfigElOverridesEl {
            availability_zone: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            priority: core::default::Default::default(),
            spot_price: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
            weighted_capacity: core::default::Default::default(),
            instance_requirements: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SpotFleetRequestLaunchTemplateConfigElOverridesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpotFleetRequestLaunchTemplateConfigElOverridesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SpotFleetRequestLaunchTemplateConfigElOverridesElRef {
        SpotFleetRequestLaunchTemplateConfigElOverridesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpotFleetRequestLaunchTemplateConfigElOverridesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
    #[doc = "Get a reference to the value of field `spot_price` after provisioning.\n"]
    pub fn spot_price(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.spot_price", self.base))
    }
    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
    #[doc = "Get a reference to the value of field `weighted_capacity` after provisioning.\n"]
    pub fn weighted_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.weighted_capacity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance_requirements` after provisioning.\n"]
    pub fn instance_requirements(
        &self,
    ) -> ListRef<SpotFleetRequestLaunchTemplateConfigElOverridesElInstanceRequirementsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_requirements", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SpotFleetRequestLaunchTemplateConfigElDynamic {
    launch_template_specification:
        Option<DynamicBlock<SpotFleetRequestLaunchTemplateConfigElLaunchTemplateSpecificationEl>>,
    overrides: Option<DynamicBlock<SpotFleetRequestLaunchTemplateConfigElOverridesEl>>,
}
#[derive(Serialize)]
pub struct SpotFleetRequestLaunchTemplateConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_specification:
        Option<Vec<SpotFleetRequestLaunchTemplateConfigElLaunchTemplateSpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overrides: Option<Vec<SpotFleetRequestLaunchTemplateConfigElOverridesEl>>,
    dynamic: SpotFleetRequestLaunchTemplateConfigElDynamic,
}
impl SpotFleetRequestLaunchTemplateConfigEl {
    #[doc = "Set the field `launch_template_specification`.\n"]
    pub fn set_launch_template_specification(
        mut self,
        v: impl Into<
            BlockAssignable<SpotFleetRequestLaunchTemplateConfigElLaunchTemplateSpecificationEl>,
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
    #[doc = "Set the field `overrides`.\n"]
    pub fn set_overrides(
        mut self,
        v: impl Into<BlockAssignable<SpotFleetRequestLaunchTemplateConfigElOverridesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.overrides = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.overrides = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SpotFleetRequestLaunchTemplateConfigEl {
    type O = BlockAssignable<SpotFleetRequestLaunchTemplateConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpotFleetRequestLaunchTemplateConfigEl {}
impl BuildSpotFleetRequestLaunchTemplateConfigEl {
    pub fn build(self) -> SpotFleetRequestLaunchTemplateConfigEl {
        SpotFleetRequestLaunchTemplateConfigEl {
            launch_template_specification: core::default::Default::default(),
            overrides: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SpotFleetRequestLaunchTemplateConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpotFleetRequestLaunchTemplateConfigElRef {
    fn new(shared: StackShared, base: String) -> SpotFleetRequestLaunchTemplateConfigElRef {
        SpotFleetRequestLaunchTemplateConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpotFleetRequestLaunchTemplateConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `launch_template_specification` after provisioning.\n"]
    pub fn launch_template_specification(
        &self,
    ) -> ListRef<SpotFleetRequestLaunchTemplateConfigElLaunchTemplateSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.launch_template_specification", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SpotFleetRequestSpotMaintenanceStrategiesElCapacityRebalanceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    replacement_strategy: Option<PrimField<String>>,
}
impl SpotFleetRequestSpotMaintenanceStrategiesElCapacityRebalanceEl {
    #[doc = "Set the field `replacement_strategy`.\n"]
    pub fn set_replacement_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.replacement_strategy = Some(v.into());
        self
    }
}
impl ToListMappable for SpotFleetRequestSpotMaintenanceStrategiesElCapacityRebalanceEl {
    type O = BlockAssignable<SpotFleetRequestSpotMaintenanceStrategiesElCapacityRebalanceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpotFleetRequestSpotMaintenanceStrategiesElCapacityRebalanceEl {}
impl BuildSpotFleetRequestSpotMaintenanceStrategiesElCapacityRebalanceEl {
    pub fn build(self) -> SpotFleetRequestSpotMaintenanceStrategiesElCapacityRebalanceEl {
        SpotFleetRequestSpotMaintenanceStrategiesElCapacityRebalanceEl {
            replacement_strategy: core::default::Default::default(),
        }
    }
}
pub struct SpotFleetRequestSpotMaintenanceStrategiesElCapacityRebalanceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpotFleetRequestSpotMaintenanceStrategiesElCapacityRebalanceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SpotFleetRequestSpotMaintenanceStrategiesElCapacityRebalanceElRef {
        SpotFleetRequestSpotMaintenanceStrategiesElCapacityRebalanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpotFleetRequestSpotMaintenanceStrategiesElCapacityRebalanceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `replacement_strategy` after provisioning.\n"]
    pub fn replacement_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replacement_strategy", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SpotFleetRequestSpotMaintenanceStrategiesElDynamic {
    capacity_rebalance:
        Option<DynamicBlock<SpotFleetRequestSpotMaintenanceStrategiesElCapacityRebalanceEl>>,
}
#[derive(Serialize)]
pub struct SpotFleetRequestSpotMaintenanceStrategiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_rebalance: Option<Vec<SpotFleetRequestSpotMaintenanceStrategiesElCapacityRebalanceEl>>,
    dynamic: SpotFleetRequestSpotMaintenanceStrategiesElDynamic,
}
impl SpotFleetRequestSpotMaintenanceStrategiesEl {
    #[doc = "Set the field `capacity_rebalance`.\n"]
    pub fn set_capacity_rebalance(
        mut self,
        v: impl Into<BlockAssignable<SpotFleetRequestSpotMaintenanceStrategiesElCapacityRebalanceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.capacity_rebalance = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.capacity_rebalance = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SpotFleetRequestSpotMaintenanceStrategiesEl {
    type O = BlockAssignable<SpotFleetRequestSpotMaintenanceStrategiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpotFleetRequestSpotMaintenanceStrategiesEl {}
impl BuildSpotFleetRequestSpotMaintenanceStrategiesEl {
    pub fn build(self) -> SpotFleetRequestSpotMaintenanceStrategiesEl {
        SpotFleetRequestSpotMaintenanceStrategiesEl {
            capacity_rebalance: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SpotFleetRequestSpotMaintenanceStrategiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpotFleetRequestSpotMaintenanceStrategiesElRef {
    fn new(shared: StackShared, base: String) -> SpotFleetRequestSpotMaintenanceStrategiesElRef {
        SpotFleetRequestSpotMaintenanceStrategiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpotFleetRequestSpotMaintenanceStrategiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `capacity_rebalance` after provisioning.\n"]
    pub fn capacity_rebalance(
        &self,
    ) -> ListRef<SpotFleetRequestSpotMaintenanceStrategiesElCapacityRebalanceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_rebalance", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SpotFleetRequestTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl SpotFleetRequestTimeoutsEl {
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
impl ToListMappable for SpotFleetRequestTimeoutsEl {
    type O = BlockAssignable<SpotFleetRequestTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpotFleetRequestTimeoutsEl {}
impl BuildSpotFleetRequestTimeoutsEl {
    pub fn build(self) -> SpotFleetRequestTimeoutsEl {
        SpotFleetRequestTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct SpotFleetRequestTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpotFleetRequestTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SpotFleetRequestTimeoutsElRef {
        SpotFleetRequestTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpotFleetRequestTimeoutsElRef {
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
struct SpotFleetRequestDynamic {
    launch_specification: Option<DynamicBlock<SpotFleetRequestLaunchSpecificationEl>>,
    launch_template_config: Option<DynamicBlock<SpotFleetRequestLaunchTemplateConfigEl>>,
    spot_maintenance_strategies: Option<DynamicBlock<SpotFleetRequestSpotMaintenanceStrategiesEl>>,
}
