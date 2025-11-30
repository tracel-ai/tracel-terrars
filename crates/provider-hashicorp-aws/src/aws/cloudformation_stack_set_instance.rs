use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct CloudformationStackSetInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    call_as: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter_overrides: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retain_stack: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stack_set_instance_region: Option<PrimField<String>>,
    stack_set_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_targets: Option<Vec<CloudformationStackSetInstanceDeploymentTargetsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operation_preferences: Option<Vec<CloudformationStackSetInstanceOperationPreferencesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudformationStackSetInstanceTimeoutsEl>,
    dynamic: CloudformationStackSetInstanceDynamic,
}

struct CloudformationStackSetInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudformationStackSetInstanceData>,
}

#[derive(Clone)]
pub struct CloudformationStackSetInstance(Rc<CloudformationStackSetInstance_>);

impl CloudformationStackSetInstance {
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

    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account_id = Some(v.into());
        self
    }

    #[doc = "Set the field `call_as`.\n"]
    pub fn set_call_as(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().call_as = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `parameter_overrides`.\n"]
    pub fn set_parameter_overrides(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().parameter_overrides = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `retain_stack`.\n"]
    pub fn set_retain_stack(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().retain_stack = Some(v.into());
        self
    }

    #[doc = "Set the field `stack_set_instance_region`.\n"]
    pub fn set_stack_set_instance_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().stack_set_instance_region = Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_targets`.\n"]
    pub fn set_deployment_targets(
        self,
        v: impl Into<BlockAssignable<CloudformationStackSetInstanceDeploymentTargetsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deployment_targets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deployment_targets = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `operation_preferences`.\n"]
    pub fn set_operation_preferences(
        self,
        v: impl Into<BlockAssignable<CloudformationStackSetInstanceOperationPreferencesEl>>,
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
    pub fn set_timeouts(self, v: impl Into<CloudformationStackSetInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `call_as` after provisioning.\n"]
    pub fn call_as(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.call_as", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organizational_unit_id` after provisioning.\n"]
    pub fn organizational_unit_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organizational_unit_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parameter_overrides` after provisioning.\n"]
    pub fn parameter_overrides(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.parameter_overrides", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `retain_stack` after provisioning.\n"]
    pub fn retain_stack(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retain_stack", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `stack_id` after provisioning.\n"]
    pub fn stack_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stack_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `stack_instance_summaries` after provisioning.\nList of stack instances created from an organizational unit deployment target. This will only be populated when `deployment_targets` is set."]
    pub fn stack_instance_summaries(
        &self,
    ) -> ListRef<CloudformationStackSetInstanceStackInstanceSummariesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stack_instance_summaries", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `stack_set_instance_region` after provisioning.\n"]
    pub fn stack_set_instance_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stack_set_instance_region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `stack_set_name` after provisioning.\n"]
    pub fn stack_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stack_set_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deployment_targets` after provisioning.\n"]
    pub fn deployment_targets(
        &self,
    ) -> ListRef<CloudformationStackSetInstanceDeploymentTargetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deployment_targets", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `operation_preferences` after provisioning.\n"]
    pub fn operation_preferences(
        &self,
    ) -> ListRef<CloudformationStackSetInstanceOperationPreferencesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.operation_preferences", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudformationStackSetInstanceTimeoutsElRef {
        CloudformationStackSetInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for CloudformationStackSetInstance {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for CloudformationStackSetInstance {}

impl ToListMappable for CloudformationStackSetInstance {
    type O = ListRef<CloudformationStackSetInstanceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CloudformationStackSetInstance_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudformation_stack_set_instance".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudformationStackSetInstance {
    pub tf_id: String,
    #[doc = ""]
    pub stack_set_name: PrimField<String>,
}

impl BuildCloudformationStackSetInstance {
    pub fn build(self, stack: &mut Stack) -> CloudformationStackSetInstance {
        let out = CloudformationStackSetInstance(Rc::new(CloudformationStackSetInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudformationStackSetInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: core::default::Default::default(),
                call_as: core::default::Default::default(),
                id: core::default::Default::default(),
                parameter_overrides: core::default::Default::default(),
                region: core::default::Default::default(),
                retain_stack: core::default::Default::default(),
                stack_set_instance_region: core::default::Default::default(),
                stack_set_name: self.stack_set_name,
                deployment_targets: core::default::Default::default(),
                operation_preferences: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudformationStackSetInstanceRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudformationStackSetInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl CloudformationStackSetInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `call_as` after provisioning.\n"]
    pub fn call_as(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.call_as", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organizational_unit_id` after provisioning.\n"]
    pub fn organizational_unit_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organizational_unit_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parameter_overrides` after provisioning.\n"]
    pub fn parameter_overrides(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.parameter_overrides", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `retain_stack` after provisioning.\n"]
    pub fn retain_stack(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retain_stack", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `stack_id` after provisioning.\n"]
    pub fn stack_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stack_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `stack_instance_summaries` after provisioning.\nList of stack instances created from an organizational unit deployment target. This will only be populated when `deployment_targets` is set."]
    pub fn stack_instance_summaries(
        &self,
    ) -> ListRef<CloudformationStackSetInstanceStackInstanceSummariesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stack_instance_summaries", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `stack_set_instance_region` after provisioning.\n"]
    pub fn stack_set_instance_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stack_set_instance_region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `stack_set_name` after provisioning.\n"]
    pub fn stack_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stack_set_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deployment_targets` after provisioning.\n"]
    pub fn deployment_targets(
        &self,
    ) -> ListRef<CloudformationStackSetInstanceDeploymentTargetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deployment_targets", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `operation_preferences` after provisioning.\n"]
    pub fn operation_preferences(
        &self,
    ) -> ListRef<CloudformationStackSetInstanceOperationPreferencesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.operation_preferences", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudformationStackSetInstanceTimeoutsElRef {
        CloudformationStackSetInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct CloudformationStackSetInstanceStackInstanceSummariesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organizational_unit_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stack_id: Option<PrimField<String>>,
}

impl CloudformationStackSetInstanceStackInstanceSummariesEl {
    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.account_id = Some(v.into());
        self
    }

    #[doc = "Set the field `organizational_unit_id`.\n"]
    pub fn set_organizational_unit_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.organizational_unit_id = Some(v.into());
        self
    }

    #[doc = "Set the field `stack_id`.\n"]
    pub fn set_stack_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stack_id = Some(v.into());
        self
    }
}

impl ToListMappable for CloudformationStackSetInstanceStackInstanceSummariesEl {
    type O = BlockAssignable<CloudformationStackSetInstanceStackInstanceSummariesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudformationStackSetInstanceStackInstanceSummariesEl {}

impl BuildCloudformationStackSetInstanceStackInstanceSummariesEl {
    pub fn build(self) -> CloudformationStackSetInstanceStackInstanceSummariesEl {
        CloudformationStackSetInstanceStackInstanceSummariesEl {
            account_id: core::default::Default::default(),
            organizational_unit_id: core::default::Default::default(),
            stack_id: core::default::Default::default(),
        }
    }
}

pub struct CloudformationStackSetInstanceStackInstanceSummariesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudformationStackSetInstanceStackInstanceSummariesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudformationStackSetInstanceStackInstanceSummariesElRef {
        CloudformationStackSetInstanceStackInstanceSummariesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudformationStackSetInstanceStackInstanceSummariesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `organizational_unit_id` after provisioning.\n"]
    pub fn organizational_unit_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organizational_unit_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `stack_id` after provisioning.\n"]
    pub fn stack_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_id", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudformationStackSetInstanceDeploymentTargetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_filter_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accounts: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accounts_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organizational_unit_ids: Option<SetField<PrimField<String>>>,
}

impl CloudformationStackSetInstanceDeploymentTargetsEl {
    #[doc = "Set the field `account_filter_type`.\n"]
    pub fn set_account_filter_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.account_filter_type = Some(v.into());
        self
    }

    #[doc = "Set the field `accounts`.\n"]
    pub fn set_accounts(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.accounts = Some(v.into());
        self
    }

    #[doc = "Set the field `accounts_url`.\n"]
    pub fn set_accounts_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accounts_url = Some(v.into());
        self
    }

    #[doc = "Set the field `organizational_unit_ids`.\n"]
    pub fn set_organizational_unit_ids(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.organizational_unit_ids = Some(v.into());
        self
    }
}

