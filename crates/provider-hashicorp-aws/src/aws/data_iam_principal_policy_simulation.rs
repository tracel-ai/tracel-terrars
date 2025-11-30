use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataIamPrincipalPolicySimulationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    action_names: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_policies_json: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caller_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions_boundary_policies_json: Option<SetField<PrimField<String>>>,
    policy_source_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_handling_option: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_owner_account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_policy_json: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<Vec<DataIamPrincipalPolicySimulationContextEl>>,
    dynamic: DataIamPrincipalPolicySimulationDynamic,
}

struct DataIamPrincipalPolicySimulation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataIamPrincipalPolicySimulationData>,
}

#[derive(Clone)]
pub struct DataIamPrincipalPolicySimulation(Rc<DataIamPrincipalPolicySimulation_>);

impl DataIamPrincipalPolicySimulation {
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

    #[doc = "Set the field `additional_policies_json`.\nAdditional principal-based policies to use in the simulation."]
    pub fn set_additional_policies_json(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().additional_policies_json = Some(v.into());
        self
    }

    #[doc = "Set the field `caller_arn`.\nARN of a user to use as the caller of the simulated requests. If not specified, defaults to the principal specified in policy_source_arn, if it is a user ARN."]
    pub fn set_caller_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().caller_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `permissions_boundary_policies_json`.\nAdditional permission boundary policies to use in the simulation."]
    pub fn set_permissions_boundary_policies_json(
        self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.0.data.borrow_mut().permissions_boundary_policies_json = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_arns`.\nARNs of specific resources to use as the targets of the specified actions during simulation. If not specified, the simulator assumes \"*\" which represents general access across all resources."]
    pub fn set_resource_arns(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().resource_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_handling_option`.\nSpecifies the type of simulation to run. Some API operations need a particular resource handling option in order to produce a correct reesult."]
    pub fn set_resource_handling_option(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().resource_handling_option = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_owner_account_id`.\nAn AWS account ID to use as the simulated owner for any resource whose ARN does not include a specific owner account ID. Defaults to the account given as part of caller_arn."]
    pub fn set_resource_owner_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().resource_owner_account_id = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_policy_json`.\nA resource policy to associate with all of the target resources for simulation purposes. The policy simulator does not automatically retrieve resource-level policies, so if a resource policy is crucial to your test then you must specify here the same policy document associated with your target resource(s)."]
    pub fn set_resource_policy_json(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().resource_policy_json = Some(v.into());
        self
    }

    #[doc = "Set the field `context`.\n"]
    pub fn set_context(
        self,
        v: impl Into<BlockAssignable<DataIamPrincipalPolicySimulationContextEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().context = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.context = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `action_names` after provisioning.\nOne or more names of actions, like \"iam:CreateUser\", that should be included in the simulation."]
    pub fn action_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.action_names", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `additional_policies_json` after provisioning.\nAdditional principal-based policies to use in the simulation."]
    pub fn additional_policies_json(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.additional_policies_json", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `all_allowed` after provisioning.\nA summary of the results attribute which is true if all of the results have decision \"allowed\", and false otherwise."]
    pub fn all_allowed(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.all_allowed", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `caller_arn` after provisioning.\nARN of a user to use as the caller of the simulated requests. If not specified, defaults to the principal specified in policy_source_arn, if it is a user ARN."]
    pub fn caller_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.caller_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nDo not use"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permissions_boundary_policies_json` after provisioning.\nAdditional permission boundary policies to use in the simulation."]
    pub fn permissions_boundary_policies_json(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.permissions_boundary_policies_json", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `policy_source_arn` after provisioning.\nARN of the principal (e.g. user, role) whose existing configured access policies will be used as the basis for the simulation. If you specify a role ARN here, you can also set caller_arn to simulate a particular user acting with the given role."]
    pub fn policy_source_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_source_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_arns` after provisioning.\nARNs of specific resources to use as the targets of the specified actions during simulation. If not specified, the simulator assumes \"*\" which represents general access across all resources."]
    pub fn resource_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.resource_arns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_handling_option` after provisioning.\nSpecifies the type of simulation to run. Some API operations need a particular resource handling option in order to produce a correct reesult."]
    pub fn resource_handling_option(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_handling_option", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_owner_account_id` after provisioning.\nAn AWS account ID to use as the simulated owner for any resource whose ARN does not include a specific owner account ID. Defaults to the account given as part of caller_arn."]
    pub fn resource_owner_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_owner_account_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_policy_json` after provisioning.\nA resource policy to associate with all of the target resources for simulation purposes. The policy simulator does not automatically retrieve resource-level policies, so if a resource policy is crucial to your test then you must specify here the same policy document associated with your target resource(s)."]
    pub fn resource_policy_json(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_policy_json", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `results` after provisioning.\n"]
    pub fn results(&self) -> SetRef<DataIamPrincipalPolicySimulationResultsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.results", self.extract_ref()),
        )
    }
}

impl Referable for DataIamPrincipalPolicySimulation {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataIamPrincipalPolicySimulation {}

impl ToListMappable for DataIamPrincipalPolicySimulation {
    type O = ListRef<DataIamPrincipalPolicySimulationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataIamPrincipalPolicySimulation_ {
    fn extract_datasource_type(&self) -> String {
        "aws_iam_principal_policy_simulation".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataIamPrincipalPolicySimulation {
    pub tf_id: String,
    #[doc = "One or more names of actions, like \"iam:CreateUser\", that should be included in the simulation."]
    pub action_names: SetField<PrimField<String>>,
    #[doc = "ARN of the principal (e.g. user, role) whose existing configured access policies will be used as the basis for the simulation. If you specify a role ARN here, you can also set caller_arn to simulate a particular user acting with the given role."]
    pub policy_source_arn: PrimField<String>,
}

impl BuildDataIamPrincipalPolicySimulation {
    pub fn build(self, stack: &mut Stack) -> DataIamPrincipalPolicySimulation {
        let out = DataIamPrincipalPolicySimulation(Rc::new(DataIamPrincipalPolicySimulation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataIamPrincipalPolicySimulationData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                action_names: self.action_names,
                additional_policies_json: core::default::Default::default(),
                caller_arn: core::default::Default::default(),
                permissions_boundary_policies_json: core::default::Default::default(),
                policy_source_arn: self.policy_source_arn,
                resource_arns: core::default::Default::default(),
                resource_handling_option: core::default::Default::default(),
                resource_owner_account_id: core::default::Default::default(),
                resource_policy_json: core::default::Default::default(),
                context: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataIamPrincipalPolicySimulationRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIamPrincipalPolicySimulationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataIamPrincipalPolicySimulationRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `action_names` after provisioning.\nOne or more names of actions, like \"iam:CreateUser\", that should be included in the simulation."]
    pub fn action_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.action_names", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `additional_policies_json` after provisioning.\nAdditional principal-based policies to use in the simulation."]
    pub fn additional_policies_json(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.additional_policies_json", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `all_allowed` after provisioning.\nA summary of the results attribute which is true if all of the results have decision \"allowed\", and false otherwise."]
    pub fn all_allowed(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.all_allowed", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `caller_arn` after provisioning.\nARN of a user to use as the caller of the simulated requests. If not specified, defaults to the principal specified in policy_source_arn, if it is a user ARN."]
    pub fn caller_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.caller_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nDo not use"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permissions_boundary_policies_json` after provisioning.\nAdditional permission boundary policies to use in the simulation."]
    pub fn permissions_boundary_policies_json(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.permissions_boundary_policies_json", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `policy_source_arn` after provisioning.\nARN of the principal (e.g. user, role) whose existing configured access policies will be used as the basis for the simulation. If you specify a role ARN here, you can also set caller_arn to simulate a particular user acting with the given role."]
    pub fn policy_source_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_source_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_arns` after provisioning.\nARNs of specific resources to use as the targets of the specified actions during simulation. If not specified, the simulator assumes \"*\" which represents general access across all resources."]
    pub fn resource_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.resource_arns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_handling_option` after provisioning.\nSpecifies the type of simulation to run. Some API operations need a particular resource handling option in order to produce a correct reesult."]
    pub fn resource_handling_option(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_handling_option", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_owner_account_id` after provisioning.\nAn AWS account ID to use as the simulated owner for any resource whose ARN does not include a specific owner account ID. Defaults to the account given as part of caller_arn."]
    pub fn resource_owner_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_owner_account_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_policy_json` after provisioning.\nA resource policy to associate with all of the target resources for simulation purposes. The policy simulator does not automatically retrieve resource-level policies, so if a resource policy is crucial to your test then you must specify here the same policy document associated with your target resource(s)."]
    pub fn resource_policy_json(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_policy_json", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `results` after provisioning.\n"]
    pub fn results(&self) -> SetRef<DataIamPrincipalPolicySimulationResultsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.results", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataIamPrincipalPolicySimulationResultsElMatchedStatementsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    source_policy_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_policy_type: Option<PrimField<String>>,
}

impl DataIamPrincipalPolicySimulationResultsElMatchedStatementsEl {
    #[doc = "Set the field `source_policy_id`.\n"]
    pub fn set_source_policy_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_policy_id = Some(v.into());
        self
    }

    #[doc = "Set the field `source_policy_type`.\n"]
    pub fn set_source_policy_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_policy_type = Some(v.into());
        self
    }
}

impl ToListMappable for DataIamPrincipalPolicySimulationResultsElMatchedStatementsEl {
    type O = BlockAssignable<DataIamPrincipalPolicySimulationResultsElMatchedStatementsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamPrincipalPolicySimulationResultsElMatchedStatementsEl {}

impl BuildDataIamPrincipalPolicySimulationResultsElMatchedStatementsEl {
    pub fn build(self) -> DataIamPrincipalPolicySimulationResultsElMatchedStatementsEl {
        DataIamPrincipalPolicySimulationResultsElMatchedStatementsEl {
            source_policy_id: core::default::Default::default(),
            source_policy_type: core::default::Default::default(),
        }
    }
}

pub struct DataIamPrincipalPolicySimulationResultsElMatchedStatementsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIamPrincipalPolicySimulationResultsElMatchedStatementsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataIamPrincipalPolicySimulationResultsElMatchedStatementsElRef {
        DataIamPrincipalPolicySimulationResultsElMatchedStatementsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamPrincipalPolicySimulationResultsElMatchedStatementsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_policy_id` after provisioning.\n"]
    pub fn source_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_policy_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `source_policy_type` after provisioning.\n"]
    pub fn source_policy_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_policy_type", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataIamPrincipalPolicySimulationResultsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    decision: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    decision_details: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matched_statements:
        Option<SetField<DataIamPrincipalPolicySimulationResultsElMatchedStatementsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    missing_context_keys: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_arn: Option<PrimField<String>>,
}

