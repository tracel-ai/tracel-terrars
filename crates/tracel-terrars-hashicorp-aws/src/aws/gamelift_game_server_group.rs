use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GameliftGameServerGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    balancing_strategy: Option<PrimField<String>>,
    game_server_group_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    game_server_protection_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    max_size: PrimField<f64>,
    min_size: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_subnets: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_scaling_policy: Option<Vec<GameliftGameServerGroupAutoScalingPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_definition: Option<Vec<GameliftGameServerGroupInstanceDefinitionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template: Option<Vec<GameliftGameServerGroupLaunchTemplateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<GameliftGameServerGroupTimeoutsEl>,
    dynamic: GameliftGameServerGroupDynamic,
}

struct GameliftGameServerGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GameliftGameServerGroupData>,
}

#[derive(Clone)]
pub struct GameliftGameServerGroup(Rc<GameliftGameServerGroup_>);

impl GameliftGameServerGroup {
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

    #[doc = "Set the field `balancing_strategy`.\n"]
    pub fn set_balancing_strategy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().balancing_strategy = Some(v.into());
        self
    }

    #[doc = "Set the field `game_server_protection_policy`.\n"]
    pub fn set_game_server_protection_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().game_server_protection_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
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

    #[doc = "Set the field `vpc_subnets`.\n"]
    pub fn set_vpc_subnets(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().vpc_subnets = Some(v.into());
        self
    }

    #[doc = "Set the field `auto_scaling_policy`.\n"]
    pub fn set_auto_scaling_policy(
        self,
        v: impl Into<BlockAssignable<GameliftGameServerGroupAutoScalingPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_scaling_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_scaling_policy = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `instance_definition`.\n"]
    pub fn set_instance_definition(
        self,
        v: impl Into<BlockAssignable<GameliftGameServerGroupInstanceDefinitionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_definition = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_definition = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `launch_template`.\n"]
    pub fn set_launch_template(self, v: impl Into<BlockAssignable<GameliftGameServerGroupLaunchTemplateEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().launch_template = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.launch_template = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<GameliftGameServerGroupTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_scaling_group_arn` after provisioning.\n"]
    pub fn auto_scaling_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_scaling_group_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `balancing_strategy` after provisioning.\n"]
    pub fn balancing_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.balancing_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `game_server_group_name` after provisioning.\n"]
    pub fn game_server_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.game_server_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `game_server_protection_policy` after provisioning.\n"]
    pub fn game_server_protection_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.game_server_protection_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_size` after provisioning.\n"]
    pub fn max_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_size` after provisioning.\n"]
    pub fn min_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_subnets` after provisioning.\n"]
    pub fn vpc_subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.vpc_subnets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_scaling_policy` after provisioning.\n"]
    pub fn auto_scaling_policy(&self) -> ListRef<GameliftGameServerGroupAutoScalingPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto_scaling_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> ListRef<GameliftGameServerGroupLaunchTemplateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.launch_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GameliftGameServerGroupTimeoutsElRef {
        GameliftGameServerGroupTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for GameliftGameServerGroup {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for GameliftGameServerGroup { }

impl ToListMappable for GameliftGameServerGroup {
    type O = ListRef<GameliftGameServerGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GameliftGameServerGroup_ {
    fn extract_resource_type(&self) -> String {
        "aws_gamelift_game_server_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGameliftGameServerGroup {
    pub tf_id: String,
    #[doc = ""]
    pub game_server_group_name: PrimField<String>,
    #[doc = ""]
    pub max_size: PrimField<f64>,
    #[doc = ""]
    pub min_size: PrimField<f64>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}

impl BuildGameliftGameServerGroup {
    pub fn build(self, stack: &mut Stack) -> GameliftGameServerGroup {
        let out = GameliftGameServerGroup(Rc::new(GameliftGameServerGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GameliftGameServerGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                balancing_strategy: core::default::Default::default(),
                game_server_group_name: self.game_server_group_name,
                game_server_protection_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                max_size: self.max_size,
                min_size: self.min_size,
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                vpc_subnets: core::default::Default::default(),
                auto_scaling_policy: core::default::Default::default(),
                instance_definition: core::default::Default::default(),
                launch_template: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GameliftGameServerGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for GameliftGameServerGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl GameliftGameServerGroupRef {
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

    #[doc = "Get a reference to the value of field `auto_scaling_group_arn` after provisioning.\n"]
    pub fn auto_scaling_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_scaling_group_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `balancing_strategy` after provisioning.\n"]
    pub fn balancing_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.balancing_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `game_server_group_name` after provisioning.\n"]
    pub fn game_server_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.game_server_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `game_server_protection_policy` after provisioning.\n"]
    pub fn game_server_protection_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.game_server_protection_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_size` after provisioning.\n"]
    pub fn max_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_size` after provisioning.\n"]
    pub fn min_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_subnets` after provisioning.\n"]
    pub fn vpc_subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.vpc_subnets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_scaling_policy` after provisioning.\n"]
    pub fn auto_scaling_policy(&self) -> ListRef<GameliftGameServerGroupAutoScalingPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto_scaling_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> ListRef<GameliftGameServerGroupLaunchTemplateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.launch_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GameliftGameServerGroupTimeoutsElRef {
        GameliftGameServerGroupTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationEl {
    target_value: PrimField<f64>,
}

impl GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationEl { }

impl ToListMappable for GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationEl {
    type O = BlockAssignable<GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationEl {
    #[doc = ""]
    pub target_value: PrimField<f64>,
}

impl BuildGameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationEl {
    pub fn build(self) -> GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationEl {
        GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationEl { target_value: self.target_value }
    }
}

pub struct GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationElRef {
        GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_value` after provisioning.\n"]
    pub fn target_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct GameliftGameServerGroupAutoScalingPolicyElDynamic {
    target_tracking_configuration: Option<
        DynamicBlock<GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct GameliftGameServerGroupAutoScalingPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    estimated_instance_warmup: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_tracking_configuration: Option<Vec<GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationEl>>,
    dynamic: GameliftGameServerGroupAutoScalingPolicyElDynamic,
}

impl GameliftGameServerGroupAutoScalingPolicyEl {
    #[doc = "Set the field `estimated_instance_warmup`.\n"]
    pub fn set_estimated_instance_warmup(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.estimated_instance_warmup = Some(v.into());
        self
    }

    #[doc = "Set the field `target_tracking_configuration`.\n"]
    pub fn set_target_tracking_configuration(
        mut self,
        v: impl Into<BlockAssignable<GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target_tracking_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target_tracking_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for GameliftGameServerGroupAutoScalingPolicyEl {
    type O = BlockAssignable<GameliftGameServerGroupAutoScalingPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGameliftGameServerGroupAutoScalingPolicyEl {}

impl BuildGameliftGameServerGroupAutoScalingPolicyEl {
    pub fn build(self) -> GameliftGameServerGroupAutoScalingPolicyEl {
        GameliftGameServerGroupAutoScalingPolicyEl {
            estimated_instance_warmup: core::default::Default::default(),
            target_tracking_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct GameliftGameServerGroupAutoScalingPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GameliftGameServerGroupAutoScalingPolicyElRef {
    fn new(shared: StackShared, base: String) -> GameliftGameServerGroupAutoScalingPolicyElRef {
        GameliftGameServerGroupAutoScalingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GameliftGameServerGroupAutoScalingPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `estimated_instance_warmup` after provisioning.\n"]
    pub fn estimated_instance_warmup(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.estimated_instance_warmup", self.base))
    }

    #[doc = "Get a reference to the value of field `target_tracking_configuration` after provisioning.\n"]
    pub fn target_tracking_configuration(
        &self,
    ) -> ListRef<GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target_tracking_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct GameliftGameServerGroupInstanceDefinitionEl {
    instance_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_capacity: Option<PrimField<String>>,
}

impl GameliftGameServerGroupInstanceDefinitionEl {
    #[doc = "Set the field `weighted_capacity`.\n"]
    pub fn set_weighted_capacity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.weighted_capacity = Some(v.into());
        self
    }
}

impl ToListMappable for GameliftGameServerGroupInstanceDefinitionEl {
    type O = BlockAssignable<GameliftGameServerGroupInstanceDefinitionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGameliftGameServerGroupInstanceDefinitionEl {
    #[doc = ""]
    pub instance_type: PrimField<String>,
}

impl BuildGameliftGameServerGroupInstanceDefinitionEl {
    pub fn build(self) -> GameliftGameServerGroupInstanceDefinitionEl {
        GameliftGameServerGroupInstanceDefinitionEl {
            instance_type: self.instance_type,
            weighted_capacity: core::default::Default::default(),
        }
    }
}

pub struct GameliftGameServerGroupInstanceDefinitionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GameliftGameServerGroupInstanceDefinitionElRef {
    fn new(shared: StackShared, base: String) -> GameliftGameServerGroupInstanceDefinitionElRef {
        GameliftGameServerGroupInstanceDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GameliftGameServerGroupInstanceDefinitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `weighted_capacity` after provisioning.\n"]
    pub fn weighted_capacity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.weighted_capacity", self.base))
    }
}

#[derive(Serialize)]
pub struct GameliftGameServerGroupLaunchTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl GameliftGameServerGroupLaunchTemplateEl {
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

impl ToListMappable for GameliftGameServerGroupLaunchTemplateEl {
    type O = BlockAssignable<GameliftGameServerGroupLaunchTemplateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGameliftGameServerGroupLaunchTemplateEl {}

impl BuildGameliftGameServerGroupLaunchTemplateEl {
    pub fn build(self) -> GameliftGameServerGroupLaunchTemplateEl {
        GameliftGameServerGroupLaunchTemplateEl {
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct GameliftGameServerGroupLaunchTemplateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GameliftGameServerGroupLaunchTemplateElRef {
    fn new(shared: StackShared, base: String) -> GameliftGameServerGroupLaunchTemplateElRef {
        GameliftGameServerGroupLaunchTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GameliftGameServerGroupLaunchTemplateElRef {
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
pub struct GameliftGameServerGroupTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl GameliftGameServerGroupTimeoutsEl {
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
}

impl ToListMappable for GameliftGameServerGroupTimeoutsEl {
    type O = BlockAssignable<GameliftGameServerGroupTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGameliftGameServerGroupTimeoutsEl {}

impl BuildGameliftGameServerGroupTimeoutsEl {
    pub fn build(self) -> GameliftGameServerGroupTimeoutsEl {
        GameliftGameServerGroupTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct GameliftGameServerGroupTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GameliftGameServerGroupTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> GameliftGameServerGroupTimeoutsElRef {
        GameliftGameServerGroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GameliftGameServerGroupTimeoutsElRef {
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
}

#[derive(Serialize, Default)]
struct GameliftGameServerGroupDynamic {
    auto_scaling_policy: Option<DynamicBlock<GameliftGameServerGroupAutoScalingPolicyEl>>,
    instance_definition: Option<DynamicBlock<GameliftGameServerGroupInstanceDefinitionEl>>,
    launch_template: Option<DynamicBlock<GameliftGameServerGroupLaunchTemplateEl>>,
}
