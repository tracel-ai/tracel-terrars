use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataprocJobData {
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
    force_delete: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for_completion: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hadoop_config: Option<Vec<DataprocJobHadoopConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hive_config: Option<Vec<DataprocJobHiveConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pig_config: Option<Vec<DataprocJobPigConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement: Option<Vec<DataprocJobPlacementEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    presto_config: Option<Vec<DataprocJobPrestoConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pyspark_config: Option<Vec<DataprocJobPysparkConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference: Option<Vec<DataprocJobReferenceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduling: Option<Vec<DataprocJobSchedulingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spark_config: Option<Vec<DataprocJobSparkConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sparksql_config: Option<Vec<DataprocJobSparksqlConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataprocJobTimeoutsEl>,
    dynamic: DataprocJobDynamic,
}
struct DataprocJob_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataprocJobData>,
}
#[derive(Clone)]
pub struct DataprocJob(Rc<DataprocJob_>);
impl DataprocJob {
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
    #[doc = "Set the field `force_delete`.\nBy default, you can only delete inactive jobs within Dataproc. Setting this to true, and calling destroy, will ensure that the job is first cancelled before issuing the delete."]
    pub fn set_force_delete(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_delete = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nOptional. The labels to associate with this job.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe project in which the cluster can be found and jobs subsequently run against. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe Cloud Dataproc region. This essentially determines which clusters are available for this job to be submitted to. If not specified, defaults to global."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `wait_for_completion`.\nIf set to true, Terraform will wait for the job to reach a terminal state (DONE, ERROR, CANCELLED, ATTEMPT_FAILURE). Otherwise, Terraform will consider the job 'created' once it is in the RUNNING state."]
    pub fn set_wait_for_completion(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_completion = Some(v.into());
        self
    }
    #[doc = "Set the field `hadoop_config`.\n"]
    pub fn set_hadoop_config(
        self,
        v: impl Into<BlockAssignable<DataprocJobHadoopConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().hadoop_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.hadoop_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `hive_config`.\n"]
    pub fn set_hive_config(self, v: impl Into<BlockAssignable<DataprocJobHiveConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().hive_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.hive_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `pig_config`.\n"]
    pub fn set_pig_config(self, v: impl Into<BlockAssignable<DataprocJobPigConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pig_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pig_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `placement`.\n"]
    pub fn set_placement(self, v: impl Into<BlockAssignable<DataprocJobPlacementEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().placement = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.placement = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `presto_config`.\n"]
    pub fn set_presto_config(
        self,
        v: impl Into<BlockAssignable<DataprocJobPrestoConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().presto_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.presto_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `pyspark_config`.\n"]
    pub fn set_pyspark_config(
        self,
        v: impl Into<BlockAssignable<DataprocJobPysparkConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pyspark_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pyspark_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `reference`.\n"]
    pub fn set_reference(self, v: impl Into<BlockAssignable<DataprocJobReferenceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().reference = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.reference = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scheduling`.\n"]
    pub fn set_scheduling(self, v: impl Into<BlockAssignable<DataprocJobSchedulingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scheduling = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scheduling = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `spark_config`.\n"]
    pub fn set_spark_config(self, v: impl Into<BlockAssignable<DataprocJobSparkConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spark_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spark_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sparksql_config`.\n"]
    pub fn set_sparksql_config(
        self,
        v: impl Into<BlockAssignable<DataprocJobSparksqlConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sparksql_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sparksql_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataprocJobTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `driver_controls_files_uri` after provisioning.\nOutput-only. If present, the location of miscellaneous control files which may be used as part of job setup and handling. If not present, control files may be placed in the same location as driver_output_uri."]
    pub fn driver_controls_files_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.driver_controls_files_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `driver_output_resource_uri` after provisioning.\nOutput-only. A URI pointing to the location of the stdout of the job's driver program"]
    pub fn driver_output_resource_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.driver_output_resource_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_delete` after provisioning.\nBy default, you can only delete inactive jobs within Dataproc. Setting this to true, and calling destroy, will ensure that the job is first cancelled before issuing the delete."]
    pub fn force_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_delete", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. The labels to associate with this job.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project in which the cluster can be found and jobs subsequently run against. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe Cloud Dataproc region. This essentially determines which clusters are available for this job to be submitted to. If not specified, defaults to global."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the job."]
    pub fn status(&self) -> ListRef<DataprocJobStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `wait_for_completion` after provisioning.\nIf set to true, Terraform will wait for the job to reach a terminal state (DONE, ERROR, CANCELLED, ATTEMPT_FAILURE). Otherwise, Terraform will consider the job 'created' once it is in the RUNNING state."]
    pub fn wait_for_completion(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_completion", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hadoop_config` after provisioning.\n"]
    pub fn hadoop_config(&self) -> ListRef<DataprocJobHadoopConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hadoop_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hive_config` after provisioning.\n"]
    pub fn hive_config(&self) -> ListRef<DataprocJobHiveConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hive_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pig_config` after provisioning.\n"]
    pub fn pig_config(&self) -> ListRef<DataprocJobPigConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pig_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `placement` after provisioning.\n"]
    pub fn placement(&self) -> ListRef<DataprocJobPlacementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.placement", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `presto_config` after provisioning.\n"]
    pub fn presto_config(&self) -> ListRef<DataprocJobPrestoConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.presto_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pyspark_config` after provisioning.\n"]
    pub fn pyspark_config(&self) -> ListRef<DataprocJobPysparkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pyspark_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reference` after provisioning.\n"]
    pub fn reference(&self) -> ListRef<DataprocJobReferenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reference", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scheduling` after provisioning.\n"]
    pub fn scheduling(&self) -> ListRef<DataprocJobSchedulingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scheduling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spark_config` after provisioning.\n"]
    pub fn spark_config(&self) -> ListRef<DataprocJobSparkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spark_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sparksql_config` after provisioning.\n"]
    pub fn sparksql_config(&self) -> ListRef<DataprocJobSparksqlConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sparksql_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocJobTimeoutsElRef {
        DataprocJobTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataprocJob {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataprocJob {}
impl ToListMappable for DataprocJob {
    type O = ListRef<DataprocJobRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataprocJob_ {
    fn extract_resource_type(&self) -> String {
        "google_dataproc_job".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataprocJob {
    pub tf_id: String,
}
impl BuildDataprocJob {
    pub fn build(self, stack: &mut Stack) -> DataprocJob {
        let out = DataprocJob(Rc::new(DataprocJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataprocJobData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                force_delete: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                wait_for_completion: core::default::Default::default(),
                hadoop_config: core::default::Default::default(),
                hive_config: core::default::Default::default(),
                pig_config: core::default::Default::default(),
                placement: core::default::Default::default(),
                presto_config: core::default::Default::default(),
                pyspark_config: core::default::Default::default(),
                reference: core::default::Default::default(),
                scheduling: core::default::Default::default(),
                spark_config: core::default::Default::default(),
                sparksql_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DataprocJobRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocJobRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataprocJobRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `driver_controls_files_uri` after provisioning.\nOutput-only. If present, the location of miscellaneous control files which may be used as part of job setup and handling. If not present, control files may be placed in the same location as driver_output_uri."]
    pub fn driver_controls_files_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.driver_controls_files_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `driver_output_resource_uri` after provisioning.\nOutput-only. A URI pointing to the location of the stdout of the job's driver program"]
    pub fn driver_output_resource_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.driver_output_resource_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_delete` after provisioning.\nBy default, you can only delete inactive jobs within Dataproc. Setting this to true, and calling destroy, will ensure that the job is first cancelled before issuing the delete."]
    pub fn force_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_delete", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. The labels to associate with this job.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project in which the cluster can be found and jobs subsequently run against. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe Cloud Dataproc region. This essentially determines which clusters are available for this job to be submitted to. If not specified, defaults to global."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the job."]
    pub fn status(&self) -> ListRef<DataprocJobStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `wait_for_completion` after provisioning.\nIf set to true, Terraform will wait for the job to reach a terminal state (DONE, ERROR, CANCELLED, ATTEMPT_FAILURE). Otherwise, Terraform will consider the job 'created' once it is in the RUNNING state."]
    pub fn wait_for_completion(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_completion", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hadoop_config` after provisioning.\n"]
    pub fn hadoop_config(&self) -> ListRef<DataprocJobHadoopConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hadoop_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hive_config` after provisioning.\n"]
    pub fn hive_config(&self) -> ListRef<DataprocJobHiveConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hive_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pig_config` after provisioning.\n"]
    pub fn pig_config(&self) -> ListRef<DataprocJobPigConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pig_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `placement` after provisioning.\n"]
    pub fn placement(&self) -> ListRef<DataprocJobPlacementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.placement", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `presto_config` after provisioning.\n"]
    pub fn presto_config(&self) -> ListRef<DataprocJobPrestoConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.presto_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pyspark_config` after provisioning.\n"]
    pub fn pyspark_config(&self) -> ListRef<DataprocJobPysparkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pyspark_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reference` after provisioning.\n"]
    pub fn reference(&self) -> ListRef<DataprocJobReferenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reference", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scheduling` after provisioning.\n"]
    pub fn scheduling(&self) -> ListRef<DataprocJobSchedulingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scheduling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spark_config` after provisioning.\n"]
    pub fn spark_config(&self) -> ListRef<DataprocJobSparkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spark_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sparksql_config` after provisioning.\n"]
    pub fn sparksql_config(&self) -> ListRef<DataprocJobSparksqlConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sparksql_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocJobTimeoutsElRef {
        DataprocJobTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocJobStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state_start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    substate: Option<PrimField<String>>,
}
impl DataprocJobStatusEl {
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.details = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `state_start_time`.\n"]
    pub fn set_state_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state_start_time = Some(v.into());
        self
    }
    #[doc = "Set the field `substate`.\n"]
    pub fn set_substate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.substate = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocJobStatusEl {
    type O = BlockAssignable<DataprocJobStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocJobStatusEl {}
impl BuildDataprocJobStatusEl {
    pub fn build(self) -> DataprocJobStatusEl {
        DataprocJobStatusEl {
            details: core::default::Default::default(),
            state: core::default::Default::default(),
            state_start_time: core::default::Default::default(),
            substate: core::default::Default::default(),
        }
    }
}
pub struct DataprocJobStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocJobStatusElRef {
    fn new(shared: StackShared, base: String) -> DataprocJobStatusElRef {
        DataprocJobStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocJobStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.details", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `state_start_time` after provisioning.\n"]
    pub fn state_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_start_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `substate` after provisioning.\n"]
    pub fn substate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.substate", self.base))
    }
}
#[derive(Serialize)]
pub struct DataprocJobHadoopConfigElLoggingConfigEl {
    driver_log_levels: RecField<PrimField<String>>,
}
impl DataprocJobHadoopConfigElLoggingConfigEl {}
impl ToListMappable for DataprocJobHadoopConfigElLoggingConfigEl {
    type O = BlockAssignable<DataprocJobHadoopConfigElLoggingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocJobHadoopConfigElLoggingConfigEl {
    #[doc = "Optional. The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'."]
    pub driver_log_levels: RecField<PrimField<String>>,
}
impl BuildDataprocJobHadoopConfigElLoggingConfigEl {
    pub fn build(self) -> DataprocJobHadoopConfigElLoggingConfigEl {
        DataprocJobHadoopConfigElLoggingConfigEl {
            driver_log_levels: self.driver_log_levels,
        }
    }
}
pub struct DataprocJobHadoopConfigElLoggingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocJobHadoopConfigElLoggingConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocJobHadoopConfigElLoggingConfigElRef {
        DataprocJobHadoopConfigElLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocJobHadoopConfigElLoggingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `driver_log_levels` after provisioning.\nOptional. The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'."]
    pub fn driver_log_levels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.driver_log_levels", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataprocJobHadoopConfigElDynamic {
    logging_config: Option<DynamicBlock<DataprocJobHadoopConfigElLoggingConfigEl>>,
}
#[derive(Serialize)]
pub struct DataprocJobHadoopConfigEl {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_config: Option<Vec<DataprocJobHadoopConfigElLoggingConfigEl>>,
    dynamic: DataprocJobHadoopConfigElDynamic,
}
impl DataprocJobHadoopConfigEl {
    #[doc = "Set the field `archive_uris`.\nHCFS URIs of archives to be extracted in the working directory of .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn set_archive_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.archive_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `args`.\nThe arguments to pass to the driver."]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }
    #[doc = "Set the field `file_uris`.\nHCFS URIs of files to be copied to the working directory of Spark drivers and distributed tasks. Useful for naively parallel tasks."]
    pub fn set_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.file_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `jar_file_uris`.\nHCFS URIs of jar files to add to the CLASSPATHs of the Spark driver and tasks."]
    pub fn set_jar_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.jar_file_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `main_class`.\nThe class containing the main method of the driver. Must be in a provided jar or jar that is already on the classpath. Conflicts with main_jar_file_uri"]
    pub fn set_main_class(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.main_class = Some(v.into());
        self
    }
    #[doc = "Set the field `main_jar_file_uri`.\nThe HCFS URI of jar file containing the driver jar. Conflicts with main_class"]
    pub fn set_main_jar_file_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.main_jar_file_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\nA mapping of property names to values, used to configure Spark. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code."]
    pub fn set_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.properties = Some(v.into());
        self
    }
    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocJobHadoopConfigElLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.logging_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.logging_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocJobHadoopConfigEl {
    type O = BlockAssignable<DataprocJobHadoopConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocJobHadoopConfigEl {}
impl BuildDataprocJobHadoopConfigEl {
    pub fn build(self) -> DataprocJobHadoopConfigEl {
        DataprocJobHadoopConfigEl {
            archive_uris: core::default::Default::default(),
            args: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            jar_file_uris: core::default::Default::default(),
            main_class: core::default::Default::default(),
            main_jar_file_uri: core::default::Default::default(),
            properties: core::default::Default::default(),
            logging_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocJobHadoopConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocJobHadoopConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocJobHadoopConfigElRef {
        DataprocJobHadoopConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocJobHadoopConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `archive_uris` after provisioning.\nHCFS URIs of archives to be extracted in the working directory of .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn archive_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\nThe arguments to pass to the driver."]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }
    #[doc = "Get a reference to the value of field `file_uris` after provisioning.\nHCFS URIs of files to be copied to the working directory of Spark drivers and distributed tasks. Useful for naively parallel tasks."]
    pub fn file_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.file_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `jar_file_uris` after provisioning.\nHCFS URIs of jar files to add to the CLASSPATHs of the Spark driver and tasks."]
    pub fn jar_file_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.jar_file_uris", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `main_class` after provisioning.\nThe class containing the main method of the driver. Must be in a provided jar or jar that is already on the classpath. Conflicts with main_jar_file_uri"]
    pub fn main_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.main_class", self.base))
    }
    #[doc = "Get a reference to the value of field `main_jar_file_uri` after provisioning.\nThe HCFS URI of jar file containing the driver jar. Conflicts with main_class"]
    pub fn main_jar_file_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.main_jar_file_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\nA mapping of property names to values, used to configure Spark. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code."]
    pub fn properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> ListRef<DataprocJobHadoopConfigElLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocJobHiveConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    continue_on_failure: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jar_file_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_file_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_list: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    script_variables: Option<RecField<PrimField<String>>>,
}
impl DataprocJobHiveConfigEl {
    #[doc = "Set the field `continue_on_failure`.\nWhether to continue executing queries if a query fails. The default value is false. Setting to true can be useful when executing independent parallel queries. Defaults to false."]
    pub fn set_continue_on_failure(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.continue_on_failure = Some(v.into());
        self
    }
    #[doc = "Set the field `jar_file_uris`.\nHCFS URIs of jar files to add to the CLASSPATH of the Hive server and Hadoop MapReduce (MR) tasks. Can contain Hive SerDes and UDFs."]
    pub fn set_jar_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.jar_file_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\nA mapping of property names and values, used to configure Hive. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in /etc/hadoop/conf/*-site.xml, /etc/hive/conf/hive-site.xml, and classes in user code."]
    pub fn set_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.properties = Some(v.into());
        self
    }
    #[doc = "Set the field `query_file_uri`.\nHCFS URI of file containing Hive script to execute as the job. Conflicts with query_list"]
    pub fn set_query_file_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.query_file_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `query_list`.\nThe list of Hive queries or statements to execute as part of the job. Conflicts with query_file_uri"]
    pub fn set_query_list(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.query_list = Some(v.into());
        self
    }
    #[doc = "Set the field `script_variables`.\nMapping of query variable names to values (equivalent to the Hive command: SET name=\"value\";)."]
    pub fn set_script_variables(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.script_variables = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocJobHiveConfigEl {
    type O = BlockAssignable<DataprocJobHiveConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocJobHiveConfigEl {}
impl BuildDataprocJobHiveConfigEl {
    pub fn build(self) -> DataprocJobHiveConfigEl {
        DataprocJobHiveConfigEl {
            continue_on_failure: core::default::Default::default(),
            jar_file_uris: core::default::Default::default(),
            properties: core::default::Default::default(),
            query_file_uri: core::default::Default::default(),
            query_list: core::default::Default::default(),
            script_variables: core::default::Default::default(),
        }
    }
}
pub struct DataprocJobHiveConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocJobHiveConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocJobHiveConfigElRef {
        DataprocJobHiveConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocJobHiveConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `continue_on_failure` after provisioning.\nWhether to continue executing queries if a query fails. The default value is false. Setting to true can be useful when executing independent parallel queries. Defaults to false."]
    pub fn continue_on_failure(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.continue_on_failure", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `jar_file_uris` after provisioning.\nHCFS URIs of jar files to add to the CLASSPATH of the Hive server and Hadoop MapReduce (MR) tasks. Can contain Hive SerDes and UDFs."]
    pub fn jar_file_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.jar_file_uris", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\nA mapping of property names and values, used to configure Hive. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in /etc/hadoop/conf/*-site.xml, /etc/hive/conf/hive-site.xml, and classes in user code."]
    pub fn properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
    #[doc = "Get a reference to the value of field `query_file_uri` after provisioning.\nHCFS URI of file containing Hive script to execute as the job. Conflicts with query_list"]
    pub fn query_file_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_file_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query_list` after provisioning.\nThe list of Hive queries or statements to execute as part of the job. Conflicts with query_file_uri"]
    pub fn query_list(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.query_list", self.base))
    }
    #[doc = "Get a reference to the value of field `script_variables` after provisioning.\nMapping of query variable names to values (equivalent to the Hive command: SET name=\"value\";)."]
    pub fn script_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.script_variables", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocJobPigConfigElLoggingConfigEl {
    driver_log_levels: RecField<PrimField<String>>,
}
impl DataprocJobPigConfigElLoggingConfigEl {}
impl ToListMappable for DataprocJobPigConfigElLoggingConfigEl {
    type O = BlockAssignable<DataprocJobPigConfigElLoggingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocJobPigConfigElLoggingConfigEl {
    #[doc = "Optional. The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'."]
    pub driver_log_levels: RecField<PrimField<String>>,
}
impl BuildDataprocJobPigConfigElLoggingConfigEl {
    pub fn build(self) -> DataprocJobPigConfigElLoggingConfigEl {
        DataprocJobPigConfigElLoggingConfigEl {
            driver_log_levels: self.driver_log_levels,
        }
    }
}
pub struct DataprocJobPigConfigElLoggingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocJobPigConfigElLoggingConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocJobPigConfigElLoggingConfigElRef {
        DataprocJobPigConfigElLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocJobPigConfigElLoggingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `driver_log_levels` after provisioning.\nOptional. The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'."]
    pub fn driver_log_levels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.driver_log_levels", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataprocJobPigConfigElDynamic {
    logging_config: Option<DynamicBlock<DataprocJobPigConfigElLoggingConfigEl>>,
}
#[derive(Serialize)]
pub struct DataprocJobPigConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    continue_on_failure: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jar_file_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_file_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_list: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    script_variables: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_config: Option<Vec<DataprocJobPigConfigElLoggingConfigEl>>,
    dynamic: DataprocJobPigConfigElDynamic,
}
impl DataprocJobPigConfigEl {
    #[doc = "Set the field `continue_on_failure`.\nWhether to continue executing queries if a query fails. The default value is false. Setting to true can be useful when executing independent parallel queries. Defaults to false."]
    pub fn set_continue_on_failure(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.continue_on_failure = Some(v.into());
        self
    }
    #[doc = "Set the field `jar_file_uris`.\nHCFS URIs of jar files to add to the CLASSPATH of the Pig Client and Hadoop MapReduce (MR) tasks. Can contain Pig UDFs."]
    pub fn set_jar_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.jar_file_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\nA mapping of property names to values, used to configure Pig. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in /etc/hadoop/conf/*-site.xml, /etc/pig/conf/pig.properties, and classes in user code."]
    pub fn set_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.properties = Some(v.into());
        self
    }
    #[doc = "Set the field `query_file_uri`.\nHCFS URI of file containing Hive script to execute as the job. Conflicts with query_list"]
    pub fn set_query_file_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.query_file_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `query_list`.\nThe list of Hive queries or statements to execute as part of the job. Conflicts with query_file_uri"]
    pub fn set_query_list(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.query_list = Some(v.into());
        self
    }
    #[doc = "Set the field `script_variables`.\nMapping of query variable names to values (equivalent to the Pig command: name=[value])."]
    pub fn set_script_variables(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.script_variables = Some(v.into());
        self
    }
    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocJobPigConfigElLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.logging_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.logging_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocJobPigConfigEl {
    type O = BlockAssignable<DataprocJobPigConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocJobPigConfigEl {}
impl BuildDataprocJobPigConfigEl {
    pub fn build(self) -> DataprocJobPigConfigEl {
        DataprocJobPigConfigEl {
            continue_on_failure: core::default::Default::default(),
            jar_file_uris: core::default::Default::default(),
            properties: core::default::Default::default(),
            query_file_uri: core::default::Default::default(),
            query_list: core::default::Default::default(),
            script_variables: core::default::Default::default(),
            logging_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocJobPigConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocJobPigConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocJobPigConfigElRef {
        DataprocJobPigConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocJobPigConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `continue_on_failure` after provisioning.\nWhether to continue executing queries if a query fails. The default value is false. Setting to true can be useful when executing independent parallel queries. Defaults to false."]
    pub fn continue_on_failure(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.continue_on_failure", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `jar_file_uris` after provisioning.\nHCFS URIs of jar files to add to the CLASSPATH of the Pig Client and Hadoop MapReduce (MR) tasks. Can contain Pig UDFs."]
    pub fn jar_file_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.jar_file_uris", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\nA mapping of property names to values, used to configure Pig. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in /etc/hadoop/conf/*-site.xml, /etc/pig/conf/pig.properties, and classes in user code."]
    pub fn properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
    #[doc = "Get a reference to the value of field `query_file_uri` after provisioning.\nHCFS URI of file containing Hive script to execute as the job. Conflicts with query_list"]
    pub fn query_file_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_file_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query_list` after provisioning.\nThe list of Hive queries or statements to execute as part of the job. Conflicts with query_file_uri"]
    pub fn query_list(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.query_list", self.base))
    }
    #[doc = "Get a reference to the value of field `script_variables` after provisioning.\nMapping of query variable names to values (equivalent to the Pig command: name=[value])."]
    pub fn script_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.script_variables", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> ListRef<DataprocJobPigConfigElLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocJobPlacementEl {
    cluster_name: PrimField<String>,
}
impl DataprocJobPlacementEl {}
impl ToListMappable for DataprocJobPlacementEl {
    type O = BlockAssignable<DataprocJobPlacementEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocJobPlacementEl {
    #[doc = "The name of the cluster where the job will be submitted"]
    pub cluster_name: PrimField<String>,
}
impl BuildDataprocJobPlacementEl {
    pub fn build(self) -> DataprocJobPlacementEl {
        DataprocJobPlacementEl {
            cluster_name: self.cluster_name,
        }
    }
}
pub struct DataprocJobPlacementElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocJobPlacementElRef {
    fn new(shared: StackShared, base: String) -> DataprocJobPlacementElRef {
        DataprocJobPlacementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocJobPlacementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\nThe name of the cluster where the job will be submitted"]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.base))
    }
    #[doc = "Get a reference to the value of field `cluster_uuid` after provisioning.\nOutput-only. A cluster UUID generated by the Cloud Dataproc service when the job is submitted"]
    pub fn cluster_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_uuid", self.base))
    }
}
#[derive(Serialize)]
pub struct DataprocJobPrestoConfigElLoggingConfigEl {
    driver_log_levels: RecField<PrimField<String>>,
}
impl DataprocJobPrestoConfigElLoggingConfigEl {}
impl ToListMappable for DataprocJobPrestoConfigElLoggingConfigEl {
    type O = BlockAssignable<DataprocJobPrestoConfigElLoggingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocJobPrestoConfigElLoggingConfigEl {
    #[doc = "Optional. The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'."]
    pub driver_log_levels: RecField<PrimField<String>>,
}
impl BuildDataprocJobPrestoConfigElLoggingConfigEl {
    pub fn build(self) -> DataprocJobPrestoConfigElLoggingConfigEl {
        DataprocJobPrestoConfigElLoggingConfigEl {
            driver_log_levels: self.driver_log_levels,
        }
    }
}
pub struct DataprocJobPrestoConfigElLoggingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocJobPrestoConfigElLoggingConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocJobPrestoConfigElLoggingConfigElRef {
        DataprocJobPrestoConfigElLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocJobPrestoConfigElLoggingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `driver_log_levels` after provisioning.\nOptional. The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'."]
    pub fn driver_log_levels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.driver_log_levels", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataprocJobPrestoConfigElDynamic {
    logging_config: Option<DynamicBlock<DataprocJobPrestoConfigElLoggingConfigEl>>,
}
#[derive(Serialize)]
pub struct DataprocJobPrestoConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    continue_on_failure: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_file_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_list: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_config: Option<Vec<DataprocJobPrestoConfigElLoggingConfigEl>>,
    dynamic: DataprocJobPrestoConfigElDynamic,
}
impl DataprocJobPrestoConfigEl {
    #[doc = "Set the field `client_tags`.\nPresto client tags to attach to this query."]
    pub fn set_client_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.client_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `continue_on_failure`.\nWhether to continue executing queries if a query fails. Setting to true can be useful when executing independent parallel queries. Defaults to false."]
    pub fn set_continue_on_failure(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.continue_on_failure = Some(v.into());
        self
    }
    #[doc = "Set the field `output_format`.\nThe format in which query output will be displayed. See the Presto documentation for supported output formats."]
    pub fn set_output_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_format = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\nA mapping of property names to values. Used to set Presto session properties Equivalent to using the --session flag in the Presto CLI."]
    pub fn set_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.properties = Some(v.into());
        self
    }
    #[doc = "Set the field `query_file_uri`.\nThe HCFS URI of the script that contains SQL queries. Conflicts with query_list"]
    pub fn set_query_file_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.query_file_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `query_list`.\nThe list of SQL queries or statements to execute as part of the job. Conflicts with query_file_uri"]
    pub fn set_query_list(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.query_list = Some(v.into());
        self
    }
    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocJobPrestoConfigElLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.logging_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.logging_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocJobPrestoConfigEl {
    type O = BlockAssignable<DataprocJobPrestoConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocJobPrestoConfigEl {}
impl BuildDataprocJobPrestoConfigEl {
    pub fn build(self) -> DataprocJobPrestoConfigEl {
        DataprocJobPrestoConfigEl {
            client_tags: core::default::Default::default(),
            continue_on_failure: core::default::Default::default(),
            output_format: core::default::Default::default(),
            properties: core::default::Default::default(),
            query_file_uri: core::default::Default::default(),
            query_list: core::default::Default::default(),
            logging_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocJobPrestoConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocJobPrestoConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocJobPrestoConfigElRef {
        DataprocJobPrestoConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocJobPrestoConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_tags` after provisioning.\nPresto client tags to attach to this query."]
    pub fn client_tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.client_tags", self.base))
    }
    #[doc = "Get a reference to the value of field `continue_on_failure` after provisioning.\nWhether to continue executing queries if a query fails. Setting to true can be useful when executing independent parallel queries. Defaults to false."]
    pub fn continue_on_failure(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.continue_on_failure", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `output_format` after provisioning.\nThe format in which query output will be displayed. See the Presto documentation for supported output formats."]
    pub fn output_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\nA mapping of property names to values. Used to set Presto session properties Equivalent to using the --session flag in the Presto CLI."]
    pub fn properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
    #[doc = "Get a reference to the value of field `query_file_uri` after provisioning.\nThe HCFS URI of the script that contains SQL queries. Conflicts with query_list"]
    pub fn query_file_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_file_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query_list` after provisioning.\nThe list of SQL queries or statements to execute as part of the job. Conflicts with query_file_uri"]
    pub fn query_list(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.query_list", self.base))
    }
    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> ListRef<DataprocJobPrestoConfigElLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocJobPysparkConfigElLoggingConfigEl {
    driver_log_levels: RecField<PrimField<String>>,
}
impl DataprocJobPysparkConfigElLoggingConfigEl {}
impl ToListMappable for DataprocJobPysparkConfigElLoggingConfigEl {
    type O = BlockAssignable<DataprocJobPysparkConfigElLoggingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocJobPysparkConfigElLoggingConfigEl {
    #[doc = "Optional. The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'."]
    pub driver_log_levels: RecField<PrimField<String>>,
}
impl BuildDataprocJobPysparkConfigElLoggingConfigEl {
    pub fn build(self) -> DataprocJobPysparkConfigElLoggingConfigEl {
        DataprocJobPysparkConfigElLoggingConfigEl {
            driver_log_levels: self.driver_log_levels,
        }
    }
}
pub struct DataprocJobPysparkConfigElLoggingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocJobPysparkConfigElLoggingConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocJobPysparkConfigElLoggingConfigElRef {
        DataprocJobPysparkConfigElLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocJobPysparkConfigElLoggingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `driver_log_levels` after provisioning.\nOptional. The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'."]
    pub fn driver_log_levels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.driver_log_levels", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataprocJobPysparkConfigElDynamic {
    logging_config: Option<DynamicBlock<DataprocJobPysparkConfigElLoggingConfigEl>>,
}
#[derive(Serialize)]
pub struct DataprocJobPysparkConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    archive_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jar_file_uris: Option<ListField<PrimField<String>>>,
    main_python_file_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_file_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_config: Option<Vec<DataprocJobPysparkConfigElLoggingConfigEl>>,
    dynamic: DataprocJobPysparkConfigElDynamic,
}
impl DataprocJobPysparkConfigEl {
    #[doc = "Set the field `archive_uris`.\nOptional. HCFS URIs of archives to be extracted in the working directory of .jar, .tar, .tar.gz, .tgz, and .zip"]
    pub fn set_archive_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.archive_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `args`.\nOptional. The arguments to pass to the driver. Do not include arguments, such as --conf, that can be set as job properties, since a collision may occur that causes an incorrect job submission"]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }
    #[doc = "Set the field `file_uris`.\nOptional. HCFS URIs of files to be copied to the working directory of Python drivers and distributed tasks. Useful for naively parallel tasks"]
    pub fn set_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.file_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `jar_file_uris`.\nOptional. HCFS URIs of jar files to add to the CLASSPATHs of the Python driver and tasks"]
    pub fn set_jar_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.jar_file_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\nOptional. A mapping of property names to values, used to configure PySpark. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code"]
    pub fn set_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.properties = Some(v.into());
        self
    }
    #[doc = "Set the field `python_file_uris`.\nOptional. HCFS file URIs of Python files to pass to the PySpark framework. Supported file types: .py, .egg, and .zip"]
    pub fn set_python_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.python_file_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocJobPysparkConfigElLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.logging_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.logging_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocJobPysparkConfigEl {
    type O = BlockAssignable<DataprocJobPysparkConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocJobPysparkConfigEl {
    #[doc = "Required. The HCFS URI of the main Python file to use as the driver. Must be a .py file"]
    pub main_python_file_uri: PrimField<String>,
}
impl BuildDataprocJobPysparkConfigEl {
    pub fn build(self) -> DataprocJobPysparkConfigEl {
        DataprocJobPysparkConfigEl {
            archive_uris: core::default::Default::default(),
            args: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            jar_file_uris: core::default::Default::default(),
            main_python_file_uri: self.main_python_file_uri,
            properties: core::default::Default::default(),
            python_file_uris: core::default::Default::default(),
            logging_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocJobPysparkConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocJobPysparkConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocJobPysparkConfigElRef {
        DataprocJobPysparkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocJobPysparkConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `archive_uris` after provisioning.\nOptional. HCFS URIs of archives to be extracted in the working directory of .jar, .tar, .tar.gz, .tgz, and .zip"]
    pub fn archive_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\nOptional. The arguments to pass to the driver. Do not include arguments, such as --conf, that can be set as job properties, since a collision may occur that causes an incorrect job submission"]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }
    #[doc = "Get a reference to the value of field `file_uris` after provisioning.\nOptional. HCFS URIs of files to be copied to the working directory of Python drivers and distributed tasks. Useful for naively parallel tasks"]
    pub fn file_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.file_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `jar_file_uris` after provisioning.\nOptional. HCFS URIs of jar files to add to the CLASSPATHs of the Python driver and tasks"]
    pub fn jar_file_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.jar_file_uris", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `main_python_file_uri` after provisioning.\nRequired. The HCFS URI of the main Python file to use as the driver. Must be a .py file"]
    pub fn main_python_file_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.main_python_file_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\nOptional. A mapping of property names to values, used to configure PySpark. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code"]
    pub fn properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
    #[doc = "Get a reference to the value of field `python_file_uris` after provisioning.\nOptional. HCFS file URIs of Python files to pass to the PySpark framework. Supported file types: .py, .egg, and .zip"]
    pub fn python_file_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.python_file_uris", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> ListRef<DataprocJobPysparkConfigElLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocJobReferenceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    job_id: Option<PrimField<String>>,
}
impl DataprocJobReferenceEl {
    #[doc = "Set the field `job_id`.\nThe job ID, which must be unique within the project. The job ID is generated by the server upon job submission or provided by the user as a means to perform retries without creating duplicate jobs"]
    pub fn set_job_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.job_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocJobReferenceEl {
    type O = BlockAssignable<DataprocJobReferenceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocJobReferenceEl {}
impl BuildDataprocJobReferenceEl {
    pub fn build(self) -> DataprocJobReferenceEl {
        DataprocJobReferenceEl {
            job_id: core::default::Default::default(),
        }
    }
}
pub struct DataprocJobReferenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocJobReferenceElRef {
    fn new(shared: StackShared, base: String) -> DataprocJobReferenceElRef {
        DataprocJobReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocJobReferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `job_id` after provisioning.\nThe job ID, which must be unique within the project. The job ID is generated by the server upon job submission or provided by the user as a means to perform retries without creating duplicate jobs"]
    pub fn job_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_id", self.base))
    }
}
#[derive(Serialize)]
pub struct DataprocJobSchedulingEl {
    max_failures_per_hour: PrimField<f64>,
    max_failures_total: PrimField<f64>,
}
impl DataprocJobSchedulingEl {}
impl ToListMappable for DataprocJobSchedulingEl {
    type O = BlockAssignable<DataprocJobSchedulingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocJobSchedulingEl {
    #[doc = "Maximum number of times per hour a driver may be restarted as a result of driver exiting with non-zero code before job is reported failed."]
    pub max_failures_per_hour: PrimField<f64>,
    #[doc = "Maximum number of times in total a driver may be restarted as a result of driver exiting with non-zero code before job is reported failed."]
    pub max_failures_total: PrimField<f64>,
}
impl BuildDataprocJobSchedulingEl {
    pub fn build(self) -> DataprocJobSchedulingEl {
        DataprocJobSchedulingEl {
            max_failures_per_hour: self.max_failures_per_hour,
            max_failures_total: self.max_failures_total,
        }
    }
}
pub struct DataprocJobSchedulingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocJobSchedulingElRef {
    fn new(shared: StackShared, base: String) -> DataprocJobSchedulingElRef {
        DataprocJobSchedulingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocJobSchedulingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_failures_per_hour` after provisioning.\nMaximum number of times per hour a driver may be restarted as a result of driver exiting with non-zero code before job is reported failed."]
    pub fn max_failures_per_hour(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_failures_per_hour", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_failures_total` after provisioning.\nMaximum number of times in total a driver may be restarted as a result of driver exiting with non-zero code before job is reported failed."]
    pub fn max_failures_total(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_failures_total", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocJobSparkConfigElLoggingConfigEl {
    driver_log_levels: RecField<PrimField<String>>,
}
impl DataprocJobSparkConfigElLoggingConfigEl {}
impl ToListMappable for DataprocJobSparkConfigElLoggingConfigEl {
    type O = BlockAssignable<DataprocJobSparkConfigElLoggingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocJobSparkConfigElLoggingConfigEl {
    #[doc = "Optional. The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'."]
    pub driver_log_levels: RecField<PrimField<String>>,
}
impl BuildDataprocJobSparkConfigElLoggingConfigEl {
    pub fn build(self) -> DataprocJobSparkConfigElLoggingConfigEl {
        DataprocJobSparkConfigElLoggingConfigEl {
            driver_log_levels: self.driver_log_levels,
        }
    }
}
pub struct DataprocJobSparkConfigElLoggingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocJobSparkConfigElLoggingConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocJobSparkConfigElLoggingConfigElRef {
        DataprocJobSparkConfigElLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocJobSparkConfigElLoggingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `driver_log_levels` after provisioning.\nOptional. The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'."]
    pub fn driver_log_levels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.driver_log_levels", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataprocJobSparkConfigElDynamic {
    logging_config: Option<DynamicBlock<DataprocJobSparkConfigElLoggingConfigEl>>,
}
#[derive(Serialize)]
pub struct DataprocJobSparkConfigEl {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_config: Option<Vec<DataprocJobSparkConfigElLoggingConfigEl>>,
    dynamic: DataprocJobSparkConfigElDynamic,
}
impl DataprocJobSparkConfigEl {
    #[doc = "Set the field `archive_uris`.\nHCFS URIs of archives to be extracted in the working directory of .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn set_archive_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.archive_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `args`.\nThe arguments to pass to the driver."]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }
    #[doc = "Set the field `file_uris`.\nHCFS URIs of files to be copied to the working directory of Spark drivers and distributed tasks. Useful for naively parallel tasks."]
    pub fn set_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.file_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `jar_file_uris`.\nHCFS URIs of jar files to add to the CLASSPATHs of the Spark driver and tasks."]
    pub fn set_jar_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.jar_file_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `main_class`.\nThe class containing the main method of the driver. Must be in a provided jar or jar that is already on the classpath. Conflicts with main_jar_file_uri"]
    pub fn set_main_class(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.main_class = Some(v.into());
        self
    }
    #[doc = "Set the field `main_jar_file_uri`.\nThe HCFS URI of jar file containing the driver jar. Conflicts with main_class"]
    pub fn set_main_jar_file_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.main_jar_file_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\nA mapping of property names to values, used to configure Spark. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code."]
    pub fn set_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.properties = Some(v.into());
        self
    }
    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocJobSparkConfigElLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.logging_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.logging_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocJobSparkConfigEl {
    type O = BlockAssignable<DataprocJobSparkConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocJobSparkConfigEl {}
impl BuildDataprocJobSparkConfigEl {
    pub fn build(self) -> DataprocJobSparkConfigEl {
        DataprocJobSparkConfigEl {
            archive_uris: core::default::Default::default(),
            args: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            jar_file_uris: core::default::Default::default(),
            main_class: core::default::Default::default(),
            main_jar_file_uri: core::default::Default::default(),
            properties: core::default::Default::default(),
            logging_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocJobSparkConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocJobSparkConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocJobSparkConfigElRef {
        DataprocJobSparkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocJobSparkConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `archive_uris` after provisioning.\nHCFS URIs of archives to be extracted in the working directory of .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn archive_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\nThe arguments to pass to the driver."]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }
    #[doc = "Get a reference to the value of field `file_uris` after provisioning.\nHCFS URIs of files to be copied to the working directory of Spark drivers and distributed tasks. Useful for naively parallel tasks."]
    pub fn file_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.file_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `jar_file_uris` after provisioning.\nHCFS URIs of jar files to add to the CLASSPATHs of the Spark driver and tasks."]
    pub fn jar_file_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.jar_file_uris", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `main_class` after provisioning.\nThe class containing the main method of the driver. Must be in a provided jar or jar that is already on the classpath. Conflicts with main_jar_file_uri"]
    pub fn main_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.main_class", self.base))
    }
    #[doc = "Get a reference to the value of field `main_jar_file_uri` after provisioning.\nThe HCFS URI of jar file containing the driver jar. Conflicts with main_class"]
    pub fn main_jar_file_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.main_jar_file_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\nA mapping of property names to values, used to configure Spark. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code."]
    pub fn properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> ListRef<DataprocJobSparkConfigElLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocJobSparksqlConfigElLoggingConfigEl {
    driver_log_levels: RecField<PrimField<String>>,
}
impl DataprocJobSparksqlConfigElLoggingConfigEl {}
impl ToListMappable for DataprocJobSparksqlConfigElLoggingConfigEl {
    type O = BlockAssignable<DataprocJobSparksqlConfigElLoggingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocJobSparksqlConfigElLoggingConfigEl {
    #[doc = "Optional. The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'."]
    pub driver_log_levels: RecField<PrimField<String>>,
}
impl BuildDataprocJobSparksqlConfigElLoggingConfigEl {
    pub fn build(self) -> DataprocJobSparksqlConfigElLoggingConfigEl {
        DataprocJobSparksqlConfigElLoggingConfigEl {
            driver_log_levels: self.driver_log_levels,
        }
    }
}
pub struct DataprocJobSparksqlConfigElLoggingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocJobSparksqlConfigElLoggingConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocJobSparksqlConfigElLoggingConfigElRef {
        DataprocJobSparksqlConfigElLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocJobSparksqlConfigElLoggingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `driver_log_levels` after provisioning.\nOptional. The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'."]
    pub fn driver_log_levels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.driver_log_levels", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataprocJobSparksqlConfigElDynamic {
    logging_config: Option<DynamicBlock<DataprocJobSparksqlConfigElLoggingConfigEl>>,
}
#[derive(Serialize)]
pub struct DataprocJobSparksqlConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    jar_file_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_file_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_list: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    script_variables: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_config: Option<Vec<DataprocJobSparksqlConfigElLoggingConfigEl>>,
    dynamic: DataprocJobSparksqlConfigElDynamic,
}
impl DataprocJobSparksqlConfigEl {
    #[doc = "Set the field `jar_file_uris`.\nHCFS URIs of jar files to be added to the Spark CLASSPATH."]
    pub fn set_jar_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.jar_file_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\nA mapping of property names to values, used to configure Spark SQL's SparkConf. Properties that conflict with values set by the Cloud Dataproc API may be overwritten."]
    pub fn set_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.properties = Some(v.into());
        self
    }
    #[doc = "Set the field `query_file_uri`.\nThe HCFS URI of the script that contains SQL queries. Conflicts with query_list"]
    pub fn set_query_file_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.query_file_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `query_list`.\nThe list of SQL queries or statements to execute as part of the job. Conflicts with query_file_uri"]
    pub fn set_query_list(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.query_list = Some(v.into());
        self
    }
    #[doc = "Set the field `script_variables`.\nMapping of query variable names to values (equivalent to the Spark SQL command: SET name=\"value\";)."]
    pub fn set_script_variables(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.script_variables = Some(v.into());
        self
    }
    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocJobSparksqlConfigElLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.logging_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.logging_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocJobSparksqlConfigEl {
    type O = BlockAssignable<DataprocJobSparksqlConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocJobSparksqlConfigEl {}
impl BuildDataprocJobSparksqlConfigEl {
    pub fn build(self) -> DataprocJobSparksqlConfigEl {
        DataprocJobSparksqlConfigEl {
            jar_file_uris: core::default::Default::default(),
            properties: core::default::Default::default(),
            query_file_uri: core::default::Default::default(),
            query_list: core::default::Default::default(),
            script_variables: core::default::Default::default(),
            logging_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocJobSparksqlConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocJobSparksqlConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocJobSparksqlConfigElRef {
        DataprocJobSparksqlConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocJobSparksqlConfigElRef {
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
    #[doc = "Get a reference to the value of field `properties` after provisioning.\nA mapping of property names to values, used to configure Spark SQL's SparkConf. Properties that conflict with values set by the Cloud Dataproc API may be overwritten."]
    pub fn properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
    #[doc = "Get a reference to the value of field `query_file_uri` after provisioning.\nThe HCFS URI of the script that contains SQL queries. Conflicts with query_list"]
    pub fn query_file_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_file_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query_list` after provisioning.\nThe list of SQL queries or statements to execute as part of the job. Conflicts with query_file_uri"]
    pub fn query_list(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.query_list", self.base))
    }
    #[doc = "Get a reference to the value of field `script_variables` after provisioning.\nMapping of query variable names to values (equivalent to the Spark SQL command: SET name=\"value\";)."]
    pub fn script_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.script_variables", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> ListRef<DataprocJobSparksqlConfigElLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocJobTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl DataprocJobTimeoutsEl {
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
}
impl ToListMappable for DataprocJobTimeoutsEl {
    type O = BlockAssignable<DataprocJobTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocJobTimeoutsEl {}
impl BuildDataprocJobTimeoutsEl {
    pub fn build(self) -> DataprocJobTimeoutsEl {
        DataprocJobTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct DataprocJobTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocJobTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataprocJobTimeoutsElRef {
        DataprocJobTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocJobTimeoutsElRef {
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
}
#[derive(Serialize, Default)]
struct DataprocJobDynamic {
    hadoop_config: Option<DynamicBlock<DataprocJobHadoopConfigEl>>,
    hive_config: Option<DynamicBlock<DataprocJobHiveConfigEl>>,
    pig_config: Option<DynamicBlock<DataprocJobPigConfigEl>>,
    placement: Option<DynamicBlock<DataprocJobPlacementEl>>,
    presto_config: Option<DynamicBlock<DataprocJobPrestoConfigEl>>,
    pyspark_config: Option<DynamicBlock<DataprocJobPysparkConfigEl>>,
    reference: Option<DynamicBlock<DataprocJobReferenceEl>>,
    scheduling: Option<DynamicBlock<DataprocJobSchedulingEl>>,
    spark_config: Option<DynamicBlock<DataprocJobSparkConfigEl>>,
    sparksql_config: Option<DynamicBlock<DataprocJobSparksqlConfigEl>>,
}
