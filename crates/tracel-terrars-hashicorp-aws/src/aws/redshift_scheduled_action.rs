use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct RedshiftScheduledActionData {
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
    enable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    iam_role: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    schedule: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_action: Option<Vec<RedshiftScheduledActionTargetActionEl>>,
    dynamic: RedshiftScheduledActionDynamic,
}

struct RedshiftScheduledAction_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RedshiftScheduledActionData>,
}

#[derive(Clone)]
pub struct RedshiftScheduledAction(Rc<RedshiftScheduledAction_>);

impl RedshiftScheduledAction {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `enable`.\n"]
    pub fn set_enable(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable = Some(v.into());
        self
    }

    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().end_time = Some(v.into());
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

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().start_time = Some(v.into());
        self
    }

    #[doc = "Set the field `target_action`.\n"]
    pub fn set_target_action(self, v: impl Into<BlockAssignable<RedshiftScheduledActionTargetActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_action = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_role` after provisioning.\n"]
    pub fn iam_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam_role", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_action` after provisioning.\n"]
    pub fn target_action(&self) -> ListRef<RedshiftScheduledActionTargetActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target_action", self.extract_ref()))
    }
}

impl Referable for RedshiftScheduledAction {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for RedshiftScheduledAction { }

impl ToListMappable for RedshiftScheduledAction {
    type O = ListRef<RedshiftScheduledActionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for RedshiftScheduledAction_ {
    fn extract_resource_type(&self) -> String {
        "aws_redshift_scheduled_action".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRedshiftScheduledAction {
    pub tf_id: String,
    #[doc = ""]
    pub iam_role: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub schedule: PrimField<String>,
}

impl BuildRedshiftScheduledAction {
    pub fn build(self, stack: &mut Stack) -> RedshiftScheduledAction {
        let out = RedshiftScheduledAction(Rc::new(RedshiftScheduledAction_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RedshiftScheduledActionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                enable: core::default::Default::default(),
                end_time: core::default::Default::default(),
                iam_role: self.iam_role,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                schedule: self.schedule,
                start_time: core::default::Default::default(),
                target_action: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RedshiftScheduledActionRef {
    shared: StackShared,
    base: String,
}

impl Ref for RedshiftScheduledActionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl RedshiftScheduledActionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_role` after provisioning.\n"]
    pub fn iam_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam_role", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_action` after provisioning.\n"]
    pub fn target_action(&self) -> ListRef<RedshiftScheduledActionTargetActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target_action", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RedshiftScheduledActionTargetActionElPauseClusterEl {
    cluster_identifier: PrimField<String>,
}

impl RedshiftScheduledActionTargetActionElPauseClusterEl { }

impl ToListMappable for RedshiftScheduledActionTargetActionElPauseClusterEl {
    type O = BlockAssignable<RedshiftScheduledActionTargetActionElPauseClusterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftScheduledActionTargetActionElPauseClusterEl {
    #[doc = ""]
    pub cluster_identifier: PrimField<String>,
}

impl BuildRedshiftScheduledActionTargetActionElPauseClusterEl {
    pub fn build(self) -> RedshiftScheduledActionTargetActionElPauseClusterEl {
        RedshiftScheduledActionTargetActionElPauseClusterEl { cluster_identifier: self.cluster_identifier }
    }
}

pub struct RedshiftScheduledActionTargetActionElPauseClusterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RedshiftScheduledActionTargetActionElPauseClusterElRef {
    fn new(shared: StackShared, base: String) -> RedshiftScheduledActionTargetActionElPauseClusterElRef {
        RedshiftScheduledActionTargetActionElPauseClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftScheduledActionTargetActionElPauseClusterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct RedshiftScheduledActionTargetActionElResizeClusterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    classic: Option<PrimField<bool>>,
    cluster_identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number_of_nodes: Option<PrimField<f64>>,
}

impl RedshiftScheduledActionTargetActionElResizeClusterEl {
    #[doc = "Set the field `classic`.\n"]
    pub fn set_classic(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.classic = Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_type`.\n"]
    pub fn set_cluster_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_type = Some(v.into());
        self
    }

    #[doc = "Set the field `node_type`.\n"]
    pub fn set_node_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_type = Some(v.into());
        self
    }

    #[doc = "Set the field `number_of_nodes`.\n"]
    pub fn set_number_of_nodes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.number_of_nodes = Some(v.into());
        self
    }
}

impl ToListMappable for RedshiftScheduledActionTargetActionElResizeClusterEl {
    type O = BlockAssignable<RedshiftScheduledActionTargetActionElResizeClusterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftScheduledActionTargetActionElResizeClusterEl {
    #[doc = ""]
    pub cluster_identifier: PrimField<String>,
}

impl BuildRedshiftScheduledActionTargetActionElResizeClusterEl {
    pub fn build(self) -> RedshiftScheduledActionTargetActionElResizeClusterEl {
        RedshiftScheduledActionTargetActionElResizeClusterEl {
            classic: core::default::Default::default(),
            cluster_identifier: self.cluster_identifier,
            cluster_type: core::default::Default::default(),
            node_type: core::default::Default::default(),
            number_of_nodes: core::default::Default::default(),
        }
    }
}

pub struct RedshiftScheduledActionTargetActionElResizeClusterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RedshiftScheduledActionTargetActionElResizeClusterElRef {
    fn new(shared: StackShared, base: String) -> RedshiftScheduledActionTargetActionElResizeClusterElRef {
        RedshiftScheduledActionTargetActionElResizeClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftScheduledActionTargetActionElResizeClusterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `classic` after provisioning.\n"]
    pub fn classic(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.classic", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_type` after provisioning.\n"]
    pub fn cluster_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_type", self.base))
    }

    #[doc = "Get a reference to the value of field `node_type` after provisioning.\n"]
    pub fn node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.base))
    }

    #[doc = "Get a reference to the value of field `number_of_nodes` after provisioning.\n"]
    pub fn number_of_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number_of_nodes", self.base))
    }
}

#[derive(Serialize)]
pub struct RedshiftScheduledActionTargetActionElResumeClusterEl {
    cluster_identifier: PrimField<String>,
}

impl RedshiftScheduledActionTargetActionElResumeClusterEl { }

impl ToListMappable for RedshiftScheduledActionTargetActionElResumeClusterEl {
    type O = BlockAssignable<RedshiftScheduledActionTargetActionElResumeClusterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftScheduledActionTargetActionElResumeClusterEl {
    #[doc = ""]
    pub cluster_identifier: PrimField<String>,
}

impl BuildRedshiftScheduledActionTargetActionElResumeClusterEl {
    pub fn build(self) -> RedshiftScheduledActionTargetActionElResumeClusterEl {
        RedshiftScheduledActionTargetActionElResumeClusterEl { cluster_identifier: self.cluster_identifier }
    }
}

pub struct RedshiftScheduledActionTargetActionElResumeClusterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RedshiftScheduledActionTargetActionElResumeClusterElRef {
    fn new(shared: StackShared, base: String) -> RedshiftScheduledActionTargetActionElResumeClusterElRef {
        RedshiftScheduledActionTargetActionElResumeClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftScheduledActionTargetActionElResumeClusterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier", self.base))
    }
}

#[derive(Serialize, Default)]
struct RedshiftScheduledActionTargetActionElDynamic {
    pause_cluster: Option<DynamicBlock<RedshiftScheduledActionTargetActionElPauseClusterEl>>,
    resize_cluster: Option<DynamicBlock<RedshiftScheduledActionTargetActionElResizeClusterEl>>,
    resume_cluster: Option<DynamicBlock<RedshiftScheduledActionTargetActionElResumeClusterEl>>,
}

#[derive(Serialize)]
pub struct RedshiftScheduledActionTargetActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pause_cluster: Option<Vec<RedshiftScheduledActionTargetActionElPauseClusterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resize_cluster: Option<Vec<RedshiftScheduledActionTargetActionElResizeClusterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resume_cluster: Option<Vec<RedshiftScheduledActionTargetActionElResumeClusterEl>>,
    dynamic: RedshiftScheduledActionTargetActionElDynamic,
}

impl RedshiftScheduledActionTargetActionEl {
    #[doc = "Set the field `pause_cluster`.\n"]
    pub fn set_pause_cluster(
        mut self,
        v: impl Into<BlockAssignable<RedshiftScheduledActionTargetActionElPauseClusterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pause_cluster = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pause_cluster = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resize_cluster`.\n"]
    pub fn set_resize_cluster(
        mut self,
        v: impl Into<BlockAssignable<RedshiftScheduledActionTargetActionElResizeClusterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resize_cluster = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resize_cluster = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resume_cluster`.\n"]
    pub fn set_resume_cluster(
        mut self,
        v: impl Into<BlockAssignable<RedshiftScheduledActionTargetActionElResumeClusterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resume_cluster = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resume_cluster = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for RedshiftScheduledActionTargetActionEl {
    type O = BlockAssignable<RedshiftScheduledActionTargetActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftScheduledActionTargetActionEl {}

impl BuildRedshiftScheduledActionTargetActionEl {
    pub fn build(self) -> RedshiftScheduledActionTargetActionEl {
        RedshiftScheduledActionTargetActionEl {
            pause_cluster: core::default::Default::default(),
            resize_cluster: core::default::Default::default(),
            resume_cluster: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct RedshiftScheduledActionTargetActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RedshiftScheduledActionTargetActionElRef {
    fn new(shared: StackShared, base: String) -> RedshiftScheduledActionTargetActionElRef {
        RedshiftScheduledActionTargetActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftScheduledActionTargetActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pause_cluster` after provisioning.\n"]
    pub fn pause_cluster(&self) -> ListRef<RedshiftScheduledActionTargetActionElPauseClusterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pause_cluster", self.base))
    }

    #[doc = "Get a reference to the value of field `resize_cluster` after provisioning.\n"]
    pub fn resize_cluster(&self) -> ListRef<RedshiftScheduledActionTargetActionElResizeClusterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.resize_cluster", self.base))
    }

    #[doc = "Get a reference to the value of field `resume_cluster` after provisioning.\n"]
    pub fn resume_cluster(&self) -> ListRef<RedshiftScheduledActionTargetActionElResumeClusterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.resume_cluster", self.base))
    }
}

#[derive(Serialize, Default)]
struct RedshiftScheduledActionDynamic {
    target_action: Option<DynamicBlock<RedshiftScheduledActionTargetActionEl>>,
}
