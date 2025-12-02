use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct Ec2FleetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excess_capacity_termination_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fleet_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fulfilled_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fulfilled_on_demand_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace_unhealthy_instances: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terminate_instances: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terminate_instances_with_expiration: Option<PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_from: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_until: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fleet_instance_set: Option<Vec<Ec2FleetFleetInstanceSetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_config: Option<Vec<Ec2FleetLaunchTemplateConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_demand_options: Option<Vec<Ec2FleetOnDemandOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot_options: Option<Vec<Ec2FleetSpotOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_capacity_specification: Option<Vec<Ec2FleetTargetCapacitySpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<Ec2FleetTimeoutsEl>,
    dynamic: Ec2FleetDynamic,
}
struct Ec2Fleet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Ec2FleetData>,
}
#[derive(Clone)]
pub struct Ec2Fleet(Rc<Ec2Fleet_>);
impl Ec2Fleet {
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
    #[doc = "Set the field `fleet_state`.\n"]
    pub fn set_fleet_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().fleet_state = Some(v.into());
        self
    }
    #[doc = "Set the field `fulfilled_capacity`.\n"]
    pub fn set_fulfilled_capacity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().fulfilled_capacity = Some(v.into());
        self
    }
    #[doc = "Set the field `fulfilled_on_demand_capacity`.\n"]
    pub fn set_fulfilled_on_demand_capacity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().fulfilled_on_demand_capacity = Some(v.into());
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
    #[doc = "Set the field `replace_unhealthy_instances`.\n"]
    pub fn set_replace_unhealthy_instances(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().replace_unhealthy_instances = Some(v.into());
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
    #[doc = "Set the field `terminate_instances`.\n"]
    pub fn set_terminate_instances(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().terminate_instances = Some(v.into());
        self
    }
    #[doc = "Set the field `terminate_instances_with_expiration`.\n"]
    pub fn set_terminate_instances_with_expiration(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().terminate_instances_with_expiration = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
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
    #[doc = "Set the field `fleet_instance_set`.\n"]
    pub fn set_fleet_instance_set(
        self,
        v: impl Into<BlockAssignable<Ec2FleetFleetInstanceSetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().fleet_instance_set = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.fleet_instance_set = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `launch_template_config`.\n"]
    pub fn set_launch_template_config(
        self,
        v: impl Into<BlockAssignable<Ec2FleetLaunchTemplateConfigEl>>,
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
    #[doc = "Set the field `on_demand_options`.\n"]
    pub fn set_on_demand_options(
        self,
        v: impl Into<BlockAssignable<Ec2FleetOnDemandOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().on_demand_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.on_demand_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `spot_options`.\n"]
    pub fn set_spot_options(self, v: impl Into<BlockAssignable<Ec2FleetSpotOptionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spot_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spot_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `target_capacity_specification`.\n"]
    pub fn set_target_capacity_specification(
        self,
        v: impl Into<BlockAssignable<Ec2FleetTargetCapacitySpecificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_capacity_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .target_capacity_specification = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<Ec2FleetTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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
    #[doc = "Get a reference to the value of field `fleet_state` after provisioning.\n"]
    pub fn fleet_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fleet_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fulfilled_capacity` after provisioning.\n"]
    pub fn fulfilled_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fulfilled_capacity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fulfilled_on_demand_capacity` after provisioning.\n"]
    pub fn fulfilled_on_demand_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fulfilled_on_demand_capacity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
    #[doc = "Get a reference to the value of field `terminate_instances` after provisioning.\n"]
    pub fn terminate_instances(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.terminate_instances", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terminate_instances_with_expiration` after provisioning.\n"]
    pub fn terminate_instances_with_expiration(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.terminate_instances_with_expiration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `fleet_instance_set` after provisioning.\n"]
    pub fn fleet_instance_set(&self) -> ListRef<Ec2FleetFleetInstanceSetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleet_instance_set", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `launch_template_config` after provisioning.\n"]
    pub fn launch_template_config(&self) -> ListRef<Ec2FleetLaunchTemplateConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.launch_template_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `on_demand_options` after provisioning.\n"]
    pub fn on_demand_options(&self) -> ListRef<Ec2FleetOnDemandOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.on_demand_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spot_options` after provisioning.\n"]
    pub fn spot_options(&self) -> ListRef<Ec2FleetSpotOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spot_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_capacity_specification` after provisioning.\n"]
    pub fn target_capacity_specification(
        &self,
    ) -> ListRef<Ec2FleetTargetCapacitySpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_capacity_specification", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Ec2FleetTimeoutsElRef {
        Ec2FleetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for Ec2Fleet {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for Ec2Fleet {}
impl ToListMappable for Ec2Fleet {
    type O = ListRef<Ec2FleetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for Ec2Fleet_ {
    fn extract_resource_type(&self) -> String {
        "aws_ec2_fleet".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildEc2Fleet {
    pub tf_id: String,
}
impl BuildEc2Fleet {
    pub fn build(self, stack: &mut Stack) -> Ec2Fleet {
        let out = Ec2Fleet(Rc::new(Ec2Fleet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Ec2FleetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                context: core::default::Default::default(),
                excess_capacity_termination_policy: core::default::Default::default(),
                fleet_state: core::default::Default::default(),
                fulfilled_capacity: core::default::Default::default(),
                fulfilled_on_demand_capacity: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                replace_unhealthy_instances: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                terminate_instances: core::default::Default::default(),
                terminate_instances_with_expiration: core::default::Default::default(),
                type_: core::default::Default::default(),
                valid_from: core::default::Default::default(),
                valid_until: core::default::Default::default(),
                fleet_instance_set: core::default::Default::default(),
                launch_template_config: core::default::Default::default(),
                on_demand_options: core::default::Default::default(),
                spot_options: core::default::Default::default(),
                target_capacity_specification: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct Ec2FleetRef {
    shared: StackShared,
    base: String,
}
impl Ref for Ec2FleetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl Ec2FleetRef {
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
    #[doc = "Get a reference to the value of field `fleet_state` after provisioning.\n"]
    pub fn fleet_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fleet_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fulfilled_capacity` after provisioning.\n"]
    pub fn fulfilled_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fulfilled_capacity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fulfilled_on_demand_capacity` after provisioning.\n"]
    pub fn fulfilled_on_demand_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fulfilled_on_demand_capacity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
    #[doc = "Get a reference to the value of field `terminate_instances` after provisioning.\n"]
    pub fn terminate_instances(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.terminate_instances", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terminate_instances_with_expiration` after provisioning.\n"]
    pub fn terminate_instances_with_expiration(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.terminate_instances_with_expiration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `fleet_instance_set` after provisioning.\n"]
    pub fn fleet_instance_set(&self) -> ListRef<Ec2FleetFleetInstanceSetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleet_instance_set", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `launch_template_config` after provisioning.\n"]
    pub fn launch_template_config(&self) -> ListRef<Ec2FleetLaunchTemplateConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.launch_template_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `on_demand_options` after provisioning.\n"]
    pub fn on_demand_options(&self) -> ListRef<Ec2FleetOnDemandOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.on_demand_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spot_options` after provisioning.\n"]
    pub fn spot_options(&self) -> ListRef<Ec2FleetSpotOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spot_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_capacity_specification` after provisioning.\n"]
    pub fn target_capacity_specification(
        &self,
    ) -> ListRef<Ec2FleetTargetCapacitySpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_capacity_specification", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Ec2FleetTimeoutsElRef {
        Ec2FleetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct Ec2FleetFleetInstanceSetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform: Option<PrimField<String>>,
}
impl Ec2FleetFleetInstanceSetEl {
    #[doc = "Set the field `instance_ids`.\n"]
    pub fn set_instance_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.instance_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }
    #[doc = "Set the field `lifecycle`.\n"]
    pub fn set_lifecycle(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle = Some(v.into());
        self
    }
    #[doc = "Set the field `platform`.\n"]
    pub fn set_platform(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.platform = Some(v.into());
        self
    }
}
impl ToListMappable for Ec2FleetFleetInstanceSetEl {
    type O = BlockAssignable<Ec2FleetFleetInstanceSetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEc2FleetFleetInstanceSetEl {}
impl BuildEc2FleetFleetInstanceSetEl {
    pub fn build(self) -> Ec2FleetFleetInstanceSetEl {
        Ec2FleetFleetInstanceSetEl {
            instance_ids: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            lifecycle: core::default::Default::default(),
            platform: core::default::Default::default(),
        }
    }
}
pub struct Ec2FleetFleetInstanceSetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Ec2FleetFleetInstanceSetElRef {
    fn new(shared: StackShared, base: String) -> Ec2FleetFleetInstanceSetElRef {
        Ec2FleetFleetInstanceSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Ec2FleetFleetInstanceSetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_ids` after provisioning.\n"]
    pub fn instance_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.instance_ids", self.base))
    }
    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lifecycle` after provisioning.\n"]
    pub fn lifecycle(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle", self.base))
    }
    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform", self.base))
    }
}
#[derive(Serialize)]
pub struct Ec2FleetLaunchTemplateConfigElLaunchTemplateSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_name: Option<PrimField<String>>,
    version: PrimField<String>,
}
impl Ec2FleetLaunchTemplateConfigElLaunchTemplateSpecificationEl {
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
}
impl ToListMappable for Ec2FleetLaunchTemplateConfigElLaunchTemplateSpecificationEl {
    type O = BlockAssignable<Ec2FleetLaunchTemplateConfigElLaunchTemplateSpecificationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEc2FleetLaunchTemplateConfigElLaunchTemplateSpecificationEl {
    #[doc = ""]
    pub version: PrimField<String>,
}
impl BuildEc2FleetLaunchTemplateConfigElLaunchTemplateSpecificationEl {
    pub fn build(self) -> Ec2FleetLaunchTemplateConfigElLaunchTemplateSpecificationEl {
        Ec2FleetLaunchTemplateConfigElLaunchTemplateSpecificationEl {
            launch_template_id: core::default::Default::default(),
            launch_template_name: core::default::Default::default(),
            version: self.version,
        }
    }
}
pub struct Ec2FleetLaunchTemplateConfigElLaunchTemplateSpecificationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Ec2FleetLaunchTemplateConfigElLaunchTemplateSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2FleetLaunchTemplateConfigElLaunchTemplateSpecificationElRef {
        Ec2FleetLaunchTemplateConfigElLaunchTemplateSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Ec2FleetLaunchTemplateConfigElLaunchTemplateSpecificationElRef {
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
pub struct Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorCountEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorCountEl {
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
    for Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorCountEl
{
    type O = BlockAssignable<
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorCountEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEc2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorCountEl {
}
impl BuildEc2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorCountEl {
    pub fn build(
        self,
    ) -> Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorCountEl {
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorCountEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}
pub struct Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorCountElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorCountElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorCountElRef {
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorCountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorCountElRef {
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
pub struct Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl {
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
    for Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl
{
    type O = BlockAssignable<
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEc2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl
{}
impl
    BuildEc2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl
{
    pub fn build(
        self,
    ) -> Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl
    {
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}
pub struct Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibElRef
    {
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibElRef { shared : shared , base : base . to_string () , }
    }
}
impl Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
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
pub struct Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
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
    for Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl
{
    type O = BlockAssignable<
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEc2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl
{}
impl BuildEc2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
    pub fn build(
        self,
    ) -> Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl
    {
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}
pub struct Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef
    {
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef { shared : shared , base : base . to_string () , }
    }
}
impl Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
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
pub struct Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl {
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
    for Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl
{
    type O = BlockAssignable<
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEc2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl {
}
impl BuildEc2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl {
    pub fn build(
        self,
    ) -> Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl {
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}
pub struct Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryGibPerVcpuElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryGibPerVcpuElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryGibPerVcpuElRef {
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryGibPerVcpuElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryGibPerVcpuElRef {
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
pub struct Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryMibEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    min: PrimField<f64>,
}
impl Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryMibEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }
}
impl ToListMappable for Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryMibEl {
    type O =
        BlockAssignable<Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryMibEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEc2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryMibEl {
    #[doc = ""]
    pub min: PrimField<f64>,
}
impl BuildEc2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryMibEl {
    pub fn build(
        self,
    ) -> Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryMibEl {
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryMibEl {
            max: core::default::Default::default(),
            min: self.min,
        }
    }
}
pub struct Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryMibElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryMibElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryMibElRef {
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryMibElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryMibElRef {
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
pub struct Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl {
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
    for Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl
{
    type O = BlockAssignable<
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEc2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl
{}
impl BuildEc2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl {
    pub fn build(
        self,
    ) -> Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl {
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}
pub struct Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkBandwidthGbpsElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkBandwidthGbpsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkBandwidthGbpsElRef
    {
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkBandwidthGbpsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkBandwidthGbpsElRef {
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
pub struct Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkInterfaceCountEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkInterfaceCountEl {
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
    for Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkInterfaceCountEl
{
    type O = BlockAssignable<
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkInterfaceCountEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEc2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkInterfaceCountEl
{}
impl BuildEc2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkInterfaceCountEl {
    pub fn build(
        self,
    ) -> Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkInterfaceCountEl {
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkInterfaceCountEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}
pub struct Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkInterfaceCountElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkInterfaceCountElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkInterfaceCountElRef
    {
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkInterfaceCountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkInterfaceCountElRef {
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
pub struct Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElTotalLocalStorageGbEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElTotalLocalStorageGbEl {
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
    for Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElTotalLocalStorageGbEl
{
    type O = BlockAssignable<
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElTotalLocalStorageGbEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEc2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElTotalLocalStorageGbEl
{}
impl BuildEc2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElTotalLocalStorageGbEl {
    pub fn build(
        self,
    ) -> Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElTotalLocalStorageGbEl {
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElTotalLocalStorageGbEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}
pub struct Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElTotalLocalStorageGbElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElTotalLocalStorageGbElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElTotalLocalStorageGbElRef
    {
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElTotalLocalStorageGbElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElTotalLocalStorageGbElRef {
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
pub struct Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElVcpuCountEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    min: PrimField<f64>,
}
impl Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElVcpuCountEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }
}
impl ToListMappable for Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElVcpuCountEl {
    type O =
        BlockAssignable<Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElVcpuCountEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEc2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElVcpuCountEl {
    #[doc = ""]
    pub min: PrimField<f64>,
}
impl BuildEc2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElVcpuCountEl {
    pub fn build(
        self,
    ) -> Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElVcpuCountEl {
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElVcpuCountEl {
            max: core::default::Default::default(),
            min: self.min,
        }
    }
}
pub struct Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElVcpuCountElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElVcpuCountElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElVcpuCountElRef {
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElVcpuCountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElVcpuCountElRef {
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
struct Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElDynamic { accelerator_count : Option < DynamicBlock < Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorCountEl >> , accelerator_total_memory_mib : Option < DynamicBlock < Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl >> , baseline_ebs_bandwidth_mbps : Option < DynamicBlock < Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl >> , memory_gib_per_vcpu : Option < DynamicBlock < Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl >> , memory_mib : Option < DynamicBlock < Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryMibEl >> , network_bandwidth_gbps : Option < DynamicBlock < Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl >> , network_interface_count : Option < DynamicBlock < Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkInterfaceCountEl >> , total_local_storage_gb : Option < DynamicBlock < Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElTotalLocalStorageGbEl >> , vcpu_count : Option < DynamicBlock < Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElVcpuCountEl >> , }
#[derive(Serialize)]
pub struct Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsEl { # [serde (skip_serializing_if = "Option::is_none")] accelerator_manufacturers : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] accelerator_names : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] accelerator_types : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] allowed_instance_types : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] bare_metal : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] burstable_performance : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] cpu_manufacturers : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] excluded_instance_types : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] instance_generations : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] local_storage : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] local_storage_types : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] max_spot_price_as_percentage_of_optimal_on_demand_price : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] on_demand_max_price_percentage_over_lowest_price : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] require_hibernate_support : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] spot_max_price_percentage_over_lowest_price : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] accelerator_count : Option < Vec < Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorCountEl > > , # [serde (skip_serializing_if = "Option::is_none")] accelerator_total_memory_mib : Option < Vec < Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl > > , # [serde (skip_serializing_if = "Option::is_none")] baseline_ebs_bandwidth_mbps : Option < Vec < Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl > > , # [serde (skip_serializing_if = "Option::is_none")] memory_gib_per_vcpu : Option < Vec < Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl > > , # [serde (skip_serializing_if = "Option::is_none")] memory_mib : Option < Vec < Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryMibEl > > , # [serde (skip_serializing_if = "Option::is_none")] network_bandwidth_gbps : Option < Vec < Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl > > , # [serde (skip_serializing_if = "Option::is_none")] network_interface_count : Option < Vec < Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkInterfaceCountEl > > , # [serde (skip_serializing_if = "Option::is_none")] total_local_storage_gb : Option < Vec < Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElTotalLocalStorageGbEl > > , # [serde (skip_serializing_if = "Option::is_none")] vcpu_count : Option < Vec < Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElVcpuCountEl > > , dynamic : Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElDynamic , }
impl Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsEl {
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
        v: impl Into<
            BlockAssignable<
                Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorCountEl,
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
        v : impl Into < BlockAssignable < Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl >>,
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
        v : impl Into < BlockAssignable < Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl >>,
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
        v: impl Into<
            BlockAssignable<
                Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl,
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
        v: impl Into<
            BlockAssignable<
                Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryMibEl,
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
        v : impl Into < BlockAssignable < Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl >>,
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
        v : impl Into < BlockAssignable < Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkInterfaceCountEl >>,
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
        v: impl Into<
            BlockAssignable<
                Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElTotalLocalStorageGbEl,
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
        v: impl Into<
            BlockAssignable<
                Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElVcpuCountEl,
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
impl ToListMappable for Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsEl {
    type O = BlockAssignable<Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEc2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsEl {}
impl BuildEc2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsEl {
    pub fn build(self) -> Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsEl {
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsEl {
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
pub struct Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElRef {
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElRef {
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
    ) -> ListRef<Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorCountElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.accelerator_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `accelerator_total_memory_mib` after provisioning.\n"]    pub fn accelerator_total_memory_mib (& self) -> ListRef < Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.accelerator_total_memory_mib", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `baseline_ebs_bandwidth_mbps` after provisioning.\n"]
    pub fn baseline_ebs_bandwidth_mbps(
        &self,
    ) -> ListRef<
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.baseline_ebs_bandwidth_mbps", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_gib_per_vcpu` after provisioning.\n"]
    pub fn memory_gib_per_vcpu(
        &self,
    ) -> ListRef<Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryGibPerVcpuElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.memory_gib_per_vcpu", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_mib` after provisioning.\n"]
    pub fn memory_mib(
        &self,
    ) -> ListRef<Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElMemoryMibElRef> {
        ListRef::new(self.shared().clone(), format!("{}.memory_mib", self.base))
    }
    #[doc = "Get a reference to the value of field `network_bandwidth_gbps` after provisioning.\n"]
    pub fn network_bandwidth_gbps(
        &self,
    ) -> ListRef<
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkBandwidthGbpsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_bandwidth_gbps", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_interface_count` after provisioning.\n"]
    pub fn network_interface_count(
        &self,
    ) -> ListRef<
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElNetworkInterfaceCountElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interface_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_local_storage_gb` after provisioning.\n"]
    pub fn total_local_storage_gb(
        &self,
    ) -> ListRef<
        Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElTotalLocalStorageGbElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.total_local_storage_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vcpu_count` after provisioning.\n"]
    pub fn vcpu_count(
        &self,
    ) -> ListRef<Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElVcpuCountElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vcpu_count", self.base))
    }
}
#[derive(Serialize, Default)]
struct Ec2FleetLaunchTemplateConfigElOverrideElDynamic {
    instance_requirements:
        Option<DynamicBlock<Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsEl>>,
}
#[derive(Serialize)]
pub struct Ec2FleetLaunchTemplateConfigElOverrideEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_price: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_requirements:
        Option<Vec<Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsEl>>,
    dynamic: Ec2FleetLaunchTemplateConfigElOverrideElDynamic,
}
impl Ec2FleetLaunchTemplateConfigElOverrideEl {
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
    #[doc = "Set the field `max_price`.\n"]
    pub fn set_max_price(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_price = Some(v.into());
        self
    }
    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
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
        v: impl Into<BlockAssignable<Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsEl>>,
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
impl ToListMappable for Ec2FleetLaunchTemplateConfigElOverrideEl {
    type O = BlockAssignable<Ec2FleetLaunchTemplateConfigElOverrideEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEc2FleetLaunchTemplateConfigElOverrideEl {}
impl BuildEc2FleetLaunchTemplateConfigElOverrideEl {
    pub fn build(self) -> Ec2FleetLaunchTemplateConfigElOverrideEl {
        Ec2FleetLaunchTemplateConfigElOverrideEl {
            availability_zone: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            max_price: core::default::Default::default(),
            priority: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
            weighted_capacity: core::default::Default::default(),
            instance_requirements: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Ec2FleetLaunchTemplateConfigElOverrideElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Ec2FleetLaunchTemplateConfigElOverrideElRef {
    fn new(shared: StackShared, base: String) -> Ec2FleetLaunchTemplateConfigElOverrideElRef {
        Ec2FleetLaunchTemplateConfigElOverrideElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Ec2FleetLaunchTemplateConfigElOverrideElRef {
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
    #[doc = "Get a reference to the value of field `max_price` after provisioning.\n"]
    pub fn max_price(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_price", self.base))
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
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
    ) -> ListRef<Ec2FleetLaunchTemplateConfigElOverrideElInstanceRequirementsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_requirements", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Ec2FleetLaunchTemplateConfigElDynamic {
    launch_template_specification:
        Option<DynamicBlock<Ec2FleetLaunchTemplateConfigElLaunchTemplateSpecificationEl>>,
    override_: Option<DynamicBlock<Ec2FleetLaunchTemplateConfigElOverrideEl>>,
}
#[derive(Serialize)]
pub struct Ec2FleetLaunchTemplateConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_specification:
        Option<Vec<Ec2FleetLaunchTemplateConfigElLaunchTemplateSpecificationEl>>,
    #[serde(rename = "override", skip_serializing_if = "Option::is_none")]
    override_: Option<Vec<Ec2FleetLaunchTemplateConfigElOverrideEl>>,
    dynamic: Ec2FleetLaunchTemplateConfigElDynamic,
}
impl Ec2FleetLaunchTemplateConfigEl {
    #[doc = "Set the field `launch_template_specification`.\n"]
    pub fn set_launch_template_specification(
        mut self,
        v: impl Into<BlockAssignable<Ec2FleetLaunchTemplateConfigElLaunchTemplateSpecificationEl>>,
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
        v: impl Into<BlockAssignable<Ec2FleetLaunchTemplateConfigElOverrideEl>>,
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
impl ToListMappable for Ec2FleetLaunchTemplateConfigEl {
    type O = BlockAssignable<Ec2FleetLaunchTemplateConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEc2FleetLaunchTemplateConfigEl {}
impl BuildEc2FleetLaunchTemplateConfigEl {
    pub fn build(self) -> Ec2FleetLaunchTemplateConfigEl {
        Ec2FleetLaunchTemplateConfigEl {
            launch_template_specification: core::default::Default::default(),
            override_: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Ec2FleetLaunchTemplateConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Ec2FleetLaunchTemplateConfigElRef {
    fn new(shared: StackShared, base: String) -> Ec2FleetLaunchTemplateConfigElRef {
        Ec2FleetLaunchTemplateConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Ec2FleetLaunchTemplateConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `launch_template_specification` after provisioning.\n"]
    pub fn launch_template_specification(
        &self,
    ) -> ListRef<Ec2FleetLaunchTemplateConfigElLaunchTemplateSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.launch_template_specification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(&self) -> ListRef<Ec2FleetLaunchTemplateConfigElOverrideElRef> {
        ListRef::new(self.shared().clone(), format!("{}.override", self.base))
    }
}
#[derive(Serialize)]
pub struct Ec2FleetOnDemandOptionsElCapacityReservationOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    usage_strategy: Option<PrimField<String>>,
}
impl Ec2FleetOnDemandOptionsElCapacityReservationOptionsEl {
    #[doc = "Set the field `usage_strategy`.\n"]
    pub fn set_usage_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.usage_strategy = Some(v.into());
        self
    }
}
impl ToListMappable for Ec2FleetOnDemandOptionsElCapacityReservationOptionsEl {
    type O = BlockAssignable<Ec2FleetOnDemandOptionsElCapacityReservationOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEc2FleetOnDemandOptionsElCapacityReservationOptionsEl {}
impl BuildEc2FleetOnDemandOptionsElCapacityReservationOptionsEl {
    pub fn build(self) -> Ec2FleetOnDemandOptionsElCapacityReservationOptionsEl {
        Ec2FleetOnDemandOptionsElCapacityReservationOptionsEl {
            usage_strategy: core::default::Default::default(),
        }
    }
}
pub struct Ec2FleetOnDemandOptionsElCapacityReservationOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Ec2FleetOnDemandOptionsElCapacityReservationOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2FleetOnDemandOptionsElCapacityReservationOptionsElRef {
        Ec2FleetOnDemandOptionsElCapacityReservationOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Ec2FleetOnDemandOptionsElCapacityReservationOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `usage_strategy` after provisioning.\n"]
    pub fn usage_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.usage_strategy", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Ec2FleetOnDemandOptionsElDynamic {
    capacity_reservation_options:
        Option<DynamicBlock<Ec2FleetOnDemandOptionsElCapacityReservationOptionsEl>>,
}
#[derive(Serialize)]
pub struct Ec2FleetOnDemandOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allocation_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_total_price: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_target_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    single_availability_zone: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    single_instance_type: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_reservation_options:
        Option<Vec<Ec2FleetOnDemandOptionsElCapacityReservationOptionsEl>>,
    dynamic: Ec2FleetOnDemandOptionsElDynamic,
}
impl Ec2FleetOnDemandOptionsEl {
    #[doc = "Set the field `allocation_strategy`.\n"]
    pub fn set_allocation_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.allocation_strategy = Some(v.into());
        self
    }
    #[doc = "Set the field `max_total_price`.\n"]
    pub fn set_max_total_price(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_total_price = Some(v.into());
        self
    }
    #[doc = "Set the field `min_target_capacity`.\n"]
    pub fn set_min_target_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_target_capacity = Some(v.into());
        self
    }
    #[doc = "Set the field `single_availability_zone`.\n"]
    pub fn set_single_availability_zone(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.single_availability_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `single_instance_type`.\n"]
    pub fn set_single_instance_type(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.single_instance_type = Some(v.into());
        self
    }
    #[doc = "Set the field `capacity_reservation_options`.\n"]
    pub fn set_capacity_reservation_options(
        mut self,
        v: impl Into<BlockAssignable<Ec2FleetOnDemandOptionsElCapacityReservationOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.capacity_reservation_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.capacity_reservation_options = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Ec2FleetOnDemandOptionsEl {
    type O = BlockAssignable<Ec2FleetOnDemandOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEc2FleetOnDemandOptionsEl {}
impl BuildEc2FleetOnDemandOptionsEl {
    pub fn build(self) -> Ec2FleetOnDemandOptionsEl {
        Ec2FleetOnDemandOptionsEl {
            allocation_strategy: core::default::Default::default(),
            max_total_price: core::default::Default::default(),
            min_target_capacity: core::default::Default::default(),
            single_availability_zone: core::default::Default::default(),
            single_instance_type: core::default::Default::default(),
            capacity_reservation_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Ec2FleetOnDemandOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Ec2FleetOnDemandOptionsElRef {
    fn new(shared: StackShared, base: String) -> Ec2FleetOnDemandOptionsElRef {
        Ec2FleetOnDemandOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Ec2FleetOnDemandOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allocation_strategy` after provisioning.\n"]
    pub fn allocation_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocation_strategy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_total_price` after provisioning.\n"]
    pub fn max_total_price(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_total_price", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_target_capacity` after provisioning.\n"]
    pub fn min_target_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_target_capacity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `single_availability_zone` after provisioning.\n"]
    pub fn single_availability_zone(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.single_availability_zone", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `single_instance_type` after provisioning.\n"]
    pub fn single_instance_type(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.single_instance_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `capacity_reservation_options` after provisioning.\n"]
    pub fn capacity_reservation_options(
        &self,
    ) -> ListRef<Ec2FleetOnDemandOptionsElCapacityReservationOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_options", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Ec2FleetSpotOptionsElMaintenanceStrategiesElCapacityRebalanceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    replacement_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    termination_delay: Option<PrimField<f64>>,
}
impl Ec2FleetSpotOptionsElMaintenanceStrategiesElCapacityRebalanceEl {
    #[doc = "Set the field `replacement_strategy`.\n"]
    pub fn set_replacement_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.replacement_strategy = Some(v.into());
        self
    }
    #[doc = "Set the field `termination_delay`.\n"]
    pub fn set_termination_delay(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.termination_delay = Some(v.into());
        self
    }
}
impl ToListMappable for Ec2FleetSpotOptionsElMaintenanceStrategiesElCapacityRebalanceEl {
    type O = BlockAssignable<Ec2FleetSpotOptionsElMaintenanceStrategiesElCapacityRebalanceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEc2FleetSpotOptionsElMaintenanceStrategiesElCapacityRebalanceEl {}
impl BuildEc2FleetSpotOptionsElMaintenanceStrategiesElCapacityRebalanceEl {
    pub fn build(self) -> Ec2FleetSpotOptionsElMaintenanceStrategiesElCapacityRebalanceEl {
        Ec2FleetSpotOptionsElMaintenanceStrategiesElCapacityRebalanceEl {
            replacement_strategy: core::default::Default::default(),
            termination_delay: core::default::Default::default(),
        }
    }
}
pub struct Ec2FleetSpotOptionsElMaintenanceStrategiesElCapacityRebalanceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Ec2FleetSpotOptionsElMaintenanceStrategiesElCapacityRebalanceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2FleetSpotOptionsElMaintenanceStrategiesElCapacityRebalanceElRef {
        Ec2FleetSpotOptionsElMaintenanceStrategiesElCapacityRebalanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Ec2FleetSpotOptionsElMaintenanceStrategiesElCapacityRebalanceElRef {
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
    #[doc = "Get a reference to the value of field `termination_delay` after provisioning.\n"]
    pub fn termination_delay(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.termination_delay", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Ec2FleetSpotOptionsElMaintenanceStrategiesElDynamic {
    capacity_rebalance:
        Option<DynamicBlock<Ec2FleetSpotOptionsElMaintenanceStrategiesElCapacityRebalanceEl>>,
}
#[derive(Serialize)]
pub struct Ec2FleetSpotOptionsElMaintenanceStrategiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_rebalance:
        Option<Vec<Ec2FleetSpotOptionsElMaintenanceStrategiesElCapacityRebalanceEl>>,
    dynamic: Ec2FleetSpotOptionsElMaintenanceStrategiesElDynamic,
}
impl Ec2FleetSpotOptionsElMaintenanceStrategiesEl {
    #[doc = "Set the field `capacity_rebalance`.\n"]
    pub fn set_capacity_rebalance(
        mut self,
        v: impl Into<BlockAssignable<Ec2FleetSpotOptionsElMaintenanceStrategiesElCapacityRebalanceEl>>,
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
impl ToListMappable for Ec2FleetSpotOptionsElMaintenanceStrategiesEl {
    type O = BlockAssignable<Ec2FleetSpotOptionsElMaintenanceStrategiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEc2FleetSpotOptionsElMaintenanceStrategiesEl {}
impl BuildEc2FleetSpotOptionsElMaintenanceStrategiesEl {
    pub fn build(self) -> Ec2FleetSpotOptionsElMaintenanceStrategiesEl {
        Ec2FleetSpotOptionsElMaintenanceStrategiesEl {
            capacity_rebalance: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Ec2FleetSpotOptionsElMaintenanceStrategiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Ec2FleetSpotOptionsElMaintenanceStrategiesElRef {
    fn new(shared: StackShared, base: String) -> Ec2FleetSpotOptionsElMaintenanceStrategiesElRef {
        Ec2FleetSpotOptionsElMaintenanceStrategiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Ec2FleetSpotOptionsElMaintenanceStrategiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `capacity_rebalance` after provisioning.\n"]
    pub fn capacity_rebalance(
        &self,
    ) -> ListRef<Ec2FleetSpotOptionsElMaintenanceStrategiesElCapacityRebalanceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_rebalance", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Ec2FleetSpotOptionsElDynamic {
    maintenance_strategies: Option<DynamicBlock<Ec2FleetSpotOptionsElMaintenanceStrategiesEl>>,
}
#[derive(Serialize)]
pub struct Ec2FleetSpotOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allocation_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_interruption_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_pools_to_use_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_total_price: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_target_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    single_availability_zone: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    single_instance_type: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_strategies: Option<Vec<Ec2FleetSpotOptionsElMaintenanceStrategiesEl>>,
    dynamic: Ec2FleetSpotOptionsElDynamic,
}
impl Ec2FleetSpotOptionsEl {
    #[doc = "Set the field `allocation_strategy`.\n"]
    pub fn set_allocation_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.allocation_strategy = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_interruption_behavior`.\n"]
    pub fn set_instance_interruption_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_interruption_behavior = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_pools_to_use_count`.\n"]
    pub fn set_instance_pools_to_use_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.instance_pools_to_use_count = Some(v.into());
        self
    }
    #[doc = "Set the field `max_total_price`.\n"]
    pub fn set_max_total_price(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_total_price = Some(v.into());
        self
    }
    #[doc = "Set the field `min_target_capacity`.\n"]
    pub fn set_min_target_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_target_capacity = Some(v.into());
        self
    }
    #[doc = "Set the field `single_availability_zone`.\n"]
    pub fn set_single_availability_zone(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.single_availability_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `single_instance_type`.\n"]
    pub fn set_single_instance_type(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.single_instance_type = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_strategies`.\n"]
    pub fn set_maintenance_strategies(
        mut self,
        v: impl Into<BlockAssignable<Ec2FleetSpotOptionsElMaintenanceStrategiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.maintenance_strategies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.maintenance_strategies = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Ec2FleetSpotOptionsEl {
    type O = BlockAssignable<Ec2FleetSpotOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEc2FleetSpotOptionsEl {}
impl BuildEc2FleetSpotOptionsEl {
    pub fn build(self) -> Ec2FleetSpotOptionsEl {
        Ec2FleetSpotOptionsEl {
            allocation_strategy: core::default::Default::default(),
            instance_interruption_behavior: core::default::Default::default(),
            instance_pools_to_use_count: core::default::Default::default(),
            max_total_price: core::default::Default::default(),
            min_target_capacity: core::default::Default::default(),
            single_availability_zone: core::default::Default::default(),
            single_instance_type: core::default::Default::default(),
            maintenance_strategies: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Ec2FleetSpotOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Ec2FleetSpotOptionsElRef {
    fn new(shared: StackShared, base: String) -> Ec2FleetSpotOptionsElRef {
        Ec2FleetSpotOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Ec2FleetSpotOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allocation_strategy` after provisioning.\n"]
    pub fn allocation_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocation_strategy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance_interruption_behavior` after provisioning.\n"]
    pub fn instance_interruption_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_interruption_behavior", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance_pools_to_use_count` after provisioning.\n"]
    pub fn instance_pools_to_use_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_pools_to_use_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_total_price` after provisioning.\n"]
    pub fn max_total_price(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_total_price", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_target_capacity` after provisioning.\n"]
    pub fn min_target_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_target_capacity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `single_availability_zone` after provisioning.\n"]
    pub fn single_availability_zone(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.single_availability_zone", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `single_instance_type` after provisioning.\n"]
    pub fn single_instance_type(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.single_instance_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_strategies` after provisioning.\n"]
    pub fn maintenance_strategies(
        &self,
    ) -> ListRef<Ec2FleetSpotOptionsElMaintenanceStrategiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_strategies", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Ec2FleetTargetCapacitySpecificationEl {
    default_target_capacity_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_demand_target_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot_target_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_capacity_unit_type: Option<PrimField<String>>,
    total_target_capacity: PrimField<f64>,
}
impl Ec2FleetTargetCapacitySpecificationEl {
    #[doc = "Set the field `on_demand_target_capacity`.\n"]
    pub fn set_on_demand_target_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.on_demand_target_capacity = Some(v.into());
        self
    }
    #[doc = "Set the field `spot_target_capacity`.\n"]
    pub fn set_spot_target_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.spot_target_capacity = Some(v.into());
        self
    }
    #[doc = "Set the field `target_capacity_unit_type`.\n"]
    pub fn set_target_capacity_unit_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_capacity_unit_type = Some(v.into());
        self
    }
}
impl ToListMappable for Ec2FleetTargetCapacitySpecificationEl {
    type O = BlockAssignable<Ec2FleetTargetCapacitySpecificationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEc2FleetTargetCapacitySpecificationEl {
    #[doc = ""]
    pub default_target_capacity_type: PrimField<String>,
    #[doc = ""]
    pub total_target_capacity: PrimField<f64>,
}
impl BuildEc2FleetTargetCapacitySpecificationEl {
    pub fn build(self) -> Ec2FleetTargetCapacitySpecificationEl {
        Ec2FleetTargetCapacitySpecificationEl {
            default_target_capacity_type: self.default_target_capacity_type,
            on_demand_target_capacity: core::default::Default::default(),
            spot_target_capacity: core::default::Default::default(),
            target_capacity_unit_type: core::default::Default::default(),
            total_target_capacity: self.total_target_capacity,
        }
    }
}
pub struct Ec2FleetTargetCapacitySpecificationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Ec2FleetTargetCapacitySpecificationElRef {
    fn new(shared: StackShared, base: String) -> Ec2FleetTargetCapacitySpecificationElRef {
        Ec2FleetTargetCapacitySpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Ec2FleetTargetCapacitySpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_target_capacity_type` after provisioning.\n"]
    pub fn default_target_capacity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_target_capacity_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `on_demand_target_capacity` after provisioning.\n"]
    pub fn on_demand_target_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_demand_target_capacity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `spot_target_capacity` after provisioning.\n"]
    pub fn spot_target_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.spot_target_capacity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_capacity_unit_type` after provisioning.\n"]
    pub fn target_capacity_unit_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_capacity_unit_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_target_capacity` after provisioning.\n"]
    pub fn total_target_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_target_capacity", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Ec2FleetTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl Ec2FleetTimeoutsEl {
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
impl ToListMappable for Ec2FleetTimeoutsEl {
    type O = BlockAssignable<Ec2FleetTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEc2FleetTimeoutsEl {}
impl BuildEc2FleetTimeoutsEl {
    pub fn build(self) -> Ec2FleetTimeoutsEl {
        Ec2FleetTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct Ec2FleetTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Ec2FleetTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> Ec2FleetTimeoutsElRef {
        Ec2FleetTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Ec2FleetTimeoutsElRef {
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
struct Ec2FleetDynamic {
    fleet_instance_set: Option<DynamicBlock<Ec2FleetFleetInstanceSetEl>>,
    launch_template_config: Option<DynamicBlock<Ec2FleetLaunchTemplateConfigEl>>,
    on_demand_options: Option<DynamicBlock<Ec2FleetOnDemandOptionsEl>>,
    spot_options: Option<DynamicBlock<Ec2FleetSpotOptionsEl>>,
    target_capacity_specification: Option<DynamicBlock<Ec2FleetTargetCapacitySpecificationEl>>,
}
