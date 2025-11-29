use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentAgentAliasData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    agent_alias_name: PrimField<String>,
    agent_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_configuration: Option<ListField<BedrockagentAgentAliasRoutingConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BedrockagentAgentAliasTimeoutsEl>,
}

struct BedrockagentAgentAlias_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BedrockagentAgentAliasData>,
}

#[derive(Clone)]
pub struct BedrockagentAgentAlias(Rc<BedrockagentAgentAlias_>);

impl BedrockagentAgentAlias {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `routing_configuration`.\n"]
    pub fn set_routing_configuration(
        self,
        v: impl Into<ListField<BedrockagentAgentAliasRoutingConfigurationEl>>,
    ) -> Self {
        self.0.data.borrow_mut().routing_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BedrockagentAgentAliasTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `agent_alias_arn` after provisioning.\n"]
    pub fn agent_alias_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_alias_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_alias_id` after provisioning.\n"]
    pub fn agent_alias_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_alias_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_alias_name` after provisioning.\n"]
    pub fn agent_alias_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_alias_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\n"]
    pub fn agent_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing_configuration` after provisioning.\n"]
    pub fn routing_configuration(&self) -> ListRef<BedrockagentAgentAliasRoutingConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.routing_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentAgentAliasTimeoutsElRef {
        BedrockagentAgentAliasTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for BedrockagentAgentAlias {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for BedrockagentAgentAlias { }

impl ToListMappable for BedrockagentAgentAlias {
    type O = ListRef<BedrockagentAgentAliasRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BedrockagentAgentAlias_ {
    fn extract_resource_type(&self) -> String {
        "aws_bedrockagent_agent_alias".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentAgentAlias {
    pub tf_id: String,
    #[doc = ""]
    pub agent_alias_name: PrimField<String>,
    #[doc = ""]
    pub agent_id: PrimField<String>,
}

impl BuildBedrockagentAgentAlias {
    pub fn build(self, stack: &mut Stack) -> BedrockagentAgentAlias {
        let out = BedrockagentAgentAlias(Rc::new(BedrockagentAgentAlias_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BedrockagentAgentAliasData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                agent_alias_name: self.agent_alias_name,
                agent_id: self.agent_id,
                description: core::default::Default::default(),
                region: core::default::Default::default(),
                routing_configuration: core::default::Default::default(),
                tags: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentAgentAliasRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentAgentAliasRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl BedrockagentAgentAliasRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_alias_arn` after provisioning.\n"]
    pub fn agent_alias_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_alias_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_alias_id` after provisioning.\n"]
    pub fn agent_alias_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_alias_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_alias_name` after provisioning.\n"]
    pub fn agent_alias_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_alias_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\n"]
    pub fn agent_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing_configuration` after provisioning.\n"]
    pub fn routing_configuration(&self) -> ListRef<BedrockagentAgentAliasRoutingConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.routing_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentAgentAliasTimeoutsElRef {
        BedrockagentAgentAliasTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BedrockagentAgentAliasRoutingConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioned_throughput: Option<PrimField<String>>,
}

impl BedrockagentAgentAliasRoutingConfigurationEl {
    #[doc = "Set the field `agent_version`.\n"]
    pub fn set_agent_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.agent_version = Some(v.into());
        self
    }

    #[doc = "Set the field `provisioned_throughput`.\n"]
    pub fn set_provisioned_throughput(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.provisioned_throughput = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentAgentAliasRoutingConfigurationEl {
    type O = BlockAssignable<BedrockagentAgentAliasRoutingConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgentAliasRoutingConfigurationEl {}

impl BuildBedrockagentAgentAliasRoutingConfigurationEl {
    pub fn build(self) -> BedrockagentAgentAliasRoutingConfigurationEl {
        BedrockagentAgentAliasRoutingConfigurationEl {
            agent_version: core::default::Default::default(),
            provisioned_throughput: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentAgentAliasRoutingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentAgentAliasRoutingConfigurationElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentAgentAliasRoutingConfigurationElRef {
        BedrockagentAgentAliasRoutingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgentAliasRoutingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_version` after provisioning.\n"]
    pub fn agent_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_version", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioned_throughput` after provisioning.\n"]
    pub fn provisioned_throughput(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioned_throughput", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentAgentAliasTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl BedrockagentAgentAliasTimeoutsEl {
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

impl ToListMappable for BedrockagentAgentAliasTimeoutsEl {
    type O = BlockAssignable<BedrockagentAgentAliasTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgentAliasTimeoutsEl {}

impl BuildBedrockagentAgentAliasTimeoutsEl {
    pub fn build(self) -> BedrockagentAgentAliasTimeoutsEl {
        BedrockagentAgentAliasTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentAgentAliasTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentAgentAliasTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentAgentAliasTimeoutsElRef {
        BedrockagentAgentAliasTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgentAliasTimeoutsElRef {
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
