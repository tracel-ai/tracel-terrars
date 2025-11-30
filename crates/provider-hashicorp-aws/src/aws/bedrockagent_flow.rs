use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct BedrockagentFlowData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_encryption_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    execution_role_arn: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    definition: Option<Vec<BedrockagentFlowDefinitionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BedrockagentFlowTimeoutsEl>,
    dynamic: BedrockagentFlowDynamic,
}

struct BedrockagentFlow_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BedrockagentFlowData>,
}

#[derive(Clone)]
pub struct BedrockagentFlow(Rc<BedrockagentFlow_>);

impl BedrockagentFlow {
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

    #[doc = "Set the field `customer_encryption_key_arn`.\n"]
    pub fn set_customer_encryption_key_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().customer_encryption_key_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
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

    #[doc = "Set the field `definition`.\n"]
    pub fn set_definition(
        self,
        v: impl Into<BlockAssignable<BedrockagentFlowDefinitionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().definition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.definition = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BedrockagentFlowTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `customer_encryption_key_arn` after provisioning.\n"]
    pub fn customer_encryption_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_encryption_key_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.updated_at", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> ListRef<BedrockagentFlowDefinitionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.definition", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentFlowTimeoutsElRef {
        BedrockagentFlowTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for BedrockagentFlow {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for BedrockagentFlow {}

impl ToListMappable for BedrockagentFlow {
    type O = ListRef<BedrockagentFlowRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BedrockagentFlow_ {
    fn extract_resource_type(&self) -> String {
        "aws_bedrockagent_flow".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentFlow {
    pub tf_id: String,
    #[doc = ""]
    pub execution_role_arn: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildBedrockagentFlow {
    pub fn build(self, stack: &mut Stack) -> BedrockagentFlow {
        let out = BedrockagentFlow(Rc::new(BedrockagentFlow_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BedrockagentFlowData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                customer_encryption_key_arn: core::default::Default::default(),
                description: core::default::Default::default(),
                execution_role_arn: self.execution_role_arn,
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                definition: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentFlowRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl BedrockagentFlowRef {
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

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `customer_encryption_key_arn` after provisioning.\n"]
    pub fn customer_encryption_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_encryption_key_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.updated_at", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> ListRef<BedrockagentFlowDefinitionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.definition", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentFlowTimeoutsElRef {
        BedrockagentFlowTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElConnectionElConfigurationElConditionalEl {
    condition: PrimField<String>,
}

impl BedrockagentFlowDefinitionElConnectionElConfigurationElConditionalEl {}

impl ToListMappable for BedrockagentFlowDefinitionElConnectionElConfigurationElConditionalEl {
    type O = BlockAssignable<BedrockagentFlowDefinitionElConnectionElConfigurationElConditionalEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElConnectionElConfigurationElConditionalEl {
    #[doc = ""]
    pub condition: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElConnectionElConfigurationElConditionalEl {
    pub fn build(self) -> BedrockagentFlowDefinitionElConnectionElConfigurationElConditionalEl {
        BedrockagentFlowDefinitionElConnectionElConfigurationElConditionalEl {
            condition: self.condition,
        }
    }
}

pub struct BedrockagentFlowDefinitionElConnectionElConfigurationElConditionalElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElConnectionElConfigurationElConditionalElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElConnectionElConfigurationElConditionalElRef {
        BedrockagentFlowDefinitionElConnectionElConfigurationElConditionalElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElConnectionElConfigurationElConditionalElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.condition", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElConnectionElConfigurationElDataEl {
    source_output: PrimField<String>,
    target_input: PrimField<String>,
}

impl BedrockagentFlowDefinitionElConnectionElConfigurationElDataEl {}

impl ToListMappable for BedrockagentFlowDefinitionElConnectionElConfigurationElDataEl {
    type O = BlockAssignable<BedrockagentFlowDefinitionElConnectionElConfigurationElDataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElConnectionElConfigurationElDataEl {
    #[doc = ""]
    pub source_output: PrimField<String>,
    #[doc = ""]
    pub target_input: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElConnectionElConfigurationElDataEl {
    pub fn build(self) -> BedrockagentFlowDefinitionElConnectionElConfigurationElDataEl {
        BedrockagentFlowDefinitionElConnectionElConfigurationElDataEl {
            source_output: self.source_output,
            target_input: self.target_input,
        }
    }
}

pub struct BedrockagentFlowDefinitionElConnectionElConfigurationElDataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElConnectionElConfigurationElDataElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElConnectionElConfigurationElDataElRef {
        BedrockagentFlowDefinitionElConnectionElConfigurationElDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElConnectionElConfigurationElDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_output` after provisioning.\n"]
    pub fn source_output(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_output", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `target_input` after provisioning.\n"]
    pub fn target_input(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_input", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElConnectionElConfigurationElDynamic {
    conditional:
        Option<DynamicBlock<BedrockagentFlowDefinitionElConnectionElConfigurationElConditionalEl>>,
    data: Option<DynamicBlock<BedrockagentFlowDefinitionElConnectionElConfigurationElDataEl>>,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElConnectionElConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    conditional: Option<Vec<BedrockagentFlowDefinitionElConnectionElConfigurationElConditionalEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Vec<BedrockagentFlowDefinitionElConnectionElConfigurationElDataEl>>,
    dynamic: BedrockagentFlowDefinitionElConnectionElConfigurationElDynamic,
}

impl BedrockagentFlowDefinitionElConnectionElConfigurationEl {
    #[doc = "Set the field `conditional`.\n"]
    pub fn set_conditional(
        mut self,
        v: impl Into<
            BlockAssignable<BedrockagentFlowDefinitionElConnectionElConfigurationElConditionalEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conditional = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conditional = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `data`.\n"]
    pub fn set_data(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentFlowDefinitionElConnectionElConfigurationElDataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionElConnectionElConfigurationEl {
    type O = BlockAssignable<BedrockagentFlowDefinitionElConnectionElConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElConnectionElConfigurationEl {}

impl BuildBedrockagentFlowDefinitionElConnectionElConfigurationEl {
    pub fn build(self) -> BedrockagentFlowDefinitionElConnectionElConfigurationEl {
        BedrockagentFlowDefinitionElConnectionElConfigurationEl {
            conditional: core::default::Default::default(),
            data: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElConnectionElConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElConnectionElConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElConnectionElConfigurationElRef {
        BedrockagentFlowDefinitionElConnectionElConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElConnectionElConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `conditional` after provisioning.\n"]
    pub fn conditional(
        &self,
    ) -> ListRef<BedrockagentFlowDefinitionElConnectionElConfigurationElConditionalElRef> {
        ListRef::new(self.shared().clone(), format!("{}.conditional", self.base))
    }

    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(
        &self,
    ) -> ListRef<BedrockagentFlowDefinitionElConnectionElConfigurationElDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElConnectionElDynamic {
    configuration: Option<DynamicBlock<BedrockagentFlowDefinitionElConnectionElConfigurationEl>>,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElConnectionEl {
    name: PrimField<String>,
    source: PrimField<String>,
    target: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<Vec<BedrockagentFlowDefinitionElConnectionElConfigurationEl>>,
    dynamic: BedrockagentFlowDefinitionElConnectionElDynamic,
}

impl BedrockagentFlowDefinitionElConnectionEl {
    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentFlowDefinitionElConnectionElConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionElConnectionEl {
    type O = BlockAssignable<BedrockagentFlowDefinitionElConnectionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElConnectionEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub source: PrimField<String>,
    #[doc = ""]
    pub target: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElConnectionEl {
    pub fn build(self) -> BedrockagentFlowDefinitionElConnectionEl {
        BedrockagentFlowDefinitionElConnectionEl {
            name: self.name,
            source: self.source,
            target: self.target,
            type_: self.type_,
            configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElConnectionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElConnectionElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentFlowDefinitionElConnectionElRef {
        BedrockagentFlowDefinitionElConnectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElConnectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(
        &self,
    ) -> ListRef<BedrockagentFlowDefinitionElConnectionElConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElAgentEl {
    agent_alias_arn: PrimField<String>,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElAgentEl {}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElAgentEl {
    type O = BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElAgentEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElAgentEl {
    #[doc = ""]
    pub agent_alias_arn: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElAgentEl {
    pub fn build(self) -> BedrockagentFlowDefinitionElNodeElConfigurationElAgentEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElAgentEl {
            agent_alias_arn: self.agent_alias_arn,
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElAgentElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElAgentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElAgentElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElAgentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElAgentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_alias_arn` after provisioning.\n"]
    pub fn agent_alias_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.agent_alias_arn", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElCollectorEl {}

impl BedrockagentFlowDefinitionElNodeElConfigurationElCollectorEl {}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElCollectorEl {
    type O = BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElCollectorEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElCollectorEl {}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElCollectorEl {
    pub fn build(self) -> BedrockagentFlowDefinitionElNodeElConfigurationElCollectorEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElCollectorEl {}
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElCollectorElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElCollectorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElCollectorElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElCollectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElCollectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElConditionElConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    name: PrimField<String>,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElConditionElConditionEl {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElConditionElConditionEl {
    type O =
        BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElConditionElConditionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElConditionElConditionEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElConditionElConditionEl {
    pub fn build(self) -> BedrockagentFlowDefinitionElNodeElConfigurationElConditionElConditionEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElConditionElConditionEl {
            expression: core::default::Default::default(),
            name: self.name,
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElConditionElConditionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElConditionElConditionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElConditionElConditionElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElConditionElConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElConditionElConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElNodeElConfigurationElConditionElDynamic {
    condition: Option<
        DynamicBlock<BedrockagentFlowDefinitionElNodeElConfigurationElConditionElConditionEl>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<BedrockagentFlowDefinitionElNodeElConfigurationElConditionElConditionEl>>,
    dynamic: BedrockagentFlowDefinitionElNodeElConfigurationElConditionElDynamic,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElConditionEl {
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl Into<
            BlockAssignable<
                BedrockagentFlowDefinitionElNodeElConfigurationElConditionElConditionEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElConditionEl {
    type O = BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElConditionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElConditionEl {}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElConditionEl {
    pub fn build(self) -> BedrockagentFlowDefinitionElNodeElConfigurationElConditionEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElConditionEl {
            condition: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElConditionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElConditionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElConditionElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(
        &self,
    ) -> ListRef<BedrockagentFlowDefinitionElNodeElConfigurationElConditionElConditionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElInlineCodeEl {
    code: PrimField<String>,
    language: PrimField<String>,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElInlineCodeEl {}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElInlineCodeEl {
    type O = BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElInlineCodeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElInlineCodeEl {
    #[doc = ""]
    pub code: PrimField<String>,
    #[doc = ""]
    pub language: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElInlineCodeEl {
    pub fn build(self) -> BedrockagentFlowDefinitionElNodeElConfigurationElInlineCodeEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElInlineCodeEl {
            code: self.code,
            language: self.language,
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElInlineCodeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElInlineCodeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElInlineCodeElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElInlineCodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElInlineCodeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `language` after provisioning.\n"]
    pub fn language(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.language", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElInputEl {}

impl BedrockagentFlowDefinitionElNodeElConfigurationElInputEl {}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElInputEl {
    type O = BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElInputEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElInputEl {}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElInputEl {
    pub fn build(self) -> BedrockagentFlowDefinitionElNodeElConfigurationElInputEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElInputEl {}
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElInputElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElInputElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElInputElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElInputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElIteratorEl {}

impl BedrockagentFlowDefinitionElNodeElConfigurationElIteratorEl {}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElIteratorEl {
    type O = BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElIteratorEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElIteratorEl {}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElIteratorEl {
    pub fn build(self) -> BedrockagentFlowDefinitionElNodeElConfigurationElIteratorEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElIteratorEl {}
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElIteratorElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElIteratorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElIteratorElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElIteratorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElIteratorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElGuardrailConfigurationEl
{
    guardrail_identifier: PrimField<String>,
    guardrail_version: PrimField<String>,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElGuardrailConfigurationEl {}

impl ToListMappable
    for BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElGuardrailConfigurationEl
{
    type O = BlockAssignable<
        BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElGuardrailConfigurationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElGuardrailConfigurationEl
{
    #[doc = ""]
    pub guardrail_identifier: PrimField<String>,
    #[doc = ""]
    pub guardrail_version: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElGuardrailConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElGuardrailConfigurationEl
    {
        BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElGuardrailConfigurationEl {
            guardrail_identifier: self.guardrail_identifier,
            guardrail_version: self.guardrail_version,
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElGuardrailConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref
    for BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElGuardrailConfigurationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElGuardrailConfigurationElRef
    {
        BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElGuardrailConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElGuardrailConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `guardrail_identifier` after provisioning.\n"]
    pub fn guardrail_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.guardrail_identifier", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `guardrail_version` after provisioning.\n"]
    pub fn guardrail_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.guardrail_version", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationElTextEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_sequences: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<PrimField<f64>>,
}

impl
    BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationElTextEl
{
    #[doc = "Set the field `max_tokens`.\n"]
    pub fn set_max_tokens(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_tokens = Some(v.into());
        self
    }

    #[doc = "Set the field `stop_sequences`.\n"]
    pub fn set_stop_sequences(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.stop_sequences = Some(v.into());
        self
    }

    #[doc = "Set the field `temperature`.\n"]
    pub fn set_temperature(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.temperature = Some(v.into());
        self
    }

    #[doc = "Set the field `top_p`.\n"]
    pub fn set_top_p(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.top_p = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationElTextEl {
    type O =
        BlockAssignable<
            BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationElTextEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationElTextEl
{}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationElTextEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationElTextEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationElTextEl {
            max_tokens: core::default::Default::default(),
            stop_sequences: core::default::Default::default(),
            temperature: core::default::Default::default(),
            top_p: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationElTextElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationElTextElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationElTextElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationElTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationElTextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_tokens` after provisioning.\n"]
    pub fn max_tokens(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `stop_sequences` after provisioning.\n"]
    pub fn stop_sequences(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.stop_sequences", self.base))
    }

    #[doc = "Get a reference to the value of field `temperature` after provisioning.\n"]
    pub fn temperature(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }

    #[doc = "Get a reference to the value of field `top_p` after provisioning.\n"]
    pub fn top_p(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.top_p", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationElDynamic {
    text: Option<
        DynamicBlock<BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationElTextEl>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Vec<BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationElTextEl>>,
    dynamic: BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationElDynamic,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationEl {
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationElTextEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.text = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.text = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationEl
{
    type O = BlockAssignable<
        BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationEl
{}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationEl
    {
        BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationEl {
            text: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref
    for BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationElRef
    {
        BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(
        &self,
    ) -> ListRef<BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationElTextElRef>{
        ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElDynamic {
    guardrail_configuration: Option<
        DynamicBlock<BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElGuardrailConfigurationEl>,
    >,
    inference_configuration: Option<
        DynamicBlock<BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseEl {
    knowledge_base_id: PrimField<String>,
    model_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number_of_results: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guardrail_configuration: Option<
        Vec<BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElGuardrailConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    inference_configuration: Option<
        Vec<BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationEl>,
    >,
    dynamic: BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElDynamic,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseEl {
    #[doc = "Set the field `number_of_results`.\n"]
    pub fn set_number_of_results(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.number_of_results = Some(v.into());
        self
    }

    #[doc = "Set the field `guardrail_configuration`.\n"]
    pub fn set_guardrail_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElGuardrailConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.guardrail_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.guardrail_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `inference_configuration`.\n"]
    pub fn set_inference_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.inference_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.inference_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseEl {
    type O = BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseEl {
    #[doc = ""]
    pub knowledge_base_id: PrimField<String>,
    #[doc = ""]
    pub model_id: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseEl {
    pub fn build(self) -> BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseEl {
            knowledge_base_id: self.knowledge_base_id,
            model_id: self.model_id,
            number_of_results: core::default::Default::default(),
            guardrail_configuration: core::default::Default::default(),
            inference_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `knowledge_base_id` after provisioning.\n"]
    pub fn knowledge_base_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.knowledge_base_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.base))
    }

    #[doc = "Get a reference to the value of field `number_of_results` after provisioning.\n"]
    pub fn number_of_results(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.number_of_results", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `guardrail_configuration` after provisioning.\n"]
    pub fn guardrail_configuration(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElGuardrailConfigurationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guardrail_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `inference_configuration` after provisioning.\n"]
    pub fn inference_configuration(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElInferenceConfigurationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inference_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElLambdaFunctionEl {
    lambda_arn: PrimField<String>,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElLambdaFunctionEl {}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElLambdaFunctionEl {
    type O = BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElLambdaFunctionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElLambdaFunctionEl {
    #[doc = ""]
    pub lambda_arn: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElLambdaFunctionEl {
    pub fn build(self) -> BedrockagentFlowDefinitionElNodeElConfigurationElLambdaFunctionEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElLambdaFunctionEl {
            lambda_arn: self.lambda_arn,
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElLambdaFunctionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElLambdaFunctionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElLambdaFunctionElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElLambdaFunctionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElLambdaFunctionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda_arn` after provisioning.\n"]
    pub fn lambda_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lambda_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElLexEl {
    bot_alias_arn: PrimField<String>,
    locale_id: PrimField<String>,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElLexEl {}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElLexEl {
    type O = BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElLexEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElLexEl {
    #[doc = ""]
    pub bot_alias_arn: PrimField<String>,
    #[doc = ""]
    pub locale_id: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElLexEl {
    pub fn build(self) -> BedrockagentFlowDefinitionElNodeElConfigurationElLexEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElLexEl {
            bot_alias_arn: self.bot_alias_arn,
            locale_id: self.locale_id,
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElLexElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElLexElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElLexElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElLexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElLexElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bot_alias_arn` after provisioning.\n"]
    pub fn bot_alias_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bot_alias_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `locale_id` after provisioning.\n"]
    pub fn locale_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.locale_id", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElOutputEl {}

impl BedrockagentFlowDefinitionElNodeElConfigurationElOutputEl {}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElOutputEl {
    type O = BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElOutputEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElOutputEl {}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElOutputEl {
    pub fn build(self) -> BedrockagentFlowDefinitionElNodeElConfigurationElOutputEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElOutputEl {}
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElOutputElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElOutputElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElOutputElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElOutputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElOutputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElGuardrailConfigurationEl {
    guardrail_identifier: PrimField<String>,
    guardrail_version: PrimField<String>,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElGuardrailConfigurationEl {}

impl ToListMappable
    for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElGuardrailConfigurationEl
{
    type O = BlockAssignable<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElGuardrailConfigurationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElGuardrailConfigurationEl {
    #[doc = ""]
    pub guardrail_identifier: PrimField<String>,
    #[doc = ""]
    pub guardrail_version: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElGuardrailConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElGuardrailConfigurationEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElGuardrailConfigurationEl {
            guardrail_identifier: self.guardrail_identifier,
            guardrail_version: self.guardrail_version,
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElGuardrailConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElGuardrailConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElGuardrailConfigurationElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElGuardrailConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElGuardrailConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `guardrail_identifier` after provisioning.\n"]
    pub fn guardrail_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.guardrail_identifier", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `guardrail_version` after provisioning.\n"]
    pub fn guardrail_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.guardrail_version", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationElTextEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_sequences: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<PrimField<f64>>,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationElTextEl {
    #[doc = "Set the field `max_tokens`.\n"]
    pub fn set_max_tokens(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_tokens = Some(v.into());
        self
    }

    #[doc = "Set the field `stop_sequences`.\n"]
    pub fn set_stop_sequences(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.stop_sequences = Some(v.into());
        self
    }

    #[doc = "Set the field `temperature`.\n"]
    pub fn set_temperature(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.temperature = Some(v.into());
        self
    }

    #[doc = "Set the field `top_p`.\n"]
    pub fn set_top_p(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.top_p = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationElTextEl {
    type O =
        BlockAssignable<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationElTextEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationElTextEl
{}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationElTextEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationElTextEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationElTextEl {
            max_tokens: core::default::Default::default(),
            stop_sequences: core::default::Default::default(),
            temperature: core::default::Default::default(),
            top_p: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationElTextElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationElTextElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationElTextElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationElTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationElTextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_tokens` after provisioning.\n"]
    pub fn max_tokens(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `stop_sequences` after provisioning.\n"]
    pub fn stop_sequences(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.stop_sequences", self.base))
    }

    #[doc = "Get a reference to the value of field `temperature` after provisioning.\n"]
    pub fn temperature(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }

    #[doc = "Get a reference to the value of field `top_p` after provisioning.\n"]
    pub fn top_p(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.top_p", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationElDynamic {
    text: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationElTextEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<
        Vec<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationElTextEl,
        >,
    >,
    dynamic: BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationElDynamic,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationEl {
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationElTextEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.text = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.text = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationEl {
    type O =
        BlockAssignable<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationEl
{}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationEl {
            text: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationElTextElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElInputVariableEl
{
    name: PrimField<String>,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElInputVariableEl {

}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElInputVariableEl {
    type O =
        BlockAssignable<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElInputVariableEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElInputVariableEl
{
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElInputVariableEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElInputVariableEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElInputVariableEl {
            name: self.name,
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElInputVariableElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElInputVariableElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElInputVariableElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElInputVariableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElInputVariableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentElCachePointEl
{
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentElCachePointEl {

}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentElCachePointEl {
    type O =
        BlockAssignable<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentElCachePointEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentElCachePointEl
{
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentElCachePointEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentElCachePointEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentElCachePointEl {
            type_: self.type_,
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentElCachePointElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentElCachePointElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentElCachePointElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentElCachePointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentElCachePointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentElDynamic {
    cache_point: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentElCachePointEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_point: Option<
        Vec<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentElCachePointEl,
        >,
    >,
    dynamic: BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentElDynamic,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentEl {
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text = Some(v.into());
        self
    }

    #[doc = "Set the field `cache_point`.\n"]
    pub fn set_cache_point(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentElCachePointEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cache_point = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_point = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentEl {
    type O =
        BlockAssignable<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentEl
{}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentEl {
            text: core::default::Default::default(),
            cache_point: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_point` after provisioning.\n"]
    pub fn cache_point(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentElCachePointElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.cache_point", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElDynamic {
    content: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageEl {
    role: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<
        Vec<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentEl,
        >,
    >,
    dynamic: BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElDynamic,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageEl {
    #[doc = "Set the field `content`.\n"]
    pub fn set_content(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.content = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.content = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageEl {
    type O =
        BlockAssignable<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageEl
{
    #[doc = ""]
    pub role: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageEl {
            role: self.role,
            content: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElContentElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.content", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemElCachePointEl
{
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemElCachePointEl {

}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemElCachePointEl {
    type O =
        BlockAssignable<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemElCachePointEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemElCachePointEl
{
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemElCachePointEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemElCachePointEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemElCachePointEl {
            type_: self.type_,
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemElCachePointElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemElCachePointElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemElCachePointElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemElCachePointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemElCachePointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemElDynamic {
    cache_point: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemElCachePointEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_point: Option<
        Vec<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemElCachePointEl,
        >,
    >,
    dynamic: BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemElDynamic,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemEl {
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text = Some(v.into());
        self
    }

    #[doc = "Set the field `cache_point`.\n"]
    pub fn set_cache_point(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemElCachePointEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cache_point = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_point = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemEl {
    type O =
        BlockAssignable<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemEl
{}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemEl {
            text: core::default::Default::default(),
            cache_point: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_point` after provisioning.\n"]
    pub fn cache_point(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemElCachePointElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.cache_point", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElCachePointEl
{
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElCachePointEl {

}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElCachePointEl {
    type O =
        BlockAssignable<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElCachePointEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElCachePointEl
{
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElCachePointEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElCachePointEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElCachePointEl {
            type_: self.type_,
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElCachePointElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElCachePointElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElCachePointElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElCachePointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElCachePointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    json: Option<PrimField<String>>,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaEl {
    #[doc = "Set the field `json`.\n"]
    pub fn set_json(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.json = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaEl {
    type O =
        BlockAssignable<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaEl
{}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaEl {
            json: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.json", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElDynamic {
    input_schema: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_schema: Option<
        Vec<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaEl,
        >,
    >,
    dynamic: BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElDynamic,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc = "Set the field `input_schema`.\n"]
    pub fn set_input_schema(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.input_schema = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.input_schema = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecEl {
    type O =
        BlockAssignable<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecEl
{
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecEl {
            description: core::default::Default::default(),
            name: self.name,
            input_schema: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `input_schema` after provisioning.\n"]
    pub fn input_schema(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElInputSchemaElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.input_schema", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElDynamic {
    cache_point: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElCachePointEl,
        >,
    >,
    tool_spec: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_point: Option<
        Vec<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElCachePointEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_spec: Option<
        Vec<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecEl,
        >,
    >,
    dynamic: BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElDynamic,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolEl {
    #[doc = "Set the field `cache_point`.\n"]
    pub fn set_cache_point(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElCachePointEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cache_point = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_point = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tool_spec`.\n"]
    pub fn set_tool_spec(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tool_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tool_spec = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolEl {
    type O =
        BlockAssignable<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolEl
{}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolEl {
            cache_point: core::default::Default::default(),
            tool_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cache_point` after provisioning.\n"]
    pub fn cache_point(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElCachePointElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.cache_point", self.base))
    }

    #[doc = "Get a reference to the value of field `tool_spec` after provisioning.\n"]
    pub fn tool_spec(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElToolSpecElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.tool_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyEl
{}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyEl {

}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyEl {
    type O =
        BlockAssignable<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyEl
{}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyEl {}
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoEl
{}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoEl {

}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoEl {
    type O =
        BlockAssignable<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoEl
{}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoEl {}
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolEl
{
    name: PrimField<String>,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolEl {

}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolEl {
    type O =
        BlockAssignable<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolEl
{
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolEl {
            name: self.name,
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElDynamic {
    any: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyEl,
        >,
    >,
    auto: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoEl,
        >,
    >,
    tool: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    any: Option<
        Vec<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto: Option<
        Vec<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool: Option<
        Vec<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolEl,
        >,
    >,
    dynamic: BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElDynamic,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceEl {
    #[doc = "Set the field `any`.\n"]
    pub fn set_any(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.any = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.any = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `auto`.\n"]
    pub fn set_auto(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.auto = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.auto = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tool`.\n"]
    pub fn set_tool(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tool = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tool = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceEl {
    type O =
        BlockAssignable<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceEl
{}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceEl {
            any: core::default::Default::default(),
            auto: core::default::Default::default(),
            tool: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `any` after provisioning.\n"]
    pub fn any(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAnyElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.any", self.base))
    }

    #[doc = "Get a reference to the value of field `auto` after provisioning.\n"]
    pub fn auto(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElAutoElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.auto", self.base))
    }

    #[doc = "Get a reference to the value of field `tool` after provisioning.\n"]
    pub fn tool(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElToolElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.tool", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElDynamic {
    tool: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolEl,
        >,
    >,
    tool_choice: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tool: Option<
        Vec<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<
        Vec<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceEl,
        >,
    >,
    dynamic: BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElDynamic,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationEl {
    #[doc = "Set the field `tool`.\n"]
    pub fn set_tool(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tool = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tool = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tool_choice`.\n"]
    pub fn set_tool_choice(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tool_choice = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tool_choice = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationEl {
    type O =
        BlockAssignable<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationEl
{}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationEl {
            tool: core::default::Default::default(),
            tool_choice: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tool` after provisioning.\n"]
    pub fn tool(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.tool", self.base))
    }

    #[doc = "Get a reference to the value of field `tool_choice` after provisioning.\n"]
    pub fn tool_choice(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElToolChoiceElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.tool_choice", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElDynamic {
    input_variable: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElInputVariableEl,
        >,
    >,
    message: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageEl,
        >,
    >,
    system: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemEl,
        >,
    >,
    tool_configuration: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    input_variable: Option<
        Vec<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElInputVariableEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<
        Vec<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<
        Vec<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_configuration: Option<
        Vec<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationEl,
        >,
    >,
    dynamic: BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElDynamic,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatEl {
    #[doc = "Set the field `input_variable`.\n"]
    pub fn set_input_variable(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElInputVariableEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.input_variable = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.input_variable = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.message = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.message = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `system`.\n"]
    pub fn set_system(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.system = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.system = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tool_configuration`.\n"]
    pub fn set_tool_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tool_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tool_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatEl {
    type O =
        BlockAssignable<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatEl
{}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatEl {
            input_variable: core::default::Default::default(),
            message: core::default::Default::default(),
            system: core::default::Default::default(),
            tool_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_variable` after provisioning.\n"]
    pub fn input_variable(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElInputVariableElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.input_variable", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElMessageElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `system` after provisioning.\n"]
    pub fn system(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElSystemElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.system", self.base))
    }

    #[doc = "Get a reference to the value of field `tool_configuration` after provisioning.\n"]
    pub fn tool_configuration(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElToolConfigurationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.tool_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElCachePointEl
{
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElCachePointEl {

}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElCachePointEl {
    type O =
        BlockAssignable<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElCachePointEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElCachePointEl
{
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElCachePointEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElCachePointEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElCachePointEl {
            type_: self.type_,
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElCachePointElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElCachePointElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElCachePointElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElCachePointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElCachePointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElInputVariableEl
{
    name: PrimField<String>,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElInputVariableEl {

}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElInputVariableEl {
    type O =
        BlockAssignable<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElInputVariableEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElInputVariableEl
{
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElInputVariableEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElInputVariableEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElInputVariableEl {
            name: self.name,
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElInputVariableElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElInputVariableElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElInputVariableElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElInputVariableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElInputVariableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElDynamic {
    cache_point: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElCachePointEl,
        >,
    >,
    input_variable: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElInputVariableEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextEl {
    text: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_point: Option<
        Vec<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElCachePointEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_variable: Option<
        Vec<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElInputVariableEl,
        >,
    >,
    dynamic: BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElDynamic,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextEl {
    #[doc = "Set the field `cache_point`.\n"]
    pub fn set_cache_point(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElCachePointEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cache_point = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_point = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input_variable`.\n"]
    pub fn set_input_variable(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElInputVariableEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.input_variable = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.input_variable = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextEl {
    type O =
        BlockAssignable<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextEl
{
    #[doc = ""]
    pub text: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextEl {
            text: self.text,
            cache_point: core::default::Default::default(),
            input_variable: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_point` after provisioning.\n"]
    pub fn cache_point(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElCachePointElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.cache_point", self.base))
    }

    #[doc = "Get a reference to the value of field `input_variable` after provisioning.\n"]
    pub fn input_variable(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElInputVariableElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.input_variable", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElDynamic {
    chat: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatEl,
        >,
    >,
    text: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat: Option<
        Vec<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<
        Vec<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextEl,
        >,
    >,
    dynamic: BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElDynamic,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationEl {
    #[doc = "Set the field `chat`.\n"]
    pub fn set_chat(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.chat = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.chat = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `text`.\n"]
    pub fn set_text(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.text = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.text = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationEl {
    type O =
        BlockAssignable<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationEl
{}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationEl {
            chat: core::default::Default::default(),
            text: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `chat` after provisioning.\n"]
    pub fn chat(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElChatElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.chat", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElTextElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElDynamic {
    inference_configuration: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationEl,
        >,
    >,
    template_configuration: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_model_request_fields: Option<PrimField<String>>,
    model_id: PrimField<String>,
    template_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inference_configuration: Option<
        Vec<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_configuration: Option<
        Vec<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationEl,
        >,
    >,
    dynamic: BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElDynamic,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineEl {
    #[doc = "Set the field `additional_model_request_fields`.\n"]
    pub fn set_additional_model_request_fields(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.additional_model_request_fields = Some(v.into());
        self
    }

    #[doc = "Set the field `inference_configuration`.\n"]
    pub fn set_inference_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.inference_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.inference_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `template_configuration`.\n"]
    pub fn set_template_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.template_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.template_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineEl
{
    type O = BlockAssignable<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineEl
{
    #[doc = ""]
    pub model_id: PrimField<String>,
    #[doc = ""]
    pub template_type: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineEl
    {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineEl {
            additional_model_request_fields: core::default::Default::default(),
            model_id: self.model_id,
            template_type: self.template_type,
            inference_configuration: core::default::Default::default(),
            template_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElRef
{
    shared: StackShared,
    base: String,
}

impl Ref
    for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElRef
    {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_model_request_fields` after provisioning.\n"]
    pub fn additional_model_request_fields(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.additional_model_request_fields", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.base))
    }

    #[doc = "Get a reference to the value of field `template_type` after provisioning.\n"]
    pub fn template_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `inference_configuration` after provisioning.\n"]
    pub fn inference_configuration(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElInferenceConfigurationElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.inference_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `template_configuration` after provisioning.\n"]
    pub fn template_configuration(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElTemplateConfigurationElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.template_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElResourceEl
{
    prompt_arn: PrimField<String>,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElResourceEl {}

impl ToListMappable
    for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElResourceEl
{
    type O = BlockAssignable<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElResourceEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElResourceEl
{
    #[doc = ""]
    pub prompt_arn: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElResourceEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElResourceEl
    {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElResourceEl {
            prompt_arn: self.prompt_arn,
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElResourceElRef
{
    shared: StackShared,
    base: String,
}

impl Ref
    for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElResourceElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElResourceElRef
    {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prompt_arn` after provisioning.\n"]
    pub fn prompt_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prompt_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElDynamic {
    inline: Option<
        DynamicBlock<BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineEl>,
    >,
    resource: Option<
        DynamicBlock<BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElResourceEl>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    inline: Option<Vec<BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<Vec<BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElResourceEl>>,
    dynamic: BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElDynamic,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationEl {
    #[doc = "Set the field `inline`.\n"]
    pub fn set_inline(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.inline = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.inline = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `resource`.\n"]
    pub fn set_resource(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElResourceEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationEl
{
    type O = BlockAssignable<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationEl {}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationEl {
            inline: core::default::Default::default(),
            resource: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `inline` after provisioning.\n"]
    pub fn inline(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElInlineElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.inline", self.base))
    }

    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElResourceElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElDynamic {
    guardrail_configuration: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElGuardrailConfigurationEl,
        >,
    >,
    source_configuration: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    guardrail_configuration: Option<
        Vec<BedrockagentFlowDefinitionElNodeElConfigurationElPromptElGuardrailConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_configuration:
        Option<Vec<BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationEl>>,
    dynamic: BedrockagentFlowDefinitionElNodeElConfigurationElPromptElDynamic,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptEl {
    #[doc = "Set the field `guardrail_configuration`.\n"]
    pub fn set_guardrail_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                BedrockagentFlowDefinitionElNodeElConfigurationElPromptElGuardrailConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.guardrail_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.guardrail_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `source_configuration`.\n"]
    pub fn set_source_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElPromptEl {
    type O = BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElPromptEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptEl {}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElPromptEl {
    pub fn build(self) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptEl {
            guardrail_configuration: core::default::Default::default(),
            source_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElPromptElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElPromptElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElPromptElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElPromptElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElPromptElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `guardrail_configuration` after provisioning.\n"]
    pub fn guardrail_configuration(
        &self,
    ) -> ListRef<BedrockagentFlowDefinitionElNodeElConfigurationElPromptElGuardrailConfigurationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guardrail_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `source_configuration` after provisioning.\n"]
    pub fn source_configuration(
        &self,
    ) -> ListRef<BedrockagentFlowDefinitionElNodeElConfigurationElPromptElSourceConfigurationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationElS3El {
    bucket_name: PrimField<String>,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationElS3El {}

impl ToListMappable
    for BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationElS3El
{
    type O = BlockAssignable<
        BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationElS3El,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationElS3El
{
    #[doc = ""]
    pub bucket_name: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationElS3El {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationElS3El
    {
        BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationElS3El {
            bucket_name: self.bucket_name,
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationElS3ElRef
{
    shared: StackShared,
    base: String,
}

impl Ref
    for BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationElS3ElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationElS3ElRef
    {
        BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationElS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationElDynamic {
    s3: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationElS3El,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<
        Vec<BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationElS3El>,
    >,
    dynamic:
        BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationElDynamic,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationEl {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationElS3El,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationEl
{
    type O = BlockAssignable<
        BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationEl
{}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationEl {
            s3: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationElS3ElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElDynamic {
    service_configuration: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    service_configuration: Option<
        Vec<BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationEl>,
    >,
    dynamic: BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElDynamic,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalEl {
    #[doc = "Set the field `service_configuration`.\n"]
    pub fn set_service_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalEl {
    type O = BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElRetrievalEl {}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElRetrievalEl {
    pub fn build(self) -> BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalEl {
            service_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `service_configuration` after provisioning.\n"]
    pub fn service_configuration(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElServiceConfigurationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationElS3El {
    bucket_name: PrimField<String>,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationElS3El {}

impl ToListMappable
    for BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationElS3El
{
    type O = BlockAssignable<
        BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationElS3El,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationElS3El
{
    #[doc = ""]
    pub bucket_name: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationElS3El {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationElS3El {
        BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationElS3El {
            bucket_name: self.bucket_name,
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationElS3ElRef {
    shared: StackShared,
    base: String,
}

impl Ref
    for BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationElS3ElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationElS3ElRef
    {
        BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationElS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationElDynamic {
    s3: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationElS3El,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<
        Vec<BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationElS3El>,
    >,
    dynamic:
        BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationElDynamic,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationEl {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationElS3El,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationEl
{
    type O = BlockAssignable<
        BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationEl {
}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationEl {
            s3: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(
        &self,
    ) -> ListRef<
        BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationElS3ElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElNodeElConfigurationElStorageElDynamic {
    service_configuration: Option<
        DynamicBlock<
            BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationElStorageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    service_configuration: Option<
        Vec<BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationEl>,
    >,
    dynamic: BedrockagentFlowDefinitionElNodeElConfigurationElStorageElDynamic,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElStorageEl {
    #[doc = "Set the field `service_configuration`.\n"]
    pub fn set_service_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationElStorageEl {
    type O = BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElStorageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationElStorageEl {}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationElStorageEl {
    pub fn build(self) -> BedrockagentFlowDefinitionElNodeElConfigurationElStorageEl {
        BedrockagentFlowDefinitionElNodeElConfigurationElStorageEl {
            service_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElStorageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElStorageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElStorageElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElStorageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElStorageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `service_configuration` after provisioning.\n"]
    pub fn service_configuration(
        &self,
    ) -> ListRef<BedrockagentFlowDefinitionElNodeElConfigurationElStorageElServiceConfigurationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_configuration", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElNodeElConfigurationElDynamic {
    agent: Option<DynamicBlock<BedrockagentFlowDefinitionElNodeElConfigurationElAgentEl>>,
    collector: Option<DynamicBlock<BedrockagentFlowDefinitionElNodeElConfigurationElCollectorEl>>,
    condition: Option<DynamicBlock<BedrockagentFlowDefinitionElNodeElConfigurationElConditionEl>>,
    inline_code:
        Option<DynamicBlock<BedrockagentFlowDefinitionElNodeElConfigurationElInlineCodeEl>>,
    input: Option<DynamicBlock<BedrockagentFlowDefinitionElNodeElConfigurationElInputEl>>,
    iterator: Option<DynamicBlock<BedrockagentFlowDefinitionElNodeElConfigurationElIteratorEl>>,
    knowledge_base:
        Option<DynamicBlock<BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseEl>>,
    lambda_function:
        Option<DynamicBlock<BedrockagentFlowDefinitionElNodeElConfigurationElLambdaFunctionEl>>,
    lex: Option<DynamicBlock<BedrockagentFlowDefinitionElNodeElConfigurationElLexEl>>,
    output: Option<DynamicBlock<BedrockagentFlowDefinitionElNodeElConfigurationElOutputEl>>,
    prompt: Option<DynamicBlock<BedrockagentFlowDefinitionElNodeElConfigurationElPromptEl>>,
    retrieval: Option<DynamicBlock<BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalEl>>,
    storage: Option<DynamicBlock<BedrockagentFlowDefinitionElNodeElConfigurationElStorageEl>>,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    agent: Option<Vec<BedrockagentFlowDefinitionElNodeElConfigurationElAgentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collector: Option<Vec<BedrockagentFlowDefinitionElNodeElConfigurationElCollectorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<BedrockagentFlowDefinitionElNodeElConfigurationElConditionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_code: Option<Vec<BedrockagentFlowDefinitionElNodeElConfigurationElInlineCodeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<Vec<BedrockagentFlowDefinitionElNodeElConfigurationElInputEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iterator: Option<Vec<BedrockagentFlowDefinitionElNodeElConfigurationElIteratorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    knowledge_base: Option<Vec<BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_function: Option<Vec<BedrockagentFlowDefinitionElNodeElConfigurationElLambdaFunctionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lex: Option<Vec<BedrockagentFlowDefinitionElNodeElConfigurationElLexEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<Vec<BedrockagentFlowDefinitionElNodeElConfigurationElOutputEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt: Option<Vec<BedrockagentFlowDefinitionElNodeElConfigurationElPromptEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retrieval: Option<Vec<BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage: Option<Vec<BedrockagentFlowDefinitionElNodeElConfigurationElStorageEl>>,
    dynamic: BedrockagentFlowDefinitionElNodeElConfigurationElDynamic,
}

impl BedrockagentFlowDefinitionElNodeElConfigurationEl {
    #[doc = "Set the field `agent`.\n"]
    pub fn set_agent(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElAgentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.agent = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.agent = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `collector`.\n"]
    pub fn set_collector(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElCollectorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.collector = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.collector = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElConditionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `inline_code`.\n"]
    pub fn set_inline_code(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElInlineCodeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.inline_code = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.inline_code = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `input`.\n"]
    pub fn set_input(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElInputEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.input = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.input = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `iterator`.\n"]
    pub fn set_iterator(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElIteratorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.iterator = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.iterator = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `knowledge_base`.\n"]
    pub fn set_knowledge_base(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.knowledge_base = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.knowledge_base = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `lambda_function`.\n"]
    pub fn set_lambda_function(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElLambdaFunctionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lambda_function = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_function = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `lex`.\n"]
    pub fn set_lex(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElLexEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lex = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lex = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `output`.\n"]
    pub fn set_output(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElOutputEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.output = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.output = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `prompt`.\n"]
    pub fn set_prompt(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElPromptEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.prompt = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.prompt = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `retrieval`.\n"]
    pub fn set_retrieval(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.retrieval = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.retrieval = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `storage`.\n"]
    pub fn set_storage(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationElStorageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.storage = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.storage = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElConfigurationEl {
    type O = BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElConfigurationEl {}

impl BuildBedrockagentFlowDefinitionElNodeElConfigurationEl {
    pub fn build(self) -> BedrockagentFlowDefinitionElNodeElConfigurationEl {
        BedrockagentFlowDefinitionElNodeElConfigurationEl {
            agent: core::default::Default::default(),
            collector: core::default::Default::default(),
            condition: core::default::Default::default(),
            inline_code: core::default::Default::default(),
            input: core::default::Default::default(),
            iterator: core::default::Default::default(),
            knowledge_base: core::default::Default::default(),
            lambda_function: core::default::Default::default(),
            lex: core::default::Default::default(),
            output: core::default::Default::default(),
            prompt: core::default::Default::default(),
            retrieval: core::default::Default::default(),
            storage: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentFlowDefinitionElNodeElConfigurationElRef {
        BedrockagentFlowDefinitionElNodeElConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent` after provisioning.\n"]
    pub fn agent(&self) -> ListRef<BedrockagentFlowDefinitionElNodeElConfigurationElAgentElRef> {
        ListRef::new(self.shared().clone(), format!("{}.agent", self.base))
    }

    #[doc = "Get a reference to the value of field `collector` after provisioning.\n"]
    pub fn collector(
        &self,
    ) -> ListRef<BedrockagentFlowDefinitionElNodeElConfigurationElCollectorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.collector", self.base))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(
        &self,
    ) -> ListRef<BedrockagentFlowDefinitionElNodeElConfigurationElConditionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc = "Get a reference to the value of field `inline_code` after provisioning.\n"]
    pub fn inline_code(
        &self,
    ) -> ListRef<BedrockagentFlowDefinitionElNodeElConfigurationElInlineCodeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.inline_code", self.base))
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(&self) -> ListRef<BedrockagentFlowDefinitionElNodeElConfigurationElInputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.input", self.base))
    }

    #[doc = "Get a reference to the value of field `iterator` after provisioning.\n"]
    pub fn iterator(
        &self,
    ) -> ListRef<BedrockagentFlowDefinitionElNodeElConfigurationElIteratorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.iterator", self.base))
    }

    #[doc = "Get a reference to the value of field `knowledge_base` after provisioning.\n"]
    pub fn knowledge_base(
        &self,
    ) -> ListRef<BedrockagentFlowDefinitionElNodeElConfigurationElKnowledgeBaseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.knowledge_base", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `lambda_function` after provisioning.\n"]
    pub fn lambda_function(
        &self,
    ) -> ListRef<BedrockagentFlowDefinitionElNodeElConfigurationElLambdaFunctionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lambda_function", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `lex` after provisioning.\n"]
    pub fn lex(&self) -> ListRef<BedrockagentFlowDefinitionElNodeElConfigurationElLexElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lex", self.base))
    }

    #[doc = "Get a reference to the value of field `output` after provisioning.\n"]
    pub fn output(&self) -> ListRef<BedrockagentFlowDefinitionElNodeElConfigurationElOutputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.output", self.base))
    }

    #[doc = "Get a reference to the value of field `prompt` after provisioning.\n"]
    pub fn prompt(&self) -> ListRef<BedrockagentFlowDefinitionElNodeElConfigurationElPromptElRef> {
        ListRef::new(self.shared().clone(), format!("{}.prompt", self.base))
    }

    #[doc = "Get a reference to the value of field `retrieval` after provisioning.\n"]
    pub fn retrieval(
        &self,
    ) -> ListRef<BedrockagentFlowDefinitionElNodeElConfigurationElRetrievalElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retrieval", self.base))
    }

    #[doc = "Get a reference to the value of field `storage` after provisioning.\n"]
    pub fn storage(
        &self,
    ) -> ListRef<BedrockagentFlowDefinitionElNodeElConfigurationElStorageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.storage", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElInputEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<PrimField<String>>,
    expression: PrimField<String>,
    name: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl BedrockagentFlowDefinitionElNodeElInputEl {
    #[doc = "Set the field `category`.\n"]
    pub fn set_category(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.category = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElInputEl {
    type O = BlockAssignable<BedrockagentFlowDefinitionElNodeElInputEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElInputEl {
    #[doc = ""]
    pub expression: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElNodeElInputEl {
    pub fn build(self) -> BedrockagentFlowDefinitionElNodeElInputEl {
        BedrockagentFlowDefinitionElNodeElInputEl {
            category: core::default::Default::default(),
            expression: self.expression,
            name: self.name,
            type_: self.type_,
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElInputElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElInputElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentFlowDefinitionElNodeElInputElRef {
        BedrockagentFlowDefinitionElNodeElInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElInputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeElOutputEl {
    name: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl BedrockagentFlowDefinitionElNodeElOutputEl {}

impl ToListMappable for BedrockagentFlowDefinitionElNodeElOutputEl {
    type O = BlockAssignable<BedrockagentFlowDefinitionElNodeElOutputEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeElOutputEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElNodeElOutputEl {
    pub fn build(self) -> BedrockagentFlowDefinitionElNodeElOutputEl {
        BedrockagentFlowDefinitionElNodeElOutputEl {
            name: self.name,
            type_: self.type_,
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElOutputElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElOutputElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentFlowDefinitionElNodeElOutputElRef {
        BedrockagentFlowDefinitionElNodeElOutputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElOutputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElNodeElDynamic {
    configuration: Option<DynamicBlock<BedrockagentFlowDefinitionElNodeElConfigurationEl>>,
    input: Option<DynamicBlock<BedrockagentFlowDefinitionElNodeElInputEl>>,
    output: Option<DynamicBlock<BedrockagentFlowDefinitionElNodeElOutputEl>>,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionElNodeEl {
    name: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<Vec<BedrockagentFlowDefinitionElNodeElConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<Vec<BedrockagentFlowDefinitionElNodeElInputEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<Vec<BedrockagentFlowDefinitionElNodeElOutputEl>>,
    dynamic: BedrockagentFlowDefinitionElNodeElDynamic,
}

impl BedrockagentFlowDefinitionElNodeEl {
    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentFlowDefinitionElNodeElConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `input`.\n"]
    pub fn set_input(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentFlowDefinitionElNodeElInputEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.input = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.input = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `output`.\n"]
    pub fn set_output(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentFlowDefinitionElNodeElOutputEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.output = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.output = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionElNodeEl {
    type O = BlockAssignable<BedrockagentFlowDefinitionElNodeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionElNodeEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBedrockagentFlowDefinitionElNodeEl {
    pub fn build(self) -> BedrockagentFlowDefinitionElNodeEl {
        BedrockagentFlowDefinitionElNodeEl {
            name: self.name,
            type_: self.type_,
            configuration: core::default::Default::default(),
            input: core::default::Default::default(),
            output: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElNodeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElNodeElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentFlowDefinitionElNodeElRef {
        BedrockagentFlowDefinitionElNodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElNodeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<BedrockagentFlowDefinitionElNodeElConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(&self) -> ListRef<BedrockagentFlowDefinitionElNodeElInputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.input", self.base))
    }

    #[doc = "Get a reference to the value of field `output` after provisioning.\n"]
    pub fn output(&self) -> ListRef<BedrockagentFlowDefinitionElNodeElOutputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.output", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDefinitionElDynamic {
    connection: Option<DynamicBlock<BedrockagentFlowDefinitionElConnectionEl>>,
    node: Option<DynamicBlock<BedrockagentFlowDefinitionElNodeEl>>,
}

#[derive(Serialize)]
pub struct BedrockagentFlowDefinitionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection: Option<Vec<BedrockagentFlowDefinitionElConnectionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node: Option<Vec<BedrockagentFlowDefinitionElNodeEl>>,
    dynamic: BedrockagentFlowDefinitionElDynamic,
}

impl BedrockagentFlowDefinitionEl {
    #[doc = "Set the field `connection`.\n"]
    pub fn set_connection(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentFlowDefinitionElConnectionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.connection = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.connection = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `node`.\n"]
    pub fn set_node(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentFlowDefinitionElNodeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BedrockagentFlowDefinitionEl {
    type O = BlockAssignable<BedrockagentFlowDefinitionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowDefinitionEl {}

impl BuildBedrockagentFlowDefinitionEl {
    pub fn build(self) -> BedrockagentFlowDefinitionEl {
        BedrockagentFlowDefinitionEl {
            connection: core::default::Default::default(),
            node: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentFlowDefinitionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowDefinitionElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentFlowDefinitionElRef {
        BedrockagentFlowDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowDefinitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection` after provisioning.\n"]
    pub fn connection(&self) -> ListRef<BedrockagentFlowDefinitionElConnectionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.connection", self.base))
    }

    #[doc = "Get a reference to the value of field `node` after provisioning.\n"]
    pub fn node(&self) -> ListRef<BedrockagentFlowDefinitionElNodeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlowTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl BedrockagentFlowTimeoutsEl {
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

impl ToListMappable for BedrockagentFlowTimeoutsEl {
    type O = BlockAssignable<BedrockagentFlowTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlowTimeoutsEl {}

impl BuildBedrockagentFlowTimeoutsEl {
    pub fn build(self) -> BedrockagentFlowTimeoutsEl {
        BedrockagentFlowTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentFlowTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentFlowTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentFlowTimeoutsElRef {
        BedrockagentFlowTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlowTimeoutsElRef {
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
struct BedrockagentFlowDynamic {
    definition: Option<DynamicBlock<BedrockagentFlowDefinitionEl>>,
}
