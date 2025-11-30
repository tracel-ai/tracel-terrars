use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataAutoscalingGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataAutoscalingGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAutoscalingGroupData>,
}

#[derive(Clone)]
pub struct DataAutoscalingGroup(Rc<DataAutoscalingGroup_>);

impl DataAutoscalingGroup {
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

    #[doc = "Get a reference to the value of field `default_cooldown` after provisioning.\n"]
    pub fn default_cooldown(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_cooldown", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `instance_maintenance_policy` after provisioning.\n"]
    pub fn instance_maintenance_policy(
        &self,
    ) -> ListRef<DataAutoscalingGroupInstanceMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_maintenance_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `launch_configuration` after provisioning.\n"]
    pub fn launch_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.launch_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> ListRef<DataAutoscalingGroupLaunchTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.launch_template", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `min_size` after provisioning.\n"]
    pub fn min_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `mixed_instances_policy` after provisioning.\n"]
    pub fn mixed_instances_policy(&self) -> ListRef<DataAutoscalingGroupMixedInstancesPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mixed_instances_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `new_instances_protected_from_scale_in` after provisioning.\n"]
    pub fn new_instances_protected_from_scale_in(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.new_instances_protected_from_scale_in",
                self.extract_ref()
            ),
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

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `suspended_processes` after provisioning.\n"]
    pub fn suspended_processes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.suspended_processes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> SetRef<DataAutoscalingGroupTagElRef> {
        SetRef::new(self.shared().clone(), format!("{}.tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_group_arns` after provisioning.\n"]
    pub fn target_group_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.target_group_arns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `termination_policies` after provisioning.\n"]
    pub fn termination_policies(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.termination_policies", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `traffic_source` after provisioning.\n"]
    pub fn traffic_source(&self) -> SetRef<DataAutoscalingGroupTrafficSourceElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.traffic_source", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_zone_identifier` after provisioning.\n"]
    pub fn vpc_zone_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_zone_identifier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `warm_pool` after provisioning.\n"]
    pub fn warm_pool(&self) -> ListRef<DataAutoscalingGroupWarmPoolElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.warm_pool", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `warm_pool_size` after provisioning.\n"]
    pub fn warm_pool_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.warm_pool_size", self.extract_ref()),
        )
    }
}

