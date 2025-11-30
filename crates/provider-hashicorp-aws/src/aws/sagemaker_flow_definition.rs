use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct SagemakerFlowDefinitionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    flow_definition_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    human_loop_activation_config: Option<Vec<SagemakerFlowDefinitionHumanLoopActivationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    human_loop_config: Option<Vec<SagemakerFlowDefinitionHumanLoopConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    human_loop_request_source: Option<Vec<SagemakerFlowDefinitionHumanLoopRequestSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_config: Option<Vec<SagemakerFlowDefinitionOutputConfigEl>>,
    dynamic: SagemakerFlowDefinitionDynamic,
}

struct SagemakerFlowDefinition_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SagemakerFlowDefinitionData>,
}

#[derive(Clone)]
pub struct SagemakerFlowDefinition(Rc<SagemakerFlowDefinition_>);

impl SagemakerFlowDefinition {
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `human_loop_activation_config`.\n"]
    pub fn set_human_loop_activation_config(
        self,
        v: impl Into<BlockAssignable<SagemakerFlowDefinitionHumanLoopActivationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().human_loop_activation_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .human_loop_activation_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `human_loop_config`.\n"]
    pub fn set_human_loop_config(
        self,
        v: impl Into<BlockAssignable<SagemakerFlowDefinitionHumanLoopConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().human_loop_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.human_loop_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `human_loop_request_source`.\n"]
    pub fn set_human_loop_request_source(
        self,
        v: impl Into<BlockAssignable<SagemakerFlowDefinitionHumanLoopRequestSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().human_loop_request_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.human_loop_request_source = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `output_config`.\n"]
    pub fn set_output_config(
        self,
        v: impl Into<BlockAssignable<SagemakerFlowDefinitionOutputConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().output_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.output_config = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `flow_definition_name` after provisioning.\n"]
    pub fn flow_definition_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.flow_definition_name", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `human_loop_activation_config` after provisioning.\n"]
    pub fn human_loop_activation_config(
        &self,
    ) -> ListRef<SagemakerFlowDefinitionHumanLoopActivationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.human_loop_activation_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `human_loop_config` after provisioning.\n"]
    pub fn human_loop_config(&self) -> ListRef<SagemakerFlowDefinitionHumanLoopConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.human_loop_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `human_loop_request_source` after provisioning.\n"]
    pub fn human_loop_request_source(
        &self,
    ) -> ListRef<SagemakerFlowDefinitionHumanLoopRequestSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.human_loop_request_source", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `output_config` after provisioning.\n"]
    pub fn output_config(&self) -> ListRef<SagemakerFlowDefinitionOutputConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_config", self.extract_ref()),
        )
    }
}

