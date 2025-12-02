use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BedrockagentcoreMemoryStrategyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_execution_role_arn: Option<PrimField<String>>,
    memory_id: PrimField<String>,
    name: PrimField<String>,
    namespaces: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<Vec<BedrockagentcoreMemoryStrategyConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BedrockagentcoreMemoryStrategyTimeoutsEl>,
    dynamic: BedrockagentcoreMemoryStrategyDynamic,
}
struct BedrockagentcoreMemoryStrategy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BedrockagentcoreMemoryStrategyData>,
}
#[derive(Clone)]
pub struct BedrockagentcoreMemoryStrategy(Rc<BedrockagentcoreMemoryStrategy_>);
impl BedrockagentcoreMemoryStrategy {
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
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_execution_role_arn`.\n"]
    pub fn set_memory_execution_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().memory_execution_role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl Into<BlockAssignable<BedrockagentcoreMemoryStrategyConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BedrockagentcoreMemoryStrategyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memory_execution_role_arn` after provisioning.\n"]
    pub fn memory_execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_execution_role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memory_id` after provisioning.\n"]
    pub fn memory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memory_strategy_id` after provisioning.\n"]
    pub fn memory_strategy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_strategy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `namespaces` after provisioning.\n"]
    pub fn namespaces(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.namespaces", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<BedrockagentcoreMemoryStrategyConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreMemoryStrategyTimeoutsElRef {
        BedrockagentcoreMemoryStrategyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BedrockagentcoreMemoryStrategy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BedrockagentcoreMemoryStrategy {}
impl ToListMappable for BedrockagentcoreMemoryStrategy {
    type O = ListRef<BedrockagentcoreMemoryStrategyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BedrockagentcoreMemoryStrategy_ {
    fn extract_resource_type(&self) -> String {
        "aws_bedrockagentcore_memory_strategy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBedrockagentcoreMemoryStrategy {
    pub tf_id: String,
    #[doc = ""]
    pub memory_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub namespaces: SetField<PrimField<String>>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildBedrockagentcoreMemoryStrategy {
    pub fn build(self, stack: &mut Stack) -> BedrockagentcoreMemoryStrategy {
        let out = BedrockagentcoreMemoryStrategy(Rc::new(BedrockagentcoreMemoryStrategy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BedrockagentcoreMemoryStrategyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                memory_execution_role_arn: core::default::Default::default(),
                memory_id: self.memory_id,
                name: self.name,
                namespaces: self.namespaces,
                region: core::default::Default::default(),
                type_: self.type_,
                configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BedrockagentcoreMemoryStrategyRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreMemoryStrategyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BedrockagentcoreMemoryStrategyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memory_execution_role_arn` after provisioning.\n"]
    pub fn memory_execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_execution_role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memory_id` after provisioning.\n"]
    pub fn memory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memory_strategy_id` after provisioning.\n"]
    pub fn memory_strategy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_strategy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `namespaces` after provisioning.\n"]
    pub fn namespaces(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.namespaces", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<BedrockagentcoreMemoryStrategyConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreMemoryStrategyTimeoutsElRef {
        BedrockagentcoreMemoryStrategyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategyConfigurationElConsolidationEl {
    append_to_prompt: PrimField<String>,
    model_id: PrimField<String>,
}
impl BedrockagentcoreMemoryStrategyConfigurationElConsolidationEl {}
impl ToListMappable for BedrockagentcoreMemoryStrategyConfigurationElConsolidationEl {
    type O = BlockAssignable<BedrockagentcoreMemoryStrategyConfigurationElConsolidationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreMemoryStrategyConfigurationElConsolidationEl {
    #[doc = ""]
    pub append_to_prompt: PrimField<String>,
    #[doc = ""]
    pub model_id: PrimField<String>,
}
impl BuildBedrockagentcoreMemoryStrategyConfigurationElConsolidationEl {
    pub fn build(self) -> BedrockagentcoreMemoryStrategyConfigurationElConsolidationEl {
        BedrockagentcoreMemoryStrategyConfigurationElConsolidationEl {
            append_to_prompt: self.append_to_prompt,
            model_id: self.model_id,
        }
    }
}
pub struct BedrockagentcoreMemoryStrategyConfigurationElConsolidationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreMemoryStrategyConfigurationElConsolidationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreMemoryStrategyConfigurationElConsolidationElRef {
        BedrockagentcoreMemoryStrategyConfigurationElConsolidationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreMemoryStrategyConfigurationElConsolidationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `append_to_prompt` after provisioning.\n"]
    pub fn append_to_prompt(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.append_to_prompt", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.base))
    }
}
#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategyConfigurationElExtractionEl {
    append_to_prompt: PrimField<String>,
    model_id: PrimField<String>,
}
impl BedrockagentcoreMemoryStrategyConfigurationElExtractionEl {}
impl ToListMappable for BedrockagentcoreMemoryStrategyConfigurationElExtractionEl {
    type O = BlockAssignable<BedrockagentcoreMemoryStrategyConfigurationElExtractionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreMemoryStrategyConfigurationElExtractionEl {
    #[doc = ""]
    pub append_to_prompt: PrimField<String>,
    #[doc = ""]
    pub model_id: PrimField<String>,
}
impl BuildBedrockagentcoreMemoryStrategyConfigurationElExtractionEl {
    pub fn build(self) -> BedrockagentcoreMemoryStrategyConfigurationElExtractionEl {
        BedrockagentcoreMemoryStrategyConfigurationElExtractionEl {
            append_to_prompt: self.append_to_prompt,
            model_id: self.model_id,
        }
    }
}
pub struct BedrockagentcoreMemoryStrategyConfigurationElExtractionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreMemoryStrategyConfigurationElExtractionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreMemoryStrategyConfigurationElExtractionElRef {
        BedrockagentcoreMemoryStrategyConfigurationElExtractionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreMemoryStrategyConfigurationElExtractionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `append_to_prompt` after provisioning.\n"]
    pub fn append_to_prompt(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.append_to_prompt", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.base))
    }
}
#[derive(Serialize, Default)]
struct BedrockagentcoreMemoryStrategyConfigurationElDynamic {
    consolidation:
        Option<DynamicBlock<BedrockagentcoreMemoryStrategyConfigurationElConsolidationEl>>,
    extraction: Option<DynamicBlock<BedrockagentcoreMemoryStrategyConfigurationElExtractionEl>>,
}
#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategyConfigurationEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consolidation: Option<Vec<BedrockagentcoreMemoryStrategyConfigurationElConsolidationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extraction: Option<Vec<BedrockagentcoreMemoryStrategyConfigurationElExtractionEl>>,
    dynamic: BedrockagentcoreMemoryStrategyConfigurationElDynamic,
}
impl BedrockagentcoreMemoryStrategyConfigurationEl {
    #[doc = "Set the field `consolidation`.\n"]
    pub fn set_consolidation(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentcoreMemoryStrategyConfigurationElConsolidationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.consolidation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.consolidation = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `extraction`.\n"]
    pub fn set_extraction(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentcoreMemoryStrategyConfigurationElExtractionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.extraction = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.extraction = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BedrockagentcoreMemoryStrategyConfigurationEl {
    type O = BlockAssignable<BedrockagentcoreMemoryStrategyConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreMemoryStrategyConfigurationEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildBedrockagentcoreMemoryStrategyConfigurationEl {
    pub fn build(self) -> BedrockagentcoreMemoryStrategyConfigurationEl {
        BedrockagentcoreMemoryStrategyConfigurationEl {
            type_: self.type_,
            consolidation: core::default::Default::default(),
            extraction: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BedrockagentcoreMemoryStrategyConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreMemoryStrategyConfigurationElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentcoreMemoryStrategyConfigurationElRef {
        BedrockagentcoreMemoryStrategyConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreMemoryStrategyConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `consolidation` after provisioning.\n"]
    pub fn consolidation(
        &self,
    ) -> ListRef<BedrockagentcoreMemoryStrategyConfigurationElConsolidationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.consolidation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `extraction` after provisioning.\n"]
    pub fn extraction(
        &self,
    ) -> ListRef<BedrockagentcoreMemoryStrategyConfigurationElExtractionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.extraction", self.base))
    }
}
#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BedrockagentcoreMemoryStrategyTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for BedrockagentcoreMemoryStrategyTimeoutsEl {
    type O = BlockAssignable<BedrockagentcoreMemoryStrategyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreMemoryStrategyTimeoutsEl {}
impl BuildBedrockagentcoreMemoryStrategyTimeoutsEl {
    pub fn build(self) -> BedrockagentcoreMemoryStrategyTimeoutsEl {
        BedrockagentcoreMemoryStrategyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BedrockagentcoreMemoryStrategyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreMemoryStrategyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentcoreMemoryStrategyTimeoutsElRef {
        BedrockagentcoreMemoryStrategyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreMemoryStrategyTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct BedrockagentcoreMemoryStrategyDynamic {
    configuration: Option<DynamicBlock<BedrockagentcoreMemoryStrategyConfigurationEl>>,
}
