use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataprocGdcServiceInstanceData {
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
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    service_instance_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gdce_cluster: Option<Vec<DataprocGdcServiceInstanceGdceClusterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spark_service_instance_config:
        Option<Vec<DataprocGdcServiceInstanceSparkServiceInstanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataprocGdcServiceInstanceTimeoutsEl>,
    dynamic: DataprocGdcServiceInstanceDynamic,
}
struct DataprocGdcServiceInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataprocGdcServiceInstanceData>,
}
#[derive(Clone)]
pub struct DataprocGdcServiceInstance(Rc<DataprocGdcServiceInstance_>);
impl DataprocGdcServiceInstance {
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
    #[doc = "Set the field `labels`.\nThe labels to associate with this service instance. Labels may be used for filtering and billing tracking. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\nRequested service account to associate with ServiceInstance."]
    pub fn set_service_account(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `gdce_cluster`.\n"]
    pub fn set_gdce_cluster(
        self,
        v: impl Into<BlockAssignable<DataprocGdcServiceInstanceGdceClusterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gdce_cluster = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gdce_cluster = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `spark_service_instance_config`.\n"]
    pub fn set_spark_service_instance_config(
        self,
        v: impl Into<BlockAssignable<DataprocGdcServiceInstanceSparkServiceInstanceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spark_service_instance_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .spark_service_instance_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataprocGdcServiceInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser-provided human-readable name to be used in user interfaces."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_service_account` after provisioning.\nEffective service account associated with ServiceInstance. This will be the service_account if specified. Otherwise, it will be an automatically created per-resource P4SA that also automatically has Fleet Workload. Identity bindings applied."]
    pub fn effective_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels to associate with this service instance. Labels may be used for filtering and billing tracking. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation of the resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the service instance."]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nWhether the service instance is currently reconciling. True if the current state of the resource does not match the intended state, and the system is working to reconcile them, whether or not the change was user initiated."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `requested_state` after provisioning.\nThe intended state to which the service instance is reconciling. Possible values:\n* 'CREATING'\n* 'ACTIVE'\n* 'DISCONNECTED'\n* 'DELETING'\n* 'STOPPING'\n* 'STOPPED'\n* 'STARTING'\n* 'UPDATING'\n* 'FAILED'"]
    pub fn requested_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.requested_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nRequested service account to associate with ServiceInstance."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_instance_id` after provisioning.\nId of the service instance."]
    pub fn service_instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state. Possible values:\n* 'CREATING'\n* 'ACTIVE'\n* 'DISCONNECTED'\n* 'DELETING'\n* 'STOPPING'\n* 'STOPPED'\n* 'STARTING'\n* 'UPDATING'\n* 'FAILED'"]
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
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem generated unique identifier for this service instance, formatted as UUID4."]
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
    #[doc = "Get a reference to the value of field `gdce_cluster` after provisioning.\n"]
    pub fn gdce_cluster(&self) -> ListRef<DataprocGdcServiceInstanceGdceClusterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gdce_cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spark_service_instance_config` after provisioning.\n"]
    pub fn spark_service_instance_config(
        &self,
    ) -> ListRef<DataprocGdcServiceInstanceSparkServiceInstanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spark_service_instance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocGdcServiceInstanceTimeoutsElRef {
        DataprocGdcServiceInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataprocGdcServiceInstance {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataprocGdcServiceInstance {}
impl ToListMappable for DataprocGdcServiceInstance {
    type O = ListRef<DataprocGdcServiceInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataprocGdcServiceInstance_ {
    fn extract_resource_type(&self) -> String {
        "google_dataproc_gdc_service_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataprocGdcServiceInstance {
    pub tf_id: String,
    #[doc = "Location of the resource."]
    pub location: PrimField<String>,
    #[doc = "Id of the service instance."]
    pub service_instance_id: PrimField<String>,
}
impl BuildDataprocGdcServiceInstance {
    pub fn build(self, stack: &mut Stack) -> DataprocGdcServiceInstance {
        let out = DataprocGdcServiceInstance(Rc::new(DataprocGdcServiceInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataprocGdcServiceInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                service_account: core::default::Default::default(),
                service_instance_id: self.service_instance_id,
                gdce_cluster: core::default::Default::default(),
                spark_service_instance_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DataprocGdcServiceInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocGdcServiceInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataprocGdcServiceInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser-provided human-readable name to be used in user interfaces."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_service_account` after provisioning.\nEffective service account associated with ServiceInstance. This will be the service_account if specified. Otherwise, it will be an automatically created per-resource P4SA that also automatically has Fleet Workload. Identity bindings applied."]
    pub fn effective_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels to associate with this service instance. Labels may be used for filtering and billing tracking. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation of the resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the service instance."]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nWhether the service instance is currently reconciling. True if the current state of the resource does not match the intended state, and the system is working to reconcile them, whether or not the change was user initiated."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `requested_state` after provisioning.\nThe intended state to which the service instance is reconciling. Possible values:\n* 'CREATING'\n* 'ACTIVE'\n* 'DISCONNECTED'\n* 'DELETING'\n* 'STOPPING'\n* 'STOPPED'\n* 'STARTING'\n* 'UPDATING'\n* 'FAILED'"]
    pub fn requested_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.requested_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nRequested service account to associate with ServiceInstance."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_instance_id` after provisioning.\nId of the service instance."]
    pub fn service_instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state. Possible values:\n* 'CREATING'\n* 'ACTIVE'\n* 'DISCONNECTED'\n* 'DELETING'\n* 'STOPPING'\n* 'STOPPED'\n* 'STARTING'\n* 'UPDATING'\n* 'FAILED'"]
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
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem generated unique identifier for this service instance, formatted as UUID4."]
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
    #[doc = "Get a reference to the value of field `gdce_cluster` after provisioning.\n"]
    pub fn gdce_cluster(&self) -> ListRef<DataprocGdcServiceInstanceGdceClusterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gdce_cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spark_service_instance_config` after provisioning.\n"]
    pub fn spark_service_instance_config(
        &self,
    ) -> ListRef<DataprocGdcServiceInstanceSparkServiceInstanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spark_service_instance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocGdcServiceInstanceTimeoutsElRef {
        DataprocGdcServiceInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocGdcServiceInstanceGdceClusterEl {
    gdce_cluster: PrimField<String>,
}
impl DataprocGdcServiceInstanceGdceClusterEl {}
impl ToListMappable for DataprocGdcServiceInstanceGdceClusterEl {
    type O = BlockAssignable<DataprocGdcServiceInstanceGdceClusterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocGdcServiceInstanceGdceClusterEl {
    #[doc = "Gdce cluster resource id."]
    pub gdce_cluster: PrimField<String>,
}
impl BuildDataprocGdcServiceInstanceGdceClusterEl {
    pub fn build(self) -> DataprocGdcServiceInstanceGdceClusterEl {
        DataprocGdcServiceInstanceGdceClusterEl {
            gdce_cluster: self.gdce_cluster,
        }
    }
}
pub struct DataprocGdcServiceInstanceGdceClusterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocGdcServiceInstanceGdceClusterElRef {
    fn new(shared: StackShared, base: String) -> DataprocGdcServiceInstanceGdceClusterElRef {
        DataprocGdcServiceInstanceGdceClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocGdcServiceInstanceGdceClusterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gdce_cluster` after provisioning.\nGdce cluster resource id."]
    pub fn gdce_cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gdce_cluster", self.base))
    }
}
#[derive(Serialize)]
pub struct DataprocGdcServiceInstanceSparkServiceInstanceConfigEl {}
impl DataprocGdcServiceInstanceSparkServiceInstanceConfigEl {}
impl ToListMappable for DataprocGdcServiceInstanceSparkServiceInstanceConfigEl {
    type O = BlockAssignable<DataprocGdcServiceInstanceSparkServiceInstanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocGdcServiceInstanceSparkServiceInstanceConfigEl {}
impl BuildDataprocGdcServiceInstanceSparkServiceInstanceConfigEl {
    pub fn build(self) -> DataprocGdcServiceInstanceSparkServiceInstanceConfigEl {
        DataprocGdcServiceInstanceSparkServiceInstanceConfigEl {}
    }
}
pub struct DataprocGdcServiceInstanceSparkServiceInstanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocGdcServiceInstanceSparkServiceInstanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocGdcServiceInstanceSparkServiceInstanceConfigElRef {
        DataprocGdcServiceInstanceSparkServiceInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocGdcServiceInstanceSparkServiceInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataprocGdcServiceInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DataprocGdcServiceInstanceTimeoutsEl {
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
impl ToListMappable for DataprocGdcServiceInstanceTimeoutsEl {
    type O = BlockAssignable<DataprocGdcServiceInstanceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocGdcServiceInstanceTimeoutsEl {}
impl BuildDataprocGdcServiceInstanceTimeoutsEl {
    pub fn build(self) -> DataprocGdcServiceInstanceTimeoutsEl {
        DataprocGdcServiceInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DataprocGdcServiceInstanceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocGdcServiceInstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataprocGdcServiceInstanceTimeoutsElRef {
        DataprocGdcServiceInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocGdcServiceInstanceTimeoutsElRef {
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
struct DataprocGdcServiceInstanceDynamic {
    gdce_cluster: Option<DynamicBlock<DataprocGdcServiceInstanceGdceClusterEl>>,
    spark_service_instance_config:
        Option<DynamicBlock<DataprocGdcServiceInstanceSparkServiceInstanceConfigEl>>,
}
