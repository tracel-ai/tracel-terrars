use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ChatbotTeamsChannelConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    channel_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_name: Option<PrimField<String>>,
    configuration_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guardrail_policy_arns: Option<ListField<PrimField<String>>>,
    iam_role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sns_topic_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    team_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    team_name: Option<PrimField<String>>,
    tenant_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_authorization_required: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ChatbotTeamsChannelConfigurationTimeoutsEl>,
}

struct ChatbotTeamsChannelConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ChatbotTeamsChannelConfigurationData>,
}

#[derive(Clone)]
pub struct ChatbotTeamsChannelConfiguration(Rc<ChatbotTeamsChannelConfiguration_>);

impl ChatbotTeamsChannelConfiguration {
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

    #[doc = "Set the field `channel_name`.\n"]
    pub fn set_channel_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().channel_name = Some(v.into());
        self
    }

    #[doc = "Set the field `guardrail_policy_arns`.\n"]
    pub fn set_guardrail_policy_arns(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().guardrail_policy_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `logging_level`.\n"]
    pub fn set_logging_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().logging_level = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `sns_topic_arns`.\n"]
    pub fn set_sns_topic_arns(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().sns_topic_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `team_name`.\n"]
    pub fn set_team_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().team_name = Some(v.into());
        self
    }

    #[doc = "Set the field `user_authorization_required`.\n"]
    pub fn set_user_authorization_required(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().user_authorization_required = Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ChatbotTeamsChannelConfigurationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `channel_id` after provisioning.\n"]
    pub fn channel_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.channel_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `channel_name` after provisioning.\n"]
    pub fn channel_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.channel_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `chat_configuration_arn` after provisioning.\n"]
    pub fn chat_configuration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.chat_configuration_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `configuration_name` after provisioning.\n"]
    pub fn configuration_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `guardrail_policy_arns` after provisioning.\n"]
    pub fn guardrail_policy_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guardrail_policy_arns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `logging_level` after provisioning.\n"]
    pub fn logging_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logging_level", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sns_topic_arns` after provisioning.\n"]
    pub fn sns_topic_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.sns_topic_arns", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `team_id` after provisioning.\n"]
    pub fn team_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.team_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `team_name` after provisioning.\n"]
    pub fn team_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.team_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tenant_id` after provisioning.\n"]
    pub fn tenant_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tenant_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_authorization_required` after provisioning.\n"]
    pub fn user_authorization_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_authorization_required", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChatbotTeamsChannelConfigurationTimeoutsElRef {
        ChatbotTeamsChannelConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for ChatbotTeamsChannelConfiguration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for ChatbotTeamsChannelConfiguration {}

impl ToListMappable for ChatbotTeamsChannelConfiguration {
    type O = ListRef<ChatbotTeamsChannelConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ChatbotTeamsChannelConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_chatbot_teams_channel_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildChatbotTeamsChannelConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub channel_id: PrimField<String>,
    #[doc = ""]
    pub configuration_name: PrimField<String>,
    #[doc = ""]
    pub iam_role_arn: PrimField<String>,
    #[doc = ""]
    pub team_id: PrimField<String>,
    #[doc = ""]
    pub tenant_id: PrimField<String>,
}

impl BuildChatbotTeamsChannelConfiguration {
    pub fn build(self, stack: &mut Stack) -> ChatbotTeamsChannelConfiguration {
        let out = ChatbotTeamsChannelConfiguration(Rc::new(ChatbotTeamsChannelConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ChatbotTeamsChannelConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                channel_id: self.channel_id,
                channel_name: core::default::Default::default(),
                configuration_name: self.configuration_name,
                guardrail_policy_arns: core::default::Default::default(),
                iam_role_arn: self.iam_role_arn,
                logging_level: core::default::Default::default(),
                region: core::default::Default::default(),
                sns_topic_arns: core::default::Default::default(),
                tags: core::default::Default::default(),
                team_id: self.team_id,
                team_name: core::default::Default::default(),
                tenant_id: self.tenant_id,
                user_authorization_required: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ChatbotTeamsChannelConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for ChatbotTeamsChannelConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl ChatbotTeamsChannelConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `channel_id` after provisioning.\n"]
    pub fn channel_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.channel_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `channel_name` after provisioning.\n"]
    pub fn channel_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.channel_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `chat_configuration_arn` after provisioning.\n"]
    pub fn chat_configuration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.chat_configuration_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `configuration_name` after provisioning.\n"]
    pub fn configuration_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `guardrail_policy_arns` after provisioning.\n"]
    pub fn guardrail_policy_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guardrail_policy_arns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `logging_level` after provisioning.\n"]
    pub fn logging_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logging_level", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sns_topic_arns` after provisioning.\n"]
    pub fn sns_topic_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.sns_topic_arns", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `team_id` after provisioning.\n"]
    pub fn team_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.team_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `team_name` after provisioning.\n"]
    pub fn team_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.team_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tenant_id` after provisioning.\n"]
    pub fn tenant_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tenant_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_authorization_required` after provisioning.\n"]
    pub fn user_authorization_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_authorization_required", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChatbotTeamsChannelConfigurationTimeoutsElRef {
        ChatbotTeamsChannelConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ChatbotTeamsChannelConfigurationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl ChatbotTeamsChannelConfigurationTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for ChatbotTeamsChannelConfigurationTimeoutsEl {
    type O = BlockAssignable<ChatbotTeamsChannelConfigurationTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChatbotTeamsChannelConfigurationTimeoutsEl {}

impl BuildChatbotTeamsChannelConfigurationTimeoutsEl {
    pub fn build(self) -> ChatbotTeamsChannelConfigurationTimeoutsEl {
        ChatbotTeamsChannelConfigurationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ChatbotTeamsChannelConfigurationTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ChatbotTeamsChannelConfigurationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ChatbotTeamsChannelConfigurationTimeoutsElRef {
        ChatbotTeamsChannelConfigurationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChatbotTeamsChannelConfigurationTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
