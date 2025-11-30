use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct CodepipelineData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pipeline_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    artifact_store: Option<Vec<CodepipelineArtifactStoreEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stage: Option<Vec<CodepipelineStageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger: Option<Vec<CodepipelineTriggerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variable: Option<Vec<CodepipelineVariableEl>>,
    dynamic: CodepipelineDynamic,
}

struct Codepipeline_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CodepipelineData>,
}

#[derive(Clone)]
pub struct Codepipeline(Rc<Codepipeline_>);

impl Codepipeline {
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

    #[doc = "Set the field `execution_mode`.\n"]
    pub fn set_execution_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().execution_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `pipeline_type`.\n"]
    pub fn set_pipeline_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().pipeline_type = Some(v.into());
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

    #[doc = "Set the field `artifact_store`.\n"]
    pub fn set_artifact_store(
        self,
        v: impl Into<BlockAssignable<CodepipelineArtifactStoreEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().artifact_store = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.artifact_store = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `stage`.\n"]
    pub fn set_stage(self, v: impl Into<BlockAssignable<CodepipelineStageEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stage = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stage = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `trigger`.\n"]
    pub fn set_trigger(self, v: impl Into<BlockAssignable<CodepipelineTriggerEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().trigger = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.trigger = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `variable`.\n"]
    pub fn set_variable(self, v: impl Into<BlockAssignable<CodepipelineVariableEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().variable = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.variable = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_mode` after provisioning.\n"]
    pub fn execution_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_mode", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `pipeline_type` after provisioning.\n"]
    pub fn pipeline_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pipeline_type", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `trigger_all` after provisioning.\n"]
    pub fn trigger_all(&self) -> ListRef<CodepipelineTriggerAllElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trigger_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> ListRef<CodepipelineStageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stage", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `trigger` after provisioning.\n"]
    pub fn trigger(&self) -> ListRef<CodepipelineTriggerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trigger", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `variable` after provisioning.\n"]
    pub fn variable(&self) -> ListRef<CodepipelineVariableElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.variable", self.extract_ref()),
        )
    }
}

impl Referable for Codepipeline {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for Codepipeline {}

impl ToListMappable for Codepipeline {
    type O = ListRef<CodepipelineRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Codepipeline_ {
    fn extract_resource_type(&self) -> String {
        "aws_codepipeline".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCodepipeline {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}

impl BuildCodepipeline {
    pub fn build(self, stack: &mut Stack) -> Codepipeline {
        let out = Codepipeline(Rc::new(Codepipeline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CodepipelineData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                execution_mode: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                pipeline_type: core::default::Default::default(),
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                artifact_store: core::default::Default::default(),
                stage: core::default::Default::default(),
                trigger: core::default::Default::default(),
                variable: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CodepipelineRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl CodepipelineRef {
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

    #[doc = "Get a reference to the value of field `execution_mode` after provisioning.\n"]
    pub fn execution_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_mode", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `pipeline_type` after provisioning.\n"]
    pub fn pipeline_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pipeline_type", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `trigger_all` after provisioning.\n"]
    pub fn trigger_all(&self) -> ListRef<CodepipelineTriggerAllElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trigger_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> ListRef<CodepipelineStageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stage", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `trigger` after provisioning.\n"]
    pub fn trigger(&self) -> ListRef<CodepipelineTriggerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trigger", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `variable` after provisioning.\n"]
    pub fn variable(&self) -> ListRef<CodepipelineVariableElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.variable", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct CodepipelineTriggerAllElGitConfigurationElPullRequestElBranchesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    excludes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    includes: Option<ListField<PrimField<String>>>,
}

impl CodepipelineTriggerAllElGitConfigurationElPullRequestElBranchesEl {
    #[doc = "Set the field `excludes`.\n"]
    pub fn set_excludes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.excludes = Some(v.into());
        self
    }

    #[doc = "Set the field `includes`.\n"]
    pub fn set_includes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.includes = Some(v.into());
        self
    }
}

impl ToListMappable for CodepipelineTriggerAllElGitConfigurationElPullRequestElBranchesEl {
    type O = BlockAssignable<CodepipelineTriggerAllElGitConfigurationElPullRequestElBranchesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineTriggerAllElGitConfigurationElPullRequestElBranchesEl {}

impl BuildCodepipelineTriggerAllElGitConfigurationElPullRequestElBranchesEl {
    pub fn build(self) -> CodepipelineTriggerAllElGitConfigurationElPullRequestElBranchesEl {
        CodepipelineTriggerAllElGitConfigurationElPullRequestElBranchesEl {
            excludes: core::default::Default::default(),
            includes: core::default::Default::default(),
        }
    }
}

pub struct CodepipelineTriggerAllElGitConfigurationElPullRequestElBranchesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineTriggerAllElGitConfigurationElPullRequestElBranchesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodepipelineTriggerAllElGitConfigurationElPullRequestElBranchesElRef {
        CodepipelineTriggerAllElGitConfigurationElPullRequestElBranchesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineTriggerAllElGitConfigurationElPullRequestElBranchesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `excludes` after provisioning.\n"]
    pub fn excludes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }

    #[doc = "Get a reference to the value of field `includes` after provisioning.\n"]
    pub fn includes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.includes", self.base))
    }
}

#[derive(Serialize)]
pub struct CodepipelineTriggerAllElGitConfigurationElPullRequestElFilePathsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    excludes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    includes: Option<ListField<PrimField<String>>>,
}

impl CodepipelineTriggerAllElGitConfigurationElPullRequestElFilePathsEl {
    #[doc = "Set the field `excludes`.\n"]
    pub fn set_excludes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.excludes = Some(v.into());
        self
    }

    #[doc = "Set the field `includes`.\n"]
    pub fn set_includes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.includes = Some(v.into());
        self
    }
}

impl ToListMappable for CodepipelineTriggerAllElGitConfigurationElPullRequestElFilePathsEl {
    type O = BlockAssignable<CodepipelineTriggerAllElGitConfigurationElPullRequestElFilePathsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineTriggerAllElGitConfigurationElPullRequestElFilePathsEl {}

impl BuildCodepipelineTriggerAllElGitConfigurationElPullRequestElFilePathsEl {
    pub fn build(self) -> CodepipelineTriggerAllElGitConfigurationElPullRequestElFilePathsEl {
        CodepipelineTriggerAllElGitConfigurationElPullRequestElFilePathsEl {
            excludes: core::default::Default::default(),
            includes: core::default::Default::default(),
        }
    }
}

pub struct CodepipelineTriggerAllElGitConfigurationElPullRequestElFilePathsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineTriggerAllElGitConfigurationElPullRequestElFilePathsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodepipelineTriggerAllElGitConfigurationElPullRequestElFilePathsElRef {
        CodepipelineTriggerAllElGitConfigurationElPullRequestElFilePathsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineTriggerAllElGitConfigurationElPullRequestElFilePathsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `excludes` after provisioning.\n"]
    pub fn excludes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }

    #[doc = "Get a reference to the value of field `includes` after provisioning.\n"]
    pub fn includes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.includes", self.base))
    }
}

#[derive(Serialize)]
pub struct CodepipelineTriggerAllElGitConfigurationElPullRequestEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branches: Option<ListField<CodepipelineTriggerAllElGitConfigurationElPullRequestElBranchesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    events: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_paths:
        Option<ListField<CodepipelineTriggerAllElGitConfigurationElPullRequestElFilePathsEl>>,
}

impl CodepipelineTriggerAllElGitConfigurationElPullRequestEl {
    #[doc = "Set the field `branches`.\n"]
    pub fn set_branches(
        mut self,
        v: impl Into<ListField<CodepipelineTriggerAllElGitConfigurationElPullRequestElBranchesEl>>,
    ) -> Self {
        self.branches = Some(v.into());
        self
    }

    #[doc = "Set the field `events`.\n"]
    pub fn set_events(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.events = Some(v.into());
        self
    }

    #[doc = "Set the field `file_paths`.\n"]
    pub fn set_file_paths(
        mut self,
        v: impl Into<ListField<CodepipelineTriggerAllElGitConfigurationElPullRequestElFilePathsEl>>,
    ) -> Self {
        self.file_paths = Some(v.into());
        self
    }
}

impl ToListMappable for CodepipelineTriggerAllElGitConfigurationElPullRequestEl {
    type O = BlockAssignable<CodepipelineTriggerAllElGitConfigurationElPullRequestEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineTriggerAllElGitConfigurationElPullRequestEl {}

impl BuildCodepipelineTriggerAllElGitConfigurationElPullRequestEl {
    pub fn build(self) -> CodepipelineTriggerAllElGitConfigurationElPullRequestEl {
        CodepipelineTriggerAllElGitConfigurationElPullRequestEl {
            branches: core::default::Default::default(),
            events: core::default::Default::default(),
            file_paths: core::default::Default::default(),
        }
    }
}

pub struct CodepipelineTriggerAllElGitConfigurationElPullRequestElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineTriggerAllElGitConfigurationElPullRequestElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodepipelineTriggerAllElGitConfigurationElPullRequestElRef {
        CodepipelineTriggerAllElGitConfigurationElPullRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineTriggerAllElGitConfigurationElPullRequestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branches` after provisioning.\n"]
    pub fn branches(
        &self,
    ) -> ListRef<CodepipelineTriggerAllElGitConfigurationElPullRequestElBranchesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.branches", self.base))
    }

    #[doc = "Get a reference to the value of field `events` after provisioning.\n"]
    pub fn events(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.events", self.base))
    }

    #[doc = "Get a reference to the value of field `file_paths` after provisioning.\n"]
    pub fn file_paths(
        &self,
    ) -> ListRef<CodepipelineTriggerAllElGitConfigurationElPullRequestElFilePathsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file_paths", self.base))
    }
}

#[derive(Serialize)]
pub struct CodepipelineTriggerAllElGitConfigurationElPushElBranchesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    excludes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    includes: Option<ListField<PrimField<String>>>,
}

impl CodepipelineTriggerAllElGitConfigurationElPushElBranchesEl {
    #[doc = "Set the field `excludes`.\n"]
    pub fn set_excludes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.excludes = Some(v.into());
        self
    }

