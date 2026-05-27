use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SccProjectNotificationConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    config_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    pubsub_topic: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    streaming_config: Option<Vec<SccProjectNotificationConfigStreamingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SccProjectNotificationConfigTimeoutsEl>,
    dynamic: SccProjectNotificationConfigDynamic,
}
struct SccProjectNotificationConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SccProjectNotificationConfigData>,
}
#[derive(Clone)]
pub struct SccProjectNotificationConfig(Rc<SccProjectNotificationConfig_>);
impl SccProjectNotificationConfig {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nThe description of the notification config (max of 1024 characters)."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `streaming_config`.\n"]
    pub fn set_streaming_config(
        self,
        v: impl Into<BlockAssignable<SccProjectNotificationConfigStreamingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().streaming_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.streaming_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SccProjectNotificationConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `config_id` after provisioning.\nThis must be unique within the organization."]
    pub fn config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the notification config (max of 1024 characters)."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of this notification config, in the format\n'projects/{{projectId}}/notificationConfigs/{{config_id}}'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pubsub_topic` after provisioning.\nThe Pub/Sub topic to send notifications to. Its format is\n\"projects/[project_id]/topics/[topic]\"."]
    pub fn pubsub_topic(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pubsub_topic", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe service account that needs \"pubsub.topics.publish\" permission to\npublish to the Pub/Sub topic."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `streaming_config` after provisioning.\n"]
    pub fn streaming_config(&self) -> ListRef<SccProjectNotificationConfigStreamingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.streaming_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SccProjectNotificationConfigTimeoutsElRef {
        SccProjectNotificationConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for SccProjectNotificationConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SccProjectNotificationConfig {}
impl ToListMappable for SccProjectNotificationConfig {
    type O = ListRef<SccProjectNotificationConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SccProjectNotificationConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_scc_project_notification_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSccProjectNotificationConfig {
    pub tf_id: String,
    #[doc = "This must be unique within the organization."]
    pub config_id: PrimField<String>,
    #[doc = "The Pub/Sub topic to send notifications to. Its format is\n\"projects/[project_id]/topics/[topic]\"."]
    pub pubsub_topic: PrimField<String>,
}
impl BuildSccProjectNotificationConfig {
    pub fn build(self, stack: &mut Stack) -> SccProjectNotificationConfig {
        let out = SccProjectNotificationConfig(Rc::new(SccProjectNotificationConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SccProjectNotificationConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                config_id: self.config_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                pubsub_topic: self.pubsub_topic,
                streaming_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SccProjectNotificationConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for SccProjectNotificationConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SccProjectNotificationConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `config_id` after provisioning.\nThis must be unique within the organization."]
    pub fn config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the notification config (max of 1024 characters)."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of this notification config, in the format\n'projects/{{projectId}}/notificationConfigs/{{config_id}}'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pubsub_topic` after provisioning.\nThe Pub/Sub topic to send notifications to. Its format is\n\"projects/[project_id]/topics/[topic]\"."]
    pub fn pubsub_topic(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pubsub_topic", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe service account that needs \"pubsub.topics.publish\" permission to\npublish to the Pub/Sub topic."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `streaming_config` after provisioning.\n"]
    pub fn streaming_config(&self) -> ListRef<SccProjectNotificationConfigStreamingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.streaming_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SccProjectNotificationConfigTimeoutsElRef {
        SccProjectNotificationConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SccProjectNotificationConfigStreamingConfigEl {
    filter: PrimField<String>,
}
impl SccProjectNotificationConfigStreamingConfigEl {}
impl ToListMappable for SccProjectNotificationConfigStreamingConfigEl {
    type O = BlockAssignable<SccProjectNotificationConfigStreamingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSccProjectNotificationConfigStreamingConfigEl {
    #[doc = "Expression that defines the filter to apply across create/update\nevents of assets or findings as specified by the event type. The\nexpression is a list of zero or more restrictions combined via\nlogical operators AND and OR. Parentheses are supported, and OR\nhas higher precedence than AND.\n\nRestrictions have the form <field> <operator> <value> and may have\na - character in front of them to indicate negation. The fields\nmap to those defined in the corresponding resource.\n\nThe supported operators are:\n\n* = for all value types.\n* >, <, >=, <= for integer values.\n* :, meaning substring matching, for strings.\n\nThe supported value types are:\n\n* string literals in quotes.\n* integer literals without quotes.\n* boolean literals true and false without quotes.\n\nSee\n[Filtering notifications](https://cloud.google.com/security-command-center/docs/how-to-api-filter-notifications)\nfor information on how to write a filter."]
    pub filter: PrimField<String>,
}
impl BuildSccProjectNotificationConfigStreamingConfigEl {
    pub fn build(self) -> SccProjectNotificationConfigStreamingConfigEl {
        SccProjectNotificationConfigStreamingConfigEl {
            filter: self.filter,
        }
    }
}
pub struct SccProjectNotificationConfigStreamingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SccProjectNotificationConfigStreamingConfigElRef {
    fn new(shared: StackShared, base: String) -> SccProjectNotificationConfigStreamingConfigElRef {
        SccProjectNotificationConfigStreamingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SccProjectNotificationConfigStreamingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nExpression that defines the filter to apply across create/update\nevents of assets or findings as specified by the event type. The\nexpression is a list of zero or more restrictions combined via\nlogical operators AND and OR. Parentheses are supported, and OR\nhas higher precedence than AND.\n\nRestrictions have the form <field> <operator> <value> and may have\na - character in front of them to indicate negation. The fields\nmap to those defined in the corresponding resource.\n\nThe supported operators are:\n\n* = for all value types.\n* >, <, >=, <= for integer values.\n* :, meaning substring matching, for strings.\n\nThe supported value types are:\n\n* string literals in quotes.\n* integer literals without quotes.\n* boolean literals true and false without quotes.\n\nSee\n[Filtering notifications](https://cloud.google.com/security-command-center/docs/how-to-api-filter-notifications)\nfor information on how to write a filter."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}
#[derive(Serialize)]
pub struct SccProjectNotificationConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl SccProjectNotificationConfigTimeoutsEl {
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
impl ToListMappable for SccProjectNotificationConfigTimeoutsEl {
    type O = BlockAssignable<SccProjectNotificationConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSccProjectNotificationConfigTimeoutsEl {}
impl BuildSccProjectNotificationConfigTimeoutsEl {
    pub fn build(self) -> SccProjectNotificationConfigTimeoutsEl {
        SccProjectNotificationConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct SccProjectNotificationConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SccProjectNotificationConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SccProjectNotificationConfigTimeoutsElRef {
        SccProjectNotificationConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SccProjectNotificationConfigTimeoutsElRef {
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
struct SccProjectNotificationConfigDynamic {
    streaming_config: Option<DynamicBlock<SccProjectNotificationConfigStreamingConfigEl>>,
}
