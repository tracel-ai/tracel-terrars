use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudformationStackInstancesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accounts: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    call_as: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter_overrides: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retain_stacks: Option<PrimField<bool>>,
    stack_set_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_targets: Option<Vec<CloudformationStackInstancesDeploymentTargetsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operation_preferences: Option<Vec<CloudformationStackInstancesOperationPreferencesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudformationStackInstancesTimeoutsEl>,
    dynamic: CloudformationStackInstancesDynamic,
}

struct CloudformationStackInstances_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudformationStackInstancesData>,
}

#[derive(Clone)]
pub struct CloudformationStackInstances(Rc<CloudformationStackInstances_>);

impl CloudformationStackInstances {
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

    #[doc = "Set the field `accounts`.\n"]
    pub fn set_accounts(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().accounts = Some(v.into());
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `regions`.\n"]
    pub fn set_regions(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().regions = Some(v.into());
        self
    }

    #[doc = "Set the field `retain_stacks`.\n"]
    pub fn set_retain_stacks(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().retain_stacks = Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_targets`.\n"]
    pub fn set_deployment_targets(
        self,
        v: impl Into<BlockAssignable<CloudformationStackInstancesDeploymentTargetsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deployment_targets = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deployment_targets = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `operation_preferences`.\n"]
    pub fn set_operation_preferences(
        self,
        v: impl Into<BlockAssignable<CloudformationStackInstancesOperationPreferencesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().operation_preferences = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.operation_preferences = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CloudformationStackInstancesTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `accounts` after provisioning.\n"]
    pub fn accounts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.accounts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `call_as` after provisioning.\n"]
    pub fn call_as(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.call_as", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameter_overrides` after provisioning.\n"]
    pub fn parameter_overrides(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameter_overrides", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retain_stacks` after provisioning.\n"]
    pub fn retain_stacks(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.retain_stacks", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stack_instance_summaries` after provisioning.\nList of stack instances created from an organizational unit deployment target. This will only be populated when `deployment_targets` is set."]
    pub fn stack_instance_summaries(&self) -> ListRef<CloudformationStackInstancesStackInstanceSummariesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.stack_instance_summaries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stack_set_id` after provisioning.\n"]
    pub fn stack_set_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_set_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stack_set_name` after provisioning.\n"]
    pub fn stack_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_targets` after provisioning.\n"]
    pub fn deployment_targets(&self) -> ListRef<CloudformationStackInstancesDeploymentTargetsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deployment_targets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operation_preferences` after provisioning.\n"]
    pub fn operation_preferences(&self) -> ListRef<CloudformationStackInstancesOperationPreferencesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.operation_preferences", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudformationStackInstancesTimeoutsElRef {
        CloudformationStackInstancesTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for CloudformationStackInstances {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for CloudformationStackInstances { }

impl ToListMappable for CloudformationStackInstances {
    type O = ListRef<CloudformationStackInstancesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CloudformationStackInstances_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudformation_stack_instances".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudformationStackInstances {
    pub tf_id: String,
    #[doc = ""]
    pub stack_set_name: PrimField<String>,
}

impl BuildCloudformationStackInstances {
    pub fn build(self, stack: &mut Stack) -> CloudformationStackInstances {
        let out = CloudformationStackInstances(Rc::new(CloudformationStackInstances_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudformationStackInstancesData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                accounts: core::default::Default::default(),
                call_as: core::default::Default::default(),
                id: core::default::Default::default(),
                parameter_overrides: core::default::Default::default(),
                region: core::default::Default::default(),
                regions: core::default::Default::default(),
                retain_stacks: core::default::Default::default(),
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

pub struct CloudformationStackInstancesRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudformationStackInstancesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl CloudformationStackInstancesRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accounts` after provisioning.\n"]
    pub fn accounts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.accounts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `call_as` after provisioning.\n"]
    pub fn call_as(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.call_as", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameter_overrides` after provisioning.\n"]
    pub fn parameter_overrides(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameter_overrides", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retain_stacks` after provisioning.\n"]
    pub fn retain_stacks(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.retain_stacks", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stack_instance_summaries` after provisioning.\nList of stack instances created from an organizational unit deployment target. This will only be populated when `deployment_targets` is set."]
    pub fn stack_instance_summaries(&self) -> ListRef<CloudformationStackInstancesStackInstanceSummariesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.stack_instance_summaries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stack_set_id` after provisioning.\n"]
    pub fn stack_set_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_set_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stack_set_name` after provisioning.\n"]
    pub fn stack_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_targets` after provisioning.\n"]
    pub fn deployment_targets(&self) -> ListRef<CloudformationStackInstancesDeploymentTargetsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deployment_targets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operation_preferences` after provisioning.\n"]
    pub fn operation_preferences(&self) -> ListRef<CloudformationStackInstancesOperationPreferencesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.operation_preferences", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudformationStackInstancesTimeoutsElRef {
        CloudformationStackInstancesTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct CloudformationStackInstancesStackInstanceSummariesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detailed_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drift_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organizational_unit_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stack_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stack_set_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_reason: Option<PrimField<String>>,
}

impl CloudformationStackInstancesStackInstanceSummariesEl {
    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.account_id = Some(v.into());
        self
    }

    #[doc = "Set the field `detailed_status`.\n"]
    pub fn set_detailed_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.detailed_status = Some(v.into());
        self
    }

    #[doc = "Set the field `drift_status`.\n"]
    pub fn set_drift_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.drift_status = Some(v.into());
        self
    }

    #[doc = "Set the field `organizational_unit_id`.\n"]
    pub fn set_organizational_unit_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.organizational_unit_id = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }

    #[doc = "Set the field `stack_id`.\n"]
    pub fn set_stack_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stack_id = Some(v.into());
        self
    }

    #[doc = "Set the field `stack_set_id`.\n"]
    pub fn set_stack_set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stack_set_id = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }

    #[doc = "Set the field `status_reason`.\n"]
    pub fn set_status_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status_reason = Some(v.into());
        self
    }
}

