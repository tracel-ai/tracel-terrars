use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataplexDatascanData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    data_scan_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
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
    data: Option<Vec<DataplexDatascanDataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_discovery_spec: Option<Vec<DataplexDatascanDataDiscoverySpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_documentation_spec: Option<Vec<DataplexDatascanDataDocumentationSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_profile_spec: Option<Vec<DataplexDatascanDataProfileSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_quality_spec: Option<Vec<DataplexDatascanDataQualitySpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_identity: Option<Vec<DataplexDatascanExecutionIdentityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_spec: Option<Vec<DataplexDatascanExecutionSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataplexDatascanTimeoutsEl>,
    dynamic: DataplexDatascanDynamic,
}
struct DataplexDatascan_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataplexDatascanData>,
}
#[derive(Clone)]
pub struct DataplexDatascan(Rc<DataplexDatascan_>);
impl DataplexDatascan {
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
    #[doc = "Set the field `description`.\nDescription of the scan."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nUser friendly display name."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nUser-defined labels for the scan. A list of key->value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(self, v: impl Into<BlockAssignable<DataplexDatascanDataEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `data_discovery_spec`.\n"]
    pub fn set_data_discovery_spec(
        self,
        v: impl Into<BlockAssignable<DataplexDatascanDataDiscoverySpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_discovery_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_discovery_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `data_documentation_spec`.\n"]
    pub fn set_data_documentation_spec(
        self,
        v: impl Into<BlockAssignable<DataplexDatascanDataDocumentationSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_documentation_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_documentation_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `data_profile_spec`.\n"]
    pub fn set_data_profile_spec(
        self,
        v: impl Into<BlockAssignable<DataplexDatascanDataProfileSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_profile_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_profile_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `data_quality_spec`.\n"]
    pub fn set_data_quality_spec(
        self,
        v: impl Into<BlockAssignable<DataplexDatascanDataQualitySpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_quality_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_quality_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `execution_identity`.\n"]
    pub fn set_execution_identity(
        self,
        v: impl Into<BlockAssignable<DataplexDatascanExecutionIdentityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().execution_identity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.execution_identity = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `execution_spec`.\n"]
    pub fn set_execution_spec(
        self,
        v: impl Into<BlockAssignable<DataplexDatascanExecutionSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().execution_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.execution_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataplexDatascanTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the scan was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_scan_id` after provisioning.\nDataScan identifier. Must contain only lowercase letters, numbers and hyphens. Must start with a letter. Must end with a number or a letter."]
    pub fn data_scan_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_scan_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the scan."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser friendly display name."]
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
    #[doc = "Get a reference to the value of field `execution_status` after provisioning.\nStatus of the data scan execution."]
    pub fn execution_status(&self) -> ListRef<DataplexDatascanExecutionStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.execution_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the scan. A list of key->value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location where the data scan should reside."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the scan, of the form: projects/{project}/locations/{locationId}/dataScans/{datascan_id}, where project refers to a project_id or project_number and locationId refers to a GCP region."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent state of the DataScan."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of DataScan."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem generated globally unique ID for the scan. This ID will be different if the scan is deleted and re-created with the same name."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the scan was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> ListRef<DataplexDatascanDataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_discovery_spec` after provisioning.\n"]
    pub fn data_discovery_spec(&self) -> ListRef<DataplexDatascanDataDiscoverySpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_discovery_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_documentation_spec` after provisioning.\n"]
    pub fn data_documentation_spec(&self) -> ListRef<DataplexDatascanDataDocumentationSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_documentation_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_profile_spec` after provisioning.\n"]
    pub fn data_profile_spec(&self) -> ListRef<DataplexDatascanDataProfileSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_profile_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_quality_spec` after provisioning.\n"]
    pub fn data_quality_spec(&self) -> ListRef<DataplexDatascanDataQualitySpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_quality_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_identity` after provisioning.\n"]
    pub fn execution_identity(&self) -> ListRef<DataplexDatascanExecutionIdentityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.execution_identity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_spec` after provisioning.\n"]
    pub fn execution_spec(&self) -> ListRef<DataplexDatascanExecutionSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.execution_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataplexDatascanTimeoutsElRef {
        DataplexDatascanTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataplexDatascan {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataplexDatascan {}
impl ToListMappable for DataplexDatascan {
    type O = ListRef<DataplexDatascanRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataplexDatascan_ {
    fn extract_resource_type(&self) -> String {
        "google_dataplex_datascan".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataplexDatascan {
    pub tf_id: String,
    #[doc = "DataScan identifier. Must contain only lowercase letters, numbers and hyphens. Must start with a letter. Must end with a number or a letter."]
    pub data_scan_id: PrimField<String>,
    #[doc = "The location where the data scan should reside."]
    pub location: PrimField<String>,
}
impl BuildDataplexDatascan {
    pub fn build(self, stack: &mut Stack) -> DataplexDatascan {
        let out = DataplexDatascan(Rc::new(DataplexDatascan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataplexDatascanData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                data_scan_id: self.data_scan_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                data: core::default::Default::default(),
                data_discovery_spec: core::default::Default::default(),
                data_documentation_spec: core::default::Default::default(),
                data_profile_spec: core::default::Default::default(),
                data_quality_spec: core::default::Default::default(),
                execution_identity: core::default::Default::default(),
                execution_spec: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DataplexDatascanRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataplexDatascanRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the scan was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_scan_id` after provisioning.\nDataScan identifier. Must contain only lowercase letters, numbers and hyphens. Must start with a letter. Must end with a number or a letter."]
    pub fn data_scan_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_scan_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the scan."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser friendly display name."]
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
    #[doc = "Get a reference to the value of field `execution_status` after provisioning.\nStatus of the data scan execution."]
    pub fn execution_status(&self) -> ListRef<DataplexDatascanExecutionStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.execution_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the scan. A list of key->value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location where the data scan should reside."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the scan, of the form: projects/{project}/locations/{locationId}/dataScans/{datascan_id}, where project refers to a project_id or project_number and locationId refers to a GCP region."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent state of the DataScan."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of DataScan."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem generated globally unique ID for the scan. This ID will be different if the scan is deleted and re-created with the same name."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the scan was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> ListRef<DataplexDatascanDataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_discovery_spec` after provisioning.\n"]
    pub fn data_discovery_spec(&self) -> ListRef<DataplexDatascanDataDiscoverySpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_discovery_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_documentation_spec` after provisioning.\n"]
    pub fn data_documentation_spec(&self) -> ListRef<DataplexDatascanDataDocumentationSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_documentation_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_profile_spec` after provisioning.\n"]
    pub fn data_profile_spec(&self) -> ListRef<DataplexDatascanDataProfileSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_profile_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_quality_spec` after provisioning.\n"]
    pub fn data_quality_spec(&self) -> ListRef<DataplexDatascanDataQualitySpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_quality_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_identity` after provisioning.\n"]
    pub fn execution_identity(&self) -> ListRef<DataplexDatascanExecutionIdentityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.execution_identity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_spec` after provisioning.\n"]
    pub fn execution_spec(&self) -> ListRef<DataplexDatascanExecutionSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.execution_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataplexDatascanTimeoutsElRef {
        DataplexDatascanTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanExecutionStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    latest_job_end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    latest_job_start_time: Option<PrimField<String>>,
}
impl DataplexDatascanExecutionStatusEl {
    #[doc = "Set the field `latest_job_end_time`.\n"]
    pub fn set_latest_job_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.latest_job_end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `latest_job_start_time`.\n"]
    pub fn set_latest_job_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.latest_job_start_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexDatascanExecutionStatusEl {
    type O = BlockAssignable<DataplexDatascanExecutionStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanExecutionStatusEl {}
impl BuildDataplexDatascanExecutionStatusEl {
    pub fn build(self) -> DataplexDatascanExecutionStatusEl {
        DataplexDatascanExecutionStatusEl {
            latest_job_end_time: core::default::Default::default(),
            latest_job_start_time: core::default::Default::default(),
        }
    }
}
pub struct DataplexDatascanExecutionStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanExecutionStatusElRef {
    fn new(shared: StackShared, base: String) -> DataplexDatascanExecutionStatusElRef {
        DataplexDatascanExecutionStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanExecutionStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `latest_job_end_time` after provisioning.\n"]
    pub fn latest_job_end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_job_end_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `latest_job_start_time` after provisioning.\n"]
    pub fn latest_job_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_job_start_time", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanDataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    entity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<PrimField<String>>,
}
impl DataplexDatascanDataEl {
    #[doc = "Set the field `entity`.\nThe Dataplex entity that represents the data source(e.g. BigQuery table) for Datascan."]
    pub fn set_entity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.entity = Some(v.into());
        self
    }
    #[doc = "Set the field `resource`.\nThe service-qualified full resource name of the cloud resource for a DataScan job to scan against. The field could be:\nCloud Storage bucket (//storage.googleapis.com/projects/PROJECT_ID/buckets/BUCKET_ID) for DataDiscoveryScan OR BigQuery table of type \"TABLE\" (/bigquery.googleapis.com/projects/PROJECT_ID/datasets/DATASET_ID/tables/TABLE_ID) for DataProfileScan/DataQualityScan."]
    pub fn set_resource(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexDatascanDataEl {
    type O = BlockAssignable<DataplexDatascanDataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataEl {}
impl BuildDataplexDatascanDataEl {
    pub fn build(self) -> DataplexDatascanDataEl {
        DataplexDatascanDataEl {
            entity: core::default::Default::default(),
            resource: core::default::Default::default(),
        }
    }
}
pub struct DataplexDatascanDataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataElRef {
    fn new(shared: StackShared, base: String) -> DataplexDatascanDataElRef {
        DataplexDatascanDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `entity` after provisioning.\nThe Dataplex entity that represents the data source(e.g. BigQuery table) for Datascan."]
    pub fn entity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.entity", self.base))
    }
    #[doc = "Get a reference to the value of field `resource` after provisioning.\nThe service-qualified full resource name of the cloud resource for a DataScan job to scan against. The field could be:\nCloud Storage bucket (//storage.googleapis.com/projects/PROJECT_ID/buckets/BUCKET_ID) for DataDiscoveryScan OR BigQuery table of type \"TABLE\" (/bigquery.googleapis.com/projects/PROJECT_ID/datasets/DATASET_ID/tables/TABLE_ID) for DataProfileScan/DataQualityScan."]
    pub fn resource(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanDataDiscoverySpecElBigqueryPublishingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_type: Option<PrimField<String>>,
}
impl DataplexDatascanDataDiscoverySpecElBigqueryPublishingConfigEl {
    #[doc = "Set the field `connection`.\nThe BigQuery connection used to create BigLake tables. Must be in the form 'projects/{projectId}/locations/{locationId}/connections/{connection_id}'."]
    pub fn set_connection(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe location of the BigQuery dataset to publish BigLake external or non-BigLake external tables to."]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe project of the BigQuery dataset to publish BigLake external or non-BigLake external tables to. If not specified, the project of the Cloud Storage bucket will be used. The format is \"projects/{project_id_or_number}\"."]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }
    #[doc = "Set the field `table_type`.\nDetermines whether to publish discovered tables as BigLake external tables or non-BigLake external tables. Possible values: [\"TABLE_TYPE_UNSPECIFIED\", \"EXTERNAL\", \"BIGLAKE\"]"]
    pub fn set_table_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.table_type = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexDatascanDataDiscoverySpecElBigqueryPublishingConfigEl {
    type O = BlockAssignable<DataplexDatascanDataDiscoverySpecElBigqueryPublishingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataDiscoverySpecElBigqueryPublishingConfigEl {}
impl BuildDataplexDatascanDataDiscoverySpecElBigqueryPublishingConfigEl {
    pub fn build(self) -> DataplexDatascanDataDiscoverySpecElBigqueryPublishingConfigEl {
        DataplexDatascanDataDiscoverySpecElBigqueryPublishingConfigEl {
            connection: core::default::Default::default(),
            location: core::default::Default::default(),
            project: core::default::Default::default(),
            table_type: core::default::Default::default(),
        }
    }
}
pub struct DataplexDatascanDataDiscoverySpecElBigqueryPublishingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataDiscoverySpecElBigqueryPublishingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanDataDiscoverySpecElBigqueryPublishingConfigElRef {
        DataplexDatascanDataDiscoverySpecElBigqueryPublishingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataDiscoverySpecElBigqueryPublishingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection` after provisioning.\nThe BigQuery connection used to create BigLake tables. Must be in the form 'projects/{projectId}/locations/{locationId}/connections/{connection_id}'."]
    pub fn connection(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the BigQuery dataset to publish BigLake external or non-BigLake external tables to."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project of the BigQuery dataset to publish BigLake external or non-BigLake external tables to. If not specified, the project of the Cloud Storage bucket will be used. The format is \"projects/{project_id_or_number}\"."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
    #[doc = "Get a reference to the value of field `table_type` after provisioning.\nDetermines whether to publish discovered tables as BigLake external tables or non-BigLake external tables. Possible values: [\"TABLE_TYPE_UNSPECIFIED\", \"EXTERNAL\", \"BIGLAKE\"]"]
    pub fn table_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanDataDiscoverySpecElStorageConfigElCsvOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delimiter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_rows: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quote: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    type_inference_disabled: Option<PrimField<bool>>,
}
impl DataplexDatascanDataDiscoverySpecElStorageConfigElCsvOptionsEl {
    #[doc = "Set the field `delimiter`.\nThe delimiter that is used to separate values. The default is ',' (comma)."]
    pub fn set_delimiter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delimiter = Some(v.into());
        self
    }
    #[doc = "Set the field `encoding`.\nThe character encoding of the data. The default is UTF-8."]
    pub fn set_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encoding = Some(v.into());
        self
    }
    #[doc = "Set the field `header_rows`.\nThe number of rows to interpret as header rows that should be skipped when reading data rows."]
    pub fn set_header_rows(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.header_rows = Some(v.into());
        self
    }
    #[doc = "Set the field `quote`.\nThe character used to quote column values. Accepts '\"' (double quotation mark) or ''' (single quotation mark). If unspecified, defaults to '\"' (double quotation mark)."]
    pub fn set_quote(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.quote = Some(v.into());
        self
    }
    #[doc = "Set the field `type_inference_disabled`.\nWhether to disable the inference of data types for CSV data. If true, all columns are registered as strings."]
    pub fn set_type_inference_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.type_inference_disabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexDatascanDataDiscoverySpecElStorageConfigElCsvOptionsEl {
    type O = BlockAssignable<DataplexDatascanDataDiscoverySpecElStorageConfigElCsvOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataDiscoverySpecElStorageConfigElCsvOptionsEl {}
impl BuildDataplexDatascanDataDiscoverySpecElStorageConfigElCsvOptionsEl {
    pub fn build(self) -> DataplexDatascanDataDiscoverySpecElStorageConfigElCsvOptionsEl {
        DataplexDatascanDataDiscoverySpecElStorageConfigElCsvOptionsEl {
            delimiter: core::default::Default::default(),
            encoding: core::default::Default::default(),
            header_rows: core::default::Default::default(),
            quote: core::default::Default::default(),
            type_inference_disabled: core::default::Default::default(),
        }
    }
}
pub struct DataplexDatascanDataDiscoverySpecElStorageConfigElCsvOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataDiscoverySpecElStorageConfigElCsvOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanDataDiscoverySpecElStorageConfigElCsvOptionsElRef {
        DataplexDatascanDataDiscoverySpecElStorageConfigElCsvOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataDiscoverySpecElStorageConfigElCsvOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delimiter` after provisioning.\nThe delimiter that is used to separate values. The default is ',' (comma)."]
    pub fn delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delimiter", self.base))
    }
    #[doc = "Get a reference to the value of field `encoding` after provisioning.\nThe character encoding of the data. The default is UTF-8."]
    pub fn encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }
    #[doc = "Get a reference to the value of field `header_rows` after provisioning.\nThe number of rows to interpret as header rows that should be skipped when reading data rows."]
    pub fn header_rows(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_rows", self.base))
    }
    #[doc = "Get a reference to the value of field `quote` after provisioning.\nThe character used to quote column values. Accepts '\"' (double quotation mark) or ''' (single quotation mark). If unspecified, defaults to '\"' (double quotation mark)."]
    pub fn quote(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.quote", self.base))
    }
    #[doc = "Get a reference to the value of field `type_inference_disabled` after provisioning.\nWhether to disable the inference of data types for CSV data. If true, all columns are registered as strings."]
    pub fn type_inference_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type_inference_disabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanDataDiscoverySpecElStorageConfigElJsonOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    type_inference_disabled: Option<PrimField<bool>>,
}
impl DataplexDatascanDataDiscoverySpecElStorageConfigElJsonOptionsEl {
    #[doc = "Set the field `encoding`.\nThe character encoding of the data. The default is UTF-8."]
    pub fn set_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encoding = Some(v.into());
        self
    }
    #[doc = "Set the field `type_inference_disabled`.\nWhether to disable the inference of data types for JSON data. If true, all columns are registered as their primitive types (strings, number, or boolean)."]
    pub fn set_type_inference_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.type_inference_disabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexDatascanDataDiscoverySpecElStorageConfigElJsonOptionsEl {
    type O = BlockAssignable<DataplexDatascanDataDiscoverySpecElStorageConfigElJsonOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataDiscoverySpecElStorageConfigElJsonOptionsEl {}
impl BuildDataplexDatascanDataDiscoverySpecElStorageConfigElJsonOptionsEl {
    pub fn build(self) -> DataplexDatascanDataDiscoverySpecElStorageConfigElJsonOptionsEl {
        DataplexDatascanDataDiscoverySpecElStorageConfigElJsonOptionsEl {
            encoding: core::default::Default::default(),
            type_inference_disabled: core::default::Default::default(),
        }
    }
}
pub struct DataplexDatascanDataDiscoverySpecElStorageConfigElJsonOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataDiscoverySpecElStorageConfigElJsonOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanDataDiscoverySpecElStorageConfigElJsonOptionsElRef {
        DataplexDatascanDataDiscoverySpecElStorageConfigElJsonOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataDiscoverySpecElStorageConfigElJsonOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `encoding` after provisioning.\nThe character encoding of the data. The default is UTF-8."]
    pub fn encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }
    #[doc = "Get a reference to the value of field `type_inference_disabled` after provisioning.\nWhether to disable the inference of data types for JSON data. If true, all columns are registered as their primitive types (strings, number, or boolean)."]
    pub fn type_inference_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type_inference_disabled", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataplexDatascanDataDiscoverySpecElStorageConfigElDynamic {
    csv_options:
        Option<DynamicBlock<DataplexDatascanDataDiscoverySpecElStorageConfigElCsvOptionsEl>>,
    json_options:
        Option<DynamicBlock<DataplexDatascanDataDiscoverySpecElStorageConfigElJsonOptionsEl>>,
}
#[derive(Serialize)]
pub struct DataplexDatascanDataDiscoverySpecElStorageConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_patterns: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_patterns: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    csv_options: Option<Vec<DataplexDatascanDataDiscoverySpecElStorageConfigElCsvOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json_options: Option<Vec<DataplexDatascanDataDiscoverySpecElStorageConfigElJsonOptionsEl>>,
    dynamic: DataplexDatascanDataDiscoverySpecElStorageConfigElDynamic,
}
impl DataplexDatascanDataDiscoverySpecElStorageConfigEl {
    #[doc = "Set the field `exclude_patterns`.\nDefines the data to exclude during discovery. Provide a list of patterns that identify the data to exclude. For Cloud Storage bucket assets, these patterns are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these patterns are interpreted as patterns to match table names."]
    pub fn set_exclude_patterns(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclude_patterns = Some(v.into());
        self
    }
    #[doc = "Set the field `include_patterns`.\nDefines the data to include during discovery when only a subset of the data should be considered. Provide a list of patterns that identify the data to include. For Cloud Storage bucket assets, these patterns are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these patterns are interpreted as patterns to match table names."]
    pub fn set_include_patterns(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.include_patterns = Some(v.into());
        self
    }
    #[doc = "Set the field `csv_options`.\n"]
    pub fn set_csv_options(
        mut self,
        v: impl Into<BlockAssignable<DataplexDatascanDataDiscoverySpecElStorageConfigElCsvOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.csv_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.csv_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `json_options`.\n"]
    pub fn set_json_options(
        mut self,
        v: impl Into<BlockAssignable<DataplexDatascanDataDiscoverySpecElStorageConfigElJsonOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.json_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.json_options = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataplexDatascanDataDiscoverySpecElStorageConfigEl {
    type O = BlockAssignable<DataplexDatascanDataDiscoverySpecElStorageConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataDiscoverySpecElStorageConfigEl {}
impl BuildDataplexDatascanDataDiscoverySpecElStorageConfigEl {
    pub fn build(self) -> DataplexDatascanDataDiscoverySpecElStorageConfigEl {
        DataplexDatascanDataDiscoverySpecElStorageConfigEl {
            exclude_patterns: core::default::Default::default(),
            include_patterns: core::default::Default::default(),
            csv_options: core::default::Default::default(),
            json_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataplexDatascanDataDiscoverySpecElStorageConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataDiscoverySpecElStorageConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanDataDiscoverySpecElStorageConfigElRef {
        DataplexDatascanDataDiscoverySpecElStorageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataDiscoverySpecElStorageConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exclude_patterns` after provisioning.\nDefines the data to exclude during discovery. Provide a list of patterns that identify the data to exclude. For Cloud Storage bucket assets, these patterns are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these patterns are interpreted as patterns to match table names."]
    pub fn exclude_patterns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_patterns", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_patterns` after provisioning.\nDefines the data to include during discovery when only a subset of the data should be considered. Provide a list of patterns that identify the data to include. For Cloud Storage bucket assets, these patterns are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these patterns are interpreted as patterns to match table names."]
    pub fn include_patterns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_patterns", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `csv_options` after provisioning.\n"]
    pub fn csv_options(
        &self,
    ) -> ListRef<DataplexDatascanDataDiscoverySpecElStorageConfigElCsvOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.csv_options", self.base))
    }
    #[doc = "Get a reference to the value of field `json_options` after provisioning.\n"]
    pub fn json_options(
        &self,
    ) -> ListRef<DataplexDatascanDataDiscoverySpecElStorageConfigElJsonOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.json_options", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataplexDatascanDataDiscoverySpecElDynamic {
    bigquery_publishing_config:
        Option<DynamicBlock<DataplexDatascanDataDiscoverySpecElBigqueryPublishingConfigEl>>,
    storage_config: Option<DynamicBlock<DataplexDatascanDataDiscoverySpecElStorageConfigEl>>,
}
#[derive(Serialize)]
pub struct DataplexDatascanDataDiscoverySpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bigquery_publishing_config:
        Option<Vec<DataplexDatascanDataDiscoverySpecElBigqueryPublishingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_config: Option<Vec<DataplexDatascanDataDiscoverySpecElStorageConfigEl>>,
    dynamic: DataplexDatascanDataDiscoverySpecElDynamic,
}
impl DataplexDatascanDataDiscoverySpecEl {
    #[doc = "Set the field `bigquery_publishing_config`.\n"]
    pub fn set_bigquery_publishing_config(
        mut self,
        v: impl Into<BlockAssignable<DataplexDatascanDataDiscoverySpecElBigqueryPublishingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.bigquery_publishing_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.bigquery_publishing_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `storage_config`.\n"]
    pub fn set_storage_config(
        mut self,
        v: impl Into<BlockAssignable<DataplexDatascanDataDiscoverySpecElStorageConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.storage_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataplexDatascanDataDiscoverySpecEl {
    type O = BlockAssignable<DataplexDatascanDataDiscoverySpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataDiscoverySpecEl {}
impl BuildDataplexDatascanDataDiscoverySpecEl {
    pub fn build(self) -> DataplexDatascanDataDiscoverySpecEl {
        DataplexDatascanDataDiscoverySpecEl {
            bigquery_publishing_config: core::default::Default::default(),
            storage_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataplexDatascanDataDiscoverySpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataDiscoverySpecElRef {
    fn new(shared: StackShared, base: String) -> DataplexDatascanDataDiscoverySpecElRef {
        DataplexDatascanDataDiscoverySpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataDiscoverySpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bigquery_publishing_config` after provisioning.\n"]
    pub fn bigquery_publishing_config(
        &self,
    ) -> ListRef<DataplexDatascanDataDiscoverySpecElBigqueryPublishingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigquery_publishing_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\n"]
    pub fn storage_config(&self) -> ListRef<DataplexDatascanDataDiscoverySpecElStorageConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanDataDocumentationSpecEl {}
impl DataplexDatascanDataDocumentationSpecEl {}
impl ToListMappable for DataplexDatascanDataDocumentationSpecEl {
    type O = BlockAssignable<DataplexDatascanDataDocumentationSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataDocumentationSpecEl {}
impl BuildDataplexDatascanDataDocumentationSpecEl {
    pub fn build(self) -> DataplexDatascanDataDocumentationSpecEl {
        DataplexDatascanDataDocumentationSpecEl {}
    }
}
pub struct DataplexDatascanDataDocumentationSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataDocumentationSpecElRef {
    fn new(shared: StackShared, base: String) -> DataplexDatascanDataDocumentationSpecElRef {
        DataplexDatascanDataDocumentationSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataDocumentationSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanDataProfileSpecElExcludeFieldsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    field_names: Option<ListField<PrimField<String>>>,
}
impl DataplexDatascanDataProfileSpecElExcludeFieldsEl {
    #[doc = "Set the field `field_names`.\nExpected input is a list of fully qualified names of fields as in the schema.\nOnly top-level field names for nested fields are supported.\nFor instance, if 'x' is of nested field type, listing 'x' is supported but 'x.y.z' is not supported. Here 'y' and 'y.z' are nested fields of 'x'."]
    pub fn set_field_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.field_names = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexDatascanDataProfileSpecElExcludeFieldsEl {
    type O = BlockAssignable<DataplexDatascanDataProfileSpecElExcludeFieldsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataProfileSpecElExcludeFieldsEl {}
impl BuildDataplexDatascanDataProfileSpecElExcludeFieldsEl {
    pub fn build(self) -> DataplexDatascanDataProfileSpecElExcludeFieldsEl {
        DataplexDatascanDataProfileSpecElExcludeFieldsEl {
            field_names: core::default::Default::default(),
        }
    }
}
pub struct DataplexDatascanDataProfileSpecElExcludeFieldsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataProfileSpecElExcludeFieldsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanDataProfileSpecElExcludeFieldsElRef {
        DataplexDatascanDataProfileSpecElExcludeFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataProfileSpecElExcludeFieldsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `field_names` after provisioning.\nExpected input is a list of fully qualified names of fields as in the schema.\nOnly top-level field names for nested fields are supported.\nFor instance, if 'x' is of nested field type, listing 'x' is supported but 'x.y.z' is not supported. Here 'y' and 'y.z' are nested fields of 'x'."]
    pub fn field_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.field_names", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanDataProfileSpecElIncludeFieldsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    field_names: Option<ListField<PrimField<String>>>,
}
impl DataplexDatascanDataProfileSpecElIncludeFieldsEl {
    #[doc = "Set the field `field_names`.\nExpected input is a list of fully qualified names of fields as in the schema.\nOnly top-level field names for nested fields are supported.\nFor instance, if 'x' is of nested field type, listing 'x' is supported but 'x.y.z' is not supported. Here 'y' and 'y.z' are nested fields of 'x'."]
    pub fn set_field_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.field_names = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexDatascanDataProfileSpecElIncludeFieldsEl {
    type O = BlockAssignable<DataplexDatascanDataProfileSpecElIncludeFieldsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataProfileSpecElIncludeFieldsEl {}
impl BuildDataplexDatascanDataProfileSpecElIncludeFieldsEl {
    pub fn build(self) -> DataplexDatascanDataProfileSpecElIncludeFieldsEl {
        DataplexDatascanDataProfileSpecElIncludeFieldsEl {
            field_names: core::default::Default::default(),
        }
    }
}
pub struct DataplexDatascanDataProfileSpecElIncludeFieldsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataProfileSpecElIncludeFieldsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanDataProfileSpecElIncludeFieldsElRef {
        DataplexDatascanDataProfileSpecElIncludeFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataProfileSpecElIncludeFieldsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `field_names` after provisioning.\nExpected input is a list of fully qualified names of fields as in the schema.\nOnly top-level field names for nested fields are supported.\nFor instance, if 'x' is of nested field type, listing 'x' is supported but 'x.y.z' is not supported. Here 'y' and 'y.z' are nested fields of 'x'."]
    pub fn field_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.field_names", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanDataProfileSpecElPostScanActionsElBigqueryExportEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    results_table: Option<PrimField<String>>,
}
impl DataplexDatascanDataProfileSpecElPostScanActionsElBigqueryExportEl {
    #[doc = "Set the field `results_table`.\nThe BigQuery table to export DataProfileScan results to.\nFormat://bigquery.googleapis.com/projects/PROJECT_ID/datasets/DATASET_ID/tables/TABLE_ID"]
    pub fn set_results_table(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.results_table = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexDatascanDataProfileSpecElPostScanActionsElBigqueryExportEl {
    type O = BlockAssignable<DataplexDatascanDataProfileSpecElPostScanActionsElBigqueryExportEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataProfileSpecElPostScanActionsElBigqueryExportEl {}
impl BuildDataplexDatascanDataProfileSpecElPostScanActionsElBigqueryExportEl {
    pub fn build(self) -> DataplexDatascanDataProfileSpecElPostScanActionsElBigqueryExportEl {
        DataplexDatascanDataProfileSpecElPostScanActionsElBigqueryExportEl {
            results_table: core::default::Default::default(),
        }
    }
}
pub struct DataplexDatascanDataProfileSpecElPostScanActionsElBigqueryExportElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataProfileSpecElPostScanActionsElBigqueryExportElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanDataProfileSpecElPostScanActionsElBigqueryExportElRef {
        DataplexDatascanDataProfileSpecElPostScanActionsElBigqueryExportElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataProfileSpecElPostScanActionsElBigqueryExportElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `results_table` after provisioning.\nThe BigQuery table to export DataProfileScan results to.\nFormat://bigquery.googleapis.com/projects/PROJECT_ID/datasets/DATASET_ID/tables/TABLE_ID"]
    pub fn results_table(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.results_table", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataplexDatascanDataProfileSpecElPostScanActionsElDynamic {
    bigquery_export:
        Option<DynamicBlock<DataplexDatascanDataProfileSpecElPostScanActionsElBigqueryExportEl>>,
}
#[derive(Serialize)]
pub struct DataplexDatascanDataProfileSpecElPostScanActionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bigquery_export:
        Option<Vec<DataplexDatascanDataProfileSpecElPostScanActionsElBigqueryExportEl>>,
    dynamic: DataplexDatascanDataProfileSpecElPostScanActionsElDynamic,
}
impl DataplexDatascanDataProfileSpecElPostScanActionsEl {
    #[doc = "Set the field `bigquery_export`.\n"]
    pub fn set_bigquery_export(
        mut self,
        v: impl Into<
            BlockAssignable<DataplexDatascanDataProfileSpecElPostScanActionsElBigqueryExportEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.bigquery_export = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.bigquery_export = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataplexDatascanDataProfileSpecElPostScanActionsEl {
    type O = BlockAssignable<DataplexDatascanDataProfileSpecElPostScanActionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataProfileSpecElPostScanActionsEl {}
impl BuildDataplexDatascanDataProfileSpecElPostScanActionsEl {
    pub fn build(self) -> DataplexDatascanDataProfileSpecElPostScanActionsEl {
        DataplexDatascanDataProfileSpecElPostScanActionsEl {
            bigquery_export: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataplexDatascanDataProfileSpecElPostScanActionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataProfileSpecElPostScanActionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanDataProfileSpecElPostScanActionsElRef {
        DataplexDatascanDataProfileSpecElPostScanActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataProfileSpecElPostScanActionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bigquery_export` after provisioning.\n"]
    pub fn bigquery_export(
        &self,
    ) -> ListRef<DataplexDatascanDataProfileSpecElPostScanActionsElBigqueryExportElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigquery_export", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataplexDatascanDataProfileSpecElDynamic {
    exclude_fields: Option<DynamicBlock<DataplexDatascanDataProfileSpecElExcludeFieldsEl>>,
    include_fields: Option<DynamicBlock<DataplexDatascanDataProfileSpecElIncludeFieldsEl>>,
    post_scan_actions: Option<DynamicBlock<DataplexDatascanDataProfileSpecElPostScanActionsEl>>,
}
#[derive(Serialize)]
pub struct DataplexDatascanDataProfileSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_publishing_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    row_filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sampling_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_fields: Option<Vec<DataplexDatascanDataProfileSpecElExcludeFieldsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_fields: Option<Vec<DataplexDatascanDataProfileSpecElIncludeFieldsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_scan_actions: Option<Vec<DataplexDatascanDataProfileSpecElPostScanActionsEl>>,
    dynamic: DataplexDatascanDataProfileSpecElDynamic,
}
impl DataplexDatascanDataProfileSpecEl {
    #[doc = "Set the field `catalog_publishing_enabled`.\nIf set, the latest DataScan job result will be published to Dataplex Catalog."]
    pub fn set_catalog_publishing_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.catalog_publishing_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `row_filter`.\nA filter applied to all rows in a single DataScan job. The filter needs to be a valid SQL expression for a WHERE clause in BigQuery standard SQL syntax. Example: col1 >= 0 AND col2 < 10"]
    pub fn set_row_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.row_filter = Some(v.into());
        self
    }
    #[doc = "Set the field `sampling_percent`.\nThe percentage of the records to be selected from the dataset for DataScan.\nValue can range between 0.0 and 100.0 with up to 3 significant decimal digits.\nSampling is not applied if 'sampling_percent' is not specified, 0 or 100."]
    pub fn set_sampling_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.sampling_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `exclude_fields`.\n"]
    pub fn set_exclude_fields(
        mut self,
        v: impl Into<BlockAssignable<DataplexDatascanDataProfileSpecElExcludeFieldsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exclude_fields = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exclude_fields = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `include_fields`.\n"]
    pub fn set_include_fields(
        mut self,
        v: impl Into<BlockAssignable<DataplexDatascanDataProfileSpecElIncludeFieldsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.include_fields = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.include_fields = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `post_scan_actions`.\n"]
    pub fn set_post_scan_actions(
        mut self,
        v: impl Into<BlockAssignable<DataplexDatascanDataProfileSpecElPostScanActionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.post_scan_actions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.post_scan_actions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataplexDatascanDataProfileSpecEl {
    type O = BlockAssignable<DataplexDatascanDataProfileSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataProfileSpecEl {}
impl BuildDataplexDatascanDataProfileSpecEl {
    pub fn build(self) -> DataplexDatascanDataProfileSpecEl {
        DataplexDatascanDataProfileSpecEl {
            catalog_publishing_enabled: core::default::Default::default(),
            row_filter: core::default::Default::default(),
            sampling_percent: core::default::Default::default(),
            exclude_fields: core::default::Default::default(),
            include_fields: core::default::Default::default(),
            post_scan_actions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataplexDatascanDataProfileSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataProfileSpecElRef {
    fn new(shared: StackShared, base: String) -> DataplexDatascanDataProfileSpecElRef {
        DataplexDatascanDataProfileSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataProfileSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `catalog_publishing_enabled` after provisioning.\nIf set, the latest DataScan job result will be published to Dataplex Catalog."]
    pub fn catalog_publishing_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.catalog_publishing_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `row_filter` after provisioning.\nA filter applied to all rows in a single DataScan job. The filter needs to be a valid SQL expression for a WHERE clause in BigQuery standard SQL syntax. Example: col1 >= 0 AND col2 < 10"]
    pub fn row_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.row_filter", self.base))
    }
    #[doc = "Get a reference to the value of field `sampling_percent` after provisioning.\nThe percentage of the records to be selected from the dataset for DataScan.\nValue can range between 0.0 and 100.0 with up to 3 significant decimal digits.\nSampling is not applied if 'sampling_percent' is not specified, 0 or 100."]
    pub fn sampling_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sampling_percent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exclude_fields` after provisioning.\n"]
    pub fn exclude_fields(&self) -> ListRef<DataplexDatascanDataProfileSpecElExcludeFieldsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_fields", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_fields` after provisioning.\n"]
    pub fn include_fields(&self) -> ListRef<DataplexDatascanDataProfileSpecElIncludeFieldsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_fields", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `post_scan_actions` after provisioning.\n"]
    pub fn post_scan_actions(
        &self,
    ) -> ListRef<DataplexDatascanDataProfileSpecElPostScanActionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.post_scan_actions", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanDataQualitySpecElPostScanActionsElBigqueryExportEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    results_table: Option<PrimField<String>>,
}
impl DataplexDatascanDataQualitySpecElPostScanActionsElBigqueryExportEl {
    #[doc = "Set the field `results_table`.\nThe BigQuery table to export DataQualityScan results to.\nFormat://bigquery.googleapis.com/projects/PROJECT_ID/datasets/DATASET_ID/tables/TABLE_ID"]
    pub fn set_results_table(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.results_table = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexDatascanDataQualitySpecElPostScanActionsElBigqueryExportEl {
    type O = BlockAssignable<DataplexDatascanDataQualitySpecElPostScanActionsElBigqueryExportEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataQualitySpecElPostScanActionsElBigqueryExportEl {}
impl BuildDataplexDatascanDataQualitySpecElPostScanActionsElBigqueryExportEl {
    pub fn build(self) -> DataplexDatascanDataQualitySpecElPostScanActionsElBigqueryExportEl {
        DataplexDatascanDataQualitySpecElPostScanActionsElBigqueryExportEl {
            results_table: core::default::Default::default(),
        }
    }
}
pub struct DataplexDatascanDataQualitySpecElPostScanActionsElBigqueryExportElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataQualitySpecElPostScanActionsElBigqueryExportElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanDataQualitySpecElPostScanActionsElBigqueryExportElRef {
        DataplexDatascanDataQualitySpecElPostScanActionsElBigqueryExportElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataQualitySpecElPostScanActionsElBigqueryExportElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `results_table` after provisioning.\nThe BigQuery table to export DataQualityScan results to.\nFormat://bigquery.googleapis.com/projects/PROJECT_ID/datasets/DATASET_ID/tables/TABLE_ID"]
    pub fn results_table(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.results_table", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobEndTriggerEl {
}
impl DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobEndTriggerEl {}
impl ToListMappable
    for DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobEndTriggerEl
{
    type O = BlockAssignable<
        DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobEndTriggerEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobEndTriggerEl
{}
impl BuildDataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobEndTriggerEl {
    pub fn build(
        self,
    ) -> DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobEndTriggerEl {
        DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobEndTriggerEl {}
    }
}
pub struct DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobEndTriggerElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobEndTriggerElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobEndTriggerElRef
    {
        DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobEndTriggerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobEndTriggerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobFailureTriggerEl
{}
impl DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobFailureTriggerEl {}
impl ToListMappable
    for DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobFailureTriggerEl
{
    type O = BlockAssignable<
        DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobFailureTriggerEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobFailureTriggerEl
{}
impl
    BuildDataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobFailureTriggerEl
{
    pub fn build(
        self,
    ) -> DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobFailureTriggerEl
    {
        DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobFailureTriggerEl {}
    }
}
pub struct DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobFailureTriggerElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobFailureTriggerElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobFailureTriggerElRef
    {
        DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobFailureTriggerElRef { shared : shared , base : base . to_string () , }
    }
}
impl DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobFailureTriggerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElRecipientsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    emails: Option<ListField<PrimField<String>>>,
}
impl DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElRecipientsEl {
    #[doc = "Set the field `emails`.\nThe email recipients who will receive the DataQualityScan results report."]
    pub fn set_emails(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.emails = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElRecipientsEl
{
    type O = BlockAssignable<
        DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElRecipientsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElRecipientsEl
{}
impl BuildDataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElRecipientsEl {
    pub fn build(
        self,
    ) -> DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElRecipientsEl {
        DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElRecipientsEl {
            emails: core::default::Default::default(),
        }
    }
}
pub struct DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElRecipientsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElRecipientsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElRecipientsElRef {
        DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElRecipientsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElRecipientsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `emails` after provisioning.\nThe email recipients who will receive the DataQualityScan results report."]
    pub fn emails(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.emails", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElScoreThresholdTriggerEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    score_threshold: Option<PrimField<f64>>,
}
impl DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElScoreThresholdTriggerEl {
    #[doc = "Set the field `score_threshold`.\nThe score range is in [0,100]."]
    pub fn set_score_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.score_threshold = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElScoreThresholdTriggerEl { type O = BlockAssignable < DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElScoreThresholdTriggerEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElScoreThresholdTriggerEl
{}
impl BuildDataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElScoreThresholdTriggerEl { pub fn build (self) -> DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElScoreThresholdTriggerEl { DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElScoreThresholdTriggerEl { score_threshold : core :: default :: Default :: default () , } } }
pub struct DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElScoreThresholdTriggerElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElScoreThresholdTriggerElRef { fn new (shared : StackShared , base : String) -> DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElScoreThresholdTriggerElRef { DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElScoreThresholdTriggerElRef { shared : shared , base : base . to_string () , } } }
impl
    DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElScoreThresholdTriggerElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `score_threshold` after provisioning.\nThe score range is in [0,100]."]
    pub fn score_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.score_threshold", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElDynamic { job_end_trigger : Option < DynamicBlock < DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobEndTriggerEl >> , job_failure_trigger : Option < DynamicBlock < DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobFailureTriggerEl >> , recipients : Option < DynamicBlock < DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElRecipientsEl >> , score_threshold_trigger : Option < DynamicBlock < DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElScoreThresholdTriggerEl >> , }
#[derive(Serialize)]
pub struct DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportEl { # [serde (skip_serializing_if = "Option::is_none")] job_end_trigger : Option < Vec < DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobEndTriggerEl > > , # [serde (skip_serializing_if = "Option::is_none")] job_failure_trigger : Option < Vec < DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobFailureTriggerEl > > , # [serde (skip_serializing_if = "Option::is_none")] recipients : Option < Vec < DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElRecipientsEl > > , # [serde (skip_serializing_if = "Option::is_none")] score_threshold_trigger : Option < Vec < DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElScoreThresholdTriggerEl > > , dynamic : DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElDynamic , }
impl DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportEl {
    #[doc = "Set the field `job_end_trigger`.\n"]
    pub fn set_job_end_trigger(
        mut self,
        v : impl Into < BlockAssignable < DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobEndTriggerEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.job_end_trigger = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.job_end_trigger = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `job_failure_trigger`.\n"]
    pub fn set_job_failure_trigger(
        mut self,
        v : impl Into < BlockAssignable < DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobFailureTriggerEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.job_failure_trigger = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.job_failure_trigger = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `recipients`.\n"]
    pub fn set_recipients(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElRecipientsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.recipients = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.recipients = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `score_threshold_trigger`.\n"]
    pub fn set_score_threshold_trigger(
        mut self,
        v : impl Into < BlockAssignable < DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElScoreThresholdTriggerEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.score_threshold_trigger = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.score_threshold_trigger = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportEl {
    type O =
        BlockAssignable<DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportEl {}
impl BuildDataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportEl {
    pub fn build(self) -> DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportEl {
        DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportEl {
            job_end_trigger: core::default::Default::default(),
            job_failure_trigger: core::default::Default::default(),
            recipients: core::default::Default::default(),
            score_threshold_trigger: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElRef {
        DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `job_end_trigger` after provisioning.\n"]
    pub fn job_end_trigger(
        &self,
    ) -> ListRef<
        DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobEndTriggerElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.job_end_trigger", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `job_failure_trigger` after provisioning.\n"]    pub fn job_failure_trigger (& self) -> ListRef < DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElJobFailureTriggerElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.job_failure_trigger", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `recipients` after provisioning.\n"]
    pub fn recipients(
        &self,
    ) -> ListRef<
        DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElRecipientsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.recipients", self.base))
    }
    #[doc = "Get a reference to the value of field `score_threshold_trigger` after provisioning.\n"]    pub fn score_threshold_trigger (& self) -> ListRef < DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElScoreThresholdTriggerElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.score_threshold_trigger", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataplexDatascanDataQualitySpecElPostScanActionsElDynamic {
    bigquery_export:
        Option<DynamicBlock<DataplexDatascanDataQualitySpecElPostScanActionsElBigqueryExportEl>>,
    notification_report: Option<
        DynamicBlock<DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportEl>,
    >,
}
#[derive(Serialize)]
pub struct DataplexDatascanDataQualitySpecElPostScanActionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bigquery_export:
        Option<Vec<DataplexDatascanDataQualitySpecElPostScanActionsElBigqueryExportEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_report:
        Option<Vec<DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportEl>>,
    dynamic: DataplexDatascanDataQualitySpecElPostScanActionsElDynamic,
}
impl DataplexDatascanDataQualitySpecElPostScanActionsEl {
    #[doc = "Set the field `bigquery_export`.\n"]
    pub fn set_bigquery_export(
        mut self,
        v: impl Into<
            BlockAssignable<DataplexDatascanDataQualitySpecElPostScanActionsElBigqueryExportEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.bigquery_export = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.bigquery_export = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `notification_report`.\n"]
    pub fn set_notification_report(
        mut self,
        v: impl Into<
            BlockAssignable<DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.notification_report = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.notification_report = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataplexDatascanDataQualitySpecElPostScanActionsEl {
    type O = BlockAssignable<DataplexDatascanDataQualitySpecElPostScanActionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataQualitySpecElPostScanActionsEl {}
impl BuildDataplexDatascanDataQualitySpecElPostScanActionsEl {
    pub fn build(self) -> DataplexDatascanDataQualitySpecElPostScanActionsEl {
        DataplexDatascanDataQualitySpecElPostScanActionsEl {
            bigquery_export: core::default::Default::default(),
            notification_report: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataplexDatascanDataQualitySpecElPostScanActionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataQualitySpecElPostScanActionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanDataQualitySpecElPostScanActionsElRef {
        DataplexDatascanDataQualitySpecElPostScanActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataQualitySpecElPostScanActionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bigquery_export` after provisioning.\n"]
    pub fn bigquery_export(
        &self,
    ) -> ListRef<DataplexDatascanDataQualitySpecElPostScanActionsElBigqueryExportElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigquery_export", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `notification_report` after provisioning.\n"]
    pub fn notification_report(
        &self,
    ) -> ListRef<DataplexDatascanDataQualitySpecElPostScanActionsElNotificationReportElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_report", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanDataQualitySpecElRulesElNonNullExpectationEl {}
impl DataplexDatascanDataQualitySpecElRulesElNonNullExpectationEl {}
impl ToListMappable for DataplexDatascanDataQualitySpecElRulesElNonNullExpectationEl {
    type O = BlockAssignable<DataplexDatascanDataQualitySpecElRulesElNonNullExpectationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataQualitySpecElRulesElNonNullExpectationEl {}
impl BuildDataplexDatascanDataQualitySpecElRulesElNonNullExpectationEl {
    pub fn build(self) -> DataplexDatascanDataQualitySpecElRulesElNonNullExpectationEl {
        DataplexDatascanDataQualitySpecElRulesElNonNullExpectationEl {}
    }
}
pub struct DataplexDatascanDataQualitySpecElRulesElNonNullExpectationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataQualitySpecElRulesElNonNullExpectationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanDataQualitySpecElRulesElNonNullExpectationElRef {
        DataplexDatascanDataQualitySpecElRulesElNonNullExpectationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataQualitySpecElRulesElNonNullExpectationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanDataQualitySpecElRulesElRangeExpectationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict_max_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict_min_enabled: Option<PrimField<bool>>,
}
impl DataplexDatascanDataQualitySpecElRulesElRangeExpectationEl {
    #[doc = "Set the field `max_value`.\nThe maximum column value allowed for a row to pass this validation. At least one of minValue and maxValue need to be provided."]
    pub fn set_max_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_value = Some(v.into());
        self
    }
    #[doc = "Set the field `min_value`.\nThe minimum column value allowed for a row to pass this validation. At least one of minValue and maxValue need to be provided."]
    pub fn set_min_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_value = Some(v.into());
        self
    }
    #[doc = "Set the field `strict_max_enabled`.\nWhether each value needs to be strictly lesser than ('<') the maximum, or if equality is allowed.\nOnly relevant if a maxValue has been defined. Default = false."]
    pub fn set_strict_max_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.strict_max_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `strict_min_enabled`.\nWhether each value needs to be strictly greater than ('>') the minimum, or if equality is allowed.\nOnly relevant if a minValue has been defined. Default = false."]
    pub fn set_strict_min_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.strict_min_enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexDatascanDataQualitySpecElRulesElRangeExpectationEl {
    type O = BlockAssignable<DataplexDatascanDataQualitySpecElRulesElRangeExpectationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataQualitySpecElRulesElRangeExpectationEl {}
impl BuildDataplexDatascanDataQualitySpecElRulesElRangeExpectationEl {
    pub fn build(self) -> DataplexDatascanDataQualitySpecElRulesElRangeExpectationEl {
        DataplexDatascanDataQualitySpecElRulesElRangeExpectationEl {
            max_value: core::default::Default::default(),
            min_value: core::default::Default::default(),
            strict_max_enabled: core::default::Default::default(),
            strict_min_enabled: core::default::Default::default(),
        }
    }
}
pub struct DataplexDatascanDataQualitySpecElRulesElRangeExpectationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataQualitySpecElRulesElRangeExpectationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanDataQualitySpecElRulesElRangeExpectationElRef {
        DataplexDatascanDataQualitySpecElRulesElRangeExpectationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataQualitySpecElRulesElRangeExpectationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_value` after provisioning.\nThe maximum column value allowed for a row to pass this validation. At least one of minValue and maxValue need to be provided."]
    pub fn max_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_value", self.base))
    }
    #[doc = "Get a reference to the value of field `min_value` after provisioning.\nThe minimum column value allowed for a row to pass this validation. At least one of minValue and maxValue need to be provided."]
    pub fn min_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_value", self.base))
    }
    #[doc = "Get a reference to the value of field `strict_max_enabled` after provisioning.\nWhether each value needs to be strictly lesser than ('<') the maximum, or if equality is allowed.\nOnly relevant if a maxValue has been defined. Default = false."]
    pub fn strict_max_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.strict_max_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `strict_min_enabled` after provisioning.\nWhether each value needs to be strictly greater than ('>') the minimum, or if equality is allowed.\nOnly relevant if a minValue has been defined. Default = false."]
    pub fn strict_min_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.strict_min_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanDataQualitySpecElRulesElRegexExpectationEl {
    regex: PrimField<String>,
}
impl DataplexDatascanDataQualitySpecElRulesElRegexExpectationEl {}
impl ToListMappable for DataplexDatascanDataQualitySpecElRulesElRegexExpectationEl {
    type O = BlockAssignable<DataplexDatascanDataQualitySpecElRulesElRegexExpectationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataQualitySpecElRulesElRegexExpectationEl {
    #[doc = "A regular expression the column value is expected to match."]
    pub regex: PrimField<String>,
}
impl BuildDataplexDatascanDataQualitySpecElRulesElRegexExpectationEl {
    pub fn build(self) -> DataplexDatascanDataQualitySpecElRulesElRegexExpectationEl {
        DataplexDatascanDataQualitySpecElRulesElRegexExpectationEl { regex: self.regex }
    }
}
pub struct DataplexDatascanDataQualitySpecElRulesElRegexExpectationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataQualitySpecElRulesElRegexExpectationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanDataQualitySpecElRulesElRegexExpectationElRef {
        DataplexDatascanDataQualitySpecElRulesElRegexExpectationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataQualitySpecElRulesElRegexExpectationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `regex` after provisioning.\nA regular expression the column value is expected to match."]
    pub fn regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanDataQualitySpecElRulesElRowConditionExpectationEl {
    sql_expression: PrimField<String>,
}
impl DataplexDatascanDataQualitySpecElRulesElRowConditionExpectationEl {}
impl ToListMappable for DataplexDatascanDataQualitySpecElRulesElRowConditionExpectationEl {
    type O = BlockAssignable<DataplexDatascanDataQualitySpecElRulesElRowConditionExpectationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataQualitySpecElRulesElRowConditionExpectationEl {
    #[doc = "The SQL expression."]
    pub sql_expression: PrimField<String>,
}
impl BuildDataplexDatascanDataQualitySpecElRulesElRowConditionExpectationEl {
    pub fn build(self) -> DataplexDatascanDataQualitySpecElRulesElRowConditionExpectationEl {
        DataplexDatascanDataQualitySpecElRulesElRowConditionExpectationEl {
            sql_expression: self.sql_expression,
        }
    }
}
pub struct DataplexDatascanDataQualitySpecElRulesElRowConditionExpectationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataQualitySpecElRulesElRowConditionExpectationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanDataQualitySpecElRulesElRowConditionExpectationElRef {
        DataplexDatascanDataQualitySpecElRulesElRowConditionExpectationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataQualitySpecElRulesElRowConditionExpectationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `sql_expression` after provisioning.\nThe SQL expression."]
    pub fn sql_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sql_expression", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanDataQualitySpecElRulesElSetExpectationEl {
    values: ListField<PrimField<String>>,
}
impl DataplexDatascanDataQualitySpecElRulesElSetExpectationEl {}
impl ToListMappable for DataplexDatascanDataQualitySpecElRulesElSetExpectationEl {
    type O = BlockAssignable<DataplexDatascanDataQualitySpecElRulesElSetExpectationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataQualitySpecElRulesElSetExpectationEl {
    #[doc = "Expected values for the column value."]
    pub values: ListField<PrimField<String>>,
}
impl BuildDataplexDatascanDataQualitySpecElRulesElSetExpectationEl {
    pub fn build(self) -> DataplexDatascanDataQualitySpecElRulesElSetExpectationEl {
        DataplexDatascanDataQualitySpecElRulesElSetExpectationEl {
            values: self.values,
        }
    }
}
pub struct DataplexDatascanDataQualitySpecElRulesElSetExpectationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataQualitySpecElRulesElSetExpectationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanDataQualitySpecElRulesElSetExpectationElRef {
        DataplexDatascanDataQualitySpecElRulesElSetExpectationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataQualitySpecElRulesElSetExpectationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\nExpected values for the column value."]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanDataQualitySpecElRulesElSqlAssertionEl {
    sql_statement: PrimField<String>,
}
impl DataplexDatascanDataQualitySpecElRulesElSqlAssertionEl {}
impl ToListMappable for DataplexDatascanDataQualitySpecElRulesElSqlAssertionEl {
    type O = BlockAssignable<DataplexDatascanDataQualitySpecElRulesElSqlAssertionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataQualitySpecElRulesElSqlAssertionEl {
    #[doc = "The SQL statement."]
    pub sql_statement: PrimField<String>,
}
impl BuildDataplexDatascanDataQualitySpecElRulesElSqlAssertionEl {
    pub fn build(self) -> DataplexDatascanDataQualitySpecElRulesElSqlAssertionEl {
        DataplexDatascanDataQualitySpecElRulesElSqlAssertionEl {
            sql_statement: self.sql_statement,
        }
    }
}
pub struct DataplexDatascanDataQualitySpecElRulesElSqlAssertionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataQualitySpecElRulesElSqlAssertionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanDataQualitySpecElRulesElSqlAssertionElRef {
        DataplexDatascanDataQualitySpecElRulesElSqlAssertionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataQualitySpecElRulesElSqlAssertionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `sql_statement` after provisioning.\nThe SQL statement."]
    pub fn sql_statement(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sql_statement", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanDataQualitySpecElRulesElStatisticRangeExpectationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_value: Option<PrimField<String>>,
    statistic: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict_max_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict_min_enabled: Option<PrimField<bool>>,
}
impl DataplexDatascanDataQualitySpecElRulesElStatisticRangeExpectationEl {
    #[doc = "Set the field `max_value`.\nThe maximum column statistic value allowed for a row to pass this validation.\nAt least one of minValue and maxValue need to be provided."]
    pub fn set_max_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_value = Some(v.into());
        self
    }
    #[doc = "Set the field `min_value`.\nThe minimum column statistic value allowed for a row to pass this validation.\nAt least one of minValue and maxValue need to be provided."]
    pub fn set_min_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_value = Some(v.into());
        self
    }
    #[doc = "Set the field `strict_max_enabled`.\nWhether column statistic needs to be strictly lesser than ('<') the maximum, or if equality is allowed.\nOnly relevant if a maxValue has been defined. Default = false."]
    pub fn set_strict_max_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.strict_max_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `strict_min_enabled`.\nWhether column statistic needs to be strictly greater than ('>') the minimum, or if equality is allowed.\nOnly relevant if a minValue has been defined. Default = false."]
    pub fn set_strict_min_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.strict_min_enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexDatascanDataQualitySpecElRulesElStatisticRangeExpectationEl {
    type O = BlockAssignable<DataplexDatascanDataQualitySpecElRulesElStatisticRangeExpectationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataQualitySpecElRulesElStatisticRangeExpectationEl {
    #[doc = "column statistics. Possible values: [\"STATISTIC_UNDEFINED\", \"MEAN\", \"MIN\", \"MAX\"]"]
    pub statistic: PrimField<String>,
}
impl BuildDataplexDatascanDataQualitySpecElRulesElStatisticRangeExpectationEl {
    pub fn build(self) -> DataplexDatascanDataQualitySpecElRulesElStatisticRangeExpectationEl {
        DataplexDatascanDataQualitySpecElRulesElStatisticRangeExpectationEl {
            max_value: core::default::Default::default(),
            min_value: core::default::Default::default(),
            statistic: self.statistic,
            strict_max_enabled: core::default::Default::default(),
            strict_min_enabled: core::default::Default::default(),
        }
    }
}
pub struct DataplexDatascanDataQualitySpecElRulesElStatisticRangeExpectationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataQualitySpecElRulesElStatisticRangeExpectationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanDataQualitySpecElRulesElStatisticRangeExpectationElRef {
        DataplexDatascanDataQualitySpecElRulesElStatisticRangeExpectationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataQualitySpecElRulesElStatisticRangeExpectationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_value` after provisioning.\nThe maximum column statistic value allowed for a row to pass this validation.\nAt least one of minValue and maxValue need to be provided."]
    pub fn max_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_value", self.base))
    }
    #[doc = "Get a reference to the value of field `min_value` after provisioning.\nThe minimum column statistic value allowed for a row to pass this validation.\nAt least one of minValue and maxValue need to be provided."]
    pub fn min_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_value", self.base))
    }
    #[doc = "Get a reference to the value of field `statistic` after provisioning.\ncolumn statistics. Possible values: [\"STATISTIC_UNDEFINED\", \"MEAN\", \"MIN\", \"MAX\"]"]
    pub fn statistic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.statistic", self.base))
    }
    #[doc = "Get a reference to the value of field `strict_max_enabled` after provisioning.\nWhether column statistic needs to be strictly lesser than ('<') the maximum, or if equality is allowed.\nOnly relevant if a maxValue has been defined. Default = false."]
    pub fn strict_max_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.strict_max_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `strict_min_enabled` after provisioning.\nWhether column statistic needs to be strictly greater than ('>') the minimum, or if equality is allowed.\nOnly relevant if a minValue has been defined. Default = false."]
    pub fn strict_min_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.strict_min_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanDataQualitySpecElRulesElTableConditionExpectationEl {
    sql_expression: PrimField<String>,
}
impl DataplexDatascanDataQualitySpecElRulesElTableConditionExpectationEl {}
impl ToListMappable for DataplexDatascanDataQualitySpecElRulesElTableConditionExpectationEl {
    type O = BlockAssignable<DataplexDatascanDataQualitySpecElRulesElTableConditionExpectationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataQualitySpecElRulesElTableConditionExpectationEl {
    #[doc = "The SQL expression."]
    pub sql_expression: PrimField<String>,
}
impl BuildDataplexDatascanDataQualitySpecElRulesElTableConditionExpectationEl {
    pub fn build(self) -> DataplexDatascanDataQualitySpecElRulesElTableConditionExpectationEl {
        DataplexDatascanDataQualitySpecElRulesElTableConditionExpectationEl {
            sql_expression: self.sql_expression,
        }
    }
}
pub struct DataplexDatascanDataQualitySpecElRulesElTableConditionExpectationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataQualitySpecElRulesElTableConditionExpectationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanDataQualitySpecElRulesElTableConditionExpectationElRef {
        DataplexDatascanDataQualitySpecElRulesElTableConditionExpectationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataQualitySpecElRulesElTableConditionExpectationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `sql_expression` after provisioning.\nThe SQL expression."]
    pub fn sql_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sql_expression", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanDataQualitySpecElRulesElTemplateReferenceElValuesEl {
    name: PrimField<String>,
    value: PrimField<String>,
}
impl DataplexDatascanDataQualitySpecElRulesElTemplateReferenceElValuesEl {}
impl ToListMappable for DataplexDatascanDataQualitySpecElRulesElTemplateReferenceElValuesEl {
    type O = BlockAssignable<DataplexDatascanDataQualitySpecElRulesElTemplateReferenceElValuesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataQualitySpecElRulesElTemplateReferenceElValuesEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = "The string representation of the parameter value."]
    pub value: PrimField<String>,
}
impl BuildDataplexDatascanDataQualitySpecElRulesElTemplateReferenceElValuesEl {
    pub fn build(self) -> DataplexDatascanDataQualitySpecElRulesElTemplateReferenceElValuesEl {
        DataplexDatascanDataQualitySpecElRulesElTemplateReferenceElValuesEl {
            name: self.name,
            value: self.value,
        }
    }
}
pub struct DataplexDatascanDataQualitySpecElRulesElTemplateReferenceElValuesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataQualitySpecElRulesElTemplateReferenceElValuesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanDataQualitySpecElRulesElTemplateReferenceElValuesElRef {
        DataplexDatascanDataQualitySpecElRulesElTemplateReferenceElValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataQualitySpecElRulesElTemplateReferenceElValuesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe string representation of the parameter value."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataplexDatascanDataQualitySpecElRulesElTemplateReferenceElDynamic {
    values:
        Option<DynamicBlock<DataplexDatascanDataQualitySpecElRulesElTemplateReferenceElValuesEl>>,
}
#[derive(Serialize)]
pub struct DataplexDatascanDataQualitySpecElRulesElTemplateReferenceEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<Vec<DataplexDatascanDataQualitySpecElRulesElTemplateReferenceElValuesEl>>,
    dynamic: DataplexDatascanDataQualitySpecElRulesElTemplateReferenceElDynamic,
}
impl DataplexDatascanDataQualitySpecElRulesElTemplateReferenceEl {
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl Into<
            BlockAssignable<DataplexDatascanDataQualitySpecElRulesElTemplateReferenceElValuesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.values = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.values = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataplexDatascanDataQualitySpecElRulesElTemplateReferenceEl {
    type O = BlockAssignable<DataplexDatascanDataQualitySpecElRulesElTemplateReferenceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataQualitySpecElRulesElTemplateReferenceEl {
    #[doc = "The resource name of the template entry."]
    pub name: PrimField<String>,
}
impl BuildDataplexDatascanDataQualitySpecElRulesElTemplateReferenceEl {
    pub fn build(self) -> DataplexDatascanDataQualitySpecElRulesElTemplateReferenceEl {
        DataplexDatascanDataQualitySpecElRulesElTemplateReferenceEl {
            name: self.name,
            values: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataplexDatascanDataQualitySpecElRulesElTemplateReferenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataQualitySpecElRulesElTemplateReferenceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanDataQualitySpecElRulesElTemplateReferenceElRef {
        DataplexDatascanDataQualitySpecElRulesElTemplateReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataQualitySpecElRulesElTemplateReferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the template entry."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanDataQualitySpecElRulesElUniquenessExpectationEl {}
impl DataplexDatascanDataQualitySpecElRulesElUniquenessExpectationEl {}
impl ToListMappable for DataplexDatascanDataQualitySpecElRulesElUniquenessExpectationEl {
    type O = BlockAssignable<DataplexDatascanDataQualitySpecElRulesElUniquenessExpectationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataQualitySpecElRulesElUniquenessExpectationEl {}
impl BuildDataplexDatascanDataQualitySpecElRulesElUniquenessExpectationEl {
    pub fn build(self) -> DataplexDatascanDataQualitySpecElRulesElUniquenessExpectationEl {
        DataplexDatascanDataQualitySpecElRulesElUniquenessExpectationEl {}
    }
}
pub struct DataplexDatascanDataQualitySpecElRulesElUniquenessExpectationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataQualitySpecElRulesElUniquenessExpectationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanDataQualitySpecElRulesElUniquenessExpectationElRef {
        DataplexDatascanDataQualitySpecElRulesElUniquenessExpectationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataQualitySpecElRulesElUniquenessExpectationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct DataplexDatascanDataQualitySpecElRulesElDynamic {
    non_null_expectation:
        Option<DynamicBlock<DataplexDatascanDataQualitySpecElRulesElNonNullExpectationEl>>,
    range_expectation:
        Option<DynamicBlock<DataplexDatascanDataQualitySpecElRulesElRangeExpectationEl>>,
    regex_expectation:
        Option<DynamicBlock<DataplexDatascanDataQualitySpecElRulesElRegexExpectationEl>>,
    row_condition_expectation:
        Option<DynamicBlock<DataplexDatascanDataQualitySpecElRulesElRowConditionExpectationEl>>,
    set_expectation: Option<DynamicBlock<DataplexDatascanDataQualitySpecElRulesElSetExpectationEl>>,
    sql_assertion: Option<DynamicBlock<DataplexDatascanDataQualitySpecElRulesElSqlAssertionEl>>,
    statistic_range_expectation:
        Option<DynamicBlock<DataplexDatascanDataQualitySpecElRulesElStatisticRangeExpectationEl>>,
    table_condition_expectation:
        Option<DynamicBlock<DataplexDatascanDataQualitySpecElRulesElTableConditionExpectationEl>>,
    template_reference:
        Option<DynamicBlock<DataplexDatascanDataQualitySpecElRulesElTemplateReferenceEl>>,
    uniqueness_expectation:
        Option<DynamicBlock<DataplexDatascanDataQualitySpecElRulesElUniquenessExpectationEl>>,
}
#[derive(Serialize)]
pub struct DataplexDatascanDataQualitySpecElRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    dimension: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_null: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suspended: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    non_null_expectation: Option<Vec<DataplexDatascanDataQualitySpecElRulesElNonNullExpectationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range_expectation: Option<Vec<DataplexDatascanDataQualitySpecElRulesElRangeExpectationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex_expectation: Option<Vec<DataplexDatascanDataQualitySpecElRulesElRegexExpectationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    row_condition_expectation:
        Option<Vec<DataplexDatascanDataQualitySpecElRulesElRowConditionExpectationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    set_expectation: Option<Vec<DataplexDatascanDataQualitySpecElRulesElSetExpectationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sql_assertion: Option<Vec<DataplexDatascanDataQualitySpecElRulesElSqlAssertionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statistic_range_expectation:
        Option<Vec<DataplexDatascanDataQualitySpecElRulesElStatisticRangeExpectationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_condition_expectation:
        Option<Vec<DataplexDatascanDataQualitySpecElRulesElTableConditionExpectationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_reference: Option<Vec<DataplexDatascanDataQualitySpecElRulesElTemplateReferenceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uniqueness_expectation:
        Option<Vec<DataplexDatascanDataQualitySpecElRulesElUniquenessExpectationEl>>,
    dynamic: DataplexDatascanDataQualitySpecElRulesElDynamic,
}
impl DataplexDatascanDataQualitySpecElRulesEl {
    #[doc = "Set the field `attributes`.\nMap of attribute name and value linked to the rule."]
    pub fn set_attributes(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.attributes = Some(v.into());
        self
    }
    #[doc = "Set the field `column`.\nThe unnested column which this rule is evaluated against."]
    pub fn set_column(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.column = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nDescription of the rule.\nThe maximum length is 1,024 characters."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_null`.\nRows with null values will automatically fail a rule, unless ignoreNull is true. In that case, such null rows are trivially considered passing. Only applicable to ColumnMap rules."]
    pub fn set_ignore_null(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_null = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nA mutable name for the rule.\nThe name must contain only letters (a-z, A-Z), numbers (0-9), or hyphens (-).\nThe maximum length is 63 characters.\nMust start with a letter.\nMust end with a number or a letter."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `suspended`.\nWhether the Rule is active or suspended. Default = false."]
    pub fn set_suspended(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.suspended = Some(v.into());
        self
    }
    #[doc = "Set the field `threshold`.\nThe minimum ratio of passing_rows / total_rows required to pass this rule, with a range of [0.0, 1.0]. 0 indicates default value (i.e. 1.0)."]
    pub fn set_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `non_null_expectation`.\n"]
    pub fn set_non_null_expectation(
        mut self,
        v: impl Into<BlockAssignable<DataplexDatascanDataQualitySpecElRulesElNonNullExpectationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.non_null_expectation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.non_null_expectation = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `range_expectation`.\n"]
    pub fn set_range_expectation(
        mut self,
        v: impl Into<BlockAssignable<DataplexDatascanDataQualitySpecElRulesElRangeExpectationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.range_expectation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.range_expectation = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `regex_expectation`.\n"]
    pub fn set_regex_expectation(
        mut self,
        v: impl Into<BlockAssignable<DataplexDatascanDataQualitySpecElRulesElRegexExpectationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.regex_expectation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.regex_expectation = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `row_condition_expectation`.\n"]
    pub fn set_row_condition_expectation(
        mut self,
        v: impl Into<BlockAssignable<DataplexDatascanDataQualitySpecElRulesElRowConditionExpectationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.row_condition_expectation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.row_condition_expectation = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `set_expectation`.\n"]
    pub fn set_set_expectation(
        mut self,
        v: impl Into<BlockAssignable<DataplexDatascanDataQualitySpecElRulesElSetExpectationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.set_expectation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.set_expectation = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sql_assertion`.\n"]
    pub fn set_sql_assertion(
        mut self,
        v: impl Into<BlockAssignable<DataplexDatascanDataQualitySpecElRulesElSqlAssertionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sql_assertion = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sql_assertion = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `statistic_range_expectation`.\n"]
    pub fn set_statistic_range_expectation(
        mut self,
        v: impl Into<
            BlockAssignable<DataplexDatascanDataQualitySpecElRulesElStatisticRangeExpectationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.statistic_range_expectation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.statistic_range_expectation = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `table_condition_expectation`.\n"]
    pub fn set_table_condition_expectation(
        mut self,
        v: impl Into<
            BlockAssignable<DataplexDatascanDataQualitySpecElRulesElTableConditionExpectationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.table_condition_expectation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.table_condition_expectation = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `template_reference`.\n"]
    pub fn set_template_reference(
        mut self,
        v: impl Into<BlockAssignable<DataplexDatascanDataQualitySpecElRulesElTemplateReferenceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.template_reference = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.template_reference = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `uniqueness_expectation`.\n"]
    pub fn set_uniqueness_expectation(
        mut self,
        v: impl Into<BlockAssignable<DataplexDatascanDataQualitySpecElRulesElUniquenessExpectationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.uniqueness_expectation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.uniqueness_expectation = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataplexDatascanDataQualitySpecElRulesEl {
    type O = BlockAssignable<DataplexDatascanDataQualitySpecElRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataQualitySpecElRulesEl {
    #[doc = "The dimension name a rule belongs to. Custom dimension name is supported with all uppercase letters and maximum length of 30 characters."]
    pub dimension: PrimField<String>,
}
impl BuildDataplexDatascanDataQualitySpecElRulesEl {
    pub fn build(self) -> DataplexDatascanDataQualitySpecElRulesEl {
        DataplexDatascanDataQualitySpecElRulesEl {
            attributes: core::default::Default::default(),
            column: core::default::Default::default(),
            description: core::default::Default::default(),
            dimension: self.dimension,
            ignore_null: core::default::Default::default(),
            name: core::default::Default::default(),
            suspended: core::default::Default::default(),
            threshold: core::default::Default::default(),
            non_null_expectation: core::default::Default::default(),
            range_expectation: core::default::Default::default(),
            regex_expectation: core::default::Default::default(),
            row_condition_expectation: core::default::Default::default(),
            set_expectation: core::default::Default::default(),
            sql_assertion: core::default::Default::default(),
            statistic_range_expectation: core::default::Default::default(),
            table_condition_expectation: core::default::Default::default(),
            template_reference: core::default::Default::default(),
            uniqueness_expectation: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataplexDatascanDataQualitySpecElRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataQualitySpecElRulesElRef {
    fn new(shared: StackShared, base: String) -> DataplexDatascanDataQualitySpecElRulesElRef {
        DataplexDatascanDataQualitySpecElRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataQualitySpecElRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `attributes` after provisioning.\nMap of attribute name and value linked to the rule."]
    pub fn attributes(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.attributes", self.base))
    }
    #[doc = "Get a reference to the value of field `column` after provisioning.\nThe unnested column which this rule is evaluated against."]
    pub fn column(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the rule.\nThe maximum length is 1,024 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `dimension` after provisioning.\nThe dimension name a rule belongs to. Custom dimension name is supported with all uppercase letters and maximum length of 30 characters."]
    pub fn dimension(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dimension", self.base))
    }
    #[doc = "Get a reference to the value of field `ignore_null` after provisioning.\nRows with null values will automatically fail a rule, unless ignoreNull is true. In that case, such null rows are trivially considered passing. Only applicable to ColumnMap rules."]
    pub fn ignore_null(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_null", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA mutable name for the rule.\nThe name must contain only letters (a-z, A-Z), numbers (0-9), or hyphens (-).\nThe maximum length is 63 characters.\nMust start with a letter.\nMust end with a number or a letter."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `suspended` after provisioning.\nWhether the Rule is active or suspended. Default = false."]
    pub fn suspended(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.base))
    }
    #[doc = "Get a reference to the value of field `threshold` after provisioning.\nThe minimum ratio of passing_rows / total_rows required to pass this rule, with a range of [0.0, 1.0]. 0 indicates default value (i.e. 1.0)."]
    pub fn threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }
    #[doc = "Get a reference to the value of field `non_null_expectation` after provisioning.\n"]
    pub fn non_null_expectation(
        &self,
    ) -> ListRef<DataplexDatascanDataQualitySpecElRulesElNonNullExpectationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.non_null_expectation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `range_expectation` after provisioning.\n"]
    pub fn range_expectation(
        &self,
    ) -> ListRef<DataplexDatascanDataQualitySpecElRulesElRangeExpectationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.range_expectation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `regex_expectation` after provisioning.\n"]
    pub fn regex_expectation(
        &self,
    ) -> ListRef<DataplexDatascanDataQualitySpecElRulesElRegexExpectationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.regex_expectation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `row_condition_expectation` after provisioning.\n"]
    pub fn row_condition_expectation(
        &self,
    ) -> ListRef<DataplexDatascanDataQualitySpecElRulesElRowConditionExpectationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.row_condition_expectation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `set_expectation` after provisioning.\n"]
    pub fn set_expectation(
        &self,
    ) -> ListRef<DataplexDatascanDataQualitySpecElRulesElSetExpectationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.set_expectation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sql_assertion` after provisioning.\n"]
    pub fn sql_assertion(
        &self,
    ) -> ListRef<DataplexDatascanDataQualitySpecElRulesElSqlAssertionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sql_assertion", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `statistic_range_expectation` after provisioning.\n"]
    pub fn statistic_range_expectation(
        &self,
    ) -> ListRef<DataplexDatascanDataQualitySpecElRulesElStatisticRangeExpectationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.statistic_range_expectation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `table_condition_expectation` after provisioning.\n"]
    pub fn table_condition_expectation(
        &self,
    ) -> ListRef<DataplexDatascanDataQualitySpecElRulesElTableConditionExpectationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.table_condition_expectation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `template_reference` after provisioning.\n"]
    pub fn template_reference(
        &self,
    ) -> ListRef<DataplexDatascanDataQualitySpecElRulesElTemplateReferenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.template_reference", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uniqueness_expectation` after provisioning.\n"]
    pub fn uniqueness_expectation(
        &self,
    ) -> ListRef<DataplexDatascanDataQualitySpecElRulesElUniquenessExpectationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.uniqueness_expectation", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataplexDatascanDataQualitySpecElDynamic {
    post_scan_actions: Option<DynamicBlock<DataplexDatascanDataQualitySpecElPostScanActionsEl>>,
    rules: Option<DynamicBlock<DataplexDatascanDataQualitySpecElRulesEl>>,
}
#[derive(Serialize)]
pub struct DataplexDatascanDataQualitySpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_publishing_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_catalog_based_rules: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    row_filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sampling_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_scan_actions: Option<Vec<DataplexDatascanDataQualitySpecElPostScanActionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<Vec<DataplexDatascanDataQualitySpecElRulesEl>>,
    dynamic: DataplexDatascanDataQualitySpecElDynamic,
}
impl DataplexDatascanDataQualitySpecEl {
    #[doc = "Set the field `catalog_publishing_enabled`.\nIf set, the latest DataScan job result will be published to Dataplex Catalog."]
    pub fn set_catalog_publishing_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.catalog_publishing_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_catalog_based_rules`.\nIf set to true, the scan will retrieve rules defined in Data Catalog for the resource."]
    pub fn set_enable_catalog_based_rules(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_catalog_based_rules = Some(v.into());
        self
    }
    #[doc = "Set the field `filter`.\nA filter to selectively run a subset of rules."]
    pub fn set_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter = Some(v.into());
        self
    }
    #[doc = "Set the field `row_filter`.\nA filter applied to all rows in a single DataScan job. The filter needs to be a valid SQL expression for a WHERE clause in BigQuery standard SQL syntax. Example: col1 >= 0 AND col2 < 10"]
    pub fn set_row_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.row_filter = Some(v.into());
        self
    }
    #[doc = "Set the field `sampling_percent`.\nThe percentage of the records to be selected from the dataset for DataScan.\nValue can range between 0.0 and 100.0 with up to 3 significant decimal digits.\nSampling is not applied if 'sampling_percent' is not specified, 0 or 100."]
    pub fn set_sampling_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.sampling_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `post_scan_actions`.\n"]
    pub fn set_post_scan_actions(
        mut self,
        v: impl Into<BlockAssignable<DataplexDatascanDataQualitySpecElPostScanActionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.post_scan_actions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.post_scan_actions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(
        mut self,
        v: impl Into<BlockAssignable<DataplexDatascanDataQualitySpecElRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rules = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataplexDatascanDataQualitySpecEl {
    type O = BlockAssignable<DataplexDatascanDataQualitySpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanDataQualitySpecEl {}
impl BuildDataplexDatascanDataQualitySpecEl {
    pub fn build(self) -> DataplexDatascanDataQualitySpecEl {
        DataplexDatascanDataQualitySpecEl {
            catalog_publishing_enabled: core::default::Default::default(),
            enable_catalog_based_rules: core::default::Default::default(),
            filter: core::default::Default::default(),
            row_filter: core::default::Default::default(),
            sampling_percent: core::default::Default::default(),
            post_scan_actions: core::default::Default::default(),
            rules: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataplexDatascanDataQualitySpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanDataQualitySpecElRef {
    fn new(shared: StackShared, base: String) -> DataplexDatascanDataQualitySpecElRef {
        DataplexDatascanDataQualitySpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanDataQualitySpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `catalog_publishing_enabled` after provisioning.\nIf set, the latest DataScan job result will be published to Dataplex Catalog."]
    pub fn catalog_publishing_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.catalog_publishing_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_catalog_based_rules` after provisioning.\nIf set to true, the scan will retrieve rules defined in Data Catalog for the resource."]
    pub fn enable_catalog_based_rules(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_catalog_based_rules", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nA filter to selectively run a subset of rules."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
    #[doc = "Get a reference to the value of field `row_filter` after provisioning.\nA filter applied to all rows in a single DataScan job. The filter needs to be a valid SQL expression for a WHERE clause in BigQuery standard SQL syntax. Example: col1 >= 0 AND col2 < 10"]
    pub fn row_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.row_filter", self.base))
    }
    #[doc = "Get a reference to the value of field `sampling_percent` after provisioning.\nThe percentage of the records to be selected from the dataset for DataScan.\nValue can range between 0.0 and 100.0 with up to 3 significant decimal digits.\nSampling is not applied if 'sampling_percent' is not specified, 0 or 100."]
    pub fn sampling_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sampling_percent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `post_scan_actions` after provisioning.\n"]
    pub fn post_scan_actions(
        &self,
    ) -> ListRef<DataplexDatascanDataQualitySpecElPostScanActionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.post_scan_actions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<DataplexDatascanDataQualitySpecElRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanExecutionIdentityElDataplexServiceAgentEl {}
impl DataplexDatascanExecutionIdentityElDataplexServiceAgentEl {}
impl ToListMappable for DataplexDatascanExecutionIdentityElDataplexServiceAgentEl {
    type O = BlockAssignable<DataplexDatascanExecutionIdentityElDataplexServiceAgentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanExecutionIdentityElDataplexServiceAgentEl {}
impl BuildDataplexDatascanExecutionIdentityElDataplexServiceAgentEl {
    pub fn build(self) -> DataplexDatascanExecutionIdentityElDataplexServiceAgentEl {
        DataplexDatascanExecutionIdentityElDataplexServiceAgentEl {}
    }
}
pub struct DataplexDatascanExecutionIdentityElDataplexServiceAgentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanExecutionIdentityElDataplexServiceAgentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanExecutionIdentityElDataplexServiceAgentElRef {
        DataplexDatascanExecutionIdentityElDataplexServiceAgentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanExecutionIdentityElDataplexServiceAgentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanExecutionIdentityElServiceAccountEl {
    email: PrimField<String>,
}
impl DataplexDatascanExecutionIdentityElServiceAccountEl {}
impl ToListMappable for DataplexDatascanExecutionIdentityElServiceAccountEl {
    type O = BlockAssignable<DataplexDatascanExecutionIdentityElServiceAccountEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanExecutionIdentityElServiceAccountEl {
    #[doc = "Service account email."]
    pub email: PrimField<String>,
}
impl BuildDataplexDatascanExecutionIdentityElServiceAccountEl {
    pub fn build(self) -> DataplexDatascanExecutionIdentityElServiceAccountEl {
        DataplexDatascanExecutionIdentityElServiceAccountEl { email: self.email }
    }
}
pub struct DataplexDatascanExecutionIdentityElServiceAccountElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanExecutionIdentityElServiceAccountElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanExecutionIdentityElServiceAccountElRef {
        DataplexDatascanExecutionIdentityElServiceAccountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanExecutionIdentityElServiceAccountElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\nService account email."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanExecutionIdentityElUserCredentialEl {}
impl DataplexDatascanExecutionIdentityElUserCredentialEl {}
impl ToListMappable for DataplexDatascanExecutionIdentityElUserCredentialEl {
    type O = BlockAssignable<DataplexDatascanExecutionIdentityElUserCredentialEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanExecutionIdentityElUserCredentialEl {}
impl BuildDataplexDatascanExecutionIdentityElUserCredentialEl {
    pub fn build(self) -> DataplexDatascanExecutionIdentityElUserCredentialEl {
        DataplexDatascanExecutionIdentityElUserCredentialEl {}
    }
}
pub struct DataplexDatascanExecutionIdentityElUserCredentialElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanExecutionIdentityElUserCredentialElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanExecutionIdentityElUserCredentialElRef {
        DataplexDatascanExecutionIdentityElUserCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanExecutionIdentityElUserCredentialElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct DataplexDatascanExecutionIdentityElDynamic {
    dataplex_service_agent:
        Option<DynamicBlock<DataplexDatascanExecutionIdentityElDataplexServiceAgentEl>>,
    service_account: Option<DynamicBlock<DataplexDatascanExecutionIdentityElServiceAccountEl>>,
    user_credential: Option<DynamicBlock<DataplexDatascanExecutionIdentityElUserCredentialEl>>,
}
#[derive(Serialize)]
pub struct DataplexDatascanExecutionIdentityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataplex_service_agent: Option<Vec<DataplexDatascanExecutionIdentityElDataplexServiceAgentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<Vec<DataplexDatascanExecutionIdentityElServiceAccountEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_credential: Option<Vec<DataplexDatascanExecutionIdentityElUserCredentialEl>>,
    dynamic: DataplexDatascanExecutionIdentityElDynamic,
}
impl DataplexDatascanExecutionIdentityEl {
    #[doc = "Set the field `dataplex_service_agent`.\n"]
    pub fn set_dataplex_service_agent(
        mut self,
        v: impl Into<BlockAssignable<DataplexDatascanExecutionIdentityElDataplexServiceAgentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dataplex_service_agent = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dataplex_service_agent = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(
        mut self,
        v: impl Into<BlockAssignable<DataplexDatascanExecutionIdentityElServiceAccountEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_account = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_account = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `user_credential`.\n"]
    pub fn set_user_credential(
        mut self,
        v: impl Into<BlockAssignable<DataplexDatascanExecutionIdentityElUserCredentialEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.user_credential = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.user_credential = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataplexDatascanExecutionIdentityEl {
    type O = BlockAssignable<DataplexDatascanExecutionIdentityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanExecutionIdentityEl {}
impl BuildDataplexDatascanExecutionIdentityEl {
    pub fn build(self) -> DataplexDatascanExecutionIdentityEl {
        DataplexDatascanExecutionIdentityEl {
            dataplex_service_agent: core::default::Default::default(),
            service_account: core::default::Default::default(),
            user_credential: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataplexDatascanExecutionIdentityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanExecutionIdentityElRef {
    fn new(shared: StackShared, base: String) -> DataplexDatascanExecutionIdentityElRef {
        DataplexDatascanExecutionIdentityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanExecutionIdentityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataplex_service_agent` after provisioning.\n"]
    pub fn dataplex_service_agent(
        &self,
    ) -> ListRef<DataplexDatascanExecutionIdentityElDataplexServiceAgentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dataplex_service_agent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(
        &self,
    ) -> ListRef<DataplexDatascanExecutionIdentityElServiceAccountElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `user_credential` after provisioning.\n"]
    pub fn user_credential(
        &self,
    ) -> ListRef<DataplexDatascanExecutionIdentityElUserCredentialElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_credential", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanExecutionSpecElTriggerElOnDemandEl {}
impl DataplexDatascanExecutionSpecElTriggerElOnDemandEl {}
impl ToListMappable for DataplexDatascanExecutionSpecElTriggerElOnDemandEl {
    type O = BlockAssignable<DataplexDatascanExecutionSpecElTriggerElOnDemandEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanExecutionSpecElTriggerElOnDemandEl {}
impl BuildDataplexDatascanExecutionSpecElTriggerElOnDemandEl {
    pub fn build(self) -> DataplexDatascanExecutionSpecElTriggerElOnDemandEl {
        DataplexDatascanExecutionSpecElTriggerElOnDemandEl {}
    }
}
pub struct DataplexDatascanExecutionSpecElTriggerElOnDemandElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanExecutionSpecElTriggerElOnDemandElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanExecutionSpecElTriggerElOnDemandElRef {
        DataplexDatascanExecutionSpecElTriggerElOnDemandElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanExecutionSpecElTriggerElOnDemandElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanExecutionSpecElTriggerElOneTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl_after_scan_completion: Option<PrimField<String>>,
}
impl DataplexDatascanExecutionSpecElTriggerElOneTimeEl {
    #[doc = "Set the field `ttl_after_scan_completion`.\nTime to live for the DataScan and its results after the one-time run completes. Accepts a string with a unit suffix 's' (e.g., '7200s'). Default is 24 hours. Ranges between 0 and 31536000 seconds (1 year)."]
    pub fn set_ttl_after_scan_completion(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ttl_after_scan_completion = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexDatascanExecutionSpecElTriggerElOneTimeEl {
    type O = BlockAssignable<DataplexDatascanExecutionSpecElTriggerElOneTimeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanExecutionSpecElTriggerElOneTimeEl {}
impl BuildDataplexDatascanExecutionSpecElTriggerElOneTimeEl {
    pub fn build(self) -> DataplexDatascanExecutionSpecElTriggerElOneTimeEl {
        DataplexDatascanExecutionSpecElTriggerElOneTimeEl {
            ttl_after_scan_completion: core::default::Default::default(),
        }
    }
}
pub struct DataplexDatascanExecutionSpecElTriggerElOneTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanExecutionSpecElTriggerElOneTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanExecutionSpecElTriggerElOneTimeElRef {
        DataplexDatascanExecutionSpecElTriggerElOneTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanExecutionSpecElTriggerElOneTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ttl_after_scan_completion` after provisioning.\nTime to live for the DataScan and its results after the one-time run completes. Accepts a string with a unit suffix 's' (e.g., '7200s'). Default is 24 hours. Ranges between 0 and 31536000 seconds (1 year)."]
    pub fn ttl_after_scan_completion(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ttl_after_scan_completion", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanExecutionSpecElTriggerElScheduleEl {
    cron: PrimField<String>,
}
impl DataplexDatascanExecutionSpecElTriggerElScheduleEl {}
impl ToListMappable for DataplexDatascanExecutionSpecElTriggerElScheduleEl {
    type O = BlockAssignable<DataplexDatascanExecutionSpecElTriggerElScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanExecutionSpecElTriggerElScheduleEl {
    #[doc = "Cron schedule for running scans periodically. This field is required for Schedule scans."]
    pub cron: PrimField<String>,
}
impl BuildDataplexDatascanExecutionSpecElTriggerElScheduleEl {
    pub fn build(self) -> DataplexDatascanExecutionSpecElTriggerElScheduleEl {
        DataplexDatascanExecutionSpecElTriggerElScheduleEl { cron: self.cron }
    }
}
pub struct DataplexDatascanExecutionSpecElTriggerElScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanExecutionSpecElTriggerElScheduleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexDatascanExecutionSpecElTriggerElScheduleElRef {
        DataplexDatascanExecutionSpecElTriggerElScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanExecutionSpecElTriggerElScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cron` after provisioning.\nCron schedule for running scans periodically. This field is required for Schedule scans."]
    pub fn cron(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cron", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataplexDatascanExecutionSpecElTriggerElDynamic {
    on_demand: Option<DynamicBlock<DataplexDatascanExecutionSpecElTriggerElOnDemandEl>>,
    one_time: Option<DynamicBlock<DataplexDatascanExecutionSpecElTriggerElOneTimeEl>>,
    schedule: Option<DynamicBlock<DataplexDatascanExecutionSpecElTriggerElScheduleEl>>,
}
#[derive(Serialize)]
pub struct DataplexDatascanExecutionSpecElTriggerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    on_demand: Option<Vec<DataplexDatascanExecutionSpecElTriggerElOnDemandEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    one_time: Option<Vec<DataplexDatascanExecutionSpecElTriggerElOneTimeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<Vec<DataplexDatascanExecutionSpecElTriggerElScheduleEl>>,
    dynamic: DataplexDatascanExecutionSpecElTriggerElDynamic,
}
impl DataplexDatascanExecutionSpecElTriggerEl {
    #[doc = "Set the field `on_demand`.\n"]
    pub fn set_on_demand(
        mut self,
        v: impl Into<BlockAssignable<DataplexDatascanExecutionSpecElTriggerElOnDemandEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.on_demand = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.on_demand = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `one_time`.\n"]
    pub fn set_one_time(
        mut self,
        v: impl Into<BlockAssignable<DataplexDatascanExecutionSpecElTriggerElOneTimeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.one_time = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.one_time = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(
        mut self,
        v: impl Into<BlockAssignable<DataplexDatascanExecutionSpecElTriggerElScheduleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schedule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schedule = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataplexDatascanExecutionSpecElTriggerEl {
    type O = BlockAssignable<DataplexDatascanExecutionSpecElTriggerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanExecutionSpecElTriggerEl {}
impl BuildDataplexDatascanExecutionSpecElTriggerEl {
    pub fn build(self) -> DataplexDatascanExecutionSpecElTriggerEl {
        DataplexDatascanExecutionSpecElTriggerEl {
            on_demand: core::default::Default::default(),
            one_time: core::default::Default::default(),
            schedule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataplexDatascanExecutionSpecElTriggerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanExecutionSpecElTriggerElRef {
    fn new(shared: StackShared, base: String) -> DataplexDatascanExecutionSpecElTriggerElRef {
        DataplexDatascanExecutionSpecElTriggerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanExecutionSpecElTriggerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `on_demand` after provisioning.\n"]
    pub fn on_demand(&self) -> ListRef<DataplexDatascanExecutionSpecElTriggerElOnDemandElRef> {
        ListRef::new(self.shared().clone(), format!("{}.on_demand", self.base))
    }
    #[doc = "Get a reference to the value of field `one_time` after provisioning.\n"]
    pub fn one_time(&self) -> ListRef<DataplexDatascanExecutionSpecElTriggerElOneTimeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.one_time", self.base))
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> ListRef<DataplexDatascanExecutionSpecElTriggerElScheduleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataplexDatascanExecutionSpecElDynamic {
    trigger: Option<DynamicBlock<DataplexDatascanExecutionSpecElTriggerEl>>,
}
#[derive(Serialize)]
pub struct DataplexDatascanExecutionSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger: Option<Vec<DataplexDatascanExecutionSpecElTriggerEl>>,
    dynamic: DataplexDatascanExecutionSpecElDynamic,
}
impl DataplexDatascanExecutionSpecEl {
    #[doc = "Set the field `field`.\nThe unnested field (of type Date or Timestamp) that contains values which monotonically increase over time. If not specified, a data scan will run for all data in the table."]
    pub fn set_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field = Some(v.into());
        self
    }
    #[doc = "Set the field `trigger`.\n"]
    pub fn set_trigger(
        mut self,
        v: impl Into<BlockAssignable<DataplexDatascanExecutionSpecElTriggerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.trigger = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.trigger = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataplexDatascanExecutionSpecEl {
    type O = BlockAssignable<DataplexDatascanExecutionSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanExecutionSpecEl {}
impl BuildDataplexDatascanExecutionSpecEl {
    pub fn build(self) -> DataplexDatascanExecutionSpecEl {
        DataplexDatascanExecutionSpecEl {
            field: core::default::Default::default(),
            trigger: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataplexDatascanExecutionSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanExecutionSpecElRef {
    fn new(shared: StackShared, base: String) -> DataplexDatascanExecutionSpecElRef {
        DataplexDatascanExecutionSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanExecutionSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `field` after provisioning.\nThe unnested field (of type Date or Timestamp) that contains values which monotonically increase over time. If not specified, a data scan will run for all data in the table."]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }
    #[doc = "Get a reference to the value of field `trigger` after provisioning.\n"]
    pub fn trigger(&self) -> ListRef<DataplexDatascanExecutionSpecElTriggerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.trigger", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexDatascanTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DataplexDatascanTimeoutsEl {
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
impl ToListMappable for DataplexDatascanTimeoutsEl {
    type O = BlockAssignable<DataplexDatascanTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexDatascanTimeoutsEl {}
impl BuildDataplexDatascanTimeoutsEl {
    pub fn build(self) -> DataplexDatascanTimeoutsEl {
        DataplexDatascanTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DataplexDatascanTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexDatascanTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataplexDatascanTimeoutsElRef {
        DataplexDatascanTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexDatascanTimeoutsElRef {
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
struct DataplexDatascanDynamic {
    data: Option<DynamicBlock<DataplexDatascanDataEl>>,
    data_discovery_spec: Option<DynamicBlock<DataplexDatascanDataDiscoverySpecEl>>,
    data_documentation_spec: Option<DynamicBlock<DataplexDatascanDataDocumentationSpecEl>>,
    data_profile_spec: Option<DynamicBlock<DataplexDatascanDataProfileSpecEl>>,
    data_quality_spec: Option<DynamicBlock<DataplexDatascanDataQualitySpecEl>>,
    execution_identity: Option<DynamicBlock<DataplexDatascanExecutionIdentityEl>>,
    execution_spec: Option<DynamicBlock<DataplexDatascanExecutionSpecEl>>,
}
