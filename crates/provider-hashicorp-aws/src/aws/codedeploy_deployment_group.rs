use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct CodedeployDeploymentGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    app_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_groups: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_config_name: Option<PrimField<String>>,
    deployment_group_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outdated_instances_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    service_role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    termination_hook_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alarm_configuration: Option<Vec<CodedeployDeploymentGroupAlarmConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_rollback_configuration: Option<Vec<CodedeployDeploymentGroupAutoRollbackConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blue_green_deployment_config: Option<Vec<CodedeployDeploymentGroupBlueGreenDeploymentConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_style: Option<Vec<CodedeployDeploymentGroupDeploymentStyleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ec2_tag_filter: Option<Vec<CodedeployDeploymentGroupEc2TagFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ec2_tag_set: Option<Vec<CodedeployDeploymentGroupEc2TagSetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecs_service: Option<Vec<CodedeployDeploymentGroupEcsServiceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer_info: Option<Vec<CodedeployDeploymentGroupLoadBalancerInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_premises_instance_tag_filter:
        Option<Vec<CodedeployDeploymentGroupOnPremisesInstanceTagFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger_configuration: Option<Vec<CodedeployDeploymentGroupTriggerConfigurationEl>>,
    dynamic: CodedeployDeploymentGroupDynamic,
}

struct CodedeployDeploymentGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CodedeployDeploymentGroupData>,
}

#[derive(Clone)]
pub struct CodedeployDeploymentGroup(Rc<CodedeployDeploymentGroup_>);

impl CodedeployDeploymentGroup {
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

    #[doc = "Set the field `autoscaling_groups`.\n"]
    pub fn set_autoscaling_groups(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().autoscaling_groups = Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_config_name`.\n"]
    pub fn set_deployment_config_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deployment_config_name = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `outdated_instances_strategy`.\n"]
    pub fn set_outdated_instances_strategy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().outdated_instances_strategy = Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `termination_hook_enabled`.\n"]
    pub fn set_termination_hook_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().termination_hook_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `alarm_configuration`.\n"]
    pub fn set_alarm_configuration(
        self,
        v: impl Into<BlockAssignable<CodedeployDeploymentGroupAlarmConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().alarm_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.alarm_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `auto_rollback_configuration`.\n"]
    pub fn set_auto_rollback_configuration(
        self,
        v: impl Into<BlockAssignable<CodedeployDeploymentGroupAutoRollbackConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_rollback_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_rollback_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `blue_green_deployment_config`.\n"]
    pub fn set_blue_green_deployment_config(
        self,
        v: impl Into<BlockAssignable<CodedeployDeploymentGroupBlueGreenDeploymentConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().blue_green_deployment_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .blue_green_deployment_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `deployment_style`.\n"]
    pub fn set_deployment_style(
        self,
        v: impl Into<BlockAssignable<CodedeployDeploymentGroupDeploymentStyleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deployment_style = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deployment_style = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `ec2_tag_filter`.\n"]
    pub fn set_ec2_tag_filter(
        self,
        v: impl Into<BlockAssignable<CodedeployDeploymentGroupEc2TagFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ec2_tag_filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ec2_tag_filter = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `ec2_tag_set`.\n"]
    pub fn set_ec2_tag_set(
        self,
        v: impl Into<BlockAssignable<CodedeployDeploymentGroupEc2TagSetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ec2_tag_set = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ec2_tag_set = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `ecs_service`.\n"]
    pub fn set_ecs_service(
        self,
        v: impl Into<BlockAssignable<CodedeployDeploymentGroupEcsServiceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ecs_service = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ecs_service = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `load_balancer_info`.\n"]
    pub fn set_load_balancer_info(
        self,
        v: impl Into<BlockAssignable<CodedeployDeploymentGroupLoadBalancerInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().load_balancer_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.load_balancer_info = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `on_premises_instance_tag_filter`.\n"]
    pub fn set_on_premises_instance_tag_filter(
        self,
        v: impl Into<BlockAssignable<CodedeployDeploymentGroupOnPremisesInstanceTagFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().on_premises_instance_tag_filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .on_premises_instance_tag_filter = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `trigger_configuration`.\n"]
    pub fn set_trigger_configuration(
        self,
        v: impl Into<BlockAssignable<CodedeployDeploymentGroupTriggerConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().trigger_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.trigger_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `app_name` after provisioning.\n"]
    pub fn app_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling_groups` after provisioning.\n"]
    pub fn autoscaling_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_groups", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `compute_platform` after provisioning.\n"]
    pub fn compute_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_platform", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deployment_config_name` after provisioning.\n"]
    pub fn deployment_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_config_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deployment_group_id` after provisioning.\n"]
    pub fn deployment_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_group_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deployment_group_name` after provisioning.\n"]
    pub fn deployment_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_group_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outdated_instances_strategy` after provisioning.\n"]
    pub fn outdated_instances_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.outdated_instances_strategy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_role_arn", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `termination_hook_enabled` after provisioning.\n"]
    pub fn termination_hook_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.termination_hook_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `alarm_configuration` after provisioning.\n"]
    pub fn alarm_configuration(&self) -> ListRef<CodedeployDeploymentGroupAlarmConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.alarm_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `auto_rollback_configuration` after provisioning.\n"]
    pub fn auto_rollback_configuration(
        &self,
    ) -> ListRef<CodedeployDeploymentGroupAutoRollbackConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_rollback_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `blue_green_deployment_config` after provisioning.\n"]
    pub fn blue_green_deployment_config(
        &self,
    ) -> ListRef<CodedeployDeploymentGroupBlueGreenDeploymentConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.blue_green_deployment_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deployment_style` after provisioning.\n"]
    pub fn deployment_style(&self) -> ListRef<CodedeployDeploymentGroupDeploymentStyleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deployment_style", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ecs_service` after provisioning.\n"]
    pub fn ecs_service(&self) -> ListRef<CodedeployDeploymentGroupEcsServiceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ecs_service", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `load_balancer_info` after provisioning.\n"]
    pub fn load_balancer_info(&self) -> ListRef<CodedeployDeploymentGroupLoadBalancerInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.load_balancer_info", self.extract_ref()),
        )
    }
}

