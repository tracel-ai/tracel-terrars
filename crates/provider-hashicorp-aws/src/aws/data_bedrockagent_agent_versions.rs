use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataBedrockagentAgentVersionsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    agent_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_version_summaries: Option<Vec<DataBedrockagentAgentVersionsAgentVersionSummariesEl>>,
    dynamic: DataBedrockagentAgentVersionsDynamic,
}

struct DataBedrockagentAgentVersions_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBedrockagentAgentVersionsData>,
}

#[derive(Clone)]
pub struct DataBedrockagentAgentVersions(Rc<DataBedrockagentAgentVersions_>);

impl DataBedrockagentAgentVersions {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `agent_version_summaries`.\n"]
    pub fn set_agent_version_summaries(
        self,
        v: impl Into<BlockAssignable<DataBedrockagentAgentVersionsAgentVersionSummariesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().agent_version_summaries = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.agent_version_summaries = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\n"]
    pub fn agent_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_version_summaries` after provisioning.\n"]
    pub fn agent_version_summaries(&self) -> ListRef<DataBedrockagentAgentVersionsAgentVersionSummariesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.agent_version_summaries", self.extract_ref()))
    }
}

impl Referable for DataBedrockagentAgentVersions {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataBedrockagentAgentVersions { }

impl ToListMappable for DataBedrockagentAgentVersions {
    type O = ListRef<DataBedrockagentAgentVersionsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataBedrockagentAgentVersions_ {
    fn extract_datasource_type(&self) -> String {
        "aws_bedrockagent_agent_versions".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBedrockagentAgentVersions {
    pub tf_id: String,
    #[doc = ""]
    pub agent_id: PrimField<String>,
}

impl BuildDataBedrockagentAgentVersions {
    pub fn build(self, stack: &mut Stack) -> DataBedrockagentAgentVersions {
        let out = DataBedrockagentAgentVersions(Rc::new(DataBedrockagentAgentVersions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBedrockagentAgentVersionsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                agent_id: self.agent_id,
                region: core::default::Default::default(),
                agent_version_summaries: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBedrockagentAgentVersionsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBedrockagentAgentVersionsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataBedrockagentAgentVersionsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\n"]
    pub fn agent_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_version_summaries` after provisioning.\n"]
    pub fn agent_version_summaries(&self) -> ListRef<DataBedrockagentAgentVersionsAgentVersionSummariesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.agent_version_summaries", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBedrockagentAgentVersionsAgentVersionSummariesElGuardrailConfigurationEl {}

impl DataBedrockagentAgentVersionsAgentVersionSummariesElGuardrailConfigurationEl { }

impl ToListMappable for DataBedrockagentAgentVersionsAgentVersionSummariesElGuardrailConfigurationEl {
    type O = BlockAssignable<DataBedrockagentAgentVersionsAgentVersionSummariesElGuardrailConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockagentAgentVersionsAgentVersionSummariesElGuardrailConfigurationEl {}

impl BuildDataBedrockagentAgentVersionsAgentVersionSummariesElGuardrailConfigurationEl {
    pub fn build(self) -> DataBedrockagentAgentVersionsAgentVersionSummariesElGuardrailConfigurationEl {
        DataBedrockagentAgentVersionsAgentVersionSummariesElGuardrailConfigurationEl {}
    }
}

pub struct DataBedrockagentAgentVersionsAgentVersionSummariesElGuardrailConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBedrockagentAgentVersionsAgentVersionSummariesElGuardrailConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBedrockagentAgentVersionsAgentVersionSummariesElGuardrailConfigurationElRef {
        DataBedrockagentAgentVersionsAgentVersionSummariesElGuardrailConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockagentAgentVersionsAgentVersionSummariesElGuardrailConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `guardrail_identifier` after provisioning.\n"]
    pub fn guardrail_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.guardrail_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `guardrail_version` after provisioning.\n"]
    pub fn guardrail_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.guardrail_version", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataBedrockagentAgentVersionsAgentVersionSummariesElDynamic {
    guardrail_configuration: Option<
        DynamicBlock<DataBedrockagentAgentVersionsAgentVersionSummariesElGuardrailConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct DataBedrockagentAgentVersionsAgentVersionSummariesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    guardrail_configuration: Option<Vec<DataBedrockagentAgentVersionsAgentVersionSummariesElGuardrailConfigurationEl>>,
    dynamic: DataBedrockagentAgentVersionsAgentVersionSummariesElDynamic,
}

impl DataBedrockagentAgentVersionsAgentVersionSummariesEl {
    #[doc = "Set the field `guardrail_configuration`.\n"]
    pub fn set_guardrail_configuration(
        mut self,
        v: impl Into<BlockAssignable<DataBedrockagentAgentVersionsAgentVersionSummariesElGuardrailConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.guardrail_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.guardrail_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DataBedrockagentAgentVersionsAgentVersionSummariesEl {
    type O = BlockAssignable<DataBedrockagentAgentVersionsAgentVersionSummariesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockagentAgentVersionsAgentVersionSummariesEl {}

impl BuildDataBedrockagentAgentVersionsAgentVersionSummariesEl {
    pub fn build(self) -> DataBedrockagentAgentVersionsAgentVersionSummariesEl {
        DataBedrockagentAgentVersionsAgentVersionSummariesEl {
            guardrail_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DataBedrockagentAgentVersionsAgentVersionSummariesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBedrockagentAgentVersionsAgentVersionSummariesElRef {
    fn new(shared: StackShared, base: String) -> DataBedrockagentAgentVersionsAgentVersionSummariesElRef {
        DataBedrockagentAgentVersionsAgentVersionSummariesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockagentAgentVersionsAgentVersionSummariesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_name` after provisioning.\n"]
    pub fn agent_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_name", self.base))
    }

    #[doc = "Get a reference to the value of field `agent_status` after provisioning.\n"]
    pub fn agent_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_status", self.base))
    }

    #[doc = "Get a reference to the value of field `agent_version` after provisioning.\n"]
    pub fn agent_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_version", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `guardrail_configuration` after provisioning.\n"]
    pub fn guardrail_configuration(
        &self,
    ) -> ListRef<DataBedrockagentAgentVersionsAgentVersionSummariesElGuardrailConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.guardrail_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataBedrockagentAgentVersionsDynamic {
    agent_version_summaries: Option<DynamicBlock<DataBedrockagentAgentVersionsAgentVersionSummariesEl>>,
}
