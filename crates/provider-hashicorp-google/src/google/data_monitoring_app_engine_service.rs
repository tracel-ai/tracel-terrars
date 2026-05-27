use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataMonitoringAppEngineServiceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    module_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataMonitoringAppEngineService_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataMonitoringAppEngineServiceData>,
}
#[derive(Clone)]
pub struct DataMonitoringAppEngineService(Rc<DataMonitoringAppEngineService_>);
impl DataMonitoringAppEngineService {
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
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nName used for UI elements listing this Service."]
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
    #[doc = "Get a reference to the value of field `module_id` after provisioning.\nThe ID of the App Engine module underlying this service. \nCorresponds to the 'moduleId' resource label for a 'gae_app'\nmonitored resource(see https://cloud.google.com/monitoring/api/resources#tag_gae_app)"]
    pub fn module_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.module_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe full resource name for this service. The syntax is:\nprojects/[PROJECT_ID]/services/[SERVICE_ID]."]
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
    #[doc = "Get a reference to the value of field `service_id` after provisioning.\nAn optional service ID to use. If not given, the server will generate a\nservice ID."]
    pub fn service_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `telemetry` after provisioning.\nConfiguration for how to query telemetry on a Service."]
    pub fn telemetry(&self) -> ListRef<DataMonitoringAppEngineServiceTelemetryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.telemetry", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_labels` after provisioning.\nLabels which have been used to annotate the service. Label keys must start\nwith a letter. Label keys and values may contain lowercase letters,\nnumbers, underscores, and dashes. Label keys and values have a maximum\nlength of 63 characters, and must be less than 128 bytes in size. Up to 64\nlabel entries may be stored. For labels which do not have a semantic value,\nthe empty string may be supplied for the label value."]
    pub fn user_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.user_labels", self.extract_ref()),
        )
    }
}
impl Referable for DataMonitoringAppEngineService {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataMonitoringAppEngineService {}
impl ToListMappable for DataMonitoringAppEngineService {
    type O = ListRef<DataMonitoringAppEngineServiceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataMonitoringAppEngineService_ {
    fn extract_datasource_type(&self) -> String {
        "google_monitoring_app_engine_service".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataMonitoringAppEngineService {
    pub tf_id: String,
    #[doc = "The ID of the App Engine module underlying this service. \nCorresponds to the 'moduleId' resource label for a 'gae_app'\nmonitored resource(see https://cloud.google.com/monitoring/api/resources#tag_gae_app)"]
    pub module_id: PrimField<String>,
}
impl BuildDataMonitoringAppEngineService {
    pub fn build(self, stack: &mut Stack) -> DataMonitoringAppEngineService {
        let out = DataMonitoringAppEngineService(Rc::new(DataMonitoringAppEngineService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataMonitoringAppEngineServiceData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                module_id: self.module_id,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataMonitoringAppEngineServiceRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMonitoringAppEngineServiceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataMonitoringAppEngineServiceRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nName used for UI elements listing this Service."]
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
    #[doc = "Get a reference to the value of field `module_id` after provisioning.\nThe ID of the App Engine module underlying this service. \nCorresponds to the 'moduleId' resource label for a 'gae_app'\nmonitored resource(see https://cloud.google.com/monitoring/api/resources#tag_gae_app)"]
    pub fn module_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.module_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe full resource name for this service. The syntax is:\nprojects/[PROJECT_ID]/services/[SERVICE_ID]."]
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
    #[doc = "Get a reference to the value of field `service_id` after provisioning.\nAn optional service ID to use. If not given, the server will generate a\nservice ID."]
    pub fn service_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `telemetry` after provisioning.\nConfiguration for how to query telemetry on a Service."]
    pub fn telemetry(&self) -> ListRef<DataMonitoringAppEngineServiceTelemetryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.telemetry", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_labels` after provisioning.\nLabels which have been used to annotate the service. Label keys must start\nwith a letter. Label keys and values may contain lowercase letters,\nnumbers, underscores, and dashes. Label keys and values have a maximum\nlength of 63 characters, and must be less than 128 bytes in size. Up to 64\nlabel entries may be stored. For labels which do not have a semantic value,\nthe empty string may be supplied for the label value."]
    pub fn user_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.user_labels", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataMonitoringAppEngineServiceTelemetryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_name: Option<PrimField<String>>,
}
impl DataMonitoringAppEngineServiceTelemetryEl {
    #[doc = "Set the field `resource_name`.\n"]
    pub fn set_resource_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataMonitoringAppEngineServiceTelemetryEl {
    type O = BlockAssignable<DataMonitoringAppEngineServiceTelemetryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMonitoringAppEngineServiceTelemetryEl {}
impl BuildDataMonitoringAppEngineServiceTelemetryEl {
    pub fn build(self) -> DataMonitoringAppEngineServiceTelemetryEl {
        DataMonitoringAppEngineServiceTelemetryEl {
            resource_name: core::default::Default::default(),
        }
    }
}
pub struct DataMonitoringAppEngineServiceTelemetryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMonitoringAppEngineServiceTelemetryElRef {
    fn new(shared: StackShared, base: String) -> DataMonitoringAppEngineServiceTelemetryElRef {
        DataMonitoringAppEngineServiceTelemetryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMonitoringAppEngineServiceTelemetryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_name` after provisioning.\n"]
    pub fn resource_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_name", self.base),
        )
    }
}