impl ToListMappable for CloudformationStackInstancesStackInstanceSummariesEl {
    type O = BlockAssignable<CloudformationStackInstancesStackInstanceSummariesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudformationStackInstancesStackInstanceSummariesEl {}

impl BuildCloudformationStackInstancesStackInstanceSummariesEl {
    pub fn build(self) -> CloudformationStackInstancesStackInstanceSummariesEl {
        CloudformationStackInstancesStackInstanceSummariesEl {
            account_id: core::default::Default::default(),
            detailed_status: core::default::Default::default(),
            drift_status: core::default::Default::default(),
            organizational_unit_id: core::default::Default::default(),
            region: core::default::Default::default(),
            stack_id: core::default::Default::default(),
            stack_set_id: core::default::Default::default(),
            status: core::default::Default::default(),
            status_reason: core::default::Default::default(),
        }
    }
}

pub struct CloudformationStackInstancesStackInstanceSummariesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudformationStackInstancesStackInstanceSummariesElRef {
    fn new(shared: StackShared, base: String) -> CloudformationStackInstancesStackInstanceSummariesElRef {
        CloudformationStackInstancesStackInstanceSummariesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudformationStackInstancesStackInstanceSummariesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `detailed_status` after provisioning.\n"]
    pub fn detailed_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.detailed_status", self.base))
    }

    #[doc = "Get a reference to the value of field `drift_status` after provisioning.\n"]
    pub fn drift_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.drift_status", self.base))
    }

    #[doc = "Get a reference to the value of field `organizational_unit_id` after provisioning.\n"]
    pub fn organizational_unit_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.organizational_unit_id", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `stack_id` after provisioning.\n"]
    pub fn stack_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_id", self.base))
    }

    #[doc = "Get a reference to the value of field `stack_set_id` after provisioning.\n"]
    pub fn stack_set_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_set_id", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudformationStackInstancesDeploymentTargetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_filter_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accounts: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accounts_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organizational_unit_ids: Option<SetField<PrimField<String>>>,
}

