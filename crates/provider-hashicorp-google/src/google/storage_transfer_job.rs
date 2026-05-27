use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct StorageTransferJobData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    description: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_stream: Option<Vec<StorageTransferJobEventStreamEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_config: Option<Vec<StorageTransferJobLoggingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_config: Option<Vec<StorageTransferJobNotificationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_spec: Option<Vec<StorageTransferJobReplicationSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<Vec<StorageTransferJobScheduleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_spec: Option<Vec<StorageTransferJobTransferSpecEl>>,
    dynamic: StorageTransferJobDynamic,
}
struct StorageTransferJob_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<StorageTransferJobData>,
}
#[derive(Clone)]
pub struct StorageTransferJob(Rc<StorageTransferJob_>);
impl StorageTransferJob {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nThe name of the Transfer Job."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\nThe user-managed service account to run the job. If this field is specified, the given service account is granted the necessary permissions to all applicable resources (e.g. GCS buckets) required by the job."]
    pub fn set_service_account(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\nStatus of the job. Default: ENABLED. NOTE: The effect of the new job status takes place during a subsequent job run. For example, if you change the job status from ENABLED to DISABLED, and an operation spawned by the transfer is running, the status change would not affect the current operation."]
    pub fn set_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().status = Some(v.into());
        self
    }
    #[doc = "Set the field `event_stream`.\n"]
    pub fn set_event_stream(
        self,
        v: impl Into<BlockAssignable<StorageTransferJobEventStreamEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_stream = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_stream = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        self,
        v: impl Into<BlockAssignable<StorageTransferJobLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `notification_config`.\n"]
    pub fn set_notification_config(
        self,
        v: impl Into<BlockAssignable<StorageTransferJobNotificationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `replication_spec`.\n"]
    pub fn set_replication_spec(
        self,
        v: impl Into<BlockAssignable<StorageTransferJobReplicationSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().replication_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.replication_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(self, v: impl Into<BlockAssignable<StorageTransferJobScheduleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schedule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schedule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `transfer_spec`.\n"]
    pub fn set_transfer_spec(
        self,
        v: impl Into<BlockAssignable<StorageTransferJobTransferSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().transfer_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.transfer_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\nWhen the Transfer Job was created."]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_time` after provisioning.\nWhen the Transfer Job was deleted."]
    pub fn deletion_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUnique description to identify the Transfer Job."]
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
    #[doc = "Get a reference to the value of field `last_modification_time` after provisioning.\nWhen the Transfer Job was last modified."]
    pub fn last_modification_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modification_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Transfer Job."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe user-managed service account to run the job. If this field is specified, the given service account is granted the necessary permissions to all applicable resources (e.g. GCS buckets) required by the job."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus of the job. Default: ENABLED. NOTE: The effect of the new job status takes place during a subsequent job run. For example, if you change the job status from ENABLED to DISABLED, and an operation spawned by the transfer is running, the status change would not affect the current operation."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_stream` after provisioning.\n"]
    pub fn event_stream(&self) -> ListRef<StorageTransferJobEventStreamElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_stream", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> ListRef<StorageTransferJobLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(&self) -> ListRef<StorageTransferJobNotificationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replication_spec` after provisioning.\n"]
    pub fn replication_spec(&self) -> ListRef<StorageTransferJobReplicationSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replication_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> ListRef<StorageTransferJobScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transfer_spec` after provisioning.\n"]
    pub fn transfer_spec(&self) -> ListRef<StorageTransferJobTransferSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.transfer_spec", self.extract_ref()),
        )
    }
}
impl Referable for StorageTransferJob {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for StorageTransferJob {}
impl ToListMappable for StorageTransferJob {
    type O = ListRef<StorageTransferJobRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for StorageTransferJob_ {
    fn extract_resource_type(&self) -> String {
        "google_storage_transfer_job".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildStorageTransferJob {
    pub tf_id: String,
    #[doc = "Unique description to identify the Transfer Job."]
    pub description: PrimField<String>,
}
impl BuildStorageTransferJob {
    pub fn build(self, stack: &mut Stack) -> StorageTransferJob {
        let out = StorageTransferJob(Rc::new(StorageTransferJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(StorageTransferJobData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: self.description,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                project: core::default::Default::default(),
                service_account: core::default::Default::default(),
                status: core::default::Default::default(),
                event_stream: core::default::Default::default(),
                logging_config: core::default::Default::default(),
                notification_config: core::default::Default::default(),
                replication_spec: core::default::Default::default(),
                schedule: core::default::Default::default(),
                transfer_spec: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct StorageTransferJobRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl StorageTransferJobRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\nWhen the Transfer Job was created."]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_time` after provisioning.\nWhen the Transfer Job was deleted."]
    pub fn deletion_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUnique description to identify the Transfer Job."]
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
    #[doc = "Get a reference to the value of field `last_modification_time` after provisioning.\nWhen the Transfer Job was last modified."]
    pub fn last_modification_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modification_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Transfer Job."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe user-managed service account to run the job. If this field is specified, the given service account is granted the necessary permissions to all applicable resources (e.g. GCS buckets) required by the job."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus of the job. Default: ENABLED. NOTE: The effect of the new job status takes place during a subsequent job run. For example, if you change the job status from ENABLED to DISABLED, and an operation spawned by the transfer is running, the status change would not affect the current operation."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_stream` after provisioning.\n"]
    pub fn event_stream(&self) -> ListRef<StorageTransferJobEventStreamElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_stream", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> ListRef<StorageTransferJobLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(&self) -> ListRef<StorageTransferJobNotificationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replication_spec` after provisioning.\n"]
    pub fn replication_spec(&self) -> ListRef<StorageTransferJobReplicationSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replication_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> ListRef<StorageTransferJobScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transfer_spec` after provisioning.\n"]
    pub fn transfer_spec(&self) -> ListRef<StorageTransferJobTransferSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.transfer_spec", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct StorageTransferJobEventStreamEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    event_stream_expiration_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_stream_start_time: Option<PrimField<String>>,
    name: PrimField<String>,
}
impl StorageTransferJobEventStreamEl {
    #[doc = "Set the field `event_stream_expiration_time`.\nSpecifies the data and time at which Storage Transfer Service stops listening for events from this stream. After this time, any transfers in progress will complete, but no new transfers are initiated"]
    pub fn set_event_stream_expiration_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.event_stream_expiration_time = Some(v.into());
        self
    }
    #[doc = "Set the field `event_stream_start_time`.\nSpecifies the date and time that Storage Transfer Service starts listening for events from this stream. If no start time is specified or start time is in the past, Storage Transfer Service starts listening immediately"]
    pub fn set_event_stream_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.event_stream_start_time = Some(v.into());
        self
    }
}
impl ToListMappable for StorageTransferJobEventStreamEl {
    type O = BlockAssignable<StorageTransferJobEventStreamEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobEventStreamEl {
    #[doc = "Specifies a unique name of the resource such as AWS SQS ARN in the form 'arn:aws:sqs:region:account_id:queue_name', or Pub/Sub subscription resource name in the form 'projects/{project}/subscriptions/{sub}'"]
    pub name: PrimField<String>,
}
impl BuildStorageTransferJobEventStreamEl {
    pub fn build(self) -> StorageTransferJobEventStreamEl {
        StorageTransferJobEventStreamEl {
            event_stream_expiration_time: core::default::Default::default(),
            event_stream_start_time: core::default::Default::default(),
            name: self.name,
        }
    }
}
pub struct StorageTransferJobEventStreamElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobEventStreamElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobEventStreamElRef {
        StorageTransferJobEventStreamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobEventStreamElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `event_stream_expiration_time` after provisioning.\nSpecifies the data and time at which Storage Transfer Service stops listening for events from this stream. After this time, any transfers in progress will complete, but no new transfers are initiated"]
    pub fn event_stream_expiration_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_stream_expiration_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `event_stream_start_time` after provisioning.\nSpecifies the date and time that Storage Transfer Service starts listening for events from this stream. If no start time is specified or start time is in the past, Storage Transfer Service starts listening immediately"]
    pub fn event_stream_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_stream_start_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nSpecifies a unique name of the resource such as AWS SQS ARN in the form 'arn:aws:sqs:region:account_id:queue_name', or Pub/Sub subscription resource name in the form 'projects/{project}/subscriptions/{sub}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageTransferJobLoggingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_on_prem_gcs_transfer_logs: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_action_states: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_actions: Option<ListField<PrimField<String>>>,
}
impl StorageTransferJobLoggingConfigEl {
    #[doc = "Set the field `enable_on_prem_gcs_transfer_logs`.\nFor transfers with a PosixFilesystem source, this option enables the Cloud Storage transfer logs for this transfer."]
    pub fn set_enable_on_prem_gcs_transfer_logs(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_on_prem_gcs_transfer_logs = Some(v.into());
        self
    }
    #[doc = "Set the field `log_action_states`.\nStates in which logActions are logged. Not supported for transfers with PosifxFilesystem data sources; use enable_on_prem_gcs_transfer_logs instead."]
    pub fn set_log_action_states(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.log_action_states = Some(v.into());
        self
    }
    #[doc = "Set the field `log_actions`.\nSpecifies the actions to be logged. Not supported for transfers with PosifxFilesystem data sources; use enable_on_prem_gcs_transfer_logs instead."]
    pub fn set_log_actions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.log_actions = Some(v.into());
        self
    }
}
impl ToListMappable for StorageTransferJobLoggingConfigEl {
    type O = BlockAssignable<StorageTransferJobLoggingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobLoggingConfigEl {}
impl BuildStorageTransferJobLoggingConfigEl {
    pub fn build(self) -> StorageTransferJobLoggingConfigEl {
        StorageTransferJobLoggingConfigEl {
            enable_on_prem_gcs_transfer_logs: core::default::Default::default(),
            log_action_states: core::default::Default::default(),
            log_actions: core::default::Default::default(),
        }
    }
}
pub struct StorageTransferJobLoggingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobLoggingConfigElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobLoggingConfigElRef {
        StorageTransferJobLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobLoggingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_on_prem_gcs_transfer_logs` after provisioning.\nFor transfers with a PosixFilesystem source, this option enables the Cloud Storage transfer logs for this transfer."]
    pub fn enable_on_prem_gcs_transfer_logs(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_on_prem_gcs_transfer_logs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `log_action_states` after provisioning.\nStates in which logActions are logged. Not supported for transfers with PosifxFilesystem data sources; use enable_on_prem_gcs_transfer_logs instead."]
    pub fn log_action_states(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_action_states", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `log_actions` after provisioning.\nSpecifies the actions to be logged. Not supported for transfers with PosifxFilesystem data sources; use enable_on_prem_gcs_transfer_logs instead."]
    pub fn log_actions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.log_actions", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageTransferJobNotificationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    event_types: Option<SetField<PrimField<String>>>,
    payload_format: PrimField<String>,
    pubsub_topic: PrimField<String>,
}
impl StorageTransferJobNotificationConfigEl {
    #[doc = "Set the field `event_types`.\nEvent types for which a notification is desired. If empty, send notifications for all event types. The valid types are \"TRANSFER_OPERATION_SUCCESS\", \"TRANSFER_OPERATION_FAILED\", \"TRANSFER_OPERATION_ABORTED\"."]
    pub fn set_event_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.event_types = Some(v.into());
        self
    }
}
impl ToListMappable for StorageTransferJobNotificationConfigEl {
    type O = BlockAssignable<StorageTransferJobNotificationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobNotificationConfigEl {
    #[doc = "The desired format of the notification message payloads. One of \"NONE\" or \"JSON\"."]
    pub payload_format: PrimField<String>,
    #[doc = "The Topic.name of the Pub/Sub topic to which to publish notifications."]
    pub pubsub_topic: PrimField<String>,
}
impl BuildStorageTransferJobNotificationConfigEl {
    pub fn build(self) -> StorageTransferJobNotificationConfigEl {
        StorageTransferJobNotificationConfigEl {
            event_types: core::default::Default::default(),
            payload_format: self.payload_format,
            pubsub_topic: self.pubsub_topic,
        }
    }
}
pub struct StorageTransferJobNotificationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobNotificationConfigElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobNotificationConfigElRef {
        StorageTransferJobNotificationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobNotificationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `event_types` after provisioning.\nEvent types for which a notification is desired. If empty, send notifications for all event types. The valid types are \"TRANSFER_OPERATION_SUCCESS\", \"TRANSFER_OPERATION_FAILED\", \"TRANSFER_OPERATION_ABORTED\"."]
    pub fn event_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.event_types", self.base))
    }
    #[doc = "Get a reference to the value of field `payload_format` after provisioning.\nThe desired format of the notification message payloads. One of \"NONE\" or \"JSON\"."]
    pub fn payload_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.payload_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pubsub_topic` after provisioning.\nThe Topic.name of the Pub/Sub topic to which to publish notifications."]
    pub fn pubsub_topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pubsub_topic", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageTransferJobReplicationSpecElGcsDataSinkEl {
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}
impl StorageTransferJobReplicationSpecElGcsDataSinkEl {
    #[doc = "Set the field `path`.\nGoogle Cloud Storage path in bucket to transfer. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should not begin with a '/'."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}
impl ToListMappable for StorageTransferJobReplicationSpecElGcsDataSinkEl {
    type O = BlockAssignable<StorageTransferJobReplicationSpecElGcsDataSinkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobReplicationSpecElGcsDataSinkEl {
    #[doc = "Google Cloud Storage bucket name."]
    pub bucket_name: PrimField<String>,
}
impl BuildStorageTransferJobReplicationSpecElGcsDataSinkEl {
    pub fn build(self) -> StorageTransferJobReplicationSpecElGcsDataSinkEl {
        StorageTransferJobReplicationSpecElGcsDataSinkEl {
            bucket_name: self.bucket_name,
            path: core::default::Default::default(),
        }
    }
}
pub struct StorageTransferJobReplicationSpecElGcsDataSinkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobReplicationSpecElGcsDataSinkElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageTransferJobReplicationSpecElGcsDataSinkElRef {
        StorageTransferJobReplicationSpecElGcsDataSinkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobReplicationSpecElGcsDataSinkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nGoogle Cloud Storage bucket name."]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nGoogle Cloud Storage path in bucket to transfer. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should not begin with a '/'."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageTransferJobReplicationSpecElGcsDataSourceEl {
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}
impl StorageTransferJobReplicationSpecElGcsDataSourceEl {
    #[doc = "Set the field `path`.\nGoogle Cloud Storage path in bucket to transfer. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should not begin with a '/'."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}
impl ToListMappable for StorageTransferJobReplicationSpecElGcsDataSourceEl {
    type O = BlockAssignable<StorageTransferJobReplicationSpecElGcsDataSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobReplicationSpecElGcsDataSourceEl {
    #[doc = "Google Cloud Storage bucket name."]
    pub bucket_name: PrimField<String>,
}
impl BuildStorageTransferJobReplicationSpecElGcsDataSourceEl {
    pub fn build(self) -> StorageTransferJobReplicationSpecElGcsDataSourceEl {
        StorageTransferJobReplicationSpecElGcsDataSourceEl {
            bucket_name: self.bucket_name,
            path: core::default::Default::default(),
        }
    }
}
pub struct StorageTransferJobReplicationSpecElGcsDataSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobReplicationSpecElGcsDataSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageTransferJobReplicationSpecElGcsDataSourceElRef {
        StorageTransferJobReplicationSpecElGcsDataSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobReplicationSpecElGcsDataSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nGoogle Cloud Storage bucket name."]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nGoogle Cloud Storage path in bucket to transfer. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should not begin with a '/'."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageTransferJobReplicationSpecElObjectConditionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_prefixes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_prefixes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified_before: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified_since: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_time_elapsed_since_last_modification: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_time_elapsed_since_last_modification: Option<PrimField<String>>,
}
impl StorageTransferJobReplicationSpecElObjectConditionsEl {
    #[doc = "Set the field `exclude_prefixes`.\nexclude_prefixes must follow the requirements described for include_prefixes."]
    pub fn set_exclude_prefixes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclude_prefixes = Some(v.into());
        self
    }
    #[doc = "Set the field `include_prefixes`.\nIf include_refixes is specified, objects that satisfy the object conditions must have names that start with one of the include_prefixes and that do not start with any of the exclude_prefixes. If include_prefixes is not specified, all objects except those that have names starting with one of the exclude_prefixes must satisfy the object conditions."]
    pub fn set_include_prefixes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.include_prefixes = Some(v.into());
        self
    }
    #[doc = "Set the field `last_modified_before`.\nIf specified, only objects with a \"last modification time\" before this timestamp and objects that don't have a \"last modification time\" are transferred. A timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_last_modified_before(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_modified_before = Some(v.into());
        self
    }
    #[doc = "Set the field `last_modified_since`.\nIf specified, only objects with a \"last modification time\" on or after this timestamp and objects that don't have a \"last modification time\" are transferred. A timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_last_modified_since(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_modified_since = Some(v.into());
        self
    }
    #[doc = "Set the field `max_time_elapsed_since_last_modification`.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_max_time_elapsed_since_last_modification(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.max_time_elapsed_since_last_modification = Some(v.into());
        self
    }
    #[doc = "Set the field `min_time_elapsed_since_last_modification`.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_min_time_elapsed_since_last_modification(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.min_time_elapsed_since_last_modification = Some(v.into());
        self
    }
}
impl ToListMappable for StorageTransferJobReplicationSpecElObjectConditionsEl {
    type O = BlockAssignable<StorageTransferJobReplicationSpecElObjectConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobReplicationSpecElObjectConditionsEl {}
impl BuildStorageTransferJobReplicationSpecElObjectConditionsEl {
    pub fn build(self) -> StorageTransferJobReplicationSpecElObjectConditionsEl {
        StorageTransferJobReplicationSpecElObjectConditionsEl {
            exclude_prefixes: core::default::Default::default(),
            include_prefixes: core::default::Default::default(),
            last_modified_before: core::default::Default::default(),
            last_modified_since: core::default::Default::default(),
            max_time_elapsed_since_last_modification: core::default::Default::default(),
            min_time_elapsed_since_last_modification: core::default::Default::default(),
        }
    }
}
pub struct StorageTransferJobReplicationSpecElObjectConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobReplicationSpecElObjectConditionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageTransferJobReplicationSpecElObjectConditionsElRef {
        StorageTransferJobReplicationSpecElObjectConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobReplicationSpecElObjectConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exclude_prefixes` after provisioning.\nexclude_prefixes must follow the requirements described for include_prefixes."]
    pub fn exclude_prefixes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_prefixes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_prefixes` after provisioning.\nIf include_refixes is specified, objects that satisfy the object conditions must have names that start with one of the include_prefixes and that do not start with any of the exclude_prefixes. If include_prefixes is not specified, all objects except those that have names starting with one of the exclude_prefixes must satisfy the object conditions."]
    pub fn include_prefixes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_prefixes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `last_modified_before` after provisioning.\nIf specified, only objects with a \"last modification time\" before this timestamp and objects that don't have a \"last modification time\" are transferred. A timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn last_modified_before(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_before", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `last_modified_since` after provisioning.\nIf specified, only objects with a \"last modification time\" on or after this timestamp and objects that don't have a \"last modification time\" are transferred. A timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn last_modified_since(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_since", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_time_elapsed_since_last_modification` after provisioning.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn max_time_elapsed_since_last_modification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_time_elapsed_since_last_modification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_time_elapsed_since_last_modification` after provisioning.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn min_time_elapsed_since_last_modification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_time_elapsed_since_last_modification", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageTransferJobReplicationSpecElTransferOptionsElMetadataOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    acl: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_class: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    symlink: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temporary_hold: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_created: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<PrimField<String>>,
}
impl StorageTransferJobReplicationSpecElTransferOptionsElMetadataOptionsEl {
    #[doc = "Set the field `acl`.\nSpecifies how each object's ACLs should be preserved for transfers between Google Cloud Storage buckets"]
    pub fn set_acl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.acl = Some(v.into());
        self
    }
    #[doc = "Set the field `gid`.\nSpecifies how each file's POSIX group ID (GID) attribute should be handled by the transfer."]
    pub fn set_gid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gid = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key`.\nSpecifies how each object's Cloud KMS customer-managed encryption key (CMEK) is preserved for transfers between Google Cloud Storage buckets"]
    pub fn set_kms_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\nSpecifies how each file's mode attribute should be handled by the transfer."]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_class`.\nSpecifies the storage class to set on objects being transferred to Google Cloud Storage buckets"]
    pub fn set_storage_class(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_class = Some(v.into());
        self
    }
    #[doc = "Set the field `symlink`.\nSpecifies how symlinks should be handled by the transfer."]
    pub fn set_symlink(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.symlink = Some(v.into());
        self
    }
    #[doc = "Set the field `temporary_hold`.\nSSpecifies how each object's temporary hold status should be preserved for transfers between Google Cloud Storage buckets"]
    pub fn set_temporary_hold(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.temporary_hold = Some(v.into());
        self
    }
    #[doc = "Set the field `time_created`.\nSpecifies how each object's timeCreated metadata is preserved for transfers."]
    pub fn set_time_created(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time_created = Some(v.into());
        self
    }
    #[doc = "Set the field `uid`.\nSpecifies how each file's POSIX user ID (UID) attribute should be handled by the transfer."]
    pub fn set_uid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uid = Some(v.into());
        self
    }
}
impl ToListMappable for StorageTransferJobReplicationSpecElTransferOptionsElMetadataOptionsEl {
    type O = BlockAssignable<StorageTransferJobReplicationSpecElTransferOptionsElMetadataOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobReplicationSpecElTransferOptionsElMetadataOptionsEl {}
impl BuildStorageTransferJobReplicationSpecElTransferOptionsElMetadataOptionsEl {
    pub fn build(self) -> StorageTransferJobReplicationSpecElTransferOptionsElMetadataOptionsEl {
        StorageTransferJobReplicationSpecElTransferOptionsElMetadataOptionsEl {
            acl: core::default::Default::default(),
            gid: core::default::Default::default(),
            kms_key: core::default::Default::default(),
            mode: core::default::Default::default(),
            storage_class: core::default::Default::default(),
            symlink: core::default::Default::default(),
            temporary_hold: core::default::Default::default(),
            time_created: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}
pub struct StorageTransferJobReplicationSpecElTransferOptionsElMetadataOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobReplicationSpecElTransferOptionsElMetadataOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageTransferJobReplicationSpecElTransferOptionsElMetadataOptionsElRef {
        StorageTransferJobReplicationSpecElTransferOptionsElMetadataOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobReplicationSpecElTransferOptionsElMetadataOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `acl` after provisioning.\nSpecifies how each object's ACLs should be preserved for transfers between Google Cloud Storage buckets"]
    pub fn acl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.acl", self.base))
    }
    #[doc = "Get a reference to the value of field `gid` after provisioning.\nSpecifies how each file's POSIX group ID (GID) attribute should be handled by the transfer."]
    pub fn gid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gid", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nSpecifies how each object's Cloud KMS customer-managed encryption key (CMEK) is preserved for transfers between Google Cloud Storage buckets"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nSpecifies how each file's mode attribute should be handled by the transfer."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\nSpecifies the storage class to set on objects being transferred to Google Cloud Storage buckets"]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_class", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `symlink` after provisioning.\nSpecifies how symlinks should be handled by the transfer."]
    pub fn symlink(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.symlink", self.base))
    }
    #[doc = "Get a reference to the value of field `temporary_hold` after provisioning.\nSSpecifies how each object's temporary hold status should be preserved for transfers between Google Cloud Storage buckets"]
    pub fn temporary_hold(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.temporary_hold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `time_created` after provisioning.\nSpecifies how each object's timeCreated metadata is preserved for transfers."]
    pub fn time_created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_created", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSpecifies how each file's POSIX user ID (UID) attribute should be handled by the transfer."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize, Default)]
struct StorageTransferJobReplicationSpecElTransferOptionsElDynamic {
    metadata_options:
        Option<DynamicBlock<StorageTransferJobReplicationSpecElTransferOptionsElMetadataOptionsEl>>,
}
#[derive(Serialize)]
pub struct StorageTransferJobReplicationSpecElTransferOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_objects_from_source_after_transfer: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_objects_unique_in_sink: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overwrite_objects_already_existing_in_sink: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overwrite_when: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_options:
        Option<Vec<StorageTransferJobReplicationSpecElTransferOptionsElMetadataOptionsEl>>,
    dynamic: StorageTransferJobReplicationSpecElTransferOptionsElDynamic,
}
impl StorageTransferJobReplicationSpecElTransferOptionsEl {
    #[doc = "Set the field `delete_objects_from_source_after_transfer`.\nWhether objects should be deleted from the source after they are transferred to the sink. Note that this option and delete_objects_unique_in_sink are mutually exclusive."]
    pub fn set_delete_objects_from_source_after_transfer(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.delete_objects_from_source_after_transfer = Some(v.into());
        self
    }
    #[doc = "Set the field `delete_objects_unique_in_sink`.\nWhether objects that exist only in the sink should be deleted. Note that this option and delete_objects_from_source_after_transfer are mutually exclusive."]
    pub fn set_delete_objects_unique_in_sink(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.delete_objects_unique_in_sink = Some(v.into());
        self
    }
    #[doc = "Set the field `overwrite_objects_already_existing_in_sink`.\nWhether overwriting objects that already exist in the sink is allowed."]
    pub fn set_overwrite_objects_already_existing_in_sink(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.overwrite_objects_already_existing_in_sink = Some(v.into());
        self
    }
    #[doc = "Set the field `overwrite_when`.\nWhen to overwrite objects that already exist in the sink. If not set, overwrite behavior is determined by overwriteObjectsAlreadyExistingInSink."]
    pub fn set_overwrite_when(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.overwrite_when = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata_options`.\n"]
    pub fn set_metadata_options(
        mut self,
        v: impl Into<
            BlockAssignable<StorageTransferJobReplicationSpecElTransferOptionsElMetadataOptionsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metadata_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metadata_options = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for StorageTransferJobReplicationSpecElTransferOptionsEl {
    type O = BlockAssignable<StorageTransferJobReplicationSpecElTransferOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobReplicationSpecElTransferOptionsEl {}
impl BuildStorageTransferJobReplicationSpecElTransferOptionsEl {
    pub fn build(self) -> StorageTransferJobReplicationSpecElTransferOptionsEl {
        StorageTransferJobReplicationSpecElTransferOptionsEl {
            delete_objects_from_source_after_transfer: core::default::Default::default(),
            delete_objects_unique_in_sink: core::default::Default::default(),
            overwrite_objects_already_existing_in_sink: core::default::Default::default(),
            overwrite_when: core::default::Default::default(),
            metadata_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct StorageTransferJobReplicationSpecElTransferOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobReplicationSpecElTransferOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageTransferJobReplicationSpecElTransferOptionsElRef {
        StorageTransferJobReplicationSpecElTransferOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobReplicationSpecElTransferOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delete_objects_from_source_after_transfer` after provisioning.\nWhether objects should be deleted from the source after they are transferred to the sink. Note that this option and delete_objects_unique_in_sink are mutually exclusive."]
    pub fn delete_objects_from_source_after_transfer(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_objects_from_source_after_transfer", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `delete_objects_unique_in_sink` after provisioning.\nWhether objects that exist only in the sink should be deleted. Note that this option and delete_objects_from_source_after_transfer are mutually exclusive."]
    pub fn delete_objects_unique_in_sink(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_objects_unique_in_sink", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `overwrite_objects_already_existing_in_sink` after provisioning.\nWhether overwriting objects that already exist in the sink is allowed."]
    pub fn overwrite_objects_already_existing_in_sink(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.overwrite_objects_already_existing_in_sink", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `overwrite_when` after provisioning.\nWhen to overwrite objects that already exist in the sink. If not set, overwrite behavior is determined by overwriteObjectsAlreadyExistingInSink."]
    pub fn overwrite_when(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.overwrite_when", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metadata_options` after provisioning.\n"]
    pub fn metadata_options(
        &self,
    ) -> ListRef<StorageTransferJobReplicationSpecElTransferOptionsElMetadataOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata_options", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct StorageTransferJobReplicationSpecElDynamic {
    gcs_data_sink: Option<DynamicBlock<StorageTransferJobReplicationSpecElGcsDataSinkEl>>,
    gcs_data_source: Option<DynamicBlock<StorageTransferJobReplicationSpecElGcsDataSourceEl>>,
    object_conditions: Option<DynamicBlock<StorageTransferJobReplicationSpecElObjectConditionsEl>>,
    transfer_options: Option<DynamicBlock<StorageTransferJobReplicationSpecElTransferOptionsEl>>,
}
#[derive(Serialize)]
pub struct StorageTransferJobReplicationSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_data_sink: Option<Vec<StorageTransferJobReplicationSpecElGcsDataSinkEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_data_source: Option<Vec<StorageTransferJobReplicationSpecElGcsDataSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_conditions: Option<Vec<StorageTransferJobReplicationSpecElObjectConditionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_options: Option<Vec<StorageTransferJobReplicationSpecElTransferOptionsEl>>,
    dynamic: StorageTransferJobReplicationSpecElDynamic,
}
impl StorageTransferJobReplicationSpecEl {
    #[doc = "Set the field `gcs_data_sink`.\n"]
    pub fn set_gcs_data_sink(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobReplicationSpecElGcsDataSinkEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gcs_data_sink = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs_data_sink = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gcs_data_source`.\n"]
    pub fn set_gcs_data_source(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobReplicationSpecElGcsDataSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gcs_data_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs_data_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `object_conditions`.\n"]
    pub fn set_object_conditions(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobReplicationSpecElObjectConditionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.object_conditions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.object_conditions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `transfer_options`.\n"]
    pub fn set_transfer_options(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobReplicationSpecElTransferOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.transfer_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.transfer_options = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for StorageTransferJobReplicationSpecEl {
    type O = BlockAssignable<StorageTransferJobReplicationSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobReplicationSpecEl {}
impl BuildStorageTransferJobReplicationSpecEl {
    pub fn build(self) -> StorageTransferJobReplicationSpecEl {
        StorageTransferJobReplicationSpecEl {
            gcs_data_sink: core::default::Default::default(),
            gcs_data_source: core::default::Default::default(),
            object_conditions: core::default::Default::default(),
            transfer_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct StorageTransferJobReplicationSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobReplicationSpecElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobReplicationSpecElRef {
        StorageTransferJobReplicationSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobReplicationSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gcs_data_sink` after provisioning.\n"]
    pub fn gcs_data_sink(&self) -> ListRef<StorageTransferJobReplicationSpecElGcsDataSinkElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcs_data_sink", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_data_source` after provisioning.\n"]
    pub fn gcs_data_source(
        &self,
    ) -> ListRef<StorageTransferJobReplicationSpecElGcsDataSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcs_data_source", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `object_conditions` after provisioning.\n"]
    pub fn object_conditions(
        &self,
    ) -> ListRef<StorageTransferJobReplicationSpecElObjectConditionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.object_conditions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `transfer_options` after provisioning.\n"]
    pub fn transfer_options(
        &self,
    ) -> ListRef<StorageTransferJobReplicationSpecElTransferOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.transfer_options", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageTransferJobScheduleElScheduleEndDateEl {
    day: PrimField<f64>,
    month: PrimField<f64>,
    year: PrimField<f64>,
}
impl StorageTransferJobScheduleElScheduleEndDateEl {}
impl ToListMappable for StorageTransferJobScheduleElScheduleEndDateEl {
    type O = BlockAssignable<StorageTransferJobScheduleElScheduleEndDateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobScheduleElScheduleEndDateEl {
    #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
    pub day: PrimField<f64>,
    #[doc = "Month of year. Must be from 1 to 12."]
    pub month: PrimField<f64>,
    #[doc = "Year of date. Must be from 1 to 9999."]
    pub year: PrimField<f64>,
}
impl BuildStorageTransferJobScheduleElScheduleEndDateEl {
    pub fn build(self) -> StorageTransferJobScheduleElScheduleEndDateEl {
        StorageTransferJobScheduleElScheduleEndDateEl {
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
}
pub struct StorageTransferJobScheduleElScheduleEndDateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobScheduleElScheduleEndDateElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobScheduleElScheduleEndDateElRef {
        StorageTransferJobScheduleElScheduleEndDateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobScheduleElScheduleEndDateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\nDay of month. Must be from 1 to 31 and valid for the year and month."]
    pub fn day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
    #[doc = "Get a reference to the value of field `month` after provisioning.\nMonth of year. Must be from 1 to 12."]
    pub fn month(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }
    #[doc = "Get a reference to the value of field `year` after provisioning.\nYear of date. Must be from 1 to 9999."]
    pub fn year(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageTransferJobScheduleElScheduleStartDateEl {
    day: PrimField<f64>,
    month: PrimField<f64>,
    year: PrimField<f64>,
}
impl StorageTransferJobScheduleElScheduleStartDateEl {}
impl ToListMappable for StorageTransferJobScheduleElScheduleStartDateEl {
    type O = BlockAssignable<StorageTransferJobScheduleElScheduleStartDateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobScheduleElScheduleStartDateEl {
    #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
    pub day: PrimField<f64>,
    #[doc = "Month of year. Must be from 1 to 12."]
    pub month: PrimField<f64>,
    #[doc = "Year of date. Must be from 1 to 9999."]
    pub year: PrimField<f64>,
}
impl BuildStorageTransferJobScheduleElScheduleStartDateEl {
    pub fn build(self) -> StorageTransferJobScheduleElScheduleStartDateEl {
        StorageTransferJobScheduleElScheduleStartDateEl {
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
}
pub struct StorageTransferJobScheduleElScheduleStartDateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobScheduleElScheduleStartDateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageTransferJobScheduleElScheduleStartDateElRef {
        StorageTransferJobScheduleElScheduleStartDateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobScheduleElScheduleStartDateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\nDay of month. Must be from 1 to 31 and valid for the year and month."]
    pub fn day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
    #[doc = "Get a reference to the value of field `month` after provisioning.\nMonth of year. Must be from 1 to 12."]
    pub fn month(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }
    #[doc = "Get a reference to the value of field `year` after provisioning.\nYear of date. Must be from 1 to 9999."]
    pub fn year(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageTransferJobScheduleElStartTimeOfDayEl {
    hours: PrimField<f64>,
    minutes: PrimField<f64>,
    nanos: PrimField<f64>,
    seconds: PrimField<f64>,
}
impl StorageTransferJobScheduleElStartTimeOfDayEl {}
impl ToListMappable for StorageTransferJobScheduleElStartTimeOfDayEl {
    type O = BlockAssignable<StorageTransferJobScheduleElStartTimeOfDayEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobScheduleElStartTimeOfDayEl {
    #[doc = "Hours of day in 24 hour format. Should be from 0 to 23."]
    pub hours: PrimField<f64>,
    #[doc = "Minutes of hour of day. Must be from 0 to 59."]
    pub minutes: PrimField<f64>,
    #[doc = "Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub nanos: PrimField<f64>,
    #[doc = "Seconds of minutes of the time. Must normally be from 0 to 59."]
    pub seconds: PrimField<f64>,
}
impl BuildStorageTransferJobScheduleElStartTimeOfDayEl {
    pub fn build(self) -> StorageTransferJobScheduleElStartTimeOfDayEl {
        StorageTransferJobScheduleElStartTimeOfDayEl {
            hours: self.hours,
            minutes: self.minutes,
            nanos: self.nanos,
            seconds: self.seconds,
        }
    }
}
pub struct StorageTransferJobScheduleElStartTimeOfDayElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobScheduleElStartTimeOfDayElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobScheduleElStartTimeOfDayElRef {
        StorageTransferJobScheduleElStartTimeOfDayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobScheduleElStartTimeOfDayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours` after provisioning.\nHours of day in 24 hour format. Should be from 0 to 23."]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
    #[doc = "Get a reference to the value of field `minutes` after provisioning.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSeconds of minutes of the time. Must normally be from 0 to 59."]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize, Default)]
struct StorageTransferJobScheduleElDynamic {
    schedule_end_date: Option<DynamicBlock<StorageTransferJobScheduleElScheduleEndDateEl>>,
    schedule_start_date: Option<DynamicBlock<StorageTransferJobScheduleElScheduleStartDateEl>>,
    start_time_of_day: Option<DynamicBlock<StorageTransferJobScheduleElStartTimeOfDayEl>>,
}
#[derive(Serialize)]
pub struct StorageTransferJobScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    repeat_interval: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_end_date: Option<Vec<StorageTransferJobScheduleElScheduleEndDateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_start_date: Option<Vec<StorageTransferJobScheduleElScheduleStartDateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time_of_day: Option<Vec<StorageTransferJobScheduleElStartTimeOfDayEl>>,
    dynamic: StorageTransferJobScheduleElDynamic,
}
impl StorageTransferJobScheduleEl {
    #[doc = "Set the field `repeat_interval`.\nInterval between the start of each scheduled transfer. If unspecified, the default value is 24 hours. This value may not be less than 1 hour. A duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_repeat_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repeat_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `schedule_end_date`.\n"]
    pub fn set_schedule_end_date(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobScheduleElScheduleEndDateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schedule_end_date = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schedule_end_date = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `schedule_start_date`.\n"]
    pub fn set_schedule_start_date(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobScheduleElScheduleStartDateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schedule_start_date = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schedule_start_date = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `start_time_of_day`.\n"]
    pub fn set_start_time_of_day(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobScheduleElStartTimeOfDayEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.start_time_of_day = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.start_time_of_day = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for StorageTransferJobScheduleEl {
    type O = BlockAssignable<StorageTransferJobScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobScheduleEl {}
impl BuildStorageTransferJobScheduleEl {
    pub fn build(self) -> StorageTransferJobScheduleEl {
        StorageTransferJobScheduleEl {
            repeat_interval: core::default::Default::default(),
            schedule_end_date: core::default::Default::default(),
            schedule_start_date: core::default::Default::default(),
            start_time_of_day: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct StorageTransferJobScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobScheduleElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobScheduleElRef {
        StorageTransferJobScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `repeat_interval` after provisioning.\nInterval between the start of each scheduled transfer. If unspecified, the default value is 24 hours. This value may not be less than 1 hour. A duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn repeat_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repeat_interval", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `schedule_end_date` after provisioning.\n"]
    pub fn schedule_end_date(&self) -> ListRef<StorageTransferJobScheduleElScheduleEndDateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schedule_end_date", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `schedule_start_date` after provisioning.\n"]
    pub fn schedule_start_date(
        &self,
    ) -> ListRef<StorageTransferJobScheduleElScheduleStartDateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schedule_start_date", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `start_time_of_day` after provisioning.\n"]
    pub fn start_time_of_day(&self) -> ListRef<StorageTransferJobScheduleElStartTimeOfDayElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.start_time_of_day", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceElS3MetadataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    list_api: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_model: Option<PrimField<String>>,
}
impl StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceElS3MetadataEl {
    #[doc = "Set the field `auth_method`.\nAuthentication and authorization method used by the storage service. When not specified, Transfer Service will attempt to determine right auth method to use."]
    pub fn set_auth_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_method = Some(v.into());
        self
    }
    #[doc = "Set the field `list_api`.\nThe Listing API to use for discovering objects. When not specified, Transfer Service will attempt to determine the right API to use."]
    pub fn set_list_api(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.list_api = Some(v.into());
        self
    }
    #[doc = "Set the field `protocol`.\nThe network protocol of the agent. When not specified, the default value of NetworkProtocol NETWORK_PROTOCOL_HTTPS is used."]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `request_model`.\nAPI request model used to call the storage service. When not specified, the default value of RequestModel REQUEST_MODEL_VIRTUAL_HOSTED_STYLE is used."]
    pub fn set_request_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.request_model = Some(v.into());
        self
    }
}
impl ToListMappable for StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceElS3MetadataEl {
    type O =
        BlockAssignable<StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceElS3MetadataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobTransferSpecElAwsS3CompatibleDataSourceElS3MetadataEl {}
impl BuildStorageTransferJobTransferSpecElAwsS3CompatibleDataSourceElS3MetadataEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceElS3MetadataEl {
        StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceElS3MetadataEl {
            auth_method: core::default::Default::default(),
            list_api: core::default::Default::default(),
            protocol: core::default::Default::default(),
            request_model: core::default::Default::default(),
        }
    }
}
pub struct StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceElS3MetadataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceElS3MetadataElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceElS3MetadataElRef {
        StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceElS3MetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceElS3MetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auth_method` after provisioning.\nAuthentication and authorization method used by the storage service. When not specified, Transfer Service will attempt to determine right auth method to use."]
    pub fn auth_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_method", self.base))
    }
    #[doc = "Get a reference to the value of field `list_api` after provisioning.\nThe Listing API to use for discovering objects. When not specified, Transfer Service will attempt to determine the right API to use."]
    pub fn list_api(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.list_api", self.base))
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\nThe network protocol of the agent. When not specified, the default value of NetworkProtocol NETWORK_PROTOCOL_HTTPS is used."]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
    #[doc = "Get a reference to the value of field `request_model` after provisioning.\nAPI request model used to call the storage service. When not specified, the default value of RequestModel REQUEST_MODEL_VIRTUAL_HOSTED_STYLE is used."]
    pub fn request_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_model", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceElDynamic {
    s3_metadata: Option<
        DynamicBlock<StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceElS3MetadataEl>,
    >,
}
#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceEl {
    bucket_name: PrimField<String>,
    endpoint: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_metadata:
        Option<Vec<StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceElS3MetadataEl>>,
    dynamic: StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceElDynamic,
}
impl StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceEl {
    #[doc = "Set the field `path`.\nSpecifies the path to transfer objects."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nSpecifies the region to sign requests with. This can be left blank if requests should be signed with an empty region."]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
    #[doc = "Set the field `s3_metadata`.\n"]
    pub fn set_s3_metadata(
        mut self,
        v: impl Into<
            BlockAssignable<
                StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceElS3MetadataEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_metadata = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_metadata = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobTransferSpecElAwsS3CompatibleDataSourceEl {
    #[doc = "Name of the bucket."]
    pub bucket_name: PrimField<String>,
    #[doc = "Endpoint of the storage service."]
    pub endpoint: PrimField<String>,
}
impl BuildStorageTransferJobTransferSpecElAwsS3CompatibleDataSourceEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceEl {
        StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceEl {
            bucket_name: self.bucket_name,
            endpoint: self.endpoint,
            path: core::default::Default::default(),
            region: core::default::Default::default(),
            s3_metadata: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceElRef {
        StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nName of the bucket."]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nEndpoint of the storage service."]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nSpecifies the path to transfer objects."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nSpecifies the region to sign requests with. This can be left blank if requests should be signed with an empty region."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
    #[doc = "Get a reference to the value of field `s3_metadata` after provisioning.\n"]
    pub fn s3_metadata(
        &self,
    ) -> ListRef<StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceElS3MetadataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_metadata", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyEl {
    access_key_id: PrimField<String>,
    secret_access_key: PrimField<String>,
}
impl StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyEl {}
impl ToListMappable for StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyEl {
    #[doc = "AWS Key ID."]
    pub access_key_id: PrimField<String>,
    #[doc = "AWS Secret Access Key."]
    pub secret_access_key: PrimField<String>,
}
impl BuildStorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyEl {
        StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyEl {
            access_key_id: self.access_key_id,
            secret_access_key: self.secret_access_key,
        }
    }
}
pub struct StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyElRef {
        StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_key_id` after provisioning.\nAWS Key ID."]
    pub fn access_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_key_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secret_access_key` after provisioning.\nAWS Secret Access Key."]
    pub fn secret_access_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_access_key", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct StorageTransferJobTransferSpecElAwsS3DataSourceElDynamic {
    aws_access_key:
        Option<DynamicBlock<StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyEl>>,
}
#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElAwsS3DataSourceEl {
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudfront_domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credentials_secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_private_network: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_access_key: Option<Vec<StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyEl>>,
    dynamic: StorageTransferJobTransferSpecElAwsS3DataSourceElDynamic,
}
impl StorageTransferJobTransferSpecElAwsS3DataSourceEl {
    #[doc = "Set the field `cloudfront_domain`.\nThe CloudFront distribution domain name pointing to this bucket, to use when fetching. See [Transfer from S3 via CloudFront](https://cloud.google.com/storage-transfer/docs/s3-cloudfront) for more information. Format: https://{id}.cloudfront.net or any valid custom domain. Must begin with https://."]
    pub fn set_cloudfront_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cloudfront_domain = Some(v.into());
        self
    }
    #[doc = "Set the field `credentials_secret`.\nThe Resource name of a secret in Secret Manager. AWS credentials must be stored in Secret Manager in JSON format. If credentials_secret is specified, do not specify role_arn or aws_access_key. Format: projects/{projectNumber}/secrets/{secret_name}."]
    pub fn set_credentials_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.credentials_secret = Some(v.into());
        self
    }
    #[doc = "Set the field `managed_private_network`.\nEgress bytes over a Google-managed private network. This network is shared between other users of Storage Transfer Service."]
    pub fn set_managed_private_network(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.managed_private_network = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\nS3 Bucket path in bucket to transfer."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `role_arn`.\nThe Amazon Resource Name (ARN) of the role to support temporary credentials via 'AssumeRoleWithWebIdentity'. For more information about ARNs, see [IAM ARNs](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns). When a role ARN is provided, Transfer Service fetches temporary credentials for the session using a 'AssumeRoleWithWebIdentity' call for the provided role using the [GoogleServiceAccount][] for this project."]
    pub fn set_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `aws_access_key`.\n"]
    pub fn set_aws_access_key(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aws_access_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_access_key = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for StorageTransferJobTransferSpecElAwsS3DataSourceEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecElAwsS3DataSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobTransferSpecElAwsS3DataSourceEl {
    #[doc = "S3 Bucket name."]
    pub bucket_name: PrimField<String>,
}
impl BuildStorageTransferJobTransferSpecElAwsS3DataSourceEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElAwsS3DataSourceEl {
        StorageTransferJobTransferSpecElAwsS3DataSourceEl {
            bucket_name: self.bucket_name,
            cloudfront_domain: core::default::Default::default(),
            credentials_secret: core::default::Default::default(),
            managed_private_network: core::default::Default::default(),
            path: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            aws_access_key: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct StorageTransferJobTransferSpecElAwsS3DataSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobTransferSpecElAwsS3DataSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageTransferJobTransferSpecElAwsS3DataSourceElRef {
        StorageTransferJobTransferSpecElAwsS3DataSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobTransferSpecElAwsS3DataSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nS3 Bucket name."]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `cloudfront_domain` after provisioning.\nThe CloudFront distribution domain name pointing to this bucket, to use when fetching. See [Transfer from S3 via CloudFront](https://cloud.google.com/storage-transfer/docs/s3-cloudfront) for more information. Format: https://{id}.cloudfront.net or any valid custom domain. Must begin with https://."]
    pub fn cloudfront_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloudfront_domain", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `credentials_secret` after provisioning.\nThe Resource name of a secret in Secret Manager. AWS credentials must be stored in Secret Manager in JSON format. If credentials_secret is specified, do not specify role_arn or aws_access_key. Format: projects/{projectNumber}/secrets/{secret_name}."]
    pub fn credentials_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credentials_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `managed_private_network` after provisioning.\nEgress bytes over a Google-managed private network. This network is shared between other users of Storage Transfer Service."]
    pub fn managed_private_network(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.managed_private_network", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nS3 Bucket path in bucket to transfer."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\nThe Amazon Resource Name (ARN) of the role to support temporary credentials via 'AssumeRoleWithWebIdentity'. For more information about ARNs, see [IAM ARNs](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns). When a role ARN is provided, Transfer Service fetches temporary credentials for the session using a 'AssumeRoleWithWebIdentity' call for the provided role using the [GoogleServiceAccount][] for this project."]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `aws_access_key` after provisioning.\n"]
    pub fn aws_access_key(
        &self,
    ) -> ListRef<StorageTransferJobTransferSpecElAwsS3DataSourceElAwsAccessKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.aws_access_key", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsEl {
    sas_token: PrimField<String>,
}
impl StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsEl {}
impl ToListMappable
    for StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsEl
{
    type O = BlockAssignable<
        StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsEl {
    #[doc = "Azure shared access signature."]
    pub sas_token: PrimField<String>,
}
impl BuildStorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsEl {
    pub fn build(
        self,
    ) -> StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsEl {
        StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsEl {
            sas_token: self.sas_token,
        }
    }
}
pub struct StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsElRef {
        StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `sas_token` after provisioning.\nAzure shared access signature."]
    pub fn sas_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sas_token", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElFederatedIdentityConfigEl {
    client_id: PrimField<String>,
    tenant_id: PrimField<String>,
}
impl StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElFederatedIdentityConfigEl {}
impl ToListMappable
    for StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElFederatedIdentityConfigEl
{
    type O = BlockAssignable<
        StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElFederatedIdentityConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobTransferSpecElAzureBlobStorageDataSourceElFederatedIdentityConfigEl
{
    #[doc = "The client (application) ID of the application with federated credentials."]
    pub client_id: PrimField<String>,
    #[doc = "The tenant (directory) ID of the application with federated credentials."]
    pub tenant_id: PrimField<String>,
}
impl BuildStorageTransferJobTransferSpecElAzureBlobStorageDataSourceElFederatedIdentityConfigEl {
    pub fn build(
        self,
    ) -> StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElFederatedIdentityConfigEl {
        StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElFederatedIdentityConfigEl {
            client_id: self.client_id,
            tenant_id: self.tenant_id,
        }
    }
}
pub struct StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElFederatedIdentityConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElFederatedIdentityConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElFederatedIdentityConfigElRef
    {
        StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElFederatedIdentityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElFederatedIdentityConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nThe client (application) ID of the application with federated credentials."]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `tenant_id` after provisioning.\nThe tenant (directory) ID of the application with federated credentials."]
    pub fn tenant_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }
}
#[derive(Serialize, Default)]
struct StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElDynamic {
    azure_credentials: Option<
        DynamicBlock<
            StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsEl,
        >,
    >,
    federated_identity_config: Option<
        DynamicBlock<
            StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElFederatedIdentityConfigEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElAzureBlobStorageDataSourceEl {
    container: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credentials_secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    storage_account: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    azure_credentials:
        Option<Vec<StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    federated_identity_config: Option<
        Vec<StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElFederatedIdentityConfigEl>,
    >,
    dynamic: StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElDynamic,
}
impl StorageTransferJobTransferSpecElAzureBlobStorageDataSourceEl {
    #[doc = "Set the field `credentials_secret`.\nThe Resource name of a secret in Secret Manager containing SAS Credentials in JSON form. Service Agent must have permissions to access secret. If credentials_secret is specified, do not specify azure_credentials."]
    pub fn set_credentials_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.credentials_secret = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\nRoot path to transfer objects. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should generally not begin with a '/'."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `azure_credentials`.\n"]
    pub fn set_azure_credentials(
        mut self,
        v: impl Into<
            BlockAssignable<
                StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.azure_credentials = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.azure_credentials = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `federated_identity_config`.\n"]
    pub fn set_federated_identity_config(
        mut self,
        v : impl Into < BlockAssignable < StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElFederatedIdentityConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.federated_identity_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.federated_identity_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for StorageTransferJobTransferSpecElAzureBlobStorageDataSourceEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecElAzureBlobStorageDataSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobTransferSpecElAzureBlobStorageDataSourceEl {
    #[doc = "The container to transfer from the Azure Storage account."]
    pub container: PrimField<String>,
    #[doc = "The name of the Azure Storage account."]
    pub storage_account: PrimField<String>,
}
impl BuildStorageTransferJobTransferSpecElAzureBlobStorageDataSourceEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElAzureBlobStorageDataSourceEl {
        StorageTransferJobTransferSpecElAzureBlobStorageDataSourceEl {
            container: self.container,
            credentials_secret: core::default::Default::default(),
            path: core::default::Default::default(),
            storage_account: self.storage_account,
            azure_credentials: core::default::Default::default(),
            federated_identity_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElRef {
        StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container` after provisioning.\nThe container to transfer from the Azure Storage account."]
    pub fn container(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container", self.base))
    }
    #[doc = "Get a reference to the value of field `credentials_secret` after provisioning.\nThe Resource name of a secret in Secret Manager containing SAS Credentials in JSON form. Service Agent must have permissions to access secret. If credentials_secret is specified, do not specify azure_credentials."]
    pub fn credentials_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credentials_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nRoot path to transfer objects. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should generally not begin with a '/'."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_account` after provisioning.\nThe name of the Azure Storage account."]
    pub fn storage_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `azure_credentials` after provisioning.\n"]
    pub fn azure_credentials(
        &self,
    ) -> ListRef<StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElAzureCredentialsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.azure_credentials", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `federated_identity_config` after provisioning.\n"]
    pub fn federated_identity_config(
        &self,
    ) -> ListRef<
        StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElFederatedIdentityConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.federated_identity_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElGcsDataSinkEl {
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}
impl StorageTransferJobTransferSpecElGcsDataSinkEl {
    #[doc = "Set the field `path`.\nGoogle Cloud Storage path in bucket to transfer. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should not begin with a '/'."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}
impl ToListMappable for StorageTransferJobTransferSpecElGcsDataSinkEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecElGcsDataSinkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobTransferSpecElGcsDataSinkEl {
    #[doc = "Google Cloud Storage bucket name."]
    pub bucket_name: PrimField<String>,
}
impl BuildStorageTransferJobTransferSpecElGcsDataSinkEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElGcsDataSinkEl {
        StorageTransferJobTransferSpecElGcsDataSinkEl {
            bucket_name: self.bucket_name,
            path: core::default::Default::default(),
        }
    }
}
pub struct StorageTransferJobTransferSpecElGcsDataSinkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobTransferSpecElGcsDataSinkElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobTransferSpecElGcsDataSinkElRef {
        StorageTransferJobTransferSpecElGcsDataSinkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobTransferSpecElGcsDataSinkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nGoogle Cloud Storage bucket name."]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nGoogle Cloud Storage path in bucket to transfer. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should not begin with a '/'."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElGcsDataSourceEl {
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}
impl StorageTransferJobTransferSpecElGcsDataSourceEl {
    #[doc = "Set the field `path`.\nGoogle Cloud Storage path in bucket to transfer. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should not begin with a '/'."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}
impl ToListMappable for StorageTransferJobTransferSpecElGcsDataSourceEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecElGcsDataSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobTransferSpecElGcsDataSourceEl {
    #[doc = "Google Cloud Storage bucket name."]
    pub bucket_name: PrimField<String>,
}
impl BuildStorageTransferJobTransferSpecElGcsDataSourceEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElGcsDataSourceEl {
        StorageTransferJobTransferSpecElGcsDataSourceEl {
            bucket_name: self.bucket_name,
            path: core::default::Default::default(),
        }
    }
}
pub struct StorageTransferJobTransferSpecElGcsDataSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobTransferSpecElGcsDataSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageTransferJobTransferSpecElGcsDataSourceElRef {
        StorageTransferJobTransferSpecElGcsDataSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobTransferSpecElGcsDataSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nGoogle Cloud Storage bucket name."]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nGoogle Cloud Storage path in bucket to transfer. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should not begin with a '/'."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElHdfsDataSourceEl {
    path: PrimField<String>,
}
impl StorageTransferJobTransferSpecElHdfsDataSourceEl {}
impl ToListMappable for StorageTransferJobTransferSpecElHdfsDataSourceEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecElHdfsDataSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobTransferSpecElHdfsDataSourceEl {
    #[doc = "Directory path to the filesystem."]
    pub path: PrimField<String>,
}
impl BuildStorageTransferJobTransferSpecElHdfsDataSourceEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElHdfsDataSourceEl {
        StorageTransferJobTransferSpecElHdfsDataSourceEl { path: self.path }
    }
}
pub struct StorageTransferJobTransferSpecElHdfsDataSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobTransferSpecElHdfsDataSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageTransferJobTransferSpecElHdfsDataSourceElRef {
        StorageTransferJobTransferSpecElHdfsDataSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobTransferSpecElHdfsDataSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nDirectory path to the filesystem."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElHttpDataSourceEl {
    list_url: PrimField<String>,
}
impl StorageTransferJobTransferSpecElHttpDataSourceEl {}
impl ToListMappable for StorageTransferJobTransferSpecElHttpDataSourceEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecElHttpDataSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobTransferSpecElHttpDataSourceEl {
    #[doc = "The URL that points to the file that stores the object list entries. This file must allow public access. Currently, only URLs with HTTP and HTTPS schemes are supported."]
    pub list_url: PrimField<String>,
}
impl BuildStorageTransferJobTransferSpecElHttpDataSourceEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElHttpDataSourceEl {
        StorageTransferJobTransferSpecElHttpDataSourceEl {
            list_url: self.list_url,
        }
    }
}
pub struct StorageTransferJobTransferSpecElHttpDataSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobTransferSpecElHttpDataSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageTransferJobTransferSpecElHttpDataSourceElRef {
        StorageTransferJobTransferSpecElHttpDataSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobTransferSpecElHttpDataSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `list_url` after provisioning.\nThe URL that points to the file that stores the object list entries. This file must allow public access. Currently, only URLs with HTTP and HTTPS schemes are supported."]
    pub fn list_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.list_url", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElObjectConditionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_prefixes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_prefixes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified_before: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified_since: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_time_elapsed_since_last_modification: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_time_elapsed_since_last_modification: Option<PrimField<String>>,
}
impl StorageTransferJobTransferSpecElObjectConditionsEl {
    #[doc = "Set the field `exclude_prefixes`.\nexclude_prefixes must follow the requirements described for include_prefixes."]
    pub fn set_exclude_prefixes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclude_prefixes = Some(v.into());
        self
    }
    #[doc = "Set the field `include_prefixes`.\nIf include_refixes is specified, objects that satisfy the object conditions must have names that start with one of the include_prefixes and that do not start with any of the exclude_prefixes. If include_prefixes is not specified, all objects except those that have names starting with one of the exclude_prefixes must satisfy the object conditions."]
    pub fn set_include_prefixes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.include_prefixes = Some(v.into());
        self
    }
    #[doc = "Set the field `last_modified_before`.\nIf specified, only objects with a \"last modification time\" before this timestamp and objects that don't have a \"last modification time\" are transferred. A timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_last_modified_before(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_modified_before = Some(v.into());
        self
    }
    #[doc = "Set the field `last_modified_since`.\nIf specified, only objects with a \"last modification time\" on or after this timestamp and objects that don't have a \"last modification time\" are transferred. A timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_last_modified_since(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_modified_since = Some(v.into());
        self
    }
    #[doc = "Set the field `max_time_elapsed_since_last_modification`.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_max_time_elapsed_since_last_modification(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.max_time_elapsed_since_last_modification = Some(v.into());
        self
    }
    #[doc = "Set the field `min_time_elapsed_since_last_modification`.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_min_time_elapsed_since_last_modification(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.min_time_elapsed_since_last_modification = Some(v.into());
        self
    }
}
impl ToListMappable for StorageTransferJobTransferSpecElObjectConditionsEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecElObjectConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobTransferSpecElObjectConditionsEl {}
impl BuildStorageTransferJobTransferSpecElObjectConditionsEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElObjectConditionsEl {
        StorageTransferJobTransferSpecElObjectConditionsEl {
            exclude_prefixes: core::default::Default::default(),
            include_prefixes: core::default::Default::default(),
            last_modified_before: core::default::Default::default(),
            last_modified_since: core::default::Default::default(),
            max_time_elapsed_since_last_modification: core::default::Default::default(),
            min_time_elapsed_since_last_modification: core::default::Default::default(),
        }
    }
}
pub struct StorageTransferJobTransferSpecElObjectConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobTransferSpecElObjectConditionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageTransferJobTransferSpecElObjectConditionsElRef {
        StorageTransferJobTransferSpecElObjectConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobTransferSpecElObjectConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exclude_prefixes` after provisioning.\nexclude_prefixes must follow the requirements described for include_prefixes."]
    pub fn exclude_prefixes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_prefixes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_prefixes` after provisioning.\nIf include_refixes is specified, objects that satisfy the object conditions must have names that start with one of the include_prefixes and that do not start with any of the exclude_prefixes. If include_prefixes is not specified, all objects except those that have names starting with one of the exclude_prefixes must satisfy the object conditions."]
    pub fn include_prefixes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_prefixes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `last_modified_before` after provisioning.\nIf specified, only objects with a \"last modification time\" before this timestamp and objects that don't have a \"last modification time\" are transferred. A timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn last_modified_before(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_before", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `last_modified_since` after provisioning.\nIf specified, only objects with a \"last modification time\" on or after this timestamp and objects that don't have a \"last modification time\" are transferred. A timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn last_modified_since(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_since", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_time_elapsed_since_last_modification` after provisioning.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn max_time_elapsed_since_last_modification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_time_elapsed_since_last_modification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_time_elapsed_since_last_modification` after provisioning.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn min_time_elapsed_since_last_modification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_time_elapsed_since_last_modification", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElPosixDataSinkEl {
    root_directory: PrimField<String>,
}
impl StorageTransferJobTransferSpecElPosixDataSinkEl {}
impl ToListMappable for StorageTransferJobTransferSpecElPosixDataSinkEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecElPosixDataSinkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobTransferSpecElPosixDataSinkEl {
    #[doc = "Root directory path to the filesystem."]
    pub root_directory: PrimField<String>,
}
impl BuildStorageTransferJobTransferSpecElPosixDataSinkEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElPosixDataSinkEl {
        StorageTransferJobTransferSpecElPosixDataSinkEl {
            root_directory: self.root_directory,
        }
    }
}
pub struct StorageTransferJobTransferSpecElPosixDataSinkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobTransferSpecElPosixDataSinkElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageTransferJobTransferSpecElPosixDataSinkElRef {
        StorageTransferJobTransferSpecElPosixDataSinkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobTransferSpecElPosixDataSinkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `root_directory` after provisioning.\nRoot directory path to the filesystem."]
    pub fn root_directory(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_directory", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElPosixDataSourceEl {
    root_directory: PrimField<String>,
}
impl StorageTransferJobTransferSpecElPosixDataSourceEl {}
impl ToListMappable for StorageTransferJobTransferSpecElPosixDataSourceEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecElPosixDataSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobTransferSpecElPosixDataSourceEl {
    #[doc = "Root directory path to the filesystem."]
    pub root_directory: PrimField<String>,
}
impl BuildStorageTransferJobTransferSpecElPosixDataSourceEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElPosixDataSourceEl {
        StorageTransferJobTransferSpecElPosixDataSourceEl {
            root_directory: self.root_directory,
        }
    }
}
pub struct StorageTransferJobTransferSpecElPosixDataSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobTransferSpecElPosixDataSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageTransferJobTransferSpecElPosixDataSourceElRef {
        StorageTransferJobTransferSpecElPosixDataSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobTransferSpecElPosixDataSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `root_directory` after provisioning.\nRoot directory path to the filesystem."]
    pub fn root_directory(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_directory", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElTransferManifestEl {
    location: PrimField<String>,
}
impl StorageTransferJobTransferSpecElTransferManifestEl {}
impl ToListMappable for StorageTransferJobTransferSpecElTransferManifestEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecElTransferManifestEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobTransferSpecElTransferManifestEl {
    #[doc = "Cloud Storage path to the manifest CSV."]
    pub location: PrimField<String>,
}
impl BuildStorageTransferJobTransferSpecElTransferManifestEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElTransferManifestEl {
        StorageTransferJobTransferSpecElTransferManifestEl {
            location: self.location,
        }
    }
}
pub struct StorageTransferJobTransferSpecElTransferManifestElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobTransferSpecElTransferManifestElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageTransferJobTransferSpecElTransferManifestElRef {
        StorageTransferJobTransferSpecElTransferManifestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobTransferSpecElTransferManifestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nCloud Storage path to the manifest CSV."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElTransferOptionsElMetadataOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    acl: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_class: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    symlink: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temporary_hold: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_created: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<PrimField<String>>,
}
impl StorageTransferJobTransferSpecElTransferOptionsElMetadataOptionsEl {
    #[doc = "Set the field `acl`.\nSpecifies how each object's ACLs should be preserved for transfers between Google Cloud Storage buckets"]
    pub fn set_acl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.acl = Some(v.into());
        self
    }
    #[doc = "Set the field `gid`.\nSpecifies how each file's POSIX group ID (GID) attribute should be handled by the transfer."]
    pub fn set_gid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gid = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key`.\nSpecifies how each object's Cloud KMS customer-managed encryption key (CMEK) is preserved for transfers between Google Cloud Storage buckets"]
    pub fn set_kms_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\nSpecifies how each file's mode attribute should be handled by the transfer."]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_class`.\nSpecifies the storage class to set on objects being transferred to Google Cloud Storage buckets"]
    pub fn set_storage_class(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_class = Some(v.into());
        self
    }
    #[doc = "Set the field `symlink`.\nSpecifies how symlinks should be handled by the transfer."]
    pub fn set_symlink(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.symlink = Some(v.into());
        self
    }
    #[doc = "Set the field `temporary_hold`.\nSSpecifies how each object's temporary hold status should be preserved for transfers between Google Cloud Storage buckets"]
    pub fn set_temporary_hold(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.temporary_hold = Some(v.into());
        self
    }
    #[doc = "Set the field `time_created`.\nSpecifies how each object's timeCreated metadata is preserved for transfers."]
    pub fn set_time_created(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time_created = Some(v.into());
        self
    }
    #[doc = "Set the field `uid`.\nSpecifies how each file's POSIX user ID (UID) attribute should be handled by the transfer."]
    pub fn set_uid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uid = Some(v.into());
        self
    }
}
impl ToListMappable for StorageTransferJobTransferSpecElTransferOptionsElMetadataOptionsEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecElTransferOptionsElMetadataOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobTransferSpecElTransferOptionsElMetadataOptionsEl {}
impl BuildStorageTransferJobTransferSpecElTransferOptionsElMetadataOptionsEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElTransferOptionsElMetadataOptionsEl {
        StorageTransferJobTransferSpecElTransferOptionsElMetadataOptionsEl {
            acl: core::default::Default::default(),
            gid: core::default::Default::default(),
            kms_key: core::default::Default::default(),
            mode: core::default::Default::default(),
            storage_class: core::default::Default::default(),
            symlink: core::default::Default::default(),
            temporary_hold: core::default::Default::default(),
            time_created: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}
pub struct StorageTransferJobTransferSpecElTransferOptionsElMetadataOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobTransferSpecElTransferOptionsElMetadataOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageTransferJobTransferSpecElTransferOptionsElMetadataOptionsElRef {
        StorageTransferJobTransferSpecElTransferOptionsElMetadataOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobTransferSpecElTransferOptionsElMetadataOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `acl` after provisioning.\nSpecifies how each object's ACLs should be preserved for transfers between Google Cloud Storage buckets"]
    pub fn acl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.acl", self.base))
    }
    #[doc = "Get a reference to the value of field `gid` after provisioning.\nSpecifies how each file's POSIX group ID (GID) attribute should be handled by the transfer."]
    pub fn gid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gid", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nSpecifies how each object's Cloud KMS customer-managed encryption key (CMEK) is preserved for transfers between Google Cloud Storage buckets"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nSpecifies how each file's mode attribute should be handled by the transfer."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\nSpecifies the storage class to set on objects being transferred to Google Cloud Storage buckets"]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_class", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `symlink` after provisioning.\nSpecifies how symlinks should be handled by the transfer."]
    pub fn symlink(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.symlink", self.base))
    }
    #[doc = "Get a reference to the value of field `temporary_hold` after provisioning.\nSSpecifies how each object's temporary hold status should be preserved for transfers between Google Cloud Storage buckets"]
    pub fn temporary_hold(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.temporary_hold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `time_created` after provisioning.\nSpecifies how each object's timeCreated metadata is preserved for transfers."]
    pub fn time_created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_created", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSpecifies how each file's POSIX user ID (UID) attribute should be handled by the transfer."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize, Default)]
struct StorageTransferJobTransferSpecElTransferOptionsElDynamic {
    metadata_options:
        Option<DynamicBlock<StorageTransferJobTransferSpecElTransferOptionsElMetadataOptionsEl>>,
}
#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecElTransferOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_objects_from_source_after_transfer: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_objects_unique_in_sink: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overwrite_objects_already_existing_in_sink: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overwrite_when: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_options:
        Option<Vec<StorageTransferJobTransferSpecElTransferOptionsElMetadataOptionsEl>>,
    dynamic: StorageTransferJobTransferSpecElTransferOptionsElDynamic,
}
impl StorageTransferJobTransferSpecElTransferOptionsEl {
    #[doc = "Set the field `delete_objects_from_source_after_transfer`.\nWhether objects should be deleted from the source after they are transferred to the sink. Note that this option and delete_objects_unique_in_sink are mutually exclusive."]
    pub fn set_delete_objects_from_source_after_transfer(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.delete_objects_from_source_after_transfer = Some(v.into());
        self
    }
    #[doc = "Set the field `delete_objects_unique_in_sink`.\nWhether objects that exist only in the sink should be deleted. Note that this option and delete_objects_from_source_after_transfer are mutually exclusive."]
    pub fn set_delete_objects_unique_in_sink(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.delete_objects_unique_in_sink = Some(v.into());
        self
    }
    #[doc = "Set the field `overwrite_objects_already_existing_in_sink`.\nWhether overwriting objects that already exist in the sink is allowed."]
    pub fn set_overwrite_objects_already_existing_in_sink(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.overwrite_objects_already_existing_in_sink = Some(v.into());
        self
    }
    #[doc = "Set the field `overwrite_when`.\nWhen to overwrite objects that already exist in the sink. If not set, overwrite behavior is determined by overwriteObjectsAlreadyExistingInSink."]
    pub fn set_overwrite_when(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.overwrite_when = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata_options`.\n"]
    pub fn set_metadata_options(
        mut self,
        v: impl Into<
            BlockAssignable<StorageTransferJobTransferSpecElTransferOptionsElMetadataOptionsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metadata_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metadata_options = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for StorageTransferJobTransferSpecElTransferOptionsEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecElTransferOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobTransferSpecElTransferOptionsEl {}
impl BuildStorageTransferJobTransferSpecElTransferOptionsEl {
    pub fn build(self) -> StorageTransferJobTransferSpecElTransferOptionsEl {
        StorageTransferJobTransferSpecElTransferOptionsEl {
            delete_objects_from_source_after_transfer: core::default::Default::default(),
            delete_objects_unique_in_sink: core::default::Default::default(),
            overwrite_objects_already_existing_in_sink: core::default::Default::default(),
            overwrite_when: core::default::Default::default(),
            metadata_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct StorageTransferJobTransferSpecElTransferOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobTransferSpecElTransferOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageTransferJobTransferSpecElTransferOptionsElRef {
        StorageTransferJobTransferSpecElTransferOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobTransferSpecElTransferOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delete_objects_from_source_after_transfer` after provisioning.\nWhether objects should be deleted from the source after they are transferred to the sink. Note that this option and delete_objects_unique_in_sink are mutually exclusive."]
    pub fn delete_objects_from_source_after_transfer(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_objects_from_source_after_transfer", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `delete_objects_unique_in_sink` after provisioning.\nWhether objects that exist only in the sink should be deleted. Note that this option and delete_objects_from_source_after_transfer are mutually exclusive."]
    pub fn delete_objects_unique_in_sink(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_objects_unique_in_sink", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `overwrite_objects_already_existing_in_sink` after provisioning.\nWhether overwriting objects that already exist in the sink is allowed."]
    pub fn overwrite_objects_already_existing_in_sink(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.overwrite_objects_already_existing_in_sink", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `overwrite_when` after provisioning.\nWhen to overwrite objects that already exist in the sink. If not set, overwrite behavior is determined by overwriteObjectsAlreadyExistingInSink."]
    pub fn overwrite_when(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.overwrite_when", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metadata_options` after provisioning.\n"]
    pub fn metadata_options(
        &self,
    ) -> ListRef<StorageTransferJobTransferSpecElTransferOptionsElMetadataOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata_options", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct StorageTransferJobTransferSpecElDynamic {
    aws_s3_compatible_data_source:
        Option<DynamicBlock<StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceEl>>,
    aws_s3_data_source: Option<DynamicBlock<StorageTransferJobTransferSpecElAwsS3DataSourceEl>>,
    azure_blob_storage_data_source:
        Option<DynamicBlock<StorageTransferJobTransferSpecElAzureBlobStorageDataSourceEl>>,
    gcs_data_sink: Option<DynamicBlock<StorageTransferJobTransferSpecElGcsDataSinkEl>>,
    gcs_data_source: Option<DynamicBlock<StorageTransferJobTransferSpecElGcsDataSourceEl>>,
    hdfs_data_source: Option<DynamicBlock<StorageTransferJobTransferSpecElHdfsDataSourceEl>>,
    http_data_source: Option<DynamicBlock<StorageTransferJobTransferSpecElHttpDataSourceEl>>,
    object_conditions: Option<DynamicBlock<StorageTransferJobTransferSpecElObjectConditionsEl>>,
    posix_data_sink: Option<DynamicBlock<StorageTransferJobTransferSpecElPosixDataSinkEl>>,
    posix_data_source: Option<DynamicBlock<StorageTransferJobTransferSpecElPosixDataSourceEl>>,
    transfer_manifest: Option<DynamicBlock<StorageTransferJobTransferSpecElTransferManifestEl>>,
    transfer_options: Option<DynamicBlock<StorageTransferJobTransferSpecElTransferOptionsEl>>,
}
#[derive(Serialize)]
pub struct StorageTransferJobTransferSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    sink_agent_pool_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_agent_pool_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_s3_compatible_data_source:
        Option<Vec<StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_s3_data_source: Option<Vec<StorageTransferJobTransferSpecElAwsS3DataSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    azure_blob_storage_data_source:
        Option<Vec<StorageTransferJobTransferSpecElAzureBlobStorageDataSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_data_sink: Option<Vec<StorageTransferJobTransferSpecElGcsDataSinkEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_data_source: Option<Vec<StorageTransferJobTransferSpecElGcsDataSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hdfs_data_source: Option<Vec<StorageTransferJobTransferSpecElHdfsDataSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_data_source: Option<Vec<StorageTransferJobTransferSpecElHttpDataSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_conditions: Option<Vec<StorageTransferJobTransferSpecElObjectConditionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    posix_data_sink: Option<Vec<StorageTransferJobTransferSpecElPosixDataSinkEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    posix_data_source: Option<Vec<StorageTransferJobTransferSpecElPosixDataSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_manifest: Option<Vec<StorageTransferJobTransferSpecElTransferManifestEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_options: Option<Vec<StorageTransferJobTransferSpecElTransferOptionsEl>>,
    dynamic: StorageTransferJobTransferSpecElDynamic,
}
impl StorageTransferJobTransferSpecEl {
    #[doc = "Set the field `sink_agent_pool_name`.\nSpecifies the agent pool name associated with the posix data source. When unspecified, the default name is used."]
    pub fn set_sink_agent_pool_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sink_agent_pool_name = Some(v.into());
        self
    }
    #[doc = "Set the field `source_agent_pool_name`.\nSpecifies the agent pool name associated with the posix data source. When unspecified, the default name is used."]
    pub fn set_source_agent_pool_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_agent_pool_name = Some(v.into());
        self
    }
    #[doc = "Set the field `aws_s3_compatible_data_source`.\n"]
    pub fn set_aws_s3_compatible_data_source(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aws_s3_compatible_data_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_s3_compatible_data_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `aws_s3_data_source`.\n"]
    pub fn set_aws_s3_data_source(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobTransferSpecElAwsS3DataSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aws_s3_data_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_s3_data_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `azure_blob_storage_data_source`.\n"]
    pub fn set_azure_blob_storage_data_source(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobTransferSpecElAzureBlobStorageDataSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.azure_blob_storage_data_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.azure_blob_storage_data_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gcs_data_sink`.\n"]
    pub fn set_gcs_data_sink(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobTransferSpecElGcsDataSinkEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gcs_data_sink = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs_data_sink = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gcs_data_source`.\n"]
    pub fn set_gcs_data_source(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobTransferSpecElGcsDataSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gcs_data_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs_data_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `hdfs_data_source`.\n"]
    pub fn set_hdfs_data_source(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobTransferSpecElHdfsDataSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hdfs_data_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hdfs_data_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `http_data_source`.\n"]
    pub fn set_http_data_source(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobTransferSpecElHttpDataSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_data_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_data_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `object_conditions`.\n"]
    pub fn set_object_conditions(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobTransferSpecElObjectConditionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.object_conditions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.object_conditions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `posix_data_sink`.\n"]
    pub fn set_posix_data_sink(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobTransferSpecElPosixDataSinkEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.posix_data_sink = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.posix_data_sink = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `posix_data_source`.\n"]
    pub fn set_posix_data_source(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobTransferSpecElPosixDataSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.posix_data_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.posix_data_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `transfer_manifest`.\n"]
    pub fn set_transfer_manifest(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobTransferSpecElTransferManifestEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.transfer_manifest = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.transfer_manifest = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `transfer_options`.\n"]
    pub fn set_transfer_options(
        mut self,
        v: impl Into<BlockAssignable<StorageTransferJobTransferSpecElTransferOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.transfer_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.transfer_options = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for StorageTransferJobTransferSpecEl {
    type O = BlockAssignable<StorageTransferJobTransferSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferJobTransferSpecEl {}
impl BuildStorageTransferJobTransferSpecEl {
    pub fn build(self) -> StorageTransferJobTransferSpecEl {
        StorageTransferJobTransferSpecEl {
            sink_agent_pool_name: core::default::Default::default(),
            source_agent_pool_name: core::default::Default::default(),
            aws_s3_compatible_data_source: core::default::Default::default(),
            aws_s3_data_source: core::default::Default::default(),
            azure_blob_storage_data_source: core::default::Default::default(),
            gcs_data_sink: core::default::Default::default(),
            gcs_data_source: core::default::Default::default(),
            hdfs_data_source: core::default::Default::default(),
            http_data_source: core::default::Default::default(),
            object_conditions: core::default::Default::default(),
            posix_data_sink: core::default::Default::default(),
            posix_data_source: core::default::Default::default(),
            transfer_manifest: core::default::Default::default(),
            transfer_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct StorageTransferJobTransferSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferJobTransferSpecElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferJobTransferSpecElRef {
        StorageTransferJobTransferSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferJobTransferSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `sink_agent_pool_name` after provisioning.\nSpecifies the agent pool name associated with the posix data source. When unspecified, the default name is used."]
    pub fn sink_agent_pool_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sink_agent_pool_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_agent_pool_name` after provisioning.\nSpecifies the agent pool name associated with the posix data source. When unspecified, the default name is used."]
    pub fn source_agent_pool_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_agent_pool_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `aws_s3_compatible_data_source` after provisioning.\n"]
    pub fn aws_s3_compatible_data_source(
        &self,
    ) -> ListRef<StorageTransferJobTransferSpecElAwsS3CompatibleDataSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.aws_s3_compatible_data_source", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `aws_s3_data_source` after provisioning.\n"]
    pub fn aws_s3_data_source(
        &self,
    ) -> ListRef<StorageTransferJobTransferSpecElAwsS3DataSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.aws_s3_data_source", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `azure_blob_storage_data_source` after provisioning.\n"]
    pub fn azure_blob_storage_data_source(
        &self,
    ) -> ListRef<StorageTransferJobTransferSpecElAzureBlobStorageDataSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.azure_blob_storage_data_source", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_data_sink` after provisioning.\n"]
    pub fn gcs_data_sink(&self) -> ListRef<StorageTransferJobTransferSpecElGcsDataSinkElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcs_data_sink", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_data_source` after provisioning.\n"]
    pub fn gcs_data_source(&self) -> ListRef<StorageTransferJobTransferSpecElGcsDataSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcs_data_source", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hdfs_data_source` after provisioning.\n"]
    pub fn hdfs_data_source(&self) -> ListRef<StorageTransferJobTransferSpecElHdfsDataSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hdfs_data_source", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `http_data_source` after provisioning.\n"]
    pub fn http_data_source(&self) -> ListRef<StorageTransferJobTransferSpecElHttpDataSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http_data_source", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `object_conditions` after provisioning.\n"]
    pub fn object_conditions(
        &self,
    ) -> ListRef<StorageTransferJobTransferSpecElObjectConditionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.object_conditions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `posix_data_sink` after provisioning.\n"]
    pub fn posix_data_sink(&self) -> ListRef<StorageTransferJobTransferSpecElPosixDataSinkElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.posix_data_sink", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `posix_data_source` after provisioning.\n"]
    pub fn posix_data_source(
        &self,
    ) -> ListRef<StorageTransferJobTransferSpecElPosixDataSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.posix_data_source", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `transfer_manifest` after provisioning.\n"]
    pub fn transfer_manifest(
        &self,
    ) -> ListRef<StorageTransferJobTransferSpecElTransferManifestElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.transfer_manifest", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `transfer_options` after provisioning.\n"]
    pub fn transfer_options(
        &self,
    ) -> ListRef<StorageTransferJobTransferSpecElTransferOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.transfer_options", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct StorageTransferJobDynamic {
    event_stream: Option<DynamicBlock<StorageTransferJobEventStreamEl>>,
    logging_config: Option<DynamicBlock<StorageTransferJobLoggingConfigEl>>,
    notification_config: Option<DynamicBlock<StorageTransferJobNotificationConfigEl>>,
    replication_spec: Option<DynamicBlock<StorageTransferJobReplicationSpecEl>>,
    schedule: Option<DynamicBlock<StorageTransferJobScheduleEl>>,
    transfer_spec: Option<DynamicBlock<StorageTransferJobTransferSpecEl>>,
}