impl Referable for DataAutoscalingGroup {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataAutoscalingGroup {}

impl ToListMappable for DataAutoscalingGroup {
    type O = ListRef<DataAutoscalingGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataAutoscalingGroup_ {
    fn extract_datasource_type(&self) -> String {
        "aws_autoscaling_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAutoscalingGroup {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildDataAutoscalingGroup {
    pub fn build(self, stack: &mut Stack) -> DataAutoscalingGroup {
        let out = DataAutoscalingGroup(Rc::new(DataAutoscalingGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAutoscalingGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAutoscalingGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAutoscalingGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataAutoscalingGroupRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
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

    #[doc = "Get a reference to the value of field `default_cooldown` after provisioning.\n"]
    pub fn default_cooldown(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_cooldown", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `instance_maintenance_policy` after provisioning.\n"]
    pub fn instance_maintenance_policy(
        &self,
    ) -> ListRef<DataAutoscalingGroupInstanceMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_maintenance_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `launch_configuration` after provisioning.\n"]
    pub fn launch_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.launch_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> ListRef<DataAutoscalingGroupLaunchTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.launch_template", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `min_size` after provisioning.\n"]
    pub fn min_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `mixed_instances_policy` after provisioning.\n"]
    pub fn mixed_instances_policy(&self) -> ListRef<DataAutoscalingGroupMixedInstancesPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mixed_instances_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `new_instances_protected_from_scale_in` after provisioning.\n"]
    pub fn new_instances_protected_from_scale_in(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.new_instances_protected_from_scale_in",
                self.extract_ref()
            ),
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

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `suspended_processes` after provisioning.\n"]
    pub fn suspended_processes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.suspended_processes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> SetRef<DataAutoscalingGroupTagElRef> {
        SetRef::new(self.shared().clone(), format!("{}.tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_group_arns` after provisioning.\n"]
    pub fn target_group_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.target_group_arns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `termination_policies` after provisioning.\n"]
    pub fn termination_policies(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.termination_policies", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `traffic_source` after provisioning.\n"]
    pub fn traffic_source(&self) -> SetRef<DataAutoscalingGroupTrafficSourceElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.traffic_source", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_zone_identifier` after provisioning.\n"]
    pub fn vpc_zone_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_zone_identifier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `warm_pool` after provisioning.\n"]
    pub fn warm_pool(&self) -> ListRef<DataAutoscalingGroupWarmPoolElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.warm_pool", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `warm_pool_size` after provisioning.\n"]
    pub fn warm_pool_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.warm_pool_size", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataAutoscalingGroupInstanceMaintenancePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_healthy_percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_healthy_percentage: Option<PrimField<f64>>,
}

impl DataAutoscalingGroupInstanceMaintenancePolicyEl {
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
}

impl ToListMappable for DataAutoscalingGroupInstanceMaintenancePolicyEl {
    type O = BlockAssignable<DataAutoscalingGroupInstanceMaintenancePolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroupInstanceMaintenancePolicyEl {}

impl BuildDataAutoscalingGroupInstanceMaintenancePolicyEl {
    pub fn build(self) -> DataAutoscalingGroupInstanceMaintenancePolicyEl {
        DataAutoscalingGroupInstanceMaintenancePolicyEl {
            max_healthy_percentage: core::default::Default::default(),
            min_healthy_percentage: core::default::Default::default(),
        }
    }
}

pub struct DataAutoscalingGroupInstanceMaintenancePolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAutoscalingGroupInstanceMaintenancePolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAutoscalingGroupInstanceMaintenancePolicyElRef {
        DataAutoscalingGroupInstanceMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroupInstanceMaintenancePolicyElRef {
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
pub struct DataAutoscalingGroupLaunchTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl DataAutoscalingGroupLaunchTemplateEl {
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

impl ToListMappable for DataAutoscalingGroupLaunchTemplateEl {
    type O = BlockAssignable<DataAutoscalingGroupLaunchTemplateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroupLaunchTemplateEl {}

impl BuildDataAutoscalingGroupLaunchTemplateEl {
    pub fn build(self) -> DataAutoscalingGroupLaunchTemplateEl {
        DataAutoscalingGroupLaunchTemplateEl {
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataAutoscalingGroupLaunchTemplateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAutoscalingGroupLaunchTemplateElRef {
    fn new(shared: StackShared, base: String) -> DataAutoscalingGroupLaunchTemplateElRef {
        DataAutoscalingGroupLaunchTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroupLaunchTemplateElRef {
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
pub struct DataAutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl {
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

impl DataAutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl {
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

impl ToListMappable for DataAutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl {
    type O = BlockAssignable<DataAutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl {}

impl BuildDataAutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl {
    pub fn build(self) -> DataAutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl {
        DataAutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl {
            on_demand_allocation_strategy: core::default::Default::default(),
            on_demand_base_capacity: core::default::Default::default(),
            on_demand_percentage_above_base_capacity: core::default::Default::default(),
            spot_allocation_strategy: core::default::Default::default(),
            spot_instance_pools: core::default::Default::default(),
            spot_max_price: core::default::Default::default(),
        }
    }
}

pub struct DataAutoscalingGroupMixedInstancesPolicyElInstancesDistributionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAutoscalingGroupMixedInstancesPolicyElInstancesDistributionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElInstancesDistributionElRef {
        DataAutoscalingGroupMixedInstancesPolicyElInstancesDistributionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroupMixedInstancesPolicyElInstancesDistributionElRef {
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
pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl {
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
    for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl
{
    type O = BlockAssignable<
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl
{}

impl BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl {
    pub fn build(
        self,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl
    {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl {
            launch_template_id: core::default::Default::default(),
            launch_template_name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref
    for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationElRef
    {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationElRef {
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
pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl {
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

impl ToListMappable for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl {
    type O =
        BlockAssignable<
            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl
{}

impl BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl {
    pub fn build(
        self,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountElRef {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountElRef {
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
pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl {
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

impl ToListMappable for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl {
    type O =
        BlockAssignable<
            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl
{}

impl BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl {
    pub fn build(
        self,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
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
pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
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

impl ToListMappable for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
    type O =
        BlockAssignable<
            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl
{}

impl BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
    pub fn build(
        self,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
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
pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl {
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

impl ToListMappable for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl {
    type O =
        BlockAssignable<
            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl
{}

impl BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl {
    pub fn build(
        self,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuElRef {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuElRef {
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
pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl {
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

impl ToListMappable for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl {
    type O =
        BlockAssignable<
            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl
{}

impl BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl {
    pub fn build(
        self,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibElRef {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibElRef {
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
pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl {
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

impl ToListMappable for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl {
    type O =
        BlockAssignable<
            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl
{}

impl BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl {
    pub fn build(
        self,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsElRef {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsElRef {
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
pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl {
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

impl ToListMappable for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl {
    type O =
        BlockAssignable<
            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl
{}

impl BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl {
    pub fn build(
        self,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountElRef {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountElRef {
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
pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl {
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

impl ToListMappable for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl {
    type O =
        BlockAssignable<
            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl
{}

impl BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl {
    pub fn build(
        self,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbElRef {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbElRef {
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
pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl {
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

impl ToListMappable for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl {
    type O =
        BlockAssignable<
            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl
{}

impl BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl {
    pub fn build(
        self,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountElRef {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountElRef {
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
pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_count: Option<
        ListField<
            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_manufacturers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_names: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_total_memory_mib: Option<
        ListField<
            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_instance_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bare_metal: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    baseline_ebs_bandwidth_mbps: Option<
        ListField<
            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl,
        >,
    >,
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
    memory_gib_per_vcpu: Option<
        ListField<
            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_mib: Option<
        ListField<
            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_bandwidth_gbps: Option<
        ListField<
            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface_count: Option<
        ListField<
            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_demand_max_price_percentage_over_lowest_price: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_hibernate_support: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot_max_price_percentage_over_lowest_price: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_local_storage_gb: Option<
        ListField<
            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcpu_count: Option<
        ListField<
            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl,
        >,
    >,
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl {
    #[doc = "Set the field `accelerator_count`.\n"]
    pub fn set_accelerator_count(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl,
                        >,
                    >,
    ) -> Self {
        self.accelerator_count = Some(v.into());
        self
    }

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

    #[doc = "Set the field `accelerator_total_memory_mib`.\n"]
    pub fn set_accelerator_total_memory_mib(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl,
                        >,
                    >,
    ) -> Self {
        self.accelerator_total_memory_mib = Some(v.into());
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

    #[doc = "Set the field `baseline_ebs_bandwidth_mbps`.\n"]
    pub fn set_baseline_ebs_bandwidth_mbps(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl,
                        >,
                    >,
    ) -> Self {
        self.baseline_ebs_bandwidth_mbps = Some(v.into());
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

    #[doc = "Set the field `memory_gib_per_vcpu`.\n"]
    pub fn set_memory_gib_per_vcpu(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl,
                        >,
                    >,
    ) -> Self {
        self.memory_gib_per_vcpu = Some(v.into());
        self
    }

    #[doc = "Set the field `memory_mib`.\n"]
    pub fn set_memory_mib(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl,
                        >,
                    >,
    ) -> Self {
        self.memory_mib = Some(v.into());
        self
    }

    #[doc = "Set the field `network_bandwidth_gbps`.\n"]
    pub fn set_network_bandwidth_gbps(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsEl,
                        >,
                    >,
    ) -> Self {
        self.network_bandwidth_gbps = Some(v.into());
        self
    }

    #[doc = "Set the field `network_interface_count`.\n"]
    pub fn set_network_interface_count(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl,
                        >,
                    >,
    ) -> Self {
        self.network_interface_count = Some(v.into());
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

    #[doc = "Set the field `total_local_storage_gb`.\n"]
    pub fn set_total_local_storage_gb(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl,
                        >,
                    >,
    ) -> Self {
        self.total_local_storage_gb = Some(v.into());
        self
    }

    #[doc = "Set the field `vcpu_count`.\n"]
    pub fn set_vcpu_count(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl,
                        >,
                    >,
    ) -> Self {
        self.vcpu_count = Some(v.into());
        self
    }
}

impl ToListMappable
    for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl
{
    type O = BlockAssignable<
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl
{}

impl
    BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl
{
    pub fn build(
        self,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl
    {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl {
            accelerator_count: core::default::Default::default(),
            accelerator_manufacturers: core::default::Default::default(),
            accelerator_names: core::default::Default::default(),
            accelerator_total_memory_mib: core::default::Default::default(),
            accelerator_types: core::default::Default::default(),
            allowed_instance_types: core::default::Default::default(),
            bare_metal: core::default::Default::default(),
            baseline_ebs_bandwidth_mbps: core::default::Default::default(),
            burstable_performance: core::default::Default::default(),
            cpu_manufacturers: core::default::Default::default(),
            excluded_instance_types: core::default::Default::default(),
            instance_generations: core::default::Default::default(),
            local_storage: core::default::Default::default(),
            local_storage_types: core::default::Default::default(),
            max_spot_price_as_percentage_of_optimal_on_demand_price:
                core::default::Default::default(),
            memory_gib_per_vcpu: core::default::Default::default(),
            memory_mib: core::default::Default::default(),
            network_bandwidth_gbps: core::default::Default::default(),
            network_interface_count: core::default::Default::default(),
            on_demand_max_price_percentage_over_lowest_price: core::default::Default::default(),
            require_hibernate_support: core::default::Default::default(),
            spot_max_price_percentage_over_lowest_price: core::default::Default::default(),
            total_local_storage_gb: core::default::Default::default(),
            vcpu_count: core::default::Default::default(),
        }
    }
}

pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElRef {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accelerator_count` after provisioning.\n"]
    pub fn accelerator_count(
        &self,
    ) -> ListRef<
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.accelerator_count", self.base),
        )
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

    #[doc = "Get a reference to the value of field `accelerator_total_memory_mib` after provisioning.\n"]
    pub fn accelerator_total_memory_mib(
        &self,
    ) -> ListRef<
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.accelerator_total_memory_mib", self.base),
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

    #[doc = "Get a reference to the value of field `baseline_ebs_bandwidth_mbps` after provisioning.\n"]
    pub fn baseline_ebs_bandwidth_mbps(
        &self,
    ) -> ListRef<
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.baseline_ebs_bandwidth_mbps", self.base),
        )
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

    #[doc = "Get a reference to the value of field `memory_gib_per_vcpu` after provisioning.\n"]
    pub fn memory_gib_per_vcpu(
        &self,
    ) -> ListRef<
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuElRef,
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
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibElRef,
    >{
        ListRef::new(self.shared().clone(), format!("{}.memory_mib", self.base))
    }

    #[doc = "Get a reference to the value of field `network_bandwidth_gbps` after provisioning.\n"]
    pub fn network_bandwidth_gbps(
        &self,
    ) -> ListRef<
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkBandwidthGbpsElRef,
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
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interface_count", self.base),
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

    #[doc = "Get a reference to the value of field `total_local_storage_gb` after provisioning.\n"]
    pub fn total_local_storage_gb(
        &self,
    ) -> ListRef<
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbElRef,
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
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountElRef,
    >{
        ListRef::new(self.shared().clone(), format!("{}.vcpu_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl {
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

impl ToListMappable for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl {
    type O =
        BlockAssignable<
            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl
{}

impl BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl {
    pub fn build(
        self,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl {
            launch_template_id: core::default::Default::default(),
            launch_template_name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationElRef {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `launch_template_id` after provisioning.\n"]
    pub fn launch_template_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.launch_template_id", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_template_name` after provisioning.\n"]
    pub fn launch_template_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.launch_template_name", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_requirements: Option<
        ListField<DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_specification: Option<
        ListField<DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_capacity: Option<PrimField<String>>,
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl {
    #[doc = "Set the field `instance_requirements`.\n"]
    pub fn set_instance_requirements(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl,
                        >,
                    >,
    ) -> Self {
        self.instance_requirements = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `launch_template_specification`.\n"]
    pub fn set_launch_template_specification(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl,
                        >,
                    >,
    ) -> Self {
        self.launch_template_specification = Some(v.into());
        self
    }

    #[doc = "Set the field `weighted_capacity`.\n"]
    pub fn set_weighted_capacity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.weighted_capacity = Some(v.into());
        self
    }
}

impl ToListMappable for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl {
    type O = BlockAssignable<DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl {}

impl BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl {
    pub fn build(self) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl {
            instance_requirements: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            launch_template_specification: core::default::Default::default(),
            weighted_capacity: core::default::Default::default(),
        }
    }
}

pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElRef {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_requirements` after provisioning.\n"]
    pub fn instance_requirements(
        &self,
    ) -> ListRef<DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_requirements", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `launch_template_specification` after provisioning.\n"]
    pub fn launch_template_specification(
        &self,
    ) -> ListRef<
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.launch_template_specification", self.base),
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
pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_specification: Option<
        ListField<
            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl,
        >,
    >,
    #[serde(rename = "override", skip_serializing_if = "Option::is_none")]
    override_:
        Option<ListField<DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl>>,
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl {
    #[doc = "Set the field `launch_template_specification`.\n"]
    pub fn set_launch_template_specification(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl,
                        >,
                    >,
    ) -> Self {
        self.launch_template_specification = Some(v.into());
        self
    }

    #[doc = "Set the field `override_`.\n"]
    pub fn set_override(
        mut self,
        v: impl Into<ListField<DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl>>,
    ) -> Self {
        self.override_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl {
    type O = BlockAssignable<DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl {}

impl BuildDataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl {
    pub fn build(self) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl {
            launch_template_specification: core::default::Default::default(),
            override_: core::default::Default::default(),
        }
    }
}

pub struct DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElRef {
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `launch_template_specification` after provisioning.\n"]
    pub fn launch_template_specification(
        &self,
    ) -> ListRef<
        DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.launch_template_specification", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(
        &self,
    ) -> ListRef<DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElRef> {
        ListRef::new(self.shared().clone(), format!("{}.override", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAutoscalingGroupMixedInstancesPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instances_distribution:
        Option<ListField<DataAutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template: Option<ListField<DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl>>,
}

impl DataAutoscalingGroupMixedInstancesPolicyEl {
    #[doc = "Set the field `instances_distribution`.\n"]
    pub fn set_instances_distribution(
        mut self,
        v: impl Into<ListField<DataAutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl>>,
    ) -> Self {
        self.instances_distribution = Some(v.into());
        self
    }

    #[doc = "Set the field `launch_template`.\n"]
    pub fn set_launch_template(
        mut self,
        v: impl Into<ListField<DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl>>,
    ) -> Self {
        self.launch_template = Some(v.into());
        self
    }
}

impl ToListMappable for DataAutoscalingGroupMixedInstancesPolicyEl {
    type O = BlockAssignable<DataAutoscalingGroupMixedInstancesPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroupMixedInstancesPolicyEl {}

impl BuildDataAutoscalingGroupMixedInstancesPolicyEl {
    pub fn build(self) -> DataAutoscalingGroupMixedInstancesPolicyEl {
        DataAutoscalingGroupMixedInstancesPolicyEl {
            instances_distribution: core::default::Default::default(),
            launch_template: core::default::Default::default(),
        }
    }
}

pub struct DataAutoscalingGroupMixedInstancesPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAutoscalingGroupMixedInstancesPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataAutoscalingGroupMixedInstancesPolicyElRef {
        DataAutoscalingGroupMixedInstancesPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroupMixedInstancesPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instances_distribution` after provisioning.\n"]
    pub fn instances_distribution(
        &self,
    ) -> ListRef<DataAutoscalingGroupMixedInstancesPolicyElInstancesDistributionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instances_distribution", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(
        &self,
    ) -> ListRef<DataAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.launch_template", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataAutoscalingGroupTagEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    propagate_at_launch: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataAutoscalingGroupTagEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `propagate_at_launch`.\n"]
    pub fn set_propagate_at_launch(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.propagate_at_launch = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAutoscalingGroupTagEl {
    type O = BlockAssignable<DataAutoscalingGroupTagEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroupTagEl {}

impl BuildDataAutoscalingGroupTagEl {
    pub fn build(self) -> DataAutoscalingGroupTagEl {
        DataAutoscalingGroupTagEl {
            key: core::default::Default::default(),
            propagate_at_launch: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAutoscalingGroupTagElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAutoscalingGroupTagElRef {
    fn new(shared: StackShared, base: String) -> DataAutoscalingGroupTagElRef {
        DataAutoscalingGroupTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroupTagElRef {
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
pub struct DataAutoscalingGroupTrafficSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identifier: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataAutoscalingGroupTrafficSourceEl {
    #[doc = "Set the field `identifier`.\n"]
    pub fn set_identifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identifier = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataAutoscalingGroupTrafficSourceEl {
    type O = BlockAssignable<DataAutoscalingGroupTrafficSourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroupTrafficSourceEl {}

impl BuildDataAutoscalingGroupTrafficSourceEl {
    pub fn build(self) -> DataAutoscalingGroupTrafficSourceEl {
        DataAutoscalingGroupTrafficSourceEl {
            identifier: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataAutoscalingGroupTrafficSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAutoscalingGroupTrafficSourceElRef {
    fn new(shared: StackShared, base: String) -> DataAutoscalingGroupTrafficSourceElRef {
        DataAutoscalingGroupTrafficSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroupTrafficSourceElRef {
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
pub struct DataAutoscalingGroupWarmPoolElInstanceReusePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    reuse_on_scale_in: Option<PrimField<bool>>,
}

impl DataAutoscalingGroupWarmPoolElInstanceReusePolicyEl {
    #[doc = "Set the field `reuse_on_scale_in`.\n"]
    pub fn set_reuse_on_scale_in(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.reuse_on_scale_in = Some(v.into());
        self
    }
}

impl ToListMappable for DataAutoscalingGroupWarmPoolElInstanceReusePolicyEl {
    type O = BlockAssignable<DataAutoscalingGroupWarmPoolElInstanceReusePolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroupWarmPoolElInstanceReusePolicyEl {}

impl BuildDataAutoscalingGroupWarmPoolElInstanceReusePolicyEl {
    pub fn build(self) -> DataAutoscalingGroupWarmPoolElInstanceReusePolicyEl {
        DataAutoscalingGroupWarmPoolElInstanceReusePolicyEl {
            reuse_on_scale_in: core::default::Default::default(),
        }
    }
}

pub struct DataAutoscalingGroupWarmPoolElInstanceReusePolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAutoscalingGroupWarmPoolElInstanceReusePolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAutoscalingGroupWarmPoolElInstanceReusePolicyElRef {
        DataAutoscalingGroupWarmPoolElInstanceReusePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroupWarmPoolElInstanceReusePolicyElRef {
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

#[derive(Serialize)]
pub struct DataAutoscalingGroupWarmPoolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_reuse_policy: Option<ListField<DataAutoscalingGroupWarmPoolElInstanceReusePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_group_prepared_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pool_state: Option<PrimField<String>>,
}

impl DataAutoscalingGroupWarmPoolEl {
    #[doc = "Set the field `instance_reuse_policy`.\n"]
    pub fn set_instance_reuse_policy(
        mut self,
        v: impl Into<ListField<DataAutoscalingGroupWarmPoolElInstanceReusePolicyEl>>,
    ) -> Self {
        self.instance_reuse_policy = Some(v.into());
        self
    }

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
}

impl ToListMappable for DataAutoscalingGroupWarmPoolEl {
    type O = BlockAssignable<DataAutoscalingGroupWarmPoolEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroupWarmPoolEl {}

impl BuildDataAutoscalingGroupWarmPoolEl {
    pub fn build(self) -> DataAutoscalingGroupWarmPoolEl {
        DataAutoscalingGroupWarmPoolEl {
            instance_reuse_policy: core::default::Default::default(),
            max_group_prepared_capacity: core::default::Default::default(),
            min_size: core::default::Default::default(),
            pool_state: core::default::Default::default(),
        }
    }
}

pub struct DataAutoscalingGroupWarmPoolElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAutoscalingGroupWarmPoolElRef {
    fn new(shared: StackShared, base: String) -> DataAutoscalingGroupWarmPoolElRef {
        DataAutoscalingGroupWarmPoolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroupWarmPoolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_reuse_policy` after provisioning.\n"]
    pub fn instance_reuse_policy(
        &self,
    ) -> ListRef<DataAutoscalingGroupWarmPoolElInstanceReusePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_reuse_policy", self.base),
        )
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
}
