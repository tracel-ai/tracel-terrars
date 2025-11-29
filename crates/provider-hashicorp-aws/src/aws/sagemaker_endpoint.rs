use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerEndpointData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    endpoint_config_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_config: Option<Vec<SagemakerEndpointDeploymentConfigEl>>,
    dynamic: SagemakerEndpointDynamic,
}

struct SagemakerEndpoint_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SagemakerEndpointData>,
}

#[derive(Clone)]
pub struct SagemakerEndpoint(Rc<SagemakerEndpoint_>);

impl SagemakerEndpoint {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
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

    #[doc = "Set the field `deployment_config`.\n"]
    pub fn set_deployment_config(self, v: impl Into<BlockAssignable<SagemakerEndpointDeploymentConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deployment_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deployment_config = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_config_name` after provisioning.\n"]
    pub fn endpoint_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_config_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_config` after provisioning.\n"]
    pub fn deployment_config(&self) -> ListRef<SagemakerEndpointDeploymentConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deployment_config", self.extract_ref()))
    }
}

impl Referable for SagemakerEndpoint {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for SagemakerEndpoint { }

impl ToListMappable for SagemakerEndpoint {
    type O = ListRef<SagemakerEndpointRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SagemakerEndpoint_ {
    fn extract_resource_type(&self) -> String {
        "aws_sagemaker_endpoint".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerEndpoint {
    pub tf_id: String,
    #[doc = ""]
    pub endpoint_config_name: PrimField<String>,
}

impl BuildSagemakerEndpoint {
    pub fn build(self, stack: &mut Stack) -> SagemakerEndpoint {
        let out = SagemakerEndpoint(Rc::new(SagemakerEndpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SagemakerEndpointData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                endpoint_config_name: self.endpoint_config_name,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                deployment_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerEndpointRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl SagemakerEndpointRef {
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

    #[doc = "Get a reference to the value of field `endpoint_config_name` after provisioning.\n"]
    pub fn endpoint_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_config_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_config` after provisioning.\n"]
    pub fn deployment_config(&self) -> ListRef<SagemakerEndpointDeploymentConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deployment_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsEl {
    alarm_name: PrimField<String>,
}

impl SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsEl { }

impl ToListMappable for SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsEl {
    type O = BlockAssignable<SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsEl {
    #[doc = ""]
    pub alarm_name: PrimField<String>,
}

impl BuildSagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsEl {
    pub fn build(self) -> SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsEl {
        SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsEl { alarm_name: self.alarm_name }
    }
}

pub struct SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsElRef {
        SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alarm_name` after provisioning.\n"]
    pub fn alarm_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alarm_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElDynamic {
    alarms: Option<DynamicBlock<SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsEl>>,
}

#[derive(Serialize)]
pub struct SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    alarms: Option<Vec<SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsEl>>,
    dynamic: SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElDynamic,
}

impl SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationEl {
    #[doc = "Set the field `alarms`.\n"]
    pub fn set_alarms(
        mut self,
        v: impl Into<BlockAssignable<SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.alarms = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.alarms = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationEl {
    type O = BlockAssignable<SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointDeploymentConfigElAutoRollbackConfigurationEl {}

impl BuildSagemakerEndpointDeploymentConfigElAutoRollbackConfigurationEl {
    pub fn build(self) -> SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationEl {
        SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationEl {
            alarms: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElRef {
    fn new(shared: StackShared, base: String) -> SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElRef {
        SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    value: PrimField<f64>,
}

impl SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeEl { }

impl ToListMappable for SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeEl {
    type O =
        BlockAssignable<
            SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeEl {
    #[doc = ""]
    pub type_: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildSagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeEl {
    pub fn build(
        self,
    ) -> SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeEl {
        SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeEl {
            type_: self.type_,
            value: self.value,
        }
    }
}

pub struct SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeElRef {
        SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    value: PrimField<f64>,
}

impl SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeEl { }

impl ToListMappable for SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeEl {
    type O =
        BlockAssignable<
            SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeEl {
    #[doc = ""]
    pub type_: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildSagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeEl {
    pub fn build(
        self,
    ) -> SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeEl {
        SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeEl {
            type_: self.type_,
            value: self.value,
        }
    }
}

pub struct SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeElRef {
        SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElDynamic {
    canary_size: Option<
        DynamicBlock<
            SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeEl,
        >,
    >,
    linear_step_size: Option<
        DynamicBlock<
            SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    wait_interval_in_seconds: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    canary_size: Option<
        Vec<SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    linear_step_size: Option<
        Vec<SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeEl>,
    >,
    dynamic: SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElDynamic,
}

impl SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationEl {
    #[doc = "Set the field `canary_size`.\n"]
    pub fn set_canary_size(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.canary_size = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.canary_size = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `linear_step_size`.\n"]
    pub fn set_linear_step_size(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.linear_step_size = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.linear_step_size = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationEl {
    type O =
        BlockAssignable<SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationEl {
    #[doc = ""]
    pub type_: PrimField<String>,
    #[doc = ""]
    pub wait_interval_in_seconds: PrimField<f64>,
}

impl BuildSagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationEl {
    pub fn build(self) -> SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationEl {
        SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationEl {
            type_: self.type_,
            wait_interval_in_seconds: self.wait_interval_in_seconds,
            canary_size: core::default::Default::default(),
            linear_step_size: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElRef {
        SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `wait_interval_in_seconds` after provisioning.\n"]
    pub fn wait_interval_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait_interval_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `canary_size` after provisioning.\n"]
    pub fn canary_size(
        &self,
    ) -> ListRef<
        SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.canary_size", self.base))
    }

    #[doc = "Get a reference to the value of field `linear_step_size` after provisioning.\n"]
    pub fn linear_step_size(
        &self,
    ) -> ListRef<
        SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.linear_step_size", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElDynamic {
    traffic_routing_configuration: Option<
        DynamicBlock<SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_execution_timeout_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    termination_wait_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    traffic_routing_configuration: Option<
        Vec<SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationEl>,
    >,
    dynamic: SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElDynamic,
}

impl SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyEl {
    #[doc = "Set the field `maximum_execution_timeout_in_seconds`.\n"]
    pub fn set_maximum_execution_timeout_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_execution_timeout_in_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `termination_wait_in_seconds`.\n"]
    pub fn set_termination_wait_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.termination_wait_in_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `traffic_routing_configuration`.\n"]
    pub fn set_traffic_routing_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.traffic_routing_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.traffic_routing_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyEl {
    type O = BlockAssignable<SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyEl {}

impl BuildSagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyEl {
    pub fn build(self) -> SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyEl {
        SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyEl {
            maximum_execution_timeout_in_seconds: core::default::Default::default(),
            termination_wait_in_seconds: core::default::Default::default(),
            traffic_routing_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElRef {
    fn new(shared: StackShared, base: String) -> SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElRef {
        SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum_execution_timeout_in_seconds` after provisioning.\n"]
    pub fn maximum_execution_timeout_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_execution_timeout_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `termination_wait_in_seconds` after provisioning.\n"]
    pub fn termination_wait_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.termination_wait_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `traffic_routing_configuration` after provisioning.\n"]
    pub fn traffic_routing_configuration(
        &self,
    ) -> ListRef<SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.traffic_routing_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElMaximumBatchSizeEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    value: PrimField<f64>,
}

impl SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElMaximumBatchSizeEl { }

impl ToListMappable for SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElMaximumBatchSizeEl {
    type O = BlockAssignable<SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElMaximumBatchSizeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointDeploymentConfigElRollingUpdatePolicyElMaximumBatchSizeEl {
    #[doc = ""]
    pub type_: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildSagemakerEndpointDeploymentConfigElRollingUpdatePolicyElMaximumBatchSizeEl {
    pub fn build(self) -> SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElMaximumBatchSizeEl {
        SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElMaximumBatchSizeEl {
            type_: self.type_,
            value: self.value,
        }
    }
}

pub struct SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElMaximumBatchSizeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElMaximumBatchSizeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElMaximumBatchSizeElRef {
        SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElMaximumBatchSizeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElMaximumBatchSizeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElRollbackMaximumBatchSizeEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    value: PrimField<f64>,
}

impl SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElRollbackMaximumBatchSizeEl { }

impl ToListMappable for SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElRollbackMaximumBatchSizeEl {
    type O = BlockAssignable<SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElRollbackMaximumBatchSizeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointDeploymentConfigElRollingUpdatePolicyElRollbackMaximumBatchSizeEl {
    #[doc = ""]
    pub type_: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildSagemakerEndpointDeploymentConfigElRollingUpdatePolicyElRollbackMaximumBatchSizeEl {
    pub fn build(self) -> SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElRollbackMaximumBatchSizeEl {
        SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElRollbackMaximumBatchSizeEl {
            type_: self.type_,
            value: self.value,
        }
    }
}

pub struct SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElRollbackMaximumBatchSizeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElRollbackMaximumBatchSizeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElRollbackMaximumBatchSizeElRef {
        SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElRollbackMaximumBatchSizeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElRollbackMaximumBatchSizeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElDynamic {
    maximum_batch_size: Option<
        DynamicBlock<SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElMaximumBatchSizeEl>,
    >,
    rollback_maximum_batch_size: Option<
        DynamicBlock<SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElRollbackMaximumBatchSizeEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerEndpointDeploymentConfigElRollingUpdatePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_execution_timeout_in_seconds: Option<PrimField<f64>>,
    wait_interval_in_seconds: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_batch_size: Option<Vec<SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElMaximumBatchSizeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rollback_maximum_batch_size: Option<
        Vec<SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElRollbackMaximumBatchSizeEl>,
    >,
    dynamic: SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElDynamic,
}

impl SagemakerEndpointDeploymentConfigElRollingUpdatePolicyEl {
    #[doc = "Set the field `maximum_execution_timeout_in_seconds`.\n"]
    pub fn set_maximum_execution_timeout_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_execution_timeout_in_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_batch_size`.\n"]
    pub fn set_maximum_batch_size(
        mut self,
        v: impl Into<BlockAssignable<SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElMaximumBatchSizeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.maximum_batch_size = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.maximum_batch_size = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rollback_maximum_batch_size`.\n"]
    pub fn set_rollback_maximum_batch_size(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElRollbackMaximumBatchSizeEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rollback_maximum_batch_size = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rollback_maximum_batch_size = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerEndpointDeploymentConfigElRollingUpdatePolicyEl {
    type O = BlockAssignable<SagemakerEndpointDeploymentConfigElRollingUpdatePolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointDeploymentConfigElRollingUpdatePolicyEl {
    #[doc = ""]
    pub wait_interval_in_seconds: PrimField<f64>,
}

impl BuildSagemakerEndpointDeploymentConfigElRollingUpdatePolicyEl {
    pub fn build(self) -> SagemakerEndpointDeploymentConfigElRollingUpdatePolicyEl {
        SagemakerEndpointDeploymentConfigElRollingUpdatePolicyEl {
            maximum_execution_timeout_in_seconds: core::default::Default::default(),
            wait_interval_in_seconds: self.wait_interval_in_seconds,
            maximum_batch_size: core::default::Default::default(),
            rollback_maximum_batch_size: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElRef {
    fn new(shared: StackShared, base: String) -> SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElRef {
        SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum_execution_timeout_in_seconds` after provisioning.\n"]
    pub fn maximum_execution_timeout_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_execution_timeout_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `wait_interval_in_seconds` after provisioning.\n"]
    pub fn wait_interval_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait_interval_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_batch_size` after provisioning.\n"]
    pub fn maximum_batch_size(
        &self,
    ) -> ListRef<SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElMaximumBatchSizeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.maximum_batch_size", self.base))
    }

    #[doc = "Get a reference to the value of field `rollback_maximum_batch_size` after provisioning.\n"]
    pub fn rollback_maximum_batch_size(
        &self,
    ) -> ListRef<SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElRollbackMaximumBatchSizeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rollback_maximum_batch_size", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpointDeploymentConfigElDynamic {
    auto_rollback_configuration: Option<DynamicBlock<SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationEl>>,
    blue_green_update_policy: Option<DynamicBlock<SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyEl>>,
    rolling_update_policy: Option<DynamicBlock<SagemakerEndpointDeploymentConfigElRollingUpdatePolicyEl>>,
}

#[derive(Serialize)]
pub struct SagemakerEndpointDeploymentConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_rollback_configuration: Option<Vec<SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blue_green_update_policy: Option<Vec<SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rolling_update_policy: Option<Vec<SagemakerEndpointDeploymentConfigElRollingUpdatePolicyEl>>,
    dynamic: SagemakerEndpointDeploymentConfigElDynamic,
}

impl SagemakerEndpointDeploymentConfigEl {
    #[doc = "Set the field `auto_rollback_configuration`.\n"]
    pub fn set_auto_rollback_configuration(
        mut self,
        v: impl Into<BlockAssignable<SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.auto_rollback_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.auto_rollback_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `blue_green_update_policy`.\n"]
    pub fn set_blue_green_update_policy(
        mut self,
        v: impl Into<BlockAssignable<SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.blue_green_update_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.blue_green_update_policy = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rolling_update_policy`.\n"]
    pub fn set_rolling_update_policy(
        mut self,
        v: impl Into<BlockAssignable<SagemakerEndpointDeploymentConfigElRollingUpdatePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rolling_update_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rolling_update_policy = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerEndpointDeploymentConfigEl {
    type O = BlockAssignable<SagemakerEndpointDeploymentConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointDeploymentConfigEl {}

impl BuildSagemakerEndpointDeploymentConfigEl {
    pub fn build(self) -> SagemakerEndpointDeploymentConfigEl {
        SagemakerEndpointDeploymentConfigEl {
            auto_rollback_configuration: core::default::Default::default(),
            blue_green_update_policy: core::default::Default::default(),
            rolling_update_policy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerEndpointDeploymentConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerEndpointDeploymentConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerEndpointDeploymentConfigElRef {
        SagemakerEndpointDeploymentConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointDeploymentConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_rollback_configuration` after provisioning.\n"]
    pub fn auto_rollback_configuration(
        &self,
    ) -> ListRef<SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto_rollback_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `blue_green_update_policy` after provisioning.\n"]
    pub fn blue_green_update_policy(&self) -> ListRef<SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.blue_green_update_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `rolling_update_policy` after provisioning.\n"]
    pub fn rolling_update_policy(&self) -> ListRef<SagemakerEndpointDeploymentConfigElRollingUpdatePolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rolling_update_policy", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpointDynamic {
    deployment_config: Option<DynamicBlock<SagemakerEndpointDeploymentConfigEl>>,
}
