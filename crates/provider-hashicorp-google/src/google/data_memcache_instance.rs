use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataMemcacheInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataMemcacheInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataMemcacheInstanceData>,
}
#[derive(Clone)]
pub struct DataMemcacheInstance(Rc<DataMemcacheInstance_>);
impl DataMemcacheInstance {
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
    #[doc = "Set the field `region`.\nThe region of the Memcache instance. If it is not provided, the provider region is used."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `authorized_network` after provisioning.\nThe full name of the GCE network to connect the instance to.  If not provided,\n'default' will be used."]
    pub fn authorized_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorized_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreation timestamp in RFC3339 text format."]
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
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the instance.\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is not set to false in Terraform state.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the instance will fail.\nWhen the field is set to false, deleting the instance is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `discovery_endpoint` after provisioning.\nEndpoint for Discovery API"]
    pub fn discovery_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.discovery_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA user-visible name for the instance."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\nMaintenance policy for an instance."]
    pub fn maintenance_policy(&self) -> ListRef<DataMemcacheInstanceMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_schedule` after provisioning.\nOutput only. Published maintenance schedule."]
    pub fn maintenance_schedule(&self) -> ListRef<DataMemcacheInstanceMaintenanceScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memcache_full_version` after provisioning.\nThe full version of memcached server running on this instance."]
    pub fn memcache_full_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memcache_full_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memcache_nodes` after provisioning.\nAdditional information about the instance state, if available."]
    pub fn memcache_nodes(&self) -> ListRef<DataMemcacheInstanceMemcacheNodesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.memcache_nodes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memcache_parameters` after provisioning.\nUser-specified parameters for this memcache instance."]
    pub fn memcache_parameters(&self) -> ListRef<DataMemcacheInstanceMemcacheParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.memcache_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memcache_version` after provisioning.\nThe major version of Memcached software. If not provided, latest supported version will be used.\nCurrently the latest supported major version is MEMCACHE_1_5. The minor version will be automatically\ndetermined by our system based on the latest supported minor version. Default value: \"MEMCACHE_1_5\" Possible values: [\"MEMCACHE_1_5\", \"MEMCACHE_1_6_15\"]"]
    pub fn memcache_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memcache_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the instance."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_config` after provisioning.\nConfiguration for memcache nodes."]
    pub fn node_config(&self) -> ListRef<DataMemcacheInstanceNodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_count` after provisioning.\nNumber of nodes in the memcache instance."]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the Memcache instance. If it is not provided, the provider region is used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reserved_ip_range_id` after provisioning.\nContains the name of allocated IP address ranges associated with\nthe private service access connection for example, \"test-default\"\nassociated with IP range 10.0.0.0/29."]
    pub fn reserved_ip_range_id(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reserved_ip_range_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zones` after provisioning.\nZones where memcache nodes should be provisioned.  If not\nprovided, all zones will be used."]
    pub fn zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.zones", self.extract_ref()),
        )
    }
}
impl Referable for DataMemcacheInstance {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataMemcacheInstance {}
impl ToListMappable for DataMemcacheInstance {
    type O = ListRef<DataMemcacheInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataMemcacheInstance_ {
    fn extract_datasource_type(&self) -> String {
        "google_memcache_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataMemcacheInstance {
    pub tf_id: String,
    #[doc = "The resource name of the instance."]
    pub name: PrimField<String>,
}
impl BuildDataMemcacheInstance {
    pub fn build(self, stack: &mut Stack) -> DataMemcacheInstance {
        let out = DataMemcacheInstance(Rc::new(DataMemcacheInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataMemcacheInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataMemcacheInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemcacheInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataMemcacheInstanceRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `authorized_network` after provisioning.\nThe full name of the GCE network to connect the instance to.  If not provided,\n'default' will be used."]
    pub fn authorized_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorized_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreation timestamp in RFC3339 text format."]
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
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the instance.\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is not set to false in Terraform state.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the instance will fail.\nWhen the field is set to false, deleting the instance is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `discovery_endpoint` after provisioning.\nEndpoint for Discovery API"]
    pub fn discovery_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.discovery_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA user-visible name for the instance."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\nMaintenance policy for an instance."]
    pub fn maintenance_policy(&self) -> ListRef<DataMemcacheInstanceMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_schedule` after provisioning.\nOutput only. Published maintenance schedule."]
    pub fn maintenance_schedule(&self) -> ListRef<DataMemcacheInstanceMaintenanceScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memcache_full_version` after provisioning.\nThe full version of memcached server running on this instance."]
    pub fn memcache_full_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memcache_full_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memcache_nodes` after provisioning.\nAdditional information about the instance state, if available."]
    pub fn memcache_nodes(&self) -> ListRef<DataMemcacheInstanceMemcacheNodesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.memcache_nodes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memcache_parameters` after provisioning.\nUser-specified parameters for this memcache instance."]
    pub fn memcache_parameters(&self) -> ListRef<DataMemcacheInstanceMemcacheParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.memcache_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memcache_version` after provisioning.\nThe major version of Memcached software. If not provided, latest supported version will be used.\nCurrently the latest supported major version is MEMCACHE_1_5. The minor version will be automatically\ndetermined by our system based on the latest supported minor version. Default value: \"MEMCACHE_1_5\" Possible values: [\"MEMCACHE_1_5\", \"MEMCACHE_1_6_15\"]"]
    pub fn memcache_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memcache_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the instance."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_config` after provisioning.\nConfiguration for memcache nodes."]
    pub fn node_config(&self) -> ListRef<DataMemcacheInstanceNodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_count` after provisioning.\nNumber of nodes in the memcache instance."]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the Memcache instance. If it is not provided, the provider region is used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reserved_ip_range_id` after provisioning.\nContains the name of allocated IP address ranges associated with\nthe private service access connection for example, \"test-default\"\nassociated with IP range 10.0.0.0/29."]
    pub fn reserved_ip_range_id(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reserved_ip_range_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zones` after provisioning.\nZones where memcache nodes should be provisioned.  If not\nprovided, all zones will be used."]
    pub fn zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.zones", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    #[doc = "Set the field `hours`.\n"]
    pub fn set_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hours = Some(v.into());
        self
    }
    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minutes = Some(v.into());
        self
    }
    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl
{
    type O = BlockAssignable<
        DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {}
impl BuildDataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    pub fn build(
        self,
    ) -> DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
        DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
        DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours` after provisioning.\n"]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<
        ListField<DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>,
    >,
}
impl DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    #[doc = "Set the field `day`.\n"]
    pub fn set_day(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.day = Some(v.into());
        self
    }
    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.duration = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v: impl Into<
            ListField<DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>,
        >,
    ) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    type O = BlockAssignable<DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {}