    #[doc = "Set the field `includes`.\n"]
    pub fn set_includes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.includes = Some(v.into());
        self
    }
}

impl ToListMappable for CodepipelineTriggerAllElGitConfigurationElPushElBranchesEl {
    type O = BlockAssignable<CodepipelineTriggerAllElGitConfigurationElPushElBranchesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineTriggerAllElGitConfigurationElPushElBranchesEl {}

impl BuildCodepipelineTriggerAllElGitConfigurationElPushElBranchesEl {
    pub fn build(self) -> CodepipelineTriggerAllElGitConfigurationElPushElBranchesEl {
        CodepipelineTriggerAllElGitConfigurationElPushElBranchesEl {
            excludes: core::default::Default::default(),
            includes: core::default::Default::default(),
        }
    }
}

pub struct CodepipelineTriggerAllElGitConfigurationElPushElBranchesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineTriggerAllElGitConfigurationElPushElBranchesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodepipelineTriggerAllElGitConfigurationElPushElBranchesElRef {
        CodepipelineTriggerAllElGitConfigurationElPushElBranchesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineTriggerAllElGitConfigurationElPushElBranchesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `excludes` after provisioning.\n"]
    pub fn excludes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }

    #[doc = "Get a reference to the value of field `includes` after provisioning.\n"]
    pub fn includes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.includes", self.base))
    }
}

#[derive(Serialize)]
pub struct CodepipelineTriggerAllElGitConfigurationElPushElFilePathsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    excludes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    includes: Option<ListField<PrimField<String>>>,
}

impl CodepipelineTriggerAllElGitConfigurationElPushElFilePathsEl {
    #[doc = "Set the field `excludes`.\n"]
    pub fn set_excludes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.excludes = Some(v.into());
        self
    }

    #[doc = "Set the field `includes`.\n"]
    pub fn set_includes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.includes = Some(v.into());
        self
    }
}

impl ToListMappable for CodepipelineTriggerAllElGitConfigurationElPushElFilePathsEl {
    type O = BlockAssignable<CodepipelineTriggerAllElGitConfigurationElPushElFilePathsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineTriggerAllElGitConfigurationElPushElFilePathsEl {}

impl BuildCodepipelineTriggerAllElGitConfigurationElPushElFilePathsEl {
    pub fn build(self) -> CodepipelineTriggerAllElGitConfigurationElPushElFilePathsEl {
        CodepipelineTriggerAllElGitConfigurationElPushElFilePathsEl {
            excludes: core::default::Default::default(),
            includes: core::default::Default::default(),
        }
    }
}

pub struct CodepipelineTriggerAllElGitConfigurationElPushElFilePathsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineTriggerAllElGitConfigurationElPushElFilePathsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodepipelineTriggerAllElGitConfigurationElPushElFilePathsElRef {
        CodepipelineTriggerAllElGitConfigurationElPushElFilePathsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineTriggerAllElGitConfigurationElPushElFilePathsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `excludes` after provisioning.\n"]
    pub fn excludes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }

    #[doc = "Get a reference to the value of field `includes` after provisioning.\n"]
    pub fn includes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.includes", self.base))
    }
}

#[derive(Serialize)]
pub struct CodepipelineTriggerAllElGitConfigurationElPushElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    excludes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    includes: Option<ListField<PrimField<String>>>,
}

impl CodepipelineTriggerAllElGitConfigurationElPushElTagsEl {
    #[doc = "Set the field `excludes`.\n"]
    pub fn set_excludes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.excludes = Some(v.into());
        self
    }

    #[doc = "Set the field `includes`.\n"]
    pub fn set_includes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.includes = Some(v.into());
        self
    }
}

impl ToListMappable for CodepipelineTriggerAllElGitConfigurationElPushElTagsEl {
    type O = BlockAssignable<CodepipelineTriggerAllElGitConfigurationElPushElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineTriggerAllElGitConfigurationElPushElTagsEl {}

impl BuildCodepipelineTriggerAllElGitConfigurationElPushElTagsEl {
    pub fn build(self) -> CodepipelineTriggerAllElGitConfigurationElPushElTagsEl {
        CodepipelineTriggerAllElGitConfigurationElPushElTagsEl {
            excludes: core::default::Default::default(),
            includes: core::default::Default::default(),
        }
    }
}

pub struct CodepipelineTriggerAllElGitConfigurationElPushElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineTriggerAllElGitConfigurationElPushElTagsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodepipelineTriggerAllElGitConfigurationElPushElTagsElRef {
        CodepipelineTriggerAllElGitConfigurationElPushElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineTriggerAllElGitConfigurationElPushElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `excludes` after provisioning.\n"]
    pub fn excludes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }

    #[doc = "Get a reference to the value of field `includes` after provisioning.\n"]
    pub fn includes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.includes", self.base))
    }
}

#[derive(Serialize)]
pub struct CodepipelineTriggerAllElGitConfigurationElPushEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branches: Option<ListField<CodepipelineTriggerAllElGitConfigurationElPushElBranchesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_paths: Option<ListField<CodepipelineTriggerAllElGitConfigurationElPushElFilePathsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<CodepipelineTriggerAllElGitConfigurationElPushElTagsEl>>,
}

impl CodepipelineTriggerAllElGitConfigurationElPushEl {
    #[doc = "Set the field `branches`.\n"]
    pub fn set_branches(
        mut self,
        v: impl Into<ListField<CodepipelineTriggerAllElGitConfigurationElPushElBranchesEl>>,
    ) -> Self {
        self.branches = Some(v.into());
        self
    }

