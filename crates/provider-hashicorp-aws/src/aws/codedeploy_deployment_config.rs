use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CodedeployDeploymentConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_platform: Option<PrimField<String>>,
    deployment_config_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_healthy_hosts: Option<Vec<CodedeployDeploymentConfigMinimumHealthyHostsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    traffic_routing_config: Option<Vec<CodedeployDeploymentConfigTrafficRoutingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zonal_config: Option<Vec<CodedeployDeploymentConfigZonalConfigEl>>,
    dynamic: CodedeployDeploymentConfigDynamic,
}

struct CodedeployDeploymentConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CodedeployDeploymentConfigData>,
}

#[derive(Clone)]
pub struct CodedeployDeploymentConfig(Rc<CodedeployDeploymentConfig_>);

impl CodedeployDeploymentConfig {
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

    #[doc = "Set the field `compute_platform`.\n"]
    pub fn set_compute_platform(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().compute_platform = Some(v.into());
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

    #[doc = "Set the field `minimum_healthy_hosts`.\n"]
    pub fn set_minimum_healthy_hosts(
        self,
        v: impl Into<BlockAssignable<CodedeployDeploymentConfigMinimumHealthyHostsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().minimum_healthy_hosts = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.minimum_healthy_hosts = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `traffic_routing_config`.\n"]
    pub fn set_traffic_routing_config(
        self,
        v: impl Into<BlockAssignable<CodedeployDeploymentConfigTrafficRoutingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().traffic_routing_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.traffic_routing_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `zonal_config`.\n"]
    pub fn set_zonal_config(self, v: impl Into<BlockAssignable<CodedeployDeploymentConfigZonalConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().zonal_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.zonal_config = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_platform` after provisioning.\n"]
    pub fn compute_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compute_platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_config_id` after provisioning.\n"]
    pub fn deployment_config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment_config_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_config_name` after provisioning.\n"]
    pub fn deployment_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment_config_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `minimum_healthy_hosts` after provisioning.\n"]
    pub fn minimum_healthy_hosts(&self) -> ListRef<CodedeployDeploymentConfigMinimumHealthyHostsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.minimum_healthy_hosts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic_routing_config` after provisioning.\n"]
    pub fn traffic_routing_config(&self) -> ListRef<CodedeployDeploymentConfigTrafficRoutingConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.traffic_routing_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zonal_config` after provisioning.\n"]
    pub fn zonal_config(&self) -> ListRef<CodedeployDeploymentConfigZonalConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.zonal_config", self.extract_ref()))
    }
}

impl Referable for CodedeployDeploymentConfig {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for CodedeployDeploymentConfig { }

impl ToListMappable for CodedeployDeploymentConfig {
    type O = ListRef<CodedeployDeploymentConfigRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CodedeployDeploymentConfig_ {
    fn extract_resource_type(&self) -> String {
        "aws_codedeploy_deployment_config".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCodedeployDeploymentConfig {
    pub tf_id: String,
    #[doc = ""]
    pub deployment_config_name: PrimField<String>,
}

impl BuildCodedeployDeploymentConfig {
    pub fn build(self, stack: &mut Stack) -> CodedeployDeploymentConfig {
        let out = CodedeployDeploymentConfig(Rc::new(CodedeployDeploymentConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CodedeployDeploymentConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                compute_platform: core::default::Default::default(),
                deployment_config_name: self.deployment_config_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                minimum_healthy_hosts: core::default::Default::default(),
                traffic_routing_config: core::default::Default::default(),
                zonal_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CodedeployDeploymentConfigRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl CodedeployDeploymentConfigRef {
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

    #[doc = "Get a reference to the value of field `compute_platform` after provisioning.\n"]
    pub fn compute_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compute_platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_config_id` after provisioning.\n"]
    pub fn deployment_config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment_config_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_config_name` after provisioning.\n"]
    pub fn deployment_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment_config_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `minimum_healthy_hosts` after provisioning.\n"]
    pub fn minimum_healthy_hosts(&self) -> ListRef<CodedeployDeploymentConfigMinimumHealthyHostsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.minimum_healthy_hosts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic_routing_config` after provisioning.\n"]
    pub fn traffic_routing_config(&self) -> ListRef<CodedeployDeploymentConfigTrafficRoutingConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.traffic_routing_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zonal_config` after provisioning.\n"]
    pub fn zonal_config(&self) -> ListRef<CodedeployDeploymentConfigZonalConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.zonal_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentConfigMinimumHealthyHostsEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl CodedeployDeploymentConfigMinimumHealthyHostsEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for CodedeployDeploymentConfigMinimumHealthyHostsEl {
    type O = BlockAssignable<CodedeployDeploymentConfigMinimumHealthyHostsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentConfigMinimumHealthyHostsEl {}

impl BuildCodedeployDeploymentConfigMinimumHealthyHostsEl {
    pub fn build(self) -> CodedeployDeploymentConfigMinimumHealthyHostsEl {
        CodedeployDeploymentConfigMinimumHealthyHostsEl {
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentConfigMinimumHealthyHostsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentConfigMinimumHealthyHostsElRef {
    fn new(shared: StackShared, base: String) -> CodedeployDeploymentConfigMinimumHealthyHostsElRef {
        CodedeployDeploymentConfigMinimumHealthyHostsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentConfigMinimumHealthyHostsElRef {
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
pub struct CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percentage: Option<PrimField<f64>>,
}

impl CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryEl {
    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.interval = Some(v.into());
        self
    }

    #[doc = "Set the field `percentage`.\n"]
    pub fn set_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percentage = Some(v.into());
        self
    }
}

impl ToListMappable for CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryEl {
    type O = BlockAssignable<CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryEl {}

impl BuildCodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryEl {
    pub fn build(self) -> CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryEl {
        CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryEl {
            interval: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryElRef {
        CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `percentage` after provisioning.\n"]
    pub fn percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percentage: Option<PrimField<f64>>,
}

impl CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearEl {
    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.interval = Some(v.into());
        self
    }

    #[doc = "Set the field `percentage`.\n"]
    pub fn set_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percentage = Some(v.into());
        self
    }
}

impl ToListMappable for CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearEl {
    type O = BlockAssignable<CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearEl {}

impl BuildCodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearEl {
    pub fn build(self) -> CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearEl {
        CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearEl {
            interval: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearElRef {
        CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `percentage` after provisioning.\n"]
    pub fn percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodedeployDeploymentConfigTrafficRoutingConfigElDynamic {
    time_based_canary: Option<DynamicBlock<CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryEl>>,
    time_based_linear: Option<DynamicBlock<CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearEl>>,
}

#[derive(Serialize)]
pub struct CodedeployDeploymentConfigTrafficRoutingConfigEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_based_canary: Option<Vec<CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_based_linear: Option<Vec<CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearEl>>,
    dynamic: CodedeployDeploymentConfigTrafficRoutingConfigElDynamic,
}

impl CodedeployDeploymentConfigTrafficRoutingConfigEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `time_based_canary`.\n"]
    pub fn set_time_based_canary(
        mut self,
        v: impl Into<BlockAssignable<CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.time_based_canary = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.time_based_canary = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `time_based_linear`.\n"]
    pub fn set_time_based_linear(
        mut self,
        v: impl Into<BlockAssignable<CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.time_based_linear = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.time_based_linear = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CodedeployDeploymentConfigTrafficRoutingConfigEl {
    type O = BlockAssignable<CodedeployDeploymentConfigTrafficRoutingConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentConfigTrafficRoutingConfigEl {}

impl BuildCodedeployDeploymentConfigTrafficRoutingConfigEl {
    pub fn build(self) -> CodedeployDeploymentConfigTrafficRoutingConfigEl {
        CodedeployDeploymentConfigTrafficRoutingConfigEl {
            type_: core::default::Default::default(),
            time_based_canary: core::default::Default::default(),
            time_based_linear: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodedeployDeploymentConfigTrafficRoutingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentConfigTrafficRoutingConfigElRef {
    fn new(shared: StackShared, base: String) -> CodedeployDeploymentConfigTrafficRoutingConfigElRef {
        CodedeployDeploymentConfigTrafficRoutingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentConfigTrafficRoutingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `time_based_canary` after provisioning.\n"]
    pub fn time_based_canary(&self) -> ListRef<CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.time_based_canary", self.base))
    }

    #[doc = "Get a reference to the value of field `time_based_linear` after provisioning.\n"]
    pub fn time_based_linear(&self) -> ListRef<CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearElRef> {
        ListRef::new(self.shared().clone(), format!("{}.time_based_linear", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentConfigZonalConfigElMinimumHealthyHostsPerZoneEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl CodedeployDeploymentConfigZonalConfigElMinimumHealthyHostsPerZoneEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for CodedeployDeploymentConfigZonalConfigElMinimumHealthyHostsPerZoneEl {
    type O = BlockAssignable<CodedeployDeploymentConfigZonalConfigElMinimumHealthyHostsPerZoneEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentConfigZonalConfigElMinimumHealthyHostsPerZoneEl {}

impl BuildCodedeployDeploymentConfigZonalConfigElMinimumHealthyHostsPerZoneEl {
    pub fn build(self) -> CodedeployDeploymentConfigZonalConfigElMinimumHealthyHostsPerZoneEl {
        CodedeployDeploymentConfigZonalConfigElMinimumHealthyHostsPerZoneEl {
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentConfigZonalConfigElMinimumHealthyHostsPerZoneElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentConfigZonalConfigElMinimumHealthyHostsPerZoneElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodedeployDeploymentConfigZonalConfigElMinimumHealthyHostsPerZoneElRef {
        CodedeployDeploymentConfigZonalConfigElMinimumHealthyHostsPerZoneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentConfigZonalConfigElMinimumHealthyHostsPerZoneElRef {
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
struct CodedeployDeploymentConfigZonalConfigElDynamic {
    minimum_healthy_hosts_per_zone: Option<
        DynamicBlock<CodedeployDeploymentConfigZonalConfigElMinimumHealthyHostsPerZoneEl>,
    >,
}

#[derive(Serialize)]
pub struct CodedeployDeploymentConfigZonalConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    first_zone_monitor_duration_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitor_duration_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_healthy_hosts_per_zone: Option<Vec<CodedeployDeploymentConfigZonalConfigElMinimumHealthyHostsPerZoneEl>>,
    dynamic: CodedeployDeploymentConfigZonalConfigElDynamic,
}

impl CodedeployDeploymentConfigZonalConfigEl {
    #[doc = "Set the field `first_zone_monitor_duration_in_seconds`.\n"]
    pub fn set_first_zone_monitor_duration_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.first_zone_monitor_duration_in_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `monitor_duration_in_seconds`.\n"]
    pub fn set_monitor_duration_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.monitor_duration_in_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `minimum_healthy_hosts_per_zone`.\n"]
    pub fn set_minimum_healthy_hosts_per_zone(
        mut self,
        v: impl Into<BlockAssignable<CodedeployDeploymentConfigZonalConfigElMinimumHealthyHostsPerZoneEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.minimum_healthy_hosts_per_zone = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.minimum_healthy_hosts_per_zone = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CodedeployDeploymentConfigZonalConfigEl {
    type O = BlockAssignable<CodedeployDeploymentConfigZonalConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentConfigZonalConfigEl {}

impl BuildCodedeployDeploymentConfigZonalConfigEl {
    pub fn build(self) -> CodedeployDeploymentConfigZonalConfigEl {
        CodedeployDeploymentConfigZonalConfigEl {
            first_zone_monitor_duration_in_seconds: core::default::Default::default(),
            monitor_duration_in_seconds: core::default::Default::default(),
            minimum_healthy_hosts_per_zone: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodedeployDeploymentConfigZonalConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodedeployDeploymentConfigZonalConfigElRef {
    fn new(shared: StackShared, base: String) -> CodedeployDeploymentConfigZonalConfigElRef {
        CodedeployDeploymentConfigZonalConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentConfigZonalConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `first_zone_monitor_duration_in_seconds` after provisioning.\n"]
    pub fn first_zone_monitor_duration_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.first_zone_monitor_duration_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `monitor_duration_in_seconds` after provisioning.\n"]
    pub fn monitor_duration_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitor_duration_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `minimum_healthy_hosts_per_zone` after provisioning.\n"]
    pub fn minimum_healthy_hosts_per_zone(
        &self,
    ) -> ListRef<CodedeployDeploymentConfigZonalConfigElMinimumHealthyHostsPerZoneElRef> {
        ListRef::new(self.shared().clone(), format!("{}.minimum_healthy_hosts_per_zone", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodedeployDeploymentConfigDynamic {
    minimum_healthy_hosts: Option<DynamicBlock<CodedeployDeploymentConfigMinimumHealthyHostsEl>>,
    traffic_routing_config: Option<DynamicBlock<CodedeployDeploymentConfigTrafficRoutingConfigEl>>,
    zonal_config: Option<DynamicBlock<CodedeployDeploymentConfigZonalConfigEl>>,
}