impl ToListMappable for CloudformationStackSetInstanceDeploymentTargetsEl {
    type O = BlockAssignable<CloudformationStackSetInstanceDeploymentTargetsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudformationStackSetInstanceDeploymentTargetsEl {}

impl BuildCloudformationStackSetInstanceDeploymentTargetsEl {
    pub fn build(self) -> CloudformationStackSetInstanceDeploymentTargetsEl {
        CloudformationStackSetInstanceDeploymentTargetsEl {
            account_filter_type: core::default::Default::default(),
            accounts: core::default::Default::default(),
            accounts_url: core::default::Default::default(),
            organizational_unit_ids: core::default::Default::default(),
        }
    }
}

pub struct CloudformationStackSetInstanceDeploymentTargetsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudformationStackSetInstanceDeploymentTargetsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudformationStackSetInstanceDeploymentTargetsElRef {
        CloudformationStackSetInstanceDeploymentTargetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudformationStackSetInstanceDeploymentTargetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_filter_type` after provisioning.\n"]
    pub fn account_filter_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_filter_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `accounts` after provisioning.\n"]
    pub fn accounts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.accounts", self.base))
    }

    #[doc = "Get a reference to the value of field `accounts_url` after provisioning.\n"]
    pub fn accounts_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.accounts_url", self.base))
    }

    #[doc = "Get a reference to the value of field `organizational_unit_ids` after provisioning.\n"]
    pub fn organizational_unit_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.organizational_unit_ids", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CloudformationStackSetInstanceOperationPreferencesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    concurrency_mode: Option<PrimField<String>>,
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

impl CloudformationStackSetInstanceOperationPreferencesEl {
    #[doc = "Set the field `concurrency_mode`.\n"]
    pub fn set_concurrency_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.concurrency_mode = Some(v.into());
        self
    }

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

impl ToListMappable for CloudformationStackSetInstanceOperationPreferencesEl {
    type O = BlockAssignable<CloudformationStackSetInstanceOperationPreferencesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudformationStackSetInstanceOperationPreferencesEl {}

impl BuildCloudformationStackSetInstanceOperationPreferencesEl {
    pub fn build(self) -> CloudformationStackSetInstanceOperationPreferencesEl {
        CloudformationStackSetInstanceOperationPreferencesEl {
            concurrency_mode: core::default::Default::default(),
            failure_tolerance_count: core::default::Default::default(),
            failure_tolerance_percentage: core::default::Default::default(),
            max_concurrent_count: core::default::Default::default(),
            max_concurrent_percentage: core::default::Default::default(),
            region_concurrency_type: core::default::Default::default(),
            region_order: core::default::Default::default(),
        }
    }
}

pub struct CloudformationStackSetInstanceOperationPreferencesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudformationStackSetInstanceOperationPreferencesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudformationStackSetInstanceOperationPreferencesElRef {
        CloudformationStackSetInstanceOperationPreferencesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudformationStackSetInstanceOperationPreferencesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `concurrency_mode` after provisioning.\n"]
    pub fn concurrency_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.concurrency_mode", self.base),
        )
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
pub struct CloudformationStackSetInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl CloudformationStackSetInstanceTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for CloudformationStackSetInstanceTimeoutsEl {
    type O = BlockAssignable<CloudformationStackSetInstanceTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudformationStackSetInstanceTimeoutsEl {}

impl BuildCloudformationStackSetInstanceTimeoutsEl {
    pub fn build(self) -> CloudformationStackSetInstanceTimeoutsEl {
        CloudformationStackSetInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct CloudformationStackSetInstanceTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudformationStackSetInstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CloudformationStackSetInstanceTimeoutsElRef {
        CloudformationStackSetInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudformationStackSetInstanceTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudformationStackSetInstanceDynamic {
    deployment_targets: Option<DynamicBlock<CloudformationStackSetInstanceDeploymentTargetsEl>>,
    operation_preferences:
        Option<DynamicBlock<CloudformationStackSetInstanceOperationPreferencesEl>>,
}
