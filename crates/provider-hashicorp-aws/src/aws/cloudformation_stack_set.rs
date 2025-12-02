use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudformationStackSetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    administration_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    call_as: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capabilities: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_role_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission_model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_body: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_deployment: Option<Vec<CloudformationStackSetAutoDeploymentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_execution: Option<Vec<CloudformationStackSetManagedExecutionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operation_preferences: Option<Vec<CloudformationStackSetOperationPreferencesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudformationStackSetTimeoutsEl>,
    dynamic: CloudformationStackSetDynamic,
}
struct CloudformationStackSet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudformationStackSetData>,
}
#[derive(Clone)]
pub struct CloudformationStackSet(Rc<CloudformationStackSet_>);
impl CloudformationStackSet {
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
    #[doc = "Set the field `administration_role_arn`.\n"]
    pub fn set_administration_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().administration_role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `call_as`.\n"]
    pub fn set_call_as(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().call_as = Some(v.into());
        self
    }
    #[doc = "Set the field `capabilities`.\n"]
    pub fn set_capabilities(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().capabilities = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `execution_role_name`.\n"]
    pub fn set_execution_role_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().execution_role_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().parameters = Some(v.into());
        self
    }
    #[doc = "Set the field `permission_model`.\n"]
    pub fn set_permission_model(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().permission_model = Some(v.into());
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
    #[doc = "Set the field `template_body`.\n"]
    pub fn set_template_body(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().template_body = Some(v.into());
        self
    }
    #[doc = "Set the field `template_url`.\n"]
    pub fn set_template_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().template_url = Some(v.into());
        self
    }
    #[doc = "Set the field `auto_deployment`.\n"]
    pub fn set_auto_deployment(
        self,
        v: impl Into<BlockAssignable<CloudformationStackSetAutoDeploymentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_deployment = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_deployment = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `managed_execution`.\n"]
    pub fn set_managed_execution(
        self,
        v: impl Into<BlockAssignable<CloudformationStackSetManagedExecutionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().managed_execution = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.managed_execution = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `operation_preferences`.\n"]
    pub fn set_operation_preferences(
        self,
        v: impl Into<BlockAssignable<CloudformationStackSetOperationPreferencesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().operation_preferences = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.operation_preferences = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CloudformationStackSetTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `administration_role_arn` after provisioning.\n"]
    pub fn administration_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.administration_role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `call_as` after provisioning.\n"]
    pub fn call_as(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.call_as", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `capabilities` after provisioning.\n"]
    pub fn capabilities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.capabilities", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_role_name` after provisioning.\n"]
    pub fn execution_role_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_role_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `permission_model` after provisioning.\n"]
    pub fn permission_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.permission_model", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stack_set_id` after provisioning.\n"]
    pub fn stack_set_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stack_set_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `template_body` after provisioning.\n"]
    pub fn template_body(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_body", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template_url` after provisioning.\n"]
    pub fn template_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_deployment` after provisioning.\n"]
    pub fn auto_deployment(&self) -> ListRef<CloudformationStackSetAutoDeploymentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_deployment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_execution` after provisioning.\n"]
    pub fn managed_execution(&self) -> ListRef<CloudformationStackSetManagedExecutionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_execution", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `operation_preferences` after provisioning.\n"]
    pub fn operation_preferences(
        &self,
    ) -> ListRef<CloudformationStackSetOperationPreferencesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.operation_preferences", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudformationStackSetTimeoutsElRef {
        CloudformationStackSetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CloudformationStackSet {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudformationStackSet {}
impl ToListMappable for CloudformationStackSet {
    type O = ListRef<CloudformationStackSetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudformationStackSet_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudformation_stack_set".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudformationStackSet {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildCloudformationStackSet {
    pub fn build(self, stack: &mut Stack) -> CloudformationStackSet {
        let out = CloudformationStackSet(Rc::new(CloudformationStackSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudformationStackSetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                administration_role_arn: core::default::Default::default(),
                call_as: core::default::Default::default(),
                capabilities: core::default::Default::default(),
                description: core::default::Default::default(),
                execution_role_name: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                parameters: core::default::Default::default(),
                permission_model: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                template_body: core::default::Default::default(),
                template_url: core::default::Default::default(),
                auto_deployment: core::default::Default::default(),
                managed_execution: core::default::Default::default(),
                operation_preferences: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudformationStackSetRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudformationStackSetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudformationStackSetRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `administration_role_arn` after provisioning.\n"]
    pub fn administration_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.administration_role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `call_as` after provisioning.\n"]
    pub fn call_as(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.call_as", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `capabilities` after provisioning.\n"]
    pub fn capabilities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.capabilities", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_role_name` after provisioning.\n"]
    pub fn execution_role_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_role_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `permission_model` after provisioning.\n"]
    pub fn permission_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.permission_model", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stack_set_id` after provisioning.\n"]
    pub fn stack_set_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stack_set_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `template_body` after provisioning.\n"]
    pub fn template_body(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_body", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template_url` after provisioning.\n"]
    pub fn template_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_deployment` after provisioning.\n"]
    pub fn auto_deployment(&self) -> ListRef<CloudformationStackSetAutoDeploymentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_deployment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_execution` after provisioning.\n"]
    pub fn managed_execution(&self) -> ListRef<CloudformationStackSetManagedExecutionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_execution", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `operation_preferences` after provisioning.\n"]
    pub fn operation_preferences(
        &self,
    ) -> ListRef<CloudformationStackSetOperationPreferencesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.operation_preferences", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudformationStackSetTimeoutsElRef {
        CloudformationStackSetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudformationStackSetAutoDeploymentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retain_stacks_on_account_removal: Option<PrimField<bool>>,
}
impl CloudformationStackSetAutoDeploymentEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `retain_stacks_on_account_removal`.\n"]
    pub fn set_retain_stacks_on_account_removal(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.retain_stacks_on_account_removal = Some(v.into());
        self
    }
}
impl ToListMappable for CloudformationStackSetAutoDeploymentEl {
    type O = BlockAssignable<CloudformationStackSetAutoDeploymentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudformationStackSetAutoDeploymentEl {}
impl BuildCloudformationStackSetAutoDeploymentEl {
    pub fn build(self) -> CloudformationStackSetAutoDeploymentEl {
        CloudformationStackSetAutoDeploymentEl {
            enabled: core::default::Default::default(),
            retain_stacks_on_account_removal: core::default::Default::default(),
        }
    }
}
pub struct CloudformationStackSetAutoDeploymentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudformationStackSetAutoDeploymentElRef {
    fn new(shared: StackShared, base: String) -> CloudformationStackSetAutoDeploymentElRef {
        CloudformationStackSetAutoDeploymentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudformationStackSetAutoDeploymentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `retain_stacks_on_account_removal` after provisioning.\n"]
    pub fn retain_stacks_on_account_removal(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retain_stacks_on_account_removal", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudformationStackSetManagedExecutionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<PrimField<bool>>,
}
impl CloudformationStackSetManagedExecutionEl {
    #[doc = "Set the field `active`.\n"]
    pub fn set_active(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.active = Some(v.into());
        self
    }
}
impl ToListMappable for CloudformationStackSetManagedExecutionEl {
    type O = BlockAssignable<CloudformationStackSetManagedExecutionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudformationStackSetManagedExecutionEl {}
impl BuildCloudformationStackSetManagedExecutionEl {
    pub fn build(self) -> CloudformationStackSetManagedExecutionEl {
        CloudformationStackSetManagedExecutionEl {
            active: core::default::Default::default(),
        }
    }
}
pub struct CloudformationStackSetManagedExecutionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudformationStackSetManagedExecutionElRef {
    fn new(shared: StackShared, base: String) -> CloudformationStackSetManagedExecutionElRef {
        CloudformationStackSetManagedExecutionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudformationStackSetManagedExecutionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `active` after provisioning.\n"]
    pub fn active(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.active", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudformationStackSetOperationPreferencesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_tolerance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_tolerance_percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_concurrent_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_concurrent_percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region_concurrency_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region_order: Option<ListField<PrimField<String>>>,
}
impl CloudformationStackSetOperationPreferencesEl {
    #[doc = "Set the field `failure_tolerance_count`.\n"]
    pub fn set_failure_tolerance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failure_tolerance_count = Some(v.into());
        self
    }
    #[doc = "Set the field `failure_tolerance_percentage`.\n"]
    pub fn set_failure_tolerance_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failure_tolerance_percentage = Some(v.into());
        self
    }
    #[doc = "Set the field `max_concurrent_count`.\n"]
    pub fn set_max_concurrent_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_concurrent_count = Some(v.into());
        self
    }
    #[doc = "Set the field `max_concurrent_percentage`.\n"]
    pub fn set_max_concurrent_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_concurrent_percentage = Some(v.into());
        self
    }
    #[doc = "Set the field `region_concurrency_type`.\n"]
    pub fn set_region_concurrency_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region_concurrency_type = Some(v.into());
        self
    }
    #[doc = "Set the field `region_order`.\n"]
    pub fn set_region_order(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.region_order = Some(v.into());
        self
    }
}
impl ToListMappable for CloudformationStackSetOperationPreferencesEl {
    type O = BlockAssignable<CloudformationStackSetOperationPreferencesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudformationStackSetOperationPreferencesEl {}
impl BuildCloudformationStackSetOperationPreferencesEl {
    pub fn build(self) -> CloudformationStackSetOperationPreferencesEl {
        CloudformationStackSetOperationPreferencesEl {
            failure_tolerance_count: core::default::Default::default(),
            failure_tolerance_percentage: core::default::Default::default(),
            max_concurrent_count: core::default::Default::default(),
            max_concurrent_percentage: core::default::Default::default(),
            region_concurrency_type: core::default::Default::default(),
            region_order: core::default::Default::default(),
        }
    }
}
pub struct CloudformationStackSetOperationPreferencesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudformationStackSetOperationPreferencesElRef {
    fn new(shared: StackShared, base: String) -> CloudformationStackSetOperationPreferencesElRef {
        CloudformationStackSetOperationPreferencesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudformationStackSetOperationPreferencesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `failure_tolerance_count` after provisioning.\n"]
    pub fn failure_tolerance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failure_tolerance_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `failure_tolerance_percentage` after provisioning.\n"]
    pub fn failure_tolerance_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failure_tolerance_percentage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_concurrent_count` after provisioning.\n"]
    pub fn max_concurrent_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_concurrent_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_concurrent_percentage` after provisioning.\n"]
    pub fn max_concurrent_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_concurrent_percentage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `region_concurrency_type` after provisioning.\n"]
    pub fn region_concurrency_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region_concurrency_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `region_order` after provisioning.\n"]
    pub fn region_order(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.region_order", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudformationStackSetTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CloudformationStackSetTimeoutsEl {
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for CloudformationStackSetTimeoutsEl {
    type O = BlockAssignable<CloudformationStackSetTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudformationStackSetTimeoutsEl {}
impl BuildCloudformationStackSetTimeoutsEl {
    pub fn build(self) -> CloudformationStackSetTimeoutsEl {
        CloudformationStackSetTimeoutsEl {
            update: core::default::Default::default(),
        }
    }
}
pub struct CloudformationStackSetTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudformationStackSetTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CloudformationStackSetTimeoutsElRef {
        CloudformationStackSetTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudformationStackSetTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudformationStackSetDynamic {
    auto_deployment: Option<DynamicBlock<CloudformationStackSetAutoDeploymentEl>>,
    managed_execution: Option<DynamicBlock<CloudformationStackSetManagedExecutionEl>>,
    operation_preferences: Option<DynamicBlock<CloudformationStackSetOperationPreferencesEl>>,
}
