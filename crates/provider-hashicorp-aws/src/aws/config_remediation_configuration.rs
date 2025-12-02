use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ConfigRemediationConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic: Option<PrimField<bool>>,
    config_rule_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_automatic_attempts: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_attempt_seconds: Option<PrimField<f64>>,
    target_id: PrimField<String>,
    target_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_controls: Option<Vec<ConfigRemediationConfigurationExecutionControlsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<Vec<ConfigRemediationConfigurationParameterEl>>,
    dynamic: ConfigRemediationConfigurationDynamic,
}
struct ConfigRemediationConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ConfigRemediationConfigurationData>,
}
#[derive(Clone)]
pub struct ConfigRemediationConfiguration(Rc<ConfigRemediationConfiguration_>);
impl ConfigRemediationConfiguration {
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
    #[doc = "Set the field `automatic`.\n"]
    pub fn set_automatic(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().automatic = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `maximum_automatic_attempts`.\n"]
    pub fn set_maximum_automatic_attempts(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().maximum_automatic_attempts = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().resource_type = Some(v.into());
        self
    }
    #[doc = "Set the field `retry_attempt_seconds`.\n"]
    pub fn set_retry_attempt_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().retry_attempt_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `target_version`.\n"]
    pub fn set_target_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().target_version = Some(v.into());
        self
    }
    #[doc = "Set the field `execution_controls`.\n"]
    pub fn set_execution_controls(
        self,
        v: impl Into<BlockAssignable<ConfigRemediationConfigurationExecutionControlsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().execution_controls = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.execution_controls = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        self,
        v: impl Into<BlockAssignable<ConfigRemediationConfigurationParameterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parameter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parameter = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `automatic` after provisioning.\n"]
    pub fn automatic(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.automatic", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config_rule_name` after provisioning.\n"]
    pub fn config_rule_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config_rule_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `maximum_automatic_attempts` after provisioning.\n"]
    pub fn maximum_automatic_attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_automatic_attempts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retry_attempt_seconds` after provisioning.\n"]
    pub fn retry_attempt_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retry_attempt_seconds", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_id` after provisioning.\n"]
    pub fn target_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_type` after provisioning.\n"]
    pub fn target_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_version` after provisioning.\n"]
    pub fn target_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_controls` after provisioning.\n"]
    pub fn execution_controls(
        &self,
    ) -> ListRef<ConfigRemediationConfigurationExecutionControlsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.execution_controls", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parameter` after provisioning.\n"]
    pub fn parameter(&self) -> ListRef<ConfigRemediationConfigurationParameterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.parameter", self.extract_ref()),
        )
    }
}
impl Referable for ConfigRemediationConfiguration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ConfigRemediationConfiguration {}
impl ToListMappable for ConfigRemediationConfiguration {
    type O = ListRef<ConfigRemediationConfigurationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ConfigRemediationConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_config_remediation_configuration".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildConfigRemediationConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub config_rule_name: PrimField<String>,
    #[doc = ""]
    pub target_id: PrimField<String>,
    #[doc = ""]
    pub target_type: PrimField<String>,
}
impl BuildConfigRemediationConfiguration {
    pub fn build(self, stack: &mut Stack) -> ConfigRemediationConfiguration {
        let out = ConfigRemediationConfiguration(Rc::new(ConfigRemediationConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ConfigRemediationConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                automatic: core::default::Default::default(),
                config_rule_name: self.config_rule_name,
                id: core::default::Default::default(),
                maximum_automatic_attempts: core::default::Default::default(),
                region: core::default::Default::default(),
                resource_type: core::default::Default::default(),
                retry_attempt_seconds: core::default::Default::default(),
                target_id: self.target_id,
                target_type: self.target_type,
                target_version: core::default::Default::default(),
                execution_controls: core::default::Default::default(),
                parameter: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ConfigRemediationConfigurationRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConfigRemediationConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ConfigRemediationConfigurationRef {
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
    #[doc = "Get a reference to the value of field `automatic` after provisioning.\n"]
    pub fn automatic(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.automatic", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config_rule_name` after provisioning.\n"]
    pub fn config_rule_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config_rule_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `maximum_automatic_attempts` after provisioning.\n"]
    pub fn maximum_automatic_attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_automatic_attempts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retry_attempt_seconds` after provisioning.\n"]
    pub fn retry_attempt_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retry_attempt_seconds", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_id` after provisioning.\n"]
    pub fn target_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_type` after provisioning.\n"]
    pub fn target_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_version` after provisioning.\n"]
    pub fn target_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_controls` after provisioning.\n"]
    pub fn execution_controls(
        &self,
    ) -> ListRef<ConfigRemediationConfigurationExecutionControlsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.execution_controls", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parameter` after provisioning.\n"]
    pub fn parameter(&self) -> ListRef<ConfigRemediationConfigurationParameterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.parameter", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ConfigRemediationConfigurationExecutionControlsElSsmControlsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    concurrent_execution_rate_percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_percentage: Option<PrimField<f64>>,
}
impl ConfigRemediationConfigurationExecutionControlsElSsmControlsEl {
    #[doc = "Set the field `concurrent_execution_rate_percentage`.\n"]
    pub fn set_concurrent_execution_rate_percentage(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.concurrent_execution_rate_percentage = Some(v.into());
        self
    }
    #[doc = "Set the field `error_percentage`.\n"]
    pub fn set_error_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.error_percentage = Some(v.into());
        self
    }
}
impl ToListMappable for ConfigRemediationConfigurationExecutionControlsElSsmControlsEl {
    type O = BlockAssignable<ConfigRemediationConfigurationExecutionControlsElSsmControlsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConfigRemediationConfigurationExecutionControlsElSsmControlsEl {}
impl BuildConfigRemediationConfigurationExecutionControlsElSsmControlsEl {
    pub fn build(self) -> ConfigRemediationConfigurationExecutionControlsElSsmControlsEl {
        ConfigRemediationConfigurationExecutionControlsElSsmControlsEl {
            concurrent_execution_rate_percentage: core::default::Default::default(),
            error_percentage: core::default::Default::default(),
        }
    }
}
pub struct ConfigRemediationConfigurationExecutionControlsElSsmControlsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConfigRemediationConfigurationExecutionControlsElSsmControlsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ConfigRemediationConfigurationExecutionControlsElSsmControlsElRef {
        ConfigRemediationConfigurationExecutionControlsElSsmControlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConfigRemediationConfigurationExecutionControlsElSsmControlsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `concurrent_execution_rate_percentage` after provisioning.\n"]
    pub fn concurrent_execution_rate_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.concurrent_execution_rate_percentage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `error_percentage` after provisioning.\n"]
    pub fn error_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.error_percentage", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ConfigRemediationConfigurationExecutionControlsElDynamic {
    ssm_controls:
        Option<DynamicBlock<ConfigRemediationConfigurationExecutionControlsElSsmControlsEl>>,
}
#[derive(Serialize)]
pub struct ConfigRemediationConfigurationExecutionControlsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ssm_controls: Option<Vec<ConfigRemediationConfigurationExecutionControlsElSsmControlsEl>>,
    dynamic: ConfigRemediationConfigurationExecutionControlsElDynamic,
}
impl ConfigRemediationConfigurationExecutionControlsEl {
    #[doc = "Set the field `ssm_controls`.\n"]
    pub fn set_ssm_controls(
        mut self,
        v: impl Into<BlockAssignable<ConfigRemediationConfigurationExecutionControlsElSsmControlsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ssm_controls = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ssm_controls = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ConfigRemediationConfigurationExecutionControlsEl {
    type O = BlockAssignable<ConfigRemediationConfigurationExecutionControlsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConfigRemediationConfigurationExecutionControlsEl {}
impl BuildConfigRemediationConfigurationExecutionControlsEl {
    pub fn build(self) -> ConfigRemediationConfigurationExecutionControlsEl {
        ConfigRemediationConfigurationExecutionControlsEl {
            ssm_controls: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ConfigRemediationConfigurationExecutionControlsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConfigRemediationConfigurationExecutionControlsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ConfigRemediationConfigurationExecutionControlsElRef {
        ConfigRemediationConfigurationExecutionControlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConfigRemediationConfigurationExecutionControlsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ssm_controls` after provisioning.\n"]
    pub fn ssm_controls(
        &self,
    ) -> ListRef<ConfigRemediationConfigurationExecutionControlsElSsmControlsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ssm_controls", self.base))
    }
}
#[derive(Serialize)]
pub struct ConfigRemediationConfigurationParameterEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    static_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    static_values: Option<ListField<PrimField<String>>>,
}
impl ConfigRemediationConfigurationParameterEl {
    #[doc = "Set the field `resource_value`.\n"]
    pub fn set_resource_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_value = Some(v.into());
        self
    }
    #[doc = "Set the field `static_value`.\n"]
    pub fn set_static_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.static_value = Some(v.into());
        self
    }
    #[doc = "Set the field `static_values`.\n"]
    pub fn set_static_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.static_values = Some(v.into());
        self
    }
}
impl ToListMappable for ConfigRemediationConfigurationParameterEl {
    type O = BlockAssignable<ConfigRemediationConfigurationParameterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConfigRemediationConfigurationParameterEl {
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildConfigRemediationConfigurationParameterEl {
    pub fn build(self) -> ConfigRemediationConfigurationParameterEl {
        ConfigRemediationConfigurationParameterEl {
            name: self.name,
            resource_value: core::default::Default::default(),
            static_value: core::default::Default::default(),
            static_values: core::default::Default::default(),
        }
    }
}
pub struct ConfigRemediationConfigurationParameterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConfigRemediationConfigurationParameterElRef {
    fn new(shared: StackShared, base: String) -> ConfigRemediationConfigurationParameterElRef {
        ConfigRemediationConfigurationParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConfigRemediationConfigurationParameterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_value` after provisioning.\n"]
    pub fn resource_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_value", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `static_value` after provisioning.\n"]
    pub fn static_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.static_value", self.base))
    }
    #[doc = "Get a reference to the value of field `static_values` after provisioning.\n"]
    pub fn static_values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.static_values", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ConfigRemediationConfigurationDynamic {
    execution_controls: Option<DynamicBlock<ConfigRemediationConfigurationExecutionControlsEl>>,
    parameter: Option<DynamicBlock<ConfigRemediationConfigurationParameterEl>>,
}