impl DataIamPrincipalPolicySimulationResultsEl {
    #[doc = "Set the field `action_name`.\n"]
    pub fn set_action_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action_name = Some(v.into());
        self
    }

    #[doc = "Set the field `allowed`.\n"]
    pub fn set_allowed(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allowed = Some(v.into());
        self
    }

    #[doc = "Set the field `decision`.\n"]
    pub fn set_decision(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.decision = Some(v.into());
        self
    }

    #[doc = "Set the field `decision_details`.\n"]
    pub fn set_decision_details(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.decision_details = Some(v.into());
        self
    }

    #[doc = "Set the field `matched_statements`.\n"]
    pub fn set_matched_statements(
        mut self,
        v: impl Into<SetField<DataIamPrincipalPolicySimulationResultsElMatchedStatementsEl>>,
    ) -> Self {
        self.matched_statements = Some(v.into());
        self
    }

    #[doc = "Set the field `missing_context_keys`.\n"]
    pub fn set_missing_context_keys(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.missing_context_keys = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_arn`.\n"]
    pub fn set_resource_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_arn = Some(v.into());
        self
    }
}

impl ToListMappable for DataIamPrincipalPolicySimulationResultsEl {
    type O = BlockAssignable<DataIamPrincipalPolicySimulationResultsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamPrincipalPolicySimulationResultsEl {}

impl BuildDataIamPrincipalPolicySimulationResultsEl {
    pub fn build(self) -> DataIamPrincipalPolicySimulationResultsEl {
        DataIamPrincipalPolicySimulationResultsEl {
            action_name: core::default::Default::default(),
            allowed: core::default::Default::default(),
            decision: core::default::Default::default(),
            decision_details: core::default::Default::default(),
            matched_statements: core::default::Default::default(),
            missing_context_keys: core::default::Default::default(),
            resource_arn: core::default::Default::default(),
        }
    }
}

pub struct DataIamPrincipalPolicySimulationResultsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIamPrincipalPolicySimulationResultsElRef {
    fn new(shared: StackShared, base: String) -> DataIamPrincipalPolicySimulationResultsElRef {
        DataIamPrincipalPolicySimulationResultsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamPrincipalPolicySimulationResultsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_name` after provisioning.\n"]
    pub fn action_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_name", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed` after provisioning.\n"]
    pub fn allowed(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allowed", self.base))
    }

    #[doc = "Get a reference to the value of field `decision` after provisioning.\n"]
    pub fn decision(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.decision", self.base))
    }

    #[doc = "Get a reference to the value of field `decision_details` after provisioning.\n"]
    pub fn decision_details(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.decision_details", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `matched_statements` after provisioning.\n"]
    pub fn matched_statements(
        &self,
    ) -> SetRef<DataIamPrincipalPolicySimulationResultsElMatchedStatementsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.matched_statements", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `missing_context_keys` after provisioning.\n"]
    pub fn missing_context_keys(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.missing_context_keys", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataIamPrincipalPolicySimulationContextEl {
    key: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl DataIamPrincipalPolicySimulationContextEl {}

impl ToListMappable for DataIamPrincipalPolicySimulationContextEl {
    type O = BlockAssignable<DataIamPrincipalPolicySimulationContextEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamPrincipalPolicySimulationContextEl {
    #[doc = "The key name of the context entry, such as \"aws:CurrentTime\"."]
    pub key: PrimField<String>,
    #[doc = "The type that the simulator should use to interpret the strings given in argument \"values\"."]
    pub type_: PrimField<String>,
    #[doc = "One or more values to assign to the context key, given as a string in a syntax appropriate for the selected value type."]
    pub values: SetField<PrimField<String>>,
}

impl BuildDataIamPrincipalPolicySimulationContextEl {
    pub fn build(self) -> DataIamPrincipalPolicySimulationContextEl {
        DataIamPrincipalPolicySimulationContextEl {
            key: self.key,
            type_: self.type_,
            values: self.values,
        }
    }
}

pub struct DataIamPrincipalPolicySimulationContextElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIamPrincipalPolicySimulationContextElRef {
    fn new(shared: StackShared, base: String) -> DataIamPrincipalPolicySimulationContextElRef {
        DataIamPrincipalPolicySimulationContextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamPrincipalPolicySimulationContextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nThe key name of the context entry, such as \"aws:CurrentTime\"."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type that the simulator should use to interpret the strings given in argument \"values\"."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\nOne or more values to assign to the context key, given as a string in a syntax appropriate for the selected value type."]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataIamPrincipalPolicySimulationDynamic {
    context: Option<DynamicBlock<DataIamPrincipalPolicySimulationContextEl>>,
}
