use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEcsTaskExecutionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_token: Option<PrimField<String>>,
    cluster: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_ecs_managed_tags: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_execute_command: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    propagate_tags: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    started_by: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    task_definition: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_provider_strategy: Option<Vec<DataEcsTaskExecutionCapacityProviderStrategyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_configuration: Option<Vec<DataEcsTaskExecutionNetworkConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overrides: Option<Vec<DataEcsTaskExecutionOverridesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement_constraints: Option<Vec<DataEcsTaskExecutionPlacementConstraintsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement_strategy: Option<Vec<DataEcsTaskExecutionPlacementStrategyEl>>,
    dynamic: DataEcsTaskExecutionDynamic,
}

struct DataEcsTaskExecution_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEcsTaskExecutionData>,
}

#[derive(Clone)]
pub struct DataEcsTaskExecution(Rc<DataEcsTaskExecution_>);

impl DataEcsTaskExecution {
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

    #[doc = "Set the field `client_token`.\n"]
    pub fn set_client_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().client_token = Some(v.into());
        self
    }

    #[doc = "Set the field `desired_count`.\n"]
    pub fn set_desired_count(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().desired_count = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_ecs_managed_tags`.\n"]
    pub fn set_enable_ecs_managed_tags(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_ecs_managed_tags = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_execute_command`.\n"]
    pub fn set_enable_execute_command(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_execute_command = Some(v.into());
        self
    }

    #[doc = "Set the field `group`.\n"]
    pub fn set_group(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().group = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `launch_type`.\n"]
    pub fn set_launch_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().launch_type = Some(v.into());
        self
    }

    #[doc = "Set the field `platform_version`.\n"]
    pub fn set_platform_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().platform_version = Some(v.into());
        self
    }

    #[doc = "Set the field `propagate_tags`.\n"]
    pub fn set_propagate_tags(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().propagate_tags = Some(v.into());
        self
    }

    #[doc = "Set the field `reference_id`.\n"]
    pub fn set_reference_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().reference_id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `started_by`.\n"]
    pub fn set_started_by(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().started_by = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_provider_strategy`.\n"]
    pub fn set_capacity_provider_strategy(
        self,
        v: impl Into<BlockAssignable<DataEcsTaskExecutionCapacityProviderStrategyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capacity_provider_strategy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.capacity_provider_strategy = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        self,
        v: impl Into<BlockAssignable<DataEcsTaskExecutionNetworkConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `overrides`.\n"]
    pub fn set_overrides(self, v: impl Into<BlockAssignable<DataEcsTaskExecutionOverridesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().overrides = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.overrides = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `placement_constraints`.\n"]
    pub fn set_placement_constraints(
        self,
        v: impl Into<BlockAssignable<DataEcsTaskExecutionPlacementConstraintsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().placement_constraints = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.placement_constraints = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `placement_strategy`.\n"]
    pub fn set_placement_strategy(self, v: impl Into<BlockAssignable<DataEcsTaskExecutionPlacementStrategyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().placement_strategy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.placement_strategy = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `client_token` after provisioning.\n"]
    pub fn client_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `desired_count` after provisioning.\n"]
    pub fn desired_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.desired_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_ecs_managed_tags` after provisioning.\n"]
    pub fn enable_ecs_managed_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_ecs_managed_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_execute_command` after provisioning.\n"]
    pub fn enable_execute_command(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_execute_command", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_type` after provisioning.\n"]
    pub fn launch_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.launch_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.propagate_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reference_id` after provisioning.\n"]
    pub fn reference_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reference_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `started_by` after provisioning.\n"]
    pub fn started_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.started_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_arns` after provisioning.\n"]
    pub fn task_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.task_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_definition` after provisioning.\n"]
    pub fn task_definition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> ListRef<DataEcsTaskExecutionNetworkConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `overrides` after provisioning.\n"]
    pub fn overrides(&self) -> ListRef<DataEcsTaskExecutionOverridesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.overrides", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_strategy` after provisioning.\n"]
    pub fn placement_strategy(&self) -> ListRef<DataEcsTaskExecutionPlacementStrategyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.placement_strategy", self.extract_ref()))
    }
}

