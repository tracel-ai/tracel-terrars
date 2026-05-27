use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataprocBatchData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_config: Option<Vec<DataprocBatchEnvironmentConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pyspark_batch: Option<Vec<DataprocBatchPysparkBatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime_config: Option<Vec<DataprocBatchRuntimeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spark_batch: Option<Vec<DataprocBatchSparkBatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spark_r_batch: Option<Vec<DataprocBatchSparkRBatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spark_sql_batch: Option<Vec<DataprocBatchSparkSqlBatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataprocBatchTimeoutsEl>,
    dynamic: DataprocBatchDynamic,
}
struct DataprocBatch_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataprocBatchData>,
}
#[derive(Clone)]
pub struct DataprocBatch(Rc<DataprocBatch_>);
impl DataprocBatch {
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
    #[doc = "Set the field `batch_id`.\nThe ID to use for the batch, which will become the final component of the batch's resource name.\nThis value must be 4-63 characters. Valid characters are /[a-z][0-9]-/."]
    pub fn set_batch_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().batch_id = Some(v.into());
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
    #[doc = "Set the field `labels`.\nThe labels to associate with this batch.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe location in which the batch will be created in."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `environment_config`.\n"]
    pub fn set_environment_config(
        self,
        v: impl Into<BlockAssignable<DataprocBatchEnvironmentConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().environment_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.environment_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `pyspark_batch`.\n"]
    pub fn set_pyspark_batch(
        self,
        v: impl Into<BlockAssignable<DataprocBatchPysparkBatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pyspark_batch = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pyspark_batch = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `runtime_config`.\n"]
    pub fn set_runtime_config(
        self,
        v: impl Into<BlockAssignable<DataprocBatchRuntimeConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().runtime_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.runtime_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `spark_batch`.\n"]
    pub fn set_spark_batch(self, v: impl Into<BlockAssignable<DataprocBatchSparkBatchEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spark_batch = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spark_batch = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `spark_r_batch`.\n"]
    pub fn set_spark_r_batch(
        self,
        v: impl Into<BlockAssignable<DataprocBatchSparkRBatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spark_r_batch = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spark_r_batch = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `spark_sql_batch`.\n"]
    pub fn set_spark_sql_batch(
        self,
        v: impl Into<BlockAssignable<DataprocBatchSparkSqlBatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spark_sql_batch = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spark_sql_batch = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataprocBatchTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `batch_id` after provisioning.\nThe ID to use for the batch, which will become the final component of the batch's resource name.\nThis value must be 4-63 characters. Valid characters are /[a-z][0-9]-/."]
    pub fn batch_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.batch_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the batch was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creator` after provisioning.\nThe email address of the user who created the batch."]
    pub fn creator(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creator", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels to associate with this batch.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location in which the batch will be created in."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the batch."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `operation` after provisioning.\nThe resource name of the operation associated with this batch."]
    pub fn operation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `runtime_info` after provisioning.\nRuntime information about batch execution."]
    pub fn runtime_info(&self) -> ListRef<DataprocBatchRuntimeInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.runtime_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the batch. For possible values, see the [API documentation](https://cloud.google.com/dataproc-serverless/docs/reference/rest/v1/projects.locations.batches#State)."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_history` after provisioning.\nHistorical state information for the batch."]
    pub fn state_history(&self) -> ListRef<DataprocBatchStateHistoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.state_history", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_message` after provisioning.\nBatch state details, such as a failure description if the state is FAILED."]
    pub fn state_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_message", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_time` after provisioning.\nBatch state details, such as a failure description if the state is FAILED."]
    pub fn state_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nA batch UUID (Unique Universal Identifier). The service generates this value when it creates the batch."]
    pub fn uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.uuid", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environment_config` after provisioning.\n"]
    pub fn environment_config(&self) -> ListRef<DataprocBatchEnvironmentConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.environment_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pyspark_batch` after provisioning.\n"]
    pub fn pyspark_batch(&self) -> ListRef<DataprocBatchPysparkBatchElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pyspark_batch", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `runtime_config` after provisioning.\n"]
    pub fn runtime_config(&self) -> ListRef<DataprocBatchRuntimeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.runtime_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spark_batch` after provisioning.\n"]
    pub fn spark_batch(&self) -> ListRef<DataprocBatchSparkBatchElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spark_batch", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spark_r_batch` after provisioning.\n"]
    pub fn spark_r_batch(&self) -> ListRef<DataprocBatchSparkRBatchElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spark_r_batch", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spark_sql_batch` after provisioning.\n"]
    pub fn spark_sql_batch(&self) -> ListRef<DataprocBatchSparkSqlBatchElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spark_sql_batch", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocBatchTimeoutsElRef {
        DataprocBatchTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataprocBatch {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataprocBatch {}
impl ToListMappable for DataprocBatch {
    type O = ListRef<DataprocBatchRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataprocBatch_ {
    fn extract_resource_type(&self) -> String {
        "google_dataproc_batch".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataprocBatch {
    pub tf_id: String,
}
impl BuildDataprocBatch {
    pub fn build(self, stack: &mut Stack) -> DataprocBatch {
        let out = DataprocBatch(Rc::new(DataprocBatch_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataprocBatchData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                batch_id: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                project: core::default::Default::default(),
                environment_config: core::default::Default::default(),
                pyspark_batch: core::default::Default::default(),
                runtime_config: core::default::Default::default(),
                spark_batch: core::default::Default::default(),
                spark_r_batch: core::default::Default::default(),
                spark_sql_batch: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DataprocBatchRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocBatchRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataprocBatchRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `batch_id` after provisioning.\nThe ID to use for the batch, which will become the final component of the batch's resource name.\nThis value must be 4-63 characters. Valid characters are /[a-z][0-9]-/."]
    pub fn batch_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.batch_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the batch was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creator` after provisioning.\nThe email address of the user who created the batch."]
    pub fn creator(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creator", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels to associate with this batch.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location in which the batch will be created in."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the batch."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `operation` after provisioning.\nThe resource name of the operation associated with this batch."]
    pub fn operation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `runtime_info` after provisioning.\nRuntime information about batch execution."]
    pub fn runtime_info(&self) -> ListRef<DataprocBatchRuntimeInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.runtime_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the batch. For possible values, see the [API documentation](https://cloud.google.com/dataproc-serverless/docs/reference/rest/v1/projects.locations.batches#State)."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_history` after provisioning.\nHistorical state information for the batch."]
    pub fn state_history(&self) -> ListRef<DataprocBatchStateHistoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.state_history", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_message` after provisioning.\nBatch state details, such as a failure description if the state is FAILED."]
    pub fn state_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_message", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_time` after provisioning.\nBatch state details, such as a failure description if the state is FAILED."]
    pub fn state_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nA batch UUID (Unique Universal Identifier). The service generates this value when it creates the batch."]
    pub fn uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.uuid", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environment_config` after provisioning.\n"]
    pub fn environment_config(&self) -> ListRef<DataprocBatchEnvironmentConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.environment_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pyspark_batch` after provisioning.\n"]
    pub fn pyspark_batch(&self) -> ListRef<DataprocBatchPysparkBatchElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pyspark_batch", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `runtime_config` after provisioning.\n"]
    pub fn runtime_config(&self) -> ListRef<DataprocBatchRuntimeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.runtime_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spark_batch` after provisioning.\n"]
    pub fn spark_batch(&self) -> ListRef<DataprocBatchSparkBatchElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spark_batch", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spark_r_batch` after provisioning.\n"]
    pub fn spark_r_batch(&self) -> ListRef<DataprocBatchSparkRBatchElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spark_r_batch", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spark_sql_batch` after provisioning.\n"]
    pub fn spark_sql_batch(&self) -> ListRef<DataprocBatchSparkSqlBatchElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spark_sql_batch", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocBatchTimeoutsElRef {
        DataprocBatchTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocBatchRuntimeInfoElApproximateUsageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    milli_accelerator_seconds: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    milli_dcu_seconds: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shuffle_storage_gb_seconds: Option<PrimField<String>>,
}
impl DataprocBatchRuntimeInfoElApproximateUsageEl {
    #[doc = "Set the field `accelerator_type`.\n"]
    pub fn set_accelerator_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accelerator_type = Some(v.into());
        self
    }
    #[doc = "Set the field `milli_accelerator_seconds`.\n"]
    pub fn set_milli_accelerator_seconds(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.milli_accelerator_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `milli_dcu_seconds`.\n"]
    pub fn set_milli_dcu_seconds(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.milli_dcu_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `shuffle_storage_gb_seconds`.\n"]
    pub fn set_shuffle_storage_gb_seconds(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.shuffle_storage_gb_seconds = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocBatchRuntimeInfoElApproximateUsageEl {
    type O = BlockAssignable<DataprocBatchRuntimeInfoElApproximateUsageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocBatchRuntimeInfoElApproximateUsageEl {}
impl BuildDataprocBatchRuntimeInfoElApproximateUsageEl {
    pub fn build(self) -> DataprocBatchRuntimeInfoElApproximateUsageEl {
        DataprocBatchRuntimeInfoElApproximateUsageEl {
            accelerator_type: core::default::Default::default(),
            milli_accelerator_seconds: core::default::Default::default(),
            milli_dcu_seconds: core::default::Default::default(),
            shuffle_storage_gb_seconds: core::default::Default::default(),
        }
    }
}
pub struct DataprocBatchRuntimeInfoElApproximateUsageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocBatchRuntimeInfoElApproximateUsageElRef {
    fn new(shared: StackShared, base: String) -> DataprocBatchRuntimeInfoElApproximateUsageElRef {
        DataprocBatchRuntimeInfoElApproximateUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocBatchRuntimeInfoElApproximateUsageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator_type` after provisioning.\n"]
    pub fn accelerator_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `milli_accelerator_seconds` after provisioning.\n"]
    pub fn milli_accelerator_seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.milli_accelerator_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `milli_dcu_seconds` after provisioning.\n"]
    pub fn milli_dcu_seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.milli_dcu_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `shuffle_storage_gb_seconds` after provisioning.\n"]
    pub fn shuffle_storage_gb_seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.shuffle_storage_gb_seconds", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocBatchRuntimeInfoElCurrentUsageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    milli_accelerator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    milli_dcu: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    milli_dcu_premium: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shuffle_storage_gb: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shuffle_storage_gb_premium: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_time: Option<PrimField<String>>,
}
impl DataprocBatchRuntimeInfoElCurrentUsageEl {
    #[doc = "Set the field `accelerator_type`.\n"]
    pub fn set_accelerator_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accelerator_type = Some(v.into());
        self
    }
    #[doc = "Set the field `milli_accelerator`.\n"]
    pub fn set_milli_accelerator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.milli_accelerator = Some(v.into());
        self
    }
    #[doc = "Set the field `milli_dcu`.\n"]
    pub fn set_milli_dcu(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.milli_dcu = Some(v.into());
        self
    }
    #[doc = "Set the field `milli_dcu_premium`.\n"]
    pub fn set_milli_dcu_premium(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.milli_dcu_premium = Some(v.into());
        self
    }
    #[doc = "Set the field `shuffle_storage_gb`.\n"]
    pub fn set_shuffle_storage_gb(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.shuffle_storage_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `shuffle_storage_gb_premium`.\n"]
    pub fn set_shuffle_storage_gb_premium(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.shuffle_storage_gb_premium = Some(v.into());
        self
    }
    #[doc = "Set the field `snapshot_time`.\n"]
    pub fn set_snapshot_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.snapshot_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocBatchRuntimeInfoElCurrentUsageEl {
    type O = BlockAssignable<DataprocBatchRuntimeInfoElCurrentUsageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocBatchRuntimeInfoElCurrentUsageEl {}
impl BuildDataprocBatchRuntimeInfoElCurrentUsageEl {
    pub fn build(self) -> DataprocBatchRuntimeInfoElCurrentUsageEl {
        DataprocBatchRuntimeInfoElCurrentUsageEl {
            accelerator_type: core::default::Default::default(),
            milli_accelerator: core::default::Default::default(),
            milli_dcu: core::default::Default::default(),
            milli_dcu_premium: core::default::Default::default(),
            shuffle_storage_gb: core::default::Default::default(),
            shuffle_storage_gb_premium: core::default::Default::default(),
            snapshot_time: core::default::Default::default(),
        }
    }
}
pub struct DataprocBatchRuntimeInfoElCurrentUsageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocBatchRuntimeInfoElCurrentUsageElRef {
    fn new(shared: StackShared, base: String) -> DataprocBatchRuntimeInfoElCurrentUsageElRef {
        DataprocBatchRuntimeInfoElCurrentUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocBatchRuntimeInfoElCurrentUsageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator_type` after provisioning.\n"]
    pub fn accelerator_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `milli_accelerator` after provisioning.\n"]
    pub fn milli_accelerator(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.milli_accelerator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `milli_dcu` after provisioning.\n"]
    pub fn milli_dcu(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.milli_dcu", self.base))
    }
    #[doc = "Get a reference to the value of field `milli_dcu_premium` after provisioning.\n"]
    pub fn milli_dcu_premium(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.milli_dcu_premium", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `shuffle_storage_gb` after provisioning.\n"]
    pub fn shuffle_storage_gb(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.shuffle_storage_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `shuffle_storage_gb_premium` after provisioning.\n"]
    pub fn shuffle_storage_gb_premium(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.shuffle_storage_gb_premium", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_time` after provisioning.\n"]
    pub fn snapshot_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot_time", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocBatchRuntimeInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    approximate_usage: Option<ListField<DataprocBatchRuntimeInfoElApproximateUsageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    current_usage: Option<ListField<DataprocBatchRuntimeInfoElCurrentUsageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    diagnostic_output_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoints: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_uri: Option<PrimField<String>>,
}
impl DataprocBatchRuntimeInfoEl {
    #[doc = "Set the field `approximate_usage`.\n"]
    pub fn set_approximate_usage(
        mut self,
        v: impl Into<ListField<DataprocBatchRuntimeInfoElApproximateUsageEl>>,
    ) -> Self {
        self.approximate_usage = Some(v.into());
        self
    }
    #[doc = "Set the field `current_usage`.\n"]
    pub fn set_current_usage(
        mut self,
        v: impl Into<ListField<DataprocBatchRuntimeInfoElCurrentUsageEl>>,
    ) -> Self {
        self.current_usage = Some(v.into());
        self
    }
    #[doc = "Set the field `diagnostic_output_uri`.\n"]
    pub fn set_diagnostic_output_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.diagnostic_output_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `endpoints`.\n"]
    pub fn set_endpoints(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.endpoints = Some(v.into());
        self
    }
    #[doc = "Set the field `output_uri`.\n"]
    pub fn set_output_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_uri = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocBatchRuntimeInfoEl {
    type O = BlockAssignable<DataprocBatchRuntimeInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocBatchRuntimeInfoEl {}
impl BuildDataprocBatchRuntimeInfoEl {
    pub fn build(self) -> DataprocBatchRuntimeInfoEl {
        DataprocBatchRuntimeInfoEl {
            approximate_usage: core::default::Default::default(),
            current_usage: core::default::Default::default(),
            diagnostic_output_uri: core::default::Default::default(),
            endpoints: core::default::Default::default(),
            output_uri: core::default::Default::default(),
        }
    }
}
pub struct DataprocBatchRuntimeInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocBatchRuntimeInfoElRef {
    fn new(shared: StackShared, base: String) -> DataprocBatchRuntimeInfoElRef {
        DataprocBatchRuntimeInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocBatchRuntimeInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `approximate_usage` after provisioning.\n"]
    pub fn approximate_usage(&self) -> ListRef<DataprocBatchRuntimeInfoElApproximateUsageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.approximate_usage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `current_usage` after provisioning.\n"]
    pub fn current_usage(&self) -> ListRef<DataprocBatchRuntimeInfoElCurrentUsageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.current_usage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `diagnostic_output_uri` after provisioning.\n"]
    pub fn diagnostic_output_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.diagnostic_output_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.endpoints", self.base))
    }
    #[doc = "Get a reference to the value of field `output_uri` after provisioning.\n"]
    pub fn output_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_uri", self.base))
    }
}
#[derive(Serialize)]
pub struct DataprocBatchStateHistoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state_start_time: Option<PrimField<String>>,
}
impl DataprocBatchStateHistoryEl {
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `state_message`.\n"]
    pub fn set_state_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state_message = Some(v.into());
        self
    }
    #[doc = "Set the field `state_start_time`.\n"]
    pub fn set_state_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state_start_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocBatchStateHistoryEl {
    type O = BlockAssignable<DataprocBatchStateHistoryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocBatchStateHistoryEl {}
impl BuildDataprocBatchStateHistoryEl {
    pub fn build(self) -> DataprocBatchStateHistoryEl {
        DataprocBatchStateHistoryEl {
            state: core::default::Default::default(),
            state_message: core::default::Default::default(),
            state_start_time: core::default::Default::default(),
        }
    }
}
pub struct DataprocBatchStateHistoryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocBatchStateHistoryElRef {
    fn new(shared: StackShared, base: String) -> DataprocBatchStateHistoryElRef {
        DataprocBatchStateHistoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocBatchStateHistoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `state_message` after provisioning.\n"]
    pub fn state_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state_start_time` after provisioning.\n"]
    pub fn state_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_start_time", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocBatchEnvironmentConfigElExecutionConfigElAuthenticationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    user_workload_authentication_type: Option<PrimField<String>>,
}
impl DataprocBatchEnvironmentConfigElExecutionConfigElAuthenticationConfigEl {
    #[doc = "Set the field `user_workload_authentication_type`.\nAuthentication type for the user workload running in containers. Possible values: [\"SERVICE_ACCOUNT\", \"END_USER_CREDENTIALS\"]"]
    pub fn set_user_workload_authentication_type(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.user_workload_authentication_type = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocBatchEnvironmentConfigElExecutionConfigElAuthenticationConfigEl {
    type O =
        BlockAssignable<DataprocBatchEnvironmentConfigElExecutionConfigElAuthenticationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocBatchEnvironmentConfigElExecutionConfigElAuthenticationConfigEl {}
impl BuildDataprocBatchEnvironmentConfigElExecutionConfigElAuthenticationConfigEl {
    pub fn build(self) -> DataprocBatchEnvironmentConfigElExecutionConfigElAuthenticationConfigEl {
        DataprocBatchEnvironmentConfigElExecutionConfigElAuthenticationConfigEl {
            user_workload_authentication_type: core::default::Default::default(),
        }
    }
}
pub struct DataprocBatchEnvironmentConfigElExecutionConfigElAuthenticationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocBatchEnvironmentConfigElExecutionConfigElAuthenticationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocBatchEnvironmentConfigElExecutionConfigElAuthenticationConfigElRef {
        DataprocBatchEnvironmentConfigElExecutionConfigElAuthenticationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocBatchEnvironmentConfigElExecutionConfigElAuthenticationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `user_workload_authentication_type` after provisioning.\nAuthentication type for the user workload running in containers. Possible values: [\"SERVICE_ACCOUNT\", \"END_USER_CREDENTIALS\"]"]
    pub fn user_workload_authentication_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_workload_authentication_type", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataprocBatchEnvironmentConfigElExecutionConfigElDynamic {
    authentication_config: Option<
        DynamicBlock<DataprocBatchEnvironmentConfigElExecutionConfigElAuthenticationConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct DataprocBatchEnvironmentConfigElExecutionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    staging_bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_config:
        Option<Vec<DataprocBatchEnvironmentConfigElExecutionConfigElAuthenticationConfigEl>>,
    dynamic: DataprocBatchEnvironmentConfigElExecutionConfigElDynamic,
}
impl DataprocBatchEnvironmentConfigElExecutionConfigEl {
    #[doc = "Set the field `kms_key`.\nThe Cloud KMS key to use for encryption."]
    pub fn set_kms_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key = Some(v.into());
        self
    }
    #[doc = "Set the field `network_tags`.\nTags used for network traffic control."]
    pub fn set_network_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.network_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `network_uri`.\nNetwork configuration for workload execution."]
    pub fn set_network_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\nService account that used to execute workload."]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `staging_bucket`.\nA Cloud Storage bucket used to stage workload dependencies, config files, and store\nworkload output and other ephemeral data, such as Spark history files. If you do not specify a staging bucket,\nCloud Dataproc will determine a Cloud Storage location according to the region where your workload is running,\nand then create and manage project-level, per-location staging and temporary buckets.\nThis field requires a Cloud Storage bucket name, not a gs://... URI to a Cloud Storage bucket."]
    pub fn set_staging_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.staging_bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork_uri`.\nSubnetwork configuration for workload execution."]
    pub fn set_subnetwork_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `ttl`.\nThe duration after which the workload will be terminated.\nWhen the workload exceeds this duration, it will be unconditionally terminated without waiting for ongoing\nwork to finish. If ttl is not specified for a batch workload, the workload will be allowed to run until it\nexits naturally (or run forever without exiting). If ttl is not specified for an interactive session,\nit defaults to 24 hours. If ttl is not specified for a batch that uses 2.1+ runtime version, it defaults to 4 hours.\nMinimum value is 10 minutes; maximum value is 14 days. If both ttl and idleTtl are specified (for an interactive session),\nthe conditions are treated as OR conditions: the workload will be terminated when it has been idle for idleTtl or\nwhen ttl has been exceeded, whichever occurs first."]
    pub fn set_ttl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `authentication_config`.\n"]
    pub fn set_authentication_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataprocBatchEnvironmentConfigElExecutionConfigElAuthenticationConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authentication_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocBatchEnvironmentConfigElExecutionConfigEl {
    type O = BlockAssignable<DataprocBatchEnvironmentConfigElExecutionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocBatchEnvironmentConfigElExecutionConfigEl {}
impl BuildDataprocBatchEnvironmentConfigElExecutionConfigEl {
    pub fn build(self) -> DataprocBatchEnvironmentConfigElExecutionConfigEl {
        DataprocBatchEnvironmentConfigElExecutionConfigEl {
            kms_key: core::default::Default::default(),
            network_tags: core::default::Default::default(),
            network_uri: core::default::Default::default(),
            service_account: core::default::Default::default(),
            staging_bucket: core::default::Default::default(),
            subnetwork_uri: core::default::Default::default(),
            ttl: core::default::Default::default(),
            authentication_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocBatchEnvironmentConfigElExecutionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocBatchEnvironmentConfigElExecutionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocBatchEnvironmentConfigElExecutionConfigElRef {
        DataprocBatchEnvironmentConfigElExecutionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocBatchEnvironmentConfigElExecutionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nThe Cloud KMS key to use for encryption."]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
    #[doc = "Get a reference to the value of field `network_tags` after provisioning.\nTags used for network traffic control."]
    pub fn network_tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.network_tags", self.base))
    }
    #[doc = "Get a reference to the value of field `network_uri` after provisioning.\nNetwork configuration for workload execution."]
    pub fn network_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nService account that used to execute workload."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `staging_bucket` after provisioning.\nA Cloud Storage bucket used to stage workload dependencies, config files, and store\nworkload output and other ephemeral data, such as Spark history files. If you do not specify a staging bucket,\nCloud Dataproc will determine a Cloud Storage location according to the region where your workload is running,\nand then create and manage project-level, per-location staging and temporary buckets.\nThis field requires a Cloud Storage bucket name, not a gs://... URI to a Cloud Storage bucket."]
    pub fn staging_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.staging_bucket", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork_uri` after provisioning.\nSubnetwork configuration for workload execution."]
    pub fn subnetwork_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnetwork_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\nThe duration after which the workload will be terminated.\nWhen the workload exceeds this duration, it will be unconditionally terminated without waiting for ongoing\nwork to finish. If ttl is not specified for a batch workload, the workload will be allowed to run until it\nexits naturally (or run forever without exiting). If ttl is not specified for an interactive session,\nit defaults to 24 hours. If ttl is not specified for a batch that uses 2.1+ runtime version, it defaults to 4 hours.\nMinimum value is 10 minutes; maximum value is 14 days. If both ttl and idleTtl are specified (for an interactive session),\nthe conditions are treated as OR conditions: the workload will be terminated when it has been idle for idleTtl or\nwhen ttl has been exceeded, whichever occurs first."]
    pub fn ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `authentication_config` after provisioning.\n"]
    pub fn authentication_config(
        &self,
    ) -> ListRef<DataprocBatchEnvironmentConfigElExecutionConfigElAuthenticationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authentication_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocBatchEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataproc_cluster: Option<PrimField<String>>,
}
impl DataprocBatchEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigEl {
    #[doc = "Set the field `dataproc_cluster`.\nResource name of an existing Dataproc Cluster to act as a Spark History Server for the workload."]
    pub fn set_dataproc_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dataproc_cluster = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataprocBatchEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigEl
{
    type O = BlockAssignable<
        DataprocBatchEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocBatchEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigEl {}
impl BuildDataprocBatchEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigEl {
    pub fn build(
        self,
    ) -> DataprocBatchEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigEl {
        DataprocBatchEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigEl {
            dataproc_cluster: core::default::Default::default(),
        }
    }
}
pub struct DataprocBatchEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocBatchEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocBatchEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigElRef {
        DataprocBatchEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocBatchEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataproc_cluster` after provisioning.\nResource name of an existing Dataproc Cluster to act as a Spark History Server for the workload."]
    pub fn dataproc_cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataproc_cluster", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataprocBatchEnvironmentConfigElPeripheralsConfigElDynamic {
    spark_history_server_config: Option<
        DynamicBlock<DataprocBatchEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct DataprocBatchEnvironmentConfigElPeripheralsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metastore_service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spark_history_server_config:
        Option<Vec<DataprocBatchEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigEl>>,
    dynamic: DataprocBatchEnvironmentConfigElPeripheralsConfigElDynamic,
}
impl DataprocBatchEnvironmentConfigElPeripheralsConfigEl {
    #[doc = "Set the field `metastore_service`.\nResource name of an existing Dataproc Metastore service."]
    pub fn set_metastore_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metastore_service = Some(v.into());
        self
    }
    #[doc = "Set the field `spark_history_server_config`.\n"]
    pub fn set_spark_history_server_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataprocBatchEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.spark_history_server_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.spark_history_server_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocBatchEnvironmentConfigElPeripheralsConfigEl {
    type O = BlockAssignable<DataprocBatchEnvironmentConfigElPeripheralsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocBatchEnvironmentConfigElPeripheralsConfigEl {}
impl BuildDataprocBatchEnvironmentConfigElPeripheralsConfigEl {
    pub fn build(self) -> DataprocBatchEnvironmentConfigElPeripheralsConfigEl {
        DataprocBatchEnvironmentConfigElPeripheralsConfigEl {
            metastore_service: core::default::Default::default(),
            spark_history_server_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocBatchEnvironmentConfigElPeripheralsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocBatchEnvironmentConfigElPeripheralsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocBatchEnvironmentConfigElPeripheralsConfigElRef {
        DataprocBatchEnvironmentConfigElPeripheralsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocBatchEnvironmentConfigElPeripheralsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metastore_service` after provisioning.\nResource name of an existing Dataproc Metastore service."]
    pub fn metastore_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metastore_service", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `spark_history_server_config` after provisioning.\n"]
    pub fn spark_history_server_config(
        &self,
    ) -> ListRef<DataprocBatchEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spark_history_server_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataprocBatchEnvironmentConfigElDynamic {
    execution_config: Option<DynamicBlock<DataprocBatchEnvironmentConfigElExecutionConfigEl>>,
    peripherals_config: Option<DynamicBlock<DataprocBatchEnvironmentConfigElPeripheralsConfigEl>>,
}
#[derive(Serialize)]
pub struct DataprocBatchEnvironmentConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_config: Option<Vec<DataprocBatchEnvironmentConfigElExecutionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peripherals_config: Option<Vec<DataprocBatchEnvironmentConfigElPeripheralsConfigEl>>,
    dynamic: DataprocBatchEnvironmentConfigElDynamic,
}
impl DataprocBatchEnvironmentConfigEl {
    #[doc = "Set the field `execution_config`.\n"]
    pub fn set_execution_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocBatchEnvironmentConfigElExecutionConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.execution_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.execution_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `peripherals_config`.\n"]
    pub fn set_peripherals_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocBatchEnvironmentConfigElPeripheralsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.peripherals_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.peripherals_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocBatchEnvironmentConfigEl {
    type O = BlockAssignable<DataprocBatchEnvironmentConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocBatchEnvironmentConfigEl {}
impl BuildDataprocBatchEnvironmentConfigEl {
    pub fn build(self) -> DataprocBatchEnvironmentConfigEl {
        DataprocBatchEnvironmentConfigEl {
            execution_config: core::default::Default::default(),
            peripherals_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocBatchEnvironmentConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocBatchEnvironmentConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocBatchEnvironmentConfigElRef {
        DataprocBatchEnvironmentConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocBatchEnvironmentConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `execution_config` after provisioning.\n"]
    pub fn execution_config(
        &self,
    ) -> ListRef<DataprocBatchEnvironmentConfigElExecutionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.execution_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `peripherals_config` after provisioning.\n"]
    pub fn peripherals_config(
        &self,
    ) -> ListRef<DataprocBatchEnvironmentConfigElPeripheralsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.peripherals_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocBatchPysparkBatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    archive_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jar_file_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    main_python_file_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_file_uris: Option<ListField<PrimField<String>>>,
}
impl DataprocBatchPysparkBatchEl {
    #[doc = "Set the field `archive_uris`.\nHCFS URIs of archives to be extracted into the working directory of each executor.\nSupported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn set_archive_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.archive_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `args`.\nThe arguments to pass to the driver. Do not include arguments that can be set as batch\nproperties, such as --conf, since a collision can occur that causes an incorrect batch submission."]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }
    #[doc = "Set the field `file_uris`.\nHCFS URIs of files to be placed in the working directory of each executor."]
    pub fn set_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.file_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `jar_file_uris`.\nHCFS URIs of jar files to add to the classpath of the Spark driver and tasks."]
    pub fn set_jar_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.jar_file_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `main_python_file_uri`.\nThe HCFS URI of the main Python file to use as the Spark driver. Must be a .py file."]
    pub fn set_main_python_file_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.main_python_file_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `python_file_uris`.\nHCFS file URIs of Python files to pass to the PySpark framework.\nSupported file types: .py, .egg, and .zip."]
    pub fn set_python_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.python_file_uris = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocBatchPysparkBatchEl {
    type O = BlockAssignable<DataprocBatchPysparkBatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocBatchPysparkBatchEl {}
impl BuildDataprocBatchPysparkBatchEl {
    pub fn build(self) -> DataprocBatchPysparkBatchEl {
        DataprocBatchPysparkBatchEl {
            archive_uris: core::default::Default::default(),
            args: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            jar_file_uris: core::default::Default::default(),
            main_python_file_uri: core::default::Default::default(),
            python_file_uris: core::default::Default::default(),
        }
    }
}
pub struct DataprocBatchPysparkBatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocBatchPysparkBatchElRef {
    fn new(shared: StackShared, base: String) -> DataprocBatchPysparkBatchElRef {
        DataprocBatchPysparkBatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocBatchPysparkBatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `archive_uris` after provisioning.\nHCFS URIs of archives to be extracted into the working directory of each executor.\nSupported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn archive_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\nThe arguments to pass to the driver. Do not include arguments that can be set as batch\nproperties, such as --conf, since a collision can occur that causes an incorrect batch submission."]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }
    #[doc = "Get a reference to the value of field `file_uris` after provisioning.\nHCFS URIs of files to be placed in the working directory of each executor."]
    pub fn file_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.file_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `jar_file_uris` after provisioning.\nHCFS URIs of jar files to add to the classpath of the Spark driver and tasks."]
    pub fn jar_file_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.jar_file_uris", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `main_python_file_uri` after provisioning.\nThe HCFS URI of the main Python file to use as the Spark driver. Must be a .py file."]
    pub fn main_python_file_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.main_python_file_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `python_file_uris` after provisioning.\nHCFS file URIs of Python files to pass to the PySpark framework.\nSupported file types: .py, .egg, and .zip."]
    pub fn python_file_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.python_file_uris", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocBatchRuntimeConfigElAutotuningConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    scenarios: Option<ListField<PrimField<String>>>,
}
impl DataprocBatchRuntimeConfigElAutotuningConfigEl {
    #[doc = "Set the field `scenarios`.\nOptional. Scenarios for which tunings are applied. Possible values: [\"AUTO\", \"SCALING\", \"BROADCAST_HASH_JOIN\", \"MEMORY\"]"]
    pub fn set_scenarios(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.scenarios = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocBatchRuntimeConfigElAutotuningConfigEl {
    type O = BlockAssignable<DataprocBatchRuntimeConfigElAutotuningConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocBatchRuntimeConfigElAutotuningConfigEl {}
impl BuildDataprocBatchRuntimeConfigElAutotuningConfigEl {
    pub fn build(self) -> DataprocBatchRuntimeConfigElAutotuningConfigEl {
        DataprocBatchRuntimeConfigElAutotuningConfigEl {
            scenarios: core::default::Default::default(),
        }
    }
}
pub struct DataprocBatchRuntimeConfigElAutotuningConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocBatchRuntimeConfigElAutotuningConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocBatchRuntimeConfigElAutotuningConfigElRef {
        DataprocBatchRuntimeConfigElAutotuningConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocBatchRuntimeConfigElAutotuningConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scenarios` after provisioning.\nOptional. Scenarios for which tunings are applied. Possible values: [\"AUTO\", \"SCALING\", \"BROADCAST_HASH_JOIN\", \"MEMORY\"]"]
    pub fn scenarios(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.scenarios", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataprocBatchRuntimeConfigElDynamic {
    autotuning_config: Option<DynamicBlock<DataprocBatchRuntimeConfigElAutotuningConfigEl>>,
}
#[derive(Serialize)]
pub struct DataprocBatchRuntimeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cohort: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autotuning_config: Option<Vec<DataprocBatchRuntimeConfigElAutotuningConfigEl>>,
    dynamic: DataprocBatchRuntimeConfigElDynamic,
}
impl DataprocBatchRuntimeConfigEl {
    #[doc = "Set the field `cohort`.\nOptional. Cohort identifier. Identifies families of the workloads having the same shape, e.g. daily ETL jobs."]
    pub fn set_cohort(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cohort = Some(v.into());
        self
    }
    #[doc = "Set the field `container_image`.\nOptional custom container image for the job runtime environment. If not specified, a default container image will be used."]
    pub fn set_container_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container_image = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\nA mapping of property names to values, which are used to configure workload execution."]
    pub fn set_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.properties = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\nVersion of the batch runtime."]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
    #[doc = "Set the field `autotuning_config`.\n"]
    pub fn set_autotuning_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocBatchRuntimeConfigElAutotuningConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.autotuning_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.autotuning_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocBatchRuntimeConfigEl {
    type O = BlockAssignable<DataprocBatchRuntimeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocBatchRuntimeConfigEl {}
impl BuildDataprocBatchRuntimeConfigEl {
    pub fn build(self) -> DataprocBatchRuntimeConfigEl {
        DataprocBatchRuntimeConfigEl {
            cohort: core::default::Default::default(),
            container_image: core::default::Default::default(),
            properties: core::default::Default::default(),
            version: core::default::Default::default(),
            autotuning_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocBatchRuntimeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocBatchRuntimeConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocBatchRuntimeConfigElRef {
        DataprocBatchRuntimeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocBatchRuntimeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cohort` after provisioning.\nOptional. Cohort identifier. Identifies families of the workloads having the same shape, e.g. daily ETL jobs."]
    pub fn cohort(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cohort", self.base))
    }
    #[doc = "Get a reference to the value of field `container_image` after provisioning.\nOptional custom container image for the job runtime environment. If not specified, a default container image will be used."]
    pub fn container_image(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_image", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `effective_properties` after provisioning.\nA mapping of property names to values, which are used to configure workload execution."]
    pub fn effective_properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_properties", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\nA mapping of property names to values, which are used to configure workload execution."]
    pub fn properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion of the batch runtime."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
    #[doc = "Get a reference to the value of field `autotuning_config` after provisioning.\n"]
    pub fn autotuning_config(&self) -> ListRef<DataprocBatchRuntimeConfigElAutotuningConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autotuning_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocBatchSparkBatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    archive_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jar_file_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    main_class: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    main_jar_file_uri: Option<PrimField<String>>,
}
impl DataprocBatchSparkBatchEl {
    #[doc = "Set the field `archive_uris`.\nHCFS URIs of archives to be extracted into the working directory of each executor.\nSupported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn set_archive_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.archive_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `args`.\nThe arguments to pass to the driver. Do not include arguments that can be set as batch\nproperties, such as --conf, since a collision can occur that causes an incorrect batch submission."]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }
    #[doc = "Set the field `file_uris`.\nHCFS URIs of files to be placed in the working directory of each executor."]
    pub fn set_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.file_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `jar_file_uris`.\nHCFS URIs of jar files to add to the classpath of the Spark driver and tasks."]
    pub fn set_jar_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.jar_file_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `main_class`.\nThe name of the driver main class. The jar file that contains the class must be in the\nclasspath or specified in jarFileUris."]
    pub fn set_main_class(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.main_class = Some(v.into());
        self
    }
    #[doc = "Set the field `main_jar_file_uri`.\nThe HCFS URI of the jar file that contains the main class."]
    pub fn set_main_jar_file_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.main_jar_file_uri = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocBatchSparkBatchEl {
    type O = BlockAssignable<DataprocBatchSparkBatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocBatchSparkBatchEl {}
impl BuildDataprocBatchSparkBatchEl {
    pub fn build(self) -> DataprocBatchSparkBatchEl {
        DataprocBatchSparkBatchEl {
            archive_uris: core::default::Default::default(),
            args: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            jar_file_uris: core::default::Default::default(),
            main_class: core::default::Default::default(),
            main_jar_file_uri: core::default::Default::default(),
        }
    }
}
pub struct DataprocBatchSparkBatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocBatchSparkBatchElRef {
    fn new(shared: StackShared, base: String) -> DataprocBatchSparkBatchElRef {
        DataprocBatchSparkBatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocBatchSparkBatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `archive_uris` after provisioning.\nHCFS URIs of archives to be extracted into the working directory of each executor.\nSupported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn archive_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\nThe arguments to pass to the driver. Do not include arguments that can be set as batch\nproperties, such as --conf, since a collision can occur that causes an incorrect batch submission."]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }
    #[doc = "Get a reference to the value of field `file_uris` after provisioning.\nHCFS URIs of files to be placed in the working directory of each executor."]
    pub fn file_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.file_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `jar_file_uris` after provisioning.\nHCFS URIs of jar files to add to the classpath of the Spark driver and tasks."]
    pub fn jar_file_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.jar_file_uris", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `main_class` after provisioning.\nThe name of the driver main class. The jar file that contains the class must be in the\nclasspath or specified in jarFileUris."]
    pub fn main_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.main_class", self.base))
    }
    #[doc = "Get a reference to the value of field `main_jar_file_uri` after provisioning.\nThe HCFS URI of the jar file that contains the main class."]
    pub fn main_jar_file_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.main_jar_file_uri", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocBatchSparkRBatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    archive_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    main_r_file_uri: Option<PrimField<String>>,
}
impl DataprocBatchSparkRBatchEl {
    #[doc = "Set the field `archive_uris`.\nHCFS URIs of archives to be extracted into the working directory of each executor.\nSupported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn set_archive_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.archive_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `args`.\nThe arguments to pass to the driver. Do not include arguments that can be set as batch\nproperties, such as --conf, since a collision can occur that causes an incorrect batch submission."]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }
    #[doc = "Set the field `file_uris`.\nHCFS URIs of files to be placed in the working directory of each executor."]
    pub fn set_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.file_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `main_r_file_uri`.\nThe HCFS URI of the main R file to use as the driver. Must be a .R or .r file."]
    pub fn set_main_r_file_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.main_r_file_uri = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocBatchSparkRBatchEl {
    type O = BlockAssignable<DataprocBatchSparkRBatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocBatchSparkRBatchEl {}
impl BuildDataprocBatchSparkRBatchEl {
    pub fn build(self) -> DataprocBatchSparkRBatchEl {
        DataprocBatchSparkRBatchEl {
            archive_uris: core::default::Default::default(),
            args: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            main_r_file_uri: core::default::Default::default(),
        }
    }
}
pub struct DataprocBatchSparkRBatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocBatchSparkRBatchElRef {
    fn new(shared: StackShared, base: String) -> DataprocBatchSparkRBatchElRef {
        DataprocBatchSparkRBatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocBatchSparkRBatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `archive_uris` after provisioning.\nHCFS URIs of archives to be extracted into the working directory of each executor.\nSupported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn archive_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\nThe arguments to pass to the driver. Do not include arguments that can be set as batch\nproperties, such as --conf, since a collision can occur that causes an incorrect batch submission."]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }
    #[doc = "Get a reference to the value of field `file_uris` after provisioning.\nHCFS URIs of files to be placed in the working directory of each executor."]
    pub fn file_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.file_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `main_r_file_uri` after provisioning.\nThe HCFS URI of the main R file to use as the driver. Must be a .R or .r file."]
    pub fn main_r_file_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.main_r_file_uri", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocBatchSparkSqlBatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    jar_file_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_file_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_variables: Option<RecField<PrimField<String>>>,
}
impl DataprocBatchSparkSqlBatchEl {
    #[doc = "Set the field `jar_file_uris`.\nHCFS URIs of jar files to be added to the Spark CLASSPATH."]
    pub fn set_jar_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.jar_file_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `query_file_uri`.\nThe HCFS URI of the script that contains Spark SQL queries to execute."]
    pub fn set_query_file_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.query_file_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `query_variables`.\nMapping of query variable names to values (equivalent to the Spark SQL command: SET name=\"value\";)."]
    pub fn set_query_variables(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.query_variables = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocBatchSparkSqlBatchEl {
    type O = BlockAssignable<DataprocBatchSparkSqlBatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocBatchSparkSqlBatchEl {}
impl BuildDataprocBatchSparkSqlBatchEl {
    pub fn build(self) -> DataprocBatchSparkSqlBatchEl {
        DataprocBatchSparkSqlBatchEl {
            jar_file_uris: core::default::Default::default(),
            query_file_uri: core::default::Default::default(),
            query_variables: core::default::Default::default(),
        }
    }
}
pub struct DataprocBatchSparkSqlBatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocBatchSparkSqlBatchElRef {
    fn new(shared: StackShared, base: String) -> DataprocBatchSparkSqlBatchElRef {
        DataprocBatchSparkSqlBatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocBatchSparkSqlBatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `jar_file_uris` after provisioning.\nHCFS URIs of jar files to be added to the Spark CLASSPATH."]
    pub fn jar_file_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.jar_file_uris", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query_file_uri` after provisioning.\nThe HCFS URI of the script that contains Spark SQL queries to execute."]
    pub fn query_file_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_file_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query_variables` after provisioning.\nMapping of query variable names to values (equivalent to the Spark SQL command: SET name=\"value\";)."]
    pub fn query_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.query_variables", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocBatchTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DataprocBatchTimeoutsEl {
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
impl ToListMappable for DataprocBatchTimeoutsEl {
    type O = BlockAssignable<DataprocBatchTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocBatchTimeoutsEl {}
impl BuildDataprocBatchTimeoutsEl {
    pub fn build(self) -> DataprocBatchTimeoutsEl {
        DataprocBatchTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DataprocBatchTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocBatchTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataprocBatchTimeoutsElRef {
        DataprocBatchTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocBatchTimeoutsElRef {
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
struct DataprocBatchDynamic {
    environment_config: Option<DynamicBlock<DataprocBatchEnvironmentConfigEl>>,
    pyspark_batch: Option<DynamicBlock<DataprocBatchPysparkBatchEl>>,
    runtime_config: Option<DynamicBlock<DataprocBatchRuntimeConfigEl>>,
    spark_batch: Option<DynamicBlock<DataprocBatchSparkBatchEl>>,
    spark_r_batch: Option<DynamicBlock<DataprocBatchSparkRBatchEl>>,
    spark_sql_batch: Option<DynamicBlock<DataprocBatchSparkSqlBatchEl>>,
}
