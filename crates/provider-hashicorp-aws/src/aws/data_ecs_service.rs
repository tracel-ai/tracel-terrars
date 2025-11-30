use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataEcsServiceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cluster_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    service_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataEcsService_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEcsServiceData>,
}

#[derive(Clone)]
pub struct DataEcsService(Rc<DataEcsService_>);

impl DataEcsService {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_rebalancing` after provisioning.\n"]
    pub fn availability_zone_rebalancing(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_rebalancing", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capacity_provider_strategy` after provisioning.\n"]
    pub fn capacity_provider_strategy(
        &self,
    ) -> SetRef<DataEcsServiceCapacityProviderStrategyElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.capacity_provider_strategy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_arn` after provisioning.\n"]
    pub fn cluster_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_by", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deployment_configuration` after provisioning.\n"]
    pub fn deployment_configuration(&self) -> ListRef<DataEcsServiceDeploymentConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deployment_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deployment_controller` after provisioning.\n"]
    pub fn deployment_controller(&self) -> ListRef<DataEcsServiceDeploymentControllerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deployment_controller", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deployments` after provisioning.\n"]
    pub fn deployments(&self) -> ListRef<DataEcsServiceDeploymentsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deployments", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `desired_count` after provisioning.\n"]
    pub fn desired_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_count", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_ecs_managed_tags` after provisioning.\n"]
    pub fn enable_ecs_managed_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_ecs_managed_tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_execute_command` after provisioning.\n"]
    pub fn enable_execute_command(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_execute_command", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `events` after provisioning.\n"]
    pub fn events(&self) -> ListRef<DataEcsServiceEventsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.events", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `health_check_grace_period_seconds` after provisioning.\n"]
    pub fn health_check_grace_period_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_check_grace_period_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `iam_role` after provisioning.\n"]
    pub fn iam_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_role", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_type` after provisioning.\n"]
    pub fn launch_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.launch_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> SetRef<DataEcsServiceLoadBalancerElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.load_balancer", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> ListRef<DataEcsServiceNetworkConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ordered_placement_strategy` after provisioning.\n"]
    pub fn ordered_placement_strategy(
        &self,
    ) -> ListRef<DataEcsServiceOrderedPlacementStrategyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ordered_placement_strategy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `pending_count` after provisioning.\n"]
    pub fn pending_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pending_count", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `placement_constraints` after provisioning.\n"]
    pub fn placement_constraints(&self) -> SetRef<DataEcsServicePlacementConstraintsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.placement_constraints", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `platform_family` after provisioning.\n"]
    pub fn platform_family(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.platform_family", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.platform_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.propagate_tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `running_count` after provisioning.\n"]
    pub fn running_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.running_count", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scheduling_strategy` after provisioning.\n"]
    pub fn scheduling_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scheduling_strategy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_registries` after provisioning.\n"]
    pub fn service_registries(&self) -> ListRef<DataEcsServiceServiceRegistriesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_registries", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `task_definition` after provisioning.\n"]
    pub fn task_definition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.task_definition", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `task_sets` after provisioning.\n"]
    pub fn task_sets(&self) -> ListRef<DataEcsServiceTaskSetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.task_sets", self.extract_ref()),
        )
    }
}

impl Referable for DataEcsService {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataEcsService {}

impl ToListMappable for DataEcsService {
    type O = ListRef<DataEcsServiceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEcsService_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ecs_service".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEcsService {
    pub tf_id: String,
    #[doc = ""]
    pub cluster_arn: PrimField<String>,
    #[doc = ""]
    pub service_name: PrimField<String>,
}

impl BuildDataEcsService {
    pub fn build(self, stack: &mut Stack) -> DataEcsService {
        let out = DataEcsService(Rc::new(DataEcsService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEcsServiceData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                cluster_arn: self.cluster_arn,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                service_name: self.service_name,
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEcsServiceRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsServiceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataEcsServiceRef {
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

    #[doc = "Get a reference to the value of field `availability_zone_rebalancing` after provisioning.\n"]
    pub fn availability_zone_rebalancing(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_rebalancing", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capacity_provider_strategy` after provisioning.\n"]
    pub fn capacity_provider_strategy(
        &self,
    ) -> SetRef<DataEcsServiceCapacityProviderStrategyElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.capacity_provider_strategy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_arn` after provisioning.\n"]
    pub fn cluster_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_by", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deployment_configuration` after provisioning.\n"]
    pub fn deployment_configuration(&self) -> ListRef<DataEcsServiceDeploymentConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deployment_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deployment_controller` after provisioning.\n"]
    pub fn deployment_controller(&self) -> ListRef<DataEcsServiceDeploymentControllerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deployment_controller", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deployments` after provisioning.\n"]
    pub fn deployments(&self) -> ListRef<DataEcsServiceDeploymentsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deployments", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `desired_count` after provisioning.\n"]
    pub fn desired_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_count", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_ecs_managed_tags` after provisioning.\n"]
    pub fn enable_ecs_managed_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_ecs_managed_tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_execute_command` after provisioning.\n"]
    pub fn enable_execute_command(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_execute_command", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `events` after provisioning.\n"]
    pub fn events(&self) -> ListRef<DataEcsServiceEventsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.events", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `health_check_grace_period_seconds` after provisioning.\n"]
    pub fn health_check_grace_period_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_check_grace_period_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `iam_role` after provisioning.\n"]
    pub fn iam_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_role", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_type` after provisioning.\n"]
    pub fn launch_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.launch_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> SetRef<DataEcsServiceLoadBalancerElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.load_balancer", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> ListRef<DataEcsServiceNetworkConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ordered_placement_strategy` after provisioning.\n"]
    pub fn ordered_placement_strategy(
        &self,
    ) -> ListRef<DataEcsServiceOrderedPlacementStrategyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ordered_placement_strategy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `pending_count` after provisioning.\n"]
    pub fn pending_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pending_count", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `placement_constraints` after provisioning.\n"]
    pub fn placement_constraints(&self) -> SetRef<DataEcsServicePlacementConstraintsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.placement_constraints", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `platform_family` after provisioning.\n"]
    pub fn platform_family(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.platform_family", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.platform_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.propagate_tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `running_count` after provisioning.\n"]
    pub fn running_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.running_count", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scheduling_strategy` after provisioning.\n"]
    pub fn scheduling_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scheduling_strategy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_registries` after provisioning.\n"]
    pub fn service_registries(&self) -> ListRef<DataEcsServiceServiceRegistriesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_registries", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `task_definition` after provisioning.\n"]
    pub fn task_definition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.task_definition", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `task_sets` after provisioning.\n"]
    pub fn task_sets(&self) -> ListRef<DataEcsServiceTaskSetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.task_sets", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataEcsServiceCapacityProviderStrategyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    base: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_provider: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}

impl DataEcsServiceCapacityProviderStrategyEl {
    #[doc = "Set the field `base`.\n"]
    pub fn set_base(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.base = Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_provider`.\n"]
    pub fn set_capacity_provider(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.capacity_provider = Some(v.into());
        self
    }

    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcsServiceCapacityProviderStrategyEl {
    type O = BlockAssignable<DataEcsServiceCapacityProviderStrategyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsServiceCapacityProviderStrategyEl {}

impl BuildDataEcsServiceCapacityProviderStrategyEl {
    pub fn build(self) -> DataEcsServiceCapacityProviderStrategyEl {
        DataEcsServiceCapacityProviderStrategyEl {
            base: core::default::Default::default(),
            capacity_provider: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

pub struct DataEcsServiceCapacityProviderStrategyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsServiceCapacityProviderStrategyElRef {
    fn new(shared: StackShared, base: String) -> DataEcsServiceCapacityProviderStrategyElRef {
        DataEcsServiceCapacityProviderStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsServiceCapacityProviderStrategyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base` after provisioning.\n"]
    pub fn base(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.base", self.base))
    }

    #[doc = "Get a reference to the value of field `capacity_provider` after provisioning.\n"]
    pub fn capacity_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_provider", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsServiceDeploymentConfigurationElAlarmsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    alarm_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rollback: Option<PrimField<bool>>,
}

impl DataEcsServiceDeploymentConfigurationElAlarmsEl {
    #[doc = "Set the field `alarm_names`.\n"]
    pub fn set_alarm_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.alarm_names = Some(v.into());
        self
    }

    #[doc = "Set the field `enable`.\n"]
    pub fn set_enable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable = Some(v.into());
        self
    }

    #[doc = "Set the field `rollback`.\n"]
    pub fn set_rollback(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.rollback = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcsServiceDeploymentConfigurationElAlarmsEl {
    type O = BlockAssignable<DataEcsServiceDeploymentConfigurationElAlarmsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsServiceDeploymentConfigurationElAlarmsEl {}

impl BuildDataEcsServiceDeploymentConfigurationElAlarmsEl {
    pub fn build(self) -> DataEcsServiceDeploymentConfigurationElAlarmsEl {
        DataEcsServiceDeploymentConfigurationElAlarmsEl {
            alarm_names: core::default::Default::default(),
            enable: core::default::Default::default(),
            rollback: core::default::Default::default(),
        }
    }
}

pub struct DataEcsServiceDeploymentConfigurationElAlarmsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsServiceDeploymentConfigurationElAlarmsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEcsServiceDeploymentConfigurationElAlarmsElRef {
        DataEcsServiceDeploymentConfigurationElAlarmsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsServiceDeploymentConfigurationElAlarmsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alarm_names` after provisioning.\n"]
    pub fn alarm_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.alarm_names", self.base))
    }

    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }

    #[doc = "Get a reference to the value of field `rollback` after provisioning.\n"]
    pub fn rollback(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.rollback", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsServiceDeploymentConfigurationElCanaryConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    canary_bake_time_in_minutes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    canary_percent: Option<PrimField<f64>>,
}

impl DataEcsServiceDeploymentConfigurationElCanaryConfigurationEl {
    #[doc = "Set the field `canary_bake_time_in_minutes`.\n"]
    pub fn set_canary_bake_time_in_minutes(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.canary_bake_time_in_minutes = Some(v.into());
        self
    }

    #[doc = "Set the field `canary_percent`.\n"]
    pub fn set_canary_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.canary_percent = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcsServiceDeploymentConfigurationElCanaryConfigurationEl {
    type O = BlockAssignable<DataEcsServiceDeploymentConfigurationElCanaryConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsServiceDeploymentConfigurationElCanaryConfigurationEl {}

impl BuildDataEcsServiceDeploymentConfigurationElCanaryConfigurationEl {
    pub fn build(self) -> DataEcsServiceDeploymentConfigurationElCanaryConfigurationEl {
        DataEcsServiceDeploymentConfigurationElCanaryConfigurationEl {
            canary_bake_time_in_minutes: core::default::Default::default(),
            canary_percent: core::default::Default::default(),
        }
    }
}

pub struct DataEcsServiceDeploymentConfigurationElCanaryConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsServiceDeploymentConfigurationElCanaryConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEcsServiceDeploymentConfigurationElCanaryConfigurationElRef {
        DataEcsServiceDeploymentConfigurationElCanaryConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsServiceDeploymentConfigurationElCanaryConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `canary_bake_time_in_minutes` after provisioning.\n"]
    pub fn canary_bake_time_in_minutes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.canary_bake_time_in_minutes", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `canary_percent` after provisioning.\n"]
    pub fn canary_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.canary_percent", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataEcsServiceDeploymentConfigurationElDeploymentCircuitBreakerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rollback: Option<PrimField<bool>>,
}

impl DataEcsServiceDeploymentConfigurationElDeploymentCircuitBreakerEl {
    #[doc = "Set the field `enable`.\n"]
    pub fn set_enable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable = Some(v.into());
        self
    }

    #[doc = "Set the field `rollback`.\n"]
    pub fn set_rollback(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.rollback = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcsServiceDeploymentConfigurationElDeploymentCircuitBreakerEl {
    type O = BlockAssignable<DataEcsServiceDeploymentConfigurationElDeploymentCircuitBreakerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsServiceDeploymentConfigurationElDeploymentCircuitBreakerEl {}

impl BuildDataEcsServiceDeploymentConfigurationElDeploymentCircuitBreakerEl {
    pub fn build(self) -> DataEcsServiceDeploymentConfigurationElDeploymentCircuitBreakerEl {
        DataEcsServiceDeploymentConfigurationElDeploymentCircuitBreakerEl {
            enable: core::default::Default::default(),
            rollback: core::default::Default::default(),
        }
    }
}

pub struct DataEcsServiceDeploymentConfigurationElDeploymentCircuitBreakerElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsServiceDeploymentConfigurationElDeploymentCircuitBreakerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEcsServiceDeploymentConfigurationElDeploymentCircuitBreakerElRef {
        DataEcsServiceDeploymentConfigurationElDeploymentCircuitBreakerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsServiceDeploymentConfigurationElDeploymentCircuitBreakerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }

    #[doc = "Get a reference to the value of field `rollback` after provisioning.\n"]
    pub fn rollback(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.rollback", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsServiceDeploymentConfigurationElLifecycleHookEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hook_details: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hook_target_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_stages: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
}

impl DataEcsServiceDeploymentConfigurationElLifecycleHookEl {
    #[doc = "Set the field `hook_details`.\n"]
    pub fn set_hook_details(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hook_details = Some(v.into());
        self
    }

    #[doc = "Set the field `hook_target_arn`.\n"]
    pub fn set_hook_target_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hook_target_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_stages`.\n"]
    pub fn set_lifecycle_stages(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.lifecycle_stages = Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_arn = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcsServiceDeploymentConfigurationElLifecycleHookEl {
    type O = BlockAssignable<DataEcsServiceDeploymentConfigurationElLifecycleHookEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsServiceDeploymentConfigurationElLifecycleHookEl {}

impl BuildDataEcsServiceDeploymentConfigurationElLifecycleHookEl {
    pub fn build(self) -> DataEcsServiceDeploymentConfigurationElLifecycleHookEl {
        DataEcsServiceDeploymentConfigurationElLifecycleHookEl {
            hook_details: core::default::Default::default(),
            hook_target_arn: core::default::Default::default(),
            lifecycle_stages: core::default::Default::default(),
            role_arn: core::default::Default::default(),
        }
    }
}

pub struct DataEcsServiceDeploymentConfigurationElLifecycleHookElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsServiceDeploymentConfigurationElLifecycleHookElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEcsServiceDeploymentConfigurationElLifecycleHookElRef {
        DataEcsServiceDeploymentConfigurationElLifecycleHookElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsServiceDeploymentConfigurationElLifecycleHookElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hook_details` after provisioning.\n"]
    pub fn hook_details(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hook_details", self.base))
    }

    #[doc = "Get a reference to the value of field `hook_target_arn` after provisioning.\n"]
    pub fn hook_target_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hook_target_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `lifecycle_stages` after provisioning.\n"]
    pub fn lifecycle_stages(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lifecycle_stages", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsServiceDeploymentConfigurationElLinearConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    step_bake_time_in_minutes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    step_percent: Option<PrimField<f64>>,
}

impl DataEcsServiceDeploymentConfigurationElLinearConfigurationEl {
    #[doc = "Set the field `step_bake_time_in_minutes`.\n"]
    pub fn set_step_bake_time_in_minutes(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.step_bake_time_in_minutes = Some(v.into());
        self
    }

    #[doc = "Set the field `step_percent`.\n"]
    pub fn set_step_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.step_percent = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcsServiceDeploymentConfigurationElLinearConfigurationEl {
    type O = BlockAssignable<DataEcsServiceDeploymentConfigurationElLinearConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsServiceDeploymentConfigurationElLinearConfigurationEl {}

impl BuildDataEcsServiceDeploymentConfigurationElLinearConfigurationEl {
    pub fn build(self) -> DataEcsServiceDeploymentConfigurationElLinearConfigurationEl {
        DataEcsServiceDeploymentConfigurationElLinearConfigurationEl {
            step_bake_time_in_minutes: core::default::Default::default(),
            step_percent: core::default::Default::default(),
        }
    }
}

pub struct DataEcsServiceDeploymentConfigurationElLinearConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsServiceDeploymentConfigurationElLinearConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEcsServiceDeploymentConfigurationElLinearConfigurationElRef {
        DataEcsServiceDeploymentConfigurationElLinearConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsServiceDeploymentConfigurationElLinearConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `step_bake_time_in_minutes` after provisioning.\n"]
    pub fn step_bake_time_in_minutes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.step_bake_time_in_minutes", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `step_percent` after provisioning.\n"]
    pub fn step_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.step_percent", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsServiceDeploymentConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    alarms: Option<ListField<DataEcsServiceDeploymentConfigurationElAlarmsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bake_time_in_minutes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    canary_configuration:
        Option<ListField<DataEcsServiceDeploymentConfigurationElCanaryConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_circuit_breaker:
        Option<ListField<DataEcsServiceDeploymentConfigurationElDeploymentCircuitBreakerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_hook: Option<SetField<DataEcsServiceDeploymentConfigurationElLifecycleHookEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    linear_configuration:
        Option<ListField<DataEcsServiceDeploymentConfigurationElLinearConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_healthy_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strategy: Option<PrimField<String>>,
}

impl DataEcsServiceDeploymentConfigurationEl {
    #[doc = "Set the field `alarms`.\n"]
    pub fn set_alarms(
        mut self,
        v: impl Into<ListField<DataEcsServiceDeploymentConfigurationElAlarmsEl>>,
    ) -> Self {
        self.alarms = Some(v.into());
        self
    }

    #[doc = "Set the field `bake_time_in_minutes`.\n"]
    pub fn set_bake_time_in_minutes(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bake_time_in_minutes = Some(v.into());
        self
    }

    #[doc = "Set the field `canary_configuration`.\n"]
    pub fn set_canary_configuration(
        mut self,
        v: impl Into<ListField<DataEcsServiceDeploymentConfigurationElCanaryConfigurationEl>>,
    ) -> Self {
        self.canary_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_circuit_breaker`.\n"]
    pub fn set_deployment_circuit_breaker(
        mut self,
        v: impl Into<ListField<DataEcsServiceDeploymentConfigurationElDeploymentCircuitBreakerEl>>,
    ) -> Self {
        self.deployment_circuit_breaker = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_hook`.\n"]
    pub fn set_lifecycle_hook(
        mut self,
        v: impl Into<SetField<DataEcsServiceDeploymentConfigurationElLifecycleHookEl>>,
    ) -> Self {
        self.lifecycle_hook = Some(v.into());
        self
    }

    #[doc = "Set the field `linear_configuration`.\n"]
    pub fn set_linear_configuration(
        mut self,
        v: impl Into<ListField<DataEcsServiceDeploymentConfigurationElLinearConfigurationEl>>,
    ) -> Self {
        self.linear_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_percent`.\n"]
    pub fn set_maximum_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_percent = Some(v.into());
        self
    }

    #[doc = "Set the field `minimum_healthy_percent`.\n"]
    pub fn set_minimum_healthy_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minimum_healthy_percent = Some(v.into());
        self
    }

    #[doc = "Set the field `strategy`.\n"]
    pub fn set_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.strategy = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcsServiceDeploymentConfigurationEl {
    type O = BlockAssignable<DataEcsServiceDeploymentConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsServiceDeploymentConfigurationEl {}

impl BuildDataEcsServiceDeploymentConfigurationEl {
    pub fn build(self) -> DataEcsServiceDeploymentConfigurationEl {
        DataEcsServiceDeploymentConfigurationEl {
            alarms: core::default::Default::default(),
            bake_time_in_minutes: core::default::Default::default(),
            canary_configuration: core::default::Default::default(),
            deployment_circuit_breaker: core::default::Default::default(),
            lifecycle_hook: core::default::Default::default(),
            linear_configuration: core::default::Default::default(),
            maximum_percent: core::default::Default::default(),
            minimum_healthy_percent: core::default::Default::default(),
            strategy: core::default::Default::default(),
        }
    }
}

pub struct DataEcsServiceDeploymentConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsServiceDeploymentConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataEcsServiceDeploymentConfigurationElRef {
        DataEcsServiceDeploymentConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsServiceDeploymentConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alarms` after provisioning.\n"]
    pub fn alarms(&self) -> ListRef<DataEcsServiceDeploymentConfigurationElAlarmsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.alarms", self.base))
    }

    #[doc = "Get a reference to the value of field `bake_time_in_minutes` after provisioning.\n"]
    pub fn bake_time_in_minutes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bake_time_in_minutes", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `canary_configuration` after provisioning.\n"]
    pub fn canary_configuration(
        &self,
    ) -> ListRef<DataEcsServiceDeploymentConfigurationElCanaryConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.canary_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `deployment_circuit_breaker` after provisioning.\n"]
    pub fn deployment_circuit_breaker(
        &self,
    ) -> ListRef<DataEcsServiceDeploymentConfigurationElDeploymentCircuitBreakerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deployment_circuit_breaker", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `lifecycle_hook` after provisioning.\n"]
    pub fn lifecycle_hook(
        &self,
    ) -> SetRef<DataEcsServiceDeploymentConfigurationElLifecycleHookElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.lifecycle_hook", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `linear_configuration` after provisioning.\n"]
    pub fn linear_configuration(
        &self,
    ) -> ListRef<DataEcsServiceDeploymentConfigurationElLinearConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.linear_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `maximum_percent` after provisioning.\n"]
    pub fn maximum_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_percent", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `minimum_healthy_percent` after provisioning.\n"]
    pub fn minimum_healthy_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minimum_healthy_percent", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `strategy` after provisioning.\n"]
    pub fn strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsServiceDeploymentControllerEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataEcsServiceDeploymentControllerEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcsServiceDeploymentControllerEl {
    type O = BlockAssignable<DataEcsServiceDeploymentControllerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsServiceDeploymentControllerEl {}

impl BuildDataEcsServiceDeploymentControllerEl {
    pub fn build(self) -> DataEcsServiceDeploymentControllerEl {
        DataEcsServiceDeploymentControllerEl {
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataEcsServiceDeploymentControllerElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsServiceDeploymentControllerElRef {
    fn new(shared: StackShared, base: String) -> DataEcsServiceDeploymentControllerElRef {
        DataEcsServiceDeploymentControllerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsServiceDeploymentControllerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsServiceDeploymentsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    running_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_definition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<PrimField<String>>,
}

impl DataEcsServiceDeploymentsEl {
    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_at = Some(v.into());
        self
    }

    #[doc = "Set the field `desired_count`.\n"]
    pub fn set_desired_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.desired_count = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `pending_count`.\n"]
    pub fn set_pending_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.pending_count = Some(v.into());
        self
    }

    #[doc = "Set the field `running_count`.\n"]
    pub fn set_running_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.running_count = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }

    #[doc = "Set the field `task_definition`.\n"]
    pub fn set_task_definition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.task_definition = Some(v.into());
        self
    }

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.updated_at = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcsServiceDeploymentsEl {
    type O = BlockAssignable<DataEcsServiceDeploymentsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsServiceDeploymentsEl {}

impl BuildDataEcsServiceDeploymentsEl {
    pub fn build(self) -> DataEcsServiceDeploymentsEl {
        DataEcsServiceDeploymentsEl {
            created_at: core::default::Default::default(),
            desired_count: core::default::Default::default(),
            id: core::default::Default::default(),
            pending_count: core::default::Default::default(),
            running_count: core::default::Default::default(),
            status: core::default::Default::default(),
            task_definition: core::default::Default::default(),
            updated_at: core::default::Default::default(),
        }
    }
}

pub struct DataEcsServiceDeploymentsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsServiceDeploymentsElRef {
    fn new(shared: StackShared, base: String) -> DataEcsServiceDeploymentsElRef {
        DataEcsServiceDeploymentsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsServiceDeploymentsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `desired_count` after provisioning.\n"]
    pub fn desired_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `pending_count` after provisioning.\n"]
    pub fn pending_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pending_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `running_count` after provisioning.\n"]
    pub fn running_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.running_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `task_definition` after provisioning.\n"]
    pub fn task_definition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.task_definition", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsServiceEventsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}

impl DataEcsServiceEventsEl {
    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_at = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcsServiceEventsEl {
    type O = BlockAssignable<DataEcsServiceEventsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsServiceEventsEl {}

impl BuildDataEcsServiceEventsEl {
    pub fn build(self) -> DataEcsServiceEventsEl {
        DataEcsServiceEventsEl {
            created_at: core::default::Default::default(),
            id: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

pub struct DataEcsServiceEventsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsServiceEventsElRef {
    fn new(shared: StackShared, base: String) -> DataEcsServiceEventsElRef {
        DataEcsServiceEventsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsServiceEventsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsServiceLoadBalancerElAdvancedConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    alternate_target_group_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    production_listener_rule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    test_listener_rule: Option<PrimField<String>>,
}

impl DataEcsServiceLoadBalancerElAdvancedConfigurationEl {
    #[doc = "Set the field `alternate_target_group_arn`.\n"]
    pub fn set_alternate_target_group_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.alternate_target_group_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `production_listener_rule`.\n"]
    pub fn set_production_listener_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.production_listener_rule = Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `test_listener_rule`.\n"]
    pub fn set_test_listener_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.test_listener_rule = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcsServiceLoadBalancerElAdvancedConfigurationEl {
    type O = BlockAssignable<DataEcsServiceLoadBalancerElAdvancedConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsServiceLoadBalancerElAdvancedConfigurationEl {}

impl BuildDataEcsServiceLoadBalancerElAdvancedConfigurationEl {
    pub fn build(self) -> DataEcsServiceLoadBalancerElAdvancedConfigurationEl {
        DataEcsServiceLoadBalancerElAdvancedConfigurationEl {
            alternate_target_group_arn: core::default::Default::default(),
            production_listener_rule: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            test_listener_rule: core::default::Default::default(),
        }
    }
}

pub struct DataEcsServiceLoadBalancerElAdvancedConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsServiceLoadBalancerElAdvancedConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEcsServiceLoadBalancerElAdvancedConfigurationElRef {
        DataEcsServiceLoadBalancerElAdvancedConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsServiceLoadBalancerElAdvancedConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alternate_target_group_arn` after provisioning.\n"]
    pub fn alternate_target_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alternate_target_group_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `production_listener_rule` after provisioning.\n"]
    pub fn production_listener_rule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.production_listener_rule", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `test_listener_rule` after provisioning.\n"]
    pub fn test_listener_rule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.test_listener_rule", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataEcsServiceLoadBalancerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_configuration: Option<ListField<DataEcsServiceLoadBalancerElAdvancedConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elb_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_group_arn: Option<PrimField<String>>,
}

impl DataEcsServiceLoadBalancerEl {
    #[doc = "Set the field `advanced_configuration`.\n"]
    pub fn set_advanced_configuration(
        mut self,
        v: impl Into<ListField<DataEcsServiceLoadBalancerElAdvancedConfigurationEl>>,
    ) -> Self {
        self.advanced_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `container_name`.\n"]
    pub fn set_container_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container_name = Some(v.into());
        self
    }

    #[doc = "Set the field `container_port`.\n"]
    pub fn set_container_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.container_port = Some(v.into());
        self
    }

    #[doc = "Set the field `elb_name`.\n"]
    pub fn set_elb_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.elb_name = Some(v.into());
        self
    }

    #[doc = "Set the field `target_group_arn`.\n"]
    pub fn set_target_group_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_group_arn = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcsServiceLoadBalancerEl {
    type O = BlockAssignable<DataEcsServiceLoadBalancerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsServiceLoadBalancerEl {}

impl BuildDataEcsServiceLoadBalancerEl {
    pub fn build(self) -> DataEcsServiceLoadBalancerEl {
        DataEcsServiceLoadBalancerEl {
            advanced_configuration: core::default::Default::default(),
            container_name: core::default::Default::default(),
            container_port: core::default::Default::default(),
            elb_name: core::default::Default::default(),
            target_group_arn: core::default::Default::default(),
        }
    }
}

pub struct DataEcsServiceLoadBalancerElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsServiceLoadBalancerElRef {
    fn new(shared: StackShared, base: String) -> DataEcsServiceLoadBalancerElRef {
        DataEcsServiceLoadBalancerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsServiceLoadBalancerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `advanced_configuration` after provisioning.\n"]
    pub fn advanced_configuration(
        &self,
    ) -> ListRef<DataEcsServiceLoadBalancerElAdvancedConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `container_name` after provisioning.\n"]
    pub fn container_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `container_port` after provisioning.\n"]
    pub fn container_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_port", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `elb_name` after provisioning.\n"]
    pub fn elb_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.elb_name", self.base))
    }

    #[doc = "Get a reference to the value of field `target_group_arn` after provisioning.\n"]
    pub fn target_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_group_arn", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataEcsServiceNetworkConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    assign_public_ip: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnets: Option<SetField<PrimField<String>>>,
}

impl DataEcsServiceNetworkConfigurationEl {
    #[doc = "Set the field `assign_public_ip`.\n"]
    pub fn set_assign_public_ip(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.assign_public_ip = Some(v.into());
        self
    }

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_groups = Some(v.into());
        self
    }

    #[doc = "Set the field `subnets`.\n"]
    pub fn set_subnets(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.subnets = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcsServiceNetworkConfigurationEl {
    type O = BlockAssignable<DataEcsServiceNetworkConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsServiceNetworkConfigurationEl {}

impl BuildDataEcsServiceNetworkConfigurationEl {
    pub fn build(self) -> DataEcsServiceNetworkConfigurationEl {
        DataEcsServiceNetworkConfigurationEl {
            assign_public_ip: core::default::Default::default(),
            security_groups: core::default::Default::default(),
            subnets: core::default::Default::default(),
        }
    }
}

pub struct DataEcsServiceNetworkConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsServiceNetworkConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataEcsServiceNetworkConfigurationElRef {
        DataEcsServiceNetworkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsServiceNetworkConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `assign_public_ip` after provisioning.\n"]
    pub fn assign_public_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assign_public_ip", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_groups", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsServiceOrderedPlacementStrategyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataEcsServiceOrderedPlacementStrategyEl {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcsServiceOrderedPlacementStrategyEl {
    type O = BlockAssignable<DataEcsServiceOrderedPlacementStrategyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsServiceOrderedPlacementStrategyEl {}

impl BuildDataEcsServiceOrderedPlacementStrategyEl {
    pub fn build(self) -> DataEcsServiceOrderedPlacementStrategyEl {
        DataEcsServiceOrderedPlacementStrategyEl {
            field: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataEcsServiceOrderedPlacementStrategyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsServiceOrderedPlacementStrategyElRef {
    fn new(shared: StackShared, base: String) -> DataEcsServiceOrderedPlacementStrategyElRef {
        DataEcsServiceOrderedPlacementStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsServiceOrderedPlacementStrategyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsServicePlacementConstraintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataEcsServicePlacementConstraintsEl {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcsServicePlacementConstraintsEl {
    type O = BlockAssignable<DataEcsServicePlacementConstraintsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsServicePlacementConstraintsEl {}

impl BuildDataEcsServicePlacementConstraintsEl {
    pub fn build(self) -> DataEcsServicePlacementConstraintsEl {
        DataEcsServicePlacementConstraintsEl {
            expression: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataEcsServicePlacementConstraintsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsServicePlacementConstraintsElRef {
    fn new(shared: StackShared, base: String) -> DataEcsServicePlacementConstraintsElRef {
        DataEcsServicePlacementConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsServicePlacementConstraintsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsServiceServiceRegistriesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registry_arn: Option<PrimField<String>>,
}

impl DataEcsServiceServiceRegistriesEl {
    #[doc = "Set the field `container_name`.\n"]
    pub fn set_container_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container_name = Some(v.into());
        self
    }

    #[doc = "Set the field `container_port`.\n"]
    pub fn set_container_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.container_port = Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }

    #[doc = "Set the field `registry_arn`.\n"]
    pub fn set_registry_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.registry_arn = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcsServiceServiceRegistriesEl {
    type O = BlockAssignable<DataEcsServiceServiceRegistriesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsServiceServiceRegistriesEl {}

impl BuildDataEcsServiceServiceRegistriesEl {
    pub fn build(self) -> DataEcsServiceServiceRegistriesEl {
        DataEcsServiceServiceRegistriesEl {
            container_name: core::default::Default::default(),
            container_port: core::default::Default::default(),
            port: core::default::Default::default(),
            registry_arn: core::default::Default::default(),
        }
    }
}

pub struct DataEcsServiceServiceRegistriesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsServiceServiceRegistriesElRef {
    fn new(shared: StackShared, base: String) -> DataEcsServiceServiceRegistriesElRef {
        DataEcsServiceServiceRegistriesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsServiceServiceRegistriesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_name` after provisioning.\n"]
    pub fn container_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `container_port` after provisioning.\n"]
    pub fn container_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_port", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `registry_arn` after provisioning.\n"]
    pub fn registry_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsServiceTaskSetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    running_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stability_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_definition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<PrimField<String>>,
}

impl DataEcsServiceTaskSetsEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_at = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `pending_count`.\n"]
    pub fn set_pending_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.pending_count = Some(v.into());
        self
    }

    #[doc = "Set the field `running_count`.\n"]
    pub fn set_running_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.running_count = Some(v.into());
        self
    }

    #[doc = "Set the field `stability_status`.\n"]
    pub fn set_stability_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stability_status = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }

    #[doc = "Set the field `task_definition`.\n"]
    pub fn set_task_definition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.task_definition = Some(v.into());
        self
    }

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.updated_at = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcsServiceTaskSetsEl {
    type O = BlockAssignable<DataEcsServiceTaskSetsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsServiceTaskSetsEl {}

impl BuildDataEcsServiceTaskSetsEl {
    pub fn build(self) -> DataEcsServiceTaskSetsEl {
        DataEcsServiceTaskSetsEl {
            arn: core::default::Default::default(),
            created_at: core::default::Default::default(),
            id: core::default::Default::default(),
            pending_count: core::default::Default::default(),
            running_count: core::default::Default::default(),
            stability_status: core::default::Default::default(),
            status: core::default::Default::default(),
            task_definition: core::default::Default::default(),
            updated_at: core::default::Default::default(),
        }
    }
}

pub struct DataEcsServiceTaskSetsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsServiceTaskSetsElRef {
    fn new(shared: StackShared, base: String) -> DataEcsServiceTaskSetsElRef {
        DataEcsServiceTaskSetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsServiceTaskSetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `pending_count` after provisioning.\n"]
    pub fn pending_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pending_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `running_count` after provisioning.\n"]
    pub fn running_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.running_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `stability_status` after provisioning.\n"]
    pub fn stability_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stability_status", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `task_definition` after provisioning.\n"]
    pub fn task_definition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.task_definition", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}
