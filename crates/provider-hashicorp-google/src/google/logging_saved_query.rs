use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct LoggingSavedQueryData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    parent: PrimField<String>,
    visibility: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_query: Option<Vec<LoggingSavedQueryLoggingQueryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ops_analytics_query: Option<Vec<LoggingSavedQueryOpsAnalyticsQueryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<LoggingSavedQueryTimeoutsEl>,
    dynamic: LoggingSavedQueryDynamic,
}
struct LoggingSavedQuery_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LoggingSavedQueryData>,
}
#[derive(Clone)]
pub struct LoggingSavedQuery(Rc<LoggingSavedQuery_>);
impl LoggingSavedQuery {
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
    #[doc = "Set the field `description`.\nA description of the saved query."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `logging_query`.\n"]
    pub fn set_logging_query(
        self,
        v: impl Into<BlockAssignable<LoggingSavedQueryLoggingQueryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_query = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_query = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ops_analytics_query`.\n"]
    pub fn set_ops_analytics_query(
        self,
        v: impl Into<BlockAssignable<LoggingSavedQueryOpsAnalyticsQueryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ops_analytics_query = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ops_analytics_query = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<LoggingSavedQueryTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The creation timestamp of the saved query."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the saved query."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe user-visible display name of the saved query."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource see\n[supported regions](https://docs.cloud.google.com/logging/docs/region-support#bucket-regions)."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the saved query. For example: 'my-saved-query'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe parent of the resource."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The last update timestamp of the saved query."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `visibility` after provisioning.\nThe visibility of the saved query. Possible values: [\"SHARED\", \"PRIVATE\"]"]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.visibility", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_query` after provisioning.\n"]
    pub fn logging_query(&self) -> ListRef<LoggingSavedQueryLoggingQueryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_query", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ops_analytics_query` after provisioning.\n"]
    pub fn ops_analytics_query(&self) -> ListRef<LoggingSavedQueryOpsAnalyticsQueryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ops_analytics_query", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LoggingSavedQueryTimeoutsElRef {
        LoggingSavedQueryTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for LoggingSavedQuery {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for LoggingSavedQuery {}
impl ToListMappable for LoggingSavedQuery {
    type O = ListRef<LoggingSavedQueryRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for LoggingSavedQuery_ {
    fn extract_resource_type(&self) -> String {
        "google_logging_saved_query".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildLoggingSavedQuery {
    pub tf_id: String,
    #[doc = "The user-visible display name of the saved query."]
    pub display_name: PrimField<String>,
    #[doc = "The location of the resource see\n[supported regions](https://docs.cloud.google.com/logging/docs/region-support#bucket-regions)."]
    pub location: PrimField<String>,
    #[doc = "The name of the saved query. For example: 'my-saved-query'"]
    pub name: PrimField<String>,
    #[doc = "The parent of the resource."]
    pub parent: PrimField<String>,
    #[doc = "The visibility of the saved query. Possible values: [\"SHARED\", \"PRIVATE\"]"]
    pub visibility: PrimField<String>,
}
impl BuildLoggingSavedQuery {
    pub fn build(self, stack: &mut Stack) -> LoggingSavedQuery {
        let out = LoggingSavedQuery(Rc::new(LoggingSavedQuery_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LoggingSavedQueryData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                parent: self.parent,
                visibility: self.visibility,
                logging_query: core::default::Default::default(),
                ops_analytics_query: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct LoggingSavedQueryRef {
    shared: StackShared,
    base: String,
}
impl Ref for LoggingSavedQueryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl LoggingSavedQueryRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The creation timestamp of the saved query."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the saved query."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe user-visible display name of the saved query."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource see\n[supported regions](https://docs.cloud.google.com/logging/docs/region-support#bucket-regions)."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the saved query. For example: 'my-saved-query'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe parent of the resource."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The last update timestamp of the saved query."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `visibility` after provisioning.\nThe visibility of the saved query. Possible values: [\"SHARED\", \"PRIVATE\"]"]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.visibility", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_query` after provisioning.\n"]
    pub fn logging_query(&self) -> ListRef<LoggingSavedQueryLoggingQueryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_query", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ops_analytics_query` after provisioning.\n"]
    pub fn ops_analytics_query(&self) -> ListRef<LoggingSavedQueryOpsAnalyticsQueryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ops_analytics_query", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LoggingSavedQueryTimeoutsElRef {
        LoggingSavedQueryTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct LoggingSavedQueryLoggingQueryElSummaryFieldsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<PrimField<String>>,
}
impl LoggingSavedQueryLoggingQueryElSummaryFieldsEl {
    #[doc = "Set the field `field`.\nThe field from the LogEntry to include in the summary line."]
    pub fn set_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field = Some(v.into());
        self
    }
}
impl ToListMappable for LoggingSavedQueryLoggingQueryElSummaryFieldsEl {
    type O = BlockAssignable<LoggingSavedQueryLoggingQueryElSummaryFieldsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLoggingSavedQueryLoggingQueryElSummaryFieldsEl {}
impl BuildLoggingSavedQueryLoggingQueryElSummaryFieldsEl {
    pub fn build(self) -> LoggingSavedQueryLoggingQueryElSummaryFieldsEl {
        LoggingSavedQueryLoggingQueryElSummaryFieldsEl {
            field: core::default::Default::default(),
        }
    }
}
pub struct LoggingSavedQueryLoggingQueryElSummaryFieldsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LoggingSavedQueryLoggingQueryElSummaryFieldsElRef {
    fn new(shared: StackShared, base: String) -> LoggingSavedQueryLoggingQueryElSummaryFieldsElRef {
        LoggingSavedQueryLoggingQueryElSummaryFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LoggingSavedQueryLoggingQueryElSummaryFieldsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `field` after provisioning.\nThe field from the LogEntry to include in the summary line."]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }
}
#[derive(Serialize, Default)]
struct LoggingSavedQueryLoggingQueryElDynamic {
    summary_fields: Option<DynamicBlock<LoggingSavedQueryLoggingQueryElSummaryFieldsEl>>,
}
#[derive(Serialize)]
pub struct LoggingSavedQueryLoggingQueryEl {
    filter: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary_field_end: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary_field_start: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary_fields: Option<Vec<LoggingSavedQueryLoggingQueryElSummaryFieldsEl>>,
    dynamic: LoggingSavedQueryLoggingQueryElDynamic,
}
impl LoggingSavedQueryLoggingQueryEl {
    #[doc = "Set the field `summary_field_end`.\nCharacters will be counted from the end of the string."]
    pub fn set_summary_field_end(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.summary_field_end = Some(v.into());
        self
    }
    #[doc = "Set the field `summary_field_start`.\nCharacters will be counted from the start of the string."]
    pub fn set_summary_field_start(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.summary_field_start = Some(v.into());
        self
    }
    #[doc = "Set the field `summary_fields`.\n"]
    pub fn set_summary_fields(
        mut self,
        v: impl Into<BlockAssignable<LoggingSavedQueryLoggingQueryElSummaryFieldsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.summary_fields = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.summary_fields = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for LoggingSavedQueryLoggingQueryEl {
    type O = BlockAssignable<LoggingSavedQueryLoggingQueryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLoggingSavedQueryLoggingQueryEl {
    #[doc = "An [advanced logs filter](https://cloud.google.com/logging/docs/view/advanced-filters) which\nis used to match log entries."]
    pub filter: PrimField<String>,
}
impl BuildLoggingSavedQueryLoggingQueryEl {
    pub fn build(self) -> LoggingSavedQueryLoggingQueryEl {
        LoggingSavedQueryLoggingQueryEl {
            filter: self.filter,
            summary_field_end: core::default::Default::default(),
            summary_field_start: core::default::Default::default(),
            summary_fields: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LoggingSavedQueryLoggingQueryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LoggingSavedQueryLoggingQueryElRef {
    fn new(shared: StackShared, base: String) -> LoggingSavedQueryLoggingQueryElRef {
        LoggingSavedQueryLoggingQueryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LoggingSavedQueryLoggingQueryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nAn [advanced logs filter](https://cloud.google.com/logging/docs/view/advanced-filters) which\nis used to match log entries."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
    #[doc = "Get a reference to the value of field `summary_field_end` after provisioning.\nCharacters will be counted from the end of the string."]
    pub fn summary_field_end(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.summary_field_end", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `summary_field_start` after provisioning.\nCharacters will be counted from the start of the string."]
    pub fn summary_field_start(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.summary_field_start", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `summary_fields` after provisioning.\n"]
    pub fn summary_fields(&self) -> ListRef<LoggingSavedQueryLoggingQueryElSummaryFieldsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.summary_fields", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct LoggingSavedQueryOpsAnalyticsQueryEl {
    sql_query_text: PrimField<String>,
}
impl LoggingSavedQueryOpsAnalyticsQueryEl {}
impl ToListMappable for LoggingSavedQueryOpsAnalyticsQueryEl {
    type O = BlockAssignable<LoggingSavedQueryOpsAnalyticsQueryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLoggingSavedQueryOpsAnalyticsQueryEl {
    #[doc = "A logs analytics SQL query, which generally follows BigQuery format."]
    pub sql_query_text: PrimField<String>,
}
impl BuildLoggingSavedQueryOpsAnalyticsQueryEl {
    pub fn build(self) -> LoggingSavedQueryOpsAnalyticsQueryEl {
        LoggingSavedQueryOpsAnalyticsQueryEl {
            sql_query_text: self.sql_query_text,
        }
    }
}
pub struct LoggingSavedQueryOpsAnalyticsQueryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LoggingSavedQueryOpsAnalyticsQueryElRef {
    fn new(shared: StackShared, base: String) -> LoggingSavedQueryOpsAnalyticsQueryElRef {
        LoggingSavedQueryOpsAnalyticsQueryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LoggingSavedQueryOpsAnalyticsQueryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `sql_query_text` after provisioning.\nA logs analytics SQL query, which generally follows BigQuery format."]
    pub fn sql_query_text(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sql_query_text", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct LoggingSavedQueryTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl LoggingSavedQueryTimeoutsEl {
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
impl ToListMappable for LoggingSavedQueryTimeoutsEl {
    type O = BlockAssignable<LoggingSavedQueryTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLoggingSavedQueryTimeoutsEl {}
impl BuildLoggingSavedQueryTimeoutsEl {
    pub fn build(self) -> LoggingSavedQueryTimeoutsEl {
        LoggingSavedQueryTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct LoggingSavedQueryTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LoggingSavedQueryTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> LoggingSavedQueryTimeoutsElRef {
        LoggingSavedQueryTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LoggingSavedQueryTimeoutsElRef {
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
struct LoggingSavedQueryDynamic {
    logging_query: Option<DynamicBlock<LoggingSavedQueryLoggingQueryEl>>,
    ops_analytics_query: Option<DynamicBlock<LoggingSavedQueryOpsAnalyticsQueryEl>>,
}
