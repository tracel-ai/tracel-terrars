use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SsmMaintenanceWindowTaskData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cutoff_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_concurrency: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_errors: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_role_arn: Option<PrimField<String>>,
    task_arn: PrimField<String>,
    task_type: PrimField<String>,
    window_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    targets: Option<Vec<SsmMaintenanceWindowTaskTargetsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_invocation_parameters: Option<Vec<SsmMaintenanceWindowTaskTaskInvocationParametersEl>>,
    dynamic: SsmMaintenanceWindowTaskDynamic,
}

struct SsmMaintenanceWindowTask_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SsmMaintenanceWindowTaskData>,
}

#[derive(Clone)]
pub struct SsmMaintenanceWindowTask(Rc<SsmMaintenanceWindowTask_>);

impl SsmMaintenanceWindowTask {
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

    #[doc = "Set the field `cutoff_behavior`.\n"]
    pub fn set_cutoff_behavior(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cutoff_behavior = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `max_concurrency`.\n"]
    pub fn set_max_concurrency(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().max_concurrency = Some(v.into());
        self
    }

    #[doc = "Set the field `max_errors`.\n"]
    pub fn set_max_errors(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().max_errors = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `service_role_arn`.\n"]
    pub fn set_service_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `targets`.\n"]
    pub fn set_targets(self, v: impl Into<BlockAssignable<SsmMaintenanceWindowTaskTargetsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().targets = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.targets = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `task_invocation_parameters`.\n"]
    pub fn set_task_invocation_parameters(
        self,
        v: impl Into<BlockAssignable<SsmMaintenanceWindowTaskTaskInvocationParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().task_invocation_parameters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.task_invocation_parameters = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cutoff_behavior` after provisioning.\n"]
    pub fn cutoff_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cutoff_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_concurrency` after provisioning.\n"]
    pub fn max_concurrency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_concurrency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_errors` after provisioning.\n"]
    pub fn max_errors(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_errors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_arn` after provisioning.\n"]
    pub fn task_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_type` after provisioning.\n"]
    pub fn task_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `window_id` after provisioning.\n"]
    pub fn window_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.window_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `window_task_id` after provisioning.\n"]
    pub fn window_task_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.window_task_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `targets` after provisioning.\n"]
    pub fn targets(&self) -> ListRef<SsmMaintenanceWindowTaskTargetsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.targets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_invocation_parameters` after provisioning.\n"]
    pub fn task_invocation_parameters(&self) -> ListRef<SsmMaintenanceWindowTaskTaskInvocationParametersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.task_invocation_parameters", self.extract_ref()))
    }
}

impl Referable for SsmMaintenanceWindowTask {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for SsmMaintenanceWindowTask { }

impl ToListMappable for SsmMaintenanceWindowTask {
    type O = ListRef<SsmMaintenanceWindowTaskRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SsmMaintenanceWindowTask_ {
    fn extract_resource_type(&self) -> String {
        "aws_ssm_maintenance_window_task".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSsmMaintenanceWindowTask {
    pub tf_id: String,
    #[doc = ""]
    pub task_arn: PrimField<String>,
    #[doc = ""]
    pub task_type: PrimField<String>,
    #[doc = ""]
    pub window_id: PrimField<String>,
}

impl BuildSsmMaintenanceWindowTask {
    pub fn build(self, stack: &mut Stack) -> SsmMaintenanceWindowTask {
        let out = SsmMaintenanceWindowTask(Rc::new(SsmMaintenanceWindowTask_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SsmMaintenanceWindowTaskData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cutoff_behavior: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                max_concurrency: core::default::Default::default(),
                max_errors: core::default::Default::default(),
                name: core::default::Default::default(),
                priority: core::default::Default::default(),
                region: core::default::Default::default(),
                service_role_arn: core::default::Default::default(),
                task_arn: self.task_arn,
                task_type: self.task_type,
                window_id: self.window_id,
                targets: core::default::Default::default(),
                task_invocation_parameters: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SsmMaintenanceWindowTaskRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmMaintenanceWindowTaskRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl SsmMaintenanceWindowTaskRef {
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

    #[doc = "Get a reference to the value of field `cutoff_behavior` after provisioning.\n"]
    pub fn cutoff_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cutoff_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_concurrency` after provisioning.\n"]
    pub fn max_concurrency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_concurrency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_errors` after provisioning.\n"]
    pub fn max_errors(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_errors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_arn` after provisioning.\n"]
    pub fn task_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_type` after provisioning.\n"]
    pub fn task_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `window_id` after provisioning.\n"]
    pub fn window_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.window_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `window_task_id` after provisioning.\n"]
    pub fn window_task_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.window_task_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `targets` after provisioning.\n"]
    pub fn targets(&self) -> ListRef<SsmMaintenanceWindowTaskTargetsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.targets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_invocation_parameters` after provisioning.\n"]
    pub fn task_invocation_parameters(&self) -> ListRef<SsmMaintenanceWindowTaskTaskInvocationParametersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.task_invocation_parameters", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SsmMaintenanceWindowTaskTargetsEl {
    key: PrimField<String>,
    values: ListField<PrimField<String>>,
}

impl SsmMaintenanceWindowTaskTargetsEl { }

impl ToListMappable for SsmMaintenanceWindowTaskTargetsEl {
    type O = BlockAssignable<SsmMaintenanceWindowTaskTargetsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmMaintenanceWindowTaskTargetsEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub values: ListField<PrimField<String>>,
}

impl BuildSsmMaintenanceWindowTaskTargetsEl {
    pub fn build(self) -> SsmMaintenanceWindowTaskTargetsEl {
        SsmMaintenanceWindowTaskTargetsEl {
            key: self.key,
            values: self.values,
        }
    }
}

pub struct SsmMaintenanceWindowTaskTargetsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmMaintenanceWindowTaskTargetsElRef {
    fn new(shared: StackShared, base: String) -> SsmMaintenanceWindowTaskTargetsElRef {
        SsmMaintenanceWindowTaskTargetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmMaintenanceWindowTaskTargetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersElParameterEl {
    name: PrimField<String>,
    values: ListField<PrimField<String>>,
}

impl SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersElParameterEl { }

impl ToListMappable for SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersElParameterEl {
    type O = BlockAssignable<SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersElParameterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersElParameterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: ListField<PrimField<String>>,
}

impl BuildSsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersElParameterEl {
    pub fn build(self) -> SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersElParameterEl {
        SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersElParameterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersElParameterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersElParameterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersElParameterElRef {
        SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersElParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersElParameterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersElDynamic {
    parameter: Option<
        DynamicBlock<SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersElParameterEl>,
    >,
}

#[derive(Serialize)]
pub struct SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    document_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<Vec<SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersElParameterEl>>,
    dynamic: SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersElDynamic,
}

impl SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersEl {
    #[doc = "Set the field `document_version`.\n"]
    pub fn set_document_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.document_version = Some(v.into());
        self
    }

    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersElParameterEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parameter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parameter = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersEl {
    type O = BlockAssignable<SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersEl {}

impl BuildSsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersEl {
    pub fn build(self) -> SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersEl {
        SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersEl {
            document_version: core::default::Default::default(),
            parameter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersElRef {
        SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `document_version` after provisioning.\n"]
    pub fn document_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.document_version", self.base))
    }
}

#[derive(Serialize)]
pub struct SsmMaintenanceWindowTaskTaskInvocationParametersElLambdaParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_context: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    qualifier: Option<PrimField<String>>,
}

impl SsmMaintenanceWindowTaskTaskInvocationParametersElLambdaParametersEl {
    #[doc = "Set the field `client_context`.\n"]
    pub fn set_client_context(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_context = Some(v.into());
        self
    }

    #[doc = "Set the field `payload`.\n"]
    pub fn set_payload(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.payload = Some(v.into());
        self
    }

    #[doc = "Set the field `qualifier`.\n"]
    pub fn set_qualifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.qualifier = Some(v.into());
        self
    }
}

impl ToListMappable for SsmMaintenanceWindowTaskTaskInvocationParametersElLambdaParametersEl {
    type O = BlockAssignable<SsmMaintenanceWindowTaskTaskInvocationParametersElLambdaParametersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmMaintenanceWindowTaskTaskInvocationParametersElLambdaParametersEl {}

impl BuildSsmMaintenanceWindowTaskTaskInvocationParametersElLambdaParametersEl {
    pub fn build(self) -> SsmMaintenanceWindowTaskTaskInvocationParametersElLambdaParametersEl {
        SsmMaintenanceWindowTaskTaskInvocationParametersElLambdaParametersEl {
            client_context: core::default::Default::default(),
            payload: core::default::Default::default(),
            qualifier: core::default::Default::default(),
        }
    }
}

pub struct SsmMaintenanceWindowTaskTaskInvocationParametersElLambdaParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmMaintenanceWindowTaskTaskInvocationParametersElLambdaParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsmMaintenanceWindowTaskTaskInvocationParametersElLambdaParametersElRef {
        SsmMaintenanceWindowTaskTaskInvocationParametersElLambdaParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmMaintenanceWindowTaskTaskInvocationParametersElLambdaParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_context` after provisioning.\n"]
    pub fn client_context(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_context", self.base))
    }

    #[doc = "Get a reference to the value of field `payload` after provisioning.\n"]
    pub fn payload(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.payload", self.base))
    }

    #[doc = "Get a reference to the value of field `qualifier` after provisioning.\n"]
    pub fn qualifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.qualifier", self.base))
    }
}

#[derive(Serialize)]
pub struct SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElCloudwatchConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_log_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_output_enabled: Option<PrimField<bool>>,
}

impl SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElCloudwatchConfigEl {
    #[doc = "Set the field `cloudwatch_log_group_name`.\n"]
    pub fn set_cloudwatch_log_group_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cloudwatch_log_group_name = Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_output_enabled`.\n"]
    pub fn set_cloudwatch_output_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cloudwatch_output_enabled = Some(v.into());
        self
    }
}

impl ToListMappable for SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElCloudwatchConfigEl {
    type O =
        BlockAssignable<SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElCloudwatchConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElCloudwatchConfigEl {}

impl BuildSsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElCloudwatchConfigEl {
    pub fn build(self) -> SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElCloudwatchConfigEl {
        SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElCloudwatchConfigEl {
            cloudwatch_log_group_name: core::default::Default::default(),
            cloudwatch_output_enabled: core::default::Default::default(),
        }
    }
}

pub struct SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElCloudwatchConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElCloudwatchConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElCloudwatchConfigElRef {
        SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElCloudwatchConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElCloudwatchConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_log_group_name` after provisioning.\n"]
    pub fn cloudwatch_log_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_log_group_name", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_output_enabled` after provisioning.\n"]
    pub fn cloudwatch_output_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_output_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElNotificationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_events: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_type: Option<PrimField<String>>,
}

impl SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElNotificationConfigEl {
    #[doc = "Set the field `notification_arn`.\n"]
    pub fn set_notification_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.notification_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `notification_events`.\n"]
    pub fn set_notification_events(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.notification_events = Some(v.into());
        self
    }

    #[doc = "Set the field `notification_type`.\n"]
    pub fn set_notification_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.notification_type = Some(v.into());
        self
    }
}

impl ToListMappable for SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElNotificationConfigEl {
    type O =
        BlockAssignable<
            SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElNotificationConfigEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElNotificationConfigEl {}

impl BuildSsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElNotificationConfigEl {
    pub fn build(
        self,
    ) -> SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElNotificationConfigEl {
        SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElNotificationConfigEl {
            notification_arn: core::default::Default::default(),
            notification_events: core::default::Default::default(),
            notification_type: core::default::Default::default(),
        }
    }
}

pub struct SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElNotificationConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElNotificationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElNotificationConfigElRef {
        SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElNotificationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElNotificationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `notification_arn` after provisioning.\n"]
    pub fn notification_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.notification_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `notification_events` after provisioning.\n"]
    pub fn notification_events(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.notification_events", self.base))
    }

    #[doc = "Get a reference to the value of field `notification_type` after provisioning.\n"]
    pub fn notification_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.notification_type", self.base))
    }
}

#[derive(Serialize)]
pub struct SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElParameterEl {
    name: PrimField<String>,
    values: ListField<PrimField<String>>,
}

impl SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElParameterEl { }

impl ToListMappable for SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElParameterEl {
    type O = BlockAssignable<SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElParameterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElParameterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: ListField<PrimField<String>>,
}

impl BuildSsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElParameterEl {
    pub fn build(self) -> SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElParameterEl {
        SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElParameterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElParameterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElParameterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElParameterElRef {
        SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElParameterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElDynamic {
    cloudwatch_config: Option<
        DynamicBlock<SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElCloudwatchConfigEl>,
    >,
    notification_config: Option<
        DynamicBlock<SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElNotificationConfigEl>,
    >,
    parameter: Option<
        DynamicBlock<SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElParameterEl>,
    >,
}

#[derive(Serialize)]
pub struct SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    document_hash: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    document_hash_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    document_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_s3_bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_s3_key_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_config: Option<
        Vec<SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElCloudwatchConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_config: Option<
        Vec<SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElNotificationConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<Vec<SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElParameterEl>>,
    dynamic: SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElDynamic,
}

impl SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersEl {
    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comment = Some(v.into());
        self
    }

    #[doc = "Set the field `document_hash`.\n"]
    pub fn set_document_hash(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.document_hash = Some(v.into());
        self
    }

    #[doc = "Set the field `document_hash_type`.\n"]
    pub fn set_document_hash_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.document_hash_type = Some(v.into());
        self
    }

    #[doc = "Set the field `document_version`.\n"]
    pub fn set_document_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.document_version = Some(v.into());
        self
    }

    #[doc = "Set the field `output_s3_bucket`.\n"]
    pub fn set_output_s3_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_s3_bucket = Some(v.into());
        self
    }

    #[doc = "Set the field `output_s3_key_prefix`.\n"]
    pub fn set_output_s3_key_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_s3_key_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `service_role_arn`.\n"]
    pub fn set_service_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_seconds`.\n"]
    pub fn set_timeout_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_config`.\n"]
    pub fn set_cloudwatch_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElCloudwatchConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `notification_config`.\n"]
    pub fn set_notification_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElNotificationConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.notification_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.notification_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElParameterEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parameter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parameter = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersEl {
    type O = BlockAssignable<SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersEl {}

impl BuildSsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersEl {
    pub fn build(self) -> SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersEl {
        SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersEl {
            comment: core::default::Default::default(),
            document_hash: core::default::Default::default(),
            document_hash_type: core::default::Default::default(),
            document_version: core::default::Default::default(),
            output_s3_bucket: core::default::Default::default(),
            output_s3_key_prefix: core::default::Default::default(),
            service_role_arn: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
            cloudwatch_config: core::default::Default::default(),
            notification_config: core::default::Default::default(),
            parameter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElRef {
        SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comment", self.base))
    }

    #[doc = "Get a reference to the value of field `document_hash` after provisioning.\n"]
    pub fn document_hash(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.document_hash", self.base))
    }

    #[doc = "Get a reference to the value of field `document_hash_type` after provisioning.\n"]
    pub fn document_hash_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.document_hash_type", self.base))
    }

    #[doc = "Get a reference to the value of field `document_version` after provisioning.\n"]
    pub fn document_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.document_version", self.base))
    }

    #[doc = "Get a reference to the value of field `output_s3_bucket` after provisioning.\n"]
    pub fn output_s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_s3_bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `output_s3_key_prefix` after provisioning.\n"]
    pub fn output_s3_key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_s3_key_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\n"]
    pub fn timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_config` after provisioning.\n"]
    pub fn cloudwatch_config(
        &self,
    ) -> ListRef<SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElCloudwatchConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloudwatch_config", self.base))
    }

    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(
        &self,
    ) -> ListRef<SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElNotificationConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.notification_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SsmMaintenanceWindowTaskTaskInvocationParametersElStepFunctionsParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl SsmMaintenanceWindowTaskTaskInvocationParametersElStepFunctionsParametersEl {
    #[doc = "Set the field `input`.\n"]
    pub fn set_input(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.input = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for SsmMaintenanceWindowTaskTaskInvocationParametersElStepFunctionsParametersEl {
    type O = BlockAssignable<SsmMaintenanceWindowTaskTaskInvocationParametersElStepFunctionsParametersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmMaintenanceWindowTaskTaskInvocationParametersElStepFunctionsParametersEl {}

impl BuildSsmMaintenanceWindowTaskTaskInvocationParametersElStepFunctionsParametersEl {
    pub fn build(self) -> SsmMaintenanceWindowTaskTaskInvocationParametersElStepFunctionsParametersEl {
        SsmMaintenanceWindowTaskTaskInvocationParametersElStepFunctionsParametersEl {
            input: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct SsmMaintenanceWindowTaskTaskInvocationParametersElStepFunctionsParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmMaintenanceWindowTaskTaskInvocationParametersElStepFunctionsParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsmMaintenanceWindowTaskTaskInvocationParametersElStepFunctionsParametersElRef {
        SsmMaintenanceWindowTaskTaskInvocationParametersElStepFunctionsParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmMaintenanceWindowTaskTaskInvocationParametersElStepFunctionsParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.input", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsmMaintenanceWindowTaskTaskInvocationParametersElDynamic {
    automation_parameters: Option<
        DynamicBlock<SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersEl>,
    >,
    lambda_parameters: Option<DynamicBlock<SsmMaintenanceWindowTaskTaskInvocationParametersElLambdaParametersEl>>,
    run_command_parameters: Option<
        DynamicBlock<SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersEl>,
    >,
    step_functions_parameters: Option<
        DynamicBlock<SsmMaintenanceWindowTaskTaskInvocationParametersElStepFunctionsParametersEl>,
    >,
}

#[derive(Serialize)]
pub struct SsmMaintenanceWindowTaskTaskInvocationParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    automation_parameters: Option<Vec<SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_parameters: Option<Vec<SsmMaintenanceWindowTaskTaskInvocationParametersElLambdaParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_command_parameters: Option<Vec<SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    step_functions_parameters: Option<Vec<SsmMaintenanceWindowTaskTaskInvocationParametersElStepFunctionsParametersEl>>,
    dynamic: SsmMaintenanceWindowTaskTaskInvocationParametersElDynamic,
}

impl SsmMaintenanceWindowTaskTaskInvocationParametersEl {
    #[doc = "Set the field `automation_parameters`.\n"]
    pub fn set_automation_parameters(
        mut self,
        v: impl Into<BlockAssignable<SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.automation_parameters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.automation_parameters = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda_parameters`.\n"]
    pub fn set_lambda_parameters(
        mut self,
        v: impl Into<BlockAssignable<SsmMaintenanceWindowTaskTaskInvocationParametersElLambdaParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lambda_parameters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_parameters = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `run_command_parameters`.\n"]
    pub fn set_run_command_parameters(
        mut self,
        v: impl Into<BlockAssignable<SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.run_command_parameters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.run_command_parameters = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `step_functions_parameters`.\n"]
    pub fn set_step_functions_parameters(
        mut self,
        v: impl Into<BlockAssignable<SsmMaintenanceWindowTaskTaskInvocationParametersElStepFunctionsParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.step_functions_parameters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.step_functions_parameters = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SsmMaintenanceWindowTaskTaskInvocationParametersEl {
    type O = BlockAssignable<SsmMaintenanceWindowTaskTaskInvocationParametersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmMaintenanceWindowTaskTaskInvocationParametersEl {}

impl BuildSsmMaintenanceWindowTaskTaskInvocationParametersEl {
    pub fn build(self) -> SsmMaintenanceWindowTaskTaskInvocationParametersEl {
        SsmMaintenanceWindowTaskTaskInvocationParametersEl {
            automation_parameters: core::default::Default::default(),
            lambda_parameters: core::default::Default::default(),
            run_command_parameters: core::default::Default::default(),
            step_functions_parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SsmMaintenanceWindowTaskTaskInvocationParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmMaintenanceWindowTaskTaskInvocationParametersElRef {
    fn new(shared: StackShared, base: String) -> SsmMaintenanceWindowTaskTaskInvocationParametersElRef {
        SsmMaintenanceWindowTaskTaskInvocationParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmMaintenanceWindowTaskTaskInvocationParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `automation_parameters` after provisioning.\n"]
    pub fn automation_parameters(
        &self,
    ) -> ListRef<SsmMaintenanceWindowTaskTaskInvocationParametersElAutomationParametersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.automation_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_parameters` after provisioning.\n"]
    pub fn lambda_parameters(&self) -> ListRef<SsmMaintenanceWindowTaskTaskInvocationParametersElLambdaParametersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lambda_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `run_command_parameters` after provisioning.\n"]
    pub fn run_command_parameters(
        &self,
    ) -> ListRef<SsmMaintenanceWindowTaskTaskInvocationParametersElRunCommandParametersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.run_command_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `step_functions_parameters` after provisioning.\n"]
    pub fn step_functions_parameters(
        &self,
    ) -> ListRef<SsmMaintenanceWindowTaskTaskInvocationParametersElStepFunctionsParametersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.step_functions_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsmMaintenanceWindowTaskDynamic {
    targets: Option<DynamicBlock<SsmMaintenanceWindowTaskTargetsEl>>,
    task_invocation_parameters: Option<DynamicBlock<SsmMaintenanceWindowTaskTaskInvocationParametersEl>>,
}
