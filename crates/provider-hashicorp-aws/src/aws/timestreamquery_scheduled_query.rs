use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct TimestreamqueryScheduledQueryData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    execution_role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    name: PrimField<String>,
    query_string: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_report_configuration:
        Option<Vec<TimestreamqueryScheduledQueryErrorReportConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_run_summary: Option<Vec<TimestreamqueryScheduledQueryLastRunSummaryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_configuration:
        Option<Vec<TimestreamqueryScheduledQueryNotificationConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recently_failed_runs: Option<Vec<TimestreamqueryScheduledQueryRecentlyFailedRunsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_configuration: Option<Vec<TimestreamqueryScheduledQueryScheduleConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_configuration: Option<Vec<TimestreamqueryScheduledQueryTargetConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<TimestreamqueryScheduledQueryTimeoutsEl>,
    dynamic: TimestreamqueryScheduledQueryDynamic,
}

struct TimestreamqueryScheduledQuery_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TimestreamqueryScheduledQueryData>,
}

#[derive(Clone)]
pub struct TimestreamqueryScheduledQuery(Rc<TimestreamqueryScheduledQuery_>);

impl TimestreamqueryScheduledQuery {
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

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = Some(v.into());
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

    #[doc = "Set the field `error_report_configuration`.\n"]
    pub fn set_error_report_configuration(
        self,
        v: impl Into<BlockAssignable<TimestreamqueryScheduledQueryErrorReportConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().error_report_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.error_report_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `last_run_summary`.\n"]
    pub fn set_last_run_summary(
        self,
        v: impl Into<BlockAssignable<TimestreamqueryScheduledQueryLastRunSummaryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().last_run_summary = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.last_run_summary = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `notification_configuration`.\n"]
    pub fn set_notification_configuration(
        self,
        v: impl Into<BlockAssignable<TimestreamqueryScheduledQueryNotificationConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `recently_failed_runs`.\n"]
    pub fn set_recently_failed_runs(
        self,
        v: impl Into<BlockAssignable<TimestreamqueryScheduledQueryRecentlyFailedRunsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().recently_failed_runs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.recently_failed_runs = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `schedule_configuration`.\n"]
    pub fn set_schedule_configuration(
        self,
        v: impl Into<BlockAssignable<TimestreamqueryScheduledQueryScheduleConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schedule_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schedule_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `target_configuration`.\n"]
    pub fn set_target_configuration(
        self,
        v: impl Into<BlockAssignable<TimestreamqueryScheduledQueryTargetConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<TimestreamqueryScheduledQueryTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `next_invocation_time` after provisioning.\n"]
    pub fn next_invocation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_invocation_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `previous_invocation_time` after provisioning.\n"]
    pub fn previous_invocation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.previous_invocation_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_string", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `error_report_configuration` after provisioning.\n"]
    pub fn error_report_configuration(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryErrorReportConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.error_report_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_run_summary` after provisioning.\n"]
    pub fn last_run_summary(&self) -> ListRef<TimestreamqueryScheduledQueryLastRunSummaryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.last_run_summary", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `notification_configuration` after provisioning.\n"]
    pub fn notification_configuration(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryNotificationConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `recently_failed_runs` after provisioning.\n"]
    pub fn recently_failed_runs(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryRecentlyFailedRunsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.recently_failed_runs", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `schedule_configuration` after provisioning.\n"]
    pub fn schedule_configuration(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryScheduleConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schedule_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `target_configuration` after provisioning.\n"]
    pub fn target_configuration(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryTargetConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> TimestreamqueryScheduledQueryTimeoutsElRef {
        TimestreamqueryScheduledQueryTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for TimestreamqueryScheduledQuery {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for TimestreamqueryScheduledQuery {}

impl ToListMappable for TimestreamqueryScheduledQuery {
    type O = ListRef<TimestreamqueryScheduledQueryRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for TimestreamqueryScheduledQuery_ {
    fn extract_resource_type(&self) -> String {
        "aws_timestreamquery_scheduled_query".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTimestreamqueryScheduledQuery {
    pub tf_id: String,
    #[doc = ""]
    pub execution_role_arn: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub query_string: PrimField<String>,
}

impl BuildTimestreamqueryScheduledQuery {
    pub fn build(self, stack: &mut Stack) -> TimestreamqueryScheduledQuery {
        let out = TimestreamqueryScheduledQuery(Rc::new(TimestreamqueryScheduledQuery_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TimestreamqueryScheduledQueryData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                execution_role_arn: self.execution_role_arn,
                kms_key_id: core::default::Default::default(),
                name: self.name,
                query_string: self.query_string,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                error_report_configuration: core::default::Default::default(),
                last_run_summary: core::default::Default::default(),
                notification_configuration: core::default::Default::default(),
                recently_failed_runs: core::default::Default::default(),
                schedule_configuration: core::default::Default::default(),
                target_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TimestreamqueryScheduledQueryRef {
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl TimestreamqueryScheduledQueryRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `next_invocation_time` after provisioning.\n"]
    pub fn next_invocation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_invocation_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `previous_invocation_time` after provisioning.\n"]
    pub fn previous_invocation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.previous_invocation_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_string", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `error_report_configuration` after provisioning.\n"]
    pub fn error_report_configuration(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryErrorReportConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.error_report_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_run_summary` after provisioning.\n"]
    pub fn last_run_summary(&self) -> ListRef<TimestreamqueryScheduledQueryLastRunSummaryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.last_run_summary", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `notification_configuration` after provisioning.\n"]
    pub fn notification_configuration(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryNotificationConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `recently_failed_runs` after provisioning.\n"]
    pub fn recently_failed_runs(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryRecentlyFailedRunsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.recently_failed_runs", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `schedule_configuration` after provisioning.\n"]
    pub fn schedule_configuration(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryScheduleConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schedule_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `target_configuration` after provisioning.\n"]
    pub fn target_configuration(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryTargetConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> TimestreamqueryScheduledQueryTimeoutsElRef {
        TimestreamqueryScheduledQueryTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryErrorReportConfigurationElS3ConfigurationEl {
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_option: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_key_prefix: Option<PrimField<String>>,
}

impl TimestreamqueryScheduledQueryErrorReportConfigurationElS3ConfigurationEl {
    #[doc = "Set the field `encryption_option`.\n"]
    pub fn set_encryption_option(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_option = Some(v.into());
        self
    }

    #[doc = "Set the field `object_key_prefix`.\n"]
    pub fn set_object_key_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.object_key_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for TimestreamqueryScheduledQueryErrorReportConfigurationElS3ConfigurationEl {
    type O =
        BlockAssignable<TimestreamqueryScheduledQueryErrorReportConfigurationElS3ConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryErrorReportConfigurationElS3ConfigurationEl {
    #[doc = ""]
    pub bucket_name: PrimField<String>,
}

impl BuildTimestreamqueryScheduledQueryErrorReportConfigurationElS3ConfigurationEl {
    pub fn build(self) -> TimestreamqueryScheduledQueryErrorReportConfigurationElS3ConfigurationEl {
        TimestreamqueryScheduledQueryErrorReportConfigurationElS3ConfigurationEl {
            bucket_name: self.bucket_name,
            encryption_option: core::default::Default::default(),
            object_key_prefix: core::default::Default::default(),
        }
    }
}

pub struct TimestreamqueryScheduledQueryErrorReportConfigurationElS3ConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryErrorReportConfigurationElS3ConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryErrorReportConfigurationElS3ConfigurationElRef {
        TimestreamqueryScheduledQueryErrorReportConfigurationElS3ConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryErrorReportConfigurationElS3ConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_option` after provisioning.\n"]
    pub fn encryption_option(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_option", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `object_key_prefix` after provisioning.\n"]
    pub fn object_key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.object_key_prefix", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQueryErrorReportConfigurationElDynamic {
    s3_configuration: Option<
        DynamicBlock<TimestreamqueryScheduledQueryErrorReportConfigurationElS3ConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryErrorReportConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_configuration:
        Option<Vec<TimestreamqueryScheduledQueryErrorReportConfigurationElS3ConfigurationEl>>,
    dynamic: TimestreamqueryScheduledQueryErrorReportConfigurationElDynamic,
}

impl TimestreamqueryScheduledQueryErrorReportConfigurationEl {
    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                TimestreamqueryScheduledQueryErrorReportConfigurationElS3ConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for TimestreamqueryScheduledQueryErrorReportConfigurationEl {
    type O = BlockAssignable<TimestreamqueryScheduledQueryErrorReportConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryErrorReportConfigurationEl {}

impl BuildTimestreamqueryScheduledQueryErrorReportConfigurationEl {
    pub fn build(self) -> TimestreamqueryScheduledQueryErrorReportConfigurationEl {
        TimestreamqueryScheduledQueryErrorReportConfigurationEl {
            s3_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TimestreamqueryScheduledQueryErrorReportConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryErrorReportConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryErrorReportConfigurationElRef {
        TimestreamqueryScheduledQueryErrorReportConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryErrorReportConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryErrorReportConfigurationElS3ConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationElS3ReportLocationEl {}

impl TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationElS3ReportLocationEl {}

impl ToListMappable
    for TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationElS3ReportLocationEl
{
    type O = BlockAssignable<
        TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationElS3ReportLocationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationElS3ReportLocationEl
{}

impl BuildTimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationElS3ReportLocationEl {
    pub fn build(
        self,
    ) -> TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationElS3ReportLocationEl {
        TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationElS3ReportLocationEl {}
    }
}

pub struct TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationElS3ReportLocationElRef {
    shared: StackShared,
    base: String,
}

impl Ref
    for TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationElS3ReportLocationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationElS3ReportLocationElRef
    {
        TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationElS3ReportLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationElS3ReportLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `object_key` after provisioning.\n"]
    pub fn object_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationElDynamic {
    s3_report_location: Option<
        DynamicBlock<
            TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationElS3ReportLocationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_report_location: Option<
        Vec<TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationElS3ReportLocationEl>,
    >,
    dynamic: TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationElDynamic,
}

impl TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationEl {
    #[doc = "Set the field `s3_report_location`.\n"]
    pub fn set_s3_report_location(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationElS3ReportLocationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_report_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_report_location = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationEl {
    type O = BlockAssignable<TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationEl {}

impl BuildTimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationEl {
    pub fn build(self) -> TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationEl {
        TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationEl {
            s3_report_location: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationElRef {
        TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_report_location` after provisioning.\n"]
    pub fn s3_report_location(
        &self,
    ) -> ListRef<
        TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationElS3ReportLocationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_report_location", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryLastRunSummaryElExecutionStatsEl {}

impl TimestreamqueryScheduledQueryLastRunSummaryElExecutionStatsEl {}

impl ToListMappable for TimestreamqueryScheduledQueryLastRunSummaryElExecutionStatsEl {
    type O = BlockAssignable<TimestreamqueryScheduledQueryLastRunSummaryElExecutionStatsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryLastRunSummaryElExecutionStatsEl {}

impl BuildTimestreamqueryScheduledQueryLastRunSummaryElExecutionStatsEl {
    pub fn build(self) -> TimestreamqueryScheduledQueryLastRunSummaryElExecutionStatsEl {
        TimestreamqueryScheduledQueryLastRunSummaryElExecutionStatsEl {}
    }
}

pub struct TimestreamqueryScheduledQueryLastRunSummaryElExecutionStatsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryLastRunSummaryElExecutionStatsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryLastRunSummaryElExecutionStatsElRef {
        TimestreamqueryScheduledQueryLastRunSummaryElExecutionStatsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryLastRunSummaryElExecutionStatsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bytes_metered` after provisioning.\n"]
    pub fn bytes_metered(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bytes_metered", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cumulative_bytes_scanned` after provisioning.\n"]
    pub fn cumulative_bytes_scanned(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cumulative_bytes_scanned", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `data_writes` after provisioning.\n"]
    pub fn data_writes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_writes", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_time_in_millis` after provisioning.\n"]
    pub fn execution_time_in_millis(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_time_in_millis", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `query_result_rows` after provisioning.\n"]
    pub fn query_result_rows(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_result_rows", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `records_ingested` after provisioning.\n"]
    pub fn records_ingested(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.records_ingested", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageElMaxEl
{}

impl
    TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageElMaxEl
{
}

impl ToListMappable for TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageElMaxEl {
    type O =
        BlockAssignable<
            TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageElMaxEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageElMaxEl
{}

impl BuildTimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageElMaxEl {
    pub fn build(
        self,
    ) -> TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageElMaxEl {
        TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageElMaxEl {}
    }
}

pub struct TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageElMaxElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageElMaxElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageElMaxElRef {
        TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageElMaxElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageElMaxElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `partition_key` after provisioning.\n"]
    pub fn partition_key(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.partition_key", self.base))
    }

    #[doc = "Get a reference to the value of field `table_arn` after provisioning.\n"]
    pub fn table_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageElDynamic {
    max: Option<
        DynamicBlock<TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageElMaxEl>,
    >,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<Vec<TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageElMaxEl>>,
    dynamic: TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageElDynamic,
}

impl TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageElMaxEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.max = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.max = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageEl
{
    type O = BlockAssignable<
        TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageEl
{}

impl
    BuildTimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageEl
{
    pub fn build(
        self,
    ) -> TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageEl
    {
        TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageEl {
            max: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageElRef {
        TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageElMaxElRef>{
        ListRef::new(self.shared().clone(), format!("{}.max", self.base))
    }
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeElMaxEl
{}

impl TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeElMaxEl {}

impl ToListMappable for TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeElMaxEl {
    type O =
        BlockAssignable<
            TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeElMaxEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeElMaxEl
{}

impl BuildTimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeElMaxEl {
    pub fn build(
        self,
    ) -> TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeElMaxEl {
        TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeElMaxEl {}
    }
}

pub struct TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeElMaxElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeElMaxElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeElMaxElRef {
        TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeElMaxElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl
    TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeElMaxElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table_arn` after provisioning.\n"]
    pub fn table_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeElDynamic {
    max: Option<
        DynamicBlock<TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeElMaxEl>,
    >,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<Vec<TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeElMaxEl>>,
    dynamic: TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeElDynamic,
}

impl TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeElMaxEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.max = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.max = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeEl
{
    type O = BlockAssignable<
        TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeEl
{}

impl BuildTimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeEl {
    pub fn build(
        self,
    ) -> TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeEl
    {
        TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeEl {
            max: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeElRef
{
    shared: StackShared,
    base: String,
}

impl Ref
    for TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeElRef
    {
        TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeElMaxElRef>{
        ListRef::new(self.shared().clone(), format!("{}.max", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElDynamic {
    query_spatial_coverage: Option<
        DynamicBlock<TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageEl>,
    >,
    query_temporal_range: Option<
        DynamicBlock<TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeEl>,
    >,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    query_spatial_coverage: Option<
        Vec<TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_temporal_range: Option<
        Vec<TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeEl>,
    >,
    dynamic: TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElDynamic,
}

impl TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseEl {
    #[doc = "Set the field `query_spatial_coverage`.\n"]
    pub fn set_query_spatial_coverage(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_spatial_coverage = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_spatial_coverage = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `query_temporal_range`.\n"]
    pub fn set_query_temporal_range(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_temporal_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_temporal_range = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseEl {
    type O = BlockAssignable<TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseEl {}

impl BuildTimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseEl {
    pub fn build(self) -> TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseEl {
        TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseEl {
            query_spatial_coverage: core::default::Default::default(),
            query_temporal_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElRef {
        TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `output_bytes` after provisioning.\n"]
    pub fn output_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `output_rows` after provisioning.\n"]
    pub fn output_rows(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_rows", self.base))
    }

    #[doc = "Get a reference to the value of field `query_table_count` after provisioning.\n"]
    pub fn query_table_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_table_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `query_spatial_coverage` after provisioning.\n"]
    pub fn query_spatial_coverage(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQuerySpatialCoverageElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_spatial_coverage", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `query_temporal_range` after provisioning.\n"]
    pub fn query_temporal_range(
        &self,
    ) -> ListRef<
        TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElQueryTemporalRangeElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_temporal_range", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQueryLastRunSummaryElDynamic {
    error_report_location:
        Option<DynamicBlock<TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationEl>>,
    execution_stats:
        Option<DynamicBlock<TimestreamqueryScheduledQueryLastRunSummaryElExecutionStatsEl>>,
    query_insights_response:
        Option<DynamicBlock<TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseEl>>,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryLastRunSummaryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    error_report_location:
        Option<Vec<TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_stats: Option<Vec<TimestreamqueryScheduledQueryLastRunSummaryElExecutionStatsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_insights_response:
        Option<Vec<TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseEl>>,
    dynamic: TimestreamqueryScheduledQueryLastRunSummaryElDynamic,
}

impl TimestreamqueryScheduledQueryLastRunSummaryEl {
    #[doc = "Set the field `error_report_location`.\n"]
    pub fn set_error_report_location(
        mut self,
        v: impl Into<
            BlockAssignable<TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.error_report_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.error_report_location = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `execution_stats`.\n"]
    pub fn set_execution_stats(
        mut self,
        v: impl Into<BlockAssignable<TimestreamqueryScheduledQueryLastRunSummaryElExecutionStatsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.execution_stats = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.execution_stats = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `query_insights_response`.\n"]
    pub fn set_query_insights_response(
        mut self,
        v: impl Into<
            BlockAssignable<TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_insights_response = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_insights_response = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for TimestreamqueryScheduledQueryLastRunSummaryEl {
    type O = BlockAssignable<TimestreamqueryScheduledQueryLastRunSummaryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryLastRunSummaryEl {}

impl BuildTimestreamqueryScheduledQueryLastRunSummaryEl {
    pub fn build(self) -> TimestreamqueryScheduledQueryLastRunSummaryEl {
        TimestreamqueryScheduledQueryLastRunSummaryEl {
            error_report_location: core::default::Default::default(),
            execution_stats: core::default::Default::default(),
            query_insights_response: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TimestreamqueryScheduledQueryLastRunSummaryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryLastRunSummaryElRef {
    fn new(shared: StackShared, base: String) -> TimestreamqueryScheduledQueryLastRunSummaryElRef {
        TimestreamqueryScheduledQueryLastRunSummaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryLastRunSummaryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failure_reason` after provisioning.\n"]
    pub fn failure_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failure_reason", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `invocation_time` after provisioning.\n"]
    pub fn invocation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.invocation_time", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `run_status` after provisioning.\n"]
    pub fn run_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.run_status", self.base))
    }

    #[doc = "Get a reference to the value of field `trigger_time` after provisioning.\n"]
    pub fn trigger_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trigger_time", self.base))
    }

    #[doc = "Get a reference to the value of field `error_report_location` after provisioning.\n"]
    pub fn error_report_location(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryLastRunSummaryElErrorReportLocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.error_report_location", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `execution_stats` after provisioning.\n"]
    pub fn execution_stats(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryLastRunSummaryElExecutionStatsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.execution_stats", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `query_insights_response` after provisioning.\n"]
    pub fn query_insights_response(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryLastRunSummaryElQueryInsightsResponseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_insights_response", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryNotificationConfigurationElSnsConfigurationEl {
    topic_arn: PrimField<String>,
}

impl TimestreamqueryScheduledQueryNotificationConfigurationElSnsConfigurationEl {}

impl ToListMappable for TimestreamqueryScheduledQueryNotificationConfigurationElSnsConfigurationEl {
    type O =
        BlockAssignable<TimestreamqueryScheduledQueryNotificationConfigurationElSnsConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryNotificationConfigurationElSnsConfigurationEl {
    #[doc = ""]
    pub topic_arn: PrimField<String>,
}

impl BuildTimestreamqueryScheduledQueryNotificationConfigurationElSnsConfigurationEl {
    pub fn build(
        self,
    ) -> TimestreamqueryScheduledQueryNotificationConfigurationElSnsConfigurationEl {
        TimestreamqueryScheduledQueryNotificationConfigurationElSnsConfigurationEl {
            topic_arn: self.topic_arn,
        }
    }
}

pub struct TimestreamqueryScheduledQueryNotificationConfigurationElSnsConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryNotificationConfigurationElSnsConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryNotificationConfigurationElSnsConfigurationElRef {
        TimestreamqueryScheduledQueryNotificationConfigurationElSnsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryNotificationConfigurationElSnsConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQueryNotificationConfigurationElDynamic {
    sns_configuration: Option<
        DynamicBlock<TimestreamqueryScheduledQueryNotificationConfigurationElSnsConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryNotificationConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    sns_configuration:
        Option<Vec<TimestreamqueryScheduledQueryNotificationConfigurationElSnsConfigurationEl>>,
    dynamic: TimestreamqueryScheduledQueryNotificationConfigurationElDynamic,
}

impl TimestreamqueryScheduledQueryNotificationConfigurationEl {
    #[doc = "Set the field `sns_configuration`.\n"]
    pub fn set_sns_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                TimestreamqueryScheduledQueryNotificationConfigurationElSnsConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sns_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sns_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for TimestreamqueryScheduledQueryNotificationConfigurationEl {
    type O = BlockAssignable<TimestreamqueryScheduledQueryNotificationConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryNotificationConfigurationEl {}

impl BuildTimestreamqueryScheduledQueryNotificationConfigurationEl {
    pub fn build(self) -> TimestreamqueryScheduledQueryNotificationConfigurationEl {
        TimestreamqueryScheduledQueryNotificationConfigurationEl {
            sns_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TimestreamqueryScheduledQueryNotificationConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryNotificationConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryNotificationConfigurationElRef {
        TimestreamqueryScheduledQueryNotificationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryNotificationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sns_configuration` after provisioning.\n"]
    pub fn sns_configuration(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryNotificationConfigurationElSnsConfigurationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sns_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationElS3ReportLocationEl
{}

impl TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationElS3ReportLocationEl {}

impl ToListMappable
    for TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationElS3ReportLocationEl
{
    type O = BlockAssignable<
        TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationElS3ReportLocationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationElS3ReportLocationEl
{}

impl BuildTimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationElS3ReportLocationEl {
    pub fn build(
        self,
    ) -> TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationElS3ReportLocationEl
    {
        TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationElS3ReportLocationEl {}
    }
}

pub struct TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationElS3ReportLocationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref
    for TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationElS3ReportLocationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationElS3ReportLocationElRef
    {
        TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationElS3ReportLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationElS3ReportLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `object_key` after provisioning.\n"]
    pub fn object_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationElDynamic {
    s3_report_location: Option<
        DynamicBlock<TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationElS3ReportLocationEl>,
    >,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_report_location: Option<
        Vec<TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationElS3ReportLocationEl>,
    >,
    dynamic: TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationElDynamic,
}

impl TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationEl {
    #[doc = "Set the field `s3_report_location`.\n"]
    pub fn set_s3_report_location(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationElS3ReportLocationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_report_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_report_location = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationEl {
    type O =
        BlockAssignable<TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationEl {}

impl BuildTimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationEl {
    pub fn build(self) -> TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationEl {
        TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationEl {
            s3_report_location: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationElRef {
        TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_report_location` after provisioning.\n"]
    pub fn s3_report_location(
        &self,
    ) -> ListRef<
        TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationElS3ReportLocationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_report_location", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryRecentlyFailedRunsElExecutionStatsEl {}

impl TimestreamqueryScheduledQueryRecentlyFailedRunsElExecutionStatsEl {}

impl ToListMappable for TimestreamqueryScheduledQueryRecentlyFailedRunsElExecutionStatsEl {
    type O = BlockAssignable<TimestreamqueryScheduledQueryRecentlyFailedRunsElExecutionStatsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryRecentlyFailedRunsElExecutionStatsEl {}

impl BuildTimestreamqueryScheduledQueryRecentlyFailedRunsElExecutionStatsEl {
    pub fn build(self) -> TimestreamqueryScheduledQueryRecentlyFailedRunsElExecutionStatsEl {
        TimestreamqueryScheduledQueryRecentlyFailedRunsElExecutionStatsEl {}
    }
}

pub struct TimestreamqueryScheduledQueryRecentlyFailedRunsElExecutionStatsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryRecentlyFailedRunsElExecutionStatsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryRecentlyFailedRunsElExecutionStatsElRef {
        TimestreamqueryScheduledQueryRecentlyFailedRunsElExecutionStatsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryRecentlyFailedRunsElExecutionStatsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bytes_metered` after provisioning.\n"]
    pub fn bytes_metered(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bytes_metered", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cumulative_bytes_scanned` after provisioning.\n"]
    pub fn cumulative_bytes_scanned(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cumulative_bytes_scanned", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `data_writes` after provisioning.\n"]
    pub fn data_writes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_writes", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_time_in_millis` after provisioning.\n"]
    pub fn execution_time_in_millis(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_time_in_millis", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `query_result_rows` after provisioning.\n"]
    pub fn query_result_rows(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_result_rows", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `records_ingested` after provisioning.\n"]
    pub fn records_ingested(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.records_ingested", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageElMaxEl
{}

impl TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageElMaxEl { }

impl ToListMappable for TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageElMaxEl {
    type O =
        BlockAssignable<
            TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageElMaxEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageElMaxEl
{}

impl BuildTimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageElMaxEl {
    pub fn build(
        self,
    ) -> TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageElMaxEl {
        TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageElMaxEl {}
    }
}

pub struct TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageElMaxElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageElMaxElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageElMaxElRef {
        TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageElMaxElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageElMaxElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `partition_key` after provisioning.\n"]
    pub fn partition_key(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.partition_key", self.base))
    }

    #[doc = "Get a reference to the value of field `table_arn` after provisioning.\n"]
    pub fn table_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageElDynamic {
    max: Option<
        DynamicBlock<
            TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageElMaxEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<Vec<TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageElMaxEl>>,
    dynamic: TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageElDynamic,
}

impl
    TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageEl
{
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageElMaxEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.max = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.max = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageEl {
    type O =
        BlockAssignable<
            TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageEl
{}

impl BuildTimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageEl {
    pub fn build(
        self,
    ) -> TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageEl {
        TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageEl {
            max: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageElRef {
        TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(
        &self,
    ) -> ListRef<
        TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageElMaxElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.max", self.base))
    }
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeElMaxEl
{}

impl TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeElMaxEl { }

impl ToListMappable for TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeElMaxEl {
    type O =
        BlockAssignable<
            TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeElMaxEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeElMaxEl
{}

impl BuildTimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeElMaxEl {
    pub fn build(
        self,
    ) -> TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeElMaxEl {
        TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeElMaxEl {}
    }
}

pub struct TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeElMaxElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeElMaxElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeElMaxElRef {
        TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeElMaxElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeElMaxElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table_arn` after provisioning.\n"]
    pub fn table_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeElDynamic {
    max: Option<
        DynamicBlock<
            TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeElMaxEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<Vec<TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeElMaxEl>>,
    dynamic: TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeElDynamic,
}

impl TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeElMaxEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.max = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.max = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeEl
{
    type O =
        BlockAssignable<
            TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeEl
{}

impl BuildTimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeEl {
    pub fn build(
        self,
    ) -> TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeEl {
        TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeEl {
            max: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeElRef {
        TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl
    TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(
        &self,
    ) -> ListRef<
        TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeElMaxElRef,
    >{
        ListRef::new(self.shared().clone(), format!("{}.max", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElDynamic {
    query_spatial_coverage: Option<
        DynamicBlock<TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageEl>,
    >,
    query_temporal_range: Option<
        DynamicBlock<TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeEl>,
    >,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    query_spatial_coverage: Option<
        Vec<TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_temporal_range: Option<
        Vec<TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeEl>,
    >,
    dynamic: TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElDynamic,
}

impl TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseEl {
    #[doc = "Set the field `query_spatial_coverage`.\n"]
    pub fn set_query_spatial_coverage(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_spatial_coverage = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_spatial_coverage = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `query_temporal_range`.\n"]
    pub fn set_query_temporal_range(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_temporal_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_temporal_range = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseEl {
    type O =
        BlockAssignable<TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseEl {}

impl BuildTimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseEl {
    pub fn build(self) -> TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseEl {
        TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseEl {
            query_spatial_coverage: core::default::Default::default(),
            query_temporal_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElRef {
        TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `output_bytes` after provisioning.\n"]
    pub fn output_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `output_rows` after provisioning.\n"]
    pub fn output_rows(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_rows", self.base))
    }

    #[doc = "Get a reference to the value of field `query_table_count` after provisioning.\n"]
    pub fn query_table_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_table_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `query_spatial_coverage` after provisioning.\n"]
    pub fn query_spatial_coverage(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQuerySpatialCoverageElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_spatial_coverage", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `query_temporal_range` after provisioning.\n"]
    pub fn query_temporal_range(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElQueryTemporalRangeElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_temporal_range", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQueryRecentlyFailedRunsElDynamic {
    error_report_location: Option<
        DynamicBlock<TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationEl>,
    >,
    execution_stats:
        Option<DynamicBlock<TimestreamqueryScheduledQueryRecentlyFailedRunsElExecutionStatsEl>>,
    query_insights_response: Option<
        DynamicBlock<TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseEl>,
    >,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryRecentlyFailedRunsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    error_report_location:
        Option<Vec<TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_stats: Option<Vec<TimestreamqueryScheduledQueryRecentlyFailedRunsElExecutionStatsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_insights_response:
        Option<Vec<TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseEl>>,
    dynamic: TimestreamqueryScheduledQueryRecentlyFailedRunsElDynamic,
}

impl TimestreamqueryScheduledQueryRecentlyFailedRunsEl {
    #[doc = "Set the field `error_report_location`.\n"]
    pub fn set_error_report_location(
        mut self,
        v: impl Into<
            BlockAssignable<TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.error_report_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.error_report_location = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `execution_stats`.\n"]
    pub fn set_execution_stats(
        mut self,
        v: impl Into<BlockAssignable<TimestreamqueryScheduledQueryRecentlyFailedRunsElExecutionStatsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.execution_stats = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.execution_stats = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `query_insights_response`.\n"]
    pub fn set_query_insights_response(
        mut self,
        v: impl Into<
            BlockAssignable<
                TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_insights_response = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_insights_response = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for TimestreamqueryScheduledQueryRecentlyFailedRunsEl {
    type O = BlockAssignable<TimestreamqueryScheduledQueryRecentlyFailedRunsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryRecentlyFailedRunsEl {}

impl BuildTimestreamqueryScheduledQueryRecentlyFailedRunsEl {
    pub fn build(self) -> TimestreamqueryScheduledQueryRecentlyFailedRunsEl {
        TimestreamqueryScheduledQueryRecentlyFailedRunsEl {
            error_report_location: core::default::Default::default(),
            execution_stats: core::default::Default::default(),
            query_insights_response: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TimestreamqueryScheduledQueryRecentlyFailedRunsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryRecentlyFailedRunsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryRecentlyFailedRunsElRef {
        TimestreamqueryScheduledQueryRecentlyFailedRunsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryRecentlyFailedRunsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failure_reason` after provisioning.\n"]
    pub fn failure_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failure_reason", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `invocation_time` after provisioning.\n"]
    pub fn invocation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.invocation_time", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `run_status` after provisioning.\n"]
    pub fn run_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.run_status", self.base))
    }

    #[doc = "Get a reference to the value of field `trigger_time` after provisioning.\n"]
    pub fn trigger_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trigger_time", self.base))
    }

    #[doc = "Get a reference to the value of field `error_report_location` after provisioning.\n"]
    pub fn error_report_location(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryRecentlyFailedRunsElErrorReportLocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.error_report_location", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `execution_stats` after provisioning.\n"]
    pub fn execution_stats(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryRecentlyFailedRunsElExecutionStatsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.execution_stats", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `query_insights_response` after provisioning.\n"]
    pub fn query_insights_response(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryRecentlyFailedRunsElQueryInsightsResponseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_insights_response", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryScheduleConfigurationEl {
    schedule_expression: PrimField<String>,
}

impl TimestreamqueryScheduledQueryScheduleConfigurationEl {}

impl ToListMappable for TimestreamqueryScheduledQueryScheduleConfigurationEl {
    type O = BlockAssignable<TimestreamqueryScheduledQueryScheduleConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryScheduleConfigurationEl {
    #[doc = ""]
    pub schedule_expression: PrimField<String>,
}

impl BuildTimestreamqueryScheduledQueryScheduleConfigurationEl {
    pub fn build(self) -> TimestreamqueryScheduledQueryScheduleConfigurationEl {
        TimestreamqueryScheduledQueryScheduleConfigurationEl {
            schedule_expression: self.schedule_expression,
        }
    }
}

pub struct TimestreamqueryScheduledQueryScheduleConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryScheduleConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryScheduleConfigurationElRef {
        TimestreamqueryScheduledQueryScheduleConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryScheduleConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schedule_expression` after provisioning.\n"]
    pub fn schedule_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schedule_expression", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElDimensionMappingEl
{
    dimension_value_type: PrimField<String>,
    name: PrimField<String>,
}

impl TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElDimensionMappingEl {}

impl ToListMappable for TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElDimensionMappingEl {
    type O =
        BlockAssignable<
            TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElDimensionMappingEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElDimensionMappingEl
{
    #[doc = ""]
    pub dimension_value_type: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildTimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElDimensionMappingEl {
    pub fn build(
        self,
    ) -> TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElDimensionMappingEl {
        TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElDimensionMappingEl {
            dimension_value_type: self.dimension_value_type,
            name: self.name,
        }
    }
}

pub struct TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElDimensionMappingElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElDimensionMappingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElDimensionMappingElRef {
        TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElDimensionMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl
    TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElDimensionMappingElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dimension_value_type` after provisioning.\n"]
    pub fn dimension_value_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dimension_value_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingElMultiMeasureAttributeMappingEl
{
    measure_value_type: PrimField<String>,
    source_column: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_multi_measure_attribute_name: Option<PrimField<String>>,
}

impl TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingElMultiMeasureAttributeMappingEl {
    #[doc = "Set the field `target_multi_measure_attribute_name`.\n"]
    pub fn set_target_multi_measure_attribute_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_multi_measure_attribute_name = Some(v.into());
        self
    }
}

impl ToListMappable for TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingElMultiMeasureAttributeMappingEl {
    type O =
        BlockAssignable<
            TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingElMultiMeasureAttributeMappingEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingElMultiMeasureAttributeMappingEl
{
    #[doc = ""]
    pub measure_value_type: PrimField<String>,
    #[doc = ""]
    pub source_column: PrimField<String>,
}

impl BuildTimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingElMultiMeasureAttributeMappingEl {
    pub fn build(
        self,
    ) -> TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingElMultiMeasureAttributeMappingEl {
        TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingElMultiMeasureAttributeMappingEl {
            measure_value_type: self.measure_value_type,
            source_column: self.source_column,
            target_multi_measure_attribute_name: core::default::Default::default(),
        }
    }
}

pub struct TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingElMultiMeasureAttributeMappingElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingElMultiMeasureAttributeMappingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingElMultiMeasureAttributeMappingElRef {
        TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingElMultiMeasureAttributeMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingElMultiMeasureAttributeMappingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `measure_value_type` after provisioning.\n"]
    pub fn measure_value_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.measure_value_type", self.base))
    }

    #[doc = "Get a reference to the value of field `source_column` after provisioning.\n"]
    pub fn source_column(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_column", self.base))
    }

    #[doc = "Get a reference to the value of field `target_multi_measure_attribute_name` after provisioning.\n"]
    pub fn target_multi_measure_attribute_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_multi_measure_attribute_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingElDynamic {
    multi_measure_attribute_mapping: Option<
        DynamicBlock<
            TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingElMultiMeasureAttributeMappingEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    measure_name: Option<PrimField<String>>,
    measure_value_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_column: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_measure_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_measure_attribute_mapping: Option<
        Vec<
            TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingElMultiMeasureAttributeMappingEl,
        >,
    >,
    dynamic: TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingElDynamic,
}

impl
    TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingEl
{
    #[doc = "Set the field `measure_name`.\n"]
    pub fn set_measure_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.measure_name = Some(v.into());
        self
    }

    #[doc = "Set the field `source_column`.\n"]
    pub fn set_source_column(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_column = Some(v.into());
        self
    }

    #[doc = "Set the field `target_measure_name`.\n"]
    pub fn set_target_measure_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_measure_name = Some(v.into());
        self
    }

    #[doc = "Set the field `multi_measure_attribute_mapping`.\n"]
    pub fn set_multi_measure_attribute_mapping(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingElMultiMeasureAttributeMappingEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.multi_measure_attribute_mapping = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.multi_measure_attribute_mapping = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingEl {
    type O =
        BlockAssignable<
            TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingEl
{
    #[doc = ""]
    pub measure_value_type: PrimField<String>,
}

impl BuildTimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingEl {
    pub fn build(
        self,
    ) -> TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingEl {
        TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingEl {
            measure_name: core::default::Default::default(),
            measure_value_type: self.measure_value_type,
            source_column: core::default::Default::default(),
            target_measure_name: core::default::Default::default(),
            multi_measure_attribute_mapping: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingElRef {
        TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `measure_name` after provisioning.\n"]
    pub fn measure_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.measure_name", self.base))
    }

    #[doc = "Get a reference to the value of field `measure_value_type` after provisioning.\n"]
    pub fn measure_value_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.measure_value_type", self.base))
    }

    #[doc = "Get a reference to the value of field `source_column` after provisioning.\n"]
    pub fn source_column(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_column", self.base))
    }

    #[doc = "Get a reference to the value of field `target_measure_name` after provisioning.\n"]
    pub fn target_measure_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_measure_name", self.base))
    }

    #[doc = "Get a reference to the value of field `multi_measure_attribute_mapping` after provisioning.\n"]
    pub fn multi_measure_attribute_mapping(
        &self,
    ) -> ListRef<
        TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingElMultiMeasureAttributeMappingElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.multi_measure_attribute_mapping", self.base))
    }
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsElMultiMeasureAttributeMappingEl
{
    measure_value_type: PrimField<String>,
    source_column: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_multi_measure_attribute_name: Option<PrimField<String>>,
}

impl TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsElMultiMeasureAttributeMappingEl {
    #[doc = "Set the field `target_multi_measure_attribute_name`.\n"]
    pub fn set_target_multi_measure_attribute_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_multi_measure_attribute_name = Some(v.into());
        self
    }
}

impl ToListMappable for TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsElMultiMeasureAttributeMappingEl {
    type O =
        BlockAssignable<
            TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsElMultiMeasureAttributeMappingEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsElMultiMeasureAttributeMappingEl
{
    #[doc = ""]
    pub measure_value_type: PrimField<String>,
    #[doc = ""]
    pub source_column: PrimField<String>,
}

impl BuildTimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsElMultiMeasureAttributeMappingEl {
    pub fn build(
        self,
    ) -> TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsElMultiMeasureAttributeMappingEl {
        TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsElMultiMeasureAttributeMappingEl {
            measure_value_type: self.measure_value_type,
            source_column: self.source_column,
            target_multi_measure_attribute_name: core::default::Default::default(),
        }
    }
}

pub struct TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsElMultiMeasureAttributeMappingElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsElMultiMeasureAttributeMappingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsElMultiMeasureAttributeMappingElRef {
        TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsElMultiMeasureAttributeMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsElMultiMeasureAttributeMappingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `measure_value_type` after provisioning.\n"]
    pub fn measure_value_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.measure_value_type", self.base))
    }

    #[doc = "Get a reference to the value of field `source_column` after provisioning.\n"]
    pub fn source_column(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_column", self.base))
    }

    #[doc = "Get a reference to the value of field `target_multi_measure_attribute_name` after provisioning.\n"]
    pub fn target_multi_measure_attribute_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_multi_measure_attribute_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsElDynamic {
    multi_measure_attribute_mapping: Option<
        DynamicBlock<
            TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsElMultiMeasureAttributeMappingEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target_multi_measure_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_measure_attribute_mapping: Option<
        Vec<
            TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsElMultiMeasureAttributeMappingEl,
        >,
    >,
    dynamic: TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsElDynamic,
}

impl TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsEl {
    #[doc = "Set the field `target_multi_measure_name`.\n"]
    pub fn set_target_multi_measure_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_multi_measure_name = Some(v.into());
        self
    }

    #[doc = "Set the field `multi_measure_attribute_mapping`.\n"]
    pub fn set_multi_measure_attribute_mapping(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsElMultiMeasureAttributeMappingEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.multi_measure_attribute_mapping = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.multi_measure_attribute_mapping = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsEl {
    type O =
        BlockAssignable<
            TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsEl
{}

impl BuildTimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsEl {
    pub fn build(
        self,
    ) -> TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsEl {
        TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsEl {
            target_multi_measure_name: core::default::Default::default(),
            multi_measure_attribute_mapping: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsElRef {
        TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_multi_measure_name` after provisioning.\n"]
    pub fn target_multi_measure_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_multi_measure_name", self.base))
    }

    #[doc = "Get a reference to the value of field `multi_measure_attribute_mapping` after provisioning.\n"]
    pub fn multi_measure_attribute_mapping(
        &self,
    ) -> ListRef<
        TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsElMultiMeasureAttributeMappingElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.multi_measure_attribute_mapping", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElDynamic {
    dimension_mapping: Option<
        DynamicBlock<TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElDimensionMappingEl>,
    >,
    mixed_measure_mapping: Option<
        DynamicBlock<TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingEl>,
    >,
    multi_measure_mappings: Option<
        DynamicBlock<
            TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationEl {
    database_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    measure_name_column: Option<PrimField<String>>,
    table_name: PrimField<String>,
    time_column: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension_mapping: Option<
        Vec<TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElDimensionMappingEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    mixed_measure_mapping: Option<
        Vec<TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_measure_mappings: Option<
        Vec<TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsEl>,
    >,
    dynamic: TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElDynamic,
}

impl TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationEl {
    #[doc = "Set the field `measure_name_column`.\n"]
    pub fn set_measure_name_column(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.measure_name_column = Some(v.into());
        self
    }

    #[doc = "Set the field `dimension_mapping`.\n"]
    pub fn set_dimension_mapping(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElDimensionMappingEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dimension_mapping = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension_mapping = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `mixed_measure_mapping`.\n"]
    pub fn set_mixed_measure_mapping(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mixed_measure_mapping = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mixed_measure_mapping = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `multi_measure_mappings`.\n"]
    pub fn set_multi_measure_mappings(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.multi_measure_mappings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.multi_measure_mappings = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationEl
{
    type O = BlockAssignable<
        TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationEl {
    #[doc = ""]
    pub database_name: PrimField<String>,
    #[doc = ""]
    pub table_name: PrimField<String>,
    #[doc = ""]
    pub time_column: PrimField<String>,
}

impl BuildTimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationEl {
    pub fn build(
        self,
    ) -> TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationEl {
        TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationEl {
            database_name: self.database_name,
            measure_name_column: core::default::Default::default(),
            table_name: self.table_name,
            time_column: self.time_column,
            dimension_mapping: core::default::Default::default(),
            mixed_measure_mapping: core::default::Default::default(),
            multi_measure_mappings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElRef {
        TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `measure_name_column` after provisioning.\n"]
    pub fn measure_name_column(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.measure_name_column", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }

    #[doc = "Get a reference to the value of field `time_column` after provisioning.\n"]
    pub fn time_column(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_column", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension_mapping` after provisioning.\n"]
    pub fn dimension_mapping(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElDimensionMappingElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.dimension_mapping", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `mixed_measure_mapping` after provisioning.\n"]
    pub fn mixed_measure_mapping(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMixedMeasureMappingElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.mixed_measure_mapping", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `multi_measure_mappings` after provisioning.\n"]
    pub fn multi_measure_mappings(
        &self,
    ) -> ListRef<
        TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElMultiMeasureMappingsElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.multi_measure_mappings", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQueryTargetConfigurationElDynamic {
    timestream_configuration: Option<
        DynamicBlock<TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryTargetConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    timestream_configuration:
        Option<Vec<TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationEl>>,
    dynamic: TimestreamqueryScheduledQueryTargetConfigurationElDynamic,
}

impl TimestreamqueryScheduledQueryTargetConfigurationEl {
    #[doc = "Set the field `timestream_configuration`.\n"]
    pub fn set_timestream_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.timestream_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.timestream_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for TimestreamqueryScheduledQueryTargetConfigurationEl {
    type O = BlockAssignable<TimestreamqueryScheduledQueryTargetConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryTargetConfigurationEl {}

impl BuildTimestreamqueryScheduledQueryTargetConfigurationEl {
    pub fn build(self) -> TimestreamqueryScheduledQueryTargetConfigurationEl {
        TimestreamqueryScheduledQueryTargetConfigurationEl {
            timestream_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TimestreamqueryScheduledQueryTargetConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryTargetConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamqueryScheduledQueryTargetConfigurationElRef {
        TimestreamqueryScheduledQueryTargetConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryTargetConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `timestream_configuration` after provisioning.\n"]
    pub fn timestream_configuration(
        &self,
    ) -> ListRef<TimestreamqueryScheduledQueryTargetConfigurationElTimestreamConfigurationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.timestream_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQueryTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl TimestreamqueryScheduledQueryTimeoutsEl {
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

impl ToListMappable for TimestreamqueryScheduledQueryTimeoutsEl {
    type O = BlockAssignable<TimestreamqueryScheduledQueryTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQueryTimeoutsEl {}

impl BuildTimestreamqueryScheduledQueryTimeoutsEl {
    pub fn build(self) -> TimestreamqueryScheduledQueryTimeoutsEl {
        TimestreamqueryScheduledQueryTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct TimestreamqueryScheduledQueryTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TimestreamqueryScheduledQueryTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> TimestreamqueryScheduledQueryTimeoutsElRef {
        TimestreamqueryScheduledQueryTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQueryTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQueryDynamic {
    error_report_configuration:
        Option<DynamicBlock<TimestreamqueryScheduledQueryErrorReportConfigurationEl>>,
    last_run_summary: Option<DynamicBlock<TimestreamqueryScheduledQueryLastRunSummaryEl>>,
    notification_configuration:
        Option<DynamicBlock<TimestreamqueryScheduledQueryNotificationConfigurationEl>>,
    recently_failed_runs: Option<DynamicBlock<TimestreamqueryScheduledQueryRecentlyFailedRunsEl>>,
    schedule_configuration:
        Option<DynamicBlock<TimestreamqueryScheduledQueryScheduleConfigurationEl>>,
    target_configuration: Option<DynamicBlock<TimestreamqueryScheduledQueryTargetConfigurationEl>>,
}