impl Referable for DataEcsTaskExecution {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataEcsTaskExecution { }

impl ToListMappable for DataEcsTaskExecution {
    type O = ListRef<DataEcsTaskExecutionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEcsTaskExecution_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ecs_task_execution".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEcsTaskExecution {
    pub tf_id: String,
    #[doc = ""]
    pub cluster: PrimField<String>,
    #[doc = ""]
    pub task_definition: PrimField<String>,
}

impl BuildDataEcsTaskExecution {
    pub fn build(self, stack: &mut Stack) -> DataEcsTaskExecution {
        let out = DataEcsTaskExecution(Rc::new(DataEcsTaskExecution_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEcsTaskExecutionData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                client_token: core::default::Default::default(),
                cluster: self.cluster,
                desired_count: core::default::Default::default(),
                enable_ecs_managed_tags: core::default::Default::default(),
                enable_execute_command: core::default::Default::default(),
                group: core::default::Default::default(),
                id: core::default::Default::default(),
                launch_type: core::default::Default::default(),
                platform_version: core::default::Default::default(),
                propagate_tags: core::default::Default::default(),
                reference_id: core::default::Default::default(),
                region: core::default::Default::default(),
                started_by: core::default::Default::default(),
                tags: core::default::Default::default(),
                task_definition: self.task_definition,
                capacity_provider_strategy: core::default::Default::default(),
                network_configuration: core::default::Default::default(),
                overrides: core::default::Default::default(),
                placement_constraints: core::default::Default::default(),
                placement_strategy: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEcsTaskExecutionRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsTaskExecutionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataEcsTaskExecutionRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `client_token` after provisioning.\n"]
    pub fn client_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `desired_count` after provisioning.\n"]
    pub fn desired_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.desired_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_ecs_managed_tags` after provisioning.\n"]
    pub fn enable_ecs_managed_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_ecs_managed_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_execute_command` after provisioning.\n"]
    pub fn enable_execute_command(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_execute_command", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_type` after provisioning.\n"]
    pub fn launch_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.launch_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.propagate_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reference_id` after provisioning.\n"]
    pub fn reference_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reference_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `started_by` after provisioning.\n"]
    pub fn started_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.started_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_arns` after provisioning.\n"]
    pub fn task_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.task_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_definition` after provisioning.\n"]
    pub fn task_definition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> ListRef<DataEcsTaskExecutionNetworkConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `overrides` after provisioning.\n"]
    pub fn overrides(&self) -> ListRef<DataEcsTaskExecutionOverridesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.overrides", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_strategy` after provisioning.\n"]
    pub fn placement_strategy(&self) -> ListRef<DataEcsTaskExecutionPlacementStrategyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.placement_strategy", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEcsTaskExecutionCapacityProviderStrategyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    base: Option<PrimField<f64>>,
    capacity_provider: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}

impl DataEcsTaskExecutionCapacityProviderStrategyEl {
    #[doc = "Set the field `base`.\n"]
    pub fn set_base(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.base = Some(v.into());
        self
    }

    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcsTaskExecutionCapacityProviderStrategyEl {
    type O = BlockAssignable<DataEcsTaskExecutionCapacityProviderStrategyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskExecutionCapacityProviderStrategyEl {
    #[doc = ""]
    pub capacity_provider: PrimField<String>,
}

impl BuildDataEcsTaskExecutionCapacityProviderStrategyEl {
    pub fn build(self) -> DataEcsTaskExecutionCapacityProviderStrategyEl {
        DataEcsTaskExecutionCapacityProviderStrategyEl {
            base: core::default::Default::default(),
            capacity_provider: self.capacity_provider,
            weight: core::default::Default::default(),
        }
    }
}

pub struct DataEcsTaskExecutionCapacityProviderStrategyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsTaskExecutionCapacityProviderStrategyElRef {
    fn new(shared: StackShared, base: String) -> DataEcsTaskExecutionCapacityProviderStrategyElRef {
        DataEcsTaskExecutionCapacityProviderStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskExecutionCapacityProviderStrategyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base` after provisioning.\n"]
    pub fn base(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.base", self.base))
    }

    #[doc = "Get a reference to the value of field `capacity_provider` after provisioning.\n"]
    pub fn capacity_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.capacity_provider", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsTaskExecutionNetworkConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    assign_public_ip: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups: Option<SetField<PrimField<String>>>,
    subnets: SetField<PrimField<String>>,
}

impl DataEcsTaskExecutionNetworkConfigurationEl {
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
}

impl ToListMappable for DataEcsTaskExecutionNetworkConfigurationEl {
    type O = BlockAssignable<DataEcsTaskExecutionNetworkConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskExecutionNetworkConfigurationEl {
    #[doc = ""]
    pub subnets: SetField<PrimField<String>>,
}

impl BuildDataEcsTaskExecutionNetworkConfigurationEl {
    pub fn build(self) -> DataEcsTaskExecutionNetworkConfigurationEl {
        DataEcsTaskExecutionNetworkConfigurationEl {
            assign_public_ip: core::default::Default::default(),
            security_groups: core::default::Default::default(),
            subnets: self.subnets,
        }
    }
}

pub struct DataEcsTaskExecutionNetworkConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsTaskExecutionNetworkConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataEcsTaskExecutionNetworkConfigurationElRef {
        DataEcsTaskExecutionNetworkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskExecutionNetworkConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `assign_public_ip` after provisioning.\n"]
    pub fn assign_public_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.assign_public_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsTaskExecutionOverridesElContainerOverridesElEnvironmentEl {
    key: PrimField<String>,
    value: PrimField<String>,
}

impl DataEcsTaskExecutionOverridesElContainerOverridesElEnvironmentEl { }

impl ToListMappable for DataEcsTaskExecutionOverridesElContainerOverridesElEnvironmentEl {
    type O = BlockAssignable<DataEcsTaskExecutionOverridesElContainerOverridesElEnvironmentEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskExecutionOverridesElContainerOverridesElEnvironmentEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildDataEcsTaskExecutionOverridesElContainerOverridesElEnvironmentEl {
    pub fn build(self) -> DataEcsTaskExecutionOverridesElContainerOverridesElEnvironmentEl {
        DataEcsTaskExecutionOverridesElContainerOverridesElEnvironmentEl {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct DataEcsTaskExecutionOverridesElContainerOverridesElEnvironmentElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsTaskExecutionOverridesElContainerOverridesElEnvironmentElRef {
    fn new(shared: StackShared, base: String) -> DataEcsTaskExecutionOverridesElContainerOverridesElEnvironmentElRef {
        DataEcsTaskExecutionOverridesElContainerOverridesElEnvironmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskExecutionOverridesElContainerOverridesElEnvironmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsTaskExecutionOverridesElContainerOverridesElResourceRequirementsEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    value: PrimField<String>,
}

impl DataEcsTaskExecutionOverridesElContainerOverridesElResourceRequirementsEl { }

impl ToListMappable for DataEcsTaskExecutionOverridesElContainerOverridesElResourceRequirementsEl {
    type O = BlockAssignable<DataEcsTaskExecutionOverridesElContainerOverridesElResourceRequirementsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskExecutionOverridesElContainerOverridesElResourceRequirementsEl {
    #[doc = ""]
    pub type_: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildDataEcsTaskExecutionOverridesElContainerOverridesElResourceRequirementsEl {
    pub fn build(self) -> DataEcsTaskExecutionOverridesElContainerOverridesElResourceRequirementsEl {
        DataEcsTaskExecutionOverridesElContainerOverridesElResourceRequirementsEl {
            type_: self.type_,
            value: self.value,
        }
    }
}

pub struct DataEcsTaskExecutionOverridesElContainerOverridesElResourceRequirementsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsTaskExecutionOverridesElContainerOverridesElResourceRequirementsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEcsTaskExecutionOverridesElContainerOverridesElResourceRequirementsElRef {
        DataEcsTaskExecutionOverridesElContainerOverridesElResourceRequirementsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskExecutionOverridesElContainerOverridesElResourceRequirementsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEcsTaskExecutionOverridesElContainerOverridesElDynamic {
    environment: Option<DynamicBlock<DataEcsTaskExecutionOverridesElContainerOverridesElEnvironmentEl>>,
    resource_requirements: Option<
        DynamicBlock<DataEcsTaskExecutionOverridesElContainerOverridesElResourceRequirementsEl>,
    >,
}

#[derive(Serialize)]
pub struct DataEcsTaskExecutionOverridesElContainerOverridesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_reservation: Option<PrimField<f64>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<Vec<DataEcsTaskExecutionOverridesElContainerOverridesElEnvironmentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_requirements: Option<Vec<DataEcsTaskExecutionOverridesElContainerOverridesElResourceRequirementsEl>>,
    dynamic: DataEcsTaskExecutionOverridesElContainerOverridesElDynamic,
}

impl DataEcsTaskExecutionOverridesElContainerOverridesEl {
    #[doc = "Set the field `command`.\n"]
    pub fn set_command(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.command = Some(v.into());
        self
    }

    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu = Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory = Some(v.into());
        self
    }

    #[doc = "Set the field `memory_reservation`.\n"]
    pub fn set_memory_reservation(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_reservation = Some(v.into());
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v: impl Into<BlockAssignable<DataEcsTaskExecutionOverridesElContainerOverridesElEnvironmentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.environment = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.environment = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_requirements`.\n"]
    pub fn set_resource_requirements(
        mut self,
        v: impl Into<BlockAssignable<DataEcsTaskExecutionOverridesElContainerOverridesElResourceRequirementsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_requirements = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_requirements = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DataEcsTaskExecutionOverridesElContainerOverridesEl {
    type O = BlockAssignable<DataEcsTaskExecutionOverridesElContainerOverridesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskExecutionOverridesElContainerOverridesEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildDataEcsTaskExecutionOverridesElContainerOverridesEl {
    pub fn build(self) -> DataEcsTaskExecutionOverridesElContainerOverridesEl {
        DataEcsTaskExecutionOverridesElContainerOverridesEl {
            command: core::default::Default::default(),
            cpu: core::default::Default::default(),
            memory: core::default::Default::default(),
            memory_reservation: core::default::Default::default(),
            name: self.name,
            environment: core::default::Default::default(),
            resource_requirements: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DataEcsTaskExecutionOverridesElContainerOverridesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsTaskExecutionOverridesElContainerOverridesElRef {
    fn new(shared: StackShared, base: String) -> DataEcsTaskExecutionOverridesElContainerOverridesElRef {
        DataEcsTaskExecutionOverridesElContainerOverridesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskExecutionOverridesElContainerOverridesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_reservation` after provisioning.\n"]
    pub fn memory_reservation(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_reservation", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEcsTaskExecutionOverridesElDynamic {
    container_overrides: Option<DynamicBlock<DataEcsTaskExecutionOverridesElContainerOverridesEl>>,
}

#[derive(Serialize)]
pub struct DataEcsTaskExecutionOverridesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_overrides: Option<Vec<DataEcsTaskExecutionOverridesElContainerOverridesEl>>,
    dynamic: DataEcsTaskExecutionOverridesElDynamic,
}

impl DataEcsTaskExecutionOverridesEl {
    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cpu = Some(v.into());
        self
    }

    #[doc = "Set the field `execution_role_arn`.\n"]
    pub fn set_execution_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.execution_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.memory = Some(v.into());
        self
    }

    #[doc = "Set the field `task_role_arn`.\n"]
    pub fn set_task_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.task_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `container_overrides`.\n"]
    pub fn set_container_overrides(
        mut self,
        v: impl Into<BlockAssignable<DataEcsTaskExecutionOverridesElContainerOverridesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.container_overrides = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.container_overrides = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DataEcsTaskExecutionOverridesEl {
    type O = BlockAssignable<DataEcsTaskExecutionOverridesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskExecutionOverridesEl {}

impl BuildDataEcsTaskExecutionOverridesEl {
    pub fn build(self) -> DataEcsTaskExecutionOverridesEl {
        DataEcsTaskExecutionOverridesEl {
            cpu: core::default::Default::default(),
            execution_role_arn: core::default::Default::default(),
            memory: core::default::Default::default(),
            task_role_arn: core::default::Default::default(),
            container_overrides: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DataEcsTaskExecutionOverridesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsTaskExecutionOverridesElRef {
    fn new(shared: StackShared, base: String) -> DataEcsTaskExecutionOverridesElRef {
        DataEcsTaskExecutionOverridesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskExecutionOverridesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }

    #[doc = "Get a reference to the value of field `task_role_arn` after provisioning.\n"]
    pub fn task_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `container_overrides` after provisioning.\n"]
    pub fn container_overrides(&self) -> ListRef<DataEcsTaskExecutionOverridesElContainerOverridesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.container_overrides", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsTaskExecutionPlacementConstraintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl DataEcsTaskExecutionPlacementConstraintsEl {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcsTaskExecutionPlacementConstraintsEl {
    type O = BlockAssignable<DataEcsTaskExecutionPlacementConstraintsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskExecutionPlacementConstraintsEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildDataEcsTaskExecutionPlacementConstraintsEl {
    pub fn build(self) -> DataEcsTaskExecutionPlacementConstraintsEl {
        DataEcsTaskExecutionPlacementConstraintsEl {
            expression: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct DataEcsTaskExecutionPlacementConstraintsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsTaskExecutionPlacementConstraintsElRef {
    fn new(shared: StackShared, base: String) -> DataEcsTaskExecutionPlacementConstraintsElRef {
        DataEcsTaskExecutionPlacementConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskExecutionPlacementConstraintsElRef {
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
pub struct DataEcsTaskExecutionPlacementStrategyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl DataEcsTaskExecutionPlacementStrategyEl {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcsTaskExecutionPlacementStrategyEl {
    type O = BlockAssignable<DataEcsTaskExecutionPlacementStrategyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskExecutionPlacementStrategyEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildDataEcsTaskExecutionPlacementStrategyEl {
    pub fn build(self) -> DataEcsTaskExecutionPlacementStrategyEl {
        DataEcsTaskExecutionPlacementStrategyEl {
            field: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct DataEcsTaskExecutionPlacementStrategyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcsTaskExecutionPlacementStrategyElRef {
    fn new(shared: StackShared, base: String) -> DataEcsTaskExecutionPlacementStrategyElRef {
        DataEcsTaskExecutionPlacementStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskExecutionPlacementStrategyElRef {
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

#[derive(Serialize, Default)]
struct DataEcsTaskExecutionDynamic {
    capacity_provider_strategy: Option<DynamicBlock<DataEcsTaskExecutionCapacityProviderStrategyEl>>,
    network_configuration: Option<DynamicBlock<DataEcsTaskExecutionNetworkConfigurationEl>>,
    overrides: Option<DynamicBlock<DataEcsTaskExecutionOverridesEl>>,
    placement_constraints: Option<DynamicBlock<DataEcsTaskExecutionPlacementConstraintsEl>>,
    placement_strategy: Option<DynamicBlock<DataEcsTaskExecutionPlacementStrategyEl>>,
}