impl Referable for SagemakerFlowDefinition {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for SagemakerFlowDefinition {}

impl ToListMappable for SagemakerFlowDefinition {
    type O = ListRef<SagemakerFlowDefinitionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SagemakerFlowDefinition_ {
    fn extract_resource_type(&self) -> String {
        "aws_sagemaker_flow_definition".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerFlowDefinition {
    pub tf_id: String,
    #[doc = ""]
    pub flow_definition_name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}

impl BuildSagemakerFlowDefinition {
    pub fn build(self, stack: &mut Stack) -> SagemakerFlowDefinition {
        let out = SagemakerFlowDefinition(Rc::new(SagemakerFlowDefinition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SagemakerFlowDefinitionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                flow_definition_name: self.flow_definition_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                human_loop_activation_config: core::default::Default::default(),
                human_loop_config: core::default::Default::default(),
                human_loop_request_source: core::default::Default::default(),
                output_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerFlowDefinitionRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerFlowDefinitionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl SagemakerFlowDefinitionRef {
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

    #[doc = "Get a reference to the value of field `flow_definition_name` after provisioning.\n"]
    pub fn flow_definition_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.flow_definition_name", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `human_loop_activation_config` after provisioning.\n"]
    pub fn human_loop_activation_config(
        &self,
    ) -> ListRef<SagemakerFlowDefinitionHumanLoopActivationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.human_loop_activation_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `human_loop_config` after provisioning.\n"]
    pub fn human_loop_config(&self) -> ListRef<SagemakerFlowDefinitionHumanLoopConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.human_loop_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `human_loop_request_source` after provisioning.\n"]
    pub fn human_loop_request_source(
        &self,
    ) -> ListRef<SagemakerFlowDefinitionHumanLoopRequestSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.human_loop_request_source", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `output_config` after provisioning.\n"]
    pub fn output_config(&self) -> ListRef<SagemakerFlowDefinitionOutputConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_config", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerFlowDefinitionHumanLoopActivationConfigElHumanLoopActivationConditionsConfigEl {
    human_loop_activation_conditions: PrimField<String>,
}

impl SagemakerFlowDefinitionHumanLoopActivationConfigElHumanLoopActivationConditionsConfigEl {}

impl ToListMappable
    for SagemakerFlowDefinitionHumanLoopActivationConfigElHumanLoopActivationConditionsConfigEl
{
    type O = BlockAssignable<
        SagemakerFlowDefinitionHumanLoopActivationConfigElHumanLoopActivationConditionsConfigEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFlowDefinitionHumanLoopActivationConfigElHumanLoopActivationConditionsConfigEl
{
    #[doc = ""]
    pub human_loop_activation_conditions: PrimField<String>,
}

impl BuildSagemakerFlowDefinitionHumanLoopActivationConfigElHumanLoopActivationConditionsConfigEl {
    pub fn build(
        self,
    ) -> SagemakerFlowDefinitionHumanLoopActivationConfigElHumanLoopActivationConditionsConfigEl
    {
        SagemakerFlowDefinitionHumanLoopActivationConfigElHumanLoopActivationConditionsConfigEl {
            human_loop_activation_conditions: self.human_loop_activation_conditions,
        }
    }
}

pub struct SagemakerFlowDefinitionHumanLoopActivationConfigElHumanLoopActivationConditionsConfigElRef
{
    shared: StackShared,
    base: String,
}

impl Ref
    for SagemakerFlowDefinitionHumanLoopActivationConfigElHumanLoopActivationConditionsConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerFlowDefinitionHumanLoopActivationConfigElHumanLoopActivationConditionsConfigElRef
    {
        SagemakerFlowDefinitionHumanLoopActivationConfigElHumanLoopActivationConditionsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFlowDefinitionHumanLoopActivationConfigElHumanLoopActivationConditionsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `human_loop_activation_conditions` after provisioning.\n"]
    pub fn human_loop_activation_conditions(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.human_loop_activation_conditions", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SagemakerFlowDefinitionHumanLoopActivationConfigElDynamic {
    human_loop_activation_conditions_config: Option<
        DynamicBlock<
            SagemakerFlowDefinitionHumanLoopActivationConfigElHumanLoopActivationConditionsConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerFlowDefinitionHumanLoopActivationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    human_loop_activation_conditions_config: Option<
        Vec<
            SagemakerFlowDefinitionHumanLoopActivationConfigElHumanLoopActivationConditionsConfigEl,
        >,
    >,
    dynamic: SagemakerFlowDefinitionHumanLoopActivationConfigElDynamic,
}

impl SagemakerFlowDefinitionHumanLoopActivationConfigEl {
    #[doc = "Set the field `human_loop_activation_conditions_config`.\n"]
    pub fn set_human_loop_activation_conditions_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerFlowDefinitionHumanLoopActivationConfigElHumanLoopActivationConditionsConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.human_loop_activation_conditions_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.human_loop_activation_conditions_config = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SagemakerFlowDefinitionHumanLoopActivationConfigEl {
    type O = BlockAssignable<SagemakerFlowDefinitionHumanLoopActivationConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFlowDefinitionHumanLoopActivationConfigEl {}

impl BuildSagemakerFlowDefinitionHumanLoopActivationConfigEl {
    pub fn build(self) -> SagemakerFlowDefinitionHumanLoopActivationConfigEl {
        SagemakerFlowDefinitionHumanLoopActivationConfigEl {
            human_loop_activation_conditions_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerFlowDefinitionHumanLoopActivationConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerFlowDefinitionHumanLoopActivationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerFlowDefinitionHumanLoopActivationConfigElRef {
        SagemakerFlowDefinitionHumanLoopActivationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFlowDefinitionHumanLoopActivationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `human_loop_activation_conditions_config` after provisioning.\n"]
    pub fn human_loop_activation_conditions_config(
        &self,
    ) -> ListRef<
        SagemakerFlowDefinitionHumanLoopActivationConfigElHumanLoopActivationConditionsConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.human_loop_activation_conditions_config", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceElAmountInUsdEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cents: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dollars: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tenth_fractions_of_a_cent: Option<PrimField<f64>>,
}

impl SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceElAmountInUsdEl {
    #[doc = "Set the field `cents`.\n"]
    pub fn set_cents(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cents = Some(v.into());
        self
    }

    #[doc = "Set the field `dollars`.\n"]
    pub fn set_dollars(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.dollars = Some(v.into());
        self
    }

    #[doc = "Set the field `tenth_fractions_of_a_cent`.\n"]
    pub fn set_tenth_fractions_of_a_cent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.tenth_fractions_of_a_cent = Some(v.into());
        self
    }
}

impl ToListMappable
    for SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceElAmountInUsdEl
{
    type O = BlockAssignable<
        SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceElAmountInUsdEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceElAmountInUsdEl {}

impl BuildSagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceElAmountInUsdEl {
    pub fn build(
        self,
    ) -> SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceElAmountInUsdEl {
        SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceElAmountInUsdEl {
            cents: core::default::Default::default(),
            dollars: core::default::Default::default(),
            tenth_fractions_of_a_cent: core::default::Default::default(),
        }
    }
}

pub struct SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceElAmountInUsdElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceElAmountInUsdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceElAmountInUsdElRef {
        SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceElAmountInUsdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceElAmountInUsdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cents` after provisioning.\n"]
    pub fn cents(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cents", self.base))
    }

    #[doc = "Get a reference to the value of field `dollars` after provisioning.\n"]
    pub fn dollars(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.dollars", self.base))
    }

    #[doc = "Get a reference to the value of field `tenth_fractions_of_a_cent` after provisioning.\n"]
    pub fn tenth_fractions_of_a_cent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tenth_fractions_of_a_cent", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceElDynamic {
    amount_in_usd: Option<
        DynamicBlock<
            SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceElAmountInUsdEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_in_usd: Option<
        Vec<SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceElAmountInUsdEl>,
    >,
    dynamic: SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceElDynamic,
}

impl SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceEl {
    #[doc = "Set the field `amount_in_usd`.\n"]
    pub fn set_amount_in_usd(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceElAmountInUsdEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.amount_in_usd = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.amount_in_usd = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceEl {
    type O = BlockAssignable<SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceEl {}

impl BuildSagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceEl {
    pub fn build(self) -> SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceEl {
        SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceEl {
            amount_in_usd: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceElRef {
        SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amount_in_usd` after provisioning.\n"]
    pub fn amount_in_usd(
        &self,
    ) -> ListRef<SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceElAmountInUsdElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.amount_in_usd", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SagemakerFlowDefinitionHumanLoopConfigElDynamic {
    public_workforce_task_price:
        Option<DynamicBlock<SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceEl>>,
}

#[derive(Serialize)]
pub struct SagemakerFlowDefinitionHumanLoopConfigEl {
    human_task_ui_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_availability_lifetime_in_seconds: Option<PrimField<f64>>,
    task_count: PrimField<f64>,
    task_description: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_keywords: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_time_limit_in_seconds: Option<PrimField<f64>>,
    task_title: PrimField<String>,
    workteam_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_workforce_task_price:
        Option<Vec<SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceEl>>,
    dynamic: SagemakerFlowDefinitionHumanLoopConfigElDynamic,
}

impl SagemakerFlowDefinitionHumanLoopConfigEl {
    #[doc = "Set the field `task_availability_lifetime_in_seconds`.\n"]
    pub fn set_task_availability_lifetime_in_seconds(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.task_availability_lifetime_in_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `task_keywords`.\n"]
    pub fn set_task_keywords(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.task_keywords = Some(v.into());
        self
    }

    #[doc = "Set the field `task_time_limit_in_seconds`.\n"]
    pub fn set_task_time_limit_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.task_time_limit_in_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `public_workforce_task_price`.\n"]
    pub fn set_public_workforce_task_price(
        mut self,
        v: impl Into<
            BlockAssignable<SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.public_workforce_task_price = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.public_workforce_task_price = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SagemakerFlowDefinitionHumanLoopConfigEl {
    type O = BlockAssignable<SagemakerFlowDefinitionHumanLoopConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFlowDefinitionHumanLoopConfigEl {
    #[doc = ""]
    pub human_task_ui_arn: PrimField<String>,
    #[doc = ""]
    pub task_count: PrimField<f64>,
    #[doc = ""]
    pub task_description: PrimField<String>,
    #[doc = ""]
    pub task_title: PrimField<String>,
    #[doc = ""]
    pub workteam_arn: PrimField<String>,
}

impl BuildSagemakerFlowDefinitionHumanLoopConfigEl {
    pub fn build(self) -> SagemakerFlowDefinitionHumanLoopConfigEl {
        SagemakerFlowDefinitionHumanLoopConfigEl {
            human_task_ui_arn: self.human_task_ui_arn,
            task_availability_lifetime_in_seconds: core::default::Default::default(),
            task_count: self.task_count,
            task_description: self.task_description,
            task_keywords: core::default::Default::default(),
            task_time_limit_in_seconds: core::default::Default::default(),
            task_title: self.task_title,
            workteam_arn: self.workteam_arn,
            public_workforce_task_price: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerFlowDefinitionHumanLoopConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerFlowDefinitionHumanLoopConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerFlowDefinitionHumanLoopConfigElRef {
        SagemakerFlowDefinitionHumanLoopConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFlowDefinitionHumanLoopConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `human_task_ui_arn` after provisioning.\n"]
    pub fn human_task_ui_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.human_task_ui_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `task_availability_lifetime_in_seconds` after provisioning.\n"]
    pub fn task_availability_lifetime_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.task_availability_lifetime_in_seconds", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `task_count` after provisioning.\n"]
    pub fn task_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_count", self.base))
    }

    #[doc = "Get a reference to the value of field `task_description` after provisioning.\n"]
    pub fn task_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.task_description", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `task_keywords` after provisioning.\n"]
    pub fn task_keywords(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.task_keywords", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `task_time_limit_in_seconds` after provisioning.\n"]
    pub fn task_time_limit_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.task_time_limit_in_seconds", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `task_title` after provisioning.\n"]
    pub fn task_title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_title", self.base))
    }

    #[doc = "Get a reference to the value of field `workteam_arn` after provisioning.\n"]
    pub fn workteam_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workteam_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `public_workforce_task_price` after provisioning.\n"]
    pub fn public_workforce_task_price(
        &self,
    ) -> ListRef<SagemakerFlowDefinitionHumanLoopConfigElPublicWorkforceTaskPriceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.public_workforce_task_price", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerFlowDefinitionHumanLoopRequestSourceEl {
    aws_managed_human_loop_request_source: PrimField<String>,
}

impl SagemakerFlowDefinitionHumanLoopRequestSourceEl {}

impl ToListMappable for SagemakerFlowDefinitionHumanLoopRequestSourceEl {
    type O = BlockAssignable<SagemakerFlowDefinitionHumanLoopRequestSourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFlowDefinitionHumanLoopRequestSourceEl {
    #[doc = ""]
    pub aws_managed_human_loop_request_source: PrimField<String>,
}

impl BuildSagemakerFlowDefinitionHumanLoopRequestSourceEl {
    pub fn build(self) -> SagemakerFlowDefinitionHumanLoopRequestSourceEl {
        SagemakerFlowDefinitionHumanLoopRequestSourceEl {
            aws_managed_human_loop_request_source: self.aws_managed_human_loop_request_source,
        }
    }
}

pub struct SagemakerFlowDefinitionHumanLoopRequestSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerFlowDefinitionHumanLoopRequestSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerFlowDefinitionHumanLoopRequestSourceElRef {
        SagemakerFlowDefinitionHumanLoopRequestSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFlowDefinitionHumanLoopRequestSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_managed_human_loop_request_source` after provisioning.\n"]
    pub fn aws_managed_human_loop_request_source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_managed_human_loop_request_source", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerFlowDefinitionOutputConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    s3_output_path: PrimField<String>,
}

impl SagemakerFlowDefinitionOutputConfigEl {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerFlowDefinitionOutputConfigEl {
    type O = BlockAssignable<SagemakerFlowDefinitionOutputConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFlowDefinitionOutputConfigEl {
    #[doc = ""]
    pub s3_output_path: PrimField<String>,
}

impl BuildSagemakerFlowDefinitionOutputConfigEl {
    pub fn build(self) -> SagemakerFlowDefinitionOutputConfigEl {
        SagemakerFlowDefinitionOutputConfigEl {
            kms_key_id: core::default::Default::default(),
            s3_output_path: self.s3_output_path,
        }
    }
}

pub struct SagemakerFlowDefinitionOutputConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerFlowDefinitionOutputConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerFlowDefinitionOutputConfigElRef {
        SagemakerFlowDefinitionOutputConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFlowDefinitionOutputConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_output_path` after provisioning.\n"]
    pub fn s3_output_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_output_path", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SagemakerFlowDefinitionDynamic {
    human_loop_activation_config:
        Option<DynamicBlock<SagemakerFlowDefinitionHumanLoopActivationConfigEl>>,
    human_loop_config: Option<DynamicBlock<SagemakerFlowDefinitionHumanLoopConfigEl>>,
    human_loop_request_source:
        Option<DynamicBlock<SagemakerFlowDefinitionHumanLoopRequestSourceEl>>,
    output_config: Option<DynamicBlock<SagemakerFlowDefinitionOutputConfigEl>>,
}
