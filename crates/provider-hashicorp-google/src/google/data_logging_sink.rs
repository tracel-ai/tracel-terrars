use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataLoggingSinkData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    id: PrimField<String>,
}
struct DataLoggingSink_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataLoggingSinkData>,
}
#[derive(Clone)]
pub struct DataLoggingSink(Rc<DataLoggingSink_>);
impl DataLoggingSink {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Get a reference to the value of field `bigquery_options` after provisioning.\nOptions that affect sinks exporting data to BigQuery."]
    pub fn bigquery_options(&self) -> ListRef<DataLoggingSinkBigqueryOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigquery_options", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `exclusions` after provisioning.\nLog entries that match any of the exclusion filters will not be exported. If a log entry is matched by both filter and one of exclusion's filters, it will not be exported."]
    pub fn exclusions(&self) -> ListRef<DataLoggingSinkExclusionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclusions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nThe filter to apply when exporting logs. Only log entries that match the filter are exported."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nRequired. An identifier for the resource in format: \"projects/[PROJECT_ID]/sinks/[SINK_NAME]\", \"organizations/[ORGANIZATION_ID]/sinks/[SINK_NAME]\", \"billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_NAME]\", \"folders/[FOLDER_ID]/sinks/[SINK_NAME]\""]
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
    #[doc = "Get a reference to the value of field `writer_identity` after provisioning.\nThe identity associated with this sink. This identity must be granted write access to the configured destination."]
    pub fn writer_identity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.writer_identity", self.extract_ref()),
        )
    }
}
impl Referable for DataLoggingSink {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataLoggingSink {}
impl ToListMappable for DataLoggingSink {
    type O = ListRef<DataLoggingSinkRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataLoggingSink_ {
    fn extract_datasource_type(&self) -> String {
        "google_logging_sink".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataLoggingSink {
    pub tf_id: String,
    #[doc = "Required. An identifier for the resource in format: \"projects/[PROJECT_ID]/sinks/[SINK_NAME]\", \"organizations/[ORGANIZATION_ID]/sinks/[SINK_NAME]\", \"billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_NAME]\", \"folders/[FOLDER_ID]/sinks/[SINK_NAME]\""]
    pub id: PrimField<String>,
}
impl BuildDataLoggingSink {
    pub fn build(self, stack: &mut Stack) -> DataLoggingSink {
        let out = DataLoggingSink(Rc::new(DataLoggingSink_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataLoggingSinkData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: self.id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataLoggingSinkRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLoggingSinkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataLoggingSinkRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `bigquery_options` after provisioning.\nOptions that affect sinks exporting data to BigQuery."]
    pub fn bigquery_options(&self) -> ListRef<DataLoggingSinkBigqueryOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigquery_options", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `exclusions` after provisioning.\nLog entries that match any of the exclusion filters will not be exported. If a log entry is matched by both filter and one of exclusion's filters, it will not be exported."]
    pub fn exclusions(&self) -> ListRef<DataLoggingSinkExclusionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclusions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nThe filter to apply when exporting logs. Only log entries that match the filter are exported."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nRequired. An identifier for the resource in format: \"projects/[PROJECT_ID]/sinks/[SINK_NAME]\", \"organizations/[ORGANIZATION_ID]/sinks/[SINK_NAME]\", \"billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_NAME]\", \"folders/[FOLDER_ID]/sinks/[SINK_NAME]\""]
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
    #[doc = "Get a reference to the value of field `writer_identity` after provisioning.\nThe identity associated with this sink. This identity must be granted write access to the configured destination."]
    pub fn writer_identity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.writer_identity", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataLoggingSinkBigqueryOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    use_partitioned_tables: Option<PrimField<bool>>,
}
impl DataLoggingSinkBigqueryOptionsEl {
    #[doc = "Set the field `use_partitioned_tables`.\n"]
    pub fn set_use_partitioned_tables(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_partitioned_tables = Some(v.into());
        self
    }
}
impl ToListMappable for DataLoggingSinkBigqueryOptionsEl {
    type O = BlockAssignable<DataLoggingSinkBigqueryOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLoggingSinkBigqueryOptionsEl {}
impl BuildDataLoggingSinkBigqueryOptionsEl {
    pub fn build(self) -> DataLoggingSinkBigqueryOptionsEl {
        DataLoggingSinkBigqueryOptionsEl {
            use_partitioned_tables: core::default::Default::default(),
        }
    }
}
pub struct DataLoggingSinkBigqueryOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLoggingSinkBigqueryOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataLoggingSinkBigqueryOptionsElRef {
        DataLoggingSinkBigqueryOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLoggingSinkBigqueryOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `use_partitioned_tables` after provisioning.\n"]
    pub fn use_partitioned_tables(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_partitioned_tables", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLoggingSinkExclusionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataLoggingSinkExclusionsEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DataLoggingSinkExclusionsEl {
    type O = BlockAssignable<DataLoggingSinkExclusionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLoggingSinkExclusionsEl {}
impl BuildDataLoggingSinkExclusionsEl {
    pub fn build(self) -> DataLoggingSinkExclusionsEl {
        DataLoggingSinkExclusionsEl {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            filter: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataLoggingSinkExclusionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLoggingSinkExclusionsElRef {
    fn new(shared: StackShared, base: String) -> DataLoggingSinkExclusionsElRef {
        DataLoggingSinkExclusionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLoggingSinkExclusionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
