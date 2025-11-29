use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentcoreAgentRuntimeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    agent_runtime_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_variables: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_configuration: Option<ListField<BedrockagentcoreAgentRuntimeLifecycleConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_runtime_artifact: Option<Vec<BedrockagentcoreAgentRuntimeAgentRuntimeArtifactEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorizer_configuration: Option<Vec<BedrockagentcoreAgentRuntimeAuthorizerConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_configuration: Option<Vec<BedrockagentcoreAgentRuntimeNetworkConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol_configuration: Option<Vec<BedrockagentcoreAgentRuntimeProtocolConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_header_configuration: Option<Vec<BedrockagentcoreAgentRuntimeRequestHeaderConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BedrockagentcoreAgentRuntimeTimeoutsEl>,
    dynamic: BedrockagentcoreAgentRuntimeDynamic,
}

struct BedrockagentcoreAgentRuntime_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BedrockagentcoreAgentRuntimeData>,
}

#[derive(Clone)]
pub struct BedrockagentcoreAgentRuntime(Rc<BedrockagentcoreAgentRuntime_>);

impl BedrockagentcoreAgentRuntime {
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

    #[doc = "Set the field `environment_variables`.\n"]
    pub fn set_environment_variables(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().environment_variables = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_configuration`.\n"]
    pub fn set_lifecycle_configuration(
        self,
        v: impl Into<ListField<BedrockagentcoreAgentRuntimeLifecycleConfigurationEl>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle_configuration = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `agent_runtime_artifact`.\n"]
    pub fn set_agent_runtime_artifact(
        self,
        v: impl Into<BlockAssignable<BedrockagentcoreAgentRuntimeAgentRuntimeArtifactEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().agent_runtime_artifact = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.agent_runtime_artifact = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `authorizer_configuration`.\n"]
    pub fn set_authorizer_configuration(
        self,
        v: impl Into<BlockAssignable<BedrockagentcoreAgentRuntimeAuthorizerConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().authorizer_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.authorizer_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        self,
        v: impl Into<BlockAssignable<BedrockagentcoreAgentRuntimeNetworkConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `protocol_configuration`.\n"]
    pub fn set_protocol_configuration(
        self,
        v: impl Into<BlockAssignable<BedrockagentcoreAgentRuntimeProtocolConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().protocol_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.protocol_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `request_header_configuration`.\n"]
    pub fn set_request_header_configuration(
        self,
        v: impl Into<BlockAssignable<BedrockagentcoreAgentRuntimeRequestHeaderConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().request_header_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.request_header_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BedrockagentcoreAgentRuntimeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `agent_runtime_arn` after provisioning.\n"]
    pub fn agent_runtime_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_runtime_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_runtime_id` after provisioning.\n"]
    pub fn agent_runtime_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_runtime_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_runtime_name` after provisioning.\n"]
    pub fn agent_runtime_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_runtime_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_runtime_version` after provisioning.\n"]
    pub fn agent_runtime_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_runtime_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\n"]
    pub fn environment_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lifecycle_configuration` after provisioning.\n"]
    pub fn lifecycle_configuration(&self) -> ListRef<BedrockagentcoreAgentRuntimeLifecycleConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lifecycle_configuration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workload_identity_details` after provisioning.\n"]
    pub fn workload_identity_details(&self) -> ListRef<BedrockagentcoreAgentRuntimeWorkloadIdentityDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.workload_identity_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_runtime_artifact` after provisioning.\n"]
    pub fn agent_runtime_artifact(&self) -> ListRef<BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElRef> {
        ListRef::new(self.shared().clone(), format!("{}.agent_runtime_artifact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorizer_configuration` after provisioning.\n"]
    pub fn authorizer_configuration(&self) -> ListRef<BedrockagentcoreAgentRuntimeAuthorizerConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.authorizer_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> ListRef<BedrockagentcoreAgentRuntimeNetworkConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol_configuration` after provisioning.\n"]
    pub fn protocol_configuration(&self) -> ListRef<BedrockagentcoreAgentRuntimeProtocolConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.protocol_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_header_configuration` after provisioning.\n"]
    pub fn request_header_configuration(&self) -> ListRef<BedrockagentcoreAgentRuntimeRequestHeaderConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.request_header_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreAgentRuntimeTimeoutsElRef {
        BedrockagentcoreAgentRuntimeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for BedrockagentcoreAgentRuntime {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for BedrockagentcoreAgentRuntime { }

impl ToListMappable for BedrockagentcoreAgentRuntime {
    type O = ListRef<BedrockagentcoreAgentRuntimeRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BedrockagentcoreAgentRuntime_ {
    fn extract_resource_type(&self) -> String {
        "aws_bedrockagentcore_agent_runtime".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentcoreAgentRuntime {
    pub tf_id: String,
    #[doc = ""]
    pub agent_runtime_name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}

impl BuildBedrockagentcoreAgentRuntime {
    pub fn build(self, stack: &mut Stack) -> BedrockagentcoreAgentRuntime {
        let out = BedrockagentcoreAgentRuntime(Rc::new(BedrockagentcoreAgentRuntime_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BedrockagentcoreAgentRuntimeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                agent_runtime_name: self.agent_runtime_name,
                description: core::default::Default::default(),
                environment_variables: core::default::Default::default(),
                lifecycle_configuration: core::default::Default::default(),
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                agent_runtime_artifact: core::default::Default::default(),
                authorizer_configuration: core::default::Default::default(),
                network_configuration: core::default::Default::default(),
                protocol_configuration: core::default::Default::default(),
                request_header_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentcoreAgentRuntimeRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreAgentRuntimeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl BedrockagentcoreAgentRuntimeRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_runtime_arn` after provisioning.\n"]
    pub fn agent_runtime_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_runtime_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_runtime_id` after provisioning.\n"]
    pub fn agent_runtime_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_runtime_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_runtime_name` after provisioning.\n"]
    pub fn agent_runtime_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_runtime_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_runtime_version` after provisioning.\n"]
    pub fn agent_runtime_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_runtime_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\n"]
    pub fn environment_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lifecycle_configuration` after provisioning.\n"]
    pub fn lifecycle_configuration(&self) -> ListRef<BedrockagentcoreAgentRuntimeLifecycleConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lifecycle_configuration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workload_identity_details` after provisioning.\n"]
    pub fn workload_identity_details(&self) -> ListRef<BedrockagentcoreAgentRuntimeWorkloadIdentityDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.workload_identity_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_runtime_artifact` after provisioning.\n"]
    pub fn agent_runtime_artifact(&self) -> ListRef<BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElRef> {
        ListRef::new(self.shared().clone(), format!("{}.agent_runtime_artifact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorizer_configuration` after provisioning.\n"]
    pub fn authorizer_configuration(&self) -> ListRef<BedrockagentcoreAgentRuntimeAuthorizerConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.authorizer_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> ListRef<BedrockagentcoreAgentRuntimeNetworkConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol_configuration` after provisioning.\n"]
    pub fn protocol_configuration(&self) -> ListRef<BedrockagentcoreAgentRuntimeProtocolConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.protocol_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_header_configuration` after provisioning.\n"]
    pub fn request_header_configuration(&self) -> ListRef<BedrockagentcoreAgentRuntimeRequestHeaderConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.request_header_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreAgentRuntimeTimeoutsElRef {
        BedrockagentcoreAgentRuntimeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntimeLifecycleConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_runtime_session_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_lifetime: Option<PrimField<f64>>,
}

impl BedrockagentcoreAgentRuntimeLifecycleConfigurationEl {
    #[doc = "Set the field `idle_runtime_session_timeout`.\n"]
    pub fn set_idle_runtime_session_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.idle_runtime_session_timeout = Some(v.into());
        self
    }

    #[doc = "Set the field `max_lifetime`.\n"]
    pub fn set_max_lifetime(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_lifetime = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentcoreAgentRuntimeLifecycleConfigurationEl {
    type O = BlockAssignable<BedrockagentcoreAgentRuntimeLifecycleConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntimeLifecycleConfigurationEl {}

impl BuildBedrockagentcoreAgentRuntimeLifecycleConfigurationEl {
    pub fn build(self) -> BedrockagentcoreAgentRuntimeLifecycleConfigurationEl {
        BedrockagentcoreAgentRuntimeLifecycleConfigurationEl {
            idle_runtime_session_timeout: core::default::Default::default(),
            max_lifetime: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentcoreAgentRuntimeLifecycleConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreAgentRuntimeLifecycleConfigurationElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentcoreAgentRuntimeLifecycleConfigurationElRef {
        BedrockagentcoreAgentRuntimeLifecycleConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntimeLifecycleConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle_runtime_session_timeout` after provisioning.\n"]
    pub fn idle_runtime_session_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.idle_runtime_session_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `max_lifetime` after provisioning.\n"]
    pub fn max_lifetime(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_lifetime", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntimeWorkloadIdentityDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    workload_identity_arn: Option<PrimField<String>>,
}

impl BedrockagentcoreAgentRuntimeWorkloadIdentityDetailsEl {
    #[doc = "Set the field `workload_identity_arn`.\n"]
    pub fn set_workload_identity_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.workload_identity_arn = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentcoreAgentRuntimeWorkloadIdentityDetailsEl {
    type O = BlockAssignable<BedrockagentcoreAgentRuntimeWorkloadIdentityDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntimeWorkloadIdentityDetailsEl {}

impl BuildBedrockagentcoreAgentRuntimeWorkloadIdentityDetailsEl {
    pub fn build(self) -> BedrockagentcoreAgentRuntimeWorkloadIdentityDetailsEl {
        BedrockagentcoreAgentRuntimeWorkloadIdentityDetailsEl {
            workload_identity_arn: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentcoreAgentRuntimeWorkloadIdentityDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreAgentRuntimeWorkloadIdentityDetailsElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentcoreAgentRuntimeWorkloadIdentityDetailsElRef {
        BedrockagentcoreAgentRuntimeWorkloadIdentityDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntimeWorkloadIdentityDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `workload_identity_arn` after provisioning.\n"]
    pub fn workload_identity_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workload_identity_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeElS3El {
    bucket: PrimField<String>,
    prefix: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_id: Option<PrimField<String>>,
}

impl BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeElS3El {
    #[doc = "Set the field `version_id`.\n"]
    pub fn set_version_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version_id = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeElS3El {
    type O = BlockAssignable<BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeElS3El>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeElS3El {
    #[doc = ""]
    pub bucket: PrimField<String>,
    #[doc = ""]
    pub prefix: PrimField<String>,
}

impl BuildBedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeElS3El {
    pub fn build(self) -> BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeElS3El {
        BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeElS3El {
            bucket: self.bucket,
            prefix: self.prefix,
            version_id: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeElS3ElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeElS3ElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeElS3ElRef {
        BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeElS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeElDynamic {
    s3: Option<DynamicBlock<BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeElS3El>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<Vec<BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeElS3El>>,
    dynamic: BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeElDynamic,
}

impl BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeEl {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeElS3El,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3 = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeEl {
    type O = BlockAssignable<BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeEl {}

impl BuildBedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeEl {
    pub fn build(self) -> BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeEl {
        BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeEl {
            s3: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeElRef {
        BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> ListRef<BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeElS3ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElDynamic {
    code: Option<DynamicBlock<BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeEl>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationEl {
    entry_point: ListField<PrimField<String>>,
    runtime: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<Vec<BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeEl>>,
    dynamic: BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElDynamic,
}

impl BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationEl {
    type O = BlockAssignable<BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationEl {
    #[doc = ""]
    pub entry_point: ListField<PrimField<String>>,
    #[doc = ""]
    pub runtime: PrimField<String>,
}

impl BuildBedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationEl {
    pub fn build(self) -> BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationEl {
        BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationEl {
            entry_point: self.entry_point,
            runtime: self.runtime,
            code: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElRef {
        BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entry_point` after provisioning.\n"]
    pub fn entry_point(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.entry_point", self.base))
    }

    #[doc = "Get a reference to the value of field `runtime` after provisioning.\n"]
    pub fn runtime(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.base))
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> ListRef<BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElCodeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.code", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElContainerConfigurationEl {
    container_uri: PrimField<String>,
}

impl BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElContainerConfigurationEl { }

impl ToListMappable for BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElContainerConfigurationEl {
    type O = BlockAssignable<BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElContainerConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntimeAgentRuntimeArtifactElContainerConfigurationEl {
    #[doc = ""]
    pub container_uri: PrimField<String>,
}

impl BuildBedrockagentcoreAgentRuntimeAgentRuntimeArtifactElContainerConfigurationEl {
    pub fn build(self) -> BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElContainerConfigurationEl {
        BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElContainerConfigurationEl {
            container_uri: self.container_uri,
        }
    }
}

pub struct BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElContainerConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElContainerConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElContainerConfigurationElRef {
        BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElContainerConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElContainerConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_uri` after provisioning.\n"]
    pub fn container_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElDynamic {
    code_configuration: Option<DynamicBlock<BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationEl>>,
    container_configuration: Option<
        DynamicBlock<BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElContainerConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntimeAgentRuntimeArtifactEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code_configuration: Option<Vec<BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_configuration: Option<Vec<BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElContainerConfigurationEl>>,
    dynamic: BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElDynamic,
}

impl BedrockagentcoreAgentRuntimeAgentRuntimeArtifactEl {
    #[doc = "Set the field `code_configuration`.\n"]
    pub fn set_code_configuration(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `container_configuration`.\n"]
    pub fn set_container_configuration(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElContainerConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.container_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.container_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentcoreAgentRuntimeAgentRuntimeArtifactEl {
    type O = BlockAssignable<BedrockagentcoreAgentRuntimeAgentRuntimeArtifactEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntimeAgentRuntimeArtifactEl {}

impl BuildBedrockagentcoreAgentRuntimeAgentRuntimeArtifactEl {
    pub fn build(self) -> BedrockagentcoreAgentRuntimeAgentRuntimeArtifactEl {
        BedrockagentcoreAgentRuntimeAgentRuntimeArtifactEl {
            code_configuration: core::default::Default::default(),
            container_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElRef {
        BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code_configuration` after provisioning.\n"]
    pub fn code_configuration(
        &self,
    ) -> ListRef<BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElCodeConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.code_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `container_configuration` after provisioning.\n"]
    pub fn container_configuration(
        &self,
    ) -> ListRef<BedrockagentcoreAgentRuntimeAgentRuntimeArtifactElContainerConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.container_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntimeAuthorizerConfigurationElCustomJwtAuthorizerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_audience: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_clients: Option<SetField<PrimField<String>>>,
    discovery_url: PrimField<String>,
}

impl BedrockagentcoreAgentRuntimeAuthorizerConfigurationElCustomJwtAuthorizerEl {
    #[doc = "Set the field `allowed_audience`.\n"]
    pub fn set_allowed_audience(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allowed_audience = Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_clients`.\n"]
    pub fn set_allowed_clients(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allowed_clients = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentcoreAgentRuntimeAuthorizerConfigurationElCustomJwtAuthorizerEl {
    type O = BlockAssignable<BedrockagentcoreAgentRuntimeAuthorizerConfigurationElCustomJwtAuthorizerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntimeAuthorizerConfigurationElCustomJwtAuthorizerEl {
    #[doc = ""]
    pub discovery_url: PrimField<String>,
}

impl BuildBedrockagentcoreAgentRuntimeAuthorizerConfigurationElCustomJwtAuthorizerEl {
    pub fn build(self) -> BedrockagentcoreAgentRuntimeAuthorizerConfigurationElCustomJwtAuthorizerEl {
        BedrockagentcoreAgentRuntimeAuthorizerConfigurationElCustomJwtAuthorizerEl {
            allowed_audience: core::default::Default::default(),
            allowed_clients: core::default::Default::default(),
            discovery_url: self.discovery_url,
        }
    }
}

pub struct BedrockagentcoreAgentRuntimeAuthorizerConfigurationElCustomJwtAuthorizerElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreAgentRuntimeAuthorizerConfigurationElCustomJwtAuthorizerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreAgentRuntimeAuthorizerConfigurationElCustomJwtAuthorizerElRef {
        BedrockagentcoreAgentRuntimeAuthorizerConfigurationElCustomJwtAuthorizerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntimeAuthorizerConfigurationElCustomJwtAuthorizerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_audience` after provisioning.\n"]
    pub fn allowed_audience(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allowed_audience", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_clients` after provisioning.\n"]
    pub fn allowed_clients(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allowed_clients", self.base))
    }

    #[doc = "Get a reference to the value of field `discovery_url` after provisioning.\n"]
    pub fn discovery_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.discovery_url", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreAgentRuntimeAuthorizerConfigurationElDynamic {
    custom_jwt_authorizer: Option<
        DynamicBlock<BedrockagentcoreAgentRuntimeAuthorizerConfigurationElCustomJwtAuthorizerEl>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntimeAuthorizerConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_jwt_authorizer: Option<Vec<BedrockagentcoreAgentRuntimeAuthorizerConfigurationElCustomJwtAuthorizerEl>>,
    dynamic: BedrockagentcoreAgentRuntimeAuthorizerConfigurationElDynamic,
}

impl BedrockagentcoreAgentRuntimeAuthorizerConfigurationEl {
    #[doc = "Set the field `custom_jwt_authorizer`.\n"]
    pub fn set_custom_jwt_authorizer(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentcoreAgentRuntimeAuthorizerConfigurationElCustomJwtAuthorizerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_jwt_authorizer = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_jwt_authorizer = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentcoreAgentRuntimeAuthorizerConfigurationEl {
    type O = BlockAssignable<BedrockagentcoreAgentRuntimeAuthorizerConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntimeAuthorizerConfigurationEl {}

impl BuildBedrockagentcoreAgentRuntimeAuthorizerConfigurationEl {
    pub fn build(self) -> BedrockagentcoreAgentRuntimeAuthorizerConfigurationEl {
        BedrockagentcoreAgentRuntimeAuthorizerConfigurationEl {
            custom_jwt_authorizer: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentcoreAgentRuntimeAuthorizerConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreAgentRuntimeAuthorizerConfigurationElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentcoreAgentRuntimeAuthorizerConfigurationElRef {
        BedrockagentcoreAgentRuntimeAuthorizerConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntimeAuthorizerConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_jwt_authorizer` after provisioning.\n"]
    pub fn custom_jwt_authorizer(
        &self,
    ) -> ListRef<BedrockagentcoreAgentRuntimeAuthorizerConfigurationElCustomJwtAuthorizerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_jwt_authorizer", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntimeNetworkConfigurationElNetworkModeConfigEl {
    security_groups: SetField<PrimField<String>>,
    subnets: SetField<PrimField<String>>,
}

impl BedrockagentcoreAgentRuntimeNetworkConfigurationElNetworkModeConfigEl { }

impl ToListMappable for BedrockagentcoreAgentRuntimeNetworkConfigurationElNetworkModeConfigEl {
    type O = BlockAssignable<BedrockagentcoreAgentRuntimeNetworkConfigurationElNetworkModeConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntimeNetworkConfigurationElNetworkModeConfigEl {
    #[doc = ""]
    pub security_groups: SetField<PrimField<String>>,
    #[doc = ""]
    pub subnets: SetField<PrimField<String>>,
}

impl BuildBedrockagentcoreAgentRuntimeNetworkConfigurationElNetworkModeConfigEl {
    pub fn build(self) -> BedrockagentcoreAgentRuntimeNetworkConfigurationElNetworkModeConfigEl {
        BedrockagentcoreAgentRuntimeNetworkConfigurationElNetworkModeConfigEl {
            security_groups: self.security_groups,
            subnets: self.subnets,
        }
    }
}

pub struct BedrockagentcoreAgentRuntimeNetworkConfigurationElNetworkModeConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreAgentRuntimeNetworkConfigurationElNetworkModeConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreAgentRuntimeNetworkConfigurationElNetworkModeConfigElRef {
        BedrockagentcoreAgentRuntimeNetworkConfigurationElNetworkModeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntimeNetworkConfigurationElNetworkModeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreAgentRuntimeNetworkConfigurationElDynamic {
    network_mode_config: Option<DynamicBlock<BedrockagentcoreAgentRuntimeNetworkConfigurationElNetworkModeConfigEl>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntimeNetworkConfigurationEl {
    network_mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_mode_config: Option<Vec<BedrockagentcoreAgentRuntimeNetworkConfigurationElNetworkModeConfigEl>>,
    dynamic: BedrockagentcoreAgentRuntimeNetworkConfigurationElDynamic,
}

impl BedrockagentcoreAgentRuntimeNetworkConfigurationEl {
    #[doc = "Set the field `network_mode_config`.\n"]
    pub fn set_network_mode_config(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentcoreAgentRuntimeNetworkConfigurationElNetworkModeConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_mode_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_mode_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentcoreAgentRuntimeNetworkConfigurationEl {
    type O = BlockAssignable<BedrockagentcoreAgentRuntimeNetworkConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntimeNetworkConfigurationEl {
    #[doc = ""]
    pub network_mode: PrimField<String>,
}

impl BuildBedrockagentcoreAgentRuntimeNetworkConfigurationEl {
    pub fn build(self) -> BedrockagentcoreAgentRuntimeNetworkConfigurationEl {
        BedrockagentcoreAgentRuntimeNetworkConfigurationEl {
            network_mode: self.network_mode,
            network_mode_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentcoreAgentRuntimeNetworkConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreAgentRuntimeNetworkConfigurationElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentcoreAgentRuntimeNetworkConfigurationElRef {
        BedrockagentcoreAgentRuntimeNetworkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntimeNetworkConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_mode` after provisioning.\n"]
    pub fn network_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `network_mode_config` after provisioning.\n"]
    pub fn network_mode_config(
        &self,
    ) -> ListRef<BedrockagentcoreAgentRuntimeNetworkConfigurationElNetworkModeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_mode_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntimeProtocolConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    server_protocol: Option<PrimField<String>>,
}

impl BedrockagentcoreAgentRuntimeProtocolConfigurationEl {
    #[doc = "Set the field `server_protocol`.\n"]
    pub fn set_server_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.server_protocol = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentcoreAgentRuntimeProtocolConfigurationEl {
    type O = BlockAssignable<BedrockagentcoreAgentRuntimeProtocolConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntimeProtocolConfigurationEl {}

impl BuildBedrockagentcoreAgentRuntimeProtocolConfigurationEl {
    pub fn build(self) -> BedrockagentcoreAgentRuntimeProtocolConfigurationEl {
        BedrockagentcoreAgentRuntimeProtocolConfigurationEl { server_protocol: core::default::Default::default() }
    }
}

pub struct BedrockagentcoreAgentRuntimeProtocolConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreAgentRuntimeProtocolConfigurationElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentcoreAgentRuntimeProtocolConfigurationElRef {
        BedrockagentcoreAgentRuntimeProtocolConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntimeProtocolConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `server_protocol` after provisioning.\n"]
    pub fn server_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_protocol", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntimeRequestHeaderConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    request_header_allowlist: Option<SetField<PrimField<String>>>,
}

impl BedrockagentcoreAgentRuntimeRequestHeaderConfigurationEl {
    #[doc = "Set the field `request_header_allowlist`.\n"]
    pub fn set_request_header_allowlist(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.request_header_allowlist = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentcoreAgentRuntimeRequestHeaderConfigurationEl {
    type O = BlockAssignable<BedrockagentcoreAgentRuntimeRequestHeaderConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntimeRequestHeaderConfigurationEl {}

impl BuildBedrockagentcoreAgentRuntimeRequestHeaderConfigurationEl {
    pub fn build(self) -> BedrockagentcoreAgentRuntimeRequestHeaderConfigurationEl {
        BedrockagentcoreAgentRuntimeRequestHeaderConfigurationEl {
            request_header_allowlist: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentcoreAgentRuntimeRequestHeaderConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreAgentRuntimeRequestHeaderConfigurationElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentcoreAgentRuntimeRequestHeaderConfigurationElRef {
        BedrockagentcoreAgentRuntimeRequestHeaderConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntimeRequestHeaderConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `request_header_allowlist` after provisioning.\n"]
    pub fn request_header_allowlist(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.request_header_allowlist", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntimeTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl BedrockagentcoreAgentRuntimeTimeoutsEl {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentcoreAgentRuntimeTimeoutsEl {
    type O = BlockAssignable<BedrockagentcoreAgentRuntimeTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntimeTimeoutsEl {}

impl BuildBedrockagentcoreAgentRuntimeTimeoutsEl {
    pub fn build(self) -> BedrockagentcoreAgentRuntimeTimeoutsEl {
        BedrockagentcoreAgentRuntimeTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentcoreAgentRuntimeTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreAgentRuntimeTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentcoreAgentRuntimeTimeoutsElRef {
        BedrockagentcoreAgentRuntimeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntimeTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreAgentRuntimeDynamic {
    agent_runtime_artifact: Option<DynamicBlock<BedrockagentcoreAgentRuntimeAgentRuntimeArtifactEl>>,
    authorizer_configuration: Option<DynamicBlock<BedrockagentcoreAgentRuntimeAuthorizerConfigurationEl>>,
    network_configuration: Option<DynamicBlock<BedrockagentcoreAgentRuntimeNetworkConfigurationEl>>,
    protocol_configuration: Option<DynamicBlock<BedrockagentcoreAgentRuntimeProtocolConfigurationEl>>,
    request_header_configuration: Option<DynamicBlock<BedrockagentcoreAgentRuntimeRequestHeaderConfigurationEl>>,
}