impl CloudformationStackInstancesDeploymentTargetsEl {
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
    pub fn set_organizational_unit_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.organizational_unit_ids = Some(v.into());
        self
    }
}

impl ToListMappable for CloudformationStackInstancesDeploymentTargetsEl {
    type O = BlockAssignable<CloudformationStackInstancesDeploymentTargetsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudformationStackInstancesDeploymentTargetsEl {}

impl BuildCloudformationStackInstancesDeploymentTargetsEl {
    pub fn build(self) -> CloudformationStackInstancesDeploymentTargetsEl {
        CloudformationStackInstancesDeploymentTargetsEl {
            account_filter_type: core::default::Default::default(),
            accounts: core::default::Default::default(),
            accounts_url: core::default::Default::default(),
            organizational_unit_ids: core::default::Default::default(),
        }
    }
}

pub struct CloudformationStackInstancesDeploymentTargetsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudformationStackInstancesDeploymentTargetsElRef {
    fn new(shared: StackShared, base: String) -> CloudformationStackInstancesDeploymentTargetsElRef {
        CloudformationStackInstancesDeploymentTargetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudformationStackInstancesDeploymentTargetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_filter_type` after provisioning.\n"]
    pub fn account_filter_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_filter_type", self.base))
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
        SetRef::new(self.shared().clone(), format!("{}.organizational_unit_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudformationStackInstancesOperationPreferencesEl {
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

impl CloudformationStackInstancesOperationPreferencesEl {
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

impl ToListMappable for CloudformationStackInstancesOperationPreferencesEl {
    type O = BlockAssignable<CloudformationStackInstancesOperationPreferencesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudformationStackInstancesOperationPreferencesEl {}

impl BuildCloudformationStackInstancesOperationPreferencesEl {
    pub fn build(self) -> CloudformationStackInstancesOperationPreferencesEl {
        CloudformationStackInstancesOperationPreferencesEl {
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

pub struct CloudformationStackInstancesOperationPreferencesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudformationStackInstancesOperationPreferencesElRef {
    fn new(shared: StackShared, base: String) -> CloudformationStackInstancesOperationPreferencesElRef {
        CloudformationStackInstancesOperationPreferencesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudformationStackInstancesOperationPreferencesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `concurrency_mode` after provisioning.\n"]
    pub fn concurrency_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.concurrency_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `failure_tolerance_count` after provisioning.\n"]
    pub fn failure_tolerance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.failure_tolerance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `failure_tolerance_percentage` after provisioning.\n"]
    pub fn failure_tolerance_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.failure_tolerance_percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `max_concurrent_count` after provisioning.\n"]
    pub fn max_concurrent_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_count", self.base))
    }

    #[doc = "Get a reference to the value of field `max_concurrent_percentage` after provisioning.\n"]
    pub fn max_concurrent_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `region_concurrency_type` after provisioning.\n"]
    pub fn region_concurrency_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region_concurrency_type", self.base))
    }

    #[doc = "Get a reference to the value of field `region_order` after provisioning.\n"]
    pub fn region_order(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.region_order", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudformationStackInstancesTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl CloudformationStackInstancesTimeoutsEl {
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

impl ToListMappable for CloudformationStackInstancesTimeoutsEl {
    type O = BlockAssignable<CloudformationStackInstancesTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudformationStackInstancesTimeoutsEl {}

impl BuildCloudformationStackInstancesTimeoutsEl {
    pub fn build(self) -> CloudformationStackInstancesTimeoutsEl {
        CloudformationStackInstancesTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct CloudformationStackInstancesTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudformationStackInstancesTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CloudformationStackInstancesTimeoutsElRef {
        CloudformationStackInstancesTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudformationStackInstancesTimeoutsElRef {
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
struct CloudformationStackInstancesDynamic {
    deployment_targets: Option<DynamicBlock<CloudformationStackInstancesDeploymentTargetsEl>>,
    operation_preferences: Option<DynamicBlock<CloudformationStackInstancesOperationPreferencesEl>>,
}
