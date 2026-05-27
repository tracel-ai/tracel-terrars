use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataprocGdcSparkApplicationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_environment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dependency_images: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RecField<PrimField<String>>>,
    serviceinstance: PrimField<String>,
    spark_application_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pyspark_application_config: Option<Vec<DataprocGdcSparkApplicationPysparkApplicationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spark_application_config: Option<Vec<DataprocGdcSparkApplicationSparkApplicationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spark_r_application_config: Option<Vec<DataprocGdcSparkApplicationSparkRApplicationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spark_sql_application_config:
        Option<Vec<DataprocGdcSparkApplicationSparkSqlApplicationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataprocGdcSparkApplicationTimeoutsEl>,
    dynamic: DataprocGdcSparkApplicationDynamic,
}
struct DataprocGdcSparkApplication_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataprocGdcSparkApplicationData>,
}
#[derive(Clone)]
pub struct DataprocGdcSparkApplication(Rc<DataprocGdcSparkApplication_>);
impl DataprocGdcSparkApplication {
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
    #[doc = "Set the field `annotations`.\nThe annotations to associate with this application. Annotations may be used to store client information, but are not used by the server. \n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `application_environment`.\nAn ApplicationEnvironment from which to inherit configuration properties."]
    pub fn set_application_environment(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().application_environment = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `dependency_images`.\nList of container image uris for additional file dependencies. Dependent files are sequentially copied from each image. If a file with the same name exists in 2 images then the file from later image is used."]
    pub fn set_dependency_images(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().dependency_images = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nUser-provided human-readable name to be used in user interfaces."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nThe labels to associate with this application. Labels may be used for filtering and billing tracking. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `namespace`.\nThe Kubernetes namespace in which to create the application. This namespace must already exist on the cluster."]
    pub fn set_namespace(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().namespace = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\napplication-specific properties."]
    pub fn set_properties(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().properties = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\nThe Dataproc version of this application."]
    pub fn set_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version = Some(v.into());
        self
    }
    #[doc = "Set the field `pyspark_application_config`.\n"]
    pub fn set_pyspark_application_config(
        self,
        v: impl Into<BlockAssignable<DataprocGdcSparkApplicationPysparkApplicationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pyspark_application_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pyspark_application_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `spark_application_config`.\n"]
    pub fn set_spark_application_config(
        self,
        v: impl Into<BlockAssignable<DataprocGdcSparkApplicationSparkApplicationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spark_application_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spark_application_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `spark_r_application_config`.\n"]
    pub fn set_spark_r_application_config(
        self,
        v: impl Into<BlockAssignable<DataprocGdcSparkApplicationSparkRApplicationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spark_r_application_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spark_r_application_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `spark_sql_application_config`.\n"]
    pub fn set_spark_sql_application_config(
        self,
        v: impl Into<BlockAssignable<DataprocGdcSparkApplicationSparkSqlApplicationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spark_sql_application_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .spark_sql_application_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataprocGdcSparkApplicationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nThe annotations to associate with this application. Annotations may be used to store client information, but are not used by the server. \n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `application_environment` after provisioning.\nAn ApplicationEnvironment from which to inherit configuration properties."]
    pub fn application_environment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the resource was created."]
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
    #[doc = "Get a reference to the value of field `dependency_images` after provisioning.\nList of container image uris for additional file dependencies. Dependent files are sequentially copied from each image. If a file with the same name exists in 2 images then the file from later image is used."]
    pub fn dependency_images(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dependency_images", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser-provided human-readable name to be used in user interfaces."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels to associate with this application. Labels may be used for filtering and billing tracking. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the spark application."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monitoring_endpoint` after provisioning.\nURL for a monitoring UI for this application (for eventual Spark PHS/UI support) Out of scope for private GA"]
    pub fn monitoring_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monitoring_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the application. Format: projects/{project}/locations/{location}/serviceInstances/{service_instance}/sparkApplications/{application}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\nThe Kubernetes namespace in which to create the application. This namespace must already exist on the cluster."]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespace", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `output_uri` after provisioning.\nAn HCFS URI pointing to the location of stdout and stdout of the application Mainly useful for Pantheon and gcloud Not in scope for private GA"]
    pub fn output_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\napplication-specific properties."]
    pub fn properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nWhether the application is currently reconciling. True if the current state of the resource does not match the intended state, and the system is working to reconcile them, whether or not the change was user initiated."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `serviceinstance` after provisioning.\nThe id of the service instance to which this spark application belongs."]
    pub fn serviceinstance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.serviceinstance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spark_application_id` after provisioning.\nThe id of the application"]
    pub fn spark_application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.spark_application_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state.\nPossible values:\n* 'STATE_UNSPECIFIED'\n* 'PENDING'\n* 'RUNNING'\n* 'CANCELLING'\n* 'CANCELLED'\n* 'SUCCEEDED'\n* 'FAILED'"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_message` after provisioning.\nA message explaining the current state."]
    pub fn state_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_message", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem generated unique identifier for this application, formatted as UUID4."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the resource was most recently updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe Dataproc version of this application."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pyspark_application_config` after provisioning.\n"]
    pub fn pyspark_application_config(
        &self,
    ) -> ListRef<DataprocGdcSparkApplicationPysparkApplicationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pyspark_application_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spark_application_config` after provisioning.\n"]
    pub fn spark_application_config(
        &self,
    ) -> ListRef<DataprocGdcSparkApplicationSparkApplicationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spark_application_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spark_r_application_config` after provisioning.\n"]
    pub fn spark_r_application_config(
        &self,
    ) -> ListRef<DataprocGdcSparkApplicationSparkRApplicationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spark_r_application_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spark_sql_application_config` after provisioning.\n"]
    pub fn spark_sql_application_config(
        &self,
    ) -> ListRef<DataprocGdcSparkApplicationSparkSqlApplicationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spark_sql_application_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocGdcSparkApplicationTimeoutsElRef {
        DataprocGdcSparkApplicationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataprocGdcSparkApplication {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataprocGdcSparkApplication {}
impl ToListMappable for DataprocGdcSparkApplication {
    type O = ListRef<DataprocGdcSparkApplicationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataprocGdcSparkApplication_ {
    fn extract_resource_type(&self) -> String {
        "google_dataproc_gdc_spark_application".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataprocGdcSparkApplication {
    pub tf_id: String,
    #[doc = "The location of the spark application."]
    pub location: PrimField<String>,
    #[doc = "The id of the service instance to which this spark application belongs."]
    pub serviceinstance: PrimField<String>,
    #[doc = "The id of the application"]
    pub spark_application_id: PrimField<String>,
}
impl BuildDataprocGdcSparkApplication {
    pub fn build(self, stack: &mut Stack) -> DataprocGdcSparkApplication {
        let out = DataprocGdcSparkApplication(Rc::new(DataprocGdcSparkApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataprocGdcSparkApplicationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                application_environment: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                dependency_images: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                namespace: core::default::Default::default(),
                project: core::default::Default::default(),
                properties: core::default::Default::default(),
                serviceinstance: self.serviceinstance,
                spark_application_id: self.spark_application_id,
                version: core::default::Default::default(),
                pyspark_application_config: core::default::Default::default(),
                spark_application_config: core::default::Default::default(),
                spark_r_application_config: core::default::Default::default(),
                spark_sql_application_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DataprocGdcSparkApplicationRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocGdcSparkApplicationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataprocGdcSparkApplicationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nThe annotations to associate with this application. Annotations may be used to store client information, but are not used by the server. \n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `application_environment` after provisioning.\nAn ApplicationEnvironment from which to inherit configuration properties."]
    pub fn application_environment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the resource was created."]
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
    #[doc = "Get a reference to the value of field `dependency_images` after provisioning.\nList of container image uris for additional file dependencies. Dependent files are sequentially copied from each image. If a file with the same name exists in 2 images then the file from later image is used."]
    pub fn dependency_images(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dependency_images", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser-provided human-readable name to be used in user interfaces."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels to associate with this application. Labels may be used for filtering and billing tracking. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the spark application."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monitoring_endpoint` after provisioning.\nURL for a monitoring UI for this application (for eventual Spark PHS/UI support) Out of scope for private GA"]
    pub fn monitoring_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monitoring_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the application. Format: projects/{project}/locations/{location}/serviceInstances/{service_instance}/sparkApplications/{application}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\nThe Kubernetes namespace in which to create the application. This namespace must already exist on the cluster."]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespace", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `output_uri` after provisioning.\nAn HCFS URI pointing to the location of stdout and stdout of the application Mainly useful for Pantheon and gcloud Not in scope for private GA"]
    pub fn output_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\napplication-specific properties."]
    pub fn properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nWhether the application is currently reconciling. True if the current state of the resource does not match the intended state, and the system is working to reconcile them, whether or not the change was user initiated."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `serviceinstance` after provisioning.\nThe id of the service instance to which this spark application belongs."]
    pub fn serviceinstance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.serviceinstance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spark_application_id` after provisioning.\nThe id of the application"]
    pub fn spark_application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.spark_application_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state.\nPossible values:\n* 'STATE_UNSPECIFIED'\n* 'PENDING'\n* 'RUNNING'\n* 'CANCELLING'\n* 'CANCELLED'\n* 'SUCCEEDED'\n* 'FAILED'"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_message` after provisioning.\nA message explaining the current state."]
    pub fn state_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_message", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem generated unique identifier for this application, formatted as UUID4."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the resource was most recently updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe Dataproc version of this application."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pyspark_application_config` after provisioning.\n"]
    pub fn pyspark_application_config(
        &self,
    ) -> ListRef<DataprocGdcSparkApplicationPysparkApplicationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pyspark_application_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spark_application_config` after provisioning.\n"]
    pub fn spark_application_config(
        &self,
    ) -> ListRef<DataprocGdcSparkApplicationSparkApplicationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spark_application_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spark_r_application_config` after provisioning.\n"]
    pub fn spark_r_application_config(
        &self,
    ) -> ListRef<DataprocGdcSparkApplicationSparkRApplicationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spark_r_application_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spark_sql_application_config` after provisioning.\n"]
    pub fn spark_sql_application_config(
        &self,
    ) -> ListRef<DataprocGdcSparkApplicationSparkSqlApplicationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spark_sql_application_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocGdcSparkApplicationTimeoutsElRef {
        DataprocGdcSparkApplicationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocGdcSparkApplicationPysparkApplicationConfigEl {
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
    python_file_uris: Option<ListField<PrimField<String>>>,
}
impl DataprocGdcSparkApplicationPysparkApplicationConfigEl {
    #[doc = "Set the field `archive_uris`.\nHCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn set_archive_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.archive_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `args`.\nThe arguments to pass to the driver.  Do not include arguments, such as '--conf', that can be set as job properties, since a collision may occur that causes an incorrect job submission."]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }
    #[doc = "Set the field `file_uris`.\nHCFS URIs of files to be placed in the working directory of each executor. Useful for naively parallel tasks."]
    pub fn set_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.file_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `jar_file_uris`.\nHCFS URIs of jar files to add to the CLASSPATHs of the Python driver and tasks."]
    pub fn set_jar_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.jar_file_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `python_file_uris`.\nHCFS file URIs of Python files to pass to the PySpark framework. Supported file types: .py, .egg, and .zip."]
    pub fn set_python_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.python_file_uris = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocGdcSparkApplicationPysparkApplicationConfigEl {
    type O = BlockAssignable<DataprocGdcSparkApplicationPysparkApplicationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocGdcSparkApplicationPysparkApplicationConfigEl {
    #[doc = "The HCFS URI of the main Python file to use as the driver. Must be a .py file."]
    pub main_python_file_uri: PrimField<String>,
}
impl BuildDataprocGdcSparkApplicationPysparkApplicationConfigEl {
    pub fn build(self) -> DataprocGdcSparkApplicationPysparkApplicationConfigEl {
        DataprocGdcSparkApplicationPysparkApplicationConfigEl {
            archive_uris: core::default::Default::default(),
            args: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            jar_file_uris: core::default::Default::default(),
            main_python_file_uri: self.main_python_file_uri,
            python_file_uris: core::default::Default::default(),
        }
    }
}
pub struct DataprocGdcSparkApplicationPysparkApplicationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocGdcSparkApplicationPysparkApplicationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocGdcSparkApplicationPysparkApplicationConfigElRef {
        DataprocGdcSparkApplicationPysparkApplicationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocGdcSparkApplicationPysparkApplicationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `archive_uris` after provisioning.\nHCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn archive_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\nThe arguments to pass to the driver.  Do not include arguments, such as '--conf', that can be set as job properties, since a collision may occur that causes an incorrect job submission."]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }
    #[doc = "Get a reference to the value of field `file_uris` after provisioning.\nHCFS URIs of files to be placed in the working directory of each executor. Useful for naively parallel tasks."]
    pub fn file_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.file_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `jar_file_uris` after provisioning.\nHCFS URIs of jar files to add to the CLASSPATHs of the Python driver and tasks."]
    pub fn jar_file_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.jar_file_uris", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `main_python_file_uri` after provisioning.\nThe HCFS URI of the main Python file to use as the driver. Must be a .py file."]
    pub fn main_python_file_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.main_python_file_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `python_file_uris` after provisioning.\nHCFS file URIs of Python files to pass to the PySpark framework. Supported file types: .py, .egg, and .zip."]
    pub fn python_file_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.python_file_uris", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocGdcSparkApplicationSparkApplicationConfigEl {
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
impl DataprocGdcSparkApplicationSparkApplicationConfigEl {
    #[doc = "Set the field `archive_uris`.\nHCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: '.jar', '.tar', '.tar.gz', '.tgz', and '.zip'."]
    pub fn set_archive_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.archive_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `args`.\nThe arguments to pass to the driver. Do not include arguments that can be set as application properties, such as '--conf', since a collision can occur that causes an incorrect application submission."]
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
    #[doc = "Set the field `main_class`.\nThe name of the driver main class. The jar file that contains the class must be in the classpath or specified in 'jar_file_uris'."]
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
impl ToListMappable for DataprocGdcSparkApplicationSparkApplicationConfigEl {
    type O = BlockAssignable<DataprocGdcSparkApplicationSparkApplicationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocGdcSparkApplicationSparkApplicationConfigEl {}
impl BuildDataprocGdcSparkApplicationSparkApplicationConfigEl {
    pub fn build(self) -> DataprocGdcSparkApplicationSparkApplicationConfigEl {
        DataprocGdcSparkApplicationSparkApplicationConfigEl {
            archive_uris: core::default::Default::default(),
            args: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            jar_file_uris: core::default::Default::default(),
            main_class: core::default::Default::default(),
            main_jar_file_uri: core::default::Default::default(),
        }
    }
}
pub struct DataprocGdcSparkApplicationSparkApplicationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocGdcSparkApplicationSparkApplicationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocGdcSparkApplicationSparkApplicationConfigElRef {
        DataprocGdcSparkApplicationSparkApplicationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocGdcSparkApplicationSparkApplicationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `archive_uris` after provisioning.\nHCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: '.jar', '.tar', '.tar.gz', '.tgz', and '.zip'."]
    pub fn archive_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\nThe arguments to pass to the driver. Do not include arguments that can be set as application properties, such as '--conf', since a collision can occur that causes an incorrect application submission."]
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
    #[doc = "Get a reference to the value of field `main_class` after provisioning.\nThe name of the driver main class. The jar file that contains the class must be in the classpath or specified in 'jar_file_uris'."]
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
pub struct DataprocGdcSparkApplicationSparkRApplicationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    archive_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_uris: Option<ListField<PrimField<String>>>,
    main_r_file_uri: PrimField<String>,
}
impl DataprocGdcSparkApplicationSparkRApplicationConfigEl {
    #[doc = "Set the field `archive_uris`.\nHCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn set_archive_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.archive_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `args`.\nThe arguments to pass to the driver.  Do not include arguments, such as '--conf', that can be set as job properties, since a collision may occur that causes an incorrect job submission."]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }
    #[doc = "Set the field `file_uris`.\nHCFS URIs of files to be placed in the working directory of each executor. Useful for naively parallel tasks."]
    pub fn set_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.file_uris = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocGdcSparkApplicationSparkRApplicationConfigEl {
    type O = BlockAssignable<DataprocGdcSparkApplicationSparkRApplicationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocGdcSparkApplicationSparkRApplicationConfigEl {
    #[doc = "The HCFS URI of the main R file to use as the driver. Must be a .R file."]
    pub main_r_file_uri: PrimField<String>,
}
impl BuildDataprocGdcSparkApplicationSparkRApplicationConfigEl {
    pub fn build(self) -> DataprocGdcSparkApplicationSparkRApplicationConfigEl {
        DataprocGdcSparkApplicationSparkRApplicationConfigEl {
            archive_uris: core::default::Default::default(),
            args: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            main_r_file_uri: self.main_r_file_uri,
        }
    }
}
pub struct DataprocGdcSparkApplicationSparkRApplicationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocGdcSparkApplicationSparkRApplicationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocGdcSparkApplicationSparkRApplicationConfigElRef {
        DataprocGdcSparkApplicationSparkRApplicationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocGdcSparkApplicationSparkRApplicationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `archive_uris` after provisioning.\nHCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn archive_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\nThe arguments to pass to the driver.  Do not include arguments, such as '--conf', that can be set as job properties, since a collision may occur that causes an incorrect job submission."]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }
    #[doc = "Get a reference to the value of field `file_uris` after provisioning.\nHCFS URIs of files to be placed in the working directory of each executor. Useful for naively parallel tasks."]
    pub fn file_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.file_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `main_r_file_uri` after provisioning.\nThe HCFS URI of the main R file to use as the driver. Must be a .R file."]
    pub fn main_r_file_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.main_r_file_uri", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocGdcSparkApplicationSparkSqlApplicationConfigElQueryListEl {
    queries: ListField<PrimField<String>>,
}
impl DataprocGdcSparkApplicationSparkSqlApplicationConfigElQueryListEl {}
impl ToListMappable for DataprocGdcSparkApplicationSparkSqlApplicationConfigElQueryListEl {
    type O = BlockAssignable<DataprocGdcSparkApplicationSparkSqlApplicationConfigElQueryListEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocGdcSparkApplicationSparkSqlApplicationConfigElQueryListEl {
    #[doc = "The queries to run."]
    pub queries: ListField<PrimField<String>>,
}
impl BuildDataprocGdcSparkApplicationSparkSqlApplicationConfigElQueryListEl {
    pub fn build(self) -> DataprocGdcSparkApplicationSparkSqlApplicationConfigElQueryListEl {
        DataprocGdcSparkApplicationSparkSqlApplicationConfigElQueryListEl {
            queries: self.queries,
        }
    }
}
pub struct DataprocGdcSparkApplicationSparkSqlApplicationConfigElQueryListElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocGdcSparkApplicationSparkSqlApplicationConfigElQueryListElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocGdcSparkApplicationSparkSqlApplicationConfigElQueryListElRef {
        DataprocGdcSparkApplicationSparkSqlApplicationConfigElQueryListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocGdcSparkApplicationSparkSqlApplicationConfigElQueryListElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `queries` after provisioning.\nThe queries to run."]
    pub fn queries(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.queries", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataprocGdcSparkApplicationSparkSqlApplicationConfigElDynamic {
    query_list:
        Option<DynamicBlock<DataprocGdcSparkApplicationSparkSqlApplicationConfigElQueryListEl>>,
}
#[derive(Serialize)]
pub struct DataprocGdcSparkApplicationSparkSqlApplicationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    jar_file_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_file_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    script_variables: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_list: Option<Vec<DataprocGdcSparkApplicationSparkSqlApplicationConfigElQueryListEl>>,
    dynamic: DataprocGdcSparkApplicationSparkSqlApplicationConfigElDynamic,
}
impl DataprocGdcSparkApplicationSparkSqlApplicationConfigEl {
    #[doc = "Set the field `jar_file_uris`.\nHCFS URIs of jar files to be added to the Spark CLASSPATH."]
    pub fn set_jar_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.jar_file_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `query_file_uri`.\nThe HCFS URI of the script that contains SQL queries."]
    pub fn set_query_file_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.query_file_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `script_variables`.\nMapping of query variable names to values (equivalent to the Spark SQL command: SET 'name=\"value\";')."]
    pub fn set_script_variables(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.script_variables = Some(v.into());
        self
    }
    #[doc = "Set the field `query_list`.\n"]
    pub fn set_query_list(
        mut self,
        v: impl Into<BlockAssignable<DataprocGdcSparkApplicationSparkSqlApplicationConfigElQueryListEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_list = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_list = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocGdcSparkApplicationSparkSqlApplicationConfigEl {
    type O = BlockAssignable<DataprocGdcSparkApplicationSparkSqlApplicationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocGdcSparkApplicationSparkSqlApplicationConfigEl {}
impl BuildDataprocGdcSparkApplicationSparkSqlApplicationConfigEl {
    pub fn build(self) -> DataprocGdcSparkApplicationSparkSqlApplicationConfigEl {
        DataprocGdcSparkApplicationSparkSqlApplicationConfigEl {
            jar_file_uris: core::default::Default::default(),
            query_file_uri: core::default::Default::default(),
            script_variables: core::default::Default::default(),
            query_list: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocGdcSparkApplicationSparkSqlApplicationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocGdcSparkApplicationSparkSqlApplicationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocGdcSparkApplicationSparkSqlApplicationConfigElRef {
        DataprocGdcSparkApplicationSparkSqlApplicationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocGdcSparkApplicationSparkSqlApplicationConfigElRef {
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
    #[doc = "Get a reference to the value of field `query_file_uri` after provisioning.\nThe HCFS URI of the script that contains SQL queries."]
    pub fn query_file_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_file_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `script_variables` after provisioning.\nMapping of query variable names to values (equivalent to the Spark SQL command: SET 'name=\"value\";')."]
    pub fn script_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.script_variables", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query_list` after provisioning.\n"]
    pub fn query_list(
        &self,
    ) -> ListRef<DataprocGdcSparkApplicationSparkSqlApplicationConfigElQueryListElRef> {
        ListRef::new(self.shared().clone(), format!("{}.query_list", self.base))
    }
}
#[derive(Serialize)]
pub struct DataprocGdcSparkApplicationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DataprocGdcSparkApplicationTimeoutsEl {
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
impl ToListMappable for DataprocGdcSparkApplicationTimeoutsEl {
    type O = BlockAssignable<DataprocGdcSparkApplicationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocGdcSparkApplicationTimeoutsEl {}
impl BuildDataprocGdcSparkApplicationTimeoutsEl {
    pub fn build(self) -> DataprocGdcSparkApplicationTimeoutsEl {
        DataprocGdcSparkApplicationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DataprocGdcSparkApplicationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocGdcSparkApplicationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataprocGdcSparkApplicationTimeoutsElRef {
        DataprocGdcSparkApplicationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocGdcSparkApplicationTimeoutsElRef {
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
struct DataprocGdcSparkApplicationDynamic {
    pyspark_application_config:
        Option<DynamicBlock<DataprocGdcSparkApplicationPysparkApplicationConfigEl>>,
    spark_application_config:
        Option<DynamicBlock<DataprocGdcSparkApplicationSparkApplicationConfigEl>>,
    spark_r_application_config:
        Option<DynamicBlock<DataprocGdcSparkApplicationSparkRApplicationConfigEl>>,
    spark_sql_application_config:
        Option<DynamicBlock<DataprocGdcSparkApplicationSparkSqlApplicationConfigEl>>,
}