impl BuildDataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    pub fn build(self) -> DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
        DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
            day: core::default::Default::default(),
            duration: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
        DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\n"]
    pub fn day(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> ListRef<DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMemcacheInstanceMaintenancePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_maintenance_window:
        Option<ListField<DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>>,
}
impl DataMemcacheInstanceMaintenancePolicyEl {
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
    #[doc = "Set the field `weekly_maintenance_window`.\n"]
    pub fn set_weekly_maintenance_window(
        mut self,
        v: impl Into<ListField<DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>>,
    ) -> Self {
        self.weekly_maintenance_window = Some(v.into());
        self
    }
}
impl ToListMappable for DataMemcacheInstanceMaintenancePolicyEl {
    type O = BlockAssignable<DataMemcacheInstanceMaintenancePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemcacheInstanceMaintenancePolicyEl {}
impl BuildDataMemcacheInstanceMaintenancePolicyEl {
    pub fn build(self) -> DataMemcacheInstanceMaintenancePolicyEl {
        DataMemcacheInstanceMaintenancePolicyEl {
            create_time: core::default::Default::default(),
            description: core::default::Default::default(),
            update_time: core::default::Default::default(),
            weekly_maintenance_window: core::default::Default::default(),
        }
    }
}
pub struct DataMemcacheInstanceMaintenancePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemcacheInstanceMaintenancePolicyElRef {
    fn new(shared: StackShared, base: String) -> DataMemcacheInstanceMaintenancePolicyElRef {
        DataMemcacheInstanceMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemcacheInstanceMaintenancePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
    #[doc = "Get a reference to the value of field `weekly_maintenance_window` after provisioning.\n"]
    pub fn weekly_maintenance_window(
        &self,
    ) -> ListRef<DataMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.weekly_maintenance_window", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataMemcacheInstanceMaintenanceScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_deadline_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl DataMemcacheInstanceMaintenanceScheduleEl {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `schedule_deadline_time`.\n"]
    pub fn set_schedule_deadline_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schedule_deadline_time = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataMemcacheInstanceMaintenanceScheduleEl {
    type O = BlockAssignable<DataMemcacheInstanceMaintenanceScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemcacheInstanceMaintenanceScheduleEl {}
impl BuildDataMemcacheInstanceMaintenanceScheduleEl {
    pub fn build(self) -> DataMemcacheInstanceMaintenanceScheduleEl {
        DataMemcacheInstanceMaintenanceScheduleEl {
            end_time: core::default::Default::default(),
            schedule_deadline_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DataMemcacheInstanceMaintenanceScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemcacheInstanceMaintenanceScheduleElRef {
    fn new(shared: StackShared, base: String) -> DataMemcacheInstanceMaintenanceScheduleElRef {
        DataMemcacheInstanceMaintenanceScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemcacheInstanceMaintenanceScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `schedule_deadline_time` after provisioning.\n"]
    pub fn schedule_deadline_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schedule_deadline_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMemcacheInstanceMemcacheNodesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
impl DataMemcacheInstanceMemcacheNodesEl {
    #[doc = "Set the field `host`.\n"]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }
    #[doc = "Set the field `node_id`.\n"]
    pub fn set_node_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_id = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
}
impl ToListMappable for DataMemcacheInstanceMemcacheNodesEl {
    type O = BlockAssignable<DataMemcacheInstanceMemcacheNodesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemcacheInstanceMemcacheNodesEl {}
impl BuildDataMemcacheInstanceMemcacheNodesEl {
    pub fn build(self) -> DataMemcacheInstanceMemcacheNodesEl {
        DataMemcacheInstanceMemcacheNodesEl {
            host: core::default::Default::default(),
            node_id: core::default::Default::default(),
            port: core::default::Default::default(),
            state: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}
pub struct DataMemcacheInstanceMemcacheNodesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemcacheInstanceMemcacheNodesElRef {
    fn new(shared: StackShared, base: String) -> DataMemcacheInstanceMemcacheNodesElRef {
        DataMemcacheInstanceMemcacheNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemcacheInstanceMemcacheNodesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `node_id` after provisioning.\n"]
    pub fn node_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_id", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMemcacheInstanceMemcacheParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<RecField<PrimField<String>>>,
}
impl DataMemcacheInstanceMemcacheParametersEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `params`.\n"]
    pub fn set_params(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.params = Some(v.into());
        self
    }
}
impl ToListMappable for DataMemcacheInstanceMemcacheParametersEl {
    type O = BlockAssignable<DataMemcacheInstanceMemcacheParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemcacheInstanceMemcacheParametersEl {}
impl BuildDataMemcacheInstanceMemcacheParametersEl {
    pub fn build(self) -> DataMemcacheInstanceMemcacheParametersEl {
        DataMemcacheInstanceMemcacheParametersEl {
            id: core::default::Default::default(),
            params: core::default::Default::default(),
        }
    }
}
pub struct DataMemcacheInstanceMemcacheParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemcacheInstanceMemcacheParametersElRef {
    fn new(shared: StackShared, base: String) -> DataMemcacheInstanceMemcacheParametersElRef {
        DataMemcacheInstanceMemcacheParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemcacheInstanceMemcacheParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.params", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMemcacheInstanceNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_size_mb: Option<PrimField<f64>>,
}
impl DataMemcacheInstanceNodeConfigEl {
    #[doc = "Set the field `cpu_count`.\n"]
    pub fn set_cpu_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu_count = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_size_mb`.\n"]
    pub fn set_memory_size_mb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_size_mb = Some(v.into());
        self
    }
}
impl ToListMappable for DataMemcacheInstanceNodeConfigEl {
    type O = BlockAssignable<DataMemcacheInstanceNodeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMemcacheInstanceNodeConfigEl {}
impl BuildDataMemcacheInstanceNodeConfigEl {
    pub fn build(self) -> DataMemcacheInstanceNodeConfigEl {
        DataMemcacheInstanceNodeConfigEl {
            cpu_count: core::default::Default::default(),
            memory_size_mb: core::default::Default::default(),
        }
    }
}
pub struct DataMemcacheInstanceNodeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMemcacheInstanceNodeConfigElRef {
    fn new(shared: StackShared, base: String) -> DataMemcacheInstanceNodeConfigElRef {
        DataMemcacheInstanceNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMemcacheInstanceNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cpu_count` after provisioning.\n"]
    pub fn cpu_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_count", self.base))
    }
    #[doc = "Get a reference to the value of field `memory_size_mb` after provisioning.\n"]
    pub fn memory_size_mb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_size_mb", self.base),
        )
    }
}