impl Referable for CodedeployDeploymentGroup {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for CodedeployDeploymentGroup {}

impl ToListMappable for CodedeployDeploymentGroup {
    type O = ListRef<CodedeployDeploymentGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CodedeployDeploymentGroup_ {
    fn extract_resource_type(&self) -> String {
        "aws_codedeploy_deployment_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCodedeployDeploymentGroup {
    pub tf_id: String,
    #[doc = ""]
    pub app_name: PrimField<String>,
    #[doc = ""]
    pub deployment_group_name: PrimField<String>,
    #[doc = ""]
    pub service_role_arn: PrimField<String>,
}

impl BuildCodedeployDeploymentGroup {
    pub fn build(self, stack: &mut Stack) -> CodedeployDeploymentGroup {
        let out = CodedeployDeploymentGroup(Rc::new(CodedeployDeploymentGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CodedeployDeploymentGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                app_name: self.app_name,
                autoscaling_groups: core::default::Default::default(),
                deployment_config_name: core::default::Default::default(),
                deployment_group_name: self.deployment_group_name,
                id: core::default::Default::default(),
                outdated_instances_strategy: core::default::Default::default(),
                region: core::default::Default::default(),
                service_role_arn: self.service_role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                termination_hook_enabled: core::default::Default::default(),
                alarm_configuration: core::default::Default::default(),
                auto_rollback_configuration: core::default::Default::default(),
                blue_green_deployment_config: core::default::Default::default(),
                deployment_style: core::default::Default::default(),
                ec2_tag_filter: core::default::Default::default(),
                ec2_tag_set: core::default::Default::default(),
                ecs_service: core::default::Default::default(),
                load_balancer_info: core::default::Default::default(),
                on_premises_instance_tag_filter: core::default::Default::default(),
                trigger_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CodedeployDeploymentGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl CodedeployDeploymentGroupRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_name` after provisioning.\n"]
    pub fn app_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling_groups` after provisioning.\n"]
    pub fn autoscaling_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_groups", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `compute_platform` after provisioning.\n"]
    pub fn compute_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_platform", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deployment_config_name` after provisioning.\n"]
    pub fn deployment_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_config_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deployment_group_id` after provisioning.\n"]
    pub fn deployment_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_group_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deployment_group_name` after provisioning.\n"]
    pub fn deployment_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_group_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outdated_instances_strategy` after provisioning.\n"]
    pub fn outdated_instances_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.outdated_instances_strategy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_role_arn", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `termination_hook_enabled` after provisioning.\n"]
    pub fn termination_hook_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.termination_hook_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `alarm_configuration` after provisioning.\n"]
    pub fn alarm_configuration(&self) -> ListRef<CodedeployDeploymentGroupAlarmConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.alarm_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `auto_rollback_configuration` after provisioning.\n"]
    pub fn auto_rollback_configuration(
        &self,
    ) -> ListRef<CodedeployDeploymentGroupAutoRollbackConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_rollback_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `blue_green_deployment_config` after provisioning.\n"]
    pub fn blue_green_deployment_config(
        &self,
    ) -> ListRef<CodedeployDeploymentGroupBlueGreenDeploymentConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.blue_green_deployment_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deployment_style` after provisioning.\n"]
    pub fn deployment_style(&self) -> ListRef<CodedeployDeploymentGroupDeploymentStyleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deployment_style", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ecs_service` after provisioning.\n"]
    pub fn ecs_service(&self) -> ListRef<CodedeployDeploymentGroupEcsServiceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ecs_service", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `load_balancer_info` after provisioning.\n"]
    pub fn load_balancer_info(&self) -> ListRef<CodedeployDeploymentGroupLoadBalancerInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.load_balancer_info", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupAlarmConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    alarms: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_poll_alarm_failure: Option<PrimField<bool>>,
}

impl CodedeployDeploymentGroupAlarmConfigurationEl {
    #[doc = "Set the field `alarms`.\n"]
    pub fn set_alarms(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.alarms = Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_poll_alarm_failure`.\n"]
    pub fn set_ignore_poll_alarm_failure(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_poll_alarm_failure = Some(v.into());
        self
    }
}

impl ToListMappable for CodedeployDeploymentGroupAlarmConfigurationEl {
    type O = BlockAssignable<CodedeployDeploymentGroupAlarmConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupAlarmConfigurationEl {}

impl BuildCodedeployDeploymentGroupAlarmConfigurationEl {
    pub fn build(self) -> CodedeployDeploymentGroupAlarmConfigurationEl {
        CodedeployDeploymentGroupAlarmConfigurationEl {
            alarms: core::default::Default::default(),
            enabled: core::default::Default::default(),
            ignore_poll_alarm_failure: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupAlarmConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentGroupAlarmConfigurationElRef {
    fn new(shared: StackShared, base: String) -> CodedeployDeploymentGroupAlarmConfigurationElRef {
        CodedeployDeploymentGroupAlarmConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupAlarmConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alarms` after provisioning.\n"]
    pub fn alarms(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.alarms", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `ignore_poll_alarm_failure` after provisioning.\n"]
    pub fn ignore_poll_alarm_failure(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_poll_alarm_failure", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupAutoRollbackConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    events: Option<SetField<PrimField<String>>>,
}

impl CodedeployDeploymentGroupAutoRollbackConfigurationEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `events`.\n"]
    pub fn set_events(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.events = Some(v.into());
        self
    }
}

impl ToListMappable for CodedeployDeploymentGroupAutoRollbackConfigurationEl {
    type O = BlockAssignable<CodedeployDeploymentGroupAutoRollbackConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupAutoRollbackConfigurationEl {}

impl BuildCodedeployDeploymentGroupAutoRollbackConfigurationEl {
    pub fn build(self) -> CodedeployDeploymentGroupAutoRollbackConfigurationEl {
        CodedeployDeploymentGroupAutoRollbackConfigurationEl {
            enabled: core::default::Default::default(),
            events: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupAutoRollbackConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentGroupAutoRollbackConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodedeployDeploymentGroupAutoRollbackConfigurationElRef {
        CodedeployDeploymentGroupAutoRollbackConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupAutoRollbackConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `events` after provisioning.\n"]
    pub fn events(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.events", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action_on_timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_time_in_minutes: Option<PrimField<f64>>,
}

impl CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionEl {
    #[doc = "Set the field `action_on_timeout`.\n"]
    pub fn set_action_on_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action_on_timeout = Some(v.into());
        self
    }

    #[doc = "Set the field `wait_time_in_minutes`.\n"]
    pub fn set_wait_time_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.wait_time_in_minutes = Some(v.into());
        self
    }
}

impl ToListMappable
    for CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionEl
{
    type O = BlockAssignable<
        CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionEl {}

impl BuildCodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionEl {
    pub fn build(
        self,
    ) -> CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionEl {
        CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionEl {
            action_on_timeout: core::default::Default::default(),
            wait_time_in_minutes: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionElRef {
        CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_on_timeout` after provisioning.\n"]
    pub fn action_on_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_on_timeout", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `wait_time_in_minutes` after provisioning.\n"]
    pub fn wait_time_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_time_in_minutes", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
}

impl CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action = Some(v.into());
        self
    }
}

impl ToListMappable
    for CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionEl
{
    type O = BlockAssignable<
        CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionEl
{}

impl BuildCodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionEl {
    pub fn build(
        self,
    ) -> CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionEl {
        CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionEl {
            action: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionElRef {
        CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    termination_wait_time_in_minutes: Option<PrimField<f64>>,
}

impl
    CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessEl
{
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action = Some(v.into());
        self
    }

    #[doc = "Set the field `termination_wait_time_in_minutes`.\n"]
    pub fn set_termination_wait_time_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.termination_wait_time_in_minutes = Some(v.into());
        self
    }
}

impl ToListMappable for CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessEl {
    type O =
        BlockAssignable<
            CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessEl
{}

impl BuildCodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessEl {
    pub fn build(
        self,
    ) -> CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessEl {
        CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessEl {
            action: core::default::Default::default(),
            termination_wait_time_in_minutes: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessElRef {
        CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `termination_wait_time_in_minutes` after provisioning.\n"]
    pub fn termination_wait_time_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.termination_wait_time_in_minutes", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodedeployDeploymentGroupBlueGreenDeploymentConfigElDynamic {
    deployment_ready_option: Option<
        DynamicBlock<CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionEl>,
    >,
    green_fleet_provisioning_option: Option<
        DynamicBlock<CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionEl>,
    >,
    terminate_blue_instances_on_deployment_success: Option<
        DynamicBlock<CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessEl>,
    >,
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupBlueGreenDeploymentConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_ready_option: Option<Vec<CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    green_fleet_provisioning_option: Option<
        Vec<CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    terminate_blue_instances_on_deployment_success: Option<
        Vec<CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessEl>,
    >,
    dynamic: CodedeployDeploymentGroupBlueGreenDeploymentConfigElDynamic,
}

impl CodedeployDeploymentGroupBlueGreenDeploymentConfigEl {
    #[doc = "Set the field `deployment_ready_option`.\n"]
    pub fn set_deployment_ready_option(
        mut self,
        v: impl Into<
            BlockAssignable<
                CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.deployment_ready_option = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.deployment_ready_option = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `green_fleet_provisioning_option`.\n"]
    pub fn set_green_fleet_provisioning_option(
        mut self,
        v: impl Into<
            BlockAssignable<
                CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.green_fleet_provisioning_option = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.green_fleet_provisioning_option = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `terminate_blue_instances_on_deployment_success`.\n"]
    pub fn set_terminate_blue_instances_on_deployment_success(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.terminate_blue_instances_on_deployment_success = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.terminate_blue_instances_on_deployment_success = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CodedeployDeploymentGroupBlueGreenDeploymentConfigEl {
    type O = BlockAssignable<CodedeployDeploymentGroupBlueGreenDeploymentConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupBlueGreenDeploymentConfigEl {}

impl BuildCodedeployDeploymentGroupBlueGreenDeploymentConfigEl {
    pub fn build(self) -> CodedeployDeploymentGroupBlueGreenDeploymentConfigEl {
        CodedeployDeploymentGroupBlueGreenDeploymentConfigEl {
            deployment_ready_option: core::default::Default::default(),
            green_fleet_provisioning_option: core::default::Default::default(),
            terminate_blue_instances_on_deployment_success: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupBlueGreenDeploymentConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentGroupBlueGreenDeploymentConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodedeployDeploymentGroupBlueGreenDeploymentConfigElRef {
        CodedeployDeploymentGroupBlueGreenDeploymentConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupBlueGreenDeploymentConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deployment_ready_option` after provisioning.\n"]
    pub fn deployment_ready_option(
        &self,
    ) -> ListRef<CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deployment_ready_option", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `green_fleet_provisioning_option` after provisioning.\n"]
    pub fn green_fleet_provisioning_option(
        &self,
    ) -> ListRef<
        CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.green_fleet_provisioning_option", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `terminate_blue_instances_on_deployment_success` after provisioning.\n"]
    pub fn terminate_blue_instances_on_deployment_success(
        &self,
    ) -> ListRef<CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessElRef>{
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.terminate_blue_instances_on_deployment_success",
                self.base
            ),
        )
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupDeploymentStyleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_option: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_type: Option<PrimField<String>>,
}

impl CodedeployDeploymentGroupDeploymentStyleEl {
    #[doc = "Set the field `deployment_option`.\n"]
    pub fn set_deployment_option(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deployment_option = Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_type`.\n"]
    pub fn set_deployment_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deployment_type = Some(v.into());
        self
    }
}

impl ToListMappable for CodedeployDeploymentGroupDeploymentStyleEl {
    type O = BlockAssignable<CodedeployDeploymentGroupDeploymentStyleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupDeploymentStyleEl {}

impl BuildCodedeployDeploymentGroupDeploymentStyleEl {
    pub fn build(self) -> CodedeployDeploymentGroupDeploymentStyleEl {
        CodedeployDeploymentGroupDeploymentStyleEl {
            deployment_option: core::default::Default::default(),
            deployment_type: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupDeploymentStyleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentGroupDeploymentStyleElRef {
    fn new(shared: StackShared, base: String) -> CodedeployDeploymentGroupDeploymentStyleElRef {
        CodedeployDeploymentGroupDeploymentStyleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupDeploymentStyleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deployment_option` after provisioning.\n"]
    pub fn deployment_option(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_option", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\n"]
    pub fn deployment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_type", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupEc2TagFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl CodedeployDeploymentGroupEc2TagFilterEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for CodedeployDeploymentGroupEc2TagFilterEl {
    type O = BlockAssignable<CodedeployDeploymentGroupEc2TagFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupEc2TagFilterEl {}

impl BuildCodedeployDeploymentGroupEc2TagFilterEl {
    pub fn build(self) -> CodedeployDeploymentGroupEc2TagFilterEl {
        CodedeployDeploymentGroupEc2TagFilterEl {
            key: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupEc2TagFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentGroupEc2TagFilterElRef {
    fn new(shared: StackShared, base: String) -> CodedeployDeploymentGroupEc2TagFilterElRef {
        CodedeployDeploymentGroupEc2TagFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupEc2TagFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
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

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupEc2TagSetElEc2TagFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl CodedeployDeploymentGroupEc2TagSetElEc2TagFilterEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for CodedeployDeploymentGroupEc2TagSetElEc2TagFilterEl {
    type O = BlockAssignable<CodedeployDeploymentGroupEc2TagSetElEc2TagFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupEc2TagSetElEc2TagFilterEl {}

impl BuildCodedeployDeploymentGroupEc2TagSetElEc2TagFilterEl {
    pub fn build(self) -> CodedeployDeploymentGroupEc2TagSetElEc2TagFilterEl {
        CodedeployDeploymentGroupEc2TagSetElEc2TagFilterEl {
            key: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupEc2TagSetElEc2TagFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentGroupEc2TagSetElEc2TagFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodedeployDeploymentGroupEc2TagSetElEc2TagFilterElRef {
        CodedeployDeploymentGroupEc2TagSetElEc2TagFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupEc2TagSetElEc2TagFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
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
struct CodedeployDeploymentGroupEc2TagSetElDynamic {
    ec2_tag_filter: Option<DynamicBlock<CodedeployDeploymentGroupEc2TagSetElEc2TagFilterEl>>,
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupEc2TagSetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ec2_tag_filter: Option<Vec<CodedeployDeploymentGroupEc2TagSetElEc2TagFilterEl>>,
    dynamic: CodedeployDeploymentGroupEc2TagSetElDynamic,
}

impl CodedeployDeploymentGroupEc2TagSetEl {
    #[doc = "Set the field `ec2_tag_filter`.\n"]
    pub fn set_ec2_tag_filter(
        mut self,
        v: impl Into<BlockAssignable<CodedeployDeploymentGroupEc2TagSetElEc2TagFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ec2_tag_filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ec2_tag_filter = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CodedeployDeploymentGroupEc2TagSetEl {
    type O = BlockAssignable<CodedeployDeploymentGroupEc2TagSetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupEc2TagSetEl {}

impl BuildCodedeployDeploymentGroupEc2TagSetEl {
    pub fn build(self) -> CodedeployDeploymentGroupEc2TagSetEl {
        CodedeployDeploymentGroupEc2TagSetEl {
            ec2_tag_filter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupEc2TagSetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentGroupEc2TagSetElRef {
    fn new(shared: StackShared, base: String) -> CodedeployDeploymentGroupEc2TagSetElRef {
        CodedeployDeploymentGroupEc2TagSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupEc2TagSetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupEcsServiceEl {
    cluster_name: PrimField<String>,
    service_name: PrimField<String>,
}

impl CodedeployDeploymentGroupEcsServiceEl {}

impl ToListMappable for CodedeployDeploymentGroupEcsServiceEl {
    type O = BlockAssignable<CodedeployDeploymentGroupEcsServiceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupEcsServiceEl {
    #[doc = ""]
    pub cluster_name: PrimField<String>,
    #[doc = ""]
    pub service_name: PrimField<String>,
}

impl BuildCodedeployDeploymentGroupEcsServiceEl {
    pub fn build(self) -> CodedeployDeploymentGroupEcsServiceEl {
        CodedeployDeploymentGroupEcsServiceEl {
            cluster_name: self.cluster_name,
            service_name: self.service_name,
        }
    }
}

pub struct CodedeployDeploymentGroupEcsServiceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentGroupEcsServiceElRef {
    fn new(shared: StackShared, base: String) -> CodedeployDeploymentGroupEcsServiceElRef {
        CodedeployDeploymentGroupEcsServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupEcsServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.base))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupLoadBalancerInfoElElbInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl CodedeployDeploymentGroupLoadBalancerInfoElElbInfoEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for CodedeployDeploymentGroupLoadBalancerInfoElElbInfoEl {
    type O = BlockAssignable<CodedeployDeploymentGroupLoadBalancerInfoElElbInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupLoadBalancerInfoElElbInfoEl {}

impl BuildCodedeployDeploymentGroupLoadBalancerInfoElElbInfoEl {
    pub fn build(self) -> CodedeployDeploymentGroupLoadBalancerInfoElElbInfoEl {
        CodedeployDeploymentGroupLoadBalancerInfoElElbInfoEl {
            name: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupLoadBalancerInfoElElbInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentGroupLoadBalancerInfoElElbInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodedeployDeploymentGroupLoadBalancerInfoElElbInfoElRef {
        CodedeployDeploymentGroupLoadBalancerInfoElElbInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupLoadBalancerInfoElElbInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoEl {
    type O = BlockAssignable<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoEl {}

impl BuildCodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoEl {
    pub fn build(self) -> CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoEl {
        CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoEl {
            name: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoElRef {
        CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteEl {
    listener_arns: SetField<PrimField<String>>,
}

impl CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteEl {}

impl ToListMappable
    for CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteEl
{
    type O = BlockAssignable<
        CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteEl {
    #[doc = ""]
    pub listener_arns: SetField<PrimField<String>>,
}

impl BuildCodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteEl {
    pub fn build(
        self,
    ) -> CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteEl {
        CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteEl {
            listener_arns: self.listener_arns,
        }
    }
}

pub struct CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteElRef {
        CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `listener_arns` after provisioning.\n"]
    pub fn listener_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.listener_arns", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupEl {
    name: PrimField<String>,
}

impl CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupEl {}

impl ToListMappable
    for CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupEl
{
    type O = BlockAssignable<
        CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildCodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupEl {
    pub fn build(
        self,
    ) -> CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupEl {
        CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupEl {
            name: self.name,
        }
    }
}

pub struct CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupElRef {
        CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteEl {
    listener_arns: SetField<PrimField<String>>,
}

impl CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteEl {}

impl ToListMappable
    for CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteEl
{
    type O = BlockAssignable<
        CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteEl {
    #[doc = ""]
    pub listener_arns: SetField<PrimField<String>>,
}

impl BuildCodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteEl {
    pub fn build(
        self,
    ) -> CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteEl {
        CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteEl {
            listener_arns: self.listener_arns,
        }
    }
}

pub struct CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteElRef {
        CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `listener_arns` after provisioning.\n"]
    pub fn listener_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.listener_arns", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElDynamic {
    prod_traffic_route: Option<
        DynamicBlock<
            CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteEl,
        >,
    >,
    target_group: Option<
        DynamicBlock<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupEl>,
    >,
    test_traffic_route: Option<
        DynamicBlock<
            CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    prod_traffic_route: Option<
        Vec<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_group:
        Option<Vec<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    test_traffic_route: Option<
        Vec<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteEl>,
    >,
    dynamic: CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElDynamic,
}

impl CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoEl {
    #[doc = "Set the field `prod_traffic_route`.\n"]
    pub fn set_prod_traffic_route(
        mut self,
        v: impl Into<
            BlockAssignable<
                CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.prod_traffic_route = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.prod_traffic_route = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `target_group`.\n"]
    pub fn set_target_group(
        mut self,
        v: impl Into<
            BlockAssignable<
                CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target_group = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target_group = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `test_traffic_route`.\n"]
    pub fn set_test_traffic_route(
        mut self,
        v: impl Into<
            BlockAssignable<
                CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.test_traffic_route = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.test_traffic_route = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoEl {
    type O = BlockAssignable<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoEl {}

impl BuildCodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoEl {
    pub fn build(self) -> CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoEl {
        CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoEl {
            prod_traffic_route: core::default::Default::default(),
            target_group: core::default::Default::default(),
            test_traffic_route: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElRef {
        CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prod_traffic_route` after provisioning.\n"]
    pub fn prod_traffic_route(
        &self,
    ) -> ListRef<
        CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.prod_traffic_route", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `target_group` after provisioning.\n"]
    pub fn target_group(
        &self,
    ) -> ListRef<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.target_group", self.base))
    }

    #[doc = "Get a reference to the value of field `test_traffic_route` after provisioning.\n"]
    pub fn test_traffic_route(
        &self,
    ) -> ListRef<
        CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.test_traffic_route", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct CodedeployDeploymentGroupLoadBalancerInfoElDynamic {
    elb_info: Option<DynamicBlock<CodedeployDeploymentGroupLoadBalancerInfoElElbInfoEl>>,
    target_group_info:
        Option<DynamicBlock<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoEl>>,
    target_group_pair_info:
        Option<DynamicBlock<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoEl>>,
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupLoadBalancerInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    elb_info: Option<Vec<CodedeployDeploymentGroupLoadBalancerInfoElElbInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_group_info: Option<Vec<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_group_pair_info:
        Option<Vec<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoEl>>,
    dynamic: CodedeployDeploymentGroupLoadBalancerInfoElDynamic,
}

impl CodedeployDeploymentGroupLoadBalancerInfoEl {
    #[doc = "Set the field `elb_info`.\n"]
    pub fn set_elb_info(
        mut self,
        v: impl Into<BlockAssignable<CodedeployDeploymentGroupLoadBalancerInfoElElbInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.elb_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.elb_info = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `target_group_info`.\n"]
    pub fn set_target_group_info(
        mut self,
        v: impl Into<BlockAssignable<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target_group_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target_group_info = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `target_group_pair_info`.\n"]
    pub fn set_target_group_pair_info(
        mut self,
        v: impl Into<BlockAssignable<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target_group_pair_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target_group_pair_info = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CodedeployDeploymentGroupLoadBalancerInfoEl {
    type O = BlockAssignable<CodedeployDeploymentGroupLoadBalancerInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupLoadBalancerInfoEl {}

impl BuildCodedeployDeploymentGroupLoadBalancerInfoEl {
    pub fn build(self) -> CodedeployDeploymentGroupLoadBalancerInfoEl {
        CodedeployDeploymentGroupLoadBalancerInfoEl {
            elb_info: core::default::Default::default(),
            target_group_info: core::default::Default::default(),
            target_group_pair_info: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupLoadBalancerInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentGroupLoadBalancerInfoElRef {
    fn new(shared: StackShared, base: String) -> CodedeployDeploymentGroupLoadBalancerInfoElRef {
        CodedeployDeploymentGroupLoadBalancerInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupLoadBalancerInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_group_pair_info` after provisioning.\n"]
    pub fn target_group_pair_info(
        &self,
    ) -> ListRef<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_group_pair_info", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupOnPremisesInstanceTagFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl CodedeployDeploymentGroupOnPremisesInstanceTagFilterEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for CodedeployDeploymentGroupOnPremisesInstanceTagFilterEl {
    type O = BlockAssignable<CodedeployDeploymentGroupOnPremisesInstanceTagFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupOnPremisesInstanceTagFilterEl {}

impl BuildCodedeployDeploymentGroupOnPremisesInstanceTagFilterEl {
    pub fn build(self) -> CodedeployDeploymentGroupOnPremisesInstanceTagFilterEl {
        CodedeployDeploymentGroupOnPremisesInstanceTagFilterEl {
            key: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupOnPremisesInstanceTagFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentGroupOnPremisesInstanceTagFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodedeployDeploymentGroupOnPremisesInstanceTagFilterElRef {
        CodedeployDeploymentGroupOnPremisesInstanceTagFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupOnPremisesInstanceTagFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
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

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupTriggerConfigurationEl {
    trigger_events: SetField<PrimField<String>>,
    trigger_name: PrimField<String>,
    trigger_target_arn: PrimField<String>,
}

impl CodedeployDeploymentGroupTriggerConfigurationEl {}

impl ToListMappable for CodedeployDeploymentGroupTriggerConfigurationEl {
    type O = BlockAssignable<CodedeployDeploymentGroupTriggerConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupTriggerConfigurationEl {
    #[doc = ""]
    pub trigger_events: SetField<PrimField<String>>,
    #[doc = ""]
    pub trigger_name: PrimField<String>,
    #[doc = ""]
    pub trigger_target_arn: PrimField<String>,
}

impl BuildCodedeployDeploymentGroupTriggerConfigurationEl {
    pub fn build(self) -> CodedeployDeploymentGroupTriggerConfigurationEl {
        CodedeployDeploymentGroupTriggerConfigurationEl {
            trigger_events: self.trigger_events,
            trigger_name: self.trigger_name,
            trigger_target_arn: self.trigger_target_arn,
        }
    }
}

pub struct CodedeployDeploymentGroupTriggerConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentGroupTriggerConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodedeployDeploymentGroupTriggerConfigurationElRef {
        CodedeployDeploymentGroupTriggerConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupTriggerConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `trigger_events` after provisioning.\n"]
    pub fn trigger_events(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.trigger_events", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `trigger_name` after provisioning.\n"]
    pub fn trigger_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trigger_name", self.base))
    }

    #[doc = "Get a reference to the value of field `trigger_target_arn` after provisioning.\n"]
    pub fn trigger_target_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trigger_target_arn", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct CodedeployDeploymentGroupDynamic {
    alarm_configuration: Option<DynamicBlock<CodedeployDeploymentGroupAlarmConfigurationEl>>,
    auto_rollback_configuration:
        Option<DynamicBlock<CodedeployDeploymentGroupAutoRollbackConfigurationEl>>,
    blue_green_deployment_config:
        Option<DynamicBlock<CodedeployDeploymentGroupBlueGreenDeploymentConfigEl>>,
    deployment_style: Option<DynamicBlock<CodedeployDeploymentGroupDeploymentStyleEl>>,
    ec2_tag_filter: Option<DynamicBlock<CodedeployDeploymentGroupEc2TagFilterEl>>,
    ec2_tag_set: Option<DynamicBlock<CodedeployDeploymentGroupEc2TagSetEl>>,
    ecs_service: Option<DynamicBlock<CodedeployDeploymentGroupEcsServiceEl>>,
    load_balancer_info: Option<DynamicBlock<CodedeployDeploymentGroupLoadBalancerInfoEl>>,
    on_premises_instance_tag_filter:
        Option<DynamicBlock<CodedeployDeploymentGroupOnPremisesInstanceTagFilterEl>>,
    trigger_configuration: Option<DynamicBlock<CodedeployDeploymentGroupTriggerConfigurationEl>>,
}