    #[doc = "Set the field `file_paths`.\n"]
    pub fn set_file_paths(
        mut self,
        v: impl Into<ListField<CodepipelineTriggerAllElGitConfigurationElPushElFilePathsEl>>,
    ) -> Self {
        self.file_paths = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<ListField<CodepipelineTriggerAllElGitConfigurationElPushElTagsEl>>,
    ) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for CodepipelineTriggerAllElGitConfigurationElPushEl {
    type O = BlockAssignable<CodepipelineTriggerAllElGitConfigurationElPushEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineTriggerAllElGitConfigurationElPushEl {}

impl BuildCodepipelineTriggerAllElGitConfigurationElPushEl {
    pub fn build(self) -> CodepipelineTriggerAllElGitConfigurationElPushEl {
        CodepipelineTriggerAllElGitConfigurationElPushEl {
            branches: core::default::Default::default(),
            file_paths: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct CodepipelineTriggerAllElGitConfigurationElPushElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineTriggerAllElGitConfigurationElPushElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodepipelineTriggerAllElGitConfigurationElPushElRef {
        CodepipelineTriggerAllElGitConfigurationElPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineTriggerAllElGitConfigurationElPushElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branches` after provisioning.\n"]
    pub fn branches(
        &self,
    ) -> ListRef<CodepipelineTriggerAllElGitConfigurationElPushElBranchesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.branches", self.base))
    }

    #[doc = "Get a reference to the value of field `file_paths` after provisioning.\n"]
    pub fn file_paths(
        &self,
    ) -> ListRef<CodepipelineTriggerAllElGitConfigurationElPushElFilePathsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file_paths", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<CodepipelineTriggerAllElGitConfigurationElPushElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct CodepipelineTriggerAllElGitConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pull_request: Option<ListField<CodepipelineTriggerAllElGitConfigurationElPullRequestEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push: Option<ListField<CodepipelineTriggerAllElGitConfigurationElPushEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_action_name: Option<PrimField<String>>,
}

impl CodepipelineTriggerAllElGitConfigurationEl {
    #[doc = "Set the field `pull_request`.\n"]
    pub fn set_pull_request(
        mut self,
        v: impl Into<ListField<CodepipelineTriggerAllElGitConfigurationElPullRequestEl>>,
    ) -> Self {
        self.pull_request = Some(v.into());
        self
    }

    #[doc = "Set the field `push`.\n"]
    pub fn set_push(
        mut self,
        v: impl Into<ListField<CodepipelineTriggerAllElGitConfigurationElPushEl>>,
    ) -> Self {
        self.push = Some(v.into());
        self
    }

    #[doc = "Set the field `source_action_name`.\n"]
    pub fn set_source_action_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_action_name = Some(v.into());
        self
    }
}

impl ToListMappable for CodepipelineTriggerAllElGitConfigurationEl {
    type O = BlockAssignable<CodepipelineTriggerAllElGitConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineTriggerAllElGitConfigurationEl {}

impl BuildCodepipelineTriggerAllElGitConfigurationEl {
    pub fn build(self) -> CodepipelineTriggerAllElGitConfigurationEl {
        CodepipelineTriggerAllElGitConfigurationEl {
            pull_request: core::default::Default::default(),
            push: core::default::Default::default(),
            source_action_name: core::default::Default::default(),
        }
    }
}

pub struct CodepipelineTriggerAllElGitConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineTriggerAllElGitConfigurationElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineTriggerAllElGitConfigurationElRef {
        CodepipelineTriggerAllElGitConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineTriggerAllElGitConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pull_request` after provisioning.\n"]
    pub fn pull_request(
        &self,
    ) -> ListRef<CodepipelineTriggerAllElGitConfigurationElPullRequestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pull_request", self.base))
    }

    #[doc = "Get a reference to the value of field `push` after provisioning.\n"]
    pub fn push(&self) -> ListRef<CodepipelineTriggerAllElGitConfigurationElPushElRef> {
        ListRef::new(self.shared().clone(), format!("{}.push", self.base))
    }

    #[doc = "Get a reference to the value of field `source_action_name` after provisioning.\n"]
    pub fn source_action_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_action_name", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CodepipelineTriggerAllEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    git_configuration: Option<ListField<CodepipelineTriggerAllElGitConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_type: Option<PrimField<String>>,
}

impl CodepipelineTriggerAllEl {
    #[doc = "Set the field `git_configuration`.\n"]
    pub fn set_git_configuration(
        mut self,
        v: impl Into<ListField<CodepipelineTriggerAllElGitConfigurationEl>>,
    ) -> Self {
        self.git_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `provider_type`.\n"]
    pub fn set_provider_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.provider_type = Some(v.into());
        self
    }
}

impl ToListMappable for CodepipelineTriggerAllEl {
    type O = BlockAssignable<CodepipelineTriggerAllEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineTriggerAllEl {}

impl BuildCodepipelineTriggerAllEl {
    pub fn build(self) -> CodepipelineTriggerAllEl {
        CodepipelineTriggerAllEl {
            git_configuration: core::default::Default::default(),
            provider_type: core::default::Default::default(),
        }
    }
}

pub struct CodepipelineTriggerAllElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineTriggerAllElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineTriggerAllElRef {
        CodepipelineTriggerAllElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineTriggerAllElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `git_configuration` after provisioning.\n"]
    pub fn git_configuration(&self) -> ListRef<CodepipelineTriggerAllElGitConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.git_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `provider_type` after provisioning.\n"]
    pub fn provider_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provider_type", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CodepipelineArtifactStoreElEncryptionKeyEl {
    id: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl CodepipelineArtifactStoreElEncryptionKeyEl {}

impl ToListMappable for CodepipelineArtifactStoreElEncryptionKeyEl {
    type O = BlockAssignable<CodepipelineArtifactStoreElEncryptionKeyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineArtifactStoreElEncryptionKeyEl {
    #[doc = ""]
    pub id: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildCodepipelineArtifactStoreElEncryptionKeyEl {
    pub fn build(self) -> CodepipelineArtifactStoreElEncryptionKeyEl {
        CodepipelineArtifactStoreElEncryptionKeyEl {
            id: self.id,
            type_: self.type_,
        }
    }
}

pub struct CodepipelineArtifactStoreElEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineArtifactStoreElEncryptionKeyElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineArtifactStoreElEncryptionKeyElRef {
        CodepipelineArtifactStoreElEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineArtifactStoreElEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineArtifactStoreElDynamic {
    encryption_key: Option<DynamicBlock<CodepipelineArtifactStoreElEncryptionKeyEl>>,
}

#[derive(Serialize)]
pub struct CodepipelineArtifactStoreEl {
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_key: Option<Vec<CodepipelineArtifactStoreElEncryptionKeyEl>>,
    dynamic: CodepipelineArtifactStoreElDynamic,
}

impl CodepipelineArtifactStoreEl {
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_key`.\n"]
    pub fn set_encryption_key(
        mut self,
        v: impl Into<BlockAssignable<CodepipelineArtifactStoreElEncryptionKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.encryption_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_key = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CodepipelineArtifactStoreEl {
    type O = BlockAssignable<CodepipelineArtifactStoreEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineArtifactStoreEl {
    #[doc = ""]
    pub location: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildCodepipelineArtifactStoreEl {
    pub fn build(self) -> CodepipelineArtifactStoreEl {
        CodepipelineArtifactStoreEl {
            location: self.location,
            region: core::default::Default::default(),
            type_: self.type_,
            encryption_key: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodepipelineArtifactStoreElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineArtifactStoreElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineArtifactStoreElRef {
        CodepipelineArtifactStoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineArtifactStoreElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_key` after provisioning.\n"]
    pub fn encryption_key(&self) -> ListRef<CodepipelineArtifactStoreElEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_key", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CodepipelineStageElActionEl {
    category: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_artifacts: Option<ListField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_artifacts: Option<ListField<PrimField<String>>>,
    owner: PrimField<String>,
    provider: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_order: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_in_minutes: Option<PrimField<f64>>,
    version: PrimField<String>,
}

impl CodepipelineStageElActionEl {
    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `input_artifacts`.\n"]
    pub fn set_input_artifacts(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.input_artifacts = Some(v.into());
        self
    }

    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace = Some(v.into());
        self
    }

    #[doc = "Set the field `output_artifacts`.\n"]
    pub fn set_output_artifacts(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.output_artifacts = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `run_order`.\n"]
    pub fn set_run_order(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.run_order = Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_in_minutes`.\n"]
    pub fn set_timeout_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_in_minutes = Some(v.into());
        self
    }
}

impl ToListMappable for CodepipelineStageElActionEl {
    type O = BlockAssignable<CodepipelineStageElActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineStageElActionEl {
    #[doc = ""]
    pub category: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub owner: PrimField<String>,
    #[doc = ""]
    pub provider: PrimField<String>,
    #[doc = ""]
    pub version: PrimField<String>,
}

impl BuildCodepipelineStageElActionEl {
    pub fn build(self) -> CodepipelineStageElActionEl {
        CodepipelineStageElActionEl {
            category: self.category,
            configuration: core::default::Default::default(),
            input_artifacts: core::default::Default::default(),
            name: self.name,
            namespace: core::default::Default::default(),
            output_artifacts: core::default::Default::default(),
            owner: self.owner,
            provider: self.provider,
            region: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            run_order: core::default::Default::default(),
            timeout_in_minutes: core::default::Default::default(),
            version: self.version,
        }
    }
}

pub struct CodepipelineStageElActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineStageElActionElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineStageElActionElRef {
        CodepipelineStageElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineStageElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `input_artifacts` after provisioning.\n"]
    pub fn input_artifacts(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_artifacts", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `output_artifacts` after provisioning.\n"]
    pub fn output_artifacts(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_artifacts", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }

    #[doc = "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `run_order` after provisioning.\n"]
    pub fn run_order(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.run_order", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_in_minutes` after provisioning.\n"]
    pub fn timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_in_minutes", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct CodepipelineStageElBeforeEntryElConditionElRuleElRuleTypeIdEl {
    category: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<PrimField<String>>,
    provider: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl CodepipelineStageElBeforeEntryElConditionElRuleElRuleTypeIdEl {
    #[doc = "Set the field `owner`.\n"]
    pub fn set_owner(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.owner = Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable for CodepipelineStageElBeforeEntryElConditionElRuleElRuleTypeIdEl {
    type O = BlockAssignable<CodepipelineStageElBeforeEntryElConditionElRuleElRuleTypeIdEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineStageElBeforeEntryElConditionElRuleElRuleTypeIdEl {
    #[doc = ""]
    pub category: PrimField<String>,
    #[doc = ""]
    pub provider: PrimField<String>,
}

impl BuildCodepipelineStageElBeforeEntryElConditionElRuleElRuleTypeIdEl {
    pub fn build(self) -> CodepipelineStageElBeforeEntryElConditionElRuleElRuleTypeIdEl {
        CodepipelineStageElBeforeEntryElConditionElRuleElRuleTypeIdEl {
            category: self.category,
            owner: core::default::Default::default(),
            provider: self.provider,
            version: core::default::Default::default(),
        }
    }
}

pub struct CodepipelineStageElBeforeEntryElConditionElRuleElRuleTypeIdElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineStageElBeforeEntryElConditionElRuleElRuleTypeIdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodepipelineStageElBeforeEntryElConditionElRuleElRuleTypeIdElRef {
        CodepipelineStageElBeforeEntryElConditionElRuleElRuleTypeIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineStageElBeforeEntryElConditionElRuleElRuleTypeIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }

    #[doc = "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineStageElBeforeEntryElConditionElRuleElDynamic {
    rule_type_id:
        Option<DynamicBlock<CodepipelineStageElBeforeEntryElConditionElRuleElRuleTypeIdEl>>,
}

#[derive(Serialize)]
pub struct CodepipelineStageElBeforeEntryElConditionElRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    commands: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_artifacts: Option<ListField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_in_minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_type_id: Option<Vec<CodepipelineStageElBeforeEntryElConditionElRuleElRuleTypeIdEl>>,
    dynamic: CodepipelineStageElBeforeEntryElConditionElRuleElDynamic,
}

impl CodepipelineStageElBeforeEntryElConditionElRuleEl {
    #[doc = "Set the field `commands`.\n"]
    pub fn set_commands(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.commands = Some(v.into());
        self
    }

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `input_artifacts`.\n"]
    pub fn set_input_artifacts(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.input_artifacts = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_in_minutes`.\n"]
    pub fn set_timeout_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_in_minutes = Some(v.into());
        self
    }

    #[doc = "Set the field `rule_type_id`.\n"]
    pub fn set_rule_type_id(
        mut self,
        v: impl Into<BlockAssignable<CodepipelineStageElBeforeEntryElConditionElRuleElRuleTypeIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rule_type_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rule_type_id = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CodepipelineStageElBeforeEntryElConditionElRuleEl {
    type O = BlockAssignable<CodepipelineStageElBeforeEntryElConditionElRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineStageElBeforeEntryElConditionElRuleEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildCodepipelineStageElBeforeEntryElConditionElRuleEl {
    pub fn build(self) -> CodepipelineStageElBeforeEntryElConditionElRuleEl {
        CodepipelineStageElBeforeEntryElConditionElRuleEl {
            commands: core::default::Default::default(),
            configuration: core::default::Default::default(),
            input_artifacts: core::default::Default::default(),
            name: self.name,
            region: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            timeout_in_minutes: core::default::Default::default(),
            rule_type_id: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodepipelineStageElBeforeEntryElConditionElRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineStageElBeforeEntryElConditionElRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodepipelineStageElBeforeEntryElConditionElRuleElRef {
        CodepipelineStageElBeforeEntryElConditionElRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineStageElBeforeEntryElConditionElRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `commands` after provisioning.\n"]
    pub fn commands(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.commands", self.base))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `input_artifacts` after provisioning.\n"]
    pub fn input_artifacts(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_artifacts", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_in_minutes` after provisioning.\n"]
    pub fn timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_in_minutes", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `rule_type_id` after provisioning.\n"]
    pub fn rule_type_id(
        &self,
    ) -> ListRef<CodepipelineStageElBeforeEntryElConditionElRuleElRuleTypeIdElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rule_type_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineStageElBeforeEntryElConditionElDynamic {
    rule: Option<DynamicBlock<CodepipelineStageElBeforeEntryElConditionElRuleEl>>,
}

#[derive(Serialize)]
pub struct CodepipelineStageElBeforeEntryElConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<Vec<CodepipelineStageElBeforeEntryElConditionElRuleEl>>,
    dynamic: CodepipelineStageElBeforeEntryElConditionElDynamic,
}

impl CodepipelineStageElBeforeEntryElConditionEl {
    #[doc = "Set the field `result`.\n"]
    pub fn set_result(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.result = Some(v.into());
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        mut self,
        v: impl Into<BlockAssignable<CodepipelineStageElBeforeEntryElConditionElRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rule = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CodepipelineStageElBeforeEntryElConditionEl {
    type O = BlockAssignable<CodepipelineStageElBeforeEntryElConditionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineStageElBeforeEntryElConditionEl {}

impl BuildCodepipelineStageElBeforeEntryElConditionEl {
    pub fn build(self) -> CodepipelineStageElBeforeEntryElConditionEl {
        CodepipelineStageElBeforeEntryElConditionEl {
            result: core::default::Default::default(),
            rule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodepipelineStageElBeforeEntryElConditionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineStageElBeforeEntryElConditionElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineStageElBeforeEntryElConditionElRef {
        CodepipelineStageElBeforeEntryElConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineStageElBeforeEntryElConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\n"]
    pub fn result(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.result", self.base))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<CodepipelineStageElBeforeEntryElConditionElRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineStageElBeforeEntryElDynamic {
    condition: Option<DynamicBlock<CodepipelineStageElBeforeEntryElConditionEl>>,
}

#[derive(Serialize)]
pub struct CodepipelineStageElBeforeEntryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<CodepipelineStageElBeforeEntryElConditionEl>>,
    dynamic: CodepipelineStageElBeforeEntryElDynamic,
}

impl CodepipelineStageElBeforeEntryEl {
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl Into<BlockAssignable<CodepipelineStageElBeforeEntryElConditionEl>>,
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

impl ToListMappable for CodepipelineStageElBeforeEntryEl {
    type O = BlockAssignable<CodepipelineStageElBeforeEntryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineStageElBeforeEntryEl {}

impl BuildCodepipelineStageElBeforeEntryEl {
    pub fn build(self) -> CodepipelineStageElBeforeEntryEl {
        CodepipelineStageElBeforeEntryEl {
            condition: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodepipelineStageElBeforeEntryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineStageElBeforeEntryElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineStageElBeforeEntryElRef {
        CodepipelineStageElBeforeEntryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineStageElBeforeEntryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ListRef<CodepipelineStageElBeforeEntryElConditionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }
}

#[derive(Serialize)]
pub struct CodepipelineStageElOnFailureElConditionElRuleElRuleTypeIdEl {
    category: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<PrimField<String>>,
    provider: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl CodepipelineStageElOnFailureElConditionElRuleElRuleTypeIdEl {
    #[doc = "Set the field `owner`.\n"]
    pub fn set_owner(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.owner = Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable for CodepipelineStageElOnFailureElConditionElRuleElRuleTypeIdEl {
    type O = BlockAssignable<CodepipelineStageElOnFailureElConditionElRuleElRuleTypeIdEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineStageElOnFailureElConditionElRuleElRuleTypeIdEl {
    #[doc = ""]
    pub category: PrimField<String>,
    #[doc = ""]
    pub provider: PrimField<String>,
}

impl BuildCodepipelineStageElOnFailureElConditionElRuleElRuleTypeIdEl {
    pub fn build(self) -> CodepipelineStageElOnFailureElConditionElRuleElRuleTypeIdEl {
        CodepipelineStageElOnFailureElConditionElRuleElRuleTypeIdEl {
            category: self.category,
            owner: core::default::Default::default(),
            provider: self.provider,
            version: core::default::Default::default(),
        }
    }
}

pub struct CodepipelineStageElOnFailureElConditionElRuleElRuleTypeIdElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineStageElOnFailureElConditionElRuleElRuleTypeIdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodepipelineStageElOnFailureElConditionElRuleElRuleTypeIdElRef {
        CodepipelineStageElOnFailureElConditionElRuleElRuleTypeIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineStageElOnFailureElConditionElRuleElRuleTypeIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }

    #[doc = "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineStageElOnFailureElConditionElRuleElDynamic {
    rule_type_id: Option<DynamicBlock<CodepipelineStageElOnFailureElConditionElRuleElRuleTypeIdEl>>,
}

#[derive(Serialize)]
pub struct CodepipelineStageElOnFailureElConditionElRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    commands: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_artifacts: Option<ListField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_in_minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_type_id: Option<Vec<CodepipelineStageElOnFailureElConditionElRuleElRuleTypeIdEl>>,
    dynamic: CodepipelineStageElOnFailureElConditionElRuleElDynamic,
}

impl CodepipelineStageElOnFailureElConditionElRuleEl {
    #[doc = "Set the field `commands`.\n"]
    pub fn set_commands(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.commands = Some(v.into());
        self
    }

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `input_artifacts`.\n"]
    pub fn set_input_artifacts(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.input_artifacts = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_in_minutes`.\n"]
    pub fn set_timeout_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_in_minutes = Some(v.into());
        self
    }

    #[doc = "Set the field `rule_type_id`.\n"]
    pub fn set_rule_type_id(
        mut self,
        v: impl Into<BlockAssignable<CodepipelineStageElOnFailureElConditionElRuleElRuleTypeIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rule_type_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rule_type_id = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CodepipelineStageElOnFailureElConditionElRuleEl {
    type O = BlockAssignable<CodepipelineStageElOnFailureElConditionElRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineStageElOnFailureElConditionElRuleEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildCodepipelineStageElOnFailureElConditionElRuleEl {
    pub fn build(self) -> CodepipelineStageElOnFailureElConditionElRuleEl {
        CodepipelineStageElOnFailureElConditionElRuleEl {
            commands: core::default::Default::default(),
            configuration: core::default::Default::default(),
            input_artifacts: core::default::Default::default(),
            name: self.name,
            region: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            timeout_in_minutes: core::default::Default::default(),
            rule_type_id: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodepipelineStageElOnFailureElConditionElRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineStageElOnFailureElConditionElRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodepipelineStageElOnFailureElConditionElRuleElRef {
        CodepipelineStageElOnFailureElConditionElRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineStageElOnFailureElConditionElRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `commands` after provisioning.\n"]
    pub fn commands(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.commands", self.base))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `input_artifacts` after provisioning.\n"]
    pub fn input_artifacts(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_artifacts", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_in_minutes` after provisioning.\n"]
    pub fn timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_in_minutes", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `rule_type_id` after provisioning.\n"]
    pub fn rule_type_id(
        &self,
    ) -> ListRef<CodepipelineStageElOnFailureElConditionElRuleElRuleTypeIdElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rule_type_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineStageElOnFailureElConditionElDynamic {
    rule: Option<DynamicBlock<CodepipelineStageElOnFailureElConditionElRuleEl>>,
}

#[derive(Serialize)]
pub struct CodepipelineStageElOnFailureElConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<Vec<CodepipelineStageElOnFailureElConditionElRuleEl>>,
    dynamic: CodepipelineStageElOnFailureElConditionElDynamic,
}

impl CodepipelineStageElOnFailureElConditionEl {
    #[doc = "Set the field `result`.\n"]
    pub fn set_result(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.result = Some(v.into());
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        mut self,
        v: impl Into<BlockAssignable<CodepipelineStageElOnFailureElConditionElRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rule = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CodepipelineStageElOnFailureElConditionEl {
    type O = BlockAssignable<CodepipelineStageElOnFailureElConditionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineStageElOnFailureElConditionEl {}

impl BuildCodepipelineStageElOnFailureElConditionEl {
    pub fn build(self) -> CodepipelineStageElOnFailureElConditionEl {
        CodepipelineStageElOnFailureElConditionEl {
            result: core::default::Default::default(),
            rule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodepipelineStageElOnFailureElConditionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineStageElOnFailureElConditionElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineStageElOnFailureElConditionElRef {
        CodepipelineStageElOnFailureElConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineStageElOnFailureElConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\n"]
    pub fn result(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.result", self.base))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<CodepipelineStageElOnFailureElConditionElRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rule", self.base))
    }
}

#[derive(Serialize)]
pub struct CodepipelineStageElOnFailureElRetryConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_mode: Option<PrimField<String>>,
}

impl CodepipelineStageElOnFailureElRetryConfigurationEl {
    #[doc = "Set the field `retry_mode`.\n"]
    pub fn set_retry_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.retry_mode = Some(v.into());
        self
    }
}

impl ToListMappable for CodepipelineStageElOnFailureElRetryConfigurationEl {
    type O = BlockAssignable<CodepipelineStageElOnFailureElRetryConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineStageElOnFailureElRetryConfigurationEl {}

impl BuildCodepipelineStageElOnFailureElRetryConfigurationEl {
    pub fn build(self) -> CodepipelineStageElOnFailureElRetryConfigurationEl {
        CodepipelineStageElOnFailureElRetryConfigurationEl {
            retry_mode: core::default::Default::default(),
        }
    }
}

pub struct CodepipelineStageElOnFailureElRetryConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineStageElOnFailureElRetryConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodepipelineStageElOnFailureElRetryConfigurationElRef {
        CodepipelineStageElOnFailureElRetryConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineStageElOnFailureElRetryConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `retry_mode` after provisioning.\n"]
    pub fn retry_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.retry_mode", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineStageElOnFailureElDynamic {
    condition: Option<DynamicBlock<CodepipelineStageElOnFailureElConditionEl>>,
    retry_configuration: Option<DynamicBlock<CodepipelineStageElOnFailureElRetryConfigurationEl>>,
}

#[derive(Serialize)]
pub struct CodepipelineStageElOnFailureEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<CodepipelineStageElOnFailureElConditionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_configuration: Option<Vec<CodepipelineStageElOnFailureElRetryConfigurationEl>>,
    dynamic: CodepipelineStageElOnFailureElDynamic,
}

impl CodepipelineStageElOnFailureEl {
    #[doc = "Set the field `result`.\n"]
    pub fn set_result(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.result = Some(v.into());
        self
    }

    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl Into<BlockAssignable<CodepipelineStageElOnFailureElConditionEl>>,
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

    #[doc = "Set the field `retry_configuration`.\n"]
    pub fn set_retry_configuration(
        mut self,
        v: impl Into<BlockAssignable<CodepipelineStageElOnFailureElRetryConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.retry_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.retry_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CodepipelineStageElOnFailureEl {
    type O = BlockAssignable<CodepipelineStageElOnFailureEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineStageElOnFailureEl {}

impl BuildCodepipelineStageElOnFailureEl {
    pub fn build(self) -> CodepipelineStageElOnFailureEl {
        CodepipelineStageElOnFailureEl {
            result: core::default::Default::default(),
            condition: core::default::Default::default(),
            retry_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodepipelineStageElOnFailureElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineStageElOnFailureElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineStageElOnFailureElRef {
        CodepipelineStageElOnFailureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineStageElOnFailureElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\n"]
    pub fn result(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.result", self.base))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ListRef<CodepipelineStageElOnFailureElConditionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_configuration` after provisioning.\n"]
    pub fn retry_configuration(
        &self,
    ) -> ListRef<CodepipelineStageElOnFailureElRetryConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retry_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CodepipelineStageElOnSuccessElConditionElRuleElRuleTypeIdEl {
    category: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<PrimField<String>>,
    provider: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl CodepipelineStageElOnSuccessElConditionElRuleElRuleTypeIdEl {
    #[doc = "Set the field `owner`.\n"]
    pub fn set_owner(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.owner = Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable for CodepipelineStageElOnSuccessElConditionElRuleElRuleTypeIdEl {
    type O = BlockAssignable<CodepipelineStageElOnSuccessElConditionElRuleElRuleTypeIdEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineStageElOnSuccessElConditionElRuleElRuleTypeIdEl {
    #[doc = ""]
    pub category: PrimField<String>,
    #[doc = ""]
    pub provider: PrimField<String>,
}

impl BuildCodepipelineStageElOnSuccessElConditionElRuleElRuleTypeIdEl {
    pub fn build(self) -> CodepipelineStageElOnSuccessElConditionElRuleElRuleTypeIdEl {
        CodepipelineStageElOnSuccessElConditionElRuleElRuleTypeIdEl {
            category: self.category,
            owner: core::default::Default::default(),
            provider: self.provider,
            version: core::default::Default::default(),
        }
    }
}

pub struct CodepipelineStageElOnSuccessElConditionElRuleElRuleTypeIdElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineStageElOnSuccessElConditionElRuleElRuleTypeIdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodepipelineStageElOnSuccessElConditionElRuleElRuleTypeIdElRef {
        CodepipelineStageElOnSuccessElConditionElRuleElRuleTypeIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineStageElOnSuccessElConditionElRuleElRuleTypeIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }

    #[doc = "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineStageElOnSuccessElConditionElRuleElDynamic {
    rule_type_id: Option<DynamicBlock<CodepipelineStageElOnSuccessElConditionElRuleElRuleTypeIdEl>>,
}

#[derive(Serialize)]
pub struct CodepipelineStageElOnSuccessElConditionElRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    commands: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_artifacts: Option<ListField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_in_minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_type_id: Option<Vec<CodepipelineStageElOnSuccessElConditionElRuleElRuleTypeIdEl>>,
    dynamic: CodepipelineStageElOnSuccessElConditionElRuleElDynamic,
}

impl CodepipelineStageElOnSuccessElConditionElRuleEl {
    #[doc = "Set the field `commands`.\n"]
    pub fn set_commands(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.commands = Some(v.into());
        self
    }

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `input_artifacts`.\n"]
    pub fn set_input_artifacts(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.input_artifacts = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_in_minutes`.\n"]
    pub fn set_timeout_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_in_minutes = Some(v.into());
        self
    }

    #[doc = "Set the field `rule_type_id`.\n"]
    pub fn set_rule_type_id(
        mut self,
        v: impl Into<BlockAssignable<CodepipelineStageElOnSuccessElConditionElRuleElRuleTypeIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rule_type_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rule_type_id = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CodepipelineStageElOnSuccessElConditionElRuleEl {
    type O = BlockAssignable<CodepipelineStageElOnSuccessElConditionElRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineStageElOnSuccessElConditionElRuleEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildCodepipelineStageElOnSuccessElConditionElRuleEl {
    pub fn build(self) -> CodepipelineStageElOnSuccessElConditionElRuleEl {
        CodepipelineStageElOnSuccessElConditionElRuleEl {
            commands: core::default::Default::default(),
            configuration: core::default::Default::default(),
            input_artifacts: core::default::Default::default(),
            name: self.name,
            region: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            timeout_in_minutes: core::default::Default::default(),
            rule_type_id: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodepipelineStageElOnSuccessElConditionElRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineStageElOnSuccessElConditionElRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodepipelineStageElOnSuccessElConditionElRuleElRef {
        CodepipelineStageElOnSuccessElConditionElRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineStageElOnSuccessElConditionElRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `commands` after provisioning.\n"]
    pub fn commands(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.commands", self.base))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `input_artifacts` after provisioning.\n"]
    pub fn input_artifacts(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_artifacts", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_in_minutes` after provisioning.\n"]
    pub fn timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_in_minutes", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `rule_type_id` after provisioning.\n"]
    pub fn rule_type_id(
        &self,
    ) -> ListRef<CodepipelineStageElOnSuccessElConditionElRuleElRuleTypeIdElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rule_type_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineStageElOnSuccessElConditionElDynamic {
    rule: Option<DynamicBlock<CodepipelineStageElOnSuccessElConditionElRuleEl>>,
}

#[derive(Serialize)]
pub struct CodepipelineStageElOnSuccessElConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<Vec<CodepipelineStageElOnSuccessElConditionElRuleEl>>,
    dynamic: CodepipelineStageElOnSuccessElConditionElDynamic,
}

impl CodepipelineStageElOnSuccessElConditionEl {
    #[doc = "Set the field `result`.\n"]
    pub fn set_result(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.result = Some(v.into());
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        mut self,
        v: impl Into<BlockAssignable<CodepipelineStageElOnSuccessElConditionElRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rule = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CodepipelineStageElOnSuccessElConditionEl {
    type O = BlockAssignable<CodepipelineStageElOnSuccessElConditionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineStageElOnSuccessElConditionEl {}

impl BuildCodepipelineStageElOnSuccessElConditionEl {
    pub fn build(self) -> CodepipelineStageElOnSuccessElConditionEl {
        CodepipelineStageElOnSuccessElConditionEl {
            result: core::default::Default::default(),
            rule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodepipelineStageElOnSuccessElConditionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineStageElOnSuccessElConditionElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineStageElOnSuccessElConditionElRef {
        CodepipelineStageElOnSuccessElConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineStageElOnSuccessElConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\n"]
    pub fn result(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.result", self.base))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<CodepipelineStageElOnSuccessElConditionElRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineStageElOnSuccessElDynamic {
    condition: Option<DynamicBlock<CodepipelineStageElOnSuccessElConditionEl>>,
}

#[derive(Serialize)]
pub struct CodepipelineStageElOnSuccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<CodepipelineStageElOnSuccessElConditionEl>>,
    dynamic: CodepipelineStageElOnSuccessElDynamic,
}

impl CodepipelineStageElOnSuccessEl {
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl Into<BlockAssignable<CodepipelineStageElOnSuccessElConditionEl>>,
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

impl ToListMappable for CodepipelineStageElOnSuccessEl {
    type O = BlockAssignable<CodepipelineStageElOnSuccessEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineStageElOnSuccessEl {}

impl BuildCodepipelineStageElOnSuccessEl {
    pub fn build(self) -> CodepipelineStageElOnSuccessEl {
        CodepipelineStageElOnSuccessEl {
            condition: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodepipelineStageElOnSuccessElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineStageElOnSuccessElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineStageElOnSuccessElRef {
        CodepipelineStageElOnSuccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineStageElOnSuccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ListRef<CodepipelineStageElOnSuccessElConditionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineStageElDynamic {
    action: Option<DynamicBlock<CodepipelineStageElActionEl>>,
    before_entry: Option<DynamicBlock<CodepipelineStageElBeforeEntryEl>>,
    on_failure: Option<DynamicBlock<CodepipelineStageElOnFailureEl>>,
    on_success: Option<DynamicBlock<CodepipelineStageElOnSuccessEl>>,
}

#[derive(Serialize)]
pub struct CodepipelineStageEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<CodepipelineStageElActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before_entry: Option<Vec<CodepipelineStageElBeforeEntryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_failure: Option<Vec<CodepipelineStageElOnFailureEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_success: Option<Vec<CodepipelineStageElOnSuccessEl>>,
    dynamic: CodepipelineStageElDynamic,
}

impl CodepipelineStageEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl Into<BlockAssignable<CodepipelineStageElActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `before_entry`.\n"]
    pub fn set_before_entry(
        mut self,
        v: impl Into<BlockAssignable<CodepipelineStageElBeforeEntryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.before_entry = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.before_entry = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `on_failure`.\n"]
    pub fn set_on_failure(
        mut self,
        v: impl Into<BlockAssignable<CodepipelineStageElOnFailureEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.on_failure = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.on_failure = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `on_success`.\n"]
    pub fn set_on_success(
        mut self,
        v: impl Into<BlockAssignable<CodepipelineStageElOnSuccessEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.on_success = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.on_success = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CodepipelineStageEl {
    type O = BlockAssignable<CodepipelineStageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineStageEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildCodepipelineStageEl {
    pub fn build(self) -> CodepipelineStageEl {
        CodepipelineStageEl {
            name: self.name,
            action: core::default::Default::default(),
            before_entry: core::default::Default::default(),
            on_failure: core::default::Default::default(),
            on_success: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodepipelineStageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineStageElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineStageElRef {
        CodepipelineStageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineStageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<CodepipelineStageElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `before_entry` after provisioning.\n"]
    pub fn before_entry(&self) -> ListRef<CodepipelineStageElBeforeEntryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.before_entry", self.base))
    }

    #[doc = "Get a reference to the value of field `on_failure` after provisioning.\n"]
    pub fn on_failure(&self) -> ListRef<CodepipelineStageElOnFailureElRef> {
        ListRef::new(self.shared().clone(), format!("{}.on_failure", self.base))
    }

    #[doc = "Get a reference to the value of field `on_success` after provisioning.\n"]
    pub fn on_success(&self) -> ListRef<CodepipelineStageElOnSuccessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.on_success", self.base))
    }
}

#[derive(Serialize)]
pub struct CodepipelineTriggerElGitConfigurationElPullRequestElBranchesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    excludes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    includes: Option<ListField<PrimField<String>>>,
}

impl CodepipelineTriggerElGitConfigurationElPullRequestElBranchesEl {
    #[doc = "Set the field `excludes`.\n"]
    pub fn set_excludes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.excludes = Some(v.into());
        self
    }

    #[doc = "Set the field `includes`.\n"]
    pub fn set_includes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.includes = Some(v.into());
        self
    }
}

impl ToListMappable for CodepipelineTriggerElGitConfigurationElPullRequestElBranchesEl {
    type O = BlockAssignable<CodepipelineTriggerElGitConfigurationElPullRequestElBranchesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineTriggerElGitConfigurationElPullRequestElBranchesEl {}

impl BuildCodepipelineTriggerElGitConfigurationElPullRequestElBranchesEl {
    pub fn build(self) -> CodepipelineTriggerElGitConfigurationElPullRequestElBranchesEl {
        CodepipelineTriggerElGitConfigurationElPullRequestElBranchesEl {
            excludes: core::default::Default::default(),
            includes: core::default::Default::default(),
        }
    }
}

pub struct CodepipelineTriggerElGitConfigurationElPullRequestElBranchesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineTriggerElGitConfigurationElPullRequestElBranchesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodepipelineTriggerElGitConfigurationElPullRequestElBranchesElRef {
        CodepipelineTriggerElGitConfigurationElPullRequestElBranchesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineTriggerElGitConfigurationElPullRequestElBranchesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `excludes` after provisioning.\n"]
    pub fn excludes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }

    #[doc = "Get a reference to the value of field `includes` after provisioning.\n"]
    pub fn includes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.includes", self.base))
    }
}

#[derive(Serialize)]
pub struct CodepipelineTriggerElGitConfigurationElPullRequestElFilePathsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    excludes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    includes: Option<ListField<PrimField<String>>>,
}

impl CodepipelineTriggerElGitConfigurationElPullRequestElFilePathsEl {
    #[doc = "Set the field `excludes`.\n"]
    pub fn set_excludes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.excludes = Some(v.into());
        self
    }

    #[doc = "Set the field `includes`.\n"]
    pub fn set_includes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.includes = Some(v.into());
        self
    }
}

impl ToListMappable for CodepipelineTriggerElGitConfigurationElPullRequestElFilePathsEl {
    type O = BlockAssignable<CodepipelineTriggerElGitConfigurationElPullRequestElFilePathsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineTriggerElGitConfigurationElPullRequestElFilePathsEl {}

impl BuildCodepipelineTriggerElGitConfigurationElPullRequestElFilePathsEl {
    pub fn build(self) -> CodepipelineTriggerElGitConfigurationElPullRequestElFilePathsEl {
        CodepipelineTriggerElGitConfigurationElPullRequestElFilePathsEl {
            excludes: core::default::Default::default(),
            includes: core::default::Default::default(),
        }
    }
}

pub struct CodepipelineTriggerElGitConfigurationElPullRequestElFilePathsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineTriggerElGitConfigurationElPullRequestElFilePathsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodepipelineTriggerElGitConfigurationElPullRequestElFilePathsElRef {
        CodepipelineTriggerElGitConfigurationElPullRequestElFilePathsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineTriggerElGitConfigurationElPullRequestElFilePathsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `excludes` after provisioning.\n"]
    pub fn excludes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }

    #[doc = "Get a reference to the value of field `includes` after provisioning.\n"]
    pub fn includes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.includes", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineTriggerElGitConfigurationElPullRequestElDynamic {
    branches: Option<DynamicBlock<CodepipelineTriggerElGitConfigurationElPullRequestElBranchesEl>>,
    file_paths:
        Option<DynamicBlock<CodepipelineTriggerElGitConfigurationElPullRequestElFilePathsEl>>,
}

#[derive(Serialize)]
pub struct CodepipelineTriggerElGitConfigurationElPullRequestEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    events: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branches: Option<Vec<CodepipelineTriggerElGitConfigurationElPullRequestElBranchesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_paths: Option<Vec<CodepipelineTriggerElGitConfigurationElPullRequestElFilePathsEl>>,
    dynamic: CodepipelineTriggerElGitConfigurationElPullRequestElDynamic,
}

impl CodepipelineTriggerElGitConfigurationElPullRequestEl {
    #[doc = "Set the field `events`.\n"]
    pub fn set_events(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.events = Some(v.into());
        self
    }

    #[doc = "Set the field `branches`.\n"]
    pub fn set_branches(
        mut self,
        v: impl Into<BlockAssignable<CodepipelineTriggerElGitConfigurationElPullRequestElBranchesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.branches = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.branches = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `file_paths`.\n"]
    pub fn set_file_paths(
        mut self,
        v: impl Into<BlockAssignable<CodepipelineTriggerElGitConfigurationElPullRequestElFilePathsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.file_paths = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.file_paths = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CodepipelineTriggerElGitConfigurationElPullRequestEl {
    type O = BlockAssignable<CodepipelineTriggerElGitConfigurationElPullRequestEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineTriggerElGitConfigurationElPullRequestEl {}

impl BuildCodepipelineTriggerElGitConfigurationElPullRequestEl {
    pub fn build(self) -> CodepipelineTriggerElGitConfigurationElPullRequestEl {
        CodepipelineTriggerElGitConfigurationElPullRequestEl {
            events: core::default::Default::default(),
            branches: core::default::Default::default(),
            file_paths: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodepipelineTriggerElGitConfigurationElPullRequestElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineTriggerElGitConfigurationElPullRequestElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodepipelineTriggerElGitConfigurationElPullRequestElRef {
        CodepipelineTriggerElGitConfigurationElPullRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineTriggerElGitConfigurationElPullRequestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `events` after provisioning.\n"]
    pub fn events(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.events", self.base))
    }

    #[doc = "Get a reference to the value of field `branches` after provisioning.\n"]
    pub fn branches(
        &self,
    ) -> ListRef<CodepipelineTriggerElGitConfigurationElPullRequestElBranchesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.branches", self.base))
    }

    #[doc = "Get a reference to the value of field `file_paths` after provisioning.\n"]
    pub fn file_paths(
        &self,
    ) -> ListRef<CodepipelineTriggerElGitConfigurationElPullRequestElFilePathsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file_paths", self.base))
    }
}

#[derive(Serialize)]
pub struct CodepipelineTriggerElGitConfigurationElPushElBranchesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    excludes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    includes: Option<ListField<PrimField<String>>>,
}

impl CodepipelineTriggerElGitConfigurationElPushElBranchesEl {
    #[doc = "Set the field `excludes`.\n"]
    pub fn set_excludes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.excludes = Some(v.into());
        self
    }

    #[doc = "Set the field `includes`.\n"]
    pub fn set_includes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.includes = Some(v.into());
        self
    }
}

impl ToListMappable for CodepipelineTriggerElGitConfigurationElPushElBranchesEl {
    type O = BlockAssignable<CodepipelineTriggerElGitConfigurationElPushElBranchesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineTriggerElGitConfigurationElPushElBranchesEl {}

impl BuildCodepipelineTriggerElGitConfigurationElPushElBranchesEl {
    pub fn build(self) -> CodepipelineTriggerElGitConfigurationElPushElBranchesEl {
        CodepipelineTriggerElGitConfigurationElPushElBranchesEl {
            excludes: core::default::Default::default(),
            includes: core::default::Default::default(),
        }
    }
}

pub struct CodepipelineTriggerElGitConfigurationElPushElBranchesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineTriggerElGitConfigurationElPushElBranchesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodepipelineTriggerElGitConfigurationElPushElBranchesElRef {
        CodepipelineTriggerElGitConfigurationElPushElBranchesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineTriggerElGitConfigurationElPushElBranchesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `excludes` after provisioning.\n"]
    pub fn excludes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }

    #[doc = "Get a reference to the value of field `includes` after provisioning.\n"]
    pub fn includes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.includes", self.base))
    }
}

#[derive(Serialize)]
pub struct CodepipelineTriggerElGitConfigurationElPushElFilePathsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    excludes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    includes: Option<ListField<PrimField<String>>>,
}

impl CodepipelineTriggerElGitConfigurationElPushElFilePathsEl {
    #[doc = "Set the field `excludes`.\n"]
    pub fn set_excludes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.excludes = Some(v.into());
        self
    }

    #[doc = "Set the field `includes`.\n"]
    pub fn set_includes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.includes = Some(v.into());
        self
    }
}

impl ToListMappable for CodepipelineTriggerElGitConfigurationElPushElFilePathsEl {
    type O = BlockAssignable<CodepipelineTriggerElGitConfigurationElPushElFilePathsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineTriggerElGitConfigurationElPushElFilePathsEl {}

impl BuildCodepipelineTriggerElGitConfigurationElPushElFilePathsEl {
    pub fn build(self) -> CodepipelineTriggerElGitConfigurationElPushElFilePathsEl {
        CodepipelineTriggerElGitConfigurationElPushElFilePathsEl {
            excludes: core::default::Default::default(),
            includes: core::default::Default::default(),
        }
    }
}

pub struct CodepipelineTriggerElGitConfigurationElPushElFilePathsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineTriggerElGitConfigurationElPushElFilePathsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodepipelineTriggerElGitConfigurationElPushElFilePathsElRef {
        CodepipelineTriggerElGitConfigurationElPushElFilePathsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineTriggerElGitConfigurationElPushElFilePathsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `excludes` after provisioning.\n"]
    pub fn excludes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }

    #[doc = "Get a reference to the value of field `includes` after provisioning.\n"]
    pub fn includes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.includes", self.base))
    }
}

#[derive(Serialize)]
pub struct CodepipelineTriggerElGitConfigurationElPushElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    excludes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    includes: Option<ListField<PrimField<String>>>,
}

impl CodepipelineTriggerElGitConfigurationElPushElTagsEl {
    #[doc = "Set the field `excludes`.\n"]
    pub fn set_excludes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.excludes = Some(v.into());
        self
    }

    #[doc = "Set the field `includes`.\n"]
    pub fn set_includes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.includes = Some(v.into());
        self
    }
}

impl ToListMappable for CodepipelineTriggerElGitConfigurationElPushElTagsEl {
    type O = BlockAssignable<CodepipelineTriggerElGitConfigurationElPushElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineTriggerElGitConfigurationElPushElTagsEl {}

impl BuildCodepipelineTriggerElGitConfigurationElPushElTagsEl {
    pub fn build(self) -> CodepipelineTriggerElGitConfigurationElPushElTagsEl {
        CodepipelineTriggerElGitConfigurationElPushElTagsEl {
            excludes: core::default::Default::default(),
            includes: core::default::Default::default(),
        }
    }
}

pub struct CodepipelineTriggerElGitConfigurationElPushElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineTriggerElGitConfigurationElPushElTagsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodepipelineTriggerElGitConfigurationElPushElTagsElRef {
        CodepipelineTriggerElGitConfigurationElPushElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineTriggerElGitConfigurationElPushElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `excludes` after provisioning.\n"]
    pub fn excludes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }

    #[doc = "Get a reference to the value of field `includes` after provisioning.\n"]
    pub fn includes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.includes", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineTriggerElGitConfigurationElPushElDynamic {
    branches: Option<DynamicBlock<CodepipelineTriggerElGitConfigurationElPushElBranchesEl>>,
    file_paths: Option<DynamicBlock<CodepipelineTriggerElGitConfigurationElPushElFilePathsEl>>,
    tags: Option<DynamicBlock<CodepipelineTriggerElGitConfigurationElPushElTagsEl>>,
}

#[derive(Serialize)]
pub struct CodepipelineTriggerElGitConfigurationElPushEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branches: Option<Vec<CodepipelineTriggerElGitConfigurationElPushElBranchesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_paths: Option<Vec<CodepipelineTriggerElGitConfigurationElPushElFilePathsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<CodepipelineTriggerElGitConfigurationElPushElTagsEl>>,
    dynamic: CodepipelineTriggerElGitConfigurationElPushElDynamic,
}

impl CodepipelineTriggerElGitConfigurationElPushEl {
    #[doc = "Set the field `branches`.\n"]
    pub fn set_branches(
        mut self,
        v: impl Into<BlockAssignable<CodepipelineTriggerElGitConfigurationElPushElBranchesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.branches = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.branches = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `file_paths`.\n"]
    pub fn set_file_paths(
        mut self,
        v: impl Into<BlockAssignable<CodepipelineTriggerElGitConfigurationElPushElFilePathsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.file_paths = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.file_paths = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<BlockAssignable<CodepipelineTriggerElGitConfigurationElPushElTagsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tags = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CodepipelineTriggerElGitConfigurationElPushEl {
    type O = BlockAssignable<CodepipelineTriggerElGitConfigurationElPushEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineTriggerElGitConfigurationElPushEl {}

impl BuildCodepipelineTriggerElGitConfigurationElPushEl {
    pub fn build(self) -> CodepipelineTriggerElGitConfigurationElPushEl {
        CodepipelineTriggerElGitConfigurationElPushEl {
            branches: core::default::Default::default(),
            file_paths: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodepipelineTriggerElGitConfigurationElPushElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineTriggerElGitConfigurationElPushElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineTriggerElGitConfigurationElPushElRef {
        CodepipelineTriggerElGitConfigurationElPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineTriggerElGitConfigurationElPushElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branches` after provisioning.\n"]
    pub fn branches(&self) -> ListRef<CodepipelineTriggerElGitConfigurationElPushElBranchesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.branches", self.base))
    }

    #[doc = "Get a reference to the value of field `file_paths` after provisioning.\n"]
    pub fn file_paths(
        &self,
    ) -> ListRef<CodepipelineTriggerElGitConfigurationElPushElFilePathsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file_paths", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<CodepipelineTriggerElGitConfigurationElPushElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineTriggerElGitConfigurationElDynamic {
    pull_request: Option<DynamicBlock<CodepipelineTriggerElGitConfigurationElPullRequestEl>>,
    push: Option<DynamicBlock<CodepipelineTriggerElGitConfigurationElPushEl>>,
}

#[derive(Serialize)]
pub struct CodepipelineTriggerElGitConfigurationEl {
    source_action_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pull_request: Option<Vec<CodepipelineTriggerElGitConfigurationElPullRequestEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push: Option<Vec<CodepipelineTriggerElGitConfigurationElPushEl>>,
    dynamic: CodepipelineTriggerElGitConfigurationElDynamic,
}

impl CodepipelineTriggerElGitConfigurationEl {
    #[doc = "Set the field `pull_request`.\n"]
    pub fn set_pull_request(
        mut self,
        v: impl Into<BlockAssignable<CodepipelineTriggerElGitConfigurationElPullRequestEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pull_request = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pull_request = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `push`.\n"]
    pub fn set_push(
        mut self,
        v: impl Into<BlockAssignable<CodepipelineTriggerElGitConfigurationElPushEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.push = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.push = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CodepipelineTriggerElGitConfigurationEl {
    type O = BlockAssignable<CodepipelineTriggerElGitConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineTriggerElGitConfigurationEl {
    #[doc = ""]
    pub source_action_name: PrimField<String>,
}

impl BuildCodepipelineTriggerElGitConfigurationEl {
    pub fn build(self) -> CodepipelineTriggerElGitConfigurationEl {
        CodepipelineTriggerElGitConfigurationEl {
            source_action_name: self.source_action_name,
            pull_request: core::default::Default::default(),
            push: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodepipelineTriggerElGitConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineTriggerElGitConfigurationElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineTriggerElGitConfigurationElRef {
        CodepipelineTriggerElGitConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineTriggerElGitConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_action_name` after provisioning.\n"]
    pub fn source_action_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_action_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `pull_request` after provisioning.\n"]
    pub fn pull_request(&self) -> ListRef<CodepipelineTriggerElGitConfigurationElPullRequestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pull_request", self.base))
    }

    #[doc = "Get a reference to the value of field `push` after provisioning.\n"]
    pub fn push(&self) -> ListRef<CodepipelineTriggerElGitConfigurationElPushElRef> {
        ListRef::new(self.shared().clone(), format!("{}.push", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineTriggerElDynamic {
    git_configuration: Option<DynamicBlock<CodepipelineTriggerElGitConfigurationEl>>,
}

#[derive(Serialize)]
pub struct CodepipelineTriggerEl {
    provider_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git_configuration: Option<Vec<CodepipelineTriggerElGitConfigurationEl>>,
    dynamic: CodepipelineTriggerElDynamic,
}

impl CodepipelineTriggerEl {
    #[doc = "Set the field `git_configuration`.\n"]
    pub fn set_git_configuration(
        mut self,
        v: impl Into<BlockAssignable<CodepipelineTriggerElGitConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.git_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.git_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CodepipelineTriggerEl {
    type O = BlockAssignable<CodepipelineTriggerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineTriggerEl {
    #[doc = ""]
    pub provider_type: PrimField<String>,
}

impl BuildCodepipelineTriggerEl {
    pub fn build(self) -> CodepipelineTriggerEl {
        CodepipelineTriggerEl {
            provider_type: self.provider_type,
            git_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodepipelineTriggerElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineTriggerElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineTriggerElRef {
        CodepipelineTriggerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineTriggerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `provider_type` after provisioning.\n"]
    pub fn provider_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provider_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `git_configuration` after provisioning.\n"]
    pub fn git_configuration(&self) -> ListRef<CodepipelineTriggerElGitConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.git_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CodepipelineVariableEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    name: PrimField<String>,
}

impl CodepipelineVariableEl {
    #[doc = "Set the field `default_value`.\n"]
    pub fn set_default_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_value = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
}

impl ToListMappable for CodepipelineVariableEl {
    type O = BlockAssignable<CodepipelineVariableEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineVariableEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildCodepipelineVariableEl {
    pub fn build(self) -> CodepipelineVariableEl {
        CodepipelineVariableEl {
            default_value: core::default::Default::default(),
            description: core::default::Default::default(),
            name: self.name,
        }
    }
}

pub struct CodepipelineVariableElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineVariableElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineVariableElRef {
        CodepipelineVariableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineVariableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_value", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineDynamic {
    artifact_store: Option<DynamicBlock<CodepipelineArtifactStoreEl>>,
    stage: Option<DynamicBlock<CodepipelineStageEl>>,
    trigger: Option<DynamicBlock<CodepipelineTriggerEl>>,
    variable: Option<DynamicBlock<CodepipelineVariableEl>>,
}
