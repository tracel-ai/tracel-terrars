use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct LoggingProjectSinkData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_writer_identity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    destination: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_writer_identity: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bigquery_options: Option<Vec<LoggingProjectSinkBigqueryOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusions: Option<Vec<LoggingProjectSinkExclusionsEl>>,
    dynamic: LoggingProjectSinkDynamic,
}
struct LoggingProjectSink_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LoggingProjectSinkData>,
}
#[derive(Clone)]
pub struct LoggingProjectSink(Rc<LoggingProjectSink_>);
impl LoggingProjectSink {
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
    #[doc = "Set the field `custom_writer_identity`.\nA service account provided by the caller that will be used to write the log entries. The format must be serviceAccount:some@email. This field can only be specified if you are routing logs to a destination outside this sink's project. If not specified, a Logging service account will automatically be generated."]
    pub fn set_custom_writer_identity(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().custom_writer_identity = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA description of this sink. The maximum length of the description is 8000 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nIf set to True, then this sink is disabled and it does not export any log entries."]
    pub fn set_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `filter`.\nThe filter to apply when exporting logs. Only log entries that match the filter are exported."]
    pub fn set_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe ID of the project to create the sink in. If omitted, the project associated with the provider is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `unique_writer_identity`.\nWhether or not to create a unique identity associated with this sink. If false (the legacy behavior), then the writer_identity used is serviceAccount:cloud-logs@system.gserviceaccount.com. If true (default), then a unique service account is created and used for this sink. If you wish to publish logs across projects, you must set unique_writer_identity to true."]
    pub fn set_unique_writer_identity(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().unique_writer_identity = Some(v.into());
        self
    }
    #[doc = "Set the field `bigquery_options`.\n"]
    pub fn set_bigquery_options(
        self,
        v: impl Into<BlockAssignable<LoggingProjectSinkBigqueryOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bigquery_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bigquery_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `exclusions`.\n"]
    pub fn set_exclusions(
        self,
        v: impl Into<BlockAssignable<LoggingProjectSinkExclusionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().exclusions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.exclusions = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `custom_writer_identity` after provisioning.\nA service account provided by the caller that will be used to write the log entries. The format must be serviceAccount:some@email. This field can only be specified if you are routing logs to a destination outside this sink's project. If not specified, a Logging service account will automatically be generated."]
    pub fn custom_writer_identity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_writer_identity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of this sink. The maximum length of the description is 8000 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination` after provisioning.\nThe destination of the sink (or, in other words, where logs are written to). Can be a Cloud Storage bucket, a PubSub topic, or a BigQuery dataset. Examples: \"storage.googleapis.com/[GCS_BUCKET]\" \"bigquery.googleapis.com/projects/[PROJECT_ID]/datasets/[DATASET]\" \"pubsub.googleapis.com/projects/[PROJECT_ID]/topics/[TOPIC_ID]\" The writer associated with the sink must have access to write to the above resource."]
    pub fn destination(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nIf set to True, then this sink is disabled and it does not export any log entries."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nThe filter to apply when exporting logs. Only log entries that match the filter are exported."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the logging sink."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project to create the sink in. If omitted, the project associated with the provider is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `unique_writer_identity` after provisioning.\nWhether or not to create a unique identity associated with this sink. If false (the legacy behavior), then the writer_identity used is serviceAccount:cloud-logs@system.gserviceaccount.com. If true (default), then a unique service account is created and used for this sink. If you wish to publish logs across projects, you must set unique_writer_identity to true."]
    pub fn unique_writer_identity(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unique_writer_identity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `writer_identity` after provisioning.\nThe identity associated with this sink. This identity must be granted write access to the configured destination."]
    pub fn writer_identity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.writer_identity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bigquery_options` after provisioning.\n"]
    pub fn bigquery_options(&self) -> ListRef<LoggingProjectSinkBigqueryOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigquery_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `exclusions` after provisioning.\n"]
    pub fn exclusions(&self) -> ListRef<LoggingProjectSinkExclusionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclusions", self.extract_ref()),
        )
    }
}
impl Referable for LoggingProjectSink {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for LoggingProjectSink {}
impl ToListMappable for LoggingProjectSink {
    type O = ListRef<LoggingProjectSinkRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for LoggingProjectSink_ {
    fn extract_resource_type(&self) -> String {
        "google_logging_project_sink".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildLoggingProjectSink {
    pub tf_id: String,
    #[doc = "The destination of the sink (or, in other words, where logs are written to). Can be a Cloud Storage bucket, a PubSub topic, or a BigQuery dataset. Examples: \"storage.googleapis.com/[GCS_BUCKET]\" \"bigquery.googleapis.com/projects/[PROJECT_ID]/datasets/[DATASET]\" \"pubsub.googleapis.com/projects/[PROJECT_ID]/topics/[TOPIC_ID]\" The writer associated with the sink must have access to write to the above resource."]
    pub destination: PrimField<String>,
    #[doc = "The name of the logging sink."]
    pub name: PrimField<String>,
}
impl BuildLoggingProjectSink {
    pub fn build(self, stack: &mut Stack) -> LoggingProjectSink {
        let out = LoggingProjectSink(Rc::new(LoggingProjectSink_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LoggingProjectSinkData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                custom_writer_identity: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                destination: self.destination,
                disabled: core::default::Default::default(),
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                unique_writer_identity: core::default::Default::default(),
                bigquery_options: core::default::Default::default(),
                exclusions: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct LoggingProjectSinkRef {
    shared: StackShared,
    base: String,
}
impl Ref for LoggingProjectSinkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl LoggingProjectSinkRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_writer_identity` after provisioning.\nA service account provided by the caller that will be used to write the log entries. The format must be serviceAccount:some@email. This field can only be specified if you are routing logs to a destination outside this sink's project. If not specified, a Logging service account will automatically be generated."]
    pub fn custom_writer_identity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_writer_identity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of this sink. The maximum length of the description is 8000 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination` after provisioning.\nThe destination of the sink (or, in other words, where logs are written to). Can be a Cloud Storage bucket, a PubSub topic, or a BigQuery dataset. Examples: \"storage.googleapis.com/[GCS_BUCKET]\" \"bigquery.googleapis.com/projects/[PROJECT_ID]/datasets/[DATASET]\" \"pubsub.googleapis.com/projects/[PROJECT_ID]/topics/[TOPIC_ID]\" The writer associated with the sink must have access to write to the above resource."]
    pub fn destination(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nIf set to True, then this sink is disabled and it does not export any log entries."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nThe filter to apply when exporting logs. Only log entries that match the filter are exported."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the logging sink."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project to create the sink in. If omitted, the project associated with the provider is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `unique_writer_identity` after provisioning.\nWhether or not to create a unique identity associated with this sink. If false (the legacy behavior), then the writer_identity used is serviceAccount:cloud-logs@system.gserviceaccount.com. If true (default), then a unique service account is created and used for this sink. If you wish to publish logs across projects, you must set unique_writer_identity to true."]
    pub fn unique_writer_identity(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unique_writer_identity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `writer_identity` after provisioning.\nThe identity associated with this sink. This identity must be granted write access to the configured destination."]
    pub fn writer_identity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.writer_identity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bigquery_options` after provisioning.\n"]
    pub fn bigquery_options(&self) -> ListRef<LoggingProjectSinkBigqueryOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigquery_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `exclusions` after provisioning.\n"]
    pub fn exclusions(&self) -> ListRef<LoggingProjectSinkExclusionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclusions", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct LoggingProjectSinkBigqueryOptionsEl {
    use_partitioned_tables: PrimField<bool>,
}
impl LoggingProjectSinkBigqueryOptionsEl {}
impl ToListMappable for LoggingProjectSinkBigqueryOptionsEl {
    type O = BlockAssignable<LoggingProjectSinkBigqueryOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLoggingProjectSinkBigqueryOptionsEl {
    #[doc = "Whether to use BigQuery's partition tables. By default, Logging creates dated tables based on the log entries' timestamps, e.g. syslog_20170523. With partitioned tables the date suffix is no longer present and special query syntax has to be used instead. In both cases, tables are sharded based on UTC timezone."]
    pub use_partitioned_tables: PrimField<bool>,
}
impl BuildLoggingProjectSinkBigqueryOptionsEl {
    pub fn build(self) -> LoggingProjectSinkBigqueryOptionsEl {
        LoggingProjectSinkBigqueryOptionsEl {
            use_partitioned_tables: self.use_partitioned_tables,
        }
    }
}
pub struct LoggingProjectSinkBigqueryOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LoggingProjectSinkBigqueryOptionsElRef {
    fn new(shared: StackShared, base: String) -> LoggingProjectSinkBigqueryOptionsElRef {
        LoggingProjectSinkBigqueryOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LoggingProjectSinkBigqueryOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `use_partitioned_tables` after provisioning.\nWhether to use BigQuery's partition tables. By default, Logging creates dated tables based on the log entries' timestamps, e.g. syslog_20170523. With partitioned tables the date suffix is no longer present and special query syntax has to be used instead. In both cases, tables are sharded based on UTC timezone."]
    pub fn use_partitioned_tables(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_partitioned_tables", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct LoggingProjectSinkExclusionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    filter: PrimField<String>,
    name: PrimField<String>,
}
impl LoggingProjectSinkExclusionsEl {
    #[doc = "Set the field `description`.\nA description of this exclusion."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nIf set to True, then this exclusion is disabled and it does not exclude any log entries"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}
impl ToListMappable for LoggingProjectSinkExclusionsEl {
    type O = BlockAssignable<LoggingProjectSinkExclusionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLoggingProjectSinkExclusionsEl {
    #[doc = "An advanced logs filter that matches the log entries to be excluded. By using the sample function, you can exclude less than 100% of the matching log entries"]
    pub filter: PrimField<String>,
    #[doc = "A client-assigned identifier, such as \"load-balancer-exclusion\". Identifiers are limited to 100 characters and can include only letters, digits, underscores, hyphens, and periods. First character has to be alphanumeric."]
    pub name: PrimField<String>,
}
impl BuildLoggingProjectSinkExclusionsEl {
    pub fn build(self) -> LoggingProjectSinkExclusionsEl {
        LoggingProjectSinkExclusionsEl {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            filter: self.filter,
            name: self.name,
        }
    }
}
pub struct LoggingProjectSinkExclusionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LoggingProjectSinkExclusionsElRef {
    fn new(shared: StackShared, base: String) -> LoggingProjectSinkExclusionsElRef {
        LoggingProjectSinkExclusionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LoggingProjectSinkExclusionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of this exclusion."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nIf set to True, then this exclusion is disabled and it does not exclude any log entries"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nAn advanced logs filter that matches the log entries to be excluded. By using the sample function, you can exclude less than 100% of the matching log entries"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA client-assigned identifier, such as \"load-balancer-exclusion\". Identifiers are limited to 100 characters and can include only letters, digits, underscores, hyphens, and periods. First character has to be alphanumeric."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize, Default)]
struct LoggingProjectSinkDynamic {
    bigquery_options: Option<DynamicBlock<LoggingProjectSinkBigqueryOptionsEl>>,
    exclusions: Option<DynamicBlock<LoggingProjectSinkExclusionsEl>>,
}
