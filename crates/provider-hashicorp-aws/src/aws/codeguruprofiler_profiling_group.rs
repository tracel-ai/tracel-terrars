use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CodeguruprofilerProfilingGroupData {
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
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_orchestration_config:
        Option<Vec<CodeguruprofilerProfilingGroupAgentOrchestrationConfigEl>>,
    dynamic: CodeguruprofilerProfilingGroupDynamic,
}
struct CodeguruprofilerProfilingGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CodeguruprofilerProfilingGroupData>,
}
#[derive(Clone)]
pub struct CodeguruprofilerProfilingGroup(Rc<CodeguruprofilerProfilingGroup_>);
impl CodeguruprofilerProfilingGroup {
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
    #[doc = "Set the field `compute_platform`.\n"]
    pub fn set_compute_platform(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().compute_platform = Some(v.into());
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
    #[doc = "Set the field `agent_orchestration_config`.\n"]
    pub fn set_agent_orchestration_config(
        self,
        v: impl Into<BlockAssignable<CodeguruprofilerProfilingGroupAgentOrchestrationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().agent_orchestration_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.agent_orchestration_config = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `compute_platform` after provisioning.\n"]
    pub fn compute_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_platform", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `agent_orchestration_config` after provisioning.\n"]
    pub fn agent_orchestration_config(
        &self,
    ) -> ListRef<CodeguruprofilerProfilingGroupAgentOrchestrationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.agent_orchestration_config", self.extract_ref()),
        )
    }
}
impl Referable for CodeguruprofilerProfilingGroup {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CodeguruprofilerProfilingGroup {}
impl ToListMappable for CodeguruprofilerProfilingGroup {
    type O = ListRef<CodeguruprofilerProfilingGroupRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CodeguruprofilerProfilingGroup_ {
    fn extract_resource_type(&self) -> String {
        "aws_codeguruprofiler_profiling_group".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCodeguruprofilerProfilingGroup {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildCodeguruprofilerProfilingGroup {
    pub fn build(self, stack: &mut Stack) -> CodeguruprofilerProfilingGroup {
        let out = CodeguruprofilerProfilingGroup(Rc::new(CodeguruprofilerProfilingGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CodeguruprofilerProfilingGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                compute_platform: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                agent_orchestration_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CodeguruprofilerProfilingGroupRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodeguruprofilerProfilingGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CodeguruprofilerProfilingGroupRef {
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
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_platform", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `agent_orchestration_config` after provisioning.\n"]
    pub fn agent_orchestration_config(
        &self,
    ) -> ListRef<CodeguruprofilerProfilingGroupAgentOrchestrationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.agent_orchestration_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CodeguruprofilerProfilingGroupAgentOrchestrationConfigEl {
    profiling_enabled: PrimField<bool>,
}
impl CodeguruprofilerProfilingGroupAgentOrchestrationConfigEl {}
impl ToListMappable for CodeguruprofilerProfilingGroupAgentOrchestrationConfigEl {
    type O = BlockAssignable<CodeguruprofilerProfilingGroupAgentOrchestrationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCodeguruprofilerProfilingGroupAgentOrchestrationConfigEl {
    #[doc = ""]
    pub profiling_enabled: PrimField<bool>,
}
impl BuildCodeguruprofilerProfilingGroupAgentOrchestrationConfigEl {
    pub fn build(self) -> CodeguruprofilerProfilingGroupAgentOrchestrationConfigEl {
        CodeguruprofilerProfilingGroupAgentOrchestrationConfigEl {
            profiling_enabled: self.profiling_enabled,
        }
    }
}
pub struct CodeguruprofilerProfilingGroupAgentOrchestrationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodeguruprofilerProfilingGroupAgentOrchestrationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodeguruprofilerProfilingGroupAgentOrchestrationConfigElRef {
        CodeguruprofilerProfilingGroupAgentOrchestrationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CodeguruprofilerProfilingGroupAgentOrchestrationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `profiling_enabled` after provisioning.\n"]
    pub fn profiling_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.profiling_enabled", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CodeguruprofilerProfilingGroupDynamic {
    agent_orchestration_config:
        Option<DynamicBlock<CodeguruprofilerProfilingGroupAgentOrchestrationConfigEl>>,
}
